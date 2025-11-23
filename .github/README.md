# 🏛️ Arxis GitHub Organization

Welcome to the `.github` directory! This folder contains all the templates and configurations for contributing to Arxis.

## 📋 Issue Templates

We have several issue templates to help you report problems and suggest improvements:

### 🐛 [Bug Report](ISSUE_TEMPLATE/bug_report.yml)
Use this template to report bugs, unexpected behavior, or errors. Includes:
- Detailed reproduction steps
- Environment information
- Minimal reproducible example
- Affected crate selection

### ✨ [Feature Request](ISSUE_TEMPLATE/feature_request.yml)
Suggest new features or enhancements. Includes:
- Problem statement and use case
- Proposed solution
- Alternative approaches
- Priority assessment

### 📚 [Documentation Issue](ISSUE_TEMPLATE/documentation.yml)
Report documentation issues or suggest improvements. Includes:
- Documentation type (API, README, examples, guides)
- Specific location and issue description
- Suggested improvements

### ⚡ [Performance Issue](ISSUE_TEMPLATE/performance.yml)
Report performance regressions or optimization opportunities. Includes:
- Benchmark results and profiling data
- Reproduction code
- Comparison with previous versions
- Optimization suggestions

### 🔒 [Security Vulnerability](ISSUE_TEMPLATE/security.yml)
Report security vulnerabilities (use for non-critical issues only). Includes:
- Severity assessment
- Vulnerability category
- Proof of concept
- Responsible disclosure agreement

> ⚠️ **For critical security issues**: Email directly to nicolas@avila.inc

## 🔄 Pull Request Template

Our [PR template](pull_request_template.md) is comprehensive and helps ensure quality contributions:

### Sections
- **Description**: Clear explanation of changes
- **Type of Change**: Bug fix, feature, breaking change, etc.
- **Affected Crates**: Which parts of the monorepo are affected
- **Changes Made**: Detailed list of additions, changes, removals, fixes
- **Testing**: Test checklist and commands
- **Performance Impact**: Benchmarks for performance-sensitive changes
- **Breaking Changes**: Migration guide if API changes
- **Checklist**: Code quality, documentation, dependencies, CI/CD

### Quick PR Commands
```bash
# Run all tests
cargo test --workspace

# Linting
cargo clippy --workspace -- -D warnings
cargo fmt --all -- --check

# Documentation
cargo doc --no-deps --open

# Benchmarks
cargo bench
```

## 📞 Contact Links

We've configured several ways to get help:

- **💬 GitHub Discussions**: Questions and general discussions
- **📚 Documentation**: Complete API docs at docs.rs
- **🏛️ Arxis Website**: Project homepage at arxis.avilaops.com
- **☁️ AVL Cloud**: Platform info at avilaops.com
- **📧 Email**: nicolas@avila.inc

## 🤝 Community Guidelines

- **[Code of Conduct](../CODE_OF_CONDUCT.md)**: We follow the Contributor Covenant
- **[Contributing Guide](CONTRIBUTING.md)**: How to contribute to Arxis
- **[Security Policy](SECURITY.md)**: How to report security issues

## 🔧 Workflow Automation

### GitHub Actions Workflows

Located in `workflows/`:

- **ci.yml**: Continuous integration (tests, linting, formatting)
- **security-audit.yml**: Security vulnerability scanning
- **deploy.yml**: Automated deployments
- **deploy-docs.yml**: Documentation deployment
- **codeql.yml**: Code quality analysis
- **release.yml**: Release automation

### Dependabot

Configured in `dependabot.yml` to:
- Keep dependencies up to date
- Security vulnerability patches
- Weekly update checks

## 🏗️ Project Structure

Arxis is a **Rust workspace monorepo** with multiple crates:

### Core Libraries
- `arxis_quaternions` - Main physics library (LISA, gravitational waves)
- `avila-math` - Mathematical kernel (quaternions, tensors, 4D geometry)
- `avila-telemetry` - Time series, anomaly detection, forecasting
- `avila-compress` - Native compression (LZ4, future: Zstd, Snappy)
- `avila-tokenizers` - NLP tokenization (BPE, WordPiece, Unigram)

### AVL Platform (Cloud Infrastructure)
- `aviladb` - Distributed database
- `avx-*` - Various platform components (gateway, auth, storage, etc.)

### Documentation
- `docs/` - Additional documentation
- `docs-site/` - Documentation website
- `examples/` - Usage examples

## 📊 Project Stats

- **101 tests passing** (main library)
- **26 tests** (avila-math)
- **22 tests** (avila-telemetry)
- **15+ examples** demonstrating usage
- **MIT/Apache-2.0** dual-licensed

## 🎯 How to Contribute

1. **Choose an issue** or create one using the appropriate template
2. **Fork the repository** and create a feature branch
3. **Make your changes** following the Rust API guidelines
4. **Write tests** and ensure all tests pass
5. **Submit a PR** using the pull request template
6. **Respond to feedback** from maintainers

## 🔍 Issue Labels

We use labels to organize issues:

- `bug` - Something isn't working
- `enhancement` - New feature or request
- `documentation` - Improvements to documentation
- `performance` - Performance related
- `security` - Security vulnerability
- `good first issue` - Good for newcomers
- `help wanted` - Extra attention needed
- `triage` - Needs initial review

## 🚀 Release Process

1. Version bump in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Create GitHub release with tag
4. Automated publish to crates.io
5. Documentation deployment

## 📖 Additional Resources

- **Website**: https://arxis.avilaops.com
- **Documentation**: https://docs.rs/arxis_quaternions
- **Organization**: https://avilaops.com
- **Manifesto**: https://github.com/avilaops/arxis/blob/main/MANIFESTO.md

## 💡 Tips for Contributors

### First-time Contributors
- Look for `good first issue` label
- Read the [Contributing Guide](CONTRIBUTING.md)
- Ask questions in Discussions
- Start with documentation improvements

### Code Style
- Follow Rust API guidelines
- Use `rustfmt` for formatting
- Pass `clippy` lints
- Write descriptive commit messages

### Testing
- Add tests for new features
- Ensure edge cases are covered
- Run benchmarks for performance changes
- Test examples to ensure they work

### Documentation
- Add rustdoc comments for public APIs
- Update README if public API changes
- Add examples for new features
- Keep CHANGELOG updated

---

**Thank you for contributing to Arxis - The Mathematical Citadel! 🏛️**

*Built with ❤️ by the Avila community*
