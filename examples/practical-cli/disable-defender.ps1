# 🛡️ Disable Windows Defender for Performance Testing
# ⚠️  ATENÇÃO: Execute como Administrador!
#
# Uso: .\disable-defender.ps1

Write-Host ""
Write-Host "╔═══════════════════════════════════════════════╗" -ForegroundColor Red
Write-Host "║  🛡️  DESABILITAR WINDOWS DEFENDER (TEMP)  🛡️   ║" -ForegroundColor Yellow
Write-Host "╚═══════════════════════════════════════════════╝" -ForegroundColor Red
Write-Host ""

# Verificar se está rodando como Admin
$isAdmin = ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)

if (-not $isAdmin) {
    Write-Host "❌ ERRO: Este script precisa ser executado como Administrador!" -ForegroundColor Red
    Write-Host ""
    Write-Host "Clique com botão direito no PowerShell e selecione 'Executar como Administrador'" -ForegroundColor Yellow
    Write-Host ""
    pause
    exit 1
}

Write-Host "⚠️  AVISO: Isso vai desabilitar proteção em tempo real!" -ForegroundColor Yellow
Write-Host "   Recomendado APENAS para testes de performance" -ForegroundColor Gray
Write-Host ""

$confirm = Read-Host "Deseja continuar? (S/N)"
if ($confirm -ne "S" -and $confirm -ne "s") {
    Write-Host "Cancelado." -ForegroundColor Yellow
    exit 0
}

Write-Host ""
Write-Host "🔧 Desabilitando Windows Defender..." -ForegroundColor Cyan

try {
    # Desabilitar Real-Time Protection
    Set-MpPreference -DisableRealtimeMonitoring $true
    Write-Host "  ✅ Real-Time Protection: DESABILITADO" -ForegroundColor Green

    # Desabilitar Behavior Monitoring
    Set-MpPreference -DisableBehaviorMonitoring $true
    Write-Host "  ✅ Behavior Monitoring: DESABILITADO" -ForegroundColor Green

    # Desabilitar IOAV Protection (scan de arquivos baixados)
    Set-MpPreference -DisableIOAVProtection $true
    Write-Host "  ✅ IOAV Protection: DESABILITADO" -ForegroundColor Green

    # Desabilitar Script Scanning
    Set-MpPreference -DisableScriptScanning $true
    Write-Host "  ✅ Script Scanning: DESABILITADO" -ForegroundColor Green

    Write-Host ""
    Write-Host "✅ Windows Defender DESABILITADO com sucesso!" -ForegroundColor Green
    Write-Host ""

    # Mostrar status
    $status = Get-MpComputerStatus
    Write-Host "📊 Status Atual:" -ForegroundColor Cyan
    Write-Host "   Real-Time Protection: $($status.RealTimeProtectionEnabled)" -ForegroundColor White
    Write-Host "   Behavior Monitoring:  $($status.BehaviorMonitorEnabled)" -ForegroundColor White
    Write-Host "   IOAV Protection:      $($status.IoavProtectionEnabled)" -ForegroundColor White
    Write-Host ""

    Write-Host "🚀 Agora você pode rodar benchmarks sem interferência!" -ForegroundColor Green
    Write-Host ""
    Write-Host "⚠️  IMPORTANTE: Windows Defender será reativado automaticamente" -ForegroundColor Yellow
    Write-Host "   após reiniciar o computador." -ForegroundColor Gray
    Write-Host ""
    Write-Host "Para reativar manualmente, execute:" -ForegroundColor Cyan
    Write-Host "   .\enable-defender.ps1" -ForegroundColor White
    Write-Host ""

} catch {
    Write-Host ""
    Write-Host "❌ ERRO ao desabilitar Windows Defender:" -ForegroundColor Red
    Write-Host $_.Exception.Message -ForegroundColor Red
    Write-Host ""
    Write-Host "💡 Possíveis soluções:" -ForegroundColor Yellow
    Write-Host "   1. Verifique se está executando como Administrador" -ForegroundColor White
    Write-Host "   2. Algumas empresas bloqueiam isso via Group Policy" -ForegroundColor White
    Write-Host "   3. Tente desabilitar manualmente:" -ForegroundColor White
    Write-Host "      Windows Security > Virus & threat protection > Manage settings" -ForegroundColor Gray
    Write-Host ""
}

pause
