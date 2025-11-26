# Setup Guide - Financial Optimization Agent

## Prerequisites

1. **Rust** (1.75+)
   ```bash
   # Install Rust
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

   # Or on Windows
   # Download from: https://rustup.rs/
   ```

2. **System Dependencies**

   **Ubuntu/Debian**:
   ```bash
   sudo apt-get update
   sudo apt-get install -y build-essential libssl-dev pkg-config
   ```

   **macOS**:
   ```bash
   brew install openssl
   ```

   **Windows**:
   - Install Visual Studio Build Tools
   - Or use MSYS2/MinGW

3. **Docker** (optional)
   - For containerized deployment

## Installation Steps

### 1. Clone Repository
```bash
git clone https://github.com/avelan/copilot-agents
cd copilot-agents/financial-optimization
```

### 2. Setup Environment
```bash
# Copy environment template
cp .env.example .env

# Edit configuration (optional)
nano .env
```

### 3. Build Project
```bash
# Debug build (faster)
cargo build

# Release build (optimized)
cargo build --release
```

### 4. Run Tests
```bash
# Run all tests
cargo test

# Run specific module tests
cargo test portugal_tax::tests
cargo test vat_optimizer::tests
cargo test financial_models::tests

# Run with output
cargo test -- --nocapture
```

### 5. Start Server
```bash
# Debug mode
cargo run

# Release mode (production)
cargo run --release
```

Server will start on `http://localhost:3000`

### 6. Verify Installation
```bash
# Health check
curl http://localhost:3000/health

# Or use PowerShell test script
pwsh ./test_api.ps1
```

## Docker Deployment

### Build Image
```bash
docker build -t financial-optimization-agent .
```

### Run Container
```bash
docker run -p 3000:3000 financial-optimization-agent
```

### Docker Compose
```bash
docker-compose up -d
```

## Development Workflow

### Watch Mode (auto-reload)
```bash
# Install cargo-watch
cargo install cargo-watch

# Run with auto-reload
cargo watch -x run
```

### Format Code
```bash
cargo fmt
```

### Lint Code
```bash
cargo clippy
```

### Generate Documentation
```bash
cargo doc --open
```

## Troubleshooting

### Issue: "cargo: command not found"
**Solution**: Rust is not installed or not in PATH
```bash
# Add to PATH (Linux/macOS)
source $HOME/.cargo/env

# Windows: Restart terminal after Rust installation
```

### Issue: "linker `cc` not found"
**Solution**: Install C compiler
```bash
# Ubuntu/Debian
sudo apt-get install build-essential

# macOS
xcode-select --install
```

### Issue: "OpenSSL not found"
**Solution**: Install OpenSSL development headers
```bash
# Ubuntu/Debian
sudo apt-get install libssl-dev pkg-config

# macOS
brew install openssl
export PKG_CONFIG_PATH="/usr/local/opt/openssl/lib/pkgconfig"
```

### Issue: Port 3000 already in use
**Solution**: Change port in `.env`
```bash
PORT=3001
```

## Performance Tuning

### Production Build Optimization
Add to `Cargo.toml`:
```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
```

### Memory Settings
```bash
# Increase stack size if needed
export RUST_MIN_STACK=16777216
```

## Monitoring

### Logs
```bash
# Set log level
export RUST_LOG=financial_optimization_agent=debug

# Or in .env
RUST_LOG=financial_optimization_agent=info,tower_http=info
```

### Metrics
- **Health**: `GET /health`
- **Prometheus**: Coming soon
- **Grafana**: Coming soon

## Next Steps

1. **Test API**: Run `pwsh ./test_api.ps1`
2. **Read Examples**: See `examples/API_EXAMPLES.md`
3. **Integration**: Check main README for integration guides
4. **Production**: Review Docker deployment options

## Support

- **Documentation**: https://docs.avila.cloud/agents/financial-optimization
- **Issues**: https://github.com/avelan/copilot-agents/issues
- **Email**: dev@avila.cloud

---

Built with ❤️ for Portugal expansion 🇵🇹
