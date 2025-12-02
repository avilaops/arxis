# Setup CI/CD - GitHub Actions

Write-Host "Setup CI/CD - ARXIS" -ForegroundColor Magenta
Write-Host ""

# Criar diretório .github/workflows
$workflowDir = ".github\workflows"
if (!(Test-Path $workflowDir)) {
    New-Item -ItemType Directory -Path $workflowDir -Force | Out-Null
    Write-Host "✓ Criado: $workflowDir" -ForegroundColor Green
}

# CI Workflow
$ciWorkflow = @'
name: CI

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

env:
  RUST_BACKTRACE: 1
  CARGO_TERM_COLOR: always

jobs:
  test:
    name: Test
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt, clippy
      
      - name: Cache cargo registry
        uses: actions/cache@v3
        with:
          path: ~/.cargo/registry
          key: ${{ runner.os }}-cargo-registry-${{ hashFiles('**/Cargo.lock') }}
      
      - name: Cache cargo index
        uses: actions/cache@v3
        with:
          path: ~/.cargo/git
          key: ${{ runner.os }}-cargo-index-${{ hashFiles('**/Cargo.lock') }}
      
      - name: Cache cargo build
        uses: actions/cache@v3
        with:
          path: target
          key: ${{ runner.os }}-cargo-build-target-${{ hashFiles('**/Cargo.lock') }}
      
      - name: Check formatting
        run: cargo fmt --all -- --check
      
      - name: Run clippy
        run: cargo clippy --all-targets --all-features -- -D warnings
      
      - name: Run tests
        run: cargo test --all-features --workspace
      
      - name: Run doc tests
        run: cargo test --doc --workspace

  coverage:
    name: Code Coverage
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
      
      - name: Install tarpaulin
        run: cargo install cargo-tarpaulin
      
      - name: Generate coverage
        run: cargo tarpaulin --out Xml --workspace
      
      - name: Upload coverage
        uses: codecov/codecov-action@v3
        with:
          files: ./cobertura.xml

  security-audit:
    name: Security Audit
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Run cargo-audit
        uses: actions-rs/audit-check@v1
        with:
          token: ${{ secrets.GITHUB_TOKEN }}
'@

$ciPath = Join-Path $workflowDir "ci.yml"
$ciWorkflow | Out-File -FilePath $ciPath -Encoding UTF8
Write-Host "✓ Criado: $ciPath" -ForegroundColor Green

# Release Workflow
$releaseWorkflow = @'
name: Release

on:
  push:
    tags:
      - 'v*.*.*'

env:
  CARGO_TERM_COLOR: always

jobs:
  publish:
    name: Publish to crates.io
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
      
      - name: Publish
        run: |
          cargo login ${{ secrets.CARGO_TOKEN }}
          cargo publish --allow-dirty
        env:
          CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_TOKEN }}
      
      - name: Create GitHub Release
        uses: softprops/action-gh-release@v1
        with:
          generate_release_notes: true
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
'@

$releasePath = Join-Path $workflowDir "release.yml"
$releaseWorkflow | Out-File -FilePath $releasePath -Encoding UTF8
Write-Host "✓ Criado: $releasePath" -ForegroundColor Green

Write-Host ""
Write-Host "CI/CD configurado com sucesso!" -ForegroundColor Green
Write-Host ""
Write-Host "Workflows criados:" -ForegroundColor Cyan
Write-Host "  - ci.yml: Testes, clippy, fmt" -ForegroundColor White
Write-Host "  - release.yml: Publicacao automatica" -ForegroundColor White
Write-Host ""
Write-Host "Proximos passos:" -ForegroundColor Cyan
Write-Host "  1. Adicionar secret CARGO_TOKEN no GitHub" -ForegroundColor White
Write-Host "  2. Ativar branch protection" -ForegroundColor White
Write-Host "  3. git add .github/" -ForegroundColor White
Write-Host "  4. git commit -m 'ci: setup GitHub Actions'" -ForegroundColor White
Write-Host "  5. git push" -ForegroundColor White
