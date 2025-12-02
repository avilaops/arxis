# Setup Semana 1 - ARXIS
# Versao simplificada e funcional

param(
    [Parameter(Mandatory=$true)]
    [ValidateSet("machine1", "machine2", "machine3")]
    [string]$Machine
)

$ErrorActionPreference = "Stop"

Write-Host "ARXIS - Inicializacao Semana 1" -ForegroundColor Magenta
Write-Host "Data: $(Get-Date -Format 'dd/MM/yyyy HH:mm')" -ForegroundColor Gray
Write-Host "Machine: $Machine" -ForegroundColor Cyan
Write-Host ""

$baseDir = "d:\arxis"
$intelligenceDir = "$baseDir\avx-intelligence"

if (!(Test-Path $baseDir)) {
    Write-Host "ERRO: Diretorio $baseDir nao encontrado!" -ForegroundColor Red
    exit 1
}

Set-Location $baseDir

function Run-Step {
    param([string]$Name, [scriptblock]$Action)
    Write-Host ">> $Name..." -ForegroundColor Yellow
    try {
        & $Action
        Write-Host "   OK" -ForegroundColor Green
    } catch {
        Write-Host "   ERRO: $_" -ForegroundColor Red
        throw
    }
    Write-Host ""
}

# MACHINE 1 - AVL-CONTROLLER
if ($Machine -eq "machine1") {
    Write-Host "Setup: Coordenador Central" -ForegroundColor Cyan
    Write-Host ""

    Run-Step "1. Atualizar repositorio" {
        git pull origin main
    }

    Run-Step "2. Verificar GitHub CLI" {
        $ghVersion = gh --version 2>$null
        if (!$ghVersion) {
            Write-Host "  Instalando GitHub CLI..." -ForegroundColor Yellow
            winget install GitHub.cli
            Write-Host "  Execute: gh auth login" -ForegroundColor Cyan
        } else {
            Write-Host "  GitHub CLI OK: $($ghVersion[0])" -ForegroundColor Gray
        }
    }

    Run-Step "3. Criar diretorios de logs" {
        $logDir = "$intelligenceDir\logs\machines\$Machine"
        if (!(Test-Path $logDir)) {
            New-Item -ItemType Directory -Path $logDir -Force | Out-Null
        }
    }

    Write-Host "Proximos passos:" -ForegroundColor Cyan
    Write-Host "  1. Executar: .\create-github-issues.ps1 -DryRun -Notebook n1" -ForegroundColor White
    Write-Host "  2. Depois: .\create-github-issues.ps1 -Notebook n1" -ForegroundColor White
    Write-Host "  3. Abrir VS Code: code $baseDir" -ForegroundColor White
}

# MACHINE 2 - AVILA-RUNTIME
elseif ($Machine -eq "machine2") {
    Write-Host "Setup: Build Engine" -ForegroundColor Cyan
    Write-Host ""

    Run-Step "1. Atualizar repositorio" {
        git pull origin main
    }

    Run-Step "2. Verificar Rust" {
        $rustVersion = rustc --version 2>$null
        if ($rustVersion) {
            Write-Host "  Rust: $rustVersion" -ForegroundColor Gray
        } else {
            Write-Host "  AVISO: Rust nao encontrado!" -ForegroundColor Red
        }
    }

    Run-Step "3. Criar diretorios de logs" {
        $logDir = "$intelligenceDir\logs\machines\$Machine"
        if (!(Test-Path $logDir)) {
            New-Item -ItemType Directory -Path $logDir -Force | Out-Null
        }
    }

    Write-Host "Proximos passos:" -ForegroundColor Cyan
    Write-Host "  1. Aguardar PRs do Notebook 1" -ForegroundColor White
    Write-Host "  2. Abrir VS Code quando necessario" -ForegroundColor White
}

# MACHINE 3 - ALV-FACTORY
elseif ($Machine -eq "machine3") {
    Write-Host "Setup: Code Factory" -ForegroundColor Cyan
    Write-Host ""

    Run-Step "1. Atualizar repositorio" {
        git pull origin main
    }

    Run-Step "2. Verificar Rust" {
        $rustVersion = rustc --version 2>$null
        if ($rustVersion) {
            Write-Host "  Rust: $rustVersion" -ForegroundColor Gray
        } else {
            Write-Host "  AVISO: Rust nao encontrado!" -ForegroundColor Red
        }
    }

    Run-Step "3. Criar branches de trabalho" {
        $modules = @("avila-primitives", "avila-error", "avila-id", "avila-time")
        foreach ($mod in $modules) {
            $branchExists = git branch --list "feat/$mod"
            if (!$branchExists) {
                git branch "feat/$mod" 2>&1 | Out-Null
                Write-Host "  Criado: feat/$mod" -ForegroundColor Gray
            }
        }
    }

    Run-Step "4. Criar diretorios de logs" {
        $logDir = "$intelligenceDir\logs\machines\$Machine"
        if (!(Test-Path $logDir)) {
            New-Item -ItemType Directory -Path $logDir -Force | Out-Null
        }
    }

    Write-Host "Proximos passos:" -ForegroundColor Cyan
    Write-Host "  1. Abrir VS Code 1: code $baseDir\avila-core-workspace" -ForegroundColor White
    Write-Host "  2. Abrir VS Code 2: code $baseDir\avila-ai-workspace" -ForegroundColor White
    Write-Host "  3. Comecar desenvolvimento (Terca-feira)" -ForegroundColor White
}

Write-Host ""
Write-Host "Setup da $Machine concluido!" -ForegroundColor Green
Write-Host ""
Write-Host "Logs em: $intelligenceDir\logs\machines\$Machine\" -ForegroundColor Cyan
Write-Host "Docs em: $intelligenceDir\machines\$($Machine.ToUpper())-PLAN.md" -ForegroundColor Cyan

# Salvar log do setup
$setupLog = @{
    timestamp = Get-Date -Format "o"
    machine = $Machine
    status = "completed"
    git_branch = (git branch --show-current)
    git_commit = (git rev-parse --short HEAD)
} | ConvertTo-Json

$timestamp = Get-Date -Format "yyyyMMdd-HHmmss"
$logFile = "$intelligenceDir\logs\machines\$Machine\setup-$timestamp.json"
$setupLog | Out-File -FilePath $logFile -Encoding UTF8

Write-Host ""
Write-Host "Log salvo: $logFile" -ForegroundColor Green
