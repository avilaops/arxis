# Script de Auditoria Completa - ARXIS Project
# Mapeia todos os crates, detecta duplicados, valida estrutura

$ErrorActionPreference = "Continue"

Write-Host "========================================" -ForegroundColor Magenta
Write-Host "  ARXIS PROJECT AUDIT - 02/12/2025" -ForegroundColor Magenta
Write-Host "========================================" -ForegroundColor Magenta
Write-Host ""

# Coletar todos os crates
$allCrates = Get-ChildItem -Directory -Path "." | Where-Object {
    Test-Path "$($_.FullName)\Cargo.toml"
}

$avilaModules = $allCrates | Where-Object { $_.Name -like "avila-*" }
$avxModules = $allCrates | Where-Object { $_.Name -like "avx-*" }

Write-Host "TOTAIS:" -ForegroundColor Cyan
Write-Host "  Total crates: $($allCrates.Count)" -ForegroundColor White
Write-Host "  avila-* : $($avilaModules.Count)" -ForegroundColor Green
Write-Host "  avx-*   : $($avxModules.Count)" -ForegroundColor Green
Write-Host ""

# Detectar duplicados
Write-Host "DUPLICADOS DETECTADOS:" -ForegroundColor Yellow
$basenames = @{}
foreach ($crate in $allCrates) {
    $basename = $crate.Name -replace '-old$','' -replace '-new$',''
    if ($basenames.ContainsKey($basename)) {
        $basenames[$basename] += @($crate.Name)
    } else {
        $basenames[$basename] = @($crate.Name)
    }
}

$duplicates = $basenames.GetEnumerator() | Where-Object { $_.Value.Count -gt 1 }
if ($duplicates) {
    foreach ($dup in $duplicates) {
        Write-Host "  $($dup.Key):" -ForegroundColor Red
        foreach ($variant in $dup.Value) {
            Write-Host "    - $variant" -ForegroundColor Gray
        }
    }
} else {
    Write-Host "  Nenhum duplicado encontrado" -ForegroundColor Green
}
Write-Host ""

# Validar estrutura
Write-Host "VALIDACAO DE ESTRUTURA:" -ForegroundColor Cyan
$issues = @()

foreach ($crate in $allCrates) {
    $hasLib = Test-Path "$($crate.FullName)\src\lib.rs"
    $hasMain = Test-Path "$($crate.FullName)\src\main.rs"
    $hasReadme = Test-Path "$($crate.FullName)\README.md"
    $hasTests = Test-Path "$($crate.FullName)\tests"
    $hasExamples = Test-Path "$($crate.FullName)\examples"

    if (-not ($hasLib -or $hasMain)) {
        $issues += [PSCustomObject]@{
            Crate = $crate.Name
            Issue = "Missing src/lib.rs or src/main.rs"
        }
    }

    if (-not $hasReadme) {
        $issues += [PSCustomObject]@{
            Crate = $crate.Name
            Issue = "Missing README.md"
        }
    }
}

if ($issues) {
    Write-Host "  Problemas encontrados: $($issues.Count)" -ForegroundColor Red
    $issues | Format-Table -AutoSize
} else {
    Write-Host "  Todos os crates tem estrutura valida!" -ForegroundColor Green
}
Write-Host ""

# Análise por categoria
Write-Host "ANALISE POR CATEGORIA:" -ForegroundColor Cyan

$categories = @{
    "Foundation" = @("nucleus", "primitives", "error", "id", "time", "atom", "cell", "serde", "log", "future", "rand", "regex", "crypto", "term")
    "Mathematics" = @("math", "numeric", "linalg", "ndarray", "fft", "parallel", "modular", "finite-fields", "curve", "prime", "bignum")
    "Data/ML" = @("dataframe", "clustering", "reduction", "tokenizer", "image", "vision", "ml", "arrow")
    "Database" = @("db", "cache", "buffer", "compress", "codec", "alloc")
    "Networking" = @("tcp", "udp", "quic", "http", "websocket", "dns", "grpc", "molecule")
    "Distributed" = @("raft", "gossip", "crdt", "election", "partition", "shard", "replication", "lease", "distributed-system")
    "Cryptography" = @("aead", "kdf", "mac", "signature", "pki", "tls", "jwt", "oauth", "zkp", "threshold", "mpc", "stealth", "quantum", "post-quantum", "hash", "onion-routing")
    "Infrastructure" = @("cli", "config", "metrics", "telemetry", "monitor", "tracing", "logger", "alert", "validate")
    "Web/Framework" = @("web", "webframework", "frontend", "framework")
    "Coordination" = @("coordinator", "orchestrator", "workflow", "service-mesh", "loadbalancer", "proxy")
    "Advanced" = @("gpu", "quantum-render", "mcp", "copilot-ai")
    "Biology" = @("organ", "organism", "tissue")
    "Other" = @("async", "sync", "lock", "pool", "examples", "tests", "tools", "scripts", "meta")
}

