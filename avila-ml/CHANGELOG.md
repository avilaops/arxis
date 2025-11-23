# Changelog

All notable changes to Avila ML will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2025-11-22

### 🎉 Initial Production Release

First production-ready release of Avila ML - a pure Rust machine learning library for scientific computing.

### Added

#### Core Features
- **Autograd System**: Complete automatic differentiation engine with computational graph
  - `Arc<Mutex<Option<ArrayD<T>>>>` gradient storage for safe sharing across tensor clones
  - Backward propagation through all operations (add, mul, matmul, mean, sum, etc.)
  - Support for complex computational graphs with multiple paths

#### Neural Network Layers
- `Linear`: Fully-connected layers with Xavier initialization
- `Conv2d`: 2D convolutions with padding and stride support (Rayon parallelized)
- `Conv4d`: **4D convolutions for spacetime data** (unique feature for scientific computing)
- `Sequential`: Container for building multi-layer models

#### Activation Functions
- ReLU, Sigmoid, Tanh, Softmax
- LogSoftmax, LeakyReLU, GELU
- All with proper backward implementations

#### Attention Mechanisms
- Self-attention with query/key/value projections
- Multi-head attention for Transformers
- Scaled dot-product attention

#### Optimizers
- SGD with momentum and weight decay
- Adam with bias correction
- AdamW (Adam with decoupled weight decay)
- RMSprop

#### Loss Functions
- MSE (Mean Squared Error)
- CrossEntropyLoss
- BCELoss (Binary Cross Entropy)
- HuberLoss
- SmoothL1Loss

#### Data Loading
- `Dataset` trait for custom datasets
- `DataLoader` with batching and shuffling
- `TensorDataset` for tensor pairs

#### Utilities
- Xavier/He initialization
- Learning rate schedulers (StepLR, ExponentialLR, CosineAnnealing)
- Early stopping
- One-hot encoding, argmax

### Testing & Validation
- **30 unit tests** covering all major components
- **7 gradient checking tests** using finite differences (validates mathematical correctness)
  - test_gradient_add
  - test_gradient_mul
  - test_gradient_matmul
  - test_gradient_linear_layer
  - test_gradient_sum
  - test_gradient_mean
  - test_gradient_chain_rule
- All tests passing ✅

### Examples
- `linear_regression.rs`: Simple regression with SGD
- `mnist_training.rs`: MNIST classification with Adam optimizer
- `conv4d_astrophysics.rs`: 4D convolution demo
- `ligo_gravitational_waves.rs`: Gravitational wave detection simulation

### Performance
- Rayon parallelization for Conv2d and Conv4d
- Optimized release builds with LTO and single codegen unit
- Multi-core CPU utilization

### Documentation
- Comprehensive README with examples
- Inline documentation for all public APIs
- Examples with scientific use cases

### Technical Details
- **Language**: Rust 2021 edition
- **Dependencies**: ndarray, rayon, num-traits, rand, serde
- **No external ML libraries**: 100% native implementation
- **License**: MIT OR Apache-2.0

## [Unreleased]

### Planned for v1.1
- FFT-based convolutions for large kernels
- im2col optimization for Conv2d
- BLAS integration for matrix operations
- Comprehensive benchmark suite

### Planned for v2.0
- GPU acceleration (CUDA/ROCm)
- Model serialization (save/load)
- Distributed training
- Mixed precision training (f16/bf16)

---

**Note**: This is the first production release. Prior versions (0.x) were development previews.
