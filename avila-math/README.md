# avila-math

## 🏛️ The Foundation of Arxis

**Mathematical kernel** - The solid bedrock upon which Arxis (ARX + AXIS) is built

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-2021-CE422B.svg)](https://www.rust-lang.org/)
[![Part of Arxis](https://img.shields.io/badge/Arxis-Foundation-00d4ff)](https://github.com/avilaops/arxis)

**avila-math** is the mathematical citadel - providing unshakeable foundations for the entire Avila ecosystem (vision, engine, arxis).

Like the **ARX** (Latin: *fortress*), this crate provides the solid mathematical primitives that protect the integrity of all computations.

## ✨ Features

### 🎯 Geometry
- **3D Quaternions (`Quat3D`)**: Rotations, SLERP interpolation, axis-angle
- **Dual Quaternions (`DualQuat`)**: Rigid body transformations (rotation + translation)
- **SO(4) Rotations**: 4D rotations using S³ × S³ representation
- **4D Geometry**: Tesseract, 24-cell, simplex, projections (4D→3D)
- **AABB**: Axis-aligned bounding boxes for collision detection

### 📊 Tensors & Linear Algebra
- **Vectors**: 2D, 3D, 4D with dot product, cross product, normalization
- **Matrices**: 3×3, 4×4 with multiplication, determinant, inverse, transpose
- **N-D Tensors**: Generalized tensor operations (convolution, pooling, etc.)
- **Conv4D**: 4D convolutional neural networks with forward/backward pass
- **SIMD Optimizations**: AVX2 accelerated operations
- **In-place Operations**: Zero-copy tensor manipulations

### 🔬 Advanced Linear Algebra
- **SVD**: Singular Value Decomposition
- **Eigenvalues/Eigenvectors**: Symmetric and general matrices
- **QR Decomposition**: Orthogonal factorization
- **LU Decomposition**: Lower-upper factorization
- **Linear System Solvers**: Direct and least-squares methods
- **Matrix Rank & Condition Number**: Numerical analysis tools

### 🌊 Signal Processing
- **FFT**: 1D, 2D, 3D, 4D Fast Fourier Transform
- **Spectral Analysis**: Power spectral density, spectrograms
- **Wavelets**: CWT, DWT for gravitational wave detection (LISA)
- **Window Functions**: Hann, Hamming, Blackman, Kaiser
- **Cross-correlation**: Signal comparison and alignment

### ∇ Calculus & Differential Operators (4D)
- **Gradient**: ∇f in 4D space
- **Divergence**: ∇·F for vector fields
- **Curl**: Generalized 4D curl (bivector)
- **Laplacian**: ∇²f for scalar fields
- **D'Alembertian**: Wave operator □ for relativity
- **Hessian**: Second-order derivatives
- **Jacobian**: Function vector derivatives
- **Directional Derivatives**: Rates along vectors

### 🎨 Interpolation (4D)
- **Linear**: lerp, bilinear, trilinear, quadrilinear
- **Bézier Curves**: Quadratic, cubic, arbitrary degree
- **Cubic Splines**: Natural and Catmull-Rom
- **B-splines**: Uniform basis splines
- **Hermite Interpolation**: Tangent-aware curves

### 🔧 Additional Features
- **Serde Support**: Serialize/deserialize tensors (feature flag)
- **Benchmarks**: Criterion-based performance testing
- **Pure Rust**: 100% native implementation
- **Parallel Computing**: Rayon for Conv4D operations

## 📦 Installation

```toml
[dependencies]
avila-math = { git = "https://github.com/avilaops/arxis", branch = "main" }

# With serialization support
avila-math = { git = "https://github.com/avilaops/arxis", branch = "main", features = ["serde"] }
```

## 🚀 Usage Examples

### Quaternions & Rotations
```rust
use avila_math::geometry::Quat3D;
use std::f64::consts::PI;

// Create rotation quaternion
let q = Quat3D::from_axis_angle([0.0, 0.0, 1.0], PI / 2.0);

// Rotate vector
let v = q.rotate_vector([1.0, 0.0, 0.0]);
// v ≈ [0.0, 1.0, 0.0]

// SLERP interpolation
let q2 = Quat3D::from_axis_angle([0.0, 0.0, 1.0], PI);
let interpolated = q.slerp(&q2, 0.5);
```

### Linear Algebra
```rust
use avila_math::linalg::{svd, eigenvalues, solve_linear_system};
use avila_math::tensor::{Matrix, Vector};

// SVD decomposition
let m = Matrix::from_data([3, 2], vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).unwrap();
let (u, singular_values, vt) = svd(&m).unwrap();

// Eigenvalues
let symmetric = Matrix::from_data([2, 2], vec![2.0, 1.0, 1.0, 2.0]).unwrap();
let eigenvals = eigenvalues(&symmetric).unwrap();

// Solve Ax = b
let a = Matrix::from_data([2, 2], vec![2.0, 1.0, 1.0, 3.0]).unwrap();
let b = Vector::from_slice(&[5.0, 6.0]);
let x = solve_linear_system(&a, &b).unwrap();
```

### Conv4D Neural Networks
```rust
use avila_math::tensor::{Conv4DLayer, Conv4DConfig, Tensor6D};

// Create 4D convolutional layer
let mut layer = Conv4DLayer::new(
    8,              // in_channels
    16,             // out_channels
    [3, 3, 3, 3],   // kernel_size
    Conv4DConfig::default()
).with_bias(16);

layer.init_xavier();

// Forward pass
let input = Tensor6D::zeros([2, 8, 16, 16, 16, 16]); // [batch, channels, d1, d2, d3, d4]
let output = layer.forward(&input).unwrap();

// Backward pass for training
let grad_output = Tensor6D::filled(output.shape, 0.1);
let (grad_input, grad_weights, grad_bias) = layer.backward(&input, &grad_output).unwrap();
```

### Differential Calculus (4D)
```rust
use avila_math::calculus::{gradient_4d, laplacian_4d, divergence_4d};

// Scalar field: f(x,y,z,w) = x² + y² + z² + w²
let f = |p: &[f64]| p[0]*p[0] + p[1]*p[1] + p[2]*p[2] + p[3]*p[3];

// Gradient: ∇f = [2x, 2y, 2z, 2w]
let grad = gradient_4d(&f, &[1.0, 2.0, 3.0, 4.0], 1e-7);

// Laplacian: ∇²f = 8
let lap = laplacian_4d(&f, &[1.0, 2.0, 3.0, 4.0], 1e-5);

// Vector field divergence
let field = |p: &[f64; 4]| [p[0], p[1], p[2], p[3]];
let div = divergence_4d(&field, &[1.0, 2.0, 3.0, 4.0], 1e-5);
```

### Interpolation & Curves
```rust
use avila_math::interpolation::{BezierCurve4D, cubic_spline_4d, catmull_rom_4d};

// Bézier curve
let control_points = vec![
    [0.0, 0.0, 0.0, 0.0],
    [1.0, 2.0, 0.0, 0.0],
    [2.0, 0.0, 0.0, 0.0],
];
let curve = BezierCurve4D::new(control_points);
let point = curve.eval(0.5);

// Cubic spline
let points = vec![
    [0.0, 0.0, 0.0, 0.0],
    [1.0, 1.0, 1.0, 1.0],
    [2.0, 0.0, 0.0, 0.0],
];
let interpolated = cubic_spline_4d(&points, 0.75);
```

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific module
cargo test --lib tensor::
```

## ⚡ Benchmarks

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench tensor_ops
cargo bench conv4d
cargo bench quaternions
cargo bench fft
```

## 📈 Performance

Optimized for **Brazil/LATAM** workloads with:
- **AVX2 SIMD**: 4x speedup on compatible CPUs
- **Rayon Parallelism**: Multi-core Conv4D operations
- **Zero-copy**: In-place tensor operations
- **LTO Optimization**: Link-time optimization in release builds

## 🏗️ Project Structure

```
avila-math/
├── src/
│   ├── geometry/        # Quaternions, 4D geometry
│   ├── tensor/          # N-D tensors, Conv4D
│   ├── signal/          # FFT, wavelets, spectral analysis
│   ├── linalg/          # SVD, eigenvalues, solvers
│   ├── calculus/        # Differential operators
│   └── interpolation/   # Curves and splines
├── benches/             # Criterion benchmarks
└── .github/workflows/   # CI/CD automation
```

## 🤝 Contributing

See `CONTRIBUTING.md` for guidelines.

## 📄 License

Licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

---

## 🏛️ Part of Arxis

**avila-math** is the foundation stone of [**Arxis**](https://github.com/avilaops/arxis) - the mathematical citadel.

**ARX** (fortress) + **AXIS** (engine) = **ARXIS**

Built with ❤️ by [Avila](https://avila.cloud)

**136 tests passing** ✅ | **4 benchmarks** ⚡ | **Pure Rust** 🦀
