const path = require("path")

const { DefinePlugin } = require("webpack")

const CopyPlugin = require("copy-webpack-plugin")
const { VueLoaderPlugin } = require("vue-loader")
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin")
const TsconfigPathsPlugin = require("tsconfig-paths-webpack-plugin")

const static_folder = path.resolve(__dirname, "static")
const pkg_folder = path.resolve(__dirname, "pkg")
const dist_folder = path.resolve(__dirname, "dist")

const envDefaults = {
  prod: false,
}

module.exports = (env = envDefaults) => ({
  mode: env.prod === true ? "production" : "development",
  devtool: env.prod === true ? "source-map" : "eval-cheap-module-source-map",
  entry: path.resolve(__dirname, "./js/main.ts"),
  experiments: { syncWebAssembly: true },

  output: {
    path: dist_folder,
    publicPath: "/dist/",
  },

  resolve: {
    extensions: [".ts", ".js", ".vue", ".json"],

    alias: {
      vue: "@vue/runtime-dom",
    },

    plugins: [new TsconfigPathsPlugin()],
  },

  module: {
    rules: [
      {
        test: /\.ts$/,

        use: [
          "babel-loader",
          {
            loader: "ts-loader",

            options: {
              appendTsSuffixTo: [/\.vue$/],
            },
          },
        ],
      },
      {
        test: /\.vue$/,
        use: "vue-loader",
      },
      { test: /\.css$/, use: ["vue-style-loader", "css-loader"] },
    ],
  },

  plugins: [
    new CopyPlugin({
      patterns: [{ from: static_folder, to: dist_folder }],
    }),

    new VueLoaderPlugin(),

    new WasmPackPlugin({
      crateDirectory: __dirname,
    }),
    // needed for wasm-pack because bugs

    new DefinePlugin({
      "process.env": {
        NODE_ENV: JSON.stringify(env.prod === true ? "production" : "development"),
      },

      __VUE_OPTIONS_API__: JSON.stringify(false),
      __VUE_PROD_DEVTOOLS__: JSON.stringify(env.prod !== false),
    }),
  ],

  watchOptions: {
    aggregateTimeout: 200,
    poll: 200,
  },
  devServer: {
    // static: './dist'
    // static: { directory: dist_folder, watch: true },
    // static: { directory: pkg_folder, watch: true },
    // static: { directory: static_folder, watch: true },
    // hot: true,
  },
})
