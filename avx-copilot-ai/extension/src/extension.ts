// Avila Copilot VS Code Extension
// Layer 7: Superior UI/UX for AI-powered code assistance

import * as vscode from 'vscode';
import {
    LanguageClient,
    LanguageClientOptions,
    ServerOptions,
    TransportKind
} from 'vscode-languageclient/node';
import { InlineCompletionProvider } from './completion';
import { DiagnosticsProvider } from './diagnostics';
import { StatusBarManager } from './statusBar';
import { CommandHandler } from './commands';

let client: LanguageClient;
let completionProvider: InlineCompletionProvider;
let diagnosticsProvider: DiagnosticsProvider;
let statusBar: StatusBarManager;
let commandHandler: CommandHandler;

export async function activate(context: vscode.ExtensionContext) {
    console.log('Avila Copilot is activating...');

    // Initialize status bar
    statusBar = new StatusBarManager();
    context.subscriptions.push(statusBar);

    // Start LSP client
    await startLspClient(context);

    // Register inline completion provider
    completionProvider = new InlineCompletionProvider(client);
    context.subscriptions.push(
        vscode.languages.registerInlineCompletionItemProvider(
            { pattern: '**' },
            completionProvider
        )
    );

    // Register diagnostics provider
    diagnosticsProvider = new DiagnosticsProvider(client);
    context.subscriptions.push(diagnosticsProvider);

    // Register commands
    commandHandler = new CommandHandler(client);
    registerCommands(context);

    statusBar.setStatus('ready');
    vscode.window.showInformationMessage('Avila Copilot activated! 🚀');
}

export function deactivate(): Thenable<void> | undefined {
    if (!client) {
        return undefined;
    }
    return client.stop();
}

async function startLspClient(context: vscode.ExtensionContext) {
    const config = vscode.workspace.getConfiguration('avilacopilot');
    let serverPath = config.get<string>('serverPath');

    if (!serverPath) {
        // Use bundled server (development mode: ../target/release/avila-copilot.exe)
        const path = require('path');
        serverPath = path.join(context.extensionPath, '..', 'target', 'release', 'avila-copilot.exe');
    }

    const serverOptions: ServerOptions = {
        command: serverPath!,
        args: []
    };

    const clientOptions: LanguageClientOptions = {
        documentSelector: [
            { scheme: 'file', language: 'rust' },
            { scheme: 'file', language: 'typescript' },
            { scheme: 'file', language: 'javascript' },
            { scheme: 'file', language: 'python' },
            { scheme: 'file', language: 'java' },
            { scheme: 'file', language: 'go' },
            { scheme: 'file', language: 'cpp' },
            { scheme: 'file', language: 'c' }
        ],
        synchronize: {
            fileEvents: vscode.workspace.createFileSystemWatcher('**/*')
        }
    };

    client = new LanguageClient(
        'avilacopilot',
        'Avila Copilot',
        serverOptions,
        clientOptions
    );

    await client.start();
}

function registerCommands(context: vscode.ExtensionContext) {
    context.subscriptions.push(
        vscode.commands.registerCommand('avila-copilot.enable', () => {
            vscode.workspace.getConfiguration('avilacopilot').update('enabled', true, true);
            vscode.window.showInformationMessage('Avila Copilot enabled');
        })
    );

    context.subscriptions.push(
        vscode.commands.registerCommand('avila-copilot.disable', () => {
            vscode.workspace.getConfiguration('avilacopilot').update('enabled', false, true);
            vscode.window.showInformationMessage('Avila Copilot disabled');
        })
    );

    context.subscriptions.push(
        vscode.commands.registerCommand('avila-copilot.generateDocs', async () => {
            await commandHandler.generateDocs();
        })
    );

    context.subscriptions.push(
        vscode.commands.registerCommand('avila-copilot.generateTests', async () => {
            await commandHandler.generateTests();
        })
    );

    context.subscriptions.push(
        vscode.commands.registerCommand('avila-copilot.refactor', async () => {
            await commandHandler.suggestRefactorings();
        })
    );

    context.subscriptions.push(
        vscode.commands.registerCommand('avila-copilot.detectBugs', async () => {
            await commandHandler.detectBugs();
        })
    );
}
