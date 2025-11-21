# Contributing to AVX-GPU

Thank you for your interest in contributing to AVX-GPU! 🎉

## Getting Started

1. **Fork the repository**
2. **Clone your fork**: `git clone https://github.com/YOUR_USERNAME/avx-gpu`
3. **Create a branch**: `git checkout -b feature/my-feature`
4. **Make your changes**
5. **Test your changes**: `cargo test --all`
6. **Submit a pull request**

## Development Setup

### Prerequisites

- Rust 1.75+ (stable)
- GPU with Vulkan/Metal/DX12 support
- (Optional) CUDA Toolkit for CUDA backend
- (Optional) ROCm for AMD backend

### Building

```bash
# Build all crates
cargo build --all

# Build with specific backend
cargo build --features cuda

# Run tests
cargo test --all

# Run benchmarks
cargo bench
```

### Code Style

- Follow standard Rust conventions
- Run `cargo fmt` before committing
- Run `cargo clippy` and fix warnings
- Add documentation for public APIs

### Testing

- Add unit tests for new functionality
- Add integration tests for complex features
- Test on multiple platforms if possible
- Include benchmarks for performance-critical code

## Areas for Contribution

### High Priority

- **CUDA backend implementation** (see `avx-gpu-backends/cuda/`)
- **Metal backend implementation** (see `avx-gpu-backends/metal/`)
- **Kernel fusion optimizer**
- **Standard library functions** (linear algebra, signal processing)

### Medium Priority

- **ROCm backend** (AMD GPU support)
- **Multi-GPU support**
- **Async execution pipelines**
- **Memory pool optimizations**

### Good First Issues

- Documentation improvements
- Example programs
- Bug fixes
- Test coverage

## Pull Request Guidelines

1. **Keep PRs focused**: One feature/fix per PR
2. **Write good commit messages**: Explain why, not just what
3. **Update documentation**: If you change APIs
4. **Add tests**: For new functionality
5. **Check CI**: Ensure all tests pass

## Code of Conduct

Be respectful, inclusive, and professional. We're all here to build something great together.

## Questions?

- Open an issue for bugs or feature requests
- Join our Discord for discussions
- Email nicolas@avila.inc for private inquiries

Thank you for contributing! 🚀
