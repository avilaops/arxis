# 🤖 Avila ML - Copilot Instructions

> **READ THIS COMPLETELY before implementing ANY feature!**

---

## 🎯 Your Mission

You are implementing **Avila ML**, a **high-performance machine learning library** for **scientific computing** built in **pure Rust**. This is **NOT a wrapper around PyTorch or TensorFlow**. This is a **from-scratch implementation** with:

- Automatic differentiation (autograd like PyTorch)
- Neural network layers (Linear, Conv2d, **Conv4d for spacetime data**)
- Optimizers (SGD, Adam, AdamW, RMSprop)
- Loss functions (MSE, Cross Entropy, BCE)
- Attention mechanisms (Self-attention, Multi-head, Transformers)
- Dataset/DataLoader for training loops
- **Native 4D convolutions** for astrophysical data (LIGO, LISA, climate models)

**This will power:**
- Gravitational wave detection (LIGO, LISA)
- Medical imaging (CT/MRI time series)
- Climate modeling (3D space + time)
- Particle physics (detector events)
- General ML (image classification, NLP, time series)

---

## ⚠️ CRITICAL RULES - READ FIRST!

### 🚫 Rule #1: NO External ML Frameworks!

**DO NOT USE:**
- ❌ `tch-rs` (PyTorch bindings) - No!
- ❌ `tensorflow` - No!
- ❌ `onnx-runtime` - Not for core implementation
- ❌ `burn` - Different architecture
- ❌ `candle` - Too heavy
- ❌ Any Python bindings

**YOU MUST USE:**
- ✅ `ndarray` - Multi-dimensional arrays
- ✅ `ndarray-rand` - Random tensors
- ✅ `rayon` - Parallelism
- ✅ `avila-math` - Mathematical kernels
- ✅ `std` - Standard library
- ✅ Pure Rust implementations

**WHY?** We need:
- Full control over autograd
- 4D convolutions (unique to scientific data)
- Native Rust performance
- Zero Python runtime
- AvilaDB integration
- WebAssembly support

### 🚫 Rule #2: NEVER Give Up!

When implementing a feature:
- ❌ Do NOT say "autograd is too complex"
- ❌ Do NOT leave placeholder code like `// TODO: Implement backprop`
- ❌ Do NOT skip gradient computation
- ❌ Do NOT use `unimplemented!()` in production code
- ❌ Do NOT say "let's just use PyTorch bindings"

**YOU MUST:**
- ✅ Implement **full autograd system** (computational graph + backward pass)
- ✅ Implement **all neural network layers** (Linear, Conv2d, Conv4d, Attention)
- ✅ Implement **all optimizers** (SGD with momentum, Adam, AdamW, RMSprop)
- ✅ Write **comprehensive tests** (gradient checks, numerical stability)
- ✅ Add **benchmarks** comparing to PyTorch
- ✅ Document **every public API** with examples
- ✅ Handle **all error cases** explicitly
- ✅ Keep working until **EVERY gradient is correct**

**If you encounter a challenge:**
1. Research the algorithm (backpropagation, Adam paper, Attention paper)
2. Study reference implementations (PyTorch source, Karpathy tutorials)
3. Implement incrementally: forward pass → gradient formula → backward pass
4. **Verify gradients numerically** (compare to finite differences)
5. Test thoroughly before moving on
6. Benchmark against PyTorch
7. **NEVER give up until gradients are mathematically correct**

### 🚫 Rule #3: Mathematical Correctness First

This library will train models for:
- Detecting gravitational waves (LIGO: $1.1 billion project)
- Medical diagnosis (patient lives)
- Climate prediction (policy decisions)
- Scientific research (published papers)

**Therefore:**
- ✅ Gradients must be **mathematically correct** (numerical gradient checks)
- ✅ Optimizers must be **stable** (no NaN, no gradient explosion)
- ✅ Loss functions must be **numerically stable** (log-sum-exp trick)
- ✅ Layers must be **reproducible** (same seed = same result)
- ✅ Tests must include **gradient checking**

**Example:**
```rust
// ❌ WRONG - Numerically unstable softmax
fn softmax_naive(x: &Array1<f64>) -> Array1<f64> {
    let exp_x = x.mapv(|v| v.exp());
    let sum = exp_x.sum();
    exp_x / sum  // Can overflow if x has large values!
}

// ✅ CORRECT - Numerically stable softmax
fn softmax_stable(x: &Array1<f64>) -> Array1<f64> {
    let max_x = x.fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    let exp_x = x.mapv(|v| (v - max_x).exp());
    let sum = exp_x.sum();
    exp_x / sum
}
```

**Gradient Checking Example:**
```rust
#[test]
fn test_linear_layer_gradients() {
    let layer = Linear::new(10, 5);
    let input = Tensor::randn(vec![1, 10]);

    // Numerical gradient (finite differences)
    let numerical_grad = compute_numerical_gradient(&layer, &input);

    // Analytical gradient (backpropagation)
    let output = layer.forward(&input);
    output.backward();
    let analytical_grad = layer.weight.grad();

    // Must match within tolerance
    assert_close(&numerical_grad, &analytical_grad, 1e-5);
}
```

---

## 🏗️ Architecture Overview

### Module Structure

