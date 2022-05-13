const path = require("path");
const dist = path.resolve(__dirname, "dist");
module.exports = {
    mode: "production",
    entry: {
        index: "./src/index.js"
    },
    output: {
        filename: "[name].js"
    },
    devServer: {
        open: true,
        compress: true,
        port: 8080,
    },
    experiments: {
        asyncWebAssembly: true,
        syncWebAssembly: true,
        topLevelAwait: true,
    }
};
