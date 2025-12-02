#!/usr/bin/env pwsh
# Script Simplificado de Publicação - Avila Cloud
# Versão: 1.0 - Funcional e Direto

Write-Host "`n╔══════════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║   🌩️  AVILA CLOUD - PUBLICAÇÃO NO CRATES.IO  🌩️  ║" -ForegroundColor Cyan
Write-Host "╚══════════════════════════════════════════════════╝`n" -ForegroundColor Cyan

$dependencies = @(
    @{
        Name = "avila-error-derive"
        Path = "d:\arxis\avila-error\avila-error-derive"
    },
    @{
        Name = "avila-error"
        Path = "d:\arxis\avila-error"
    },
    @{
        Name = "avx-gateway"
        Path = "d:\arxis\avx-gateway"
    },
    @{
        Name = "avl-loadbalancer"
        Path = "d:\arxis\avl-loadbalancer"
    }
)

Write-Host "📦 Dependências a publicar:" -ForegroundColor Yellow
foreach ($dep in $dependencies) {
    Write-Host "   • $($dep.Name)" -ForegroundColor White
}

Write-Host "`n⚠️  Pressione ENTER para continuar ou CTRL+C para cancelar..." -ForegroundColor Magenta
Read-Host

$successCount = 0
$failedCount = 0
$skippedCount = 0

foreach ($dep in $dependencies) {
    Write-Host "`n========================================" -ForegroundColor Cyan
    Write-Host "📦 Processando: $($dep.Name)" -ForegroundColor Yellow
    Write-Host "========================================" -ForegroundColor Cyan

    if (-not (Test-Path $dep.Path)) {
        Write-Host "❌ Diretório não encontrado: $($dep.Path)" -ForegroundColor Red
        $failedCount++
        continue
    }

    Push-Location $dep.Path

    # Verificar se já está publicado
    Write-Host "🔍 Verificando status no crates.io..." -ForegroundColor Cyan
    $searchResult = cargo search "^$($dep.Name)`$" --limit 1 2>&1 | Out-String

    if ($searchResult -match "^$($dep.Name)\s*=") {
        Write-Host "⚠️  Pacote já publicado no crates.io" -ForegroundColor Yellow

        $response = Read-Host "   Publicar mesmo assim? (S/N)"
        if ($response -ne 'S' -and $response -ne 's') {
            Write-Host "⏭️  Pulando $($dep.Name)" -ForegroundColor Yellow
            Pop-Location
            $skippedCount++
            continue
        }
    }

    # Validar com dry-run
    Write-Host "🧪 Validando pacote (dry-run)..." -ForegroundColor Cyan
    $dryOutput = cargo publish --dry-run --allow-dirty 2>&1 | Out-String

    if ($LASTEXITCODE -ne 0) {
        Write-Host "❌ Validação falhou!" -ForegroundColor Red
        Write-Host $dryOutput -ForegroundColor Red
        Pop-Location
        $failedCount++
        continue
    }

    Write-Host "✅ Validação OK" -ForegroundColor Green

    # Publicar
    Write-Host "🚀 Publicando no crates.io..." -ForegroundColor Cyan
    $pubOutput = cargo publish --allow-dirty 2>&1 | Out-String

    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ $($dep.Name) publicado com sucesso!" -ForegroundColor Green
        $successCount++
    } else {
        Write-Host "❌ Falha ao publicar $($dep.Name)" -ForegroundColor Red
        Write-Host $pubOutput -ForegroundColor Red
        $failedCount++

        # Verificar rate limit
        if ($pubOutput -match "429 Too Many Requests") {
            Write-Host "`n⚠️  RATE LIMIT ATINGIDO!" -ForegroundColor Magenta
            Write-Host "Aguarde o período especificado e tente novamente." -ForegroundColor Yellow
            Pop-Location
            break
        }
    }

    Pop-Location

    # Aguardar entre publicações
    if ($successCount -lt $dependencies.Count) {
        Write-Host "`n⏳ Aguardando 10 segundos..." -ForegroundColor Yellow
        Start-Sleep -Seconds 10
    }
}

# Resumo
Write-Host "`n╔══════════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║              📊 RESUMO DA PUBLICAÇÃO             ║" -ForegroundColor Cyan
Write-Host "╚══════════════════════════════════════════════════╝`n" -ForegroundColor Cyan

Write-Host "✅ Publicados: $successCount" -ForegroundColor Green
Write-Host "❌ Falharam: $failedCount" -ForegroundColor Red
Write-Host "⏭️  Pulados: $skippedCount" -ForegroundColor Yellow
Write-Host "📦 Total: $($dependencies.Count)`n" -ForegroundColor White

if ($successCount -gt 0) {
    Write-Host "🎉 Pacotes publicados com sucesso!" -ForegroundColor Green
    Write-Host "`nPróximo passo:" -ForegroundColor Cyan
    Write-Host "  Atualize o Cargo.toml do avila-cloud para usar as versões do crates.io`n" -ForegroundColor White
}

if ($failedCount -gt 0) {
    Write-Host "⚠️  Alguns pacotes falharam. Revise os erros acima.`n" -ForegroundColor Yellow
    exit 1
}

exit 0
