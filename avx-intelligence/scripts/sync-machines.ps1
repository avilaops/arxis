# Script para sincronizar dados entre as 3 máquinas

param(
    [string]$SharedPath = "\\AVILA\avx-shared",
    [switch]$Upload,
    [switch]$Download,
    [switch]$Bidirectional
)

$ErrorActionPreference = "Stop"

$RootPath = "d:\arxis\avx-intelligence"
$MachineId = & "$PSScriptRoot\capture-logs.ps1" -MachineId "auto-detect"

Write-Host "🔄 Sincronizando máquina: $MachineId" -ForegroundColor Cyan

# Verificar se caminho compartilhado existe
if (-not (Test-Path $SharedPath)) {
    Write-Host "⚠️ Caminho compartilhado não encontrado: $SharedPath" -ForegroundColor Yellow
    Write-Host "Criando estrutura local..." -ForegroundColor Yellow
    $SharedPath = "$RootPath\..\avx-shared"
    New-Item -ItemType Directory -Force -Path $SharedPath | Out-Null
}

# Estrutura de pastas compartilhadas
$SharedFolders = @{
    queue = "$SharedPath\queue"
    results = "$SharedPath\results"
    logs = "$SharedPath\logs"
    prompts = "$SharedPath\prompts"
    modules = "$SharedPath\modules"
}

foreach ($folder in $SharedFolders.Values) {
    New-Item -ItemType Directory -Force -Path $folder | Out-Null
}

# Upload local → compartilhado
if ($Upload -or $Bidirectional) {
    Write-Host "⬆️ Enviando dados locais..." -ForegroundColor Yellow
    
    # Logs
    $LocalLogs = "$RootPath\logs\machines\$MachineId"
    $RemoteLogs = "$($SharedFolders.logs)\$MachineId"
    if (Test-Path $LocalLogs) {
        Copy-Item -Path $LocalLogs -Destination $RemoteLogs -Recurse -Force
        Write-Host "  ✓ Logs enviados" -ForegroundColor Green
    }
    
    # Resultados
    $LocalResults = "$RootPath\analytics\results"
    if (Test-Path $LocalResults) {
        Copy-Item -Path "$LocalResults\*" -Destination $SharedFolders.results -Force
        Write-Host "  ✓ Resultados enviados" -ForegroundColor Green
    }
}

# Download compartilhado → local
if ($Download -or $Bidirectional) {
    Write-Host "⬇️ Baixando dados compartilhados..." -ForegroundColor Yellow
    
    # Prompts globais
    $RemotePrompts = "$($SharedFolders.prompts)\*"
    $LocalPrompts = "$RootPath\prompts"
    New-Item -ItemType Directory -Force -Path $LocalPrompts | Out-Null
    Copy-Item -Path $RemotePrompts -Destination $LocalPrompts -Force -ErrorAction SilentlyContinue
    
    # Fila de tarefas
    $QueueFiles = Get-ChildItem "$($SharedFolders.queue)" -Filter "*.$MachineId.task" -ErrorAction SilentlyContinue
    if ($QueueFiles) {
        Write-Host "  📋 $($QueueFiles.Count) tarefas encontradas" -ForegroundColor Cyan
    }
}

Write-Host "`n✅ Sincronização concluída!" -ForegroundColor Green
