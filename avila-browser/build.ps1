# Build script for Avila Browser releases

param(
    [string]$Target = "all"
)

Write-Host "Building Avila Browser..." -ForegroundColor Cyan

# Clean previous builds
if (Test-Path "target/release") {
    Remove-Item -Recurse -Force "target/release"
}

# Create releases directory
New-Item -ItemType Directory -Force -Path "releases" | Out-Null

# Build release
Write-Host "Compiling release build..." -ForegroundColor Yellow
cargo build --release --examples

if ($LASTEXITCODE -ne 0) {
    Write-Host "Build failed!" -ForegroundColor Red
    exit 1
}

Write-Host "Build successful!" -ForegroundColor Green

# Package Windows
if ($Target -eq "all" -or $Target -eq "windows") {
    Write-Host "Packaging Windows release..." -ForegroundColor Yellow
    
    $WindowsDir = "releases/avila-browser-windows-x64"
    New-Item -ItemType Directory -Force -Path $WindowsDir | Out-Null
    
    Copy-Item "../target/release/examples/browser_demo.exe" "$WindowsDir/avila-browser.exe"
    Copy-Item "README.md" "$WindowsDir/"
    Copy-Item "LICENSE" "$WindowsDir/"
    Copy-Item "assets/logo.svg" "$WindowsDir/"
    
    # Create installer script
    @"
@echo off
echo Avila Browser Installer
echo.
echo Installing to: %LOCALAPPDATA%\AvilaBrowser
echo.

mkdir "%LOCALAPPDATA%\AvilaBrowser" 2>nul
copy avila-browser.exe "%LOCALAPPDATA%\AvilaBrowser\" >nul
copy logo.svg "%LOCALAPPDATA%\AvilaBrowser\" >nul

echo.
echo Installation complete!
echo.
echo Run: %LOCALAPPDATA%\AvilaBrowser\avila-browser.exe
echo.
pause
"@ | Out-File -FilePath "$WindowsDir/install.bat" -Encoding ASCII
    
    Compress-Archive -Path "$WindowsDir/*" -DestinationPath "releases/avila-browser-windows-x64.zip" -Force
    Write-Host "Windows package created: releases/avila-browser-windows-x64.zip" -ForegroundColor Green
}

# Package Linux
if ($Target -eq "all" -or $Target -eq "linux") {
    Write-Host "Packaging Linux release..." -ForegroundColor Yellow
    
    $LinuxDir = "releases/avila-browser-linux-x64"
    New-Item -ItemType Directory -Force -Path $LinuxDir | Out-Null
    
    # Note: This assumes cross-compilation is set up
    # For actual Linux build, run on Linux system or use cross
    Write-Host "Note: Linux build requires cross-compilation setup" -ForegroundColor Yellow
    Write-Host "Run on Linux: cargo build --release --target x86_64-unknown-linux-gnu" -ForegroundColor Yellow
}

# Package macOS
if ($Target -eq "all" -or $Target -eq "macos") {
    Write-Host "Packaging macOS release..." -ForegroundColor Yellow
    
    Write-Host "Note: macOS build requires compilation on macOS" -ForegroundColor Yellow
    Write-Host "Run on macOS: cargo build --release --target x86_64-apple-darwin" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "Build complete!" -ForegroundColor Green
Write-Host "Releases available in: releases/" -ForegroundColor Cyan
