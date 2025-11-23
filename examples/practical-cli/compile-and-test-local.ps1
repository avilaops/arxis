# 🔥 COMPILE E TESTE TUDO - Versão Local Otimizada
# Execute em C:\Temp\arxis-benchmark para máxima performance

Write-Host ""
Write-Host "╔═══════════════════════════════════════════════╗" -ForegroundColor Red
Write-Host "║  🔥 COMPILAÇÃO + BENCHMARK OTIMIZADO 🔥       ║" -ForegroundColor Yellow
Write-Host "╚═══════════════════════════════════════════════╝" -ForegroundColor Red
Write-Host ""

# Verificar se está em C:\Temp
$currentPath = $PWD.Path
if ($currentPath -notlike "C:\Temp*") {
    Write-Host "⚠️  AVISO: Você não está em C:\Temp!" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "Para MÁXIMA performance, execute:" -ForegroundColor Cyan
    Write-Host "   .\setup-local.ps1" -ForegroundColor White
    Write-Host ""
    Write-Host "Isso vai copiar o projeto para C:\Temp (fora do OneDrive)" -ForegroundColor Gray
    Write-Host ""
    $continue = Read-Host "Continuar mesmo assim? (S/N)"
    if ($continue -ne "S" -and $continue -ne "s") {
        exit 0
    }
}

Write-Host "📊 Informações do Sistema:" -ForegroundColor Cyan
$cpu = Get-WmiObject Win32_Processor | Select-Object -First 1
$disk = Get-PhysicalDisk | Where-Object { $_.DeviceID -eq 0 } | Select-Object -First 1
$ram = [math]::Round((Get-WmiObject Win32_ComputerSystem).TotalPhysicalMemory / 1GB, 2)

Write-Host "   CPU:   $($cpu.Name)" -ForegroundColor White
Write-Host "   Cores: $($cpu.NumberOfCores) físicos, $($cpu.NumberOfLogicalProcessors) lógicos" -ForegroundColor White
Write-Host "   RAM:   $ram GB" -ForegroundColor White
Write-Host "   Disco: $($disk.FriendlyName) ($($disk.MediaType))" -ForegroundColor White
Write-Host ""

# Limpar build anterior
if (Test-Path "target") {
    Write-Host "🗑️  Limpando build anterior..." -ForegroundColor Yellow
    Remove-Item -Path "target" -Recurse -Force
}

Write-Host "🔨 COMPILANDO COM OTIMIZAÇÕES MÁXIMAS..." -ForegroundColor Cyan
Write-Host "   (Primeira compilação demora ~2 minutos)" -ForegroundColor Gray
Write-Host ""

$compileStart = Get-Date

# Compilar com flags de otimização
$env:RUSTFLAGS = "-C target-cpu=native -C opt-level=3"
cargo build --release 2>&1 | Select-String -Pattern "Compiling|Finished|error" | ForEach-Object {
    if ($_ -like "*error*") {
        Write-Host $_ -ForegroundColor Red
    }
    elseif ($_ -like "*Finished*") {
        Write-Host $_ -ForegroundColor Green
    }
    else {
        Write-Host $_ -ForegroundColor Gray
    }
}

$compileTime = ((Get-Date) - $compileStart).TotalSeconds

if (-not (Test-Path "target\release\avila-compress-cli.exe")) {
    Write-Host ""
    Write-Host "❌ ERRO: Compilação falhou!" -ForegroundColor Red
    Write-Host ""
    exit 1
}

Write-Host ""
Write-Host "✅ Compilação concluída em $([math]::Round($compileTime, 1))s!" -ForegroundColor Green
Write-Host ""

# Mostrar tamanhos dos binários
Write-Host "📦 Binários gerados:" -ForegroundColor Cyan
Get-ChildItem "target\release\*.exe" | ForEach-Object {
    $sizeMB = [math]::Round($_.Length / 1MB, 2)
    Write-Host "   $($_.Name): $sizeMB MB" -ForegroundColor White
}
Write-Host ""

# Verificar Windows Defender
$defenderStatus = Get-MpComputerStatus -ErrorAction SilentlyContinue
if ($defenderStatus -and $defenderStatus.RealTimeProtectionEnabled) {
    Write-Host "⚠️  Windows Defender ATIVO - Pode impactar performance!" -ForegroundColor Yellow
    Write-Host "   Para testes extremos, execute (como Admin):" -ForegroundColor Gray
    Write-Host "   .\disable-defender.ps1" -ForegroundColor Cyan
    Write-Host ""
}

