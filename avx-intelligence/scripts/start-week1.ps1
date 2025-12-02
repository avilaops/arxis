# Script de inicialização da Semana 1
# Execute uma vez em cada máquina na segunda-feira

param(
    [Parameter(Mandatory=$true)]
    [ValidateSet("machine1", "machine2", "machine3")]
    [string]$Machine
)

$ErrorActionPreference = "Stop"

Write-Host "🚀 ARXIS - Inicialização Semana 1" -ForegroundColor Magenta
Write-Host "📅 Data: $(Get-Date -Format 'dd/MM/yyyy HH:mm')" -ForegroundColor Gray
Write-Host "💻 Machine: $Machine" -ForegroundColor Cyan
Write-Host ""

# Diretório base
$baseDir = "d:\arxis"
$intelligenceDir = "$baseDir\avx-intelligence"

# Verificar se está no diretório correto
if (!(Test-Path $baseDir)) {
    Write-Host "❌ Diretório $baseDir não encontrado!" -ForegroundColor Red
    exit 1
}

Set-Location $baseDir

# Função auxiliar
function Step {
    param([string]$Name, [scriptblock]$Action)
    Write-Host "▶ $Name..." -ForegroundColor Yellow
    try {
        & $Action
        Write-Host "  ✓ Concluído" -ForegroundColor Green
    } catch {
        Write-Host "  ✗ Erro: $_" -ForegroundColor Red
        throw
    }
    Write-Host ""
}

# ========================================
# MACHINE 1 - AVL-CONTROLLER
# ========================================
if ($Machine -eq "machine1") {
    Write-Host "🎯 Setup: Coordenador Central" -ForegroundColor Cyan
    Write-Host ""

    Step "1. Atualizar repositório" {
        git pull origin main
    }

    Step "2. Verificar GitHub CLI" {
        $ghVersion = gh --version 2>$null
        if (!$ghVersion) {
            Write-Host "  Instalando GitHub CLI..." -ForegroundColor Yellow
            winget install GitHub.cli
            Write-Host "  Execute gh auth login manualmente" -ForegroundColor Cyan
        } else {
            Write-Host "  GitHub CLI: $($ghVersion[0])" -ForegroundColor Gray
        }
    }

    Step "3. Capturar logs iniciais" {
        & "$intelligenceDir\scripts\capture-logs.ps1" -All
    }

    Step "4. Criar issues (preview)" {
        & "$intelligenceDir\scripts\create-github-issues.ps1" -DryRun -Notebook n1
    }

    Write-Host "📋 Próximos passos manuais:" -ForegroundColor Cyan
    Write-Host "  1. Revisar preview das issues" -ForegroundColor White
    Write-Host "  2. Executar: .\scripts\create-github-issues.ps1 -Notebook n1" -ForegroundColor Gray
    Write-Host "  3. Configurar CI/CD (.github/workflows/ci.yml)" -ForegroundColor White
    Write-Host "  4. Abrir VS Code no workspace correto" -ForegroundColor White
    Write-Host ""
    Write-Host "VS Code:" -ForegroundColor Cyan
    Write-Host "  code $baseDir" -ForegroundColor Gray
}

# ========================================
# MACHINE 2 - AVILA-RUNTIME
# ========================================
elseif ($Machine -eq "machine2") {
    Write-Host "🎯 Setup: Build Engine" -ForegroundColor Cyan
    Write-Host ""

    Step "1. Atualizar repositório" {
        git pull origin main
    }

    Step "2. Verificar Rust toolchain" {
        $rustVersion = rustc --version
        Write-Host "  Rust: $rustVersion" -ForegroundColor Gray

        # Adicionar componentes
        rustup component add clippy rustfmt llvm-tools-preview 2>&1 | Out-Null
        Write-Host "  Componentes: clippy, rustfmt, llvm-tools" -ForegroundColor Gray
    }

    Step "3. Instalar ferramentas de build" {
        $tools = @("cargo-nextest", "cargo-llvm-cov", "cargo-criterion")
        foreach ($tool in $tools) {
            $installed = cargo install --list | Select-String $tool
            if (!$installed) {
                Write-Host "  Instalando $tool..." -ForegroundColor Yellow
                cargo install $tool 2>&1 | Out-Null
            }
        }
        Write-Host "  Ferramentas instaladas" -ForegroundColor Gray
    }

    Step "4. Build inicial (verificação)" {
        cargo check --all
    }

    Step "5. Capturar logs iniciais" {
        & "$intelligenceDir\scripts\capture-logs.ps1" -All
    }

    Write-Host "📋 Próximos passos:" -ForegroundColor Cyan
    Write-Host "  1. Aguardar PRs do Notebook 1" -ForegroundColor White
    Write-Host "  2. Rodar testes quando código chegar" -ForegroundColor White
    Write-Host "  3. Monitorar build metrics" -ForegroundColor White
    Write-Host ""
    Write-Host "VS Codes (quando módulos chegarem):" -ForegroundColor Cyan
    Write-Host "  code $baseDir\avila-core-workspace" -ForegroundColor Gray
}

