param(
    [switch]$DryRun,
    [switch]$SkipPublish
)

$ErrorActionPreference = 'Stop'

$repoRoot = Resolve-Path (Join-Path $PSScriptRoot '..\..')
$workspaceManifest = Resolve-Path (Join-Path $repoRoot 'core\Cargo.toml')

Write-Host "[core] Formatando código..." -ForegroundColor Cyan
cargo fmt --manifest-path $workspaceManifest --all

Write-Host "[core] Rodando clippy..." -ForegroundColor Cyan
cargo clippy --manifest-path $workspaceManifest --workspace --all-targets -- -D warnings

Write-Host "[core] Executando testes..." -ForegroundColor Cyan
cargo test --manifest-path $workspaceManifest --workspace

Write-Host "[core] Gerando documentação..." -ForegroundColor Cyan
cargo doc --manifest-path $workspaceManifest --workspace --no-deps

if ($SkipPublish) {
    Write-Host "[core] Publicação pulada (SkipPublish)." -ForegroundColor Yellow
    return
}

$publishArgs = @('--locked')
if ($DryRun) {
    $publishArgs += '--dry-run'
}

$coreCrates = @(
    'avila',
    'avila-arrow',
    'avila-compress',
    'avila-linalg',
    'avila-math',
    'avila-reduction',
    'avila-telemetry'
)

foreach ($crate in $coreCrates) {
    $manifestPath = Resolve-Path (Join-Path $repoRoot "$crate\Cargo.toml")
    Write-Host "[core] Publicando $crate..." -ForegroundColor Green
    cargo publish --manifest-path $manifestPath @publishArgs
}

Write-Host "[core] Release finalizada." -ForegroundColor Green
