# avila-numeric

**Generic numeric traits for AVL Platform**

[![Crates.io](https://img.shields.io/crates/v/avila-numeric.svg)](https://crates.io/crates/avila-numeric)
[![Documentation](https://docs.rs/avila-numeric/badge.svg)](https://docs.rs/avila-numeric)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

## 🚀 Overview

`avila-numeric` provides generic numeric traits for mathematical operations across different numeric types in the AVL Platform ecosystem.

### Key Features

- ✅ **Generic numeric traits** (Zero, One, Num, Float, Integer)
- ✅ **No dependencies** - pure Rust implementation
- ✅ **no_std compatible** - works in embedded contexts
- ✅ **Type-safe** mathematical operations
- ✅ **Optimized for AVL Platform** computations

## 📦 Installation

```toml
[dependencies]
avila-numeric = "0.1"
```

## 🎯 Quick Start

```rust
use avila_numeric::{Zero, One, Num, Float};

fn add_one<T: Num>(x: T) -> T {
    x + T::one()
}

fn is_zero<T: Zero + PartialEq>(x: T) -> bool {
    x == T::zero()
}

fn sqrt_generic<T: Float>(x: T) -> T {
    x.sqrt()
}
```

## 📚 Traits

### Zero
```rust
pub trait Zero: Sized {
    fn zero() -> Self;
    fn is_zero(&self) -> bool;
}
```

### One
```rust
pub trait One: Sized {
    fn one() -> Self;
}
```

### Num
```rust
pub trait Num: Zero + One + PartialEq + Add + Sub + Mul + Div {
    // Generic numeric operations
}
```

### Float
```rust
pub trait Float: Num + Neg {
    fn nan() -> Self;
    fn infinity() -> Self;
    fn neg_infinity() -> Self;
    fn is_nan(self) -> bool;
    fn is_infinite(self) -> bool;
    fn sqrt(self) -> Self;
    fn abs(self) -> Self;
    fn floor(self) -> Self;
    fn ceil(self) -> Self;
    // ... more float operations
}
```

### Integer
```rust
pub trait Integer: Num + Ord {
    fn div_floor(&self, other: &Self) -> Self;
    fn mod_floor(&self, other: &Self) -> Self;
    fn gcd(&self, other: &Self) -> Self;
}
```

## 🎮 Use Cases

### Generic Algorithms
```rust
use avila_numeric::{Num, Float};

fn distance<T: Float>(x1: T, y1: T, x2: T, y2: T) -> T {
    let dx = x2 - x1;
    let dy = y2 - y1;
    (dx * dx + dy * dy).sqrt()
}

// Works with f32 or f64
let dist_f32 = distance(0.0f32, 0.0f32, 3.0f32, 4.0f32);
let dist_f64 = distance(0.0f64, 0.0f64, 3.0f64, 4.0f64);
```

### Statistical Functions
```rust
use avila_numeric::{Num, Zero};

fn sum<T: Num>(values: &[T]) -> T {
    values.iter().cloned().fold(T::zero(), |acc, x| acc + x)
}

fn mean<T: Num + From<usize>>(values: &[T]) -> T {
    let total = sum(values);
    total / T::from(values.len())
}
```

## 📖 Documentation

Full documentation at [docs.rs/avila-numeric](https://docs.rs/avila-numeric)

## 🤝 Contributing

Part of the [AVL Platform](https://avila.inc) - contributions welcome!

## 📄 License

MIT License - see [LICENSE](LICENSE) file for details

---

**Built with 🇧🇷 by Avila Development Team**
