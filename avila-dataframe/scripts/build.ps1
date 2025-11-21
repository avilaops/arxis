#!/usr/bin/env pwsh
# Build script for avila-dataframe

Write-Host "🚀 Building AvilaDB DataFrame..." -ForegroundColor Cyan

# Check for Rust
if (!(Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host "❌ Error: Rust/Cargo not found. Install from https://rustup.rs" -ForegroundColor Red
    exit 1
}

Write-Host "📦 Building library..." -ForegroundColor Yellow
cargo build --release --all-features

if ($LASTEXITCODE -eq 0) {
    Write-Host "✅ Build successful!" -ForegroundColor Green

    Write-Host "`n🧪 Running tests..." -ForegroundColor Yellow
    cargo test --all-features

    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ All tests passed!" -ForegroundColor Green
    } else {
        Write-Host "❌ Tests failed!" -ForegroundColor Red
        exit 1
    }
} else {
    Write-Host "❌ Build failed!" -ForegroundColor Red
    exit 1
}

Write-Host "`n📊 Build complete! Binary at: target/release/" -ForegroundColor Cyan
