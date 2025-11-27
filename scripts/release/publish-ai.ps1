param(
    [switch]$DryRun,
    [switch]$SkipPublish
)

$ErrorActionPreference = 'Stop'

$repoRoot = Resolve-Path (Join-Path $PSScriptRoot '..\..')
$workspaceManifest = Resolve-Path (Join-Path $repoRoot 'ai\Cargo.toml')

Write-Host "[ai] Formatando código..." -ForegroundColor Cyan
cargo fmt --manifest-path $workspaceManifest --all

Write-Host "[ai] Rodando clippy..." -ForegroundColor Cyan
cargo clippy --manifest-path $workspaceManifest --workspace --all-targets -- -D warnings

Write-Host "[ai] Executando testes..." -ForegroundColor Cyan
cargo test --manifest-path $workspaceManifest --workspace

Write-Host "[ai] Gerando documentação..." -ForegroundColor Cyan
cargo doc --manifest-path $workspaceManifest --workspace --no-deps

if ($SkipPublish) {
    Write-Host "[ai] Publicação pulada (SkipPublish)." -ForegroundColor Yellow
    return
}

$publishArgs = @('--locked')
if ($DryRun) {
    $publishArgs += '--dry-run'
}

$aiCrates = @(
    'avila-clustering',
    'avila-dataframe',
    'avila-ml',
    'avila-tokenizer'
)

foreach ($crate in $aiCrates) {
    $manifestPath = Resolve-Path (Join-Path $repoRoot "$crate\Cargo.toml")
    Write-Host "[ai] Publicando $crate..." -ForegroundColor Green
    cargo publish --manifest-path $manifestPath @publishArgs
}

Write-Host "[ai] Release finalizada." -ForegroundColor Green
