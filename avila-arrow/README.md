# avila-arrow

[![Crates.io](https://img.shields.io/crates/v/avila-arrow.svg)](https://crates.io/crates/avila-arrow)
[![Documentation](https://docs.rs/avila-arrow/badge.svg)](https://docs.rs/avila-arrow)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)

Zero-copy columnar format with scientific extensions for high-performance computing.

## 🚀 Features

- **11 Primitive Arrays**: Int8-64, UInt8-64, Float32/64, Boolean, UTF-8
- **4 Scientific Arrays**: Quaternions, Complex64, Tensor4D, Spinors
- **25+ Compute Operations**: Aggregations, filters, comparisons, sorting, arithmetic (SIMD)
- **SIMD Acceleration**: AVX2-optimized operations up to **35x faster**
- **Zero Dependencies**: Only `byteorder` required
- **AvilaDB Native**: Direct integration with AvilaDB
- **Production Ready**: 74 tests passing, proven benchmarks

## 🎯 Unique in the World

Avila-arrow is the **only columnar format** with native support for:

- **QuaternionArray**: SLERP interpolation for aerospace rotations
- **ComplexArray**: FFT-ready signal processing
- **Tensor4DArray**: Relativistic spacetime metrics
- **SpinorArray**: Quantum mechanics states

## 📦 Installation

```toml
[dependencies]
avila-arrow = "0.1"
```

## 🔥 Quick Start

```rust
use avila_arrow::{Schema, Field, DataType, RecordBatch};
use avila_arrow::array::Int64Array;
use avila_arrow::compute::*;

// Create schema
let schema = Schema::new(vec![
    Field::new("id", DataType::Int64),
    Field::new("value", DataType::Float64),
]);

// Create arrays
let ids = Int64Array::from(vec![1, 2, 3, 4, 5]);
let values = Float64Array::from(vec![10.0, 20.0, 30.0, 40.0, 50.0]);

// Compute operations
let sum = sum_f64(&values);
let mean = mean_f64(&values).unwrap();
let filtered = filter_f64(&values, &gt_f64(&values, 25.0))?;

println!("Sum: {}, Mean: {}, Filtered: {:?}", sum, mean, filtered.values());
```

## 🧪 Scientific Computing

```rust
use avila_arrow::scientific::*;

// Quaternion arrays for spacecraft orientation
let q1 = Quaternion::from_axis_angle([0.0, 0.0, 1.0], PI / 2.0);
let q2 = Quaternion::from_axis_angle([0.0, 0.0, 1.0], PI);
let array1 = QuaternionArray::new(vec![q1; 1000]);
let array2 = QuaternionArray::new(vec![q2; 1000]);

// SLERP interpolation for smooth rotation
let interpolated = array1.slerp(&array2, 0.5).unwrap();

// Complex arrays for FFT
let signal = ComplexArray::new(vec![
    Complex64::new(1.0, 0.0),
    Complex64::new(0.0, 1.0),
]);
let magnitudes = signal.magnitude();
let phases = signal.phase();
```

## ⚡ SIMD Performance

Avila-arrow uses **AVX2 intrinsics** for hardware-accelerated operations with proven speedups:

```rust
use avila_arrow::compute::*;

let data = Float64Array::from((0..1_000_000).map(|i| i as f64).collect::<Vec<_>>());

// Automatically uses SIMD when AVX2 is available
let sum = sum_f64(&data);  // 4.24x faster than scalar
```

## 📊 Benchmarks (100K-1M elements)

**Basic Operations:**
| Operation | Size | Scalar | SIMD | Speedup |
|-----------|------|--------|------|---------|
| Sum | 100K | 61.4μs | 14.5μs | **4.24x** |
| Add | 10K | 38.8μs | 4.4μs | **8.81x** |
| Multiply | 100 | 856ns | 24.4ns | **35x** |
| Subtract | 1K | 4.64μs | 611ns | **7.59x** |
| Divide | 10K | 76.3μs | 34.7μs | **2.20x** |
| Sqrt | 1M | 8.67ms | 4.98ms | **1.74x** |
| FMA | 10K | 54.9μs | 9.43μs | **5.82x** |

**Complex Pipelines (3 operations):**
| Size | Scalar | SIMD | Speedup |
|------|--------|------|---------|
| 10K | 99.3μs | 24.7μs | **4.02x** |
| 100K | 1.03ms | 586μs | **1.75x** |
| 1M | 12.0ms | 10.8ms | **1.11x** |

**Memory Throughput:**
| Elements | Scalar | SIMD | Speedup |
|----------|--------|------|---------|
| 100K | 61.4μs | 14.5μs | **4.24x** |
| 1M | 721μs | 292μs | **2.47x** |

> **Note**: Benchmarks run on Intel AVX2 CPU. SIMD excels at small-medium datasets (100-100K). For 10M+ elements, consider parallel processing.

## 🎓 Examples

See `examples/` directory:
- `basic.rs` - Arrays and RecordBatch
- `scientific.rs` - Quaternions, Complex, Tensors
- `compute.rs` - Data analysis operations
- `ipc.rs` - Serialization (coming soon)

Run with:
```bash
cargo run --example compute
```

## 🧬 Use Cases

- **Aerospace**: Spacecraft orientation tracking with quaternions
- **Signal Processing**: FFT analysis with complex arrays
- **Physics**: Relativistic simulations with tensors
- **Quantum Computing**: State vectors with spinors
- **Data Analytics**: High-performance columnar analytics

## 🛠️ Features

```toml
[dependencies.avila-arrow]
version = "0.1"
features = ["scientific", "compression", "ipc"]
```

- `scientific` (default): Scientific array types
- `compression`: Compression support
- `ipc`: Arrow IPC format
- `aviladb`: AvilaDB integration

## 📈 Roadmap

- [x] Primitive arrays (Int8-64, UInt8-64, Float32/64)
- [x] Scientific arrays (Quaternion, Complex, Tensor4D, Spinor)
- [x] Compute kernels (sum, mean, filter, sort, arithmetic)
- [x] SIMD acceleration (AVX2 with sub, div, sqrt, fma)
- [x] Comprehensive benchmarks (35x speedup proven)
- [ ] Arrow IPC format compatibility
- [ ] GPU acceleration (CUDA/ROCm)
- [ ] Distributed computing support
- [ ] AVX-512 support for next-gen CPUs

## 🤝 Contributing

Contributions welcome! Please open an issue or PR.

## 📄 License

Dual licensed under MIT OR Apache-2.0.

## 🌟 Credits

Built with ❤️ by [avilaops](https://github.com/avilaops) for the Brazilian scientific computing community.

---

**Status**: v0.1.3 - 74 tests passing ✅ | Benchmarks validated ✅
