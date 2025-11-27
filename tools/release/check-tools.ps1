param(
    [switch]$SkipDocs
)

$ErrorActionPreference = 'Stop'

$repoRoot = Resolve-Path (Join-Path $PSScriptRoot '..\..')
$manifest = Resolve-Path (Join-Path $repoRoot 'tools\Cargo.toml')

Write-Host "[tools] Verificando formatação" -ForegroundColor Cyan
cargo fmt --manifest-path $manifest --all -- --check

Write-Host "[tools] Rodando clippy" -ForegroundColor Cyan
cargo clippy --manifest-path $manifest --workspace --all-targets -- -D warnings

Write-Host "[tools] Executando testes" -ForegroundColor Cyan
cargo test --manifest-path $manifest --workspace

if (-not $SkipDocs) {
    Write-Host "[tools] Validando documentação" -ForegroundColor Cyan
    cargo doc --manifest-path $manifest --workspace --no-deps
}

Write-Host "[tools] Checklist OK" -ForegroundColor Green
