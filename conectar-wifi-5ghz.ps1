# Script para conectar ao WiFi 5GHz
# Este script tenta conectar ao mesmo roteador mas na banda 5GHz

Write-Host "=====================================" -ForegroundColor Cyan
Write-Host "  Conectar ao WiFi 5GHz" -ForegroundColor Cyan
Write-Host "=====================================" -ForegroundColor Cyan
Write-Host ""

# Mostrar redes disponíveis
Write-Host "📡 Redes WiFi 5GHz disponíveis:" -ForegroundColor Yellow
Write-Host ""

$networks = netsh wlan show networks mode=bssid | Out-String

# Filtrar redes 5GHz
if ($networks -match "Don Branco Barbearia") {
    Write-Host "✅ Rede 'Don Branco Barbearia' detectada" -ForegroundColor Green

    # Verificar se há 5GHz disponível
    if ($networks -match "5 GHz") {
        Write-Host "✅ Banda 5GHz disponível (BSSID: 44:89:6d:62:b6:4e)" -ForegroundColor Green
        Write-Host ""
        Write-Host "⚠️  ATENÇÃO: O sinal 5GHz é mais fraco (25%) mas mais rápido e estável" -ForegroundColor Yellow
        Write-Host ""

        $confirm = Read-Host "Deseja tentar conectar ao 5GHz? (S/N)"

        if ($confirm -eq "S" -or $confirm -eq "s") {
            Write-Host ""
            Write-Host "🔄 Tentando conectar..." -ForegroundColor Yellow

            # Desconectar da rede atual
            netsh wlan disconnect | Out-Null
            Start-Sleep -Seconds 2

            # Reconectar (o Windows deve preferir 5GHz se configurado)
            netsh wlan connect name="Don Branco Barbearia" | Out-Null
            Start-Sleep -Seconds 5

            # Verificar resultado
            $status = netsh wlan show interfaces | Out-String

            if ($status -match "5 GHz") {
                Write-Host "✅ Conectado ao 5GHz com sucesso!" -ForegroundColor Green
            } elseif ($status -match "2,4 GHz") {
                Write-Host "⚠️  Ainda conectado ao 2.4GHz" -ForegroundColor Yellow
                Write-Host ""
                Write-Host "O Windows pode preferir 2.4GHz devido ao sinal mais forte." -ForegroundColor White
                Write-Host "Para forçar 5GHz, você precisa:" -ForegroundColor White
                Write-Host "1. Acessar Configurações > Rede e Internet > Wi-Fi" -ForegroundColor White
                Write-Host "2. Propriedades da rede" -ForegroundColor White
                Write-Host "3. Banda de frequência de rede > 5 GHz preferencial" -ForegroundColor White
            }

            Write-Host ""
            Write-Host "Status atual:" -ForegroundColor Cyan
            netsh wlan show interfaces | Select-String "SSID|Banda|Sinal|Taxa"
        }
    } else {
        Write-Host "❌ 5GHz não detectado no momento" -ForegroundColor Red
        Write-Host "   O roteador pode estar com 5GHz desabilitado ou fora de alcance" -ForegroundColor Yellow
    }
} else {
    Write-Host "❌ Rede 'Don Branco Barbearia' não encontrada" -ForegroundColor Red
}

Write-Host ""
Write-Host "=====================================" -ForegroundColor Cyan
Write-Host ""

# Mostrar todas as redes com 5GHz disponíveis
Write-Host "📋 Todas as redes 5GHz detectadas:" -ForegroundColor Cyan
$lines = $networks -split "`n"
$currentSSID = ""
foreach ($line in $lines) {
    if ($line -match "SSID \d+ : (.+)") {
        $currentSSID = $matches[1].Trim()
    }
    if ($line -match "5 GHz" -and $currentSSID) {
        $signal = ""
        if ($networks -match "(?s)$currentSSID.*?Sinal\s+:\s+(\d+%)") {
            $signal = " - Sinal: $($matches[1])"
        }
        Write-Host "   • $currentSSID $signal" -ForegroundColor White
        $currentSSID = ""
    }
}

Write-Host ""
Write-Host "Pressione qualquer tecla para sair..."
$null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")
