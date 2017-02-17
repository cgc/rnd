const dim = [400, 275];
const agentDiam = 30;
const agentRadius = agentDiam / 2;
const rayLength = 220;
const rayAngleSpan = 180 / 6; // degrees
const rayCount = 7;
const objectVelocity = 3;
const objectCircleDiam = 30;
const objectCircleRadius = objectCircleDiam / 2;
const objectDiamondSide = 30;
const agentY = dim[1] - 65;
let canvas;

function simpleAgent() {
  let ct = 0;
  function step(rays) {
    return new Promise(function(resolve) {
      setTimeout(function() {
        let objectLeftBound = -1;
        let objectRightBound = -1;
        for (let i = 0; i < rays.length; i++) {
          if (rays[i] != 0) {
            objectLeftBound = i;
            break;
          }
        }
        for (let i = rays.length - 1; i >= 0; i--) {
          if (rays[i] != 0) {
            objectRightBound = i;
            break;
          }
        }

        let args;
        if (objectLeftBound === -1) {
          args = Math.floor(ct / 10) % 2 ? [0, 2] : [2, 0];
        } else {
          const objectCenter = (objectRightBound - objectLeftBound) / 2 + objectLeftBound;
          args = objectCenter < 3 ? [2, 0] :
            objectCenter > 3 ? [0, 2] :
            [0, 0];
        }
        // console.log('simple agent', args, rays);
        ct++;
        resolve(args);
      }, 0);
    });
  }

  function finalize() {}

  return { step, finalize };
}

function nnAgent() {
  const net = new synaptic.Architect.Perceptron(rayCount, 5, 2);
  let trials = [];
  let curr = [];

  function _step(input) {
    const actionProbs = net.activate(input);
    const leftProb = actionProbs[0] / (actionProbs[0] + actionProbs[1]);
    const actionIndex = Math.random() < leftProb ? 0 : 1;
    curr.push({
      input,
      actionProbs,
      actionIndex,
    });
    return Promise.resolve(actionIndex === 0 ? [2, 0] : [0, 2]);
  }

  function step(input) {
    if (agent.watch) {
      return new Promise(function(resolve) {
        setTimeout(function() {
          resolve(_step(input));
        }, 100);
      });
    } else {
      return _step(input);
    }
  }

  function finalize(success) {
    trials.push({ success, moments: curr });
    curr = [];
  }

  function train() {
    for (let i = 0; i < 100; i++) {
      for (const trial of trials) {
        for (const moment of trial.moments) {
          let allZeros = true;
          for (let idx = 0; idx < moment.input.length; idx++) {
            if (moment.input[idx] !== 0) {
              allZeros = false;
            }
          }
          if (allZeros) {
            // HACK does this make sense?
            continue;
          }
          net.activate(moment.input);
          const actionProbs = [1, 1];
          let penalizeIndex;
          // reduce probability of losing action
          // XXX should this be concerned with negative probs?
          if (trial.success) {
            penalizeIndex = moment.actionIndex === 0 ? 1 : 0;
          } else {
            penalizeIndex = moment.actionIndex;
          }
          actionProbs[penalizeIndex] = 0
          net.propagate(.4, actionProbs);
          // console.log('propagate(', 1, JSON.stringify(moment), actionProbs);
        }
      }
    }
    trials = [];
  }

  const agent = {
    net,
    step,
    finalize,
    train,
    testAgent: { step, finalize: function() { curr = []; } },
  };
  return agent;
}

const defaultStyle = {
  strokeColor: 'black',
};

