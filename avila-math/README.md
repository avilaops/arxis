# avila-math

## 🏛️ The Foundation of Arxis

**Mathematical kernel** - The solid bedrock upon which Arxis (ARX + AXIS) is built

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-2021-CE422B.svg)](https://www.rust-lang.org/)
[![Part of Arxis](https://img.shields.io/badge/Arxis-Foundation-00d4ff)](https://github.com/avilaops/arxis)

**avila-math** is the mathematical citadel - providing unshakeable foundations for the entire Avila ecosystem (vision, engine, arxis).

Like the **ARX** (Latin: *fortress*), this crate provides the solid mathematical primitives that protect the integrity of all computations.

## Features

### Geometry
- **3D Quaternions (`Quat3D`)**: Rotations, SLERP interpolation, axis-angle
- **Dual Quaternions (`DualQuat`)**: Rigid body transformations (rotation + translation)
- **SO(4) Rotations**: 4D rotations using S³ × S³ representation
- **4D Geometry**: Tesseract, 24-cell, simplex, projections (4D→3D)
- **AABB**: Axis-aligned bounding boxes for collision detection

### Tensors
- **Vectors**: 2D, 3D, 4D with dot product, cross product, normalization
- **Matrices**: 3×3, 4×4 with multiplication, determinant, inverse, transpose
- **N-D Tensors**: Generalized tensor operations (convolution, pooling, etc.)

## Usage

```rust
use avila_math::geometry::{Quat3D, SO4Rotation, Vector3};

// 3D rotation
let q = Quat3D::from_axis_angle([0.0, 0.0, 1.0], std::f64::consts::PI / 2.0);
let v = q.rotate_vector([1.0, 0.0, 0.0]);
// v ≈ [0.0, 1.0, 0.0]

// 4D rotation
let so4 = SO4Rotation::from_left(q);
let v4 = so4.rotate_vector_4d([1.0, 0.0, 0.0, 0.0]);
```

## Installation

```toml
[dependencies]
avila-math = { git = "https://github.com/avilaops/arxis", branch = "main" }
```

## Tests

```bash
cargo test -p avila-math
```

---

## 🏛️ Part of Arxis

**avila-math** is the foundation stone of [**Arxis**](https://github.com/avilaops/arxis) - the mathematical citadel.

**ARX** (fortress) + **AXIS** (engine) = **ARXIS**

Built with ❤️ by [Avila](https://avila.cloud)

**26 tests passing** ✅

## License

MIT - See LICENSE for details
