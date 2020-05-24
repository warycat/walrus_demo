const path = require("path");
const CopyPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const HtmlWebpackPlugin = require('html-webpack-plugin');

const dist = path.resolve(__dirname, "dist");

module.exports = {
  mode: "production",
  entry: {
    index: "./js/index.js"
  },
  output: {
    path: dist,
    filename: "[name].js"
  },
  devServer: {
    contentBase: dist,
  },
  plugins: [
    new CopyPlugin([
      path.resolve(__dirname, "static")
    ]),
    new WasmPackPlugin({
      crateDirectory: __dirname,
      extraArgs: "--out-name index"
    }),
    new HtmlWebpackPlugin({
      title: 'Custom template',
      // Load a custom template (lodash by default)
      template: 'index.html'
    })
  ],
  node: {
    fs: 'empty'
  },
  devtool: 'source-map',
  module: {
    rules: [
      {
        test: /\.txt$/i,
        use: 'raw-loader',
      },
    ],
  },
};