# ========================================
# MACHINE 3 - ALV-FACTORY
# ========================================
elseif ($Machine -eq "machine3") {
    Write-Host "🎯 Setup: Code Factory" -ForegroundColor Cyan
    Write-Host ""

    Step "1. Atualizar repositório" {
        git pull origin main
    }

    Step "2. Verificar Rust e Copilot" {
        $rustVersion = rustc --version
        Write-Host "  Rust: $rustVersion" -ForegroundColor Gray

        Write-Host "  Copilot: Verificar nas configurações do VS Code" -ForegroundColor Gray
    }

    Step "3. Criar branches de trabalho" {
        # Primeiros 4 módulos da Área 1
        $modules = @("avila-primitives", "avila-error", "avila-id", "avila-time")
        foreach ($mod in $modules) {
            $branchExists = git branch --list "feat/$mod"
            if (!$branchExists) {
                git branch "feat/$mod" 2>&1 | Out-Null
                Write-Host "  Criado: feat/$mod" -ForegroundColor Gray
            }
        }
    }

    Step "4. Capturar logs iniciais" {
        & "$intelligenceDir\scripts\capture-logs.ps1" -All
    }

    Step "5. Preparar workspaces" {
        # Criar estrutura se não existir
        $workspaces = @(
            "$baseDir\avila-core-workspace",
            "$baseDir\avila-ai-workspace",
            "$baseDir\avila-geo-workspace"
        )

        foreach ($ws in $workspaces) {
            if (!(Test-Path $ws)) {
                Write-Host "  ⚠️  Workspace não encontrado: $ws" -ForegroundColor Yellow
            }
        }
    }

    Write-Host "📋 Próximos passos:" -ForegroundColor Cyan
    Write-Host "  1. Abrir 2 VS Codes (workspaces diferentes)" -ForegroundColor White
    Write-Host "  2. Começar desenvolvimento (Terça-feira)" -ForegroundColor White
    Write-Host "  3. Seguir MACHINE3-PLAN.md para prompts" -ForegroundColor White
    Write-Host ""
    Write-Host "VS Code 1 (Área 1 - Primitivos):" -ForegroundColor Cyan
    Write-Host "  code $baseDir\avila-core-workspace" -ForegroundColor Gray
    Write-Host ""
    Write-Host "VS Code 2 (Área 2 - Core Types):" -ForegroundColor Cyan
    Write-Host "  code $baseDir\avila-ai-workspace" -ForegroundColor Gray
    Write-Host ""
    Write-Host "Referências:" -ForegroundColor Cyan
    Write-Host "  - $intelligenceDir\NOTEBOOK1-MANIFESTO.md" -ForegroundColor Gray
    Write-Host "  - $intelligenceDir\WEEK1-KICKOFF.md" -ForegroundColor Gray
    Write-Host "  - $intelligenceDir\machines\MACHINE3-PLAN.md" -ForegroundColor Gray
}

Write-Host ""
Write-Host "✅ Setup da $Machine concluído!" -ForegroundColor Green
Write-Host ""
Write-Host "📊 Logs salvos em:" -ForegroundColor Cyan
Write-Host "  $intelligenceDir\logs\machines\$Machine\" -ForegroundColor Gray
Write-Host ""
Write-Host "📖 Documentação completa:" -ForegroundColor Cyan
$machineName = $Machine.ToUpper()
Write-Host "  $intelligenceDir\machines\$machineName-PLAN.md" -ForegroundColor Gray
Write-Host ""

# Salvar log do setup
$setupLog = @{
    timestamp = Get-Date -Format "o"
    machine = $Machine
    status = "completed"
    rust_version = (rustc --version)
    git_branch = (git branch --show-current)
    git_commit = (git rev-parse --short HEAD)
} | ConvertTo-Json

$timestamp = Get-Date -Format "yyyyMMdd-HHmmss"
$logFile = "$intelligenceDir\logs\machines\$Machine\setup-$timestamp.json"
$setupLog | Out-File -FilePath $logFile -Encoding UTF8

Write-Host "✓ Log de setup salvo: $logFile" -ForegroundColor Green
