# AvilaDB SDK - Validation Script
# Run this script to validate the SDK before publication

param(
    [switch]$DryRun = $false,
    [switch]$Verbose = $false
)

$ErrorActionPreference = "Stop"

Write-Host "🗄️  AvilaDB SDK - Validation Script`n" -ForegroundColor Cyan

# Change to aviladb directory
Set-Location "$PSScriptRoot"

# Track results
$results = @{
    Passed = @()
    Failed = @()
    Warnings = @()
}

function Test-Step {
    param(
        [string]$Name,
        [scriptblock]$Action
    )

    Write-Host "▶️  $Name..." -NoNewline

    try {
        & $Action
        Write-Host " ✅" -ForegroundColor Green
        $results.Passed += $Name
        return $true
    }
    catch {
        Write-Host " ❌" -ForegroundColor Red
        if ($Verbose) {
            Write-Host "   Error: $_" -ForegroundColor Red
        }
        $results.Failed += $Name
        return $false
    }
}

Write-Host "=== 1. Code Quality Checks ===`n" -ForegroundColor Yellow

Test-Step "Format check" {
    $output = cargo fmt --check 2>&1
    if ($LASTEXITCODE -ne 0) {
        throw "Code formatting issues found. Run: cargo fmt"
    }
}

Test-Step "Clippy lints" {
    $output = cargo clippy --all-features -- -D warnings 2>&1
    if ($LASTEXITCODE -ne 0) {
        throw "Clippy warnings found"
    }
}

Write-Host "`n=== 2. Build Tests ===`n" -ForegroundColor Yellow

Test-Step "Build (debug)" {
    cargo build 2>&1 | Out-Null
    if ($LASTEXITCODE -ne 0) {
        throw "Build failed"
    }
}

Test-Step "Build (release)" {
    cargo build --release 2>&1 | Out-Null
    if ($LASTEXITCODE -ne 0) {
        throw "Release build failed"
    }
}

Test-Step "Build with all features" {
    cargo build --all-features 2>&1 | Out-Null
    if ($LASTEXITCODE -ne 0) {
        throw "Build with all features failed"
    }
}

Write-Host "`n=== 3. Tests ===`n" -ForegroundColor Yellow

Test-Step "Unit tests" {
    cargo test --lib 2>&1 | Out-Null
    if ($LASTEXITCODE -ne 0) {
        throw "Unit tests failed"
    }
}

Test-Step "Integration tests" {
    cargo test --test integration_tests 2>&1 | Out-Null
    if ($LASTEXITCODE -ne 0) {
        throw "Integration tests failed"
    }
}

Test-Step "Doc tests" {
    cargo test --doc 2>&1 | Out-Null
    if ($LASTEXITCODE -ne 0) {
        throw "Doc tests failed"
    }
}

Write-Host "`n=== 4. Documentation ===`n" -ForegroundColor Yellow

Test-Step "Generate docs" {
    cargo doc --no-deps --all-features 2>&1 | Out-Null
    if ($LASTEXITCODE -ne 0) {
        throw "Documentation generation failed"
    }
}

Test-Step "Check README.md" {
    if (-not (Test-Path "README.md")) {
        throw "README.md not found"
    }
}

Test-Step "Check CHANGELOG.md" {
    if (-not (Test-Path "CHANGELOG.md")) {
        $results.Warnings += "CHANGELOG.md not found (recommended)"
    }
}

Write-Host "`n=== 5. Examples ===`n" -ForegroundColor Yellow

$examples = @("quickstart", "game_backend", "ai_chat_rag", "iot_telemetry")

foreach ($example in $examples) {
    Test-Step "Build example: $example" {
        cargo build --example $example 2>&1 | Out-Null
        if ($LASTEXITCODE -ne 0) {
            throw "Example build failed"
        }
    }
}

Write-Host "`n=== 6. Benchmarks ===`n" -ForegroundColor Yellow

Test-Step "Build benchmarks" {
    cargo bench --no-run 2>&1 | Out-Null
    if ($LASTEXITCODE -ne 0) {
        throw "Benchmark build failed"
    }
}

Write-Host "`n=== 7. Package Validation ===`n" -ForegroundColor Yellow

Test-Step "Cargo package" {
    cargo package --allow-dirty 2>&1 | Out-Null
    if ($LASTEXITCODE -ne 0) {
        throw "Package creation failed"
    }
}

if ($DryRun) {
    Test-Step "Publish dry-run" {
        cargo publish --dry-run --allow-dirty 2>&1 | Out-Null
        if ($LASTEXITCODE -ne 0) {
            throw "Publish dry-run failed"
        }
    }
}

Write-Host "`n=== 8. Dependency Audit ===`n" -ForegroundColor Yellow

Test-Step "Check Cargo.toml" {
    $cargo_toml = Get-Content "Cargo.toml" -Raw

    # Check for required fields
    $required = @("name", "version", "edition", "license", "description", "repository")
    foreach ($field in $required) {
        if ($cargo_toml -notmatch "$field\s*=") {
            throw "Missing required field: $field"
        }
    }
}

# Summary
Write-Host "`n" + ("=" * 60) -ForegroundColor Cyan
Write-Host "📊 VALIDATION SUMMARY" -ForegroundColor Cyan
Write-Host ("=" * 60) -ForegroundColor Cyan

Write-Host "`n✅ Passed: $($results.Passed.Count)" -ForegroundColor Green
foreach ($test in $results.Passed) {
    Write-Host "   - $test" -ForegroundColor Gray
}

if ($results.Failed.Count -gt 0) {
    Write-Host "`n❌ Failed: $($results.Failed.Count)" -ForegroundColor Red
    foreach ($test in $results.Failed) {
        Write-Host "   - $test" -ForegroundColor Red
    }
}

if ($results.Warnings.Count -gt 0) {
    Write-Host "`n⚠️  Warnings: $($results.Warnings.Count)" -ForegroundColor Yellow
    foreach ($warning in $results.Warnings) {
        Write-Host "   - $warning" -ForegroundColor Yellow
    }
}

Write-Host "`n" + ("=" * 60) -ForegroundColor Cyan

# Exit with appropriate code
if ($results.Failed.Count -gt 0) {
    Write-Host "`n❌ VALIDATION FAILED - Fix errors before publishing`n" -ForegroundColor Red
    exit 1
} else {
    Write-Host "`n✅ VALIDATION PASSED - SDK is ready!`n" -ForegroundColor Green

    if (-not $DryRun) {
        Write-Host "💡 Next step: Run with -DryRun to test publishing`n" -ForegroundColor Cyan
    } else {
        Write-Host "🚀 Ready to publish: cargo publish`n" -ForegroundColor Green
    }

    exit 0
}
