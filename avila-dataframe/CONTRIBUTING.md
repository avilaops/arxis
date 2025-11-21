# Contributing to AvilaDB DataFrame

Thank you for your interest in contributing to AvilaDB DataFrame! 🚀

## Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork** locally
3. **Create a branch** for your feature/fix
4. **Make your changes**
5. **Run checks**: `.\scripts\check.ps1`
6. **Submit a pull request**

## Development Setup

### Prerequisites

- Rust 1.75+ (install via [rustup](https://rustup.rs))
- Git
- PowerShell 7+ (for scripts)

### Building

```powershell
# Build the library
cargo build

# Build with all features
cargo build --all-features

# Run in release mode
cargo build --release --all-features
```

### Testing

```powershell
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with logging
RUST_LOG=debug cargo test
```

### Running Examples

```powershell
# Run all examples
.\scripts\run-examples.ps1

# Run specific example
cargo run --example basic_usage
```

### Code Quality

Before submitting, ensure all checks pass:

```powershell
# Run all checks (formatting, linting, tests, docs)
.\scripts\check.ps1

# Auto-fix formatting and clippy issues
.\scripts\check.ps1 -Fix
```

## Code Style

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `rustfmt` for formatting (automatic with check.ps1)
- Pass `clippy` with no warnings
- Document all public APIs with examples

### Documentation Format

```rust
/// Brief one-line description
///
/// Longer explanation if needed with details about:
/// - What the function does
/// - Important behaviors
/// - Edge cases
///
/// # Arguments
///
/// * `param1` - Description of param1
/// * `param2` - Description of param2
///
/// # Returns
///
/// Description of return value
///
/// # Errors
///
/// When this function returns an error and why
///
/// # Examples
///
/// ```
/// use avila_dataframe::prelude::*;
///
/// let df = DataFrame::new(vec![...])?;
/// let result = df.some_operation()?;
/// ```
pub fn function(param1: Type1, param2: Type2) -> Result<ReturnType> {
    // Implementation
}
```

## Performance Requirements

All operations must meet or exceed Polars performance:

- **Group by** (100M rows): < 1.5s (target < Polars 2.3s)
- **Join** (10M × 10M): < 1.0s (target < Polars 1.8s)
- **FFT** (1M samples): < 0.5s with SIMD

### Profiling

```powershell
# Profile with criterion
cargo bench

# Profile with flamegraph (Linux/macOS)
cargo flamegraph --example basic_usage
```

## What to Contribute

### High Priority

- [ ] Complete filter/group_by implementation
- [ ] Join operations (inner, left, right, outer)
- [ ] Parquet I/O (read/write)
- [ ] FFT implementation with rustfft
- [ ] GPU acceleration integration
- [ ] SQL query engine

### Good First Issues

Look for issues tagged with `good-first-issue`:

- Documentation improvements
- Additional examples
- Test coverage
- Performance benchmarks

### Scientific Features

We especially welcome contributions for:

- Astronomy functions (redshift, luminosity, etc.)
- Signal processing (wavelets, filters)
- Statistical tests
- Time series analysis

## Pull Request Process

1. **Update documentation** for any changed APIs
2. **Add tests** for new functionality
3. **Update CHANGELOG.md** with your changes
4. **Ensure all checks pass**: `.\scripts\check.ps1`
5. **Write clear commit messages** following [Conventional Commits](https://www.conventionalcommits.org/)

### Commit Message Format

```
type(scope): brief description

Longer explanation if needed with:
- What changed
- Why it changed
- Any breaking changes

Closes #123
```

Types: `feat`, `fix`, `docs`, `perf`, `refactor`, `test`, `chore`

Examples:
- `feat(core): add quaternion support for 3D rotations`
- `fix(io): correct parquet compression handling`
- `perf(ops): optimize group_by with SIMD`
- `docs(readme): update performance benchmarks`

## Testing Guidelines

### Unit Tests

Each module should have comprehensive unit tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_specific_functionality() {
        let result = function_under_test();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_error_handling() {
        let result = function_that_should_fail();
        assert!(result.is_err());
    }
}
```

### Integration Tests

Located in `tests/`:

```rust
// tests/test_dataframe_operations.rs
use avila_dataframe::prelude::*;

#[test]
fn test_complex_workflow() {
    // Test realistic usage patterns
}
```

### Property-Based Tests

Use `proptest` for property testing:

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_property(value in any::<f64>()) {
        // Property that should hold for all inputs
    }
}
```

## Benchmarking

Add benchmarks to `benches/benchmark_main.rs`:

```rust
fn benchmark_new_feature(c: &mut Criterion) {
    c.bench_function("feature_name", |b| {
        b.iter(|| {
            // Code to benchmark
        });
    });
}
```

## Code of Conduct

- Be respectful and inclusive
- Provide constructive feedback
- Focus on improving the library
- Help others learn and grow

## Questions?

- **Discord**: [Join our Discord](https://discord.gg/avilacloud)
- **Issues**: Create a GitHub issue for bugs/features
- **Discussions**: Use GitHub Discussions for questions

## License

By contributing, you agree that your contributions will be licensed under the Apache License 2.0.

---

**Thank you for helping make AvilaDB DataFrame the best DataFrame library in the world!** 🇧🇷🚀
