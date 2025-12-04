# Script para rodar o AvilaGIS Desktop
Write-Host "🗺️  AvilaGIS Desktop - Inicializador" -ForegroundColor Cyan
Write-Host "=" * 50

$projectPath = "d:\arxis\avila-gis-desktop"
$exePath = "$projectPath\target\release\avilagis.exe"

Set-Location $projectPath

if (Test-Path $exePath) {
    Write-Host "`n✅ Executável encontrado!" -ForegroundColor Green
    Write-Host "`n🚀 Iniciando AvilaGIS Desktop...`n" -ForegroundColor Yellow

    # Executar em nova janela
    Start-Process $exePath

    Write-Host "✨ Aplicação iniciada com sucesso!" -ForegroundColor Green
    Write-Host "   Verifique a nova janela que se abriu.`n" -ForegroundColor Gray
} else {
    Write-Host "`n⏳ Executável não encontrado. Compilando...`n" -ForegroundColor Yellow

    # Compilar
    cargo build --release

    if ($LASTEXITCODE -eq 0) {
        Write-Host "`n✅ Compilação completa!" -ForegroundColor Green
        Write-Host "`n🚀 Iniciando AvilaGIS Desktop...`n" -ForegroundColor Yellow
        Start-Process $exePath
        Write-Host "✨ Aplicação iniciada com sucesso!" -ForegroundColor Green
    } else {
        Write-Host "`n❌ Erro na compilação!" -ForegroundColor Red
        Write-Host "   Verifique os erros acima.`n" -ForegroundColor Gray
    }
}
