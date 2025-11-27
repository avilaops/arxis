param(
    [switch]$SkipDocs,
    [switch]$SkipBenchmarks
)

$ErrorActionPreference = 'Stop'

$repoRoot = Resolve-Path (Join-Path $PSScriptRoot '..\..')
$manifest = Resolve-Path (Join-Path $repoRoot 'geo\Cargo.toml')

Write-Host "[geo] Verificando formatação" -ForegroundColor Cyan
cargo fmt --manifest-path $manifest --all -- --check

Write-Host "[geo] Rodando clippy" -ForegroundColor Cyan
cargo clippy --manifest-path $manifest --workspace --all-targets -- -D warnings

Write-Host "[geo] Executando testes" -ForegroundColor Cyan
cargo test --manifest-path $manifest --workspace

if (-not $SkipBenchmarks) {
    Write-Host "[geo] Executando benchmarks críticos" -ForegroundColor Cyan
    cargo bench --manifest-path $manifest --workspace --no-run
}

if (-not $SkipDocs) {
    Write-Host "[geo] Validando documentação" -ForegroundColor Cyan
    cargo doc --manifest-path $manifest --workspace --no-deps
}

Write-Host "[geo] Checklist OK" -ForegroundColor Green
