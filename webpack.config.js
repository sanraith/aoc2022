const path = require('path');
const webpack = require('webpack');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const WEB_DIR = 'web';

module.exports = {
    mode: 'production',
    entry: path.resolve(__dirname, WEB_DIR, 'app.js'),
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: 'app.js',
    },
    plugins: [
        new HtmlWebpackPlugin({
            template: path.resolve(__dirname, WEB_DIR, 'index.html'),
            favicon: path.resolve(__dirname, WEB_DIR, 'favicon.ico'),
        }),
        new WasmPackPlugin({
            crateDirectory: path.resolve(__dirname, 'aoc-ui'),
            outDir: path.resolve(WEB_DIR, 'pkg')
        })
    ],
    module: {
        rules: [
            {
                test: /\.css$/i,
                use: ['style-loader', 'css-loader'],
            },
        ],
    },
    devServer: {
        watchFiles: [path.resolve(__dirname, WEB_DIR, '*')]
    },
    experiments: {
        asyncWebAssembly: true
    },
    performance: {
        maxAssetSize: 5 * 1024 * 1024,
        maxEntrypointSize: 5 * 1024 * 1024
    }
};
