# Gerenciador de Slots para Janelas do VS Code

Automatiza o posicionamento de novas janelas do VS Code em um grid 2x2 (ou outro layout personalizado). Sempre que o VS Code iniciar — inclusive ao pressionar `Ctrl+Shift+N` — o script monitora os processos `Code.exe`/`Code - Insiders.exe` e encaixa a janela no próximo slot disponível.

## Como funciona

- Usa o PowerShell para monitorar os processos do VS Code.
- Calcula slots com base na área útil do monitor configurado.
- Atribui cada nova janela ao próximo slot livre; se todos estiverem ocupados, reutiliza em ordem circular.
- Pode rodar continuamente em segundo plano (modo padrão) ou apenas alinhar as janelas existentes (`-Once`).

## Instalação rápida

1. Abra um PowerShell **com permissões de administrador** na raiz do repositório `arxis`.
2. Execute:
   ```powershell
   cd tools\windows\vscode-window-slot
   .\register-task.ps1 -Action Install -Force
   ```
   Isso registra uma tarefa agendada que inicia o gerenciador a cada logon (em modo oculto).
3. Efetue logoff/logon ou execute `Ctrl+C` para interromper o VS Code atual e reabri-lo; as novas janelas já passarão a respeitar o layout.

### Teste imediato (sem registrar tarefa)
```powershell
cd tools\windows\vscode-window-slot
powershell -ExecutionPolicy Bypass -File .\Manage-VSCodeWindowSlots.ps1 -Once -Verbose
```
Abra janelas adicionais (`Ctrl+Shift+N`) para validar se cada uma ocupa um slot diferente.

## Personalização

O arquivo `config.json` controla o comportamento:

```jsonc
{
  "monitor": 0,               // Índice do monitor alvo (0 = principal)
  "layout": {
    "columns": 2,
    "rows": 2,
    "padding": 12             // Espaçamento entre janelas em px
  },
  "slotOrder": [0, 1, 2, 3],  // Ordem em que os slots são usados
  "processNames": [
    "Code",
    "Code - Insiders"
  ],
  "pollingIntervalMs": 750    // Intervalo de varredura em milissegundos
}
```

- Para um layout customizado, substitua `layout` por um array `slots` com coordenadas (valores entre 0 e 1 são interpretados como porcentagens da área útil):
  ```jsonc
  "slots": [
    { "x": 0.0, "y": 0.0, "width": 0.5, "height": 0.5 },
    { "x": 0.5, "y": 0.0, "width": 0.5, "height": 0.5 }
  ]
  ```
- Ajuste `slotOrder` para priorizar outras posições.
- Inclua mais nomes em `processNames` se usar builds customizadas do VS Code.

## Remoção / manutenção

- Ver status da tarefa: `.









- Para travar também reposicionamentos manuais, adapte `Update-AllWindows` para chamar `Apply-WindowPosition` mesmo quando a janela já tiver slot atribuído.- A janela volta a ocupar o slot automaticamente apenas na criação; se você mover manualmente, o script não força o retorno para evitar frustrações.- O script usa apenas bibliotecas do Windows/.NET; nada precisa ser instalado além do PowerShell padrão (5.1 ou superior).## Observações- Reinstalar após alterações no script/config: `.
egister-task.ps1 -Action Install -Force`- Remover a automação: `.
egister-task.ps1 -Action Uninstall`egister-task.ps1 -Action Status`
