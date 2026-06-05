// import { workspace, ExtensionContext, commands } from "vscode";
// import {
//     LanguageClient,
//     LanguageClientOptions,
//     ServerOptions,
//     TransportKind,
// } from "vscode-languageclient/node";

// let client: LanguageClient;

// const startServer = async () => {
//     const serverBin =
//         "/home/redstone/Projects/DPScript/target/release/dpscript-lsp";

//     const serverOpts: ServerOptions = {
//         command: serverBin,
//         transport: TransportKind.stdio,
//     };

//     const clientOpts: LanguageClientOptions = {
//         documentSelector: [{ scheme: "file", language: "dpscript" }],
//         synchronize: {},
//     };

//     client = new LanguageClient(
//         "dpscript-lsp",
//         "DPScript Language Server",
//         serverOpts,
//         clientOpts
//     );

//     await client.start();
// };

// export function activate(cx: ExtensionContext) {
//     startServer();

//     cx.subscriptions.push(
//         commands.registerCommand("dpscript.lsp.restart", async () => {
//             if (client) await client.stop();

//             startServer();
//         })
//     );
// }

// export function deactivate(): Thenable<void> | undefined {
//     if (!client) return undefined;

//     return client.stop();
// }
