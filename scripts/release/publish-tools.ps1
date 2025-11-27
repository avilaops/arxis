param(
    [switch]$DryRun,
    [switch]$SkipPublish
)

$ErrorActionPreference = 'Stop'

$repoRoot = Resolve-Path (Join-Path $PSScriptRoot '..\..')
$workspaceManifest = Resolve-Path (Join-Path $repoRoot 'tools\Cargo.toml')

Write-Host "[tools] Formatando código..." -ForegroundColor Cyan
cargo fmt --manifest-path $workspaceManifest --all

Write-Host "[tools] Rodando clippy..." -ForegroundColor Cyan
cargo clippy --manifest-path $workspaceManifest --workspace --all-targets -- -D warnings

Write-Host "[tools] Executando testes..." -ForegroundColor Cyan
cargo test --manifest-path $workspaceManifest --workspace

Write-Host "[tools] Validando documentação..." -ForegroundColor Cyan
cargo doc --manifest-path $workspaceManifest --workspace --no-deps

if ($SkipPublish) {
    Write-Host "[tools] Publicação pulada (SkipPublish)." -ForegroundColor Yellow
    return
}

$publishArgs = @('--locked')
if ($DryRun) {
    $publishArgs += '--dry-run'
}

$toolsCrates = @(
    'tools\xtask',
    'examples\practical-cli'
)

foreach ($cratePath in $toolsCrates) {
    $manifestCandidate = Join-Path $repoRoot "$cratePath\Cargo.toml"
    if (-not (Test-Path $manifestCandidate)) {
        Write-Host "[tools] Ignorando $cratePath (manifesto não encontrado)." -ForegroundColor Yellow
        continue
    }

    $manifestPath = Resolve-Path $manifestCandidate
    Write-Host "[tools] Publicando $cratePath..." -ForegroundColor Green
    cargo publish --manifest-path $manifestPath @publishArgs
}

Write-Host "[tools] Release finalizada." -ForegroundColor Green
