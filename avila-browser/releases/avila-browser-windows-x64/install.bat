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
