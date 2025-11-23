# Build script for avila-ml standalone
Write-Host "Building Avila ML..." -ForegroundColor Cyan

# Build the library
cargo build --release --lib

if ($LASTEXITCODE -eq 0) {
    Write-Host "✓ Build successful!" -ForegroundColor Green
}
else {
    Write-Host "✗ Build failed!" -ForegroundColor Red
    exit 1
}

# Run tests
Write-Host "`nRunning tests..." -ForegroundColor Cyan
cargo test --lib

if ($LASTEXITCODE -eq 0) {
    Write-Host "✓ Tests passed!" -ForegroundColor Green
}
else {
    Write-Host "✗ Tests failed!" -ForegroundColor Red
    exit 1
}

Write-Host "`n✨ Avila ML is ready!" -ForegroundColor Green
