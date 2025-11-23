# 🔥 Quick Performance Test Script
# Execute: .\quick-benchmark.ps1

Write-Host ""
Write-Host "╔═══════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║     🔬 ARXIS Performance Benchmark 🔬     ║" -ForegroundColor Yellow
Write-Host "╚═══════════════════════════════════════════╝" -ForegroundColor Cyan
Write-Host ""

$CLI_PATH = "target\release"

# Check if compiled
if (-not (Test-Path "$CLI_PATH\avila-compress-cli.exe")) {
    Write-Host "❌ Binários não encontrados!" -ForegroundColor Red
    Write-Host "   Execute: cargo build --release" -ForegroundColor Yellow
    exit 1
}

# Test 1: Compression Speed
Write-Host "📦 Teste 1: Compressão de Arquivo" -ForegroundColor Cyan
Write-Host "   File: ..\..\README.md" -ForegroundColor Gray

$iterations = 10
$times = @()

for ($i = 1; $i -le $iterations; $i++) {
    $time = (Measure-Command {
            & "$CLI_PATH\avila-compress-cli.exe" compress ..\..\README.md -o test_$i.avz 2>&1 | Out-Null
        }).TotalMilliseconds
    $times += $time
    Write-Host "   Run $i : $([math]::Round($time, 2)) ms" -ForegroundColor White
    Remove-Item test_$i.avz -ErrorAction SilentlyContinue
}

$avgTime = ($times | Measure-Object -Average).Average
$minTime = ($times | Measure-Object -Minimum).Minimum
$maxTime = ($times | Measure-Object -Maximum).Maximum

Write-Host ""
Write-Host "   📊 Estatísticas:" -ForegroundColor Yellow
Write-Host "      Média:  $([math]::Round($avgTime, 2)) ms" -ForegroundColor Green
Write-Host "      Mínimo: $([math]::Round($minTime, 2)) ms" -ForegroundColor Green
Write-Host "      Máximo: $([math]::Round($maxTime, 2)) ms" -ForegroundColor Green
Write-Host ""

# Test 2: File Size
Write-Host "📏 Teste 2: Taxa de Compressão" -ForegroundColor Cyan
$originalSize = (Get-Item ..\..\README.md).Length
& "$CLI_PATH\avila-compress-cli.exe" compress ..\..\README.md -o test_final.avz 2>&1 | Out-Null
$compressedSize = (Get-Item test_final.avz).Length
$ratio = [math]::Round($originalSize / $compressedSize, 2)
$saved = $originalSize - $compressedSize

Write-Host "   Original:    $($originalSize) bytes" -ForegroundColor White
Write-Host "   Comprimido:  $($compressedSize) bytes" -ForegroundColor White
Write-Host "   Ratio:       ${ratio}x" -ForegroundColor Green
Write-Host "   Economizado: $($saved) bytes ($([math]::Round(($saved/$originalSize)*100, 1))%)" -ForegroundColor Green
Write-Host ""

Remove-Item test_final.avz -ErrorAction SilentlyContinue

# Test 3: Throughput
Write-Host "⚡ Teste 3: Throughput" -ForegroundColor Cyan
$throughputMBps = [math]::Round(($originalSize / 1MB) / ($avgTime / 1000), 2)
Write-Host "   $throughputMBps MB/s" -ForegroundColor Green
Write-Host ""

# Test 4: System Info
Write-Host "💻 Informações do Sistema:" -ForegroundColor Cyan
$cpu = Get-WmiObject Win32_Processor | Select-Object -First 1
$disk = Get-PhysicalDisk | Where-Object { $_.DeviceID -eq 0 } | Select-Object -First 1

Write-Host "   CPU:   $($cpu.Name)" -ForegroundColor White
Write-Host "   Cores: $($cpu.NumberOfCores) physical, $($cpu.NumberOfLogicalProcessors) logical" -ForegroundColor White
Write-Host "   Disco: $($disk.FriendlyName) ($($disk.MediaType))" -ForegroundColor White
Write-Host ""

# Verdict
Write-Host "🎯 Veredicto:" -ForegroundColor Magenta

if ($avgTime -lt 50) {
    Write-Host "   🔥 EXCELENTE! Performance nativa Rust!" -ForegroundColor Green
}
elseif ($avgTime -lt 200) {
    Write-Host "   ✅ BOM! Performance aceitável." -ForegroundColor Green
}
elseif ($avgTime -lt 500) {
    Write-Host "   ⚠️  MÉDIO. Possível overhead de I/O ou antivírus." -ForegroundColor Yellow
}
else {
    Write-Host "   ❌ LENTO! Verifique:" -ForegroundColor Red
    Write-Host "      - Compilou com --release?" -ForegroundColor Yellow
    Write-Host "      - Antivírus ativo?" -ForegroundColor Yellow
    Write-Host "      - Disco HDD ao invés de SSD?" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "📚 Para análise detalhada, leia:" -ForegroundColor Cyan
Write-Host "   PERFORMANCE_GUIDE.md" -ForegroundColor White
Write-Host ""
