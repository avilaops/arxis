# 🚀 ARXIS CLI Tools - Script de Teste Rápido
# Execute: .\TEST_CLI.ps1

Write-Host ""
Write-Host "╔═══════════════════════════════════════════╗" -ForegroundColor Green
Write-Host "║  🚀 ARXIS CLI TOOLS - PRONTO PRA USAR! 🚀  ║" -ForegroundColor Yellow
Write-Host "╚═══════════════════════════════════════════╝" -ForegroundColor Green
Write-Host ""

$CLI_PATH = "examples\practical-cli\target\release"

Write-Host "✅ 4 ferramentas compiladas:" -ForegroundColor Cyan
Write-Host ""

Get-ChildItem "$CLI_PATH\*.exe" | ForEach-Object {
    $sizeMB = "{0:N2} MB" -f ($_.Length / 1MB)
    Write-Host "   📦 $($_.Name) - $sizeMB" -ForegroundColor White
}

Write-Host ""
Write-Host "🧪 Teste Rápido - Compressão:" -ForegroundColor Yellow
Write-Host ""

if (Test-Path "$CLI_PATH\avila-compress-cli.exe") {
    & "$CLI_PATH\avila-compress-cli.exe" compress README.md
    Write-Host ""
}

Write-Host "📍 Localização:" -ForegroundColor Magenta
Write-Host "   $CLI_PATH" -ForegroundColor Cyan
Write-Host ""

Write-Host "📚 Leia mais em:" -ForegroundColor Yellow
Write-Host "   examples\practical-cli\README.md" -ForegroundColor Cyan
Write-Host "   examples\practical-cli\GUIA_DE_USO.md" -ForegroundColor Cyan
Write-Host ""

Write-Host "🎉 SEU PC ESTÁ PRONTO PRA VOAR! 🇧🇷" -ForegroundColor Green
Write-Host ""
