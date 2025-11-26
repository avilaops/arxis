# Setup Instructions

## Prerequisites

### 1. Install Rust

Windows (PowerShell):
```powershell
# Download and run rustup installer
Invoke-WebRequest -Uri https://win.rustup.rs/x86_64 -OutFile rustup-init.exe
.\rustup-init.exe
```

Or visit: https://rustup.rs/

After installation, verify:
```bash
rustc --version
cargo --version
```

### 2. Build the Project

```bash
cd d:\GitHub\Avelan\copilot-agents\geospatial-analysis
cargo build --release
```

### 3. Run Tests

```bash
cargo test
```

Expected output: All tests should pass ✅

### 4. Run Benchmarks

```bash
cargo bench
```

This will generate HTML reports in `target/criterion/`.

### 5. Run Examples

```bash
cargo run --example aviladb_integration
```

### 6. Generate Documentation

```bash
cargo doc --open
```

This will open the full API documentation in your browser.

## Quick Verification

After setup, run this to verify everything works:

```bash
# Run all checks
cargo test && cargo clippy && cargo fmt --check
```

## Troubleshooting

### "cargo: command not found"
- Restart your terminal after installing Rust
- Make sure `~/.cargo/bin` is in your PATH

### Compilation errors
- Update Rust: `rustup update`
- Clean and rebuild: `cargo clean && cargo build`

### Missing dependencies
- All dependencies will be downloaded automatically by Cargo

## Next Steps

1. Read `README.md` for usage examples
2. Check `DEVELOPMENT.md` for development guide
3. Explore `examples/` directory for practical examples
4. Review `INSTRUCTIONS.md` for the Copilot Agent identity

## Editor Setup (VS Code)

Recommended extensions:
- `rust-analyzer` - Rust language server
- `crates` - Crates.io dependency management
- `better-toml` - TOML syntax support

## Performance Tuning

For maximum performance:
```bash
RUSTFLAGS="-C target-cpu=native" cargo build --release
```

This enables CPU-specific optimizations.
