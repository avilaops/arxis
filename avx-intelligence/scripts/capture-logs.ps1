# Script para capturar logs automaticamente das 3 máquinas

param(
    [string]$MachineId = "auto-detect",
    [switch]$CopilotLogs,
    [switch]$TerminalLogs,
    [switch]$MetricsLogs,
    [switch]$All
)

$ErrorActionPreference = "Stop"

# Configuração
$RootPath = "d:\arxis\avx-intelligence"
$LogsPath = "$RootPath\logs"
$Timestamp = Get-Date -Format "yyyyMMdd-HHmmss"

# Detectar máquina
function Get-MachineId {
    $hostname = $env:COMPUTERNAME
    switch -Wildcard ($hostname) {
        "*AVL*" { return "avl-controller" }
        "*AVILA*" { return "avila-runtime" }
        "*ALV*" { return "alv-factory" }
        default { return $hostname.ToLower() }
    }
}

if ($MachineId -eq "auto-detect") {
    $MachineId = Get-MachineId
}

Write-Host "🎯 Capturando logs da máquina: $MachineId" -ForegroundColor Cyan

# Criar estrutura de pastas
$MachinePath = "$LogsPath\machines\$MachineId"
New-Item -ItemType Directory -Force -Path $MachinePath | Out-Null

# Capturar logs do VS Code / Copilot
if ($CopilotLogs -or $All) {
    Write-Host "📝 Capturando logs do Copilot..." -ForegroundColor Yellow

    $CopilotPath = "$LogsPath\copilots\$MachineId"
    New-Item -ItemType Directory -Force -Path $CopilotPath | Out-Null

    # VS Code logs
    $VsCodeLogs = "$env:APPDATA\Code\logs"
    if (Test-Path $VsCodeLogs) {
        $DestPath = "$CopilotPath\vscode-$Timestamp"
        Copy-Item -Path $VsCodeLogs -Destination $DestPath -Recurse -Force
        Write-Host "  ✓ Logs VS Code copiados" -ForegroundColor Green
    }

    # Copilot chat exports (manual - instruções)
    Write-Host "  ℹ Para exportar conversas do Copilot:" -ForegroundColor Blue
    Write-Host "    1. Abra o chat do Copilot" -ForegroundColor Gray
    Write-Host "    2. Use Ctrl+Shift+P > 'Export Chat'" -ForegroundColor Gray
    Write-Host "    3. Salve em: $CopilotPath\chat-$Timestamp.json" -ForegroundColor Gray
}

# Capturar outputs de terminal
if ($TerminalLogs -or $All) {
    Write-Host "🖥️ Capturando logs de terminal..." -ForegroundColor Yellow

    $TerminalPath = "$LogsPath\terminals\$MachineId"
    New-Item -ItemType Directory -Force -Path $TerminalPath | Out-Null

    # PowerShell history
    $PSHistory = "$env:APPDATA\Microsoft\Windows\PowerShell\PSReadLine\ConsoleHost_history.txt"
    if (Test-Path $PSHistory) {
        Copy-Item -Path $PSHistory -Destination "$TerminalPath\powershell-history-$Timestamp.txt"
        Write-Host "  ✓ Histórico PowerShell copiado" -ForegroundColor Green
    }

    # Git logs
    $GitRepos = @("d:\arxis")
    foreach ($repo in $GitRepos) {
        if (Test-Path "$repo\.git") {
            $repoName = Split-Path $repo -Leaf
            git -C $repo log --since="24 hours ago" --pretty=format:"%h - %an, %ar : %s" > "$TerminalPath\git-$repoName-$Timestamp.txt"
            Write-Host "  ✓ Git log de $repoName copiado" -ForegroundColor Green
        }
    }
}

# Capturar métricas da máquina
if ($MetricsLogs -or $All) {
    Write-Host "📊 Capturando métricas do sistema..." -ForegroundColor Yellow

    $MetricsFile = "$MachinePath\metrics-$Timestamp.json"

    $Metrics = @{
        timestamp = $Timestamp
        machine_id = $MachineId
        cpu = @{
            usage_percent = (Get-Counter '\Processor(_Total)\% Processor Time').CounterSamples.CookedValue
            cores = (Get-WmiObject Win32_Processor).NumberOfCores
        }
        memory = @{
            total_gb = [math]::Round((Get-WmiObject Win32_ComputerSystem).TotalPhysicalMemory / 1GB, 2)
            available_gb = [math]::Round((Get-Counter '\Memory\Available MBytes').CounterSamples.CookedValue / 1024, 2)
        }
        disk = @{
            free_gb = [math]::Round((Get-PSDrive D).Free / 1GB, 2)
            used_gb = [math]::Round((Get-PSDrive D).Used / 1GB, 2)
        }
        processes = @{
            vscode_count = (Get-Process -Name "Code" -ErrorAction SilentlyContinue).Count
            cargo_running = (Get-Process -Name "cargo" -ErrorAction SilentlyContinue).Count -gt 0
        }
    }

    $Metrics | ConvertTo-Json -Depth 5 | Out-File $MetricsFile
    Write-Host "  ✓ Métricas salvas em $MetricsFile" -ForegroundColor Green
}

Write-Host "`n✅ Captura concluída!" -ForegroundColor Green
Write-Host "📂 Logs salvos em: $LogsPath" -ForegroundColor Cyan
