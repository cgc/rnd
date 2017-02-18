const paper = require('paper');
const synaptic = require('synaptic');

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
  const agentV = 2;
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
          args = Math.floor(ct / 10) % 2 ? [0, agentV] : [agentV, 0];
        } else {
          const objectCenter = (objectRightBound - objectLeftBound) / 2 + objectLeftBound;
          args = objectCenter < 3 ? [agentV, 0] :
            objectCenter > 3 ? [0, agentV] :
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

function randomParam() {
  // Initially, a random population of vectors is generated
  // by initializing each component of every individual to
  // random values uniformly distributed over the range +/- 1.
  return Math.random() * 2 - 1;
}

function findIndexForOffset(offset, weights) {
  /*
    offset - can vary from 0 to 1. used to index into weights.
        usually the result of Math.random()
    weights - list of weights for each index
  */
  let total = 0;
  for (const weight of weights) {
    total += weight;
  }

  let prev = 0;
  for (let idx = 0; idx < weights.length; idx++) {
    const normalized = weights[idx] / total;
    if (offset < prev + normalized) {
      return idx;
    }
    prev += normalized;
  }
}
exports._findIndexForOffset = findIndexForOffset;

function nnAgent() {
  const net = new synaptic.Architect.Perceptron(rayCount, 5, 2);
  const exp = [];
  const memoryRate = .2;

  for (const {neuron} of net.neurons()) {
    neuron.bias = randomParam();
    const projected = neuron.connections.projected;
    for (const id in projected) {
      projected[id].weight = randomParam();
    }
  }

  const agentV = 5;
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
    return Promise.resolve(actionIndex === 0 ? [agentV, 0] : [0, agentV]);
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
    const moments = [];
    for (const trial of trials) {
      for (let idx = 0; idx < trial.moments.length; idx++) {
        const moment = trial.moments[idx];
        // the rewardProb varies linearly through the trial, from .55 to .95
        const rewardProb = (idx / (trial.moments.length - 1)) * .4 + .55;

        const expectedOutput = [rewardProb, rewardProb];
        // reduce probability of losing action
        const penalizeIndex = trial.success ?
          (moment.actionIndex === 0 ? 1 : 0) : moment.actionIndex;
        expectedOutput[penalizeIndex] = 1 - rewardProb;
        moment.output = expectedOutput;
        moments.push(moment);
      }
    }
    const expCopy = exp.slice();

    for (const moment of moments) {
      if (Math.random() < memoryRate) {
        if (exp.length < 400) {
          exp.push(moment);
        } else {
          exp[Math.floor(Math.random() * exp.length)] = moment;
        }
      }
    }

    console.log('moments', moments.length, moments[20], 'exp', exp.length);
    return net.trainer.trainAsync(moments.concat(expCopy), {
      log: 200,
      iterations: 3000,
      rate: .2,
    }).then(function(result) {
      console.log('done', result);
      trials = [];
    });
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
    const ob = object.bounds;
    const ab = agentBody.bounds;
    if (ob.y + ob.height >= ab.y) {
      // negate the case where they don't collide
      const collide = !(
        // they don't collide when agent right comes before object left
        ab.x + ab.width < ob.x ||
        // or when object right comes before agent left
        ob.x + ob.width < ab.x);
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
    const width = 100;
    for (let idx = 0; idx < 20; idx++) {
      const offset = Math.floor(Math.random() * width - width / 2);
      p.push(() => newGame(paper, a, 'circle', offset));
      p.push(() => newGame(paper, a, 'diamond', offset));
    }
    return promiseSerial(p);
  };

	paper.setup(canvas);
  const agent = nnAgent();

  console.log(agent.net.toJSON());
  Promise.resolve().then(() =>
    report('before training', runGames(agent.testAgent))
  ).then(() =>
    runGames(agent)).then(() => agent.train()
  ).then(() =>
    runGames(agent)).then(() => agent.train()
  ).then(() =>
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
