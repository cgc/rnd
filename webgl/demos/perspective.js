import {mat4,vec3, vec4} from 'gl-matrix';
import {drawBuffer} from './tools.js';

export default function main(regl) {
  const s = 1;
  const floorPositions = [
    [-s, 0, -s],
    [s, 0, -s],
    [-s, 0, s],
    [-s, 0, s],
    [s, 0, -s],
    [s, 0, s],
  ];

  const drawShapes = regl({
    attributes: {
      position: regl.buffer(floorPositions),
    },
    uniforms: {
      im: regl.prop("im"),
      model: regl.prop("model"),
      withPerspective: regl.prop('withPerspective'),
    },
    count: floorPositions.length,
  })

  function makeCheckerboard(color=[0x33, 0x33, 0x33]) {
    const w = 5;
    const h = 5;
    const d = new Uint8Array(w*h*4);
    for (var r = 0; r < h; r++) {
      for (var c = 0; c < w; c++) {
        var idx = 4*(r*w+c);
        const v = ((r+c)%2==0) ? [0xEE, 0xEE, 0xEE] : color
        d[idx+0] = v[0];
        d[idx+1] = v[1];
        d[idx+2] = v[2];
        d[idx+3] = 0xFF;
      }
    }
    return regl.texture({
      width: w,
      height: h,
      data: d,
    })
  }
  const red = makeCheckerboard([0xCC, 0x11, 0x11]);
  const green = makeCheckerboard([0x11, 0xCC, 0x11]);

  const projection = ({viewportWidth, viewportHeight}) =>
    mat4.perspective([], Math.PI / 4, viewportWidth / viewportHeight, 0.01, 30);

  const drawShadow = regl({
    frag: `
      precision mediump float;
      uniform sampler2D im;
      varying vec2 co;

      void main() {
        vec2 tex = co*.5+.5;
        gl_FragColor = texture2D(im, tex).rgba;
      }`,

    vert: `
      precision mediump float;
      attribute vec3 position;
      uniform mat4 projection, view, model;
      uniform bool withPerspective;

      varying vec2 co;

      void main() {
        co = position.xz;
        vec4 p = projection * view * model * vec4(position, 1);
        if (!withPerspective) {
          p = p / p.w;
        }
        gl_Position = p;
      }`,

    uniforms: {
      projection,
      view: ({time}) => {
        const t = 0.6 * time;
        return mat4.lookAt([],
          [5 * Math.sin(t), 2.5, 5],
          [0, 0.0, 0],
          [0, 1, 0]);
      },
    },
  });

  regl.frame(({time}) => {
    regl.clear({ color: [0, 0, 0, 1], depth: 1 });
    drawShadow(() => {
      function model(rot, vec) {
        var m = mat4.identity([]);
        mat4.translate(m, m, vec);
        mat4.rotateY(m, m, rot);
        return m;
      }
      const off = 1.2;
      drawShapes({withPerspective: false, model: model(0, [+off, 0, -off]), im: red});
      drawShapes({withPerspective: false, model: model(Math.PI/2, [+off, 0, +off]), im: red});
      drawShapes({withPerspective: true, model: model(0, [-off, 0, -off]), im: green});
      drawShapes({withPerspective: true, model: model(Math.PI/2, [-off, 0, +off]), im: green});
    });
  });
}
