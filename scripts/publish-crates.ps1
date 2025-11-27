# Script de Publicação Automatizada - Workspace Arxis
# Data: 27 de novembro de 2025
# Autor: Nícolas Ávila <nicolas@avila.inc>

param(
    [switch]$DryRun,
    [switch]$SkipTests,
    [ValidateSet('Fase1', 'Fase2', 'Fase3', 'Todas')]
    [string]$Fase = 'Fase1'
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest

# Cores
$ColorSuccess = 'Green'
$ColorError = 'Red'
$ColorWarning = 'Yellow'
$ColorInfo = 'Cyan'

# Banner
function Show-Banner {
    Write-Host "`n╔══════════════════════════════════════════════════════════════════════════════╗" -ForegroundColor $ColorInfo
    Write-Host "║            🚀 PUBLICAÇÃO AUTOMATIZADA - WORKSPACE ARXIS 🚀                   ║" -ForegroundColor $ColorInfo
    Write-Host "╚══════════════════════════════════════════════════════════════════════════════╝`n" -ForegroundColor $ColorInfo
}

# Definição das fases
$fases = @{
    'Fase1' = @(
        @{Nome='avila-dataframe'; Prioridade='CRÍTICA'; Desc='Data science BLOQUEADOR'},
        @{Nome='avila-ml'; Prioridade='CRÍTICA'; Desc='Machine Learning BLOQUEADOR'},
        @{Nome='avx-api-core'; Prioridade='CRÍTICA'; Desc='Tipos API BLOQUEADOR'},
        @{Nome='avx-gateway'; Prioridade='CRÍTICA'; Desc='Gateway BLOQUEADOR'},
        @{Nome='avila-geo'; Prioridade='ALTA'; Desc='Geolocalização'}
    )
    'Fase2' = @(
        @{Nome='avila-reduction'; Prioridade='MÉDIA'; Desc='PCA, t-SNE'},
        @{Nome='avila-tokenizer'; Prioridade='MÉDIA'; Desc='NLP/LLMs'},
        @{Nome='avx-gpu'; Prioridade='MÉDIA'; Desc='Computação GPU'}
    )
    'Fase3' = @(
        @{Nome='avl-loadbalancer'; Prioridade='BAIXA'; Desc='L7 load balancer'},
        @{Nome='avx-quantum-render'; Prioridade='BAIXA'; Desc='Renderer QED experimental'}
    )
}

# Função para verificar se crate já está publicada
function Test-CratePublished {
    param([string]$CrateName)

    try {
        $response = Invoke-RestMethod -Uri "https://crates.io/api/v1/crates/$CrateName" -ErrorAction Stop
        return $true
    } catch {
        return $false
    }
}

# Função para publicar uma crate
function Publish-Crate {
    param([hashtable]$Crate)

    $crateName = $Crate.Nome
    $cratePath = Join-Path $PSScriptRoot $crateName

    Write-Host "`n╔════════════════════════════════════════════════════════════════╗" -ForegroundColor $ColorInfo
    Write-Host "║  Publicando: $crateName" -ForegroundColor $ColorInfo
    Write-Host "║  Prioridade: $($Crate.Prioridade)" -ForegroundColor $ColorInfo
    Write-Host "║  Descrição:  $($Crate.Desc)" -ForegroundColor $ColorInfo
    Write-Host "╚════════════════════════════════════════════════════════════════╝" -ForegroundColor $ColorInfo

    # Verificar se crate já está publicada
    if (Test-CratePublished -CrateName $crateName) {
        Write-Host "⚠️  $crateName já está publicada no crates.io. Pulando..." -ForegroundColor $ColorWarning
        return $true
    }

    # Verificar se diretório existe
    if (-not (Test-Path $cratePath)) {
        Write-Host "❌ Diretório não encontrado: $cratePath" -ForegroundColor $ColorError
        return $false
    }

    Push-Location $cratePath
    try {
        # 1. Verificar Cargo.toml
        Write-Host "`n[1/5] Verificando Cargo.toml..." -ForegroundColor $ColorInfo
        if (-not (Test-Path "Cargo.toml")) {
            Write-Host "❌ Cargo.toml não encontrado em $cratePath" -ForegroundColor $ColorError
            return $false
        }
        Write-Host "✅ Cargo.toml encontrado" -ForegroundColor $ColorSuccess

        # 2. Executar testes
        if (-not $SkipTests) {
            Write-Host "`n[2/5] Executando testes..." -ForegroundColor $ColorInfo
            $testResult = cargo test --all-features 2>&1
            if ($LASTEXITCODE -ne 0) {
                Write-Host "❌ Testes falharam:" -ForegroundColor $ColorError
                Write-Host $testResult
                return $false
            }
            Write-Host "✅ Todos os testes passaram" -ForegroundColor $ColorSuccess
        } else {
            Write-Host "`n[2/5] Pulando testes (--SkipTests)" -ForegroundColor $ColorWarning
        }

        # 3. Executar clippy
        Write-Host "`n[3/5] Executando clippy..." -ForegroundColor $ColorInfo
        $clippyResult = cargo clippy --all-features -- -D warnings 2>&1
        if ($LASTEXITCODE -ne 0) {
            Write-Host "⚠️  Clippy encontrou warnings (continuando mesmo assim)" -ForegroundColor $ColorWarning
        } else {
            Write-Host "✅ Clippy passou sem warnings" -ForegroundColor $ColorSuccess
        }

        # 4. Gerar documentação
        Write-Host "`n[4/5] Gerando documentação..." -ForegroundColor $ColorInfo
        $docResult = cargo doc --no-deps 2>&1
        if ($LASTEXITCODE -ne 0) {
            Write-Host "⚠️  Documentação gerada com warnings" -ForegroundColor $ColorWarning
        } else {
            Write-Host "✅ Documentação gerada com sucesso" -ForegroundColor $ColorSuccess
        }

        # 5. Publicar (ou dry-run)
        if ($DryRun) {
            Write-Host "`n[5/5] Executando dry-run..." -ForegroundColor $ColorInfo
            $publishResult = cargo publish --dry-run 2>&1
            if ($LASTEXITCODE -ne 0) {
                Write-Host "❌ Dry-run falhou:" -ForegroundColor $ColorError
                Write-Host $publishResult
                return $false
            }
            Write-Host "✅ Dry-run bem-sucedido (modo simulação)" -ForegroundColor $ColorSuccess
        } else {
            Write-Host "`n[5/5] Publicando no crates.io..." -ForegroundColor $ColorInfo
            $publishResult = cargo publish 2>&1
            if ($LASTEXITCODE -ne 0) {
                Write-Host "❌ Publicação falhou:" -ForegroundColor $ColorError
                Write-Host $publishResult
                return $false
            }
            Write-Host "✅ $crateName publicado com sucesso!" -ForegroundColor $ColorSuccess

            # Aguardar indexação do crates.io
            Write-Host "`n⏳ Aguardando 30s para indexação do crates.io..." -ForegroundColor $ColorWarning
            Start-Sleep -Seconds 30
        }

        return $true

    } finally {
        Pop-Location
    }
}

# Main
Show-Banner

Write-Host "Configuração:" -ForegroundColor $ColorInfo
Write-Host "  • Modo:        $(if ($DryRun) { 'DRY-RUN (simulação)' } else { 'PUBLICAÇÃO REAL' })" -ForegroundColor $(if ($DryRun) { $ColorWarning } else { $ColorError })
Write-Host "  • Fase:        $Fase" -ForegroundColor $ColorInfo
Write-Host "  • Pular testes: $(if ($SkipTests) { 'SIM' } else { 'NÃO' })" -ForegroundColor $(if ($SkipTests) { $ColorWarning } else { $ColorSuccess })
Write-Host ""

# Determinar quais crates publicar
$cratesToPublish = @()
if ($Fase -eq 'Todas') {
    $cratesToPublish = $fases['Fase1'] + $fases['Fase2'] + $fases['Fase3']
} else {
    $cratesToPublish = $fases[$Fase]
}

Write-Host "🎯 Crates a publicar: $($cratesToPublish.Count)" -ForegroundColor $ColorInfo
foreach ($crate in $cratesToPublish) {
    Write-Host "   • $($crate.Nome) [$($crate.Prioridade)]" -ForegroundColor $ColorInfo
}

# Confirmar se não for dry-run
if (-not $DryRun) {
    Write-Host "`n⚠️  ATENÇÃO: Você está prestes a PUBLICAR $($cratesToPublish.Count) crate(s) no crates.io!" -ForegroundColor $ColorError
    $confirmation = Read-Host "Digite 'SIM' para confirmar"
    if ($confirmation -ne 'SIM') {
        Write-Host "`n❌ Publicação cancelada pelo usuário." -ForegroundColor $ColorWarning
        exit 0
    }
}

# Publicar crates
$successCount = 0
$failedCrates = @()

foreach ($crate in $cratesToPublish) {
    $success = Publish-Crate -Crate $crate
    if ($success) {
        $successCount++
    } else {
        $failedCrates += $crate.Nome
    }
}

# Resumo final
Write-Host "`n╔══════════════════════════════════════════════════════════════════════════════╗" -ForegroundColor $ColorInfo
Write-Host "║                          📊 RESUMO DA PUBLICAÇÃO 📊                          ║" -ForegroundColor $ColorInfo
Write-Host "╚══════════════════════════════════════════════════════════════════════════════╝`n" -ForegroundColor $ColorInfo

Write-Host "Total de crates:   $($cratesToPublish.Count)" -ForegroundColor $ColorInfo
Write-Host "✅ Sucesso:        $successCount" -ForegroundColor $ColorSuccess
Write-Host "❌ Falhas:         $($failedCrates.Count)" -ForegroundColor $(if ($failedCrates.Count -gt 0) { $ColorError } else { $ColorSuccess })

if ($failedCrates.Count -gt 0) {
    Write-Host "`nCrates que falharam:" -ForegroundColor $ColorError
    foreach ($failed in $failedCrates) {
        Write-Host "  • $failed" -ForegroundColor $ColorError
    }
}

if ($DryRun) {
    Write-Host "`n💡 Este foi um DRY-RUN. Execute sem --DryRun para publicar de verdade." -ForegroundColor $ColorWarning
}

if ($successCount -eq $cratesToPublish.Count) {
    Write-Host "`n🎉 TODAS AS CRATES FORAM PUBLICADAS COM SUCESSO!" -ForegroundColor $ColorSuccess
    exit 0
} else {
    Write-Host "`n⚠️  Algumas crates falharam. Verifique os logs acima." -ForegroundColor $ColorError
    exit 1
}
