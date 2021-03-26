class API {
  // https://github.com/JoeKays/dpts/blob/master/src/dpt.ts
  constructor() {
    this.url = 'https://digitalpaper.local:8443'
    this.clientId = localStorage.getItem('deviceid');
    this.privateKey = localStorage.getItem('privatekey');
    if (this.privateKey) {
      this.privateKey = prepareKey(this.privateKey);
    }
  }

  /* High-level API */

  async downloadUploadShow(url) {
    function determineName(url, response) {
      // variant of https://stackoverflow.com/questions/23054475/javascript-regex-for-extracting-filename-from-content-disposition-header
      const rx = /filename[^;=\n]*=(?:(['"])(.*?)\1|([^;\n]*))/;
      const m = rx.exec(response.headers['Content-Disposition']);
      if (m) {
        // group 2 is quoted name, group 3 is unquoted.
        return m[2] || m[3];
      }

      const urlbits = url.split('/');
      let name = url.replace(/\//g, '-');
      if (!name.endsWith('.pdf')) {
        name += '.pdf';
      }
      return name;

      // old sanitization routine
      //let name = urlbits[urlbits.length - 1];
      // sanitize! intended to help with URLs like this: https://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.422.9049&rep=rep1&type=pdf
      // both period (.) and equals (=) seem to not be valid in the name
      //name = name.replace(/[^-\.=_a-zA-Z0-9]/g, '-');
    }

    // very high-level!

    // Get the data
    const response = await fetch(url);
    const blob = await response.blob();

    // Name the file and directory.
    const dir = 'Document/Received/';
    const name = determineName(url, response).slice(0, 200); // Truncate name to be valid.
    const fullpath = dir+name;

    // Check out any existing file.
    const info = await this.resolveObject(fullpath);
    if (info) {
      // If file size is equal, then we just open the document.
      if (info.file_size == blob.size) {
        await this.displayDocument(info.entry_id);
        return;
      } else {
        // Delete any existing file otherwise.
        await this.deleteDocument(info.entry_id);
      }
    }

    // Upload it
    const id = await this.uploadBlob(dir, name, blob);

    // Show it!
    await this.displayDocument(id);
  }

  async authenticate() {
    const response = await this.request('/auth/nonce/' + this.clientId);
    const nonce = (await response.json()).nonce;
    const signed = await this.signNonce(nonce, this.privateKey);
    return await this.request('/auth', {
      method: 'PUT',
      json: {client_id: this.clientId, nonce_signed: signed},
    });
  }

  async resolveObject(path) {
    try {
      const response = await this.request(`/resolve/entry/path/${encodeURIComponent(path)}`);
      return await response.json();
    } catch (e) {
      if (e.response && e.response.status == 404) {
        return null;
      }
      throw e;
    }
  }

  async getObjectId(pathOrId) {
    // This function accepts either paths or document IDs and returns a document ID.
    if (isUUID(pathOrId)) {
      return pathOrId;
    }
    const info = await this.resolveObject(pathOrId);
    if (info) {
      return info.entry_id;
    } else {
      return null;
    }
  }

  async uploadBlob(directory, name, blob) {
    if (blob.type !== 'application/pdf') {
      throw new Error(`Blob with invalid content type: ${blob.type}`);
    }

    // Make the file entry
    const directory_id = await this.getObjectId(directory);
    const json = { file_name: name, parent_folder_id: directory_id, document_source: ''};
    const doc = await (await this.request('/documents2', {method: 'POST', json})).json();

    // Upload the file content.
    const body = new FormData();
    body.append('file', blob);
    await this.request(`/documents/${doc['document_id']}/file`, {method: 'PUT', body});

    // Return the file's ID.
    return doc.document_id;
  }

  async displayDocument(path, page=undefined) {
    const id = await this.getObjectId(path);
    await this.request('/viewer/controls/open2', {method: 'PUT', json: {document_id: id, page}});
  }

  async deleteDocument(path) {
    const id = await this.getObjectId(path);
    if (id) {
      await this.request(`/documents/${id}`, {method: 'DELETE'});
    }
  }

  async ping() {
    // Fast timeout so that we don't wait too long.
    await this.request('/ping', {timeout: 1000});
  }

  /* Low-level API */

  async request(path, options = {}) {
    // Process options
    options.credentials ||= 'include';
    if ('json' in options) {
      options.headers ||= {};
      options.headers['Content-Type'] = 'application/json';
      options.body = JSON.stringify(options.json);
      delete options.json;
    }
    if ('timeout' in options) {
      const controller = new AbortController();
      setTimeout(() => controller.abort(), options.timeout);
      options.signal = controller.signal;

      delete options.timeout;
    }

    // Send the request!
    let response;
    try {
      response = await fetch(this.url + path, options);
    } catch (e) {
      if (e.message == 'The user aborted a request.') {
        throw new Error('Request timed out.');
      }
      throw e;
    }
    if (!response.ok) {
      const e = new Error('Error: Request returned status code: ' + response.status);
      e.response = response;
      throw e;
    }
    return response;
  }

  async signNonce(nonce, keyStr) {
    const options = {name: 'RSASSA-PKCS1-v1_5', hash: 'SHA-256'};
    const key = await crypto.subtle.importKey('pkcs8', b64.decode(keyStr), options, false, ['sign']);
    const signature = await crypto.subtle.sign('RSASSA-PKCS1-v1_5', key, new TextEncoder().encode(nonce).buffer);
    return b64.encode(new Uint8Array(signature));
  }
}

/* utilities */

function isUUID(maybeUUID) {
  // This is too permissive a check, but was simple to code. We mostly intend to distinguish between
  // document IDs and URLs, so this will suffice.
  return /[-0-9a-fA-F]{36}/.test(maybeUUID);
}

function promisifyChrome(thunking) {
  return function() {
    const that = this;
    const args = Array.from(arguments);
    return new Promise(resolve => {
      args.push(resolve);
      thunking.apply(that, args);
    });
  };
}

function loadFile(f) {
  return new Promise((resolve, reject) => {
    var r = new FileReader();
    r.onload = function(e) {
      resolve(e.target.result);
    };
    r.onerror = function(e) {
      reject(r.error);
    };
    r.readAsText(f);
  });
}

/* ui flow */

async function main() {
  const tabs = await promisifyChrome(chrome.tabs.query)({active: true, currentWindow: true});
  const url = tabs[0].url;

  const api = new API();
  statusText.textContent = `Connection check...`;
  await api.ping();

  statusText.textContent = `Connected. Uploading...`;
  await api.downloadUploadShow(url);

  statusText.textContent = `Uploaded.`;

  setTimeout(() => {
    window.close();
  }, 5000);
}

function render() {
  authErrorText.textContent = '';
  const id = localStorage.getItem('deviceid');
  if (id) {
    credsContainer.classList.add('hide');
    sendContainer.classList.remove('hide');
    statusText.textContent = 'Ready to send.';
  } else {
    credsContainer.classList.remove('hide');
    sendContainer.classList.add('hide');
    statusText.textContent = '';
  }
}

async function creds() {
  const inputId = document.querySelector('input[type=file][name=id]');
  const inputKey = document.querySelector('input[type=file][name=key]');
  if (!inputId.files.length || !inputKey.files.length) {
    throw new Error('Must select files.');
  }
  if (inputId.files[0].name != 'deviceid.dat') {
    throw new Error('Device ID filename is incorrect.');
  }
  if (inputKey.files[0].name != 'privatekey.dat') {
    throw new Error('Device ID filename is incorrect.');
  }
  const id = (await loadFile(inputId.files[0])).trim();
  if (id.length !== 36) {
    throw new Error('Device ID seems to be invalid.');
  }
  const key = (await loadFile(inputKey.files[0])).trim();
  if (!key.startsWith('-----BEGIN RSA PRIVATE KEY-----') || !key.endsWith('-----END RSA PRIVATE KEY-----')) {
    throw new Error('Private key seems to be invalid.');
  }

  try {
    localStorage.setItem('deviceid', id);
    localStorage.setItem('privatekey', key);
    const api = new API();
    statusText.textContent = `Authenticating...`;
    await api.authenticate();
  } catch(e) {
    localStorage.removeItem('deviceid');
    localStorage.removeItem('privatekey');
    throw e;
  }

  render();
}

async function resetCreds() {
  try {
    // If we have good stored credentials, but just need to reauthorize, we do this flow:
    const api = new API();
    await api.authenticate();
    render();
  } catch(e) {
    // If authorizing fails because the credentials are not good, we do this:

    console.log('Was not able to reuse stored credentials.', e);

    localStorage.removeItem('deviceid');
    localStorage.removeItem('privatekey');
    render();
  }
}

document.addEventListener('DOMContentLoaded', () => {
  render();

  credsButton.addEventListener('click', () => {
    credsButton.disabled = true;
    creds().catch(e => {
      credsButton.disabled = false;
      return errorHandler(e);
    });
  });

  sendButton.addEventListener('click', () => {
    sendButton.disabled = true;
    main().catch(e => {
      sendButton.disabled = false;
      return errorHandler(e);
    });
  });
});

async function errorToMessage(e) {
  //  Try to show a more detailed error from the device.
  if (e.response && e.response.headers['Content-Type'] && e.response.headers['Content-Type'].includes('application/json')) {
    try {
      const parsed = await e.response.json();

      if (parsed.message) {
        return `HTTP ${e.response.status} from device. Error ${parsed.error_code}: ${parsed.message}`;
      }
    } catch(e) {
      console.log('Could not load JSON to show detailed error.', e);
    }
  }

  return e.message;
}

function errorHandler(e) {
  console.error(e);
  console.error(e.message);
  console.error(e.stack);

  errorToMessage(e).then(m => { statusText.textContent = m; });

  // Do something special for 401 errors.
  if (e.response && e.response.status == 401) {
    // maybe when there is auth error, we show link to clear credentials & restart app?
    authErrorText.innerHTML = `<br /><a href="#" id="resetLink">Reset credentials.</a>`;
    resetLink.addEventListener('click', (e) => {
      e.preventDefault();
      authErrorText.textContent = '';
      resetCreds().catch(errorHandler);
    });
  }
}
