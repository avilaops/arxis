// Inline completion provider

import * as vscode from 'vscode';
import { LanguageClient } from 'vscode-languageclient/node';

export class InlineCompletionProvider implements vscode.InlineCompletionItemProvider {
    constructor(private client: LanguageClient) {}

    async provideInlineCompletionItems(
        document: vscode.TextDocument,
        position: vscode.Position,
        context: vscode.InlineCompletionContext,
        token: vscode.CancellationToken
    ): Promise<vscode.InlineCompletionItem[] | vscode.InlineCompletionList> {
        const config = vscode.workspace.getConfiguration('avilacopilot');

        if (!config.get<boolean>('enabled') || !config.get<boolean>('autoComplete')) {
            return [];
        }

        try {
            const startTime = Date.now();

            // Get text before cursor
            const textBeforeCursor = document.getText(
                new vscode.Range(new vscode.Position(0, 0), position)
            );

            // Request completion from LSP
            const response = await this.client.sendRequest('textDocument/completion', {
                text: textBeforeCursor,
                position: document.offsetAt(position)
            });

            const latency = Date.now() - startTime;
            const maxLatency = config.get<number>('maxLatencyMs', 50);

            if (latency > maxLatency) {
                console.warn(`Completion latency ${latency}ms exceeds target ${maxLatency}ms`);
            }

            if (!response || !Array.isArray(response)) {
                return [];
            }

            return response.map((item: any) => {
                const completionItem = new vscode.InlineCompletionItem(item.label);
                completionItem.range = new vscode.Range(position, position);
                return completionItem;
            });
        } catch (error) {
            console.error('Error getting completion:', error);
            return [];
        }
    }
}
