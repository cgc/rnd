/*
Some questions:
**why does p.z/p.w remove triangle?
I think that's because in the perspective transform, z=w sometimes?
So dividing maps you back to the z=1 plane. So that would erase any depth

**why do we do xy/w but not z/w in the frag shader for drawing?
From above reasoning, the xy/w brings us to the z=1 plane. We need this
to be able to have something in [-1, 1]. On the other hand, z/w would
just be 1, so that removes our ability to do a comparison

** why are the two above true, even though xxx
AHCK HACK ANSWER HACK

**can this be done in world space? If so, how do you map in? I'm wondering
if the point light example relies a lot on the world coordiantes being in [0, 1]
*/
import {mat4,vec3, vec4} from 'gl-matrix';
import {drawBuffer} from './tools.js';

export default function main(regl) {
  console.dir(regl.context())
  const positions = [
    [-1, -1, -4],
    [1, -1, -4],
    [0,  1, -4]
  ];

  const s = 40;
  const floorPositions = [
    [-s, -2, -s],
    [s, -2, -s],
    [-s, -2, s],
    [-s, -2, s],
    [s, -2, -s],
    [s, -2, s],
  ];

  const drawShapes = regl({
    attributes: {
      position: regl.buffer(positions.concat(floorPositions)),
      normal: regl.buffer(positions.map(([x, y, z]) => {
        // normals are a blend of the surface normal and position, convenient
        // since the object is ~centered on the origin
        let v = vec3.create();
        vec3.lerp(v, vec3.fromValues(0, 0, 1), vec3.fromValues(x, y, 0), 0.5);
        vec3.normalize(v, v);
        return v;
      }).concat(floorPositions.map(p => [0, 1, 0]))),
    },
    count: 9,
  })

  const lightPosition = [2, 3, 5]; // in world space
  const lightView = mat4.lookAt([],
    lightPosition,
    [0, 0, 0],
    [0, 1, 0]);

  const shadowBufferSize = 512;
  const shadow = regl.framebuffer({
    radius: shadowBufferSize,
    colorType: 'float',
  });

  const projection = ({viewportWidth, viewportHeight}) =>
    mat4.perspective([], Math.PI / 4, viewportWidth / viewportHeight, 1, 100);
  const lightProjection = projection({viewportWidth: shadowBufferSize, viewportHeight: shadowBufferSize});

  const computeShadow = regl({
      frag: `
        precision mediump float;
        uniform vec3 lightPosition;
        varying vec4 pos;
        varying vec3 wPos;
        void main() {
          //gl_FragColor = vec4(vec3(pos.z/pos.w), 1);
          gl_FragColor = vec4(vec3(length(wPos-lightPosition)), 1);
        }`,

      vert: `
        precision mediump float;
        attribute vec3 position;
        uniform mat4 projection, view;
        varying vec4 pos;
        varying vec3 wPos;
        void main() {
          wPos = position;
          pos = projection * view * vec4(position, 1);
          gl_Position = pos;
        }`,

      uniforms: {
        view: lightView,
        projection: lightProjection,
        lightPosition,
      },
  });

  const drawShadow = regl({
    frag: `
      precision mediump float;
      uniform vec4 ambient, diffuse, specular;

      uniform vec3 lightPosition;
      uniform mat4 view;
      uniform sampler2D shadowMap;

      varying vec4 vPosition;
      varying vec4 vNormal;
      varying vec4 vPositionFromLight;
      varying vec3 wPos;

      void main() {
        vec3 surfaceNormal = normalize(vNormal.xyz);
        vec3 toLight = normalize((view * vec4(lightPosition, 1) - vPosition).xyz);
        vec3 toEye = normalize(-vPosition.xyz);

        float lambert = max(0., dot(toLight, surfaceNormal));
        vec3 bounce = reflect(-toLight, surfaceNormal);
        float specularCoef = pow(max(0., dot(bounce, toEye)), 50.);
        gl_FragColor = ambient + lambert * diffuse + specularCoef * specular;

        vec3 pos = vPositionFromLight.xyz/vPositionFromLight.w;
        vec2 tex = pos.xy*.5 + .5; // in texture coordinates
        //float vPosDepth = pos.z;
        float vPosDepth = length(wPos-lightPosition);

        float mapDepth = texture2D(shadowMap, tex).z;
        bool valid = 0. < tex.x && tex.x < 1. && 0. < tex.y && tex.y < 1.;
        float bias = 1.01;
        if (valid && vPosDepth > mapDepth*bias) {
          gl_FragColor = ambient;
        }
      }`,

    vert: `
      precision mediump float;
      attribute vec3 position;
      attribute vec3 normal;
      uniform mat4 projection;
      uniform mat4 view;
      uniform mat4 lightView;
      uniform mat4 lightProjection;

      varying vec4 vPosition;
      varying vec4 vNormal;
      varying vec4 vPositionFromLight;
      varying vec3 wPos;

      void main() {
        wPos = position;
        vPosition = view * vec4(position, 1);
        gl_Position = projection * vPosition;
        // HACK not fully accurate
        vNormal = view * vec4(normalize(normal), 0);
        vPositionFromLight = (lightProjection * lightView * vec4(position, 1));
      }`,

    uniforms: {
      ambient: [.2, .2, .2, 1],
      diffuse: [.4, .4, .4, 1],
      specular: [.2, .2, .2, 1],
      lightPosition,
      projection,
      lightProjection,
      lightView,
      shadowMap: shadow,
      view: ({time}) => {
        const t = 0.3 * time;
        return mat4.lookAt([],
          [5 * Math.sin(t), 2.5, 5],
          [0, 0.0, 0],
          [0, 1, 0]);
      },
    },
  });
  console.log(lightProjection, lightView)

  regl.frame(({time}) => {
    regl({framebuffer: shadow})(() => {
      regl.clear({ color: [0, 0, 0, 1], depth: 1 });
      computeShadow(drawShapes);
    });
    regl.clear({ color: [0, 0, 0, 1], depth: 1 });
    //drawBuffer(regl, shadow.depth || shadow.depthStencil, 1/2, 1/2);
    //drawBuffer(regl, shadow, 1/30, 1/2);
    //computeShadow(drawShapes);
    drawShadow(drawShapes);
  });
}
