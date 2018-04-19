const path = require("path");
const webpack = require("webpack");

const isProd = process.env.NODE_ENV === "production";

const outputDir = path.join(__dirname, "dist/");
const srcDir = path.join(__dirname, "ui/");

module.exports = {
  entry: "./ui/Main.bs.js",
  mode: isProd ? "production" : "development",
  output: {
    path: outputDir,
    filename: "main.js"
  },
  plugins: [new webpack.HotModuleReplacementPlugin()],
  devServer: {
    stats: "minimal",
    inline: true,
    hot: true,
    open: true,
    contentBase: srcDir,
    proxy: {
      "/api": "http://127.0.0.1:50545"
    }
  }
};
