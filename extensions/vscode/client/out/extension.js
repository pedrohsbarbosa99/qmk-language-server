"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.activate = activate;
exports.deactivate = deactivate;
const path = require("path");
const vscode_1 = require("vscode");
const node_1 = require("vscode-languageclient/node");
let client;
function activate(context) {
    // Assuming the server is in the target/release directory relative to the client folder
    // when developing heavily. In production, we'd bundle it or expect it in PATH.
    // For this dev setup, we'll try to find it in the target/release folder.
    const serverPath = context.asAbsolutePath(
        path.join("server", "bin", "qmk-lsp")
    );

    console.log('QMK LSP Server Path:', serverPath);
    const serverOptions = {
        run: { command: serverPath },
        debug: { command: serverPath },
    };
    const clientOptions = {
        documentSelector: [{ scheme: 'file', language: 'c' }, { scheme: 'file', language: 'cpp' }],
        synchronize: {
            fileEvents: vscode_1.workspace.createFileSystemWatcher('**/.clientrc'),
        },
    };
    client = new node_1.LanguageClient('qmkLsp', 'QMK Language Server', serverOptions, clientOptions);
    client.start();
}
function deactivate() {
    if (!client) {
        return undefined;
    }
    return client.stop();
}
//# sourceMappingURL=extension.js.map