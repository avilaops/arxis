#!/usr/bin/env pwsh
# Development workflow - check, test, lint

param(
    [switch]$Fix
)

Write-Host "🔍 Running Development Checks..." -ForegroundColor Cyan
Write-Host "=================================`n" -ForegroundColor Cyan

# Check for Rust/Cargo
if (!(Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host "⚠️  Warning: Cargo not found in PATH. Attempting to locate..." -ForegroundColor Yellow

    # Try common Cargo locations
    $cargoPath = $null
    $possiblePaths = @(
        "$env:USERPROFILE\.cargo\bin\cargo.exe",
        "C:\Users\$env:USERNAME\.cargo\bin\cargo.exe",
        "$env:CARGO_HOME\bin\cargo.exe"
    )

    foreach ($path in $possiblePaths) {
        if (Test-Path $path) {
            $cargoPath = $path
            Write-Host "✅ Found cargo at: $cargoPath" -ForegroundColor Green
            break
        }
    }

    if (-not $cargoPath) {
        Write-Host "❌ Error: Rust/Cargo not found. Install from https://rustup.rs" -ForegroundColor Red
        Write-Host "   Or ensure cargo is in your PATH" -ForegroundColor Red
        exit 1
    }

    # Use full path
    function Invoke-Cargo {
        param([string[]]$Arguments)
        & $cargoPath $Arguments
    }
}
else {
    function Invoke-Cargo {
        param([string[]]$Arguments)
        & cargo $Arguments
    }
}

# Format check
Write-Host "1️⃣  Checking code formatting..." -ForegroundColor Yellow
if ($Fix) {
    Invoke-Cargo @("fmt")
    Write-Host "✅ Code formatted!" -ForegroundColor Green
}
else {
    Invoke-Cargo @("fmt", "--check")
    if ($LASTEXITCODE -ne 0) {
        Write-Host "❌ Formatting issues found. Run with -Fix to auto-format." -ForegroundColor Red
        exit 1
    }
    Write-Host "✅ Formatting OK!" -ForegroundColor Green
}

# Clippy
Write-Host "`n2️⃣  Running Clippy (linter)..." -ForegroundColor Yellow
if ($Fix) {
    Invoke-Cargo @("clippy", "--all-features", "--fix", "--allow-dirty")
}
else {
    Invoke-Cargo @("clippy", "--all-features", "--", "-D", "warnings")
}

if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Clippy found issues!" -ForegroundColor Red
    exit 1
}
Write-Host "✅ Clippy OK!" -ForegroundColor Green

# Tests
Write-Host "`n3️⃣  Running tests..." -ForegroundColor Yellow
Invoke-Cargo @("test", "--all-features")

if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Tests failed!" -ForegroundColor Red
    exit 1
}
Write-Host "✅ Tests passed!" -ForegroundColor Green

# Doc check
Write-Host "`n4️⃣  Checking documentation..." -ForegroundColor Yellow
Invoke-Cargo @("doc", "--all-features", "--no-deps")

if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Documentation build failed!" -ForegroundColor Red
    exit 1
}
Write-Host "✅ Documentation OK!" -ForegroundColor Green

Write-Host "`n✨ All checks passed! Ready to commit. ✨" -ForegroundColor Green
