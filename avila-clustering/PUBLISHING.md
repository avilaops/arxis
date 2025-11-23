# Publishing Guide for avila-clustering

This guide walks through the process of publishing `avila-clustering` to crates.io.

## Pre-Publication Checklist

### 1. Code Quality
- [ ] All tests passing: `cargo test --all-features`
- [ ] No clippy warnings: `cargo clippy --all-features -- -D warnings`
- [ ] Code formatted: `cargo fmt --check`
- [ ] Documentation builds: `cargo doc --no-deps`
- [ ] All examples run: `cargo run --example customer_segmentation`
- [ ] Benchmarks compile: `cargo bench --no-run`

### 2. Documentation
- [ ] README.md is up to date
- [ ] CHANGELOG.md has current version
- [ ] All public APIs documented
- [ ] Examples in docs are tested
- [ ] CONTRIBUTING.md is current
- [ ] License files present (MIT/Apache-2.0)

### 3. Metadata
- [ ] Version number updated in `Cargo.toml`
- [ ] Keywords and categories appropriate
- [ ] Repository URL correct
- [ ] Documentation URL set
- [ ] License specified
- [ ] Authors listed

### 4. Testing
- [ ] Tested on Windows, Linux, macOS
- [ ] Tested with Rust stable, beta, nightly
- [ ] No platform-specific bugs
- [ ] Dependencies are minimal and justified
- [ ] Feature flags work correctly

### 5. Performance
- [ ] Benchmarks show acceptable performance
- [ ] No memory leaks (run with valgrind/miri)
- [ ] Scales appropriately with data size
- [ ] Parallelism works correctly

## Publication Steps

### Step 1: Final Testing

```powershell
# Clean build
cargo clean

# Test everything
cargo test --all-features --release

# Run clippy
cargo clippy --all-features -- -D warnings

# Format check
cargo fmt -- --check

# Build docs
cargo doc --no-deps --all-features

# Run examples
cargo run --example basic_clustering
cargo run --example customer_segmentation
cargo run --example image_segmentation
cargo run --example anomaly_detection

# Run benchmarks (smoke test)
cargo bench --no-run
```

### Step 2: Version Update

Update version in `Cargo.toml`:

```toml
[package]
name = "avila-clustering"
version = "0.1.0"  # Update this
```

Update `CHANGELOG.md`:

```markdown
## [0.1.0] - 2024-XX-XX

### Added
- Initial release
- [List major features]
```

### Step 3: Git Tag

```bash
# Commit final changes
git add .
git commit -m "Prepare for v0.1.0 release"

# Create annotated tag
git tag -a v0.1.0 -m "Release v0.1.0"

# Push to remote
git push origin main
git push origin v0.1.0
```

### Step 4: Dry Run

```powershell
# Check what will be published
cargo publish --dry-run

# Verify the package contents
cargo package --list

# Check package size
cargo package
# Look for the .crate file size in target/package/
```

### Step 5: Publish to crates.io

```powershell
# Login to crates.io (one-time)
cargo login

# Publish (no going back!)
cargo publish

# Verify publication
Start-Process "https://crates.io/crates/avila-clustering"
```

### Step 6: Create GitHub Release

1. Go to GitHub repository
2. Click "Releases" → "Draft a new release"
3. Select tag: v0.1.0
4. Title: "avila-clustering v0.1.0"
5. Description: Copy from CHANGELOG.md
6. Attach files if needed
7. Publish release

### Step 7: Announce

- [ ] Tweet about release
- [ ] Post to Reddit (r/rust, r/MachineLearning)
- [ ] Share on Discord/Slack communities
- [ ] Update docs.rs link in README
- [ ] Update homepage/blog

## Post-Publication

### Monitoring

- Watch GitHub issues for bug reports
- Monitor crates.io download stats
- Check docs.rs build status
- Respond to community feedback

### Maintenance

- Address critical bugs with patch releases
- Update dependencies regularly
- Keep documentation current
- Continue benchmark improvements

## Versioning Strategy

Follow [Semantic Versioning](https://semver.org/):

- **MAJOR** (1.0.0): Incompatible API changes
- **MINOR** (0.1.0): New features, backward compatible
- **PATCH** (0.1.1): Bug fixes, backward compatible

### Version Guidelines

- Start with 0.1.0 for initial release
- Increment MINOR for new algorithms/features
- Increment PATCH for bug fixes only
- Reserve MAJOR for breaking changes (post 1.0.0)

## Yanking a Release

If you need to yank a broken release:

```powershell
# Yank specific version
cargo yank --vers 0.1.0

# Undo yank if needed
cargo yank --vers 0.1.0 --undo
```

**Note**: Yanking doesn't delete the package, it just prevents new projects from using it.

## Troubleshooting

### Publication Fails

**Error: "failed to verify that package tarball matches source directory"**
- Cause: Generated files not matching
- Fix: Ensure all generated files are reproducible or excluded

**Error: "the uploaded file is too large"**
- Cause: Package > 10MB
- Fix: Add large files to `exclude` in Cargo.toml

**Error: "crate name is already taken"**
- Cause: Name conflict on crates.io
- Fix: Choose different name or contact crates.io team

### Documentation Issues

**Docs.rs build fails**
- Check https://docs.rs/crate/avila-clustering/latest/builds
- Ensure all dependencies are available
- Test locally: `cargo doc --no-deps`

### CI/CD Issues

**GitHub Actions failing**
- Check workflow logs
- Test locally with act: `act -j test`
- Ensure all required secrets are set

## Resources

- [Cargo Book - Publishing](https://doc.rust-lang.org/cargo/reference/publishing.html)
- [crates.io Policies](https://crates.io/policies)
- [docs.rs Documentation](https://docs.rs/about)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)

## Support

If you have questions about publishing:

- Email: dev@avila.inc
- GitHub Discussions: https://github.com/avilaops/arxis/discussions
- Discord: https://discord.gg/avila

---

**Last Updated**: 2024
**Next Review**: Before v0.2.0 release
