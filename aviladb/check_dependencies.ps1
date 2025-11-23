#!/usr/bin/env pwsh
# AvilaDB Benchmark Dependencies Checker

Write-Host ""
Write-Host "🔍 Checking AvilaDB Benchmark Dependencies..." -ForegroundColor Cyan
Write-Host "================================================" -ForegroundColor Cyan
Write-Host ""

$allGood = $true

# Check Rust
Write-Host "[1/6] Checking Rust..." -NoNewline
if (Get-Command cargo -ErrorAction SilentlyContinue) {
    $rustVersion = (cargo --version) -replace 'cargo ', ''
    Write-Host " ✅" -ForegroundColor Green
    Write-Host "      Version: $rustVersion" -ForegroundColor Gray
}
else {
    Write-Host " ❌" -ForegroundColor Red
    Write-Host "      Install from: https://rustup.rs/" -ForegroundColor Yellow
    $allGood = $false
}

# Check LLVM/Clang
Write-Host "[2/6] Checking LLVM/Clang..." -NoNewline
if (Get-Command clang -ErrorAction SilentlyContinue) {
    $clangVersion = (clang --version | Select-Object -First 1) -replace 'clang version ', ''
    Write-Host " ✅" -ForegroundColor Green
    Write-Host "      Version: $clangVersion" -ForegroundColor Gray
}
else {
    Write-Host " ❌" -ForegroundColor Red
    if ($IsWindows -or $env:OS -eq "Windows_NT") {
        Write-Host "      Install: choco install llvm" -ForegroundColor Yellow
    }
    elseif ($IsLinux) {
        Write-Host "      Install: sudo apt-get install clang libclang-dev" -ForegroundColor Yellow
    }
    elseif ($IsMacOS) {
        Write-Host "      Install: xcode-select --install" -ForegroundColor Yellow
    }
    $allGood = $false
}

# Check libclang
Write-Host "[3/6] Checking libclang..." -NoNewline
if ($env:LIBCLANG_PATH) {
    Write-Host " ✅" -ForegroundColor Green
    Write-Host "      Path: $env:LIBCLANG_PATH" -ForegroundColor Gray
}
else {
    # Try to find it
    $libclangPaths = @(
        "C:\Program Files\LLVM\bin",
        "C:\Program Files (x86)\LLVM\bin",
        "/usr/lib/llvm-*/lib",
        "/usr/local/opt/llvm/lib"
    )

    $found = $false
    foreach ($path in $libclangPaths) {
        if (Test-Path $path -ErrorAction SilentlyContinue) {
            Write-Host " ⚠️" -ForegroundColor Yellow
            Write-Host "      Found at: $path" -ForegroundColor Gray
            Write-Host "      Set LIBCLANG_PATH environment variable" -ForegroundColor Yellow
            $found = $true
            break
        }
    }

    if (-not $found) {
        Write-Host " ❌" -ForegroundColor Red
        Write-Host "      Not found. Install LLVM first." -ForegroundColor Yellow
        $allGood = $false
    }
}

# Check PowerShell
Write-Host "[4/6] Checking PowerShell..." -NoNewline
$psVersion = $PSVersionTable.PSVersion
if ($psVersion.Major -ge 5) {
    Write-Host " ✅" -ForegroundColor Green
    Write-Host "      Version: $($psVersion.Major).$($psVersion.Minor)" -ForegroundColor Gray
}
else {
    Write-Host " ⚠️" -ForegroundColor Yellow
    Write-Host "      Version $($psVersion.Major).$($psVersion.Minor) (5.0+ recommended)" -ForegroundColor Yellow
}

# Check C++ Build Tools (Windows only)
if ($IsWindows -or $env:OS -eq "Windows_NT") {
    Write-Host "[5/6] Checking C++ Build Tools..." -NoNewline
    if (Get-Command cl -ErrorAction SilentlyContinue) {
        Write-Host " ✅" -ForegroundColor Green
        $clVersion = (cl 2>&1 | Select-String "Version" | Select-Object -First 1) -replace '.*Version ', '' -replace ' .*', ''
        Write-Host "      MSVC Version: $clVersion" -ForegroundColor Gray
    }
    else {
        Write-Host " ⚠️" -ForegroundColor Yellow
        Write-Host "      Not found (optional but recommended)" -ForegroundColor Gray
        Write-Host "      Install: choco install visualstudio2022buildtools" -ForegroundColor Yellow
    }
}
else {
    Write-Host "[5/6] Checking GCC/Clang..." -NoNewline
    if ((Get-Command gcc -ErrorAction SilentlyContinue) -or (Get-Command clang -ErrorAction SilentlyContinue)) {
        Write-Host " ✅" -ForegroundColor Green
    }
    else {
        Write-Host " ❌" -ForegroundColor Red
        Write-Host "      Install build essentials" -ForegroundColor Yellow
        $allGood = $false
    }
}

# Check Criterion (optional)
Write-Host "[6/6] Checking Criterion..." -NoNewline
$cargoToml = Get-Content "Cargo.toml" -ErrorAction SilentlyContinue
if ($cargoToml -and ($cargoToml -match "criterion")) {
    Write-Host " ✅" -ForegroundColor Green
    Write-Host "      Configured in Cargo.toml" -ForegroundColor Gray
}
else {
    Write-Host " ⚠️" -ForegroundColor Yellow
    Write-Host "      Will be installed automatically" -ForegroundColor Gray
}

Write-Host ""
Write-Host "================================================" -ForegroundColor Cyan

if ($allGood) {
    Write-Host "✅ All required dependencies are installed!" -ForegroundColor Green
    Write-Host ""
    Write-Host "🚀 You're ready to run benchmarks:" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "   cargo bench                                  # Run all benchmarks" -ForegroundColor White
    Write-Host "   .\bench.ps1 basic                            # Run CRUD tests" -ForegroundColor White
    Write-Host "   .\scripts\analyze_benchmarks.ps1 -GenerateHTML  # Analyze results" -ForegroundColor White
    Write-Host ""
}
else {
    Write-Host "❌ Some dependencies are missing!" -ForegroundColor Red
    Write-Host ""
    Write-Host "📋 Installation Guide:" -ForegroundColor Yellow
    Write-Host ""

    if ($IsWindows -or $env:OS -eq "Windows_NT") {
        Write-Host "   Windows Quick Setup:" -ForegroundColor Cyan
        Write-Host "   1. Install Chocolatey: https://chocolatey.org/install" -ForegroundColor White
        Write-Host "   2. Run: choco install llvm -y" -ForegroundColor White
        Write-Host "   3. Run: choco install rust -y" -ForegroundColor White
        Write-Host "   4. Restart terminal and run this script again" -ForegroundColor White
    }
    elseif ($IsLinux) {
        Write-Host "   Linux Quick Setup:" -ForegroundColor Cyan
        Write-Host "   sudo apt-get update" -ForegroundColor White
        Write-Host "   sudo apt-get install -y clang libclang-dev build-essential" -ForegroundColor White
        Write-Host "   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh" -ForegroundColor White
    }
    elseif ($IsMacOS) {
        Write-Host "   macOS Quick Setup:" -ForegroundColor Cyan
        Write-Host "   xcode-select --install" -ForegroundColor White
        Write-Host "   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh" -ForegroundColor White
    }

    Write-Host ""
    Write-Host "   See SETUP_BENCHMARKS.md for detailed instructions" -ForegroundColor Gray
    Write-Host ""
}

Write-Host ""
