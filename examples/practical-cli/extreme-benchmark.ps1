# 🔥 TESTE DE PERFORMANCE EXTREMA - SEM DEFENDER
# Execute APÓS desabilitar o Windows Defender

Write-Host ""
Write-Host "╔═══════════════════════════════════════════════╗" -ForegroundColor Red
Write-Host "║  🔥 BENCHMARK EXTREMO (SEM DEFENDER) 🔥       ║" -ForegroundColor Yellow
Write-Host "╚═══════════════════════════════════════════════╝" -ForegroundColor Red
Write-Host ""

# Verificar se Defender está desabilitado
$defenderStatus = Get-MpComputerStatus
if ($defenderStatus.RealTimeProtectionEnabled) {
    Write-Host "⚠️  AVISO: Windows Defender AINDA ESTÁ ATIVO!" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "Para resultados precisos, execute primeiro:" -ForegroundColor Cyan
    Write-Host "   .\disable-defender.ps1" -ForegroundColor White
    Write-Host ""
    $continue = Read-Host "Continuar mesmo assim? (S/N)"
    if ($continue -ne "S" -and $continue -ne "s") {
        exit 0
    }
}

$CLI_PATH = "target\release\avila-compress-cli.exe"

if (-not (Test-Path $CLI_PATH)) {
    Write-Host "❌ Binário não encontrado!" -ForegroundColor Red
    Write-Host "   Execute: cargo build --release" -ForegroundColor Yellow
    exit 1
}

Write-Host "🚀 TESTE 1: Arquivo Pequeno (README.md)" -ForegroundColor Cyan
Write-Host ""

$warmup = 5
$iterations = 20

# Warmup
Write-Host "🔥 Warmup ($warmup runs)..." -ForegroundColor Yellow
for ($i = 1; $i -le $warmup; $i++) {
    & $CLI_PATH compress ..\..\README.md -o test_warmup.avz 2>&1 | Out-Null
    Remove-Item test_warmup.avz -ErrorAction SilentlyContinue
}

Write-Host "📊 Benchmark ($iterations runs)..." -ForegroundColor Cyan
$times = @()

for ($i = 1; $i -le $iterations; $i++) {
    $time = (Measure-Command {
        & $CLI_PATH compress ..\..\README.md -o test_$i.avz 2>&1 | Out-Null
    }).TotalMilliseconds
    $times += $time

    if ($i % 5 -eq 0) {
        Write-Host "   Completed: $i/$iterations" -ForegroundColor Gray
    }

    Remove-Item test_$i.avz -ErrorAction SilentlyContinue
}

$avg = ($times | Measure-Object -Average).Average
$min = ($times | Measure-Object -Minimum).Minimum
$max = ($times | Measure-Object -Maximum).Maximum
$median = ($times | Sort-Object)[[math]::Floor($times.Count / 2)]

Write-Host ""
Write-Host "📈 RESULTADOS (Arquivo Pequeno):" -ForegroundColor Green
Write-Host "   Média:   $([math]::Round($avg, 2)) ms" -ForegroundColor White
Write-Host "   Mediana: $([math]::Round($median, 2)) ms" -ForegroundColor White
Write-Host "   Mínimo:  $([math]::Round($min, 2)) ms" -ForegroundColor Cyan
Write-Host "   Máximo:  $([math]::Round($max, 2)) ms" -ForegroundColor White
Write-Host ""

# Teste com arquivo maior
Write-Host "🚀 TESTE 2: Arquivo Grande (10 MB)" -ForegroundColor Cyan
Write-Host ""
Write-Host "Criando arquivo de teste..." -ForegroundColor Yellow

$data = [byte[]]::new(10MB)
(New-Object Random).NextBytes($data)
[System.IO.File]::WriteAllBytes("test_10mb.dat", $data)

Write-Host "Comprimindo..." -ForegroundColor Cyan

$time = (Measure-Command {
    & $CLI_PATH compress test_10mb.dat -o test_10mb.avz 2>&1 | Out-Null
}).TotalMilliseconds