foreach ($cat in $categories.GetEnumerator() | Sort-Object Name) {
    $matching = $avilaModules | Where-Object {
        $name = $_.Name -replace '^avila-',''
        $cat.Value -contains $name
    }

    if ($matching.Count -gt 0) {
        Write-Host "  $($cat.Key): $($matching.Count) modulos" -ForegroundColor Green
    }
}
Write-Host ""

# Workspaces
Write-Host "WORKSPACES:" -ForegroundColor Cyan
$workspaces = Get-ChildItem -Directory | Where-Object { $_.Name -like "*-workspace" }
foreach ($ws in $workspaces) {
    Write-Host "  $($ws.Name)" -ForegroundColor Gray
    $wsToml = Get-Content "$($ws.FullName)\Cargo.toml" -Raw
    if ($wsToml -match 'members\s*=\s*\[([^\]]+)\]') {
        $members = $matches[1] -split ',' | ForEach-Object { $_.Trim().Trim('"') }
        Write-Host "    Members: $($members.Count)" -ForegroundColor White
    }
}
Write-Host ""

# Testes
Write-Host "TESTES:" -ForegroundColor Cyan
$withTests = $allCrates | Where-Object { Test-Path "$($_.FullName)\tests" }
$withSrcTests = $allCrates | Where-Object {
    $libPath = "$($_.FullName)\src\lib.rs"
    if (Test-Path $libPath) {
        $content = Get-Content $libPath -Raw
        $content -match '#\[cfg\(test\)\]' -or $content -match '#\[test\]'
    }
}

Write-Host "  Com diretorio tests/: $($withTests.Count)" -ForegroundColor White
Write-Host "  Com testes em src/: $($withSrcTests.Count)" -ForegroundColor White
Write-Host "  Total com algum teste: $(($withTests + $withSrcTests | Select-Object -Unique).Count)" -ForegroundColor Green
Write-Host ""

# Publicação
Write-Host "STATUS DE PUBLICACAO:" -ForegroundColor Cyan
$published = @()
$ready = @()
$needsWork = @()

foreach ($crate in $avilaModules | Select-Object -First 20) {
    $cargoToml = Get-Content "$($crate.FullName)\Cargo.toml" -Raw

    $hasVersion = $cargoToml -match 'version\s*=\s*"([^"]+)"'
    $hasDescription = $cargoToml -match 'description\s*='
    $hasLicense = $cargoToml -match 'license\s*='
    $hasReadme = Test-Path "$($crate.FullName)\README.md"

    if ($hasVersion -and $hasDescription -and $hasLicense -and $hasReadme) {
        $ready += $crate.Name
    } else {
        $needsWork += $crate.Name
    }
}

Write-Host "  Prontos para publicacao (sample): $($ready.Count)" -ForegroundColor Green
Write-Host "  Precisam de ajustes (sample): $($needsWork.Count)" -ForegroundColor Yellow
Write-Host ""

# Salvar relatório
$report = @{
    timestamp = Get-Date -Format "o"
    total_crates = $allCrates.Count
    avila_modules = $avilaModules.Count
    avx_modules = $avxModules.Count
    duplicates = $duplicates.Count
    issues = $issues.Count
    with_tests = ($withTests + $withSrcTests | Select-Object -Unique).Count
    ready_to_publish = $ready.Count
} | ConvertTo-Json

$reportPath = ".\avx-intelligence\logs\audit-$(Get-Date -Format 'yyyyMMdd-HHmmss').json"
$report | Out-File -FilePath $reportPath -Encoding UTF8

Write-Host "========================================" -ForegroundColor Magenta
Write-Host "Relatorio salvo: $reportPath" -ForegroundColor White
Write-Host "========================================" -ForegroundColor Magenta
