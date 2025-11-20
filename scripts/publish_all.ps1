# Avila Rust Ecosystem - Automated Publishing Script
# Usage: .\scripts\publish_all.ps1

param(
    [switch]$DryRun = $false,
    [switch]$SkipTests = $false,
    [switch]$NoConfirm = $false
)

$ErrorActionPreference = "Stop"

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  Avila Rust Ecosystem - Publisher" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# Change to project root
$ProjectRoot = Split-Path -Parent $PSScriptRoot
Set-Location $ProjectRoot

# Publishing order (respects dependencies)
$PublishOrder = @(
    @{ Name = "avila-math"; Path = "avila-math" },
    @{ Name = "avila-telemetry"; Path = "avila-telemetry" },
    @{ Name = "avx-config"; Path = "avx-config" },
    @{ Name = "avx-telemetry"; Path = "avx-telemetry" },
    @{ Name = "avx-quantum-render"; Path = "avx-quantum-render" },
    @{ Name = "avx-image"; Path = "avx-image" },
    @{ Name = "avx-gateway"; Path = "avx-gateway" },
    @{ Name = "avx-api-core"; Path = "avx-api-core" },
    @{ Name = "avx-cli"; Path = "avx-cli" },
    @{ Name = "avx-events"; Path = "avx-events" },
    @{ Name = "arxis_quaternions"; Path = "." }
)

function Test-Crate {
    param([string]$CratePath)

    Write-Host "  Running tests..." -ForegroundColor Yellow
    Push-Location $CratePath
    try {
        $result = cargo test --release 2>&1
        if ($LASTEXITCODE -ne 0) {
            Write-Host "  ❌ Tests failed!" -ForegroundColor Red
            Write-Host $result
            return $false
        }
        Write-Host "  ✅ Tests passed" -ForegroundColor Green
        return $true
    }
    finally {
        Pop-Location
    }
}

function Build-Docs {
    param([string]$CratePath)

    Write-Host "  Building documentation..." -ForegroundColor Yellow
    Push-Location $CratePath
    try {
        $result = cargo doc --no-deps 2>&1
        if ($LASTEXITCODE -ne 0) {
            Write-Host "  ⚠️  Documentation build failed!" -ForegroundColor Yellow
            Write-Host $result
            return $false
        }
        Write-Host "  ✅ Documentation built" -ForegroundColor Green
        return $true
    }
    finally {
        Pop-Location
    }
}

function Publish-Crate {
    param(
        [string]$CrateName,
        [string]$CratePath
    )

    Write-Host ""
    Write-Host "📦 Publishing: $CrateName" -ForegroundColor Cyan
    Write-Host "   Path: $CratePath" -ForegroundColor Gray

    # Test crate
    if (-not $SkipTests) {
        if (-not (Test-Crate $CratePath)) {
            throw "Tests failed for $CrateName"
        }
    }

    # Build docs
    if (-not (Build-Docs $CratePath)) {
        Write-Host "  ⚠️  Continuing despite documentation issues..." -ForegroundColor Yellow
    }

    # Publish
    if ($DryRun) {
        Write-Host "  🔍 DRY RUN: Would publish $CrateName" -ForegroundColor Magenta
        Push-Location $CratePath
        cargo publish --dry-run
        Pop-Location
    }
    else {
        Write-Host "  🚀 Publishing to crates.io..." -ForegroundColor Green
        Push-Location $CratePath
        try {
            cargo publish
            if ($LASTEXITCODE -ne 0) {
                throw "Publish failed for $CrateName"
            }
            Write-Host "  ✅ Published successfully!" -ForegroundColor Green
        }
        finally {
            Pop-Location
        }

        # Wait for crates.io indexing
        Write-Host "  ⏳ Waiting 45s for crates.io indexing..." -ForegroundColor Yellow
        Start-Sleep -Seconds 45
    }
}

# Pre-flight checks
Write-Host "🔍 Pre-flight checks..." -ForegroundColor Yellow
Write-Host ""