function newGame(p, agentWrapper, objectType, objectOffset) {
  const P = p.Point;
  let agentCenter = new P(dim[0] / 2 + agentRadius, agentY);
  objectOffset = typeof objectOffset === 'undefined' ? Math.random() * 100 - 50 : objectOffset;

  const agentBody = new p.Path.Circle(agentCenter, agentRadius);
  const agent = new p.Group();
  agent.addChild(agentBody);

  const betweenRayAngle = rayAngleSpan / (rayCount - 1);
  const rayEnd = agentCenter.subtract([0, agentRadius + rayLength]);
  const rays = [];
  const middleRay = Math.floor(rayCount / 2);
  for (let idx = 0; idx < rayCount; idx++) {
    const ray = p.Path.Line(agentCenter, rayEnd)
      .rotate((idx - middleRay) * betweenRayAngle, agentCenter);
    rays.push(ray);
    agent.addChild(ray);
    ray.style.dashArray = [2, 6];
  }
  agent.style = defaultStyle;

  const objectCenter = new P(objectOffset, 0).add([agentCenter.x, 0]);
  const object = objectType === 'circle' ? new p.Path.Circle(objectCenter, objectCircleRadius)
    : new p.Path.Rectangle(objectCenter, objectDiamondSide).rotate(45);
  object.style = defaultStyle;

  function doIteration() {
    if (object.bounds.y + object.bounds.height >= agentBody.bounds.y) {
      const collide = agentBody.bounds.x < object.bounds.x + object.bounds.width ||
        object.bounds.x < agentBody.bounds.x + agentBody.bounds.width;
      p.project.clear();
      const result = objectType === 'circle' ? collide : !collide;
      agentWrapper.finalize(result);
      return result;
    }
    object.position = object.position.add([0, objectVelocity]);

    const rayInputs = rays.map(function(ray) {
      // this distance includes the agentRadius, since we are computing
      // from the agent center.
      let minDistance = rayLength + agentRadius;
      const intersections = ray.getIntersections(object);
      for (const i of intersections) {
        const distance = i.point.getDistance(agentBody.position);
        // console.log('hi', object.position, agentBody.position)
        if (distance < minDistance) {
          minDistance = distance;
        }
      }
      if (minDistance < agentRadius) {
        console.error('minDistance is too small', ray, minDistance, agentRadius);
      }
      return 1 - ((minDistance - agentRadius) / rayLength);
    });

    return agentWrapper.step(rayInputs).then(function([left, right]) {
      agent.position = agent.position.add([right - left, 0]);
      return doIteration();
    });
  }

  return doIteration();
}

function promiseSerial(promiseCreators) {
  const results = [];
  function nextPromise() {
    const promise = promiseCreators.shift()();
    return promise.then(function(result) {
      results.push(result);
      if (promiseCreators.length) {
        return nextPromise();
      } else {
        return results;
      }
    });
  }
  return nextPromise();
}

function testNNAgent() {
  function report(msg, games) {
    return games.then((results) => {
      results.length
      let ct = 0;
      for (const r of results) {
        if (r) {
          ct++;
        }
      }
      console.log(msg, ct + ' of ' + results.length + ' were successful');
    });
  }

  const runGames = (a) => {
    const p = [];
    for (let i = 0; i <= 100; i++) {
      p.push(() => newGame(paper, a, 'circle', i - 50));
      p.push(() => newGame(paper, a, 'diamond', i - 50));
    }
    return promiseSerial(p);
  };

	paper.setup(canvas);
  const agent = nnAgent();

  console.log(agent.net.toJSON());
  Promise.resolve().then(() =>
    runGames(agent)).then(() => agent.train()
  ).then(() =>
    runGames(agent)).then(() => agent.train()
  ).then(() =>
    report('after training', runGames(agent.testAgent))
  ).then(() => {
    console.log(agent.net.toJSON());
    agent.watch = true;
    newGame(paper, agent.testAgent, 'diamond', -30).then(function() {
      newGame(paper, agent.testAgent, 'circle', -30);
    });
  });
}

window.onload = function() {
	canvas = document.getElementById('can');
  canvas.width = dim[0];
  canvas.height = dim[1];

  testNNAgent();
  return;

	// Create an empty project and a view for the canvas:
	paper.setup(canvas);
	//paper.setup(dim);

  newGame(paper, simpleAgent(), 'circle').then(function(result) {
    console.log(result);
    newGame(paper, simpleAgent(), 'diamond').then(function(result) {
      console.log(result);
    });
  });
}
