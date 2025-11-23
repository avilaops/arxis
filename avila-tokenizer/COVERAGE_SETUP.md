# Code Coverage Setup Guide

This guide explains how to set up code coverage reporting with Codecov for the avila-tokenizer project.

## Overview

The project uses:
- **cargo-tarpaulin**: Generates code coverage reports
- **Codecov**: Visualizes coverage and tracks changes over time
- **GitHub Actions**: Automatically runs coverage on every push/PR

## Setup Instructions

### 1. Sign Up for Codecov

1. Go to [codecov.io](https://codecov.io)
2. Sign in with your GitHub account
3. Authorize Codecov to access your repositories

### 2. Add Repository to Codecov

1. In Codecov dashboard, click "Add new repository"
2. Find `avilaops/arxis` in the list
3. Click to enable coverage for this repository

### 3. Get Upload Token (Optional for Public Repos)

**Note**: For public GitHub repositories, Codecov tokens are optional. The GitHub Action can authenticate automatically using the `GITHUB_TOKEN`.

If you still want to use a token:

1. In Codecov, navigate to Settings > General
2. Copy the "Repository Upload Token"
3. In GitHub, go to repository Settings > Secrets and variables > Actions
4. Click "New repository secret"
5. Name: `CODECOV_TOKEN`
6. Value: Paste the token from Codecov
7. Click "Add secret"

### 4. Update GitHub Actions Workflow (Already Configured)

The `.github/workflows/ci.yml` already includes the coverage job:

```yaml
coverage:
    name: Code Coverage
    runs-on: ubuntu-latest
    defaults:
        run:
            working-directory: ./avila-tokenizer
    steps:
        - uses: actions/checkout@v3
        - uses: dtolnay/rust-toolchain@stable

        - name: Install cargo-tarpaulin
          run: cargo install cargo-tarpaulin

        - name: Generate coverage
          run: cargo tarpaulin --verbose --all-features --workspace --timeout 120 --out Xml

        - name: Upload coverage to Codecov
          uses: codecov/codecov-action@v3
          with:
              files: ./avila-tokenizer/cobertura.xml
              fail_ci_if_error: false
```

### 5. Configure Codecov (Optional)

Create a `codecov.yml` file in the project root to customize behavior:

```yaml
# codecov.yml
coverage:
  status:
    project:
      default:
        target: 80%
        threshold: 5%
    patch:
      default:
        target: 80%

comment:
  layout: "header, diff, files"
  behavior: default

ignore:
  - "tests/"
  - "benches/"
  - "examples/"
```

## Running Coverage Locally

### Install cargo-tarpaulin

```bash
cargo install cargo-tarpaulin
```

### Generate Coverage Report

```bash
# Basic coverage
cargo tarpaulin --out Html

# With all features
cargo tarpaulin --all-features --workspace --out Html

# View in browser
open tarpaulin-report.html  # macOS
xdg-open tarpaulin-report.html  # Linux
start tarpaulin-report.html  # Windows
```

### Generate XML for Codecov

```bash
cargo tarpaulin --all-features --workspace --out Xml
```

## Understanding Coverage Reports

### Coverage Metrics

- **Line Coverage**: Percentage of code lines executed
- **Branch Coverage**: Percentage of conditional branches tested
- **Function Coverage**: Percentage of functions called

### Target Coverage

- **Minimum**: 70% overall coverage
- **Good**: 80% overall coverage
- **Excellent**: 90%+ overall coverage

### Areas to Focus

1. **Core Algorithms**: BPE, WordPiece, Unigram (target: 95%+)
2. **Models**: GPT-2, BERT, Llama (target: 90%+)
3. **Utilities**: Normalizers, pre-tokenizers (target: 85%+)
4. **Error Handling**: All error paths (target: 100%)

## Coverage Badge

The README includes a coverage badge:

```markdown
[![codecov](https://codecov.io/gh/avilaops/arxis/branch/main/graph/badge.svg)](https://codecov.io/gh/avilaops/arxis)
```

This badge automatically updates when coverage changes.

## Troubleshooting

### Token Authentication Issues

**Problem**: "Error uploading to Codecov: HTTP Error 401"

**Solution**:
- For public repos: Remove `token` from the upload step
- For private repos: Verify `CODECOV_TOKEN` secret is set correctly

### File Path Issues

**Problem**: "Unable to locate file" in Codecov

**Solution**: Ensure the file path in the workflow matches the actual location:
```yaml
files: ./avila-tokenizer/cobertura.xml
```

### Low Coverage Warning

**Problem**: Coverage drops below threshold

**Solution**:
1. Run `cargo tarpaulin --out Html` locally
2. Open `tarpaulin-report.html` to see uncovered lines
3. Add tests for uncovered code
4. Focus on critical paths first

### Tarpaulin Installation Fails

**Problem**: cargo-tarpaulin fails to install

**Solution**:
```bash
# On Linux, you may need:
sudo apt-get install libssl-dev pkg-config

# On macOS:
brew install openssl pkg-config

# Then retry:
cargo install cargo-tarpaulin
```

## Alternative Tools

If cargo-tarpaulin doesn't work for your platform:

### llvm-cov (Rust 1.60+)

```bash
# Install llvm-tools
rustup component add llvm-tools-preview

# Generate coverage
cargo install cargo-llvm-cov
cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info
```

### grcov

```bash
# Install grcov
cargo install grcov

# Generate coverage
export RUSTFLAGS="-C instrument-coverage"
cargo build
cargo test
grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./coverage/
```

## Best Practices

1. **Write Tests First**: Aim for high coverage from the start
2. **Focus on Critical Code**: Prioritize core algorithms over utilities
3. **Test Edge Cases**: Cover error conditions and boundary cases
4. **Monitor Trends**: Track coverage over time, don't let it drop
5. **Review Coverage in PRs**: Check coverage impact before merging

## Resources

- [Codecov Documentation](https://docs.codecov.com/)
- [cargo-tarpaulin GitHub](https://github.com/xd009642/tarpaulin)
- [Rust Code Coverage Book](https://doc.rust-lang.org/rustc/instrument-coverage.html)

## Support

For issues with:
- **Codecov**: Open an issue at [codecov/feedback](https://github.com/codecov/feedback)
- **cargo-tarpaulin**: Open an issue at [xd009642/tarpaulin](https://github.com/xd009642/tarpaulin)
- **This project**: Open an issue at [avilaops/arxis](https://github.com/avilaops/arxis)