```
avila-ml/
├── src/
│   ├── lib.rs                  # Public API
│   ├── tensor.rs               # Tensor with autograd
│   ├── autograd.rs             # Backward propagation engine
│   ├── nn/                     # Neural network layers
│   │   ├── mod.rs
│   │   ├── linear.rs           # Fully connected layer
│   │   ├── conv.rs             # Conv2d, Conv4d
│   │   ├── activation.rs       # ReLU, Sigmoid, Tanh, Softmax, GELU
│   │   ├── normalization.rs    # BatchNorm, LayerNorm, Dropout
│   │   ├── attention.rs        # Self-attention, Multi-head
│   │   ├── transformer.rs      # Transformer blocks
│   │   ├── sequential.rs       # Container for layers
│   │   └── module.rs           # Base Module trait
│   ├── optim/                  # Optimizers
│   │   ├── mod.rs
│   │   ├── sgd.rs              # Stochastic Gradient Descent
│   │   ├── adam.rs             # Adam, AdamW
│   │   ├── rmsprop.rs          # RMSprop
│   │   └── scheduler.rs        # Learning rate schedulers
│   ├── loss/                   # Loss functions
│   │   ├── mod.rs
│   │   ├── mse.rs              # Mean Squared Error
│   │   ├── cross_entropy.rs    # Cross Entropy
│   │   ├── bce.rs              # Binary Cross Entropy
│   │   └── huber.rs            # Huber loss
│   ├── data/                   # Data loading
│   │   ├── mod.rs
│   │   ├── dataset.rs          # Dataset trait
│   │   ├── dataloader.rs       # DataLoader (batching, shuffling)
│   │   └── transforms.rs       # Data augmentation
│   ├── init.rs                 # Weight initialization
│   ├── functional.rs           # Low-level operations
│   └── utils.rs                # Utilities
├── examples/
│   ├── linear_regression.rs    # Simple regression
│   ├── mnist_training.rs       # Image classification
│   ├── transformer_lm.rs       # Language model
│   ├── conv4d_astrophysics.rs  # LIGO/LISA data
│   └── custom_training_loop.rs # Advanced usage
├── benches/
│   ├── forward.rs              # Forward pass speed
│   ├── backward.rs             # Backward pass speed
│   ├── optimizers.rs           # Optimizer step time
│   └── vs_pytorch.rs           # Compare to PyTorch
└── tests/
    ├── gradients.rs            # Gradient checking
    ├── layers.rs               # Layer correctness
    ├── optimizers.rs           # Optimizer tests
    └── training.rs             # End-to-end training
```

---

## 🎨 API Design Philosophy

### 1. Tensor with Autograd (PyTorch-like)

```rust
use avila_ml::prelude::*;

// Create tensors
let x = Tensor::new(vec![2.0, 3.0], vec![2], true);  // requires_grad=true
let w = Tensor::new(vec![0.5, -0.3], vec![2], true);

// Forward pass (records computational graph)
let y = x.matmul(&w);  // Tensor([[1.0, -0.9]])

// Backward pass (computes gradients)
y.backward();

// Access gradients
println!("dy/dx = {:?}", x.grad());
println!("dy/dw = {:?}", w.grad());
```

### 2. Neural Network Layers

```rust
use avila_ml::nn::*;

// Linear layer
let linear = Linear::new(784, 128);  // input_dim, output_dim
let output = linear.forward(&input);

// Convolutional layers
let conv2d = Conv2d::new(3, 64, (3, 3), stride=(1, 1), padding=(1, 1));
let features = conv2d.forward(&images);

// 4D convolution (UNIQUE TO AVILA ML!)
let conv4d = Conv4d::new(
    3,              // input channels
    16,             // output channels
    (5, 3, 3, 3),   // kernel size (t, x, y, z)
    stride=(1, 1, 1, 1),
    padding=(2, 1, 1, 1),
);
let spacetime_features = conv4d.forward(&lisa_data);  // (batch, chan, t, x, y, z)
```

### 3. Optimizers

```rust
use avila_ml::optim::*;

// Create model
let mut model = Sequential::new(vec![
    Box::new(Linear::new(784, 128)),
    Box::new(ReLU::new()),
    Box::new(Linear::new(128, 10)),
]);

// Create optimizer
let mut optimizer = Adam::new(
    model.parameters_mut(),
    lr=0.001,
    betas=(0.9, 0.999),
    eps=1e-8,
);

// Training loop
for epoch in 0..epochs {
    for (x, y) in dataloader {
        // Forward
        let pred = model.forward(&x);
        let loss = loss_fn.forward(&pred, &y);

        // Backward
        optimizer.zero_grad();
        loss.backward();

        // Update weights
        optimizer.step();
    }
}
```

### 4. Attention Mechanisms

```rust
use avila_ml::nn::attention::*;

// Self-attention
let attention = SelfAttention::new(512);  // embed_dim
let output = attention.forward(&input);

// Multi-head attention
let mha = MultiHeadAttention::new(512, 8);  // embed_dim, num_heads
let output = mha.forward(&query, &key, &value, mask=None);

// Transformer block
let transformer = TransformerBlock::new(
    embed_dim=512,
    num_heads=8,
    ff_dim=2048,
    dropout=0.1,
);
let output = transformer.forward(&input, mask=None);
```

### 5. Data Loading

