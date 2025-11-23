# Rust Edition and Version Information

## Current Configuration

- **Rust Edition**: 2021
- **Minimum Supported Rust Version (MSRV)**: 1.70.0

## About Rust Edition 2021

The project uses Rust Edition 2021, which is the current stable edition as of 2025. While Rust Edition 2024 has been announced, we maintain compatibility with Edition 2021 for the following reasons:

### Why Edition 2021?

1. **Stability**: Edition 2021 is mature and well-tested across all platforms
2. **Compatibility**: Ensures broad compatibility with existing toolchains
3. **Features**: Provides all features needed for this project, including:
   - Disjoint closure captures
   - Improved panic macros
   - IntoIterator for arrays
   - Better inference for cargo features

### When to Upgrade to Edition 2024?

We will consider upgrading to Rust Edition 2024 when:

1. Edition 2024 reaches full stability (currently in development)
2. All major platforms have stable support
3. Key dependencies have migrated
4. MSRV considerations allow (Edition 2024 will likely require Rust 1.76+)

## Minimum Supported Rust Version (MSRV)

### Current: 1.70.0

The MSRV of 1.70.0 (June 2023) provides:

- Stable `std::sync::OnceLock`
- Improved type inference
- Better const generics support
- All features required by our dependencies

### MSRV Policy

- We test MSRV in CI to ensure compatibility
- MSRV will only be bumped for compelling features or dependency requirements
- MSRV bumps are considered breaking changes and will bump the minor version

### Checking Your Rust Version

```bash
# Check installed Rust version
rustc --version

# Install a specific version
rustup install 1.70.0

# Use a specific version for this project
rustup override set 1.70.0
```

## Dependencies and Rust Version

All dependencies are compatible with Rust 1.70.0:

- `regex`: Works with 1.60.0+
- `unicode-normalization`: Works with 1.56.0+
- `serde`: Works with 1.60.0+
- `rayon`: Works with 1.63.0+
- `lru`: Works with 1.60.0+

## CI Configuration

Our CI tests on:

- **Stable**: Latest stable Rust
- **Beta**: Latest beta Rust (to catch regressions early)
- **MSRV**: 1.70.0 (to ensure MSRV compatibility)

## Future Considerations

### Rust 2024 Edition Features (Preview)

When Edition 2024 becomes stable, we may benefit from:

- Improved pattern matching
- Better async/await ergonomics
- Enhanced const evaluation
- Refined trait system improvements

### Migration Path

When we decide to upgrade:

1. Update `Cargo.toml`: `edition = "2024"`
2. Run `cargo fix --edition` to apply automatic fixes
3. Address any manual migration issues
4. Update MSRV accordingly
5. Update documentation
6. Announce in CHANGELOG as a breaking change

## Compatibility Matrix

| Component | Minimum Version | Recommended   |
| --------- | --------------- | ------------- |
| rustc     | 1.70.0          | latest stable |
| cargo     | 1.70.0          | latest stable |
| Edition   | 2021            | 2021          |

## References

- [Rust Edition Guide](https://doc.rust-lang.org/edition-guide/)
- [Edition 2021 Features](https://doc.rust-lang.org/edition-guide/rust-2021/index.html)
- [Edition 2024 RFC](https://github.com/rust-lang/rfcs/pull/3501)
