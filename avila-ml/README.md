# Avila ML 🚀

[![Crates.io](https://img.shields.io/crates/v/avila-ml)](https://crates.io/crates/avila-ml)
[![Documentation](https://docs.rs/avila-ml/badge.svg)](https://docs.rs/avila-ml)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)](LICENSE-MIT)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)](https://github.com/avilaops/arxis)

**High-performance Machine Learning library for scientific computing, built in pure Rust. 100% native, zero Python dependencies.**

Avila ML is a production-ready machine learning framework designed for **scientific applications**, with native support for **4D convolutions** (spacetime data), **automatic differentiation**, and **parallel computing**. Built by [Avila Cloud](https://avila.cloud) 🇧🇷

## ✨ Features

- 🔥 **Autograd**: Complete automatic differentiation with computational graph (validated with gradient checking)
- 🧠 **Neural Networks**: Linear, Conv2d, **Conv4d** (unique for spacetime scientific data)
- 🎯 **Optimizers**: SGD, Adam, AdamW, RMSprop with learning rate schedulers
- 📊 **Loss Functions**: MSE, Cross Entropy, BCE, Huber, Smooth L1
- 🔄 **Data Loading**: Dataset, DataLoader with batching and shuffling
- 🌟 **Attention**: Self-attention, Multi-head attention for Transformers
- 🧪 **Scientific Computing**: Conv4d for astrophysical/climate/medical data
- ⚡ **Performance**: Rayon parallelization, optimized for multi-core CPUs
- 🦀 **Pure Rust**: No Python dependencies, type-safe, memory-safe
- ✅ **Production Ready**: 30+ unit tests, 7 gradient tests, all passing

## 🚀 Quick Start

```toml
[dependencies]
avila-ml = "1.0"
ndarray = "0.16"
```

### Simple Neural Network

```rust
use avila_ml::prelude::*;
use avila_ml::tensor::{Tensor, TensorLike};
use ndarray::ArrayD;

// Create tensors with autograd
let x = Tensor::new(ArrayD::from_elem(ndarray::IxDyn(&[1, 784]), 0.5_f32)).requires_grad_();

// Build a neural network
let linear1 = Linear::new(784, 128);
let relu = ReLU::new();
let linear2 = Linear::new(128, 10);

// Forward pass
let h = linear1.forward(&x);
let h = relu.forward(&h);
let output = linear2.forward(&h);

// Backward pass (automatic differentiation)
let mut loss = output.mean();
loss.backward();

// Optimizer step
let mut optimizer = Adam::new(vec![&linear1.weight, &linear2.weight], 0.001);
optimizer.step();
```

### 4D Convolution for Gravitational Wave Detection

```rust
use avila_ml::nn::Conv4d;
use avila_ml::tensor::Tensor;
use ndarray::ArrayD;

// LIGO/LISA gravitational wave data: (batch, channels, time, x, y, z)
let input = Tensor::new(ArrayD::from_shape_fn(
    ndarray::IxDyn(&[1, 3, 16, 8, 8, 8]),
    |_| rand::random::<f32>()
));

// 4D convolution - unique to Avila ML!
let conv4d = Conv4d::new(
    3,           // input channels (h+, hx, frequency)
    16,          // output channels
    (3, 3, 3, 3) // kernel size (t, x, y, z)
);

let output = conv4d.forward(&input);
println!("Detected spacetime patterns: {:?}", output.shape());
// Output: [1, 16, 14, 6, 6, 6] - learned features in 4D spacetime
```

## ✅ Production Quality

**All tests passing:**
- ✅ 30 unit tests (tensor, layers, optimizers)
- ✅ 7 gradient checking tests (validates mathematical correctness)
- ✅ 4 working examples (regression, MNIST, Conv4d, LIGO)
- ✅ Doctests validated
- ✅ Release build optimized (LTO, codegen-units=1)

## 📦 Architecture

```
avila-ml/
├── tensor.rs        # Tensor with autograd support
├── autograd.rs      # Backward propagation engine
├── nn/
│   ├── mod.rs       # Linear, Conv2d, Conv4d, Sequential
│   ├── activation.rs # ReLU, Sigmoid, Tanh, Softmax, GELU
│   ├── normalization.rs # BatchNorm, LayerNorm, Dropout
│   └── attention.rs # Attention, MultiHeadAttention, Transformer
├── optim.rs         # SGD, Adam, AdamW, RMSprop
├── loss.rs          # MSE, CrossEntropy, BCE, Huber
├── data.rs          # Dataset, DataLoader
└── utils.rs         # Initialization, schedulers, early stopping
```

## 🧪 Examples

### Linear Regression

```bash
cargo run --example linear_regression
```

### MNIST Classification

```bash
cargo run --example mnist_training
```

### Gravitational Wave Detection (4D Conv)

```bash
cargo run --example conv4d_astrophysics
```

## 🎯 Use Cases

### Scientific Computing
- 🌌 **Gravitational wave detection** (LIGO, LISA)
- 🌍 **Climate modeling** (3D space + time)
- 🧬 **Medical imaging** (CT/MRI sequences)
- ⚛️ **Particle physics** (detector events)
- 🌊 **Fluid dynamics** simulations

### General ML
- 📸 Image classification (MNIST, CIFAR)
- 💬 Natural language processing (transformers)
- 🎮 Reinforcement learning
- 📊 Time series forecasting
- 🔍 Anomaly detection

## 🆚 Comparisons

### Avila ML vs. PyTorch/TensorFlow
- ✅ **Pure Rust** - No Python runtime, better deployment
- ✅ **Memory safe** - No segfaults, guaranteed by Rust
- ✅ **Conv4d native** - Built for scientific data
- ✅ **Lightweight** - No heavy dependencies
- ⚠️ **Early stage** - Ecosystem still growing

### Avila ML vs. Other Rust ML Libraries
- ✅ **Conv4d support** - Unique for 4D spacetime data
- ✅ **Full autograd** - Like PyTorch, but in Rust
- ✅ **Scientific focus** - Optimized for research
- ✅ **Modern API** - Ergonomic and type-safe

## 🗺️ Roadmap

### ✅ Version 1.0 (Production Ready)
- [x] Tensor with autograd (Arc<Mutex> gradient sharing)
- [x] Complete backward propagation engine
- [x] Gradient checking (7 tests validating correctness)
- [x] Linear, Conv2d, Conv4d layers
- [x] Activation functions (ReLU, Sigmoid, Tanh, Softmax)
- [x] Optimizers (SGD, Adam, AdamW, RMSprop)
- [x] Loss functions (MSE, CrossEntropy, BCE, Huber)
- [x] DataLoader with batching
- [x] Attention mechanisms
- [x] Conv4d for scientific data (parallelized with Rayon)
- [x] Examples: MNIST, Linear Regression, LIGO/LISA

### 🚧 Version 1.1 (Performance)
- [ ] FFT-based convolutions for large kernels
- [ ] im2col optimization for Conv2d
- [ ] BLAS integration for matrix operations
- [ ] Benchmarking suite

### 🔮 Version 2.0 (GPU & Advanced)
- [ ] GPU acceleration (CUDA/ROCm via wgpu)
- [ ] Model serialization (save/load with serde)
- [ ] Distributed training
- [ ] Mixed precision (f16/bf16)
- [ ] Advanced architectures (ResNet, Vision Transformer)
- [ ] Integration with AvilaDB for ML pipelines

## 🤝 Contributing

We welcome contributions! Priority areas:

**High Priority:**
- GPU support (CUDA/ROCm via wgpu or cudarc)
- FFT-based convolutions for performance
- Model serialization (saving/loading trained models)
- More examples and tutorials

**Medium Priority:**
- BLAS integration for faster matrix operations
- Benchmark suite comparing with PyTorch
- Documentation improvements
- Advanced architectures (ResNet, ViT, BERT)

**Getting Started:**
1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes (ensure tests pass: `cargo test --release`)
4. Commit your changes (`git commit -m 'Add amazing feature'`)
5. Push to the branch (`git push origin feature/amazing-feature`)
6. Open a Pull Request

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines.

## 📚 Resources

- [Documentation](https://docs.rs/avila-ml)
- [Examples](./examples)
- [Arxis Physics Engine](https://github.com/avilaops/arxis)
- [Avila Cloud Platform](https://avila.cloud)

## 📄 License

Licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.

## 🇧🇷 Made in Brazil

Built with ❤️ by [Avila Cloud](https://avila.cloud) - Infrastructure for Brazilian developers.

**Avila ML v1.0** - Machine Learning genuíno do Brasil! 🚀

---

### Related Projects
- **[AvilaDB](https://docs.avila.cloud/aviladb)** - Distributed NoSQL database with 4MB documents and vector search
- **[Arxis](https://github.com/avilaops/arxis)** - Physics engine for scientific simulations
- **[AVL Platform](https://avila.cloud)** - Cloud platform built for Brazil and LATAM
