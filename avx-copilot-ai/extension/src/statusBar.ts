// Status bar manager

import * as vscode from 'vscode';

export class StatusBarManager implements vscode.Disposable {
    private statusBarItem: vscode.StatusBarItem;

    constructor() {
        this.statusBarItem = vscode.window.createStatusBarItem(
            vscode.StatusBarAlignment.Right,
            100
        );
        this.statusBarItem.text = '$(loading~spin) Avila Copilot';
        this.statusBarItem.show();
    }

    setStatus(status: 'initializing' | 'ready' | 'working' | 'error') {
        switch (status) {
            case 'initializing':
                this.statusBarItem.text = '$(loading~spin) Avila Copilot';
                this.statusBarItem.tooltip = 'Initializing...';
                this.statusBarItem.backgroundColor = undefined;
                break;
            case 'ready':
                this.statusBarItem.text = '$(check) Avila Copilot';
                this.statusBarItem.tooltip = 'Ready';
                this.statusBarItem.backgroundColor = undefined;
                break;
            case 'working':
                this.statusBarItem.text = '$(loading~spin) Avila Copilot';
                this.statusBarItem.tooltip = 'Generating...';
                this.statusBarItem.backgroundColor = undefined;
                break;
            case 'error':
                this.statusBarItem.text = '$(error) Avila Copilot';
                this.statusBarItem.tooltip = 'Error occurred';
                this.statusBarItem.backgroundColor = new vscode.ThemeColor('statusBarItem.errorBackground');
                break;
        }
    }

    setLatency(latencyMs: number) {
        const maxLatency = vscode.workspace.getConfiguration('avilacopilot').get<number>('maxLatencyMs', 50);

        if (latencyMs <= maxLatency) {
            this.statusBarItem.tooltip = `Ready (${latencyMs}ms)`;
        } else {
            this.statusBarItem.tooltip = `Ready (${latencyMs}ms - exceeds target)`;
        }
    }

    dispose() {
        this.statusBarItem.dispose();
    }
}
