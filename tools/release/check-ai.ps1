param(
    [switch]$SkipDocs
)

$ErrorActionPreference = 'Stop'

$repoRoot = Resolve-Path (Join-Path $PSScriptRoot '..\..')
$manifest = Resolve-Path (Join-Path $repoRoot 'ai\Cargo.toml')

Write-Host "[ai] Verificando formatação" -ForegroundColor Cyan
cargo fmt --manifest-path $manifest --all -- --check

Write-Host "[ai] Rodando clippy" -ForegroundColor Cyan
cargo clippy --manifest-path $manifest --workspace --all-targets -- -D warnings

Write-Host "[ai] Executando testes" -ForegroundColor Cyan
cargo test --manifest-path $manifest --workspace

if (-not $SkipDocs) {
    Write-Host "[ai] Validando documentação" -ForegroundColor Cyan
    cargo doc --manifest-path $manifest --workspace --no-deps
}

Write-Host "[ai] Checklist OK" -ForegroundColor Green
