# Release Guide for avila-tokenizers

This guide explains how to publish a new release of avila-tokenizers to crates.io and GitHub.

## Prerequisites

Before releasing, ensure you have:

1. **Crates.io Account**: Create one at [crates.io](https://crates.io)
2. **API Token**: Generate at [crates.io/settings/tokens](https://crates.io/settings/tokens)
3. **GitHub Push Access**: Permission to push tags to the repository
4. **All Tests Passing**: Verify CI is green on main branch

## Release Process

### 1. Update Version Number

Edit `avila-tokenizer/Cargo.toml`:

```toml
[package]
name = "avila-tokenizers"
version = "0.2.0"  # Update this
edition = "2021"
# ...
```

Follow [Semantic Versioning](https://semver.org/):
- **MAJOR** (1.0.0): Breaking changes
- **MINOR** (0.2.0): New features, backward compatible
- **PATCH** (0.1.1): Bug fixes, backward compatible

### 2. Update CHANGELOG.md

Move changes from `[Unreleased]` to a new version section:

```markdown
## [0.2.0] - 2025-11-22

### Added
- New feature X
- Support for Y

### Changed
- Improved performance of Z

### Fixed
- Bug in W

## [0.1.0] - 2025-11-22
...

[Unreleased]: https://github.com/avilaops/arxis/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/avilaops/arxis/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/avilaops/arxis/releases/tag/v0.1.0
```

### 3. Update Documentation

Review and update:
- README.md (if needed)
- API documentation in code
- Examples (ensure they work)

### 4. Run Final Checks

```bash
cd avila-tokenizer

# Clean build
cargo clean

# Run all tests
cargo test --all-features

# Run clippy
cargo clippy --all-targets --all-features -- -D warnings

# Check formatting
cargo fmt --all -- --check

# Build documentation
cargo doc --no-deps --all-features

# Verify package contents
cargo package --list

# Dry run publish
cargo publish --dry-run
```

### 5. Commit Version Changes

```bash
git add avila-tokenizer/Cargo.toml avila-tokenizer/CHANGELOG.md
git commit -m "chore: bump version to 0.2.0"
git push origin main
```

### 6. Create Git Tag

```bash
# Create annotated tag
git tag -a v0.2.0 -m "Release v0.2.0"

# Push tag to GitHub
git push origin v0.2.0
```

**Important**: The tag must start with `v` (e.g., `v0.2.0`) to trigger the release workflow.

### 7. Automated Release Workflow

Once you push the tag, GitHub Actions will automatically:

1. ✅ Create a GitHub Release
2. ✅ Publish to crates.io
3. ✅ Build binaries for multiple platforms
4. ✅ Upload binaries to GitHub Release
5. ✅ Update documentation

Monitor the workflow at: `https://github.com/avilaops/arxis/actions`

### 8. Manual crates.io Publishing (If Needed)

If the automated workflow fails:

```bash
cd avila-tokenizer

# Set your crates.io token
cargo login

# Publish
cargo publish
```

### 9. Verify Release

1. **Check crates.io**: https://crates.io/crates/avila-tokenizers
2. **Check GitHub Release**: https://github.com/avilaops/arxis/releases
3. **Check docs.rs**: https://docs.rs/avila-tokenizers
4. **Test installation**:
   ```bash
   cargo new test-avila-tokenizer
   cd test-avila-tokenizer
   cargo add avila-tokenizers@0.2.0
   ```

### 10. Announce Release

- [ ] Update project website (if exists)
- [ ] Post on social media
- [ ] Notify users in Discord/Slack (if applicable)
- [ ] Update dependent projects

## GitHub Secrets Setup

For automated releases to work, configure these secrets in GitHub:

### CARGO_TOKEN (Required)

1. Go to [crates.io/settings/tokens](https://crates.io/settings/tokens)
2. Click "New Token"
3. Name: "GitHub Actions - avila-tokenizers"
4. Click "Generate"
5. Copy the token
6. In GitHub: Settings > Secrets and variables > Actions
7. Click "New repository secret"
8. Name: `CARGO_TOKEN`
9. Value: Paste the token
10. Click "Add secret"

### GITHUB_TOKEN (Automatic)

This is automatically provided by GitHub Actions, no setup needed.

## Troubleshooting

### Release Workflow Fails

**Problem**: GitHub Actions release workflow fails

**Solutions**:
1. Check workflow logs in GitHub Actions
2. Verify `CARGO_TOKEN` secret is set correctly
3. Ensure all tests pass before tagging
4. Check `working-directory` in workflow file

### crates.io Publishing Fails

**Problem**: `cargo publish` fails with authentication error

**Solution**:
```bash
# Re-login to crates.io
cargo login

# Retry publish
cargo publish
```

**Problem**: "crate version X already exists"

**Solution**: You cannot overwrite published versions. Bump to the next version.

### Tag Already Exists

**Problem**: Tag already exists locally or remotely

**Solution**:
```bash
# Delete local tag
git tag -d v0.2.0

# Delete remote tag
git push origin :refs/tags/v0.2.0

# Create new tag
git tag -a v0.2.0 -m "Release v0.2.0"
git push origin v0.2.0
```

### Documentation Not Updated

**Problem**: docs.rs doesn't update automatically

**Solution**:
1. Go to https://docs.rs/avila-tokenizers
2. Click "Rebuild" button
3. Wait for build to complete

## Release Checklist

Use this checklist for each release:

- [ ] All tests passing in CI
- [ ] Version bumped in `Cargo.toml`
- [ ] CHANGELOG.md updated
- [ ] Documentation reviewed
- [ ] Examples tested
- [ ] `cargo publish --dry-run` succeeds
- [ ] Changes committed to main
- [ ] Git tag created and pushed
- [ ] GitHub release created
- [ ] crates.io updated
- [ ] docs.rs updated
- [ ] Release announced

## Version Planning

### Current: v0.1.0
Initial release with core features.

### Future: v0.2.0
- Additional model support (Mistral, Claude)
- Performance improvements
- More examples

### Future: v1.0.0
- Stable API
- 100% test coverage of core
- Production-ready
- Performance benchmarks published

## Release Schedule

- **Patch releases**: As needed for bug fixes
- **Minor releases**: Every 1-2 months
- **Major releases**: When breaking changes are necessary

## Support Policy

- **Latest version**: Full support
- **Previous minor**: Bug fixes only
- **Older versions**: No support (upgrade recommended)

## Resources

- [crates.io Publishing Guide](https://doc.rust-lang.org/cargo/reference/publishing.html)
- [Semantic Versioning](https://semver.org/)
- [Keep a Changelog](https://keepachangelog.com/)
- [GitHub Releases Documentation](https://docs.github.com/en/repositories/releasing-projects-on-github)

## Contact

For release-related questions:
- Open an issue: https://github.com/avilaops/arxis/issues
- Email: nicolas@avila.inc
