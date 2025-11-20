# Contributing to Arxis

First off, thank you for considering contributing to Arxis! It's people like you that make Arxis such a great tool for scientific computing.

## Code of Conduct

This project and everyone participating in it is governed by our Code of Conduct. By participating, you are expected to uphold this code.

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check the existing issues to avoid duplicates. When you create a bug report, include as many details as possible:

- **Use a clear and descriptive title**
- **Describe the exact steps to reproduce the problem**
- **Provide specific examples** (minimal reproducible code)
- **Describe the behavior you observed** and what you expected
- **Include your environment details**: OS, Rust version, crate version

Use our bug report template when creating an issue.

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion:

- **Use a clear and descriptive title**
- **Provide a detailed description** of the suggested enhancement
- **Explain why this enhancement would be useful** to users
- **List any alternative solutions** you've considered

Use our feature request template when creating an issue.

### Pull Requests

1. **Fork the repo** and create your branch from `main`
2. **Write tests** for your changes
3. **Ensure the test suite passes** (`cargo test --workspace`)
4. **Format your code** (`cargo fmt --all`)
5. **Run clippy** (`cargo clippy --workspace`)
6. **Update documentation** if needed
7. **Write a good commit message**

#### Pull Request Process

1. Update the README.md with details of changes, if applicable
2. Update CHANGELOG.md following the existing format
3. The PR will be merged once you have approval from maintainers

## Development Setup

### Prerequisites

- Rust 1.70 or higher
- Cargo

### Building from Source

```bash
# Clone the repository
git clone https://github.com/avilaops/arxis.git
cd arxis

# Build all workspace members
cargo build --workspace

# Run tests
cargo test --workspace

# Run specific tests
cargo test -p avila-math
cargo test -p arxis_quaternions
```

### Running Examples

```bash
# LISA pipeline example
cargo run --example lisa_example

# Math kernel examples
cargo run --example rotations_example
cargo run --example geometry4d_example

# See all examples
ls examples/
```

## Coding Style

We follow the standard Rust style guidelines:

- Use `rustfmt` for formatting: `cargo fmt --all`
- Use `clippy` for linting: `cargo clippy --workspace`
- Write documentation comments (`///`) for public APIs
- Include examples in documentation when helpful

### Documentation

- All public items should have documentation comments
- Include usage examples when possible
- Use proper markdown formatting
- Reference related functions and types using backticks and links

Example:
```rust
/// Calculates the gravitational wave frequency of a binary system.
///
/// This function uses the Newtonian approximation for circular orbits.
///
/// # Arguments
///
/// * `m1` - Mass of first object in solar masses
/// * `m2` - Mass of second object in solar masses
/// * `separation` - Orbital separation in kilometers
///
/// # Returns
///
/// Frequency in Hertz
///
/// # Examples
///
/// ```
/// use arxis_quaternions::physics::*;
///
/// let freq = gravitational_wave_frequency(1e6, 5e5, 1000.0);
/// assert!(freq > 0.0);
/// ```
pub fn gravitational_wave_frequency(m1: f64, m2: f64, separation: f64) -> f64 {
    // Implementation
}
```

## Testing

- Write unit tests for new functionality
- Integration tests should go in the `tests/` directory
- Benchmarks should use the `criterion` crate
- Aim for >80% code coverage

### Running Tests

```bash
# All tests
cargo test --workspace

# Specific crate
cargo test -p avila-math

# With output
cargo test -- --nocapture

# Specific test
cargo test test_quaternion_multiplication
```

## Project Structure

```
arxis/
├── avila-math/          # Mathematical kernel
├── avila-telemetry/     # Time series analysis
├── avx-*/               # AVX platform crates
├── src/                 # Main library (arxis_quaternions)
├── examples/            # Usage examples
├── tests/               # Integration tests
└── docs/                # Additional documentation
```

## Commit Messages

Follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

- `feat: add new LISA waveform model`
- `fix: correct quaternion multiplication bug`
- `docs: update README with installation instructions`
- `test: add tests for 4D geometry`
- `refactor: simplify tensor operations`
- `perf: optimize FFT implementation`
- `chore: update dependencies`

## Versioning

We use [Semantic Versioning](https://semver.org/):

- **MAJOR**: Breaking changes
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes (backward compatible)

## License

By contributing, you agree that your contributions will be licensed under both MIT and Apache-2.0 licenses.

## Questions?

- 💬 [GitHub Discussions](https://github.com/avilaops/arxis/discussions)
- 📧 Email: nicolas@avila.inc
- 📚 [Documentation](https://docs.rs/arxis_quaternions)

## Recognition

Contributors will be recognized in:
- README.md contributors section
- Release notes
- CHANGELOG.md

Thank you for contributing to Arxis! 🚀
