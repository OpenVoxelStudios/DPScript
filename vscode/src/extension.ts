import * as path from "path";
import { commands, ExtensionContext, extensions, window, workspace } from "vscode";

import {
    LanguageClient,
    LanguageClientOptions,
    ServerOptions,
    TransportKind,
} from "vscode-languageclient/node";

let client: LanguageClient;

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
            window.showErrorMessage(
                `Unsupported CPU architecture: ${arch}`,
            );

            throw new Error(`Unsupported CPU architecture: ${arch}`);
    }

    switch (platform) {
        case "linux":
            return `${cpu}-unknown-linux-${libc}`;

        case "darwin":
            if (arch == "arm" || arch == "ia32") {
                window.showErrorMessage(
                    `Unsupported CPU architecture: ${arch}`,
                );

                throw new Error(`Unsupported CPU architecture: ${arch}`);
            }

            return `${cpu}-apple-darwin`;

        case "win32":
            if (arch == "arm" || arch == "arm64") {
                window.showErrorMessage(
                    `Unsupported CPU architecture: ${arch}`,
                );

                throw new Error(`Unsupported CPU architecture: ${arch}`);
            }

            return `${cpu}-pc-windows-gnu`;

        default:
            window.showErrorMessage(`Unsupported platform: ${platform}`);
            throw new Error(`Unsupported platform: ${platform}`);
    }
};

const restart = async () => {
    if (!client) {
        return window.showErrorMessage(
            "Language client was not created properly!",
        );
    }

    await client.stop();
    await client.start();

    window.showInformationMessage("Language server restarted!");
};

const start = async () => {
    if (!client) {
        return window.showErrorMessage(
            "Language client was not created properly!",
        );
    }

    await client.start();

    window.showInformationMessage("Language server started!");
};

const stop = async () => {
    if (!client) {
        return window.showErrorMessage(
            "Language client was not created properly!",
        );
    }

    await client.stop();

    window.showInformationMessage("Language server stopped!");
};

export function activate(cx: ExtensionContext) {
    cx.subscriptions.push(commands.registerCommand("dpscript.server.start", start));
    cx.subscriptions.push(commands.registerCommand("dpscript.server.restart", restart));
    cx.subscriptions.push(commands.registerCommand("dpscript.server.stop", stop));

    const target = getRustTarget();
    const serverBinary = cx.asAbsolutePath(path.join("bin", target, "dscls"));

    const serverOptions: ServerOptions = {
        run: { command: serverBinary, transport: TransportKind.stdio },

        debug: {
            command: serverBinary,
            transport: TransportKind.stdio,
        },
    };

    const clientOptions: LanguageClientOptions = {
        documentSelector: [{ scheme: "file", language: "dpscript" }],

        synchronize: {
            fileEvents: workspace.createFileSystemWatcher("**/.dscls"),
        },
    };

    client = new LanguageClient(
        "dscls-client",
        "DPScript Language Client",
        serverOptions,
        clientOptions,
    );

    client.start();
}

export function deactivate(): Thenable<void> | undefined {
    if (!client) {
        return undefined;
    }

    return client.stop();
}
