function addCSS(href) { ss=document.createElement('link');ss.rel='stylesheet';ss.href=href;document.body.appendChild(ss); }

function addJS(src) {
    return new Promise((resolve, reject) => {
        const ss=document.createElement('script');
        ss.src=src;
        ss.async=false;
        document.body.appendChild(ss);
        ss.addEventListener('load', () => resolve());
        ss.addEventListener('error', () => reject(new Error(`${this.src} failed to load.`)));
    });
}

addCSS('https://cdnjs.cloudflare.com/ajax/libs/github-markdown-css/4.0.0/github-markdown.min.css');
addCSS('https://cdn.jsdelivr.net/npm/katex@0.11.1/dist/katex.min.css');
const sourceLoading = [
    addJS('https://cdnjs.cloudflare.com/ajax/libs/markdown-it/11.0.0/markdown-it.min.js'),
    addJS('https://cdn.jsdelivr.net/npm/katex@0.11.1/dist/katex.min.js'),
];

function divided(items, divider) {
  // Divides a sequence based on a divider.
  let idx = 0;
  const result = [];
  while (idx < items.length) {
    let next = items.indexOf(divider, idx);
    if (next == -1) {
      next = items.length;
    }
    const section = items.slice(idx, next);
    if (section.length) {
      result.push(section);
    }
    idx = next + 1;
  }
  return result;
}

/*
console.log(divided(['', 'f', 'g', '', 'j'], ''))
// should be [['f', 'g'], ['j']]
console.log(divided(['f', 'g', '', '', 'j', 'k', ''], ''))
// should be [['f', 'g'], ['j', 'k']]
*/

function edits(prev, value) {
  function remove(end) {
    for (let remove = previdx; remove < end; remove++) {
      edits.push({op: 'remove', previdx: remove});
    }
  }

  let previdx = 0;
  const edits = [];
  value.forEach((item, idx) => {
    const found = prev.indexOf(item, previdx);

    // If it wasn't there before, simply add it before where we are in previous text.
    if (found == -1) {
      edits.push({op: 'insertBefore', previdx, idx});
      return;
    }

    // If it was there, then we should remove things that have been deleted.
    remove(found);

    // Now move index in prev to last found item.
    previdx = found + 1;
  });

  // Remove all that remain
  remove(prev.length);

  // Edits should be applied in reverse.
  edits.reverse();
  return edits;
}

function applyEditsString(prev, value, edits) {
  // Only for simple arrays... Meant for testing.
  prev = prev.slice();
  for (const edit of edits) {
    if (edit.op == 'insertBefore') {
      prev.splice(edit.previdx, 0, value[edit.idx]);
    } else {
      prev.splice(edit.previdx, 1);
    }
  }
  return prev;
}

/*
const prev = ['f', 'g', 'a', 'b', 'j', 'k'];
let value = ['x', 'g', 'c', 'b', 'j'];
console.log(edits(prev, value));
console.log('matches expected:', JSON.stringify(applyEditsString(prev, value, edits(prev, value)))==JSON.stringify(value));
value = ['f', 'g', 'a', 'b', 'j', 'k', 'h'];
console.log('matches expected:', JSON.stringify(applyEditsString(prev, value, edits(prev, value)))==JSON.stringify(value));
*/

function applyEditsDOM(parent, prev, value, edits, render) {
  const nodes = parent.children;
  if (nodes.length != prev.length) {
    console.log(nodes.length, prev);
    alert('Nodes mismatch!' + navigator.userAgent);
  }

  for (const edit of edits) {
    if (edit.op == 'insertBefore') {
      const n = nodes[edit.previdx];
      if (n) {
        n.insertAdjacentHTML('beforebegin', render(value[edit.idx]));
      } else {
        // Appending if there is no node at that index.
        parent.insertAdjacentHTML('beforeend', render(value[edit.idx]));
      }
    } else {
      nodes[edit.previdx].remove();
    }
  }
}

const content = document.createElement('div');
document.querySelector('.pdf').appendChild(content);
content.classList.add('markdown-body');
Object.assign(content.style, {
    position: 'absolute',
    width: '100%',
    height: '100%',
    background: 'white',
    padding: '1rem 1rem 3rem 1rem',
    overflow: 'scroll',
    boxSizing: 'border-box',
});

function getAceEditor() {
    return ace.edit($('.ace-editor-body')[0]);
}