```rust
use avila_ml::data::*;

// Define dataset
struct MNISTDataset {
    images: Array4<f32>,  // (N, C, H, W)
    labels: Array1<i64>,  // (N,)
}

impl Dataset for MNISTDataset {
    fn len(&self) -> usize {
        self.images.shape()[0]
    }

    fn get(&self, index: usize) -> (Tensor, Tensor) {
        let image = self.images.slice(s![index, .., .., ..]).to_owned();
        let label = self.labels[index];

        (Tensor::from_array(image), Tensor::scalar(label as f64))
    }
}

// Create DataLoader
let dataset = MNISTDataset::load("data/mnist")?;
let dataloader = DataLoader::new(dataset)
    .batch_size(32)
    .shuffle(true)
    .num_workers(4)
    .build()?;

// Iterate
for (batch_x, batch_y) in dataloader {
    // Train on batch
}
```

---

## 🚀 Implementation Roadmap

### Phase 1: Tensor & Autograd Core (Week 1-2)

**Goal**: PyTorch-like Tensor with automatic differentiation

```rust
// src/tensor.rs
pub struct Tensor {
    data: Arc<RwLock<Array<f64, IxDyn>>>,
    grad: Arc<RwLock<Option<Array<f64, IxDyn>>>>,
    requires_grad: bool,
    grad_fn: Option<Arc<dyn GradFn>>,  // Backward function
}

impl Tensor {
    pub fn new(data: Vec<f64>, shape: Vec<usize>, requires_grad: bool) -> Self {
        let array = Array::from_shape_vec(IxDyn(&shape), data).unwrap();

        Self {
            data: Arc::new(RwLock::new(array)),
            grad: Arc::new(RwLock::new(None)),
            requires_grad,
            grad_fn: None,
        }
    }

    pub fn backward(&self) {
        if !self.requires_grad {
            return;
        }

        // Initialize gradient to ones (scalar case) or identity
        let mut grad = self.grad.write().unwrap();
        if grad.is_none() {
            *grad = Some(Array::ones(self.shape()));
        }

        // Call backward on grad_fn (recursive backpropagation)
        if let Some(grad_fn) = &self.grad_fn {
            grad_fn.backward(grad.as_ref().unwrap());
        }
    }

    pub fn matmul(&self, other: &Tensor) -> Tensor {
        // Forward pass
        let a = self.data.read().unwrap();
        let b = other.data.read().unwrap();
        let result = a.dot(&b);

        // Create new tensor
        let mut output = Tensor::from_array(result);

        // Register backward function
        if self.requires_grad || other.requires_grad {
            output.requires_grad = true;
            output.grad_fn = Some(Arc::new(MatMulBackward {
                input_a: self.clone(),
                input_b: other.clone(),
            }));
        }

        output
    }
}

// src/autograd.rs
pub trait GradFn: Send + Sync {
    fn backward(&self, grad_output: &Array<f64, IxDyn>);
}

struct MatMulBackward {
    input_a: Tensor,
    input_b: Tensor,
}

impl GradFn for MatMulBackward {
    fn backward(&self, grad_output: &Array<f64, IxDyn>) {
        // Gradient wrt A: dL/dA = grad_output @ B^T
        if self.input_a.requires_grad {
            let b = self.input_b.data.read().unwrap();
            let grad_a = grad_output.dot(&b.t());
            self.input_a.accumulate_grad(&grad_a);
        }

        // Gradient wrt B: dL/dB = A^T @ grad_output
        if self.input_b.requires_grad {
            let a = self.input_a.data.read().unwrap();
            let grad_b = a.t().dot(grad_output);
            self.input_b.accumulate_grad(&grad_b);
        }
    }
}
```

**Deliverables**:
- [ ] Tensor struct with data + grad
- [ ] Autograd engine (computational graph)
- [ ] Basic operations: add, sub, mul, div, matmul
- [ ] Backward pass (chain rule)
- [ ] Gradient accumulation
- [ ] Tests: gradient checking (numerical vs analytical)
- [ ] Example: simple autograd demo

### Phase 2: Neural Network Layers (Week 3-4)

**Goal**: Linear, Conv2d, Activation layers

