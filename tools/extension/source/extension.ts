import * as vscode from "vscode";
import { LanguageClient, TransportKind } from "vscode-languageclient/node";
import type { ServerOptions, LanguageClientOptions } from "vscode-languageclient/node";

let client: LanguageClient | undefined;

export function activate(context: vscode.ExtensionContext): void {
  const bundledPath = context.asAbsolutePath("pix-language-server");
  const configured = vscode.workspace
    .getConfiguration("pix")
    .get<string>("languageServerPath");
  const command = configured || bundledPath;

  const serverOptions: ServerOptions = {
    run: { command, transport: TransportKind.stdio },
    debug: { command, transport: TransportKind.stdio },
  };

  const clientOptions: LanguageClientOptions = {
    documentSelector: [{ scheme: "file", language: "pix" }],
  };

  client = new LanguageClient(
    "pix-language-server",
    "Pix Language Server",
    serverOptions,
    clientOptions,
  );

  client.start();
}

export function deactivate(): Promise<void> | undefined {
  if (client) {
    return client.stop();
  }
  return undefined;
}
