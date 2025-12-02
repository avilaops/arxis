// Command handler

import * as vscode from 'vscode';
import { LanguageClient } from 'vscode-languageclient/node';

export class CommandHandler {
    constructor(private client: LanguageClient) {}

    async generateDocs(): Promise<void> {
        const editor = vscode.window.activeTextEditor;
        if (!editor) {
            vscode.window.showErrorMessage('No active editor');
            return;
        }

        try {
            const document = editor.document;
            const text = document.getText();

            vscode.window.withProgress({
                location: vscode.ProgressLocation.Notification,
                title: 'Generating documentation...',
                cancellable: false
            }, async () => {
                const response = await this.client.sendRequest('avila/generateDocs', { text });

                if (response && typeof response === 'string') {
                    // Insert documentation above current position
                    const position = editor.selection.active;
                    await editor.edit(editBuilder => {
                        editBuilder.insert(position, response + '\n');
                    });

                    vscode.window.showInformationMessage('Documentation generated!');
                }
            });
        } catch (error) {
            vscode.window.showErrorMessage(`Error generating documentation: ${error}`);
        }
    }

    async generateTests(): Promise<void> {
        const editor = vscode.window.activeTextEditor;
        if (!editor) {
            vscode.window.showErrorMessage('No active editor');
            return;
        }

        try {
            const document = editor.document;
            const text = document.getText();

            vscode.window.withProgress({
                location: vscode.ProgressLocation.Notification,
                title: 'Generating tests...',
                cancellable: false
            }, async () => {
                const response = await this.client.sendRequest('avila/generateTests', { text });

                if (response && typeof response === 'string') {
                    // Create new test file or append to current
                    const testFileName = this.getTestFileName(document.fileName);
                    const testUri = vscode.Uri.file(testFileName);

                    await vscode.workspace.fs.writeFile(
                        testUri,
                        Buffer.from(response, 'utf8')
                    );

                    const testDoc = await vscode.workspace.openTextDocument(testUri);
                    await vscode.window.showTextDocument(testDoc);

                    vscode.window.showInformationMessage('Tests generated!');
                }
            });
        } catch (error) {
            vscode.window.showErrorMessage(`Error generating tests: ${error}`);
        }
    }

    async suggestRefactorings(): Promise<void> {
        const editor = vscode.window.activeTextEditor;
        if (!editor) {
            vscode.window.showErrorMessage('No active editor');
            return;
        }

        try {
            const document = editor.document;
            const text = document.getText();

            const response = await this.client.sendRequest('textDocument/codeAction', { text });

            if (!response || !Array.isArray(response) || response.length === 0) {
                vscode.window.showInformationMessage('No refactorings suggested');
                return;
            }

            // Show quick pick with refactoring options
            const items = response.map((action: any) => ({
                label: action.title,
                description: action.kind,
                action
            }));

            const selected = await vscode.window.showQuickPick(items, {
                placeHolder: 'Select refactoring to apply'
            });

            if (selected) {
                vscode.window.showInformationMessage(`Selected: ${selected.label}`);
                // TODO: Apply refactoring
            }
        } catch (error) {
            vscode.window.showErrorMessage(`Error suggesting refactorings: ${error}`);
        }
    }

    async detectBugs(): Promise<void> {
        const editor = vscode.window.activeTextEditor;
        if (!editor) {
            vscode.window.showErrorMessage('No active editor');
            return;
        }

        vscode.window.showInformationMessage('Bug detection runs automatically. Check the Problems panel.');
    }

    private getTestFileName(fileName: string): string {
        const ext = fileName.substring(fileName.lastIndexOf('.'));
        const base = fileName.substring(0, fileName.lastIndexOf('.'));
        return `${base}.test${ext}`;
    }
}
