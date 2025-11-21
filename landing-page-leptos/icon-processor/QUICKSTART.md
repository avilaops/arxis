# Arxis Icon Processor - Quick Start

## 🚀 Installation

```bash
# 1. Make sure you have Rust installed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. Build the project
cd icon-processor
cargo build --release

# 3. The binary will be at: target/release/arxis-icons
```

## ⚡ Basic Usage

```bash
# Process images from ./input to ./output
./target/release/arxis-icons

# Or with custom directories
./target/release/arxis-icons --input ./logos --output ./icons

# With background removal
./target/release/arxis-icons --input ./logos --remove-bg
```

## 📋 Common Commands

```bash
# List all platforms
./target/release/arxis-icons list

# Generate only web icons
./target/release/arxis-icons --platforms favicon,social

# Full processing with all options
./target/release/arxis-icons \
  --input ./my-logos \
  --output ./generated \
  --remove-bg \
  --quality 95
```

## 📁 Prepare Your Images

1. Create an `input` folder
2. Put your logo/icon files there (PNG, JPG, WebP supported)
3. Run the processor
4. Check the `output` folder for results

## 🎯 What You Get

For each input image:
- 40+ optimized icon sizes
- `favicon.ico` for browsers
- `manifest.json` for PWA
- `html_snippet.txt` with ready-to-use meta tags
- `README.md` with instructions

## 💡 Tips

- Use high-resolution input (1024x1024 or larger)
- Square images work best
- PNG with transparency is ideal
- Use `--remove-bg` for logos with backgrounds

## ❓ Help

```bash
./target/release/arxis-icons --help
```

---

Full documentation: [README.md](README.md)