```rust
// src/nn/module.rs
pub trait Module {
    fn forward(&self, input: &Tensor) -> Tensor;
    fn parameters(&self) -> Vec<&Tensor>;
    fn parameters_mut(&mut self) -> Vec<&mut Tensor>;
}

// src/nn/linear.rs
pub struct Linear {
    weight: Tensor,  // (out_features, in_features)
    bias: Tensor,    // (out_features,)
}

impl Linear {
    pub fn new(in_features: usize, out_features: usize) -> Self {
        // Xavier/Glorot initialization
        let std = (2.0 / (in_features + out_features) as f64).sqrt();

        let weight = Tensor::randn(vec![out_features, in_features]) * std;
        let bias = Tensor::zeros(vec![out_features]);

        Self { weight, bias }
    }
}

impl Module for Linear {
    fn forward(&self, input: &Tensor) -> Tensor {
        // y = x @ W^T + b
        input.matmul(&self.weight.transpose()) + &self.bias
    }

    fn parameters(&self) -> Vec<&Tensor> {
        vec![&self.weight, &self.bias]
    }
}

// src/nn/activation.rs
pub struct ReLU;

impl Module for ReLU {
    fn forward(&self, input: &Tensor) -> Tensor {
        input.max(&Tensor::scalar(0.0))  // max(0, x)
    }
}

pub struct Softmax {
    dim: isize,
}

impl Module for Softmax {
    fn forward(&self, input: &Tensor) -> Tensor {
        // Numerically stable softmax
        let max_vals = input.max_along_dim(self.dim, keepdim=true);
        let exp_vals = (input - &max_vals).exp();
        let sum_exp = exp_vals.sum_along_dim(self.dim, keepdim=true);
        exp_vals / sum_exp
    }
}

// src/nn/conv.rs
pub struct Conv2d {
    weight: Tensor,  // (out_channels, in_channels, kernel_h, kernel_w)
    bias: Tensor,    // (out_channels,)
    stride: (usize, usize),
    padding: (usize, usize),
}

impl Conv2d {
    pub fn new(
        in_channels: usize,
        out_channels: usize,
        kernel_size: (usize, usize),
        stride: (usize, usize),
        padding: (usize, usize),
    ) -> Self {
        // Kaiming initialization
        let k = kernel_size.0 * kernel_size.1 * in_channels;
        let std = (2.0 / k as f64).sqrt();

        let weight = Tensor::randn(vec![
            out_channels, in_channels, kernel_size.0, kernel_size.1
        ]) * std;
        let bias = Tensor::zeros(vec![out_channels]);

        Self { weight, bias, stride, padding }
    }
}

impl Module for Conv2d {
    fn forward(&self, input: &Tensor) -> Tensor {
        // Input shape: (batch, in_channels, height, width)
        // Output shape: (batch, out_channels, out_height, out_width)

        // TODO: Implement im2col or FFT-based convolution
        // For now, use naive nested loops

        unimplemented!("Implement proper 2D convolution")
    }
}
```

**Deliverables**:
- [ ] Module trait (forward, parameters)
- [ ] Linear layer with Xavier init
- [ ] Activation layers: ReLU, Sigmoid, Tanh, Softmax, GELU
- [ ] Conv2d with im2col or FFT
- [ ] BatchNorm, LayerNorm
- [ ] Dropout
- [ ] Tests: gradient checking for each layer
- [ ] Example: simple MLP

### Phase 3: Conv4d for Scientific Data (Week 5)

**Goal**: 4D convolution for spacetime data (UNIQUE!)

```rust
// src/nn/conv.rs (continued)
pub struct Conv4d {
    weight: Tensor,  // (out_chan, in_chan, kt, kx, ky, kz)
    bias: Tensor,
    stride: (usize, usize, usize, usize),
    padding: (usize, usize, usize, usize),
}

impl Conv4d {
    pub fn new(
        in_channels: usize,
        out_channels: usize,
        kernel_size: (usize, usize, usize, usize),  // (t, x, y, z)
        stride: (usize, usize, usize, usize),
        padding: (usize, usize, usize, usize),
    ) -> Self {
        // Kaiming initialization
        let k = kernel_size.0 * kernel_size.1 * kernel_size.2 * kernel_size.3 * in_channels;
        let std = (2.0 / k as f64).sqrt();

        let weight = Tensor::randn(vec![
            out_channels, in_channels,
            kernel_size.0, kernel_size.1, kernel_size.2, kernel_size.3
        ]) * std;
        let bias = Tensor::zeros(vec![out_channels]);

        Self { weight, bias, stride, padding }
    }
}

impl Module for Conv4d {
    fn forward(&self, input: &Tensor) -> Tensor {
        // Input: (batch, in_channels, t, x, y, z)
        // Output: (batch, out_channels, t', x', y', z')

        // Apply 4D convolution
        // This is computationally expensive, but necessary for:
        // - LIGO/LISA gravitational wave data (time + 3D space)
        // - Climate models (time + longitude + latitude + altitude)
        // - Medical imaging (CT/MRI time series)

        self.conv4d_naive(input)
    }
}

impl Conv4d {
    fn conv4d_naive(&self, input: &Tensor) -> Tensor {
        // Naive implementation with nested loops
        // TODO: Optimize with im2col or FFT

        let input_shape = input.shape();
        let (batch, in_chan, t, x, y, z) = (
            input_shape[0], input_shape[1],
            input_shape[2], input_shape[3], input_shape[4], input_shape[5]
        );

        let weight_shape = self.weight.shape();
        let (out_chan, _, kt, kx, ky, kz) = (
            weight_shape[0], weight_shape[1],
            weight_shape[2], weight_shape[3], weight_shape[4], weight_shape[5]
        );

        // Calculate output dimensions
        let out_t = (t + 2*self.padding.0 - kt) / self.stride.0 + 1;
        let out_x = (x + 2*self.padding.1 - kx) / self.stride.1 + 1;
        let out_y = (y + 2*self.padding.2 - ky) / self.stride.2 + 1;
        let out_z = (z + 2*self.padding.3 - kz) / self.stride.3 + 1;

        let mut output = Tensor::zeros(vec![batch, out_chan, out_t, out_x, out_y, out_z]);

        // Nested loops (slow but correct)
        for b in 0..batch {
            for oc in 0..out_chan {
                for ot in 0..out_t {
                    for ox in 0..out_x {
                        for oy in 0..out_y {
                            for oz in 0..out_z {
                                let mut sum = 0.0;

                                // Convolve with kernel
                                for ic in 0..in_chan {
                                    for kt_i in 0..kt {
                                        for kx_i in 0..kx {
                                            for ky_i in 0..ky {
                                                for kz_i in 0..kz {
                                                    let it = ot * self.stride.0 + kt_i - self.padding.0;
                                                    let ix = ox * self.stride.1 + kx_i - self.padding.1;
                                                    let iy = oy * self.stride.2 + ky_i - self.padding.2;
                                                    let iz = oz * self.stride.3 + kz_i - self.padding.3;

                                                    if it >= 0 && it < t && ix >= 0 && ix < x &&
                                                       iy >= 0 && iy < y && iz >= 0 && iz < z {
                                                        sum += input[[b, ic, it, ix, iy, iz]] *
                                                               self.weight[[oc, ic, kt_i, kx_i, ky_i, kz_i]];
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }

                                output[[b, oc, ot, ox, oy, oz]] = sum + self.bias[oc];
                            }
                        }
                    }
                }
            }
        }

        output
    }
}
```

