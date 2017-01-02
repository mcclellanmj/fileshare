var path = require('path');

module.exports = {
  entry: './index.js',
  output: {
    path: './dist',
    filename: 'index.js'
  },
  resolve: {
    modulesDirectories: ['node_modules'],
    extensions: ['', '.js', '.elm']
  },
  module: {
    loaders: [
      {
        test: /\.html$/,
        exclude: /node_modules/,
        loader: 'file-loader?name=[name].[ext]'
      },
      {
        test: /.\/src\/.*elm$/,
        exclude: [/elm-stuff/, /node_modules/],
        loader: 'elm-webpack'
      },
      {
        test: /.\/stylesheet\/.*elm$/,
        loader: 'file-loader?name=base.css!elm-css-webpack'
      }
    ],
    noParse: /\.elm$/
  },
  stats: { colors: true },
  devServer: {
    stats: 'errors-only'
  }
};