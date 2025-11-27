# AVX-HTTP Publication Script
# Automates the publishing process to crates.io

param(
    [switch]$DryRun,
    [switch]$SkipTests,
    [switch]$SkipLint
)

Write-Host "═══════════════════════════════════════" -ForegroundColor Cyan
Write-Host "   AVX-HTTP Publication Script v0.4.0   " -ForegroundColor Cyan
Write-Host "═══════════════════════════════════════" -ForegroundColor Cyan
Write-Host ""

$ErrorActionPreference = "Stop"

# Change to project directory
Set-Location $PSScriptRoot

# 1. Format code
if (-not $SkipLint) {
    Write-Host "📝 Formatting code..." -ForegroundColor Yellow
    cargo fmt --all
    if ($LASTEXITCODE -ne 0) {
        Write-Host "❌ Format failed!" -ForegroundColor Red
        exit 1
    }
    Write-Host "✅ Format OK" -ForegroundColor Green
    Write-Host ""
}

# 2. Clippy
if (-not $SkipLint) {
    Write-Host "🔍 Running clippy..." -ForegroundColor Yellow
    cargo clippy --all-targets --all-features -- -W clippy::all
    if ($LASTEXITCODE -ne 0) {
        Write-Host "⚠️  Clippy warnings found (continuing...)" -ForegroundColor Yellow
    } else {
        Write-Host "✅ Clippy OK" -ForegroundColor Green
    }
    Write-Host ""
}

# 3. Build
Write-Host "🔨 Building release..." -ForegroundColor Yellow
cargo build --release --all-features
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Build failed!" -ForegroundColor Red
    exit 1
}
Write-Host "✅ Build OK" -ForegroundColor Green
Write-Host ""

# 4. Tests
if (-not $SkipTests) {
    Write-Host "🧪 Running tests..." -ForegroundColor Yellow

    # Core tests
    Write-Host "  → Core features..." -ForegroundColor Gray
    cargo test --lib

    # TLS tests
    Write-Host "  → With TLS..." -ForegroundColor Gray
    cargo test --lib --features tls

    Write-Host "✅ Tests OK" -ForegroundColor Green
    Write-Host ""
}

# 5. Documentation
Write-Host "📚 Building documentation..." -ForegroundColor Yellow
cargo doc --no-deps --all-features
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Doc build failed!" -ForegroundColor Red
    exit 1
}
Write-Host "✅ Documentation OK" -ForegroundColor Green
Write-Host ""

# 6. Package
Write-Host "📦 Packaging..." -ForegroundColor Yellow
cargo package --list
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Package failed!" -ForegroundColor Red
    exit 1
}
Write-Host "✅ Package OK" -ForegroundColor Green
Write-Host ""

# 7. Dry run
if ($DryRun) {
    Write-Host "🔬 Dry run publish..." -ForegroundColor Yellow
    cargo publish --dry-run --allow-dirty
    if ($LASTEXITCODE -ne 0) {
        Write-Host "❌ Dry run failed!" -ForegroundColor Red
        exit 1
    }
    Write-Host "✅ Dry run OK" -ForegroundColor Green
    Write-Host ""
    Write-Host "🎉 Ready to publish! Run without -DryRun to publish." -ForegroundColor Cyan
    exit 0
}

# 8. Confirm publication
Write-Host ""
Write-Host "═══════════════════════════════════════" -ForegroundColor Yellow
Write-Host "        READY TO PUBLISH!" -ForegroundColor Yellow
Write-Host "═══════════════════════════════════════" -ForegroundColor Yellow
Write-Host ""
Write-Host "Version: 0.4.0" -ForegroundColor Cyan
Write-Host "Package: avx-http" -ForegroundColor Cyan
Write-Host "Target:  crates.io" -ForegroundColor Cyan
Write-Host ""

$confirmation = Read-Host "Publish to crates.io? (yes/no)"
if ($confirmation -ne "yes") {
    Write-Host "❌ Publication cancelled." -ForegroundColor Yellow
    exit 0
}

# 9. Publish!
Write-Host ""
Write-Host "🚀 Publishing to crates.io..." -ForegroundColor Green
cargo publish --allow-dirty
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Publication failed!" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "═══════════════════════════════════════" -ForegroundColor Green
Write-Host "     🎉 PUBLICATION SUCCESSFUL! 🎉      " -ForegroundColor Green
Write-Host "═══════════════════════════════════════" -ForegroundColor Green
Write-Host ""
Write-Host "Next steps:" -ForegroundColor Cyan
Write-Host "  1. Tag release:    git tag v0.4.0" -ForegroundColor White
Write-Host "  2. Push tag:       git push origin v0.4.0" -ForegroundColor White
Write-Host "  3. Create GitHub release" -ForegroundColor White
Write-Host "  4. Announce on Reddit r/rust" -ForegroundColor White
Write-Host ""
Write-Host "View at: https://crates.io/crates/avx-http" -ForegroundColor Cyan
Write-Host ""
