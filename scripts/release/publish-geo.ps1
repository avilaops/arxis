param(
    [switch]$DryRun,
    [switch]$SkipPublish
)

$ErrorActionPreference = 'Stop'

$repoRoot = Resolve-Path (Join-Path $PSScriptRoot '..\..')
$workspaceManifest = Resolve-Path (Join-Path $repoRoot 'geo\Cargo.toml')

Write-Host "[geo] Formatando código..." -ForegroundColor Cyan
cargo fmt --manifest-path $workspaceManifest --all

Write-Host "[geo] Rodando clippy..." -ForegroundColor Cyan
cargo clippy --manifest-path $workspaceManifest --workspace --all-targets -- -D warnings

Write-Host "[geo] Executando testes..." -ForegroundColor Cyan
cargo test --manifest-path $workspaceManifest --workspace

Write-Host "[geo] Gerando documentação..." -ForegroundColor Cyan
cargo doc --manifest-path $workspaceManifest --workspace --no-deps

if ($SkipPublish) {
    Write-Host "[geo] Publicação pulada (SkipPublish)." -ForegroundColor Yellow
    return
}

$publishArgs = @('--locked')
if ($DryRun) {
    $publishArgs += '--dry-run'
}

$geoCrates = @(
    'avila-geo',
    'avila-geo\avila-analises',
    'avila-geo\avila-location',
    'avila-geo\data-extraction',
    'avila-geo\financial-optimization',
    'avila-geo\geospatial-analysis',
    'avila-geo\avx-image'
)

foreach ($cratePath in $geoCrates) {
    $manifestPath = Resolve-Path (Join-Path $repoRoot "$cratePath\Cargo.toml")
    Write-Host "[geo] Publicando $cratePath..." -ForegroundColor Green
    cargo publish --manifest-path $manifestPath @publishArgs
}

Write-Host "[geo] Release finalizada." -ForegroundColor Green
