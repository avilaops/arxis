param(
    [switch]$SkipDocs
)

$ErrorActionPreference = 'Stop'

$repoRoot = Resolve-Path (Join-Path $PSScriptRoot '..\..')
$manifest = Resolve-Path (Join-Path $repoRoot 'platform\Cargo.toml')

Write-Host "[platform] Verificando formatação" -ForegroundColor Cyan
cargo fmt --manifest-path $manifest --all -- --check

Write-Host "[platform] Rodando clippy" -ForegroundColor Cyan
cargo clippy --manifest-path $manifest --workspace --all-targets -- -D warnings

Write-Host "[platform] Executando testes" -ForegroundColor Cyan
cargo test --manifest-path $manifest --workspace

if (-not $SkipDocs) {
    Write-Host "[platform] Validando documentação" -ForegroundColor Cyan
    cargo doc --manifest-path $manifest --workspace --no-deps
}

Write-Host "[platform] Checklist OK" -ForegroundColor Green
