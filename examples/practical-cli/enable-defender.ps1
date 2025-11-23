# 🛡️ Re-Enable Windows Defender
# Execute como Administrador!

Write-Host ""
Write-Host "╔═══════════════════════════════════════════════╗" -ForegroundColor Green
Write-Host "║  🛡️  REATIVAR WINDOWS DEFENDER  🛡️            ║" -ForegroundColor Yellow
Write-Host "╚═══════════════════════════════════════════════╝" -ForegroundColor Green
Write-Host ""

$isAdmin = ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)

if (-not $isAdmin) {
    Write-Host "❌ ERRO: Execute como Administrador!" -ForegroundColor Red
    pause
    exit 1
}

Write-Host "🔧 Reativando Windows Defender..." -ForegroundColor Cyan

try {
    Set-MpPreference -DisableRealtimeMonitoring $false
    Set-MpPreference -DisableBehaviorMonitoring $false
    Set-MpPreference -DisableIOAVProtection $false
    Set-MpPreference -DisableScriptScanning $false

    Write-Host "  ✅ Real-Time Protection: ATIVADO" -ForegroundColor Green
    Write-Host "  ✅ Behavior Monitoring: ATIVADO" -ForegroundColor Green
    Write-Host "  ✅ IOAV Protection: ATIVADO" -ForegroundColor Green
    Write-Host "  ✅ Script Scanning: ATIVADO" -ForegroundColor Green
    Write-Host ""
    Write-Host "✅ Windows Defender REATIVADO com sucesso!" -ForegroundColor Green
    Write-Host ""

    $status = Get-MpComputerStatus
    Write-Host "📊 Status:" -ForegroundColor Cyan
    Write-Host "   Real-Time Protection: $($status.RealTimeProtectionEnabled)" -ForegroundColor White
    Write-Host ""

} catch {
    Write-Host "❌ ERRO: $($_.Exception.Message)" -ForegroundColor Red
}

pause
