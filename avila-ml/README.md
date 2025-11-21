# Avila ML 🚀

[![Crates.io](https://img.shields.io/crates/v/avila-ml)](https://crates.io/crates/avila-ml)
[![Documentation](https://docs.rs/avila-ml/badge.svg)](https://docs.rs/avila-ml)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)](LICENSE-MIT)

**High-performance Machine Learning library for scientific computing, built in Rust.**

Avila ML is a pure-Rust machine learning framework designed for **scientific applications**, with native support for **4D convolutions** (spacetime data), **automatic differentiation**, and **parallel computing**. Built by [Avila Cloud](https://avila.cloud) 🇧🇷

## ✨ Features

- 🔥 **Autograd**: Automatic differentiation with computational graph
- 🧠 **Neural Networks**: Linear, Conv2d, **Conv4d** (unique for scientific data)
- 🎯 **Optimizers**: SGD, Adam, AdamW, RMSprop with learning rate schedulers
- 📊 **Loss Functions**: MSE, Cross Entropy, BCE, Huber, Smooth L1
- 🔄 **Data Loading**: Dataset, DataLoader with batching and shuffling
- 🌟 **Attention**: Self-attention, Multi-head attention, Transformers
- 🧪 **Scientific Computing**: Conv4d for astrophysical/climate data
- ⚡ **Performance**: Built on ndarray with Rayon parallelism
- 🦀 **Pure Rust**: No Python dependencies, type-safe, memory-safe

## 🚀 Quick Start

```toml
[dependencies]
avila-ml = "0.1.0"
```

### Simple Neural Network

```rust
use avila_ml::prelude::*;

// Create a neural network
let model = Sequential::new(vec![
    Box::new(Linear::new(784, 128)),
    Box::new(ReLU::new()),
    Box::new(Linear::new(128, 10)),
    Box::new(Softmax::new(-1)),
]);

// Train with optimizer
let mut optimizer = Adam::new(model.parameters_mut(), 0.001);
let loss_fn = CrossEntropyLoss::new();

for (x, y) in dataloader {
    let pred = model.forward(&x);
    let loss = loss_fn.forward(&pred, &y);

    optimizer.zero_grad();
    loss.backward();
    optimizer.step();
}
```

### 4D Convolution for Astrophysical Data

```rust
use avila_ml::prelude::*;

// LISA gravitational wave data: (time, x, y, z)
let input = Tensor::randn(vec![1, 3, 100, 32, 32, 32]);

// 4D convolution - unique to Avila ML!
let conv4d = Conv4d::new(
    3,           // input channels (h+, hx, frequency)
    16,          // output channels
    (5, 3, 3, 3) // kernel size (t, x, y, z)
);

let output = conv4d.forward(&input);
// Detects spatio-temporal patterns in 4D!
```

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

### ✅ Completed
- [x] Tensor with autograd
- [x] Backward propagation engine
- [x] Linear, activation layers
- [x] Optimizers (SGD, Adam, AdamW, RMSprop)
- [x] Loss functions
- [x] DataLoader
- [x] Attention mechanisms
- [x] Conv4d for scientific data

### 🚧 In Progress
- [ ] Proper convolution implementations (im2col/FFT)
- [ ] GPU acceleration (CUDA/ROCm)
- [ ] Model serialization (save/load)
- [ ] More examples and tutorials

### 🔮 Future
- [ ] Distributed training
- [ ] Mixed precision training
- [ ] Advanced architectures (ResNet, ViT)
- [ ] Integration with AvilaDB for ML pipelines
- [ ] Hugging Face model compatibility

## 🤝 Contributing

We welcome contributions! Areas where you can help:
- Implement proper Conv2d/Conv4d (im2col or FFT-based)
- Add GPU support (CUDA/ROCm)
- Write more examples and tutorials
- Improve documentation
- Add benchmarks

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

**Avila ML** - Machine Learning genuíno do Brasil! 🚀

Autograd (diferenciação automática) -
Camadas neurais (Linear, Conv, Attention)
Otimizadores (SGD, Adam) - fácil
Loss functions
Dataset/DataLoader -
Training loop
