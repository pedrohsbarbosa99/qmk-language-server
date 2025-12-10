import * as path from 'path';
import { workspace, ExtensionContext } from 'vscode';
import {
    LanguageClient,
    LanguageClientOptions,
    ServerOptions,
} from 'vscode-languageclient/node';

let client: LanguageClient;

export function activate(context: ExtensionContext) {
    // Assuming the server is in the target/release directory relative to the client folder
    // when developing heavily. In production, we'd bundle it or expect it in PATH.
    // For this dev setup, we'll try to find it in the target/release folder.

    const serverPath = context.asAbsolutePath(
        path.join('..', 'target', 'release', 'qmk-lsp')
    );
    console.log('QMK LSP Server Path:', serverPath);

    const serverOptions: ServerOptions = {
        run: { command: serverPath },
        debug: { command: serverPath },
    };

    const clientOptions: LanguageClientOptions = {
        documentSelector: [{ scheme: 'file', language: 'c' }, { scheme: 'file', language: 'cpp' }],
        synchronize: {
            fileEvents: workspace.createFileSystemWatcher('**/.clientrc'),
        },
    };

    client = new LanguageClient(
        'qmkLsp',
        'QMK Language Server',
        serverOptions,
        clientOptions
    );

    client.start();
}

export function deactivate(): Thenable<void> | undefined {
    if (!client) {
        return undefined;
    }
    return client.stop();
}