$originalSize = (Get-Item test_10mb.dat).Length
$compressedSize = (Get-Item test_10mb.avz).Length
$throughput = [math]::Round(($originalSize / 1MB) / ($time / 1000), 2)
$ratio = [math]::Round($originalSize / $compressedSize, 2)

Write-Host ""
Write-Host "📈 RESULTADOS (Arquivo Grande):" -ForegroundColor Green
Write-Host "   Tempo:       $([math]::Round($time, 2)) ms" -ForegroundColor White
Write-Host "   Throughput:  $throughput MB/s" -ForegroundColor Cyan
Write-Host "   Compressão:  ${ratio}x" -ForegroundColor White
Write-Host ""

Remove-Item test_10mb.dat, test_10mb.avz -ErrorAction SilentlyContinue

# Teste MEGA STRESS
Write-Host "🔥 TESTE 3: STRESS TEST (100 MB)" -ForegroundColor Red
Write-Host ""
Write-Host "Criando arquivo massivo..." -ForegroundColor Yellow

$data = [byte[]]::new(100MB)
(New-Object Random).NextBytes($data)
[System.IO.File]::WriteAllBytes("test_100mb.dat", $data)

Write-Host "Comprimindo (isso pode demorar)..." -ForegroundColor Cyan

$time = (Measure-Command {
    & $CLI_PATH compress test_100mb.dat -o test_100mb.avz 2>&1 | Out-Null
}).TotalMilliseconds

$originalSize = (Get-Item test_100mb.dat).Length
$compressedSize = (Get-Item test_100mb.avz).Length
$throughput = [math]::Round(($originalSize / 1MB) / ($time / 1000), 2)
$ratio = [math]::Round($originalSize / $compressedSize, 2)

Write-Host ""
Write-Host "📈 RESULTADOS (STRESS TEST):" -ForegroundColor Green
Write-Host "   Tempo:       $([math]::Round($time / 1000, 2)) segundos" -ForegroundColor White
Write-Host "   Throughput:  $throughput MB/s" -ForegroundColor Cyan
Write-Host "   Compressão:  ${ratio}x" -ForegroundColor White
Write-Host ""

Remove-Item test_100mb.dat, test_100mb.avz -ErrorAction SilentlyContinue

# Veredicto Final
Write-Host "╔═══════════════════════════════════════════════╗" -ForegroundColor Magenta
Write-Host "║           🏆 VEREDICTO FINAL 🏆               ║" -ForegroundColor Yellow
Write-Host "╚═══════════════════════════════════════════════╝" -ForegroundColor Magenta
Write-Host ""

if ($throughput -gt 200) {
    Write-Host "🔥 ULTRA RÁPIDO! ($throughput MB/s)" -ForegroundColor Green
    Write-Host "   Rust nativo competindo com C/C++!" -ForegroundColor White
} elseif ($throughput -gt 100) {
    Write-Host "✅ MUITO BOM! ($throughput MB/s)" -ForegroundColor Green
    Write-Host "   Performance excelente!" -ForegroundColor White
} elseif ($throughput -gt 50) {
    Write-Host "👍 BOM! ($throughput MB/s)" -ForegroundColor Cyan
    Write-Host "   Performance adequada" -ForegroundColor White
} else {
    Write-Host "⚠️  MÉDIO ($throughput MB/s)" -ForegroundColor Yellow
    Write-Host "   Pode haver gargalo de I/O" -ForegroundColor Gray
}

Write-Host ""
Write-Host "📊 Comparação com outras ferramentas:" -ForegroundColor Cyan
Write-Host "   LZ4 (C):       300-500 MB/s" -ForegroundColor Gray
Write-Host "   Zstd (fast):   200-400 MB/s" -ForegroundColor Gray
Write-Host "   7-Zip (LZMA):  20-50 MB/s" -ForegroundColor Gray
Write-Host "   ARXIS (Rust):  $throughput MB/s" -ForegroundColor Yellow
Write-Host ""

if (-not $defenderStatus.RealTimeProtectionEnabled) {
    Write-Host "⚠️  Lembre-se de reativar o Windows Defender:" -ForegroundColor Yellow
    Write-Host "   .\enable-defender.ps1" -ForegroundColor White
    Write-Host ""
}