# Check if logged in to crates.io
Write-Host "  Checking crates.io authentication..." -ForegroundColor Gray
$credentials = "$env:USERPROFILE\.cargo\credentials.toml"
if (-not (Test-Path $credentials)) {
    Write-Host "  ❌ Not logged in to crates.io!" -ForegroundColor Red
    Write-Host "  Run: cargo login <token>" -ForegroundColor Yellow
    exit 1
}
Write-Host "  ✅ Authenticated" -ForegroundColor Green

# Check git status
Write-Host "  Checking git status..." -ForegroundColor Gray
$gitStatus = git status --porcelain
if ($gitStatus -and -not $DryRun -and -not $NoConfirm) {
    Write-Host "  ⚠️  Uncommitted changes detected!" -ForegroundColor Yellow
    Write-Host "  Consider committing changes before publishing." -ForegroundColor Yellow
    $continue = Read-Host "  Continue anyway? (y/N)"
    if ($continue -ne "y") {
        Write-Host "  Aborted." -ForegroundColor Red
        exit 1
    }
}
elseif ($gitStatus) {
    Write-Host "  ⚠️  Uncommitted changes (continuing anyway)" -ForegroundColor Yellow
}
else {
    Write-Host "  ✅ Git clean" -ForegroundColor Green
}

# Check LICENSE files
Write-Host "  Checking LICENSE files..." -ForegroundColor Gray
if (-not (Test-Path "LICENSE-MIT") -or -not (Test-Path "LICENSE-APACHE")) {
    Write-Host "  ❌ LICENSE files missing!" -ForegroundColor Red
    exit 1
}
Write-Host "  ✅ LICENSE files present" -ForegroundColor Green

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  Ready to publish!" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

if ($DryRun) {
    Write-Host "🔍 DRY RUN MODE - No actual publishing" -ForegroundColor Magenta
    Write-Host ""
}

# Confirm
if (-not $DryRun -and -not $NoConfirm) {
    Write-Host "⚠️  This will publish $($PublishOrder.Count) crates to crates.io" -ForegroundColor Yellow
    Write-Host "   This action CANNOT be undone!" -ForegroundColor Yellow
    Write-Host ""
    $confirm = Read-Host "Are you sure you want to continue? (yes/N)"
    if ($confirm -ne "yes") {
        Write-Host "Aborted." -ForegroundColor Red
        exit 0
    }
}
elseif (-not $DryRun) {
    Write-Host "⚠️  Publishing $($PublishOrder.Count) crates to crates.io (no confirmation)" -ForegroundColor Yellow
}

Write-Host ""

# Publish each crate in order
$published = @()
$failed = @()

foreach ($crate in $PublishOrder) {
    try {
        Publish-Crate -CrateName $crate.Name -CratePath $crate.Path
        $published += $crate.Name
    }
    catch {
        Write-Host ""
        Write-Host "❌ Failed to publish $($crate.Name)" -ForegroundColor Red
        Write-Host "   Error: $_" -ForegroundColor Red
        $failed += $crate.Name

        $continue = Read-Host "Continue with remaining crates? (y/N)"
        if ($continue -ne "y") {
            break
        }
    }
}

# Summary
Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  Publishing Summary" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

Write-Host "✅ Successfully published: $($published.Count)" -ForegroundColor Green
foreach ($name in $published) {
    Write-Host "   - $name" -ForegroundColor Green
}

if ($failed.Count -gt 0) {
    Write-Host ""
    Write-Host "❌ Failed: $($failed.Count)" -ForegroundColor Red
    foreach ($name in $failed) {
        Write-Host "   - $name" -ForegroundColor Red
    }
}

Write-Host ""
Write-Host "🔗 Check your crates at:" -ForegroundColor Cyan
Write-Host "   https://crates.io/users/nicolasavila" -ForegroundColor Gray
Write-Host ""
Write-Host "📚 Documentation will be available at:" -ForegroundColor Cyan
foreach ($name in $published) {
    Write-Host "   https://docs.rs/$name" -ForegroundColor Gray
}

Write-Host ""
Write-Host "🎉 Done!" -ForegroundColor Green
