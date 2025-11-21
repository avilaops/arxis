#!/usr/bin/env pwsh
# Run all examples

Write-Host "🎯 Running AvilaDB DataFrame Examples" -ForegroundColor Cyan
Write-Host "======================================`n" -ForegroundColor Cyan

$examples = @("basic_usage", "scientific_types")

foreach ($example in $examples) {
    Write-Host "Running example: $example" -ForegroundColor Yellow
    Write-Host "-----------------------------------" -ForegroundColor Gray

    cargo run --example $example

    if ($LASTEXITCODE -ne 0) {
        Write-Host "❌ Example '$example' failed!" -ForegroundColor Red
        exit 1
    }

    Write-Host "`n" -ForegroundColor Gray
}

Write-Host "✅ All examples completed successfully!" -ForegroundColor Green