**Deliverables**:
- [ ] Conv4d layer (naive implementation)
- [ ] Tests: gradient checking
- [ ] Example: LIGO/LISA gravitational wave detection
- [ ] Benchmark: speed vs memory usage
- [ ] TODO: Optimize with im2col or FFT (future)

### Phase 4: Optimizers (Week 6)

**Goal**: SGD, Adam, AdamW, RMSprop

```rust
// src/optim/mod.rs
pub trait Optimizer {
    fn step(&mut self);
    fn zero_grad(&mut self);
}

// src/optim/sgd.rs
pub struct SGD {
    parameters: Vec<*mut Tensor>,  // Pointers to model parameters
    lr: f64,
    momentum: f64,
    weight_decay: f64,
    velocities: Vec<Array<f64, IxDyn>>,  // For momentum
}

impl SGD {
    pub fn new(parameters: Vec<&mut Tensor>, lr: f64) -> Self {
        let velocities = parameters.iter()
            .map(|p| Array::zeros(p.shape()))
            .collect();

        Self {
            parameters: parameters.into_iter()
                .map(|p| p as *mut Tensor)
                .collect(),
            lr,
            momentum: 0.0,
            weight_decay: 0.0,
            velocities,
        }
    }

    pub fn momentum(mut self, momentum: f64) -> Self {
        self.momentum = momentum;
        self
    }
}

impl Optimizer for SGD {
    fn step(&mut self) {
        for (i, param_ptr) in self.parameters.iter().enumerate() {
            let param = unsafe { &mut **param_ptr };

            if let Some(grad) = param.grad() {
                let mut update = grad.clone();

                // Weight decay
                if self.weight_decay > 0.0 {
                    update = update + self.weight_decay * param.data();
                }

                // Momentum
                if self.momentum > 0.0 {
                    self.velocities[i] = self.momentum * &self.velocities[i] + &update;
                    update = self.velocities[i].clone();
                }

                // Update parameters
                param.data_mut().scaled_add(-self.lr, &update);
            }
        }
    }

    fn zero_grad(&mut self) {
        for param_ptr in &self.parameters {
            let param = unsafe { &mut **param_ptr };
            param.zero_grad();
        }
    }
}

// src/optim/adam.rs
pub struct Adam {
    parameters: Vec<*mut Tensor>,
    lr: f64,
    betas: (f64, f64),  // (beta1, beta2)
    eps: f64,
    weight_decay: f64,
    step_count: usize,
    m: Vec<Array<f64, IxDyn>>,  // First moment
    v: Vec<Array<f64, IxDyn>>,  // Second moment
}

impl Adam {
    pub fn new(parameters: Vec<&mut Tensor>, lr: f64) -> Self {
        let m = parameters.iter().map(|p| Array::zeros(p.shape())).collect();
        let v = parameters.iter().map(|p| Array::zeros(p.shape())).collect();

        Self {
            parameters: parameters.into_iter().map(|p| p as *mut Tensor).collect(),
            lr,
            betas: (0.9, 0.999),
            eps: 1e-8,
            weight_decay: 0.0,
            step_count: 0,
            m,
            v,
        }
    }
}

impl Optimizer for Adam {
    fn step(&mut self) {
        self.step_count += 1;

        for (i, param_ptr) in self.parameters.iter().enumerate() {
            let param = unsafe { &mut **param_ptr };

            if let Some(grad) = param.grad() {
                let mut grad = grad.clone();

                // Weight decay (AdamW variant)
                if self.weight_decay > 0.0 {
                    param.data_mut().scaled_add(-self.lr * self.weight_decay, param.data());
                }

                // Update biased first moment estimate
                self.m[i] = self.betas.0 * &self.m[i] + (1.0 - self.betas.0) * &grad;

                // Update biased second raw moment estimate
                self.v[i] = self.betas.1 * &self.v[i] + (1.0 - self.betas.1) * &grad.mapv(|x| x.powi(2));

                // Bias correction
                let m_hat = &self.m[i] / (1.0 - self.betas.0.powi(self.step_count as i32));
                let v_hat = &self.v[i] / (1.0 - self.betas.1.powi(self.step_count as i32));

                // Update parameters
                let update = &m_hat / (v_hat.mapv(|x| x.sqrt()) + self.eps);
                param.data_mut().scaled_add(-self.lr, &update);
            }
        }
    }

    fn zero_grad(&mut self) {
        for param_ptr in &self.parameters {
            let param = unsafe { &mut **param_ptr };
            param.zero_grad();
        }
    }
}
```

