[CmdletBinding()]
param(
    [ValidateSet('Install', 'Uninstall', 'Status', 'RunOnce')]
    [string]$Action = 'Install',
    [string]$TaskName = 'ArxisVSCodeWindowSlots',
    [string]$ScriptPath = "$PSScriptRoot\\Manage-VSCodeWindowSlots.ps1",
    [string]$ConfigPath = "$PSScriptRoot\\config.json",
    [switch]$Force
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

function Resolve-FullPath {
    param([string]$Path)
    if (-not (Test-Path -LiteralPath $Path)) {
        throw "Caminho não encontrado: $Path"
    }
    return (Resolve-Path -LiteralPath $Path).ProviderPath
}

switch ($Action) {
    'Install' {
        $scriptFull = Resolve-FullPath -Path $ScriptPath
        $configFull = Resolve-FullPath -Path $ConfigPath

        $arguments = "-NoProfile -WindowStyle Hidden -ExecutionPolicy Bypass -File `"$scriptFull`" -ConfigPath `"$configFull`""
        $trigger = New-ScheduledTaskTrigger -AtLogOn
        $action = New-ScheduledTaskAction -Execute "powershell.exe" -Argument $arguments

        if (Get-ScheduledTask -TaskName $TaskName -ErrorAction SilentlyContinue) {
            if (-not $Force) {
                throw "A tarefa '$TaskName' já existe. Use -Force para substituir."
            }
            Unregister-ScheduledTask -TaskName $TaskName -Confirm:$false
        }

        Register-ScheduledTask -TaskName $TaskName -Trigger $trigger -Action $action -Description "Garante que janelas do VS Code abram nos slots pré-definidos." -RunLevel Highest | Out-Null
        Write-Host "Tarefa '$TaskName' registrada. As janelas do VS Code serão organizadas automaticamente a cada logon." -ForegroundColor Green
    }
    'Uninstall' {
        if (Get-ScheduledTask -TaskName $TaskName -ErrorAction SilentlyContinue) {
            Unregister-ScheduledTask -TaskName $TaskName -Confirm:$false
            Write-Host "Tarefa '$TaskName' removida." -ForegroundColor Yellow
        } else {
            Write-Host "Nenhuma tarefa chamada '$TaskName' foi encontrada." -ForegroundColor Yellow
        }
    }
    'Status' {
        $task = Get-ScheduledTask -TaskName $TaskName -ErrorAction SilentlyContinue
        if ($null -eq $task) {
            Write-Host "Tarefa '$TaskName' não está registrada." -ForegroundColor Yellow
        } else {
            $task | Get-ScheduledTaskInfo | Format-List
        }
    }
    'RunOnce' {
        $scriptFull = Resolve-FullPath -Path $ScriptPath
        $configFull = Resolve-FullPath -Path $ConfigPath
        Write-Host "Executando script uma única vez para alinhar janelas..." -ForegroundColor Cyan
        Start-Process powershell.exe -ArgumentList "-NoProfile","-ExecutionPolicy","Bypass","-File","$scriptFull","-ConfigPath","$configFull","-Once" -WindowStyle Hidden | Out-Null
    }
}
