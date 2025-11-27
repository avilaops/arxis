param(
    [switch]$DryRun,
    [switch]$SkipPublish
)

$ErrorActionPreference = 'Stop'

$repoRoot = Resolve-Path (Join-Path $PSScriptRoot '..\..')
$workspaceManifest = Resolve-Path (Join-Path $repoRoot 'platform\Cargo.toml')

Write-Host "[platform] Formatando código..." -ForegroundColor Cyan
cargo fmt --manifest-path $workspaceManifest --all

Write-Host "[platform] Rodando clippy..." -ForegroundColor Cyan
cargo clippy --manifest-path $workspaceManifest --workspace --all-targets -- -D warnings

Write-Host "[platform] Executando testes..." -ForegroundColor Cyan
cargo test --manifest-path $workspaceManifest --workspace

Write-Host "[platform] Validando documentação..." -ForegroundColor Cyan
cargo doc --manifest-path $workspaceManifest --workspace --no-deps

if ($SkipPublish) {
    Write-Host "[platform] Publicação pulada (SkipPublish)." -ForegroundColor Yellow
    return
}

$publishArgs = @('--locked')
if ($DryRun) {
    $publishArgs += '--dry-run'
}

$platformCrates = @(
    'avl-auth',
    'avl-console',
    'avl-loadbalancer',
    'avl-observability',
    'avl-queue',
    'avl-secrets',
    'avl-storage',
    'avx-api-core',
    'avx-cli',
    'avx-config',
    'avx-events',
    'avx-gateway',
    'avx-http',
    'avx-telemetry',
    'aviladb'
)

foreach ($crate in $platformCrates) {
    $manifestPath = Resolve-Path (Join-Path $repoRoot "$crate\Cargo.toml")
    Write-Host "[platform] Publicando $crate..." -ForegroundColor Green
    cargo publish --manifest-path $manifestPath @publishArgs
}

Write-Host "[platform] Release finalizada." -ForegroundColor Green
