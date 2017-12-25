var path = require('path');

module.exports = {
  entry: {
    app: ['./index.js']
  },

  module: {
    loaders: [
      { test: /\.js$/, exclude: /node_modules/, loader: 'babel-loader' }
    ]
  },

  output: {
    path: path.resolve(__dirname),
    publicPath: '/assets/',
    filename: 'assets/bundle.js'
  }
};
