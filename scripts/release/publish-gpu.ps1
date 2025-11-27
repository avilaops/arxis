param(
    [switch]$DryRun,
    [switch]$SkipPublish
)

$ErrorActionPreference = 'Stop'

$repoRoot = Resolve-Path (Join-Path $PSScriptRoot '..\..')
$workspaceManifest = Resolve-Path (Join-Path $repoRoot 'gpu\Cargo.toml')

Write-Host "[gpu] Formatando código..." -ForegroundColor Cyan
cargo fmt --manifest-path $workspaceManifest --all

Write-Host "[gpu] Rodando clippy..." -ForegroundColor Cyan
cargo clippy --manifest-path $workspaceManifest --workspace --all-targets -- -D warnings

Write-Host "[gpu] Executando testes..." -ForegroundColor Cyan
cargo test --manifest-path $workspaceManifest --workspace

Write-Host "[gpu] Gerando documentação..." -ForegroundColor Cyan
cargo doc --manifest-path $workspaceManifest --workspace --no-deps

if ($SkipPublish) {
    Write-Host "[gpu] Publicação pulada (SkipPublish)." -ForegroundColor Yellow
    return
}

$publishArgs = @('--locked')
if ($DryRun) {
    $publishArgs += '--dry-run'
}

$gpuCrates = @(
    'avx-gpu\avx-gpu-core',
    'avx-gpu\avx-gpu-runtime',
    'avx-gpu\avx-gpu-std',
    'avx-gpu\avx-gpu-macros',
    'avx-gpu\avx-gpu-compiler',
    'avx-gpu\avx-gpu-backends\wgpu',
    'avx-gpu\avx-gpu-backends\vulkan',
    'avx-gpu\avx-gpu-backends\cuda',
    'avx-gpu\avx-gpu-backends\metal',
    'avx-gpu\avx-gpu-backends\rocm',
    'avx-quantum-render'
)

foreach ($cratePath in $gpuCrates) {
    $manifestCandidate = Join-Path $repoRoot "$cratePath\Cargo.toml"
    if (-not (Test-Path $manifestCandidate)) {
        Write-Host "[gpu] Ignorando $cratePath (manifesto não encontrado)." -ForegroundColor Yellow
        continue
    }

    $manifestPath = Resolve-Path $manifestCandidate
    Write-Host "[gpu] Publicando $cratePath..." -ForegroundColor Green
    cargo publish --manifest-path $manifestPath @publishArgs
}

Write-Host "[gpu] Release finalizada." -ForegroundColor Green