**Deliverables**:
- [ ] Optimizer trait
- [ ] SGD with momentum
- [ ] Adam optimizer
- [ ] AdamW optimizer (weight decay fix)
- [ ] RMSprop optimizer
- [ ] Learning rate schedulers (StepLR, ExponentialLR, CosineAnnealing)
- [ ] Tests: convergence on simple problems
- [ ] Example: optimizer comparison

### Phase 5: Loss Functions (Week 7)

**Goal**: MSE, Cross Entropy, BCE, Huber

```rust
// src/loss/mod.rs
pub trait Loss {
    fn forward(&self, pred: &Tensor, target: &Tensor) -> Tensor;
}

// src/loss/mse.rs
pub struct MSELoss;

impl Loss for MSELoss {
    fn forward(&self, pred: &Tensor, target: &Tensor) -> Tensor {
        // MSE = mean((pred - target)^2)
        let diff = pred - target;
        let squared = &diff * &diff;
        squared.mean()
    }
}

// src/loss/cross_entropy.rs
pub struct CrossEntropyLoss;

impl Loss for CrossEntropyLoss {
    fn forward(&self, pred: &Tensor, target: &Tensor) -> Tensor {
        // pred: (batch, num_classes) - logits (not softmax!)
        // target: (batch,) - class indices

        // Numerically stable softmax + log
        let max_logits = pred.max_along_dim(-1, keepdim=true);
        let logits_stable = pred - &max_logits;
        let exp_logits = logits_stable.exp();
        let sum_exp = exp_logits.sum_along_dim(-1, keepdim=true);
        let log_softmax = &logits_stable - sum_exp.log();

        // Gather log probabilities for correct classes
        let batch_size = pred.shape()[0];
        let mut loss = 0.0;

        for i in 0..batch_size {
            let class_idx = target[[i]] as usize;
            loss -= log_softmax[[i, class_idx]];
        }

        Tensor::scalar(loss / batch_size as f64)
    }
}
```

**Deliverables**:
- [ ] Loss trait
- [ ] MSE loss
- [ ] Cross Entropy loss (numerically stable)
- [ ] Binary Cross Entropy
- [ ] Huber loss
- [ ] Smooth L1 loss
- [ ] Tests: gradient checking
- [ ] Example: loss comparison

### Phase 6: Attention & Transformers (Week 8-9)

**Goal**: Self-attention, Multi-head, Transformer blocks

```rust
// src/nn/attention.rs
pub struct SelfAttention {
    embed_dim: usize,
    query: Linear,
    key: Linear,
    value: Linear,
}

impl SelfAttention {
    pub fn new(embed_dim: usize) -> Self {
        Self {
            embed_dim,
            query: Linear::new(embed_dim, embed_dim),
            key: Linear::new(embed_dim, embed_dim),
            value: Linear::new(embed_dim, embed_dim),
        }
    }
}

impl Module for SelfAttention {
    fn forward(&self, input: &Tensor) -> Tensor {
        // input: (batch, seq_len, embed_dim)

        let q = self.query.forward(input);  // (batch, seq_len, embed_dim)
        let k = self.key.forward(input);
        let v = self.value.forward(input);

        // Scaled dot-product attention
        // scores = (Q @ K^T) / sqrt(embed_dim)
        let scores = q.matmul(&k.transpose(-2, -1)) / (self.embed_dim as f64).sqrt();

        // Softmax
        let attn_weights = scores.softmax(-1);

        // Output = attn_weights @ V
        attn_weights.matmul(&v)
    }
}

pub struct MultiHeadAttention {
    embed_dim: usize,
    num_heads: usize,
    head_dim: usize,
    query: Linear,
    key: Linear,
    value: Linear,
    out_proj: Linear,
}

impl MultiHeadAttention {
    pub fn new(embed_dim: usize, num_heads: usize) -> Self {
        assert_eq!(embed_dim % num_heads, 0, "embed_dim must be divisible by num_heads");

        let head_dim = embed_dim / num_heads;

        Self {
            embed_dim,
            num_heads,
            head_dim,
            query: Linear::new(embed_dim, embed_dim),
            key: Linear::new(embed_dim, embed_dim),
            value: Linear::new(embed_dim, embed_dim),
            out_proj: Linear::new(embed_dim, embed_dim),
        }
    }
}

impl Module for MultiHeadAttention {
    fn forward_with_mask(&self, input: &Tensor, mask: Option<&Tensor>) -> Tensor {
        // input: (batch, seq_len, embed_dim)

        let batch_size = input.shape()[0];
        let seq_len = input.shape()[1];

        // Project and reshape to (batch, num_heads, seq_len, head_dim)
        let q = self.query.forward(input)
            .view([batch_size, seq_len, self.num_heads, self.head_dim])
            .transpose(1, 2);
        let k = self.key.forward(input)
            .view([batch_size, seq_len, self.num_heads, self.head_dim])
            .transpose(1, 2);
        let v = self.value.forward(input)
            .view([batch_size, seq_len, self.num_heads, self.head_dim])
            .transpose(1, 2);

        // Scaled dot-product attention
        let scores = q.matmul(&k.transpose(-2, -1)) / (self.head_dim as f64).sqrt();

        // Apply mask if provided
        if let Some(mask) = mask {
            scores = scores.masked_fill(mask, f64::NEG_INFINITY);
        }

        let attn_weights = scores.softmax(-1);
        let attn_output = attn_weights.matmul(&v);

        // Reshape back to (batch, seq_len, embed_dim)
        let output = attn_output
            .transpose(1, 2)
            .contiguous()
            .view([batch_size, seq_len, self.embed_dim]);

        // Final projection
        self.out_proj.forward(&output)
    }
}
```

