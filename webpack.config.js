const path = require('path');
const webpack = require('webpack');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const CopyPlugin = require("copy-webpack-plugin");
const WEB_DIR = 'web';

/** @type {webpack.Configuration} */
const base_config = {
    mode: 'production',
    entry: path.resolve(WEB_DIR, 'app.js'),
    output: {
        path: path.resolve('dist'),
        filename: 'app.js',
        clean: true
    },
    plugins: [
        new HtmlWebpackPlugin({
            template: path.resolve(WEB_DIR, 'index.html'),
            favicon: path.resolve(WEB_DIR, 'favicon.ico'),
        }),
        new WasmPackPlugin({
            crateDirectory: path.resolve('aoc-ui'),
            outDir: path.resolve(WEB_DIR, 'pkg')
        }),
        new CopyPlugin({
            patterns: [
                path.resolve(WEB_DIR, '.nojekyll'),
                path.resolve(WEB_DIR, 'favicon.ico'),
                { from: 'input', to: 'input' }
            ]
        })
    ],
    module: {
        rules: [{
            test: /\.css$/i,
            use: ['style-loader', 'css-loader'],
        }]
    },
    devServer: {
        watchFiles: [path.resolve(WEB_DIR, '*')],
    },
    experiments: {
        asyncWebAssembly: true
    },
    performance: {
        maxAssetSize: 5 * 1024 * 1024,
        maxEntrypointSize: 5 * 1024 * 1024
    }
};

/** @type {webpack.Configuration} */
const release_config = {
    mode: 'production',
    ...base_config,
    output: {
        ...base_config.output,
        path: path.resolve('docs'),
    },
};

module.exports = (env, argv) => {
    if (env.mode === 'production') {
        return release_config;
    } else {
        return base_config;
    }
};
