// to let browser code run ok
global.window = {};

const assert = require('assert');
const { _findIndexForOffset } = require('../index');

describe('findIndexForOffset', function() {
  const weights = [.4, .3, .1];
  it('works', function() {
    assert.equal(_findIndexForOffset(.1, weights), 0);
    assert.equal(_findIndexForOffset(.45, weights), 0);
    assert.equal(_findIndexForOffset(.55, weights), 1);
    assert.equal(_findIndexForOffset(.85, weights), 1);
    assert.equal(_findIndexForOffset(.9, weights), 2);
  });
});
