# Release Creation Guide

## Criar Release no GitHub

### 1. Acessar Releases

https://github.com/avilaops/arxis/releases/new

### 2. Preencher Informações

**Tag version**: `avila-browser-v0.1.0`

**Release title**: `Avila Browser v0.1.0 - Initial Release`

**Description**:

```markdown
# Avila Browser v0.1.0

Ultra-secure web browser with 7-layer anonymity protection.

## Features

- 7 protection layers (Tor + VPN + I2P + Obfuscation)
- 99.2% anonymity level
- 340ms latency
- Traffic analysis resistance (ρ < 0.30)
- DNS leak prevention
- Censorship circumvention

## Technical Specifications

- **Anonymity Level**: 99.2% (A = 1 - 1/2^7)
- **Information Entropy**: 56 bits (2^56 paths)
- **Total Latency**: 340ms
- **Bandwidth Overhead**: 2.4x

## Downloads

### Windows
- [avila-browser-windows-x64.zip](https://github.com/avilaops/arxis/releases/download/avila-browser-v0.1.0/avila-browser-windows-x64.zip)

### macOS (Coming Soon)
- Intel: avila-browser-macos-x64.tar.gz
- Apple Silicon: avila-browser-macos-arm64.tar.gz

### Linux (Coming Soon)
- x64: avila-browser-linux-x64.tar.gz
- ARM: avila-browser-linux-arm64.tar.gz

## Installation

### Windows
1. Download `avila-browser-windows-x64.zip`
2. Extract to desired location
3. Run `install.bat`
4. Launch from Start Menu or desktop shortcut

### macOS
```bash
tar -xzf avila-browser-macos-x64.tar.gz
cd avila-browser-macos-x64
./install.sh
```

### Linux
```bash
tar -xzf avila-browser-linux-x64.tar.gz
cd avila-browser-linux-x64
sudo ./install.sh
```

## Verification

Verify package integrity:

```powershell
# Windows
Get-FileHash avila-browser-windows-x64.zip -Algorithm SHA256
```

Expected SHA256:
- Windows: [hash will be here]

## Documentation

- Landing Page: https://browser.avila.inc
- User Guide: https://github.com/avilaops/arxis/tree/main/avila-browser/README.md
- Technical Docs: https://github.com/avilaops/arxis/tree/main/avila-browser/docs

## System Requirements

### Minimum
- OS: Windows 10, macOS 10.15, Linux (kernel 4.0+)
- RAM: 4GB
- Disk: 100MB
- Network: Internet connection

### Recommended
- OS: Windows 11, macOS 13+, Linux (kernel 5.0+)
- RAM: 8GB
- Disk: 500MB
- Network: High-speed connection

## Known Issues

- JavaScript engine not yet implemented (disabled for security)
- GPU acceleration not yet available
- Some websites may not render correctly

## Roadmap

- v0.2.0: JavaScript engine integration
- v0.3.0: GPU-accelerated rendering
- v0.4.0: Mobile support (Android/iOS)
- v1.0.0: Production-ready release

## License

MIT License - see [LICENSE](https://github.com/avilaops/arxis/blob/main/avila-browser/LICENSE)

## Support

- GitHub Issues: https://github.com/avilaops/arxis/issues
- Discussions: https://github.com/avilaops/arxis/discussions
```

### 3. Upload Assets

Faça upload dos seguintes arquivos:

1. `avila-browser-windows-x64.zip` (já criado em `releases/`)
2. `avila-browser-macos-x64.tar.gz` (quando disponível)
3. `avila-browser-linux-x64.tar.gz` (quando disponível)

### 4. Publicar

- Marque **Set as the latest release**
- Clique em **Publish release**

### 5. Atualizar Landing Page

Após criar a release, os links de download na landing page devem funcionar automaticamente:

```
https://github.com/avilaops/arxis/releases/latest/download/avila-browser-windows-x64.zip
```

## Comandos Úteis

### Criar SHA256 Hash

```powershell
Get-FileHash releases/avila-browser-windows-x64.zip -Algorithm SHA256 | Select-Object Hash
```

### Verificar Tamanho

```powershell
(Get-Item releases/avila-browser-windows-x64.zip).Length / 1MB
```

### Testar Download

```powershell
Invoke-WebRequest -Uri "https://github.com/avilaops/arxis/releases/latest/download/avila-browser-windows-x64.zip" -OutFile "test-download.zip"
```
