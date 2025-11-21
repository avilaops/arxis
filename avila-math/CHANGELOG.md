# Changelog

All notable changes to avila-math will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- **Linear Algebra Module**: SVD, eigenvalues, eigenvectors, QR/LU decompositions
- **Calculus Module**: 4D differential operators (gradient, divergence, curl, Laplacian)
- **Interpolation Module**: Bézier curves, cubic splines, B-splines (all in 4D)
- **Serde Support**: Optional serialization for tensors
- **Benchmarks**: Comprehensive Criterion benchmarks for all major modules
- **CI/CD**: GitHub Actions workflow with tests, formatting, clippy, and coverage
- **Documentation**: Enhanced README with usage examples and API docs
- **Release Optimization**: Aggressive LTO and optimization settings

### Changed
- Improved Conv4D backward pass implementation
- Enhanced SIMD operations with better AVX2 support
- Updated dependencies to latest versions

### Fixed
- Laplacian test tolerance adjustment
- Memory layout optimizations in Conv4D

## [0.1.0] - 2025-11-21

### Added
- Initial release
- Quaternion 3D rotations
- Dual quaternions for rigid body transforms
- SO(4) rotations for 4D space
- 4D geometry primitives (tesseract, 24-cell, simplex)
- N-dimensional tensor system
- Conv4D layers with forward/backward pass
- FFT 1D/2D/3D/4D implementations
- Wavelet transforms (CWT, DWT)
- Spectral analysis tools
- SIMD optimizations (AVX2)
- AABB collision detection
- 26 initial tests

[Unreleased]: https://github.com/avilaops/arxis/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/avilaops/arxis/releases/tag/v0.1.0