let prevInputs = [];
function update(e) {
    const md = markdownit({
      html: true,
    });
    math_plugin(md);

    const editor = getAceEditor();
    const lines = editor.getSession().getDocument().getAllLines();

    const inputs = divided(lines, '').map(block => block.join('\n'));
    function render(text) {
        // HAVE TO wrap in a <div> so we can easily index into list of nodes.
        return '<div>'+md.render(text)+'</div>';
    }

    // Compute edits necessary.
    const ed = edits(prevInputs, inputs);

    // Apply edits.
    console.log(`Applying ${ed.length} edits`);
    applyEditsDOM(content, prevInputs, inputs, ed, render);
    // Change image sources to full links.
    for (const img of content.querySelectorAll('img')) {
        const newSrc = getImage(img.getAttribute('src'));
        if (newSrc) {
            img.src = newSrc;
        }
    }

    // Save inputs for next update.
    prevInputs = inputs;
}

function init() {
    const docs = angular.element(document.querySelector('#ide-body')).scope().docs;
    const isMD = docs.find(d => d.doc.selected).path.endsWith('.md');
    const validationProblems = document.querySelector('.pdf-validation-problems');

    if (isMD) {
        content.style.display = 'block';
        validationProblems.style.display = 'none';
        update();
        getAceEditor().getSession().getDocument().on('change', update);
    } else {
        content.style.display = 'none';
        validationProblems.style.display = 'block';
    }
}

Promise.all(sourceLoading).then(() => init());

angular.element(document.querySelector('#editor')).scope().$on('doc:opened', function(e) {
    setTimeout(() => {
        console.log(e);
        init();
    }, 0);
});

function get$scope() {
    return angular.element(document.querySelector('#ide-body')).scope();
}

function getImage(src) {
    // Only process relative URLs
    if (src.startsWith('http')) {
        return;
    }
    const $scope = angular.element(document.querySelector('.ui-layout-pane-west')).scope();
    const root = $scope.rootFolder;
    const fileToEntity = flattenFolderTree(root);
    const entity = fileToEntity[src];
    if (!entity) {
        return;
    }
    const base = [location.protocol, '/', '/', location.host, location.pathname].join('');
    return `${base}/file/${entity.id}`;
}

function getEntityForHref(href) {
    const docs = angular.element(document.querySelector('#ide-body')).scope().docs;
    const doc = docs.find(d => d.path == href);
    return doc && doc.doc;

    // old...
    const $scope = angular.element(document.querySelector('.ui-layout-pane-west')).scope();
    const fileToEntity = flattenFolderTree($scope.rootFolder);
    return fileToEntity[href];
}

content.addEventListener('click', function(e) {
    const target = e.target;
    if (target.tagName == 'A') {
        const href = target.getAttribute('href');
        if (href.startsWith('http')) {
            return; // We only process relative links.
        }
        e.preventDefault();
        const entity = getEntityForHref(href);
        if (entity) {
            const $scope = angular.element(document.querySelector('.ui-layout-pane-west')).scope();
            $scope.$emit('entity:selected', entity);
        }
    }
});

content.addEventListener('dblclick', function(e) {
  let el = e.target;
  if (el == content) {
    return;
  }
  // Find top-level div that this element is in.
  while (el.parentElement !== content) {
    el = el.parentElement;
  }
  // What's the index of the top-level div?
  const idx = Array.from(content.children).indexOf(el);
  // Figure out how many non-empty lines came before it.
  const linesBefore = prevInputs.slice(0, idx).reduce((acc, val) => {
    return acc + val.split('\n').length;
  }, 0);
  const editor = getAceEditor();
  const lines = editor.getSession().getDocument().getAllLines();
  // Now, filter out empty lines & find the line for the target.
  const line = lines.map((l, i) => [l, i]).filter(([l, i]) => l != '')[linesBefore][1];
  // Go to the line! Editor seems to use 1-based indexing.
  editor.gotoLine(line+1);
});

function flattenFolderTree(folder, prefix) {
    /*
     * Returns a flattened version of the files in the folder tree, a dictionary
     * with full path to file as key and file object as value.
     */
    prefix = prefix || '';
    let result = {};
    for (const f of folder.children) {
        if (f.type == 'folder') {
            Object.assign(result, flattenFolderTree(f, prefix + f.name + '/'));
        } else {
            result[prefix+f.name] = f;
        }
    }
    return result;
}
