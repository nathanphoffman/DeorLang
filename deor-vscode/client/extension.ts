import * as path from 'path';
import { ExtensionContext, workspace } from 'vscode';
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
  TransportKind,
} from 'vscode-languageclient/node';

let client: LanguageClient;

export function activate(context: ExtensionContext): void {
  const serverModule = context.asAbsolutePath(path.join('out', 'server.js'));

  const serverOptions: ServerOptions = {
    run:   { module: serverModule, transport: TransportKind.ipc },
    debug: { module: serverModule, transport: TransportKind.ipc },
  };

  const clientOptions: LanguageClientOptions = {
    documentSelector: [{ scheme: 'file', language: 'deor' }],
  };

  client = new LanguageClient('deor', 'Deor Language Server', serverOptions, clientOptions);
  client.start();

  if (!context.globalState.get('iconThemeSet')) {
    workspace.getConfiguration().update('workbench.iconTheme', 'deor-icons', true);
    context.globalState.update('iconThemeSet', true);
  }
}

export function deactivate(): Thenable<void> | undefined {
  return client?.stop();
}
