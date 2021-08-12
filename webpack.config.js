const path = require("path");
const CopyPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const dist = path.resolve(__dirname, "dist");

module.exports = {
  mode: "production",
  entry: {
    index: "./js/index.js"
  },
  output: {
    path: dist,
    filename: "[name].js",
  },
  devServer: {
    contentBase: dist,
    host: '0.0.0.0',
  },
  plugins: [
    new CopyPlugin([
      path.resolve(__dirname, "html"),
      path.resolve(__dirname, "js")
    ]),

    new CopyPlugin([{ from: 'static/favicon.ico', to: 'favicon.ico' }]),

    new CopyPlugin([{ from: 'static', to: 'static' }]),

    new WasmPackPlugin({
      crateDirectory: __dirname,
    }),
  ]
};