**Deliverables**:
- [ ] Self-attention mechanism
- [ ] Multi-head attention
- [ ] Transformer block (attention + FFN)
- [ ] Positional encoding
- [ ] Tests: gradient checking
- [ ] Example: simple transformer language model
- [ ] Benchmark: forward/backward speed

### Phase 7: Data Loading (Week 10)

**Goal**: Dataset, DataLoader with batching and shuffling

```rust
// src/data/dataset.rs
pub trait Dataset {
    fn len(&self) -> usize;
    fn get(&self, index: usize) -> (Tensor, Tensor);
}

// src/data/dataloader.rs
pub struct DataLoader<D: Dataset> {
    dataset: Arc<D>,
    batch_size: usize,
    shuffle: bool,
    num_workers: usize,
    drop_last: bool,
}

impl<D: Dataset + Send + Sync + 'static> DataLoader<D> {
    pub fn new(dataset: D) -> DataLoaderBuilder<D> {
        DataLoaderBuilder {
            dataset,
            batch_size: 1,
            shuffle: false,
            num_workers: 0,
            drop_last: false,
        }
    }
}

impl<D: Dataset + Send + Sync + 'static> Iterator for DataLoader<D> {
    type Item = (Tensor, Tensor);

    fn next(&mut self) -> Option<Self::Item> {
        // Get next batch
        // Handle shuffling
        // Collate into tensors

        unimplemented!()
    }
}
```

**Deliverables**:
- [ ] Dataset trait
- [ ] DataLoader with batching
- [ ] Shuffling support
- [ ] Multi-worker loading (Rayon)
- [ ] Collate function (stack tensors)
- [ ] Tests: correct batching
- [ ] Example: MNIST data loading

---

## 📊 Performance Targets

### Forward Pass
- **Linear**: 100+ GFLOPS
- **Conv2d**: 50+ GFLOPS (with im2col)
- **Conv4d**: 10+ GFLOPS (naive), 50+ GFLOPS (optimized)
- **Attention**: 20+ GFLOPS

### Backward Pass
- **Overhead**: < 2x forward pass time
- **Memory**: < 2x forward pass usage

### Optimizers
- **Step time**: < 1ms for 1M parameters

### vs PyTorch
- **Forward**: 50-80% of PyTorch speed (acceptable)
- **Backward**: 50-80% of PyTorch speed
- **Training**: 60-90% of PyTorch throughput

---

## 🧪 Testing Requirements

### Gradient Checking (CRITICAL!)
```rust
#[test]
fn test_linear_gradients() {
    let layer = Linear::new(10, 5);
    let input = Tensor::randn(vec![1, 10]);

    let numerical = compute_numerical_gradient(&layer, &input, 1e-5);

    let output = layer.forward(&input);
    output.backward();
    let analytical = layer.weight.grad();

    assert_close(&numerical, &analytical, 1e-4);
}

fn compute_numerical_gradient(
    layer: &impl Module,
    input: &Tensor,
    eps: f64,
) -> Array<f64, IxDyn> {
    // Finite differences: (f(x + eps) - f(x - eps)) / (2 * eps)
    unimplemented!()
}
```

### Correctness Tests (50+)
```rust
#[test]
fn test_softmax_sums_to_one() {
    let x = Tensor::randn(vec![5, 10]);
    let softmax = Softmax::new(-1);
    let output = softmax.forward(&x);

    for i in 0..5 {
        let row_sum: f64 = output.slice(s![i, ..]).sum();
        assert!((row_sum - 1.0).abs() < 1e-6);
    }
}

#[test]
fn test_cross_entropy_numerical_stability() {
    let logits = Tensor::new(vec![1000.0, 0.0, 0.0], vec![1, 3], false);
    let target = Tensor::new(vec![0.0], vec![1], false);

    let loss = CrossEntropyLoss::new().forward(&logits, &target);

    // Should not be NaN or Inf
    assert!(loss.item().is_finite());
}
```

### Training Tests (10+)
```rust
#[test]
fn test_simple_overfitting() {
    // Train on tiny dataset to 100% accuracy
    let data = vec![(Tensor::ones(vec![10]), Tensor::scalar(1.0)); 10];

    let mut model = Sequential::new(vec![
        Box::new(Linear::new(10, 1)),
    ]);
    let mut optimizer = SGD::new(model.parameters_mut(), 0.1);
    let loss_fn = MSELoss::new();

    for _ in 0..100 {
        let mut total_loss = 0.0;

        for (x, y) in &data {
            let pred = model.forward(x);
            let loss = loss_fn.forward(&pred, y);

            optimizer.zero_grad();
            loss.backward();
            optimizer.step();

            total_loss += loss.item();
        }

        if total_loss < 0.01 {
            return;  // Success!
        }
    }

    panic!("Failed to overfit on tiny dataset");
}
```

