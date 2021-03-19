// from https://github.com/JoeKays/dpts

function getIntegerBytes(value) {
    const bytes = [];
    if (value > 0xffffff)
        bytes.push((value >> 24) & 0xff);
    if (value > 0xffff)
        bytes.push((value >> 16) & 0xff);
    if (value > 0xff)
        bytes.push((value >> 8) & 0xff);
    bytes.push(value & 0xff);
    return new Uint8Array(bytes);
}

function removeKeyTags(key) {
    return key.replace(/-+(?:BEGIN|END)( RSA)? PRIVATE KEY-+/g, '');
}

function trimWhitespace(input) {
    return input.replace(/\s/g, '');
}

const fixedBytes = new Uint8Array([
    0x02, 0x01, 0x00, 0x30, 0x0D, 0x06, 0x09, 0x2A,
    0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x01, 0x01,
    0x05, 0x00, 0x04
]);

const PKCS1Converter = {
    toPKCS8(key) {
        const data = key;
        const embeddedLength = data.length;
        const totalLength = embeddedLength + 22;
        const embeddedLengthBytes = getIntegerBytes(embeddedLength);
        const totalLengthBytes = getIntegerBytes(totalLength);
        const outputData = new Uint8Array(totalLength + totalLengthBytes.length + 2);
        let i = 0;
        outputData[i++] = 0x30;
        outputData[i++] = 0x80 | totalLengthBytes.length;
        outputData.set(totalLengthBytes, i);
        i += totalLengthBytes.length;
        outputData.set(fixedBytes, i);
        i += fixedBytes.length;
        outputData[i++] = 0x80 | embeddedLengthBytes.length;
        outputData.set(embeddedLengthBytes, i);
        i += embeddedLengthBytes.length;
        outputData.set(data, i);
        return outputData;
    },

    strToPKCS8Str(keyString) {
      const inval = new Uint8Array(b64.decode(keyString));
      return b64.encode(this.toPKCS8(inval).buffer);
    },
};

function prepareKey(key) {
    let keyPKCS8;
    if (key[0] !== '-') // could be the raw base64 data -> we assume pkcs1 (this is arbitrary)
        keyPKCS8 = PKCS1Converter.strToPKCS8Str(key);
    else if ((key.slice(11, 14) === 'RSA')) // it's a pkcs1 key
        keyPKCS8 = PKCS1Converter.strToPKCS8Str(trimWhitespace(removeKeyTags(key)));
    else if (key.slice(11, 18) === 'PRIVATE') // it's probably already pkcs8
        keyPKCS8 = trimWhitespace(removeKeyTags(key));
    else {
        console.error('Error: Not a valid key format!');
        return null;
    }
    return keyPKCS8;
}
