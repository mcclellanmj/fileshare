var ExtractTextPlugin = require("extract-text-webpack-plugin");

module.exports = {
  entry: './index.js',
  output: {
    path: './dist',
    filename: 'index.js',
    chunkFilename: '[id].js'
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
        test: /.elm$/,
        exclude: [/elm-stuff/, /node_modules/],
        loader: 'elm-webpack'
      },
      {
        test: /stylesheets\/.*elm$/,
        loader: ExtractTextPlugin.extract('style-loader', 'css-loader!elm-css-webpack')
      }
    ],

    noParse: /\.elm$/
  },
  plugins: [ new ExtractTextPlugin("main.css") ],
  devServer: {
    inline: true,
    stats: 'errors-only'
  }
};