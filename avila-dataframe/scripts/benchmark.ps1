#!/usr/bin/env pwsh
# Run benchmarks

Write-Host "📊 Running AvilaDB DataFrame Benchmarks" -ForegroundColor Cyan
Write-Host "========================================`n" -ForegroundColor Cyan

Write-Host "🔥 This will compare performance against targets..." -ForegroundColor Yellow
Write-Host "Target: Beat Polars by 20-30%`n" -ForegroundColor Yellow

cargo bench --all-features

if ($LASTEXITCODE -eq 0) {
    Write-Host "`n✅ Benchmarks completed!" -ForegroundColor Green
    Write-Host "📈 Results saved to target/criterion/" -ForegroundColor Cyan
    Write-Host "🌐 Open target/criterion/report/index.html to view detailed results" -ForegroundColor Cyan
} else {
    Write-Host "`n❌ Benchmarks failed!" -ForegroundColor Red
    exit 1
}