# Criar arquivo de teste se não existe
if (-not (Test-Path "C:\Temp\README.md")) {
    Write-Host "📄 Criando arquivo de teste..." -ForegroundColor Yellow
    $testContent = "# Test File`n" + ("Lorem ipsum dolor sit amet. " * 1000)
    Set-Content -Path "C:\Temp\README.md" -Value $testContent
}

Write-Host "🚀 INICIANDO BENCHMARK..." -ForegroundColor Green
Write-Host ""

# Benchmark rápido (10 iterações)
Write-Host "📊 Teste 1: Compressão Rápida (10 iterações)" -ForegroundColor Cyan

$times = @()
for ($i = 1; $i -le 10; $i++) {
    $time = (Measure-Command {
            & "target\release\avila-compress-cli.exe" compress C:\Temp\README.md -o "C:\Temp\test_$i.avz" 2>&1 | Out-Null
        }).TotalMilliseconds
    $times += $time
    Remove-Item "C:\Temp\test_$i.avz" -ErrorAction SilentlyContinue

    if ($i -eq 1) {
        Write-Host "   Run $i (cold): $([math]::Round($time, 2)) ms" -ForegroundColor Gray
    }
    else {
        Write-Host "   Run $i: $([math]::Round($time, 2)) ms" -ForegroundColor White
    }
}

$avg = ($times | Measure-Object -Average).Average
$min = ($times | Measure-Object -Minimum).Minimum
$median = ($times | Sort-Object)[[math]::Floor($times.Count / 2)]

Write-Host ""
Write-Host "   Média:   $([math]::Round($avg, 2)) ms" -ForegroundColor Yellow
Write-Host "   Mediana: $([math]::Round($median, 2)) ms" -ForegroundColor Yellow
Write-Host "   Mínimo:  $([math]::Round($min, 2)) ms" -ForegroundColor Green
Write-Host ""

# Teste com arquivo grande
Write-Host "📊 Teste 2: Arquivo Grande (50 MB)" -ForegroundColor Cyan
Write-Host "   Criando arquivo de teste..." -ForegroundColor Gray

$data = [byte[]]::new(50MB)
(New-Object Random).NextBytes($data)
[System.IO.File]::WriteAllBytes("C:\Temp\test_50mb.dat", $data)

Write-Host "   Comprimindo..." -ForegroundColor Gray

$time = (Measure-Command {
        & "target\release\avila-compress-cli.exe" compress C:\Temp\test_50mb.dat -o C:\Temp\test_50mb.avz 2>&1 | Out-Null
    }).TotalMilliseconds

$originalSize = (Get-Item C:\Temp\test_50mb.dat).Length
$compressedSize = (Get-Item C:\Temp\test_50mb.avz).Length
$throughput = [math]::Round(($originalSize / 1MB) / ($time / 1000), 2)
$ratio = [math]::Round($originalSize / $compressedSize, 2)

Write-Host ""
Write-Host "   Tempo:       $([math]::Round($time, 2)) ms" -ForegroundColor White
Write-Host "   Throughput:  $throughput MB/s" -ForegroundColor Green
Write-Host "   Compressão:  ${ratio}x" -ForegroundColor White
Write-Host ""

Remove-Item C:\Temp\test_50mb.dat, C:\Temp\test_50mb.avz -ErrorAction SilentlyContinue

# Veredicto
Write-Host "╔═══════════════════════════════════════════════╗" -ForegroundColor Magenta
Write-Host "║           🏆 RESULTADOS FINAIS 🏆            ║" -ForegroundColor Yellow
Write-Host "╚═══════════════════════════════════════════════╝" -ForegroundColor Magenta
Write-Host ""

Write-Host "📈 Performance em C:\Temp:" -ForegroundColor Cyan
Write-Host "   Arquivo pequeno: $([math]::Round($min, 2)) ms (melhor)" -ForegroundColor White
Write-Host "   Arquivo grande:  $throughput MB/s" -ForegroundColor White
Write-Host ""

if ($throughput -gt 200) {
    Write-Host "🔥 EXCELENTE! Performance nativa!" -ForegroundColor Green
}
elseif ($throughput -gt 100) {
    Write-Host "✅ MUITO BOM! Rust competitivo!" -ForegroundColor Green
}
elseif ($throughput -gt 50) {
    Write-Host "👍 BOM! Performance adequada" -ForegroundColor Cyan
}
else {
    Write-Host "⚠️  MÉDIO. Verifique Defender e disco" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "📚 Próximos passos:" -ForegroundColor Yellow
Write-Host "   • Teste outras ferramentas: .\target\release\avx-bench.exe https://google.com" -ForegroundColor White
Write-Host "   • Benchmark extremo: .\extreme-benchmark.ps1" -ForegroundColor White
Write-Host "   • Ler guias: PERFORMANCE_GUIDE.md" -ForegroundColor White
Write-Host ""
