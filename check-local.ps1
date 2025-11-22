#!/usr/bin/env pwsh
# check-local.ps1 - Verifica código localmente antes de fazer push

Write-Host "`n🔍 Arxis - Pre-push checks" -ForegroundColor Cyan
Write-Host "=" * 60 -ForegroundColor Gray

$ErrorActionPreference = "Stop"

# 1. Formatação
Write-Host "`n1️⃣  Checking formatting..." -ForegroundColor Yellow
cargo fmt --all -- --check
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Formatting failed! Run: cargo fmt --all" -ForegroundColor Red
    exit 1
}
Write-Host "✅ Formatting OK" -ForegroundColor Green

# 2. Clippy (sem features python)
Write-Host "`n2️⃣  Running Clippy..." -ForegroundColor Yellow
cargo clippy --workspace --lib -- -D warnings
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Clippy failed!" -ForegroundColor Red
    exit 1
}
Write-Host "✅ Clippy OK" -ForegroundColor Green

# 3. Build (sem features python)
Write-Host "`n3️⃣  Building workspace..." -ForegroundColor Yellow
cargo build --workspace --lib
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Build failed!" -ForegroundColor Red
    exit 1
}
Write-Host "✅ Build OK" -ForegroundColor Green

# 4. Testes (sem features python)
Write-Host "`n4️⃣  Running tests..." -ForegroundColor Yellow
cargo test --workspace --lib
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Tests failed!" -ForegroundColor Red
    exit 1
}
Write-Host "✅ Tests OK" -ForegroundColor Green

# Summary
Write-Host "`n" + ("=" * 60) -ForegroundColor Gray
Write-Host "✅ All checks passed! Safe to push." -ForegroundColor Green
Write-Host "`n💡 Tip: Run 'cargo fmt --all' before committing" -ForegroundColor Cyan
Write-Host ""
