# Contributing to avila-clustering

Thank you for your interest in contributing to avila-clustering! This document provides guidelines and instructions for contributing.

## Code of Conduct

This project adheres to a code of conduct that all contributors are expected to follow. Please be respectful and professional in all interactions.

## Getting Started

### Prerequisites

- Rust 1.70 or later
- Git
- Basic understanding of clustering algorithms

### Setting Up Development Environment

```bash
# Clone the repository
git clone https://github.com/avilaops/arxis.git
cd arxis/avila-clustering

# Build the project
cargo build

# Run tests
cargo test

# Run benchmarks
cargo bench

# Check formatting
cargo fmt --check

# Run linter
cargo clippy --all-features
```

## How to Contribute

### Reporting Bugs

When filing an issue, please include:

1. **Clear description** of the problem
2. **Steps to reproduce** the issue
3. **Expected behavior** vs actual behavior
4. **System information** (OS, Rust version)
5. **Minimal code example** that demonstrates the issue

### Suggesting Features

Feature requests should include:

1. **Use case** - Why is this feature needed?
2. **Proposed API** - How should it work?
3. **Alternatives considered** - What other approaches were evaluated?
4. **Implementation ideas** - Any thoughts on how to implement it?

### Submitting Pull Requests

1. **Fork the repository** and create a feature branch
2. **Write tests** for new functionality
3. **Update documentation** (rustdoc, README, examples)
4. **Follow code style** (run `cargo fmt`)
5. **Ensure CI passes** (tests, clippy, benchmarks)
6. **Write clear commit messages**
7. **Submit PR** with detailed description

#### Commit Message Format

```
type(scope): Brief description

Detailed explanation of changes (optional)

Fixes #123
```

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `chore`

### Code Style

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` with default settings
- Address all `cargo clippy` warnings
- Prefer explicit over implicit
- Document all public APIs with examples
- Keep functions focused and testable

### Testing

All contributions must include appropriate tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn test_kmeans_basic() {
        let data = array![[0.0, 0.0], [1.0, 1.0], [10.0, 10.0]];
        let kmeans = KMeansBuilder::new(2)
            .fit(data.view())
            .unwrap();

        assert_eq!(kmeans.n_clusters(), 2);
    }

    #[test]
    fn test_kmeans_empty_data() {
        let data = array![[0.0; 0]; 0];
        let result = KMeansBuilder::new(2).fit(data.view());

        assert!(result.is_err());
    }
}
```

### Documentation

#### Rustdoc Standards

```rust
/// Performs K-Means clustering on the input data.
///
/// K-Means is a centroid-based clustering algorithm that partitions data
/// into k clusters by minimizing within-cluster variance.
///
/// # Arguments
///
/// * `data` - Input data matrix of shape (n_samples, n_features)
///
/// # Returns
///
/// Returns a `Result` containing the clustering result or an error.
///
/// # Examples
///
/// ```
/// use avila_clustering::algorithms::kmeans::KMeansBuilder;
/// use ndarray::array;
///
/// let data = array![[0.0, 0.0], [1.0, 1.0], [10.0, 10.0], [11.0, 11.0]];
/// let kmeans = KMeansBuilder::new(2)
///     .max_iter(100)
///     .fit(data.view())
///     .unwrap();
///
/// assert_eq!(kmeans.n_clusters(), 2);
/// ```
///
/// # Panics
///
/// This function will panic if the number of clusters is 0.
///
/// # Performance
///
/// Time complexity: O(n × k × i × d) where:
/// - n: number of samples
/// - k: number of clusters
/// - i: number of iterations
/// - d: number of features
///
/// # References
///
/// Lloyd, S. (1982). Least squares quantization in PCM.
/// IEEE Transactions on Information Theory, 28(2), 129-137.
pub fn fit(&self, data: ArrayView2<f64>) -> Result<KMeans> {
    // Implementation
}
```

### Benchmarking

Add benchmarks for new algorithms:

```rust
fn my_algorithm_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("my_algorithm");

    for size in [100, 1000, 10000].iter() {
        let data = generate_data(*size);

        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            size,
            |b, &_| {
                b.iter(|| {
                    MyAlgorithm::new()
                        .fit(black_box(data.view()))
                        .unwrap()
                });
            },
        );
    }

    group.finish();
}
```

### Adding New Algorithms

When adding a new clustering algorithm:

1. **Research**: Study the original paper and existing implementations
2. **API Design**: Follow the builder pattern used by existing algorithms
3. **Implementation**: Write clean, well-documented code
4. **Testing**: Add comprehensive unit and integration tests
5. **Benchmarks**: Compare performance with existing algorithms
6. **Examples**: Create a practical example in `examples/`
7. **Documentation**: Update README and write detailed rustdoc

Example structure:

```rust
// src/algorithms/my_algorithm.rs

//! My Algorithm implementation
//!
//! [Algorithm description, mathematical formulation, use cases]
//!
//! # References
//!
//! [Citations to original papers]

use crate::prelude::*;

pub struct MyAlgorithmBuilder {
    param1: f64,
    param2: usize,
}

impl MyAlgorithmBuilder {
    pub fn new() -> Self { /* ... */ }
    pub fn param1(mut self, value: f64) -> Self { /* ... */ }
    pub fn fit(self, data: ArrayView2<f64>) -> Result<MyAlgorithm> { /* ... */ }
}

pub struct MyAlgorithm {
    pub labels: Vec<i32>,
    // Other fields
}

impl MyAlgorithm {
    pub fn predict(&self, data: ArrayView2<f64>) -> Result<Vec<i32>> { /* ... */ }
}

#[cfg(test)]
mod tests { /* ... */ }
```

## Performance Considerations

- Profile before optimizing
- Use `#[inline]` judiciously for hot paths
- Prefer iterators over indexing
- Leverage Rayon for parallelism
- Consider SIMD for numerical operations
- Minimize allocations in tight loops

## Review Process

1. All PRs require at least one approval
2. CI must pass (tests, clippy, fmt)
3. Documentation must be updated
4. Benchmarks should show no regression (or justify it)
5. Code coverage should not decrease

## Getting Help

- **Discord**: [Join our community](https://discord.gg/avila)
- **GitHub Discussions**: Ask questions, share ideas
- **Email**: dev@avila.inc

## License

By contributing, you agree that your contributions will be licensed under the same MIT OR Apache-2.0 dual license.

## Recognition

All contributors will be acknowledged in the CONTRIBUTORS.md file and release notes.

Thank you for contributing to avila-clustering! 🚀
