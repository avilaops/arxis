# Contributing to Avila ML 🚀

Thank you for your interest in contributing to Avila ML! This document provides guidelines and information for contributors.

## 🎯 Areas Where We Need Help

### High Priority
1. **Convolution Implementations**
   - Implement proper Conv2d using im2col or FFT
   - Optimize Conv4d for large scientific datasets
   - Add support for different padding modes and strides

2. **GPU Acceleration**
   - CUDA backend integration
   - ROCm support for AMD GPUs
   - Metal support for Apple Silicon

3. **Autograd Improvements**
   - Complete backward implementations for all operations
   - Memory optimization for gradient computation
   - Support for higher-order derivatives

4. **Documentation**
   - More examples and tutorials
   - API documentation improvements
   - Scientific computing guides

### Medium Priority
5. **Model Serialization**
   - Save/load model weights
   - ONNX export support
   - Integration with model registries

6. **Advanced Layers**
   - More normalization techniques
   - Advanced pooling operations
   - Recurrent layers (LSTM, GRU)

7. **Optimizations**
   - Better memory management
   - Parallel batch processing
   - Quantization support

### Nice to Have
8. **Ecosystem Integration**
   - Integration with AvilaDB for ML pipelines
   - Cloud deployment helpers
   - Distributed training support

## 🛠️ Development Setup

### Prerequisites
- Rust 1.70 or later
- Cargo

### Setup Steps
```bash
# Clone the repository
git clone https://github.com/avilaops/arxis
cd arxis/avila-ml

# Build the project
cargo build

# Run tests
cargo test

# Run examples
cargo run --example linear_regression

# Run benchmarks
cargo bench
```

## 📝 Code Style

We follow standard Rust conventions:
- Use `rustfmt` for formatting: `cargo fmt`
- Use `clippy` for linting: `cargo clippy`
- Write documentation for all public APIs
- Include tests for new features

### Documentation
- Use `///` for public items
- Include examples in doc comments
- Reference related functions/types with backticks

Example:
```rust
/// Performs matrix multiplication between two tensors.
///
/// # Arguments
///
/// * `other` - The tensor to multiply with
///
/// # Returns
///
/// A new tensor containing the result of the matrix multiplication
///
/// # Example
///
/// ```
/// use avila_ml::tensor::Tensor;
/// let a = Tensor::new(arr2(&[[1.0, 2.0], [3.0, 4.0]]).into_dyn());
/// let b = Tensor::new(arr2(&[[2.0, 0.0], [1.0, 2.0]]).into_dyn());
/// let c = a.matmul(&b);
/// ```
pub fn matmul(&self, other: &Self) -> Self {
    // implementation
}
```

## 🧪 Testing

### Unit Tests
- Add tests in the same file as the implementation
- Use the `#[test]` attribute
- Test edge cases and error conditions

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tensor_addition() {
        let a = Tensor::new(arr1(&[1.0, 2.0, 3.0]).into_dyn());
        let b = Tensor::new(arr1(&[4.0, 5.0, 6.0]).into_dyn());
        let c = a.add(&b);
        assert_eq!(c.data[[0]], 5.0);
    }
}
```

### Integration Tests
- Place in `tests/` directory
- Test module interactions
- Test realistic use cases

### Benchmarks
- Place in `benches/` directory
- Use criterion for benchmarking
- Compare performance with other libraries when relevant

## 🐛 Bug Reports

When filing a bug report, please include:
1. Avila ML version
2. Rust version (`rustc --version`)
3. Operating system
4. Minimal reproducible example
5. Expected vs actual behavior
6. Stack trace if applicable

## 💡 Feature Requests

When proposing a feature:
1. Describe the use case
2. Explain why it's beneficial
3. Provide example API if possible
4. Consider implementation complexity

## 📬 Pull Request Process

1. **Fork and Clone**
   ```bash
   git clone https://github.com/YOUR_USERNAME/arxis
   cd arxis/avila-ml
   ```

2. **Create a Branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```

3. **Make Changes**
   - Write code
   - Add tests
   - Update documentation
   - Run `cargo fmt` and `cargo clippy`

4. **Commit**
   ```bash
   git add .
   git commit -m "feat: add Conv2d im2col implementation"
   ```

   Use conventional commits:
   - `feat:` for new features
   - `fix:` for bug fixes
   - `docs:` for documentation
   - `perf:` for performance improvements
   - `test:` for adding tests
   - `refactor:` for code refactoring

5. **Push and Create PR**
   ```bash
   git push origin feature/your-feature-name
   ```
   Then create a PR on GitHub

6. **Review Process**
   - Maintainers will review your PR
   - Address feedback if needed
   - Once approved, it will be merged

## 🎓 Learning Resources

### Rust ML Resources
- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Rust ML Book](https://rust-ml.github.io/book/)
- [ndarray documentation](https://docs.rs/ndarray/)

### ML Theory
- [Deep Learning Book](https://www.deeplearningbook.org/)
- [Automatic Differentiation](https://en.wikipedia.org/wiki/Automatic_differentiation)
- [Convolution Arithmetic](https://github.com/vdumoulin/conv_arithmetic)

### Scientific Computing
- [LIGO Documentation](https://www.ligo.org/)
- [NumPy Convolution Guide](https://numpy.org/doc/stable/reference/generated/numpy.convolve.html)

## 📞 Contact

- **Issues**: [GitHub Issues](https://github.com/avilaops/arxis/issues)
- **Discussions**: [GitHub Discussions](https://github.com/avilaops/arxis/discussions)
- **Email**: nicolas@avila.inc

## 📄 License

By contributing, you agree that your contributions will be licensed under both MIT and Apache-2.0 licenses.

---

**Thank you for contributing to Avila ML!** 🇧🇷🚀
