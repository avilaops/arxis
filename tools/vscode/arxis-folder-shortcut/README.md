# Arxis Folder Shortcut

Extensão local do VS Code que adiciona rapidamente a pasta `D:\GitHub\arxis` ao workspace atual. O comando pode ser acionado a partir de um atalho de teclado (configurado neste repositório) ou pelo palette de comandos.

## Instalação

1. Abra um terminal PowerShell na raiz do repositório `arxis`.
2. Execute o script `tools\vscode\arxis-folder-shortcut\install-extension.ps1` para copiar a extensão para o diretório local de extensões do VS Code.
3. Reinicie o VS Code ou recarregue a janela (`Developer: Reload Window`).

Depois da instalação, o comando **Adicionar pasta Arxis ao workspace** ficará disponível no palette (`Ctrl+Shift+P`).

> 💡 O caminho padrão pode ser ajustado nas configurações (`arxisFolderShortcut.targetPath`) caso a pasta `arxis` esteja em outro local.