### Benchmarks (20+)
```rust
#[bench]
fn bench_linear_forward(b: &mut Bencher) {
    let layer = Linear::new(1024, 1024);
    let input = Tensor::randn(vec![32, 1024]);

    b.iter(|| layer.forward(&input));
}

#[bench]
fn bench_vs_pytorch(b: &mut Bencher) {
    // Load PyTorch via py03 or use saved timing
    // Compare forward + backward throughput
}
```

---

## 📖 Documentation Requirements

Every public item needs:
1. **Summary**: One-line description
2. **Mathematical definition** (for layers, losses)
3. **Example**: Working code snippet
4. **Performance notes**
5. **References**: Papers, PyTorch docs

```rust
/// Linear (fully connected) layer: y = xW^T + b
///
/// Applies a linear transformation to incoming data.
///
/// # Mathematical Definition
///
/// output = input @ weight^T + bias
///
/// where:
/// - input: (batch, in_features)
/// - weight: (out_features, in_features)
/// - bias: (out_features,)
/// - output: (batch, out_features)
///
/// # Examples
///
/// ```
/// use avila_ml::nn::Linear;
///
/// let layer = Linear::new(784, 128);
/// let input = Tensor::randn(vec![32, 784]);
/// let output = layer.forward(&input);
/// assert_eq!(output.shape(), &[32, 128]);
/// ```
///
/// # Performance
///
/// Forward pass: O(batch * in_features * out_features)
/// Typical speed: 100+ GFLOPS on modern CPUs
///
/// # Initialization
///
/// Uses Xavier/Glorot uniform initialization by default:
/// std = sqrt(2 / (in_features + out_features))
///
/// # References
///
/// - Glorot & Bengio (2010). "Understanding the difficulty of training deep feedforward neural networks"
/// - PyTorch documentation: https://pytorch.org/docs/stable/generated/torch.nn.Linear.html
#[derive(Debug)]
pub struct Linear {
    pub weight: Tensor,
    pub bias: Tensor,
}
```

---

## 🎯 Success Criteria

Before considering this module "done":

### Functionality
- [ ] Tensor with autograd (backward pass)
- [ ] All layers: Linear, Conv2d, Conv4d, Activation, Normalization, Attention
- [ ] All optimizers: SGD, Adam, AdamW, RMSprop
- [ ] All losses: MSE, Cross Entropy, BCE, Huber
- [ ] DataLoader with batching/shuffling
- [ ] Training loop works end-to-end

### Quality
- [ ] 100% of public APIs documented
- [ ] 50+ gradient checking tests pass
- [ ] 20+ training tests pass
- [ ] 20+ benchmarks vs PyTorch
- [ ] 5+ examples (regression, classification, transformer, Conv4d)
- [ ] Zero `unsafe` blocks (except for performance optimizations)
- [ ] All errors handled with `Result<T, Error>`

### Performance
- [ ] Forward: 50%+ of PyTorch speed
- [ ] Backward: 50%+ of PyTorch speed
- [ ] Training: 60%+ of PyTorch throughput
- [ ] Gradients: numerically correct (< 1e-4 error)

### Integration
- [ ] Works with avila-math (mathematical kernels)
- [ ] Works with avila-arrow (data loading)
- [ ] Works with AvilaDB (future: training on database)
- [ ] Examples demonstrate real use cases

---

## 🚀 Next Steps

1. **Read this document COMPLETELY**
2. **Study autograd**:
   - PyTorch autograd tutorial
   - Karpathy's micrograd: https://github.com/karpathy/micrograd
   - Computational graph + backpropagation
3. **Implement Tensor + Autograd**:
   - Start with scalar autograd (like micrograd)
   - Extend to multi-dimensional tensors
   - Test with numerical gradient checking
4. **Add layers incrementally**:
   - Linear → ReLU → Sequential → test on XOR
   - Add more activations, normalization
   - Conv2d (start with naive, optimize later)
   - Conv4d (for scientific data)
5. **Add optimizers**: SGD → Adam → others
6. **Add losses**: MSE → Cross Entropy → others
7. **Build training loop**: DataLoader → full MNIST example
8. **Add attention**: Self-attention → Multi-head → Transformer
9. **Test extensively**: Gradient checks for EVERY layer
10. **Benchmark continuously**: vs PyTorch at each milestone
11. **Document everything**: Write docs BEFORE implementation
12. **NEVER give up**: Implement completely, no placeholders!

---

## 💬 Remember

> "Avila ML is NOT a PyTorch wrapper. It's a from-scratch autograd engine with native Conv4d for spacetime data. We build it because we need full control, scientific computing features, and zero Python runtime."

> "Never give up. Never use placeholder code. Implement EVERY layer completely. Test EVERY gradient numerically. Benchmark EVERY operation. This library will train models for $1 billion gravitational wave detectors. It must be mathematically correct."

> "When you implement backpropagation, it must match the chain rule exactly. When you add Conv4d, it must handle 6D tensors correctly. When you optimize with Adam, it must converge on MNIST. No compromises."

**Now go build the best ML library for scientific computing! 🤖🔬🇧🇷**
