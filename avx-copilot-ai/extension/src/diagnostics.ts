// Diagnostics provider for bug detection

import * as vscode from 'vscode';
import { LanguageClient } from 'vscode-languageclient/node';

export class DiagnosticsProvider implements vscode.Disposable {
    private diagnosticCollection: vscode.DiagnosticCollection;
    private disposables: vscode.Disposable[] = [];

    constructor(private client: LanguageClient) {
        this.diagnosticCollection = vscode.languages.createDiagnosticCollection('avilacopilot');

        // Update diagnostics on document change
        this.disposables.push(
            vscode.workspace.onDidChangeTextDocument(async (e) => {
                await this.updateDiagnostics(e.document);
            })
        );

        // Update diagnostics on document open
        this.disposables.push(
            vscode.workspace.onDidOpenTextDocument(async (doc) => {
                await this.updateDiagnostics(doc);
            })
        );
    }

    async updateDiagnostics(document: vscode.TextDocument): Promise<void> {
        const config = vscode.workspace.getConfiguration('avilacopilot');

        if (!config.get<boolean>('enabled') || !config.get<boolean>('bugDetection')) {
            return;
        }

        try {
            const text = document.getText();

            const response = await this.client.sendRequest('textDocument/diagnostic', {
                text
            });

            if (!response || !Array.isArray(response)) {
                return;
            }

            const diagnostics: vscode.Diagnostic[] = response.map((item: any) => {
                const range = new vscode.Range(
                    new vscode.Position(item.range.start.line, item.range.start.character),
                    new vscode.Position(item.range.end.line, item.range.end.character)
                );

                const severity = this.mapSeverity(item.severity);
                const diagnostic = new vscode.Diagnostic(range, item.message, severity);
                diagnostic.source = 'Avila Copilot';

                return diagnostic;
            });

            this.diagnosticCollection.set(document.uri, diagnostics);
        } catch (error) {
            console.error('Error updating diagnostics:', error);
        }
    }

    private mapSeverity(severity: number): vscode.DiagnosticSeverity {
        switch (severity) {
            case 1:
                return vscode.DiagnosticSeverity.Error;
            case 2:
                return vscode.DiagnosticSeverity.Warning;
            case 3:
                return vscode.DiagnosticSeverity.Information;
            case 4:
                return vscode.DiagnosticSeverity.Hint;
            default:
                return vscode.DiagnosticSeverity.Warning;
        }
    }

    dispose() {
        this.diagnosticCollection.dispose();
        this.disposables.forEach(d => d.dispose());
    }
}
