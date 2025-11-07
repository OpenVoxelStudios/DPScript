"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.activate = activate;
exports.deactivate = deactivate;
const path = require("path");
const vscode_1 = require("vscode");
const node_1 = require("vscode-languageclient/node");
let client;
const getRustTarget = () => {
    const { platform, arch } = process;
    let cpu = "x86_64";
    let libc = "gnu";
    switch (arch) {
        case "arm":
            cpu = "arm";
            libc = "gnueabihf";
            break;
        case "arm64":
            cpu = "aarch64";
            break;
        case "ia32":
            cpu = "i686";
            break;
        case "x64":
            break;
        default:
            vscode_1.window.showErrorMessage(`Unsupported CPU architecture: ${arch}`);
            throw new Error(`Unsupported CPU architecture: ${arch}`);
    }
    switch (platform) {
        case "linux":
            return `${cpu}-unknown-linux-${libc}`;
        case "darwin":
            if (arch == "arm" || arch == "ia32") {
                vscode_1.window.showErrorMessage(`Unsupported CPU architecture: ${arch}`);
                throw new Error(`Unsupported CPU architecture: ${arch}`);
            }
            return `${cpu}-apple-darwin`;
        case "win32":
            if (arch == "arm" || arch == "arm64") {
                vscode_1.window.showErrorMessage(`Unsupported CPU architecture: ${arch}`);
                throw new Error(`Unsupported CPU architecture: ${arch}`);
            }
            return `${cpu}-pc-windows-gnu`;
        default:
            vscode_1.window.showErrorMessage(`Unsupported platform: ${platform}`);
            throw new Error(`Unsupported platform: ${platform}`);
    }
};
const restart = async () => {
    if (!client) {
        return vscode_1.window.showErrorMessage("Language client was not created properly!");
    }
    await client.stop();
    await client.start();
    vscode_1.window.showInformationMessage("Language server restarted!");
};
const start = async () => {
    if (!client) {
        return vscode_1.window.showErrorMessage("Language client was not created properly!");
    }
    await client.start();
    vscode_1.window.showInformationMessage("Language server started!");
};
const stop = async () => {
    if (!client) {
        return vscode_1.window.showErrorMessage("Language client was not created properly!");
    }
    await client.stop();
    vscode_1.window.showInformationMessage("Language server stopped!");
};
function activate(cx) {
    cx.subscriptions.push(vscode_1.commands.registerCommand("dpscript.server.start", start));
    cx.subscriptions.push(vscode_1.commands.registerCommand("dpscript.server.restart", restart));
    cx.subscriptions.push(vscode_1.commands.registerCommand("dpscript.server.stop", stop));
    const target = getRustTarget();
    const serverBinary = cx.asAbsolutePath(path.join("bin", target, "dscls"));
    const serverOptions = {
        run: { command: serverBinary, transport: node_1.TransportKind.stdio },
        debug: {
            command: serverBinary,
            transport: node_1.TransportKind.stdio,
        },
    };
    const clientOptions = {
        documentSelector: [{ scheme: "file", language: "dpscript" }],
        synchronize: {
            fileEvents: vscode_1.workspace.createFileSystemWatcher("**/.dscls"),
        },
    };
    client = new node_1.LanguageClient("dscls-client", "DPScript Language Client", serverOptions, clientOptions);
    client.start();
}
function deactivate() {
    if (!client) {
        return undefined;
    }
    return client.stop();
}
//# sourceMappingURL=extension.js.map