class InvariantError extends Error {
  constructor(message) {
    super(message);
    this.name = "InvariantError";
  }
}

export function invariant(pred, msg) {
  if (!pred) {
    throw new InvariantError(msg || '');
  }
}

export function drawBuffer(regl, framebuffer, scale, offset) {
  scale = scale || 1;
  offset = offset || 0;
  return regl({
    frag: `
      precision mediump float;
      uniform sampler2D buf;
      uniform float scale;
      uniform float offset;
      varying vec2 pos;
      void main() {
        gl_FragColor = vec4(texture2D(buf, (pos+1.)/2.).rgb*scale+offset, 1);
      }
    `,
    vert: `
      precision mediump float;
      attribute vec2 inPos;
      varying vec2 pos;
      void main() {
        pos = inPos;
        gl_Position = vec4(pos.x, pos.y, 0, 1);
      }
    `,
    attributes: {
      inPos: regl.buffer([
        [-1, -1],
        [1, -1],
        [-1, 1],
        [-1, 1],
        [1, -1],
        [1, 1],
      ]),
    },
    uniforms: {
      scale,
      offset,
      buf: framebuffer,
    },
    count: 6,
  })();
}
