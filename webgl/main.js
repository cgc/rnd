import regl from 'regl';

import shadowTriSimple from './demos/shadow-tri-simple.js';
import perspective from './demos/perspective.js';

const demos = {
  shadowTriSimple,
  perspective,
};

let CURRENT_REGL;

function main() {
  const query = new URLSearchParams(window.location.search);
  const demo = demos[query.get('demo')];
  if (demo) {
    teardown();
    CURRENT_REGL = regl({
      extensions: ['OES_texture_float', 'WEBGL_depth_texture'],
    });
    demo(CURRENT_REGL);
  } else {
    let html = '';
    for (const key of Object.keys(demos)) {
      html += `
        <p><a href="?demo=${key}">${key}</a></p>
      `;
    }
    window.demos.innerHTML = html;
    //document.body.insertAdjacentHTML('beforeend', html);
  }
}

function teardown() {
  if (CURRENT_REGL) {
    CURRENT_REGL.destroy();
    CURRENT_REGL = null;
  }
}

main();

if (module.hot) {
  module.hot.dispose(function (data) {
    teardown();
  });
  module.hot.accept(function (getParents) {});
}
