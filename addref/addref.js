require('@citation-js/plugin-bibtex');
require('@citation-js/plugin-zotero-translation-server');
const { plugins } = require('@citation-js/core');
const {Cite} = require('@citation-js/core');
const fs = require('fs');

plugins.config.get('@zotero').host = 'https://t0guvf0w17.execute-api.us-east-1.amazonaws.com/Prod';

async function addref(dest, query) {
    const ref = await Cite.async(query);
    const output = ref.format('bibtex', { lang: 'en-US' });
    if (fs.existsSync(dest)) {
        const data = fs.readFileSync(dest);
        if (data.indexOf(output) !== -1) {
            console.log('Already in references.');
            process.exit(-1);
        }
    }
    console.log('Adding', output);
    const dt = new Date().toISOString();
    fs.appendFileSync(dest, `% ${dt} from ${query}\n${output}\n`);
}

const dest = process.argv[2];
const q = process.argv[3];
if (!q) {
    console.log('Usage: addref $REFERENCES_FILE $QUERY');
    process.exit(-1);
}
addref(dest, q).catch(function(e) {
    console.log(`Error when saving '${q}': ${e.stack}`);
});

