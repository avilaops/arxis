# 🔧 AvilaDB Benchmarks - Setup Guide

## ⚠️ Prerequisites

### Windows

Para compilar os benchmarks no Windows, você precisa instalar o **LLVM** (para libclang):

#### Opção 1: Via Chocolatey (Recomendado)
```powershell
choco install llvm
```

#### Opção 2: Download Manual
1. Baixar LLVM de: https://github.com/llvm/llvm-project/releases
2. Instalar e adicionar ao PATH
3. Configurar variável de ambiente:
   ```powershell
   $env:LIBCLANG_PATH = "C:\Program Files\LLVM\bin"
   ```

#### Opção 3: Via Visual Studio
Instalar "C++ Clang tools for Windows" no Visual Studio Installer

### Linux

```bash
# Ubuntu/Debian
sudo apt-get install clang libclang-dev

# Fedora/RHEL
sudo dnf install clang clang-devel

# Arch Linux
sudo pacman -S clang
```

### macOS

```bash
# Xcode Command Line Tools (inclui clang)
xcode-select --install

# Ou via Homebrew
brew install llvm
```

---

## 🚀 Quick Start

Após instalar as dependências:

```powershell
# 1. Verificar instalação
cargo check --benches

# 2. Rodar benchmarks
cargo bench

# 3. Analisar resultados
.\scripts\analyze_benchmarks.ps1 -GenerateHTML
```

---

## 🐛 Troubleshooting

### Erro: "Unable to find libclang"

**Solução Windows:**
```powershell
# Instalar LLVM
choco install llvm

# Ou configurar PATH manualmente
$env:LIBCLANG_PATH = "C:\Program Files\LLVM\bin"
$env:PATH += ";C:\Program Files\LLVM\bin"
```

**Solução Linux:**
```bash
sudo apt-get install libclang-dev
```

### Erro: "link.exe not found"

**Solução:** Instalar Visual Studio Build Tools
```powershell
choco install visualstudio2022buildtools --package-parameters "--add Microsoft.VisualStudio.Workload.VCTools"
```

### Erro: PowerShell script não executa

**Solução:**
```powershell
Set-ExecutionPolicy -Scope CurrentUser RemoteSigned
```

---

## ✅ Verificação Completa

Execute este script para verificar todas as dependências:

```powershell
# check_dependencies.ps1
Write-Host "🔍 Checking AvilaDB Benchmark Dependencies..." -ForegroundColor Cyan
Write-Host ""

# Check Rust
Write-Host "Checking Rust..." -NoNewline
if (Get-Command cargo -ErrorAction SilentlyContinue) {
    $rustVersion = cargo --version
    Write-Host " ✅ $rustVersion" -ForegroundColor Green
} else {
    Write-Host " ❌ Not found" -ForegroundColor Red
    Write-Host "   Install from: https://rustup.rs/"
}

# Check LLVM/Clang
Write-Host "Checking LLVM/Clang..." -NoNewline
if (Get-Command clang -ErrorAction SilentlyContinue) {
    $clangVersion = clang --version | Select-Object -First 1
    Write-Host " ✅ $clangVersion" -ForegroundColor Green
} else {
    Write-Host " ❌ Not found" -ForegroundColor Red
    Write-Host "   Install: choco install llvm"
}

# Check PowerShell
Write-Host "Checking PowerShell..." -NoNewline
$psVersion = $PSVersionTable.PSVersion
if ($psVersion.Major -ge 5) {
    Write-Host " ✅ Version $psVersion" -ForegroundColor Green
} else {
    Write-Host " ⚠️  Version $psVersion (5.0+ recommended)" -ForegroundColor Yellow
}

# Check Visual Studio Build Tools (Windows)
if ($IsWindows -or $env:OS -eq "Windows_NT") {
    Write-Host "Checking VS Build Tools..." -NoNewline
    if (Get-Command cl -ErrorAction SilentlyContinue) {
        Write-Host " ✅ Found" -ForegroundColor Green
    } else {
        Write-Host " ⚠️  Not found" -ForegroundColor Yellow
        Write-Host "   Recommended: choco install visualstudio2022buildtools"
    }
}

Write-Host ""
Write-Host "✅ Dependency check complete!" -ForegroundColor Green
Write-Host ""
Write-Host "Next steps:" -ForegroundColor Cyan
Write-Host "  cargo bench                                  # Run benchmarks"
Write-Host "  .\scripts\analyze_benchmarks.ps1 -GenerateHTML   # Analyze results"
```

Salve como `check_dependencies.ps1` e execute:
```powershell
.\check_dependencies.ps1
```

---

## 📚 Mais Informações

- **Documentação completa**: [benches/README.md](./benches/README.md)
- **Quick start**: [benches/QUICKSTART.md](./benches/QUICKSTART.md)
- **Overview**: [BENCHMARKS_COMPLETE.md](./BENCHMARKS_COMPLETE.md)

---

**AvilaDB** - The fastest NoSQL for Brazil 🇧🇷
