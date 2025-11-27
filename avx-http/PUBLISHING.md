# AVX-HTTP Publishing Guide

## Pre-Publishing Checklist

### 1. Documentation
- [ ] All public APIs documented
- [ ] README.md updated
- [ ] CHANGELOG.md updated
- [ ] Examples working

### 2. Code Quality
```bash
# Run all checks
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo doc --no-deps --all-features
```

### 3. Version Bump
Update version in:
- `Cargo.toml`
- `README.md` badges
- Documentation links

### 4. Build & Test
```bash
# Test all feature combinations
cargo test
cargo test --no-default-features
cargo test --features tls
cargo test --all-features

# Test examples
cargo run --example async_http_server
cargo run --example async_runtime
```

## Publishing Steps

### 1. Login to crates.io
```bash
cargo login
```

### 2. Dry Run
```bash
cargo publish --dry-run --allow-dirty
```

### 3. Package Verification
```bash
cargo package --list
cargo package
```

### 4. Publish!
```bash
cargo publish
```

## Post-Publishing

### 1. Tag Release
```bash
git tag -a v0.4.0 -m "Release v0.4.0 - Async Runtime Complete"
git push origin v0.4.0
```

### 2. GitHub Release
Create release on GitHub with:
- Tag: v0.4.0
- Title: "AVX-HTTP v0.4.0 - Async Runtime & TLS Support"
- Description: See CHANGELOG.md

### 3. Announce
- Reddit r/rust
- Rust Users Forum
- Twitter/X
- Blog post

## Feature Flags

Document clearly:

```toml
[dependencies]
avx-http = "0.4"                    # Core only, zero deps
avx-http = { version = "0.4", features = ["tls"] }  # With HTTPS
```

## Badges

Add to README:
```markdown
[![Crates.io](https://img.shields.io/crates/v/avx-http.svg)](https://crates.io/crates/avx-http)
[![Documentation](https://docs.rs/avx-http/badge.svg)](https://docs.rs/avx-http)
[![Downloads](https://img.shields.io/crates/d/avx-http.svg)](https://crates.io/crates/avx-http)
[![License](https://img.shields.io/crates/l/avx-http.svg)](LICENSE)
```

## Marketing Points

**Zero Dependencies Core:**
- No tokio, no hyper, no serde
- ~500KB binary
- 3s compile time
- 100% auditable

**Complete Features:**
- HTTP/1.1 + HTTP/2
- Custom async runtime
- HPACK compression
- Optional TLS 1.3
- epoll/kqueue/IOCP

**Performance:**
- <120μs p50 latency
- O(1) timer operations
- Zero-copy where possible

## Version History

- **v0.1.0** - HTTP/1.1 basic
- **v0.2.0** - HTTP/2 frames
- **v0.3.0** - HPACK compression
- **v0.4.0** - Async runtime + TLS ✨
- **v0.5.0** - Server Push (planned)
- **v1.0.0** - Production ready (planned)

## Support

- Docs: https://docs.rs/avx-http
- Issues: https://github.com/avilaops/arxis/issues
- Discussions: https://github.com/avilaops/arxis/discussions
