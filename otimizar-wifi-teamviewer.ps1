# Script de Otimização WiFi para TeamViewer
# Execute como Administrador: clique com botão direito > Executar como administrador

Write-Host "=====================================" -ForegroundColor Cyan
Write-Host "  Otimização WiFi para TeamViewer" -ForegroundColor Cyan
Write-Host "=====================================" -ForegroundColor Cyan
Write-Host ""

# Verificar se está executando como administrador
$isAdmin = ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)

if (-not $isAdmin) {
    Write-Host "⚠️  Este script precisa ser executado como ADMINISTRADOR!" -ForegroundColor Red
    Write-Host ""
    Write-Host "Como executar:" -ForegroundColor Yellow
    Write-Host "1. Clique com botão direito no arquivo" -ForegroundColor Yellow
    Write-Host "2. Selecione 'Executar com PowerShell'" -ForegroundColor Yellow
    Write-Host "3. Ou abra PowerShell como Admin e execute: .\otimizar-wifi-teamviewer.ps1" -ForegroundColor Yellow
    Write-Host ""
    pause
    exit 1
}

Write-Host "✅ Executando como Administrador" -ForegroundColor Green
Write-Host ""

# 1. Desabilitar economia de energia do adaptador WiFi
Write-Host "[1/7] Desabilitando economia de energia do WiFi..." -ForegroundColor Yellow
try {
    $adapter = Get-NetAdapter -Name "Wi-Fi" -ErrorAction Stop
    $path = "HKLM:\SYSTEM\CurrentControlSet\Control\Class\{4d36e972-e325-11ce-bfc1-08002be10318}"

    Get-ChildItem $path | ForEach-Object {
        $key = Get-ItemProperty $_.PSPath -ErrorAction SilentlyContinue
        if ($key.DriverDesc -like "*Wireless*" -or $key.DriverDesc -like "*Wi-Fi*") {
            Set-ItemProperty -Path $_.PSPath -Name "PnPCapabilities" -Value 24 -ErrorAction SilentlyContinue
        }
    }

    Write-Host "   ✓ Economia de energia desabilitada" -ForegroundColor Green
} catch {
    Write-Host "   ⚠ Não foi possível desabilitar economia de energia" -ForegroundColor Red
}

# 2. Configurar QoS para priorizar TeamViewer
Write-Host "[2/7] Configurando QoS para TeamViewer..." -ForegroundColor Yellow
try {
    # Habilitar QoS Packet Scheduler
    Set-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Services\Tcpip\QoS" -Name "Do not use NLA" -Value 0 -ErrorAction SilentlyContinue

    # Reservar 0% de largura de banda para sistema (libera para aplicações)
    Set-ItemProperty -Path "HKLM:\SOFTWARE\Policies\Microsoft\Windows\Psched" -Name "NonBestEffortLimit" -Value 0 -Force -ErrorAction SilentlyContinue

    Write-Host "   ✓ QoS configurado" -ForegroundColor Green
} catch {
    Write-Host "   ⚠ Erro ao configurar QoS" -ForegroundColor Red
}

# 3. Otimizar configurações TCP/IP
Write-Host "[3/7] Otimizando TCP/IP..." -ForegroundColor Yellow
try {
    # Auto-tuning level normal (melhor para maioria dos casos)
    netsh int tcp set global autotuninglevel=normal | Out-Null

    # Habilitar RSS (Receive Side Scaling)
    netsh int tcp set global rss=enabled | Out-Null

    # Habilitar ECN (Explicit Congestion Notification)
    netsh int tcp set global ecncapability=enabled | Out-Null

    # Reduzir Initial RTO para conexões mais rápidas
    netsh int tcp set global initialrto=1000 | Out-Null

    # Aumentar tentativas de retransmissão SYN
    netsh int tcp set global maxsynretransmissions=4 | Out-Null

    Write-Host "   ✓ TCP/IP otimizado" -ForegroundColor Green
} catch {
    Write-Host "   ⚠ Erro ao otimizar TCP/IP" -ForegroundColor Red
}

# 4. Configurar MTU ideal
Write-Host "[4/7] Configurando MTU..." -ForegroundColor Yellow
try {
    netsh interface ipv4 set subinterface "Wi-Fi" mtu=1500 store=persistent | Out-Null
    Write-Host "   ✓ MTU configurado para 1500" -ForegroundColor Green
} catch {
    Write-Host "   ⚠ Erro ao configurar MTU" -ForegroundColor Red
}

# 5. Desabilitar limitação de largura de banda do Windows Update
Write-Host "[5/7] Desabilitando limitação do Windows Update..." -ForegroundColor Yellow
try {
    Set-ItemProperty -Path "HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\DeliveryOptimization\Config" -Name "DODownloadMode" -Value 0 -ErrorAction SilentlyContinue
    Write-Host "   ✓ Limitação removida" -ForegroundColor Green
} catch {
    Write-Host "   ⚠ Erro ao configurar Windows Update" -ForegroundColor Red
}

# 6. Configurar DNS para menor latência (Cloudflare + Google)
Write-Host "[6/7] Configurando DNS rápido..." -ForegroundColor Yellow
try {
    Set-DnsClientServerAddress -InterfaceAlias "Wi-Fi" -ServerAddresses ("1.1.1.1", "8.8.8.8") -ErrorAction Stop
    Write-Host "   ✓ DNS configurado (Cloudflare 1.1.1.1 + Google 8.8.8.8)" -ForegroundColor Green
} catch {
    Write-Host "   ⚠ Erro ao configurar DNS" -ForegroundColor Red
}

# 7. Limpar cache DNS e NetBIOS
Write-Host "[7/7] Limpando caches..." -ForegroundColor Yellow
try {
    ipconfig /flushdns | Out-Null
    nbtstat -R | Out-Null
    nbtstat -RR | Out-Null
    Write-Host "   ✓ Caches limpos" -ForegroundColor Green
} catch {
    Write-Host "   ⚠ Erro ao limpar caches" -ForegroundColor Red
}

Write-Host ""
Write-Host "=====================================" -ForegroundColor Cyan
Write-Host "  ✅ Otimização Concluída!" -ForegroundColor Green
Write-Host "=====================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "📊 Resumo das otimizações:" -ForegroundColor Cyan
Write-Host "   • Economia de energia WiFi desabilitada" -ForegroundColor White
Write-Host "   • QoS configurado para priorizar tráfego" -ForegroundColor White
Write-Host "   • TCP/IP otimizado (auto-tuning, RSS, ECN)" -ForegroundColor White
Write-Host "   • MTU ajustado para 1500" -ForegroundColor White
Write-Host "   • DNS alterado para Cloudflare/Google" -ForegroundColor White
Write-Host "   • Caches limpos" -ForegroundColor White
Write-Host ""
Write-Host "🔄 Próximos passos recomendados:" -ForegroundColor Yellow
Write-Host "   1. REINICIE o computador para aplicar todas as mudanças" -ForegroundColor White
Write-Host "   2. Considere conectar ao WiFi 5GHz se disponível" -ForegroundColor White
Write-Host "   3. No TeamViewer, vá em Extras > Opções > Avançado" -ForegroundColor White
Write-Host "      e configure 'Qualidade' para 'Otimizar velocidade'" -ForegroundColor White
Write-Host ""

# Mostrar configurações atuais
Write-Host "📡 Status atual da conexão:" -ForegroundColor Cyan
netsh wlan show interfaces | Select-String "SSID|Sinal|Taxa de"

Write-Host ""
Write-Host "Pressione qualquer tecla para sair..."
$null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")
