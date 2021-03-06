const path = require('path');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const CopyWebpackPlugin = require('copy-webpack-plugin');

const distPath = path.resolve(__dirname, "dist");
module.exports = (env, argv) => {
    return {
        devServer: {
            historyApiFallback: {
                index:'/'
            },
            contentBase: distPath,
            host: '0.0.0.0',
            port: 8000,
            proxy: {
                '/api': 'http://127.0.0.1:3030'
            }
        },
        entry: './index.js',
        output: {
            path: distPath,
            filename: "tanoshi-web.js",
            webassemblyModuleFilename: "tanoshi-web.wasm",
            publicPath: "/"
        },
        plugins: [
            new CopyWebpackPlugin([
                { from: './static', to: distPath }
            ]),
            new WasmPackPlugin({
                crateDirectory: ".",
                extraArgs: "--no-typescript",
            }),
        ],
        watch: argv.mode !== "production",
        module: {
            rules: [
                {
                    test: /\.css$/,
                    exclude: /node_modules/,
                    use: [
                        {
                            loader: 'style-loader',
                        },
                        {
                            loader: 'css-loader',
                            options: {
                                importLoaders: 1,
                            }
                        },
                        {
                            loader: 'postcss-loader'
                        }
                    ]
                }
            ]
        }
    };
};