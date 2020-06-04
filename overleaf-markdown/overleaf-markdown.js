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
    addJS('https://cdn.jsdelivr.net/npm/marked/marked.min.js'),
    // HACK should make sure these aren't minified...
    addJS('https://cdn.jsdelivr.net/npm/katex@0.11.1/dist/katex.js'),
    addJS('https://cdn.jsdelivr.net/npm/katex@0.11.1/dist/contrib/auto-render.js'),
];

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

function update(e) {
    e && console.log(e);
    const editor = getAceEditor();
    content.innerHTML = marked(editor.getSession().getDocument().getAllLines().join('\n'));
    for (const img of content.querySelectorAll('img')) {
        img.src = getImage(img.getAttribute('src'));
    }
    try {
        renderMathInElement(content, {
            throwOnError: false,
            // From github.com/KaTeX/KaTeX/issues/712#issuecomment-303618254
            delimiters: [
                {left: "$$", right: "$$", display: true},
                {left: "$", right: "$", display: false},
            ],
        });
    } catch (e) {
        console.log('Math error', e);
    }
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
    const $scope = angular.element(document.querySelector('.ui-layout-pane-west')).scope();
    const root = $scope.rootFolder;
    const fileToEntity = flattenFolderTree(root);
    const entity = fileToEntity[src];
    if (!entity) {
        return;
    }
    return `${document.location.pathname}/file/${entity.id}`;
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
        console.log(target, href);
        const entity = getEntityForHref(href);
        if (entity) {
            console.log(href);
            const $scope = angular.element(document.querySelector('.ui-layout-pane-west')).scope();
            $scope.$emit('entity:selected', entity);
        }
    }
    console.log(e); 
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
