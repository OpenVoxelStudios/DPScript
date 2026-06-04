"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.activate = activate;
exports.deactivate = deactivate;
const vscode_1 = require("vscode");
const node_1 = require("vscode-languageclient/node");
let client;
const startServer = async () => {
    const serverBin = "/home/redstone/Projects/DPScript/target/release/dpscript-lsp";
    const serverOpts = {
        command: serverBin,
        transport: node_1.TransportKind.stdio,
    };
    const clientOpts = {
        documentSelector: [{ scheme: "file", language: "dpscript" }],
        synchronize: {},
    };
    client = new node_1.LanguageClient("dpscript-lsp", "DPScript Language Server", serverOpts, clientOpts);
    await client.start();
};
function activate(cx) {
    startServer();
    cx.subscriptions.push(vscode_1.commands.registerCommand("dpscript.lsp.restart", async () => {
        if (client)
            await client.stop();
        startServer();
    }));
}
function deactivate() {
    if (!client)
        return undefined;
    return client.stop();
}
//# sourceMappingURL=extension.js.map