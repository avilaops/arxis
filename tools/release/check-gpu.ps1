param(
    [switch]$SkipDocs,
    [switch]$SkipBenchmarks
)

$ErrorActionPreference = 'Stop'

$repoRoot = Resolve-Path (Join-Path $PSScriptRoot '..\..')
$manifest = Resolve-Path (Join-Path $repoRoot 'gpu\Cargo.toml')

Write-Host "[gpu] Verificando formatação" -ForegroundColor Cyan
cargo fmt --manifest-path $manifest --all -- --check

Write-Host "[gpu] Rodando clippy" -ForegroundColor Cyan
cargo clippy --manifest-path $manifest --workspace --all-targets -- -D warnings

Write-Host "[gpu] Executando testes" -ForegroundColor Cyan
cargo test --manifest-path $manifest --workspace

if (-not $SkipBenchmarks) {
    Write-Host "[gpu] Realizando benches principais" -ForegroundColor Cyan
    cargo bench --manifest-path $manifest --workspace --no-run
}

if (-not $SkipDocs) {
    Write-Host "[gpu] Validando documentação" -ForegroundColor Cyan
    cargo doc --manifest-path $manifest --workspace --no-deps
}

Write-Host "[gpu] Checklist OK" -ForegroundColor Green
