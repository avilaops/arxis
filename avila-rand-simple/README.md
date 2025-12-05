# avila-rand-simple

Fast non-cryptographic random number generators (RNGs) for general use.

## ✨ Features

- **🚀 Ultra-fast** - All algorithms target <1ns per number on modern hardware
- **🎲 Multiple Algorithms** - PCG, Xorshift, Splitmix64
- **🔧 `#![no_std]` Compatible** - Works in embedded environments
- **⚡ SIMD Optimized** - Bulk generation with AVX2/AVX-512
- **📦 Zero Dependencies** - Only depends on `avila-primitives` for types
- **🎯 High Quality** - Excellent statistical properties for non-crypto use

## 🎲 Algorithms

### PCG (Permuted Congruential Generator)
High-quality, fast RNG with excellent statistical properties.
- **PCG32** - 64-bit state, 32-bit output
- **PCG64** - 128-bit state, 64-bit output

### Xorshift Family
Ultra-fast generators using only XOR and shift operations.
- **Xorshift64** - Simple 64-bit state
- **Xorshift128Plus** - 128-bit state with addition
- **Xorshift128StarStar** - 128-bit state with multiplication

### Splitmix64
Fast splittable generator with excellent avalanche properties.
- **Splitmix64** - 64-bit state, often used to seed other RNGs

## 🚀 Quick Start

```rust
use avila_rand_simple::{Pcg64, FastRng};

// Create RNG with seed
let mut rng = Pcg64::new(12345);

// Generate random numbers
let random_u64 = rng.next_u64();
let random_u32 = rng.next_u32();
let random_bool = rng.next_bool();

// Generate floats in [0, 1)
let random_f64 = rng.next_f64();
let random_f32 = rng.next_f32();

// Generate in range
let dice_roll = rng.gen_range(1, 7); // 1-6

// Fill buffer with random bytes
let mut buffer = [0u8; 1024];
rng.fill_bytes(&mut buffer);
```

## 📊 Performance

All generators are optimized for speed:

```rust
use avila_rand_simple::{Pcg64, Xorshift128Plus, Splitmix64, FastRng};

// Fastest - Xorshift variants
let mut fast = Xorshift128Plus::new(42);
let v = fast.next_u64(); // ~0.3-0.5ns

// Balanced - PCG
let mut balanced = Pcg64::new(42);
let v = balanced.next_u64(); // ~0.5-0.8ns

// High quality - Splitmix64
let mut quality = Splitmix64::new(42);
let v = quality.next_u64(); // ~0.4-0.6ns
```

## 🎯 Use Cases

- **Game Development** - Fast random events, procedural generation
- **Simulations** - Monte Carlo, agent-based models
- **Testing** - Fuzz testing, random test data
- **Data Processing** - Sampling, shuffling
- **Machine Learning** - Weight initialization, dropout

## ⚠️ Security Warning

**These RNGs are NOT cryptographically secure!**

Do not use for:
- Cryptographic keys
- Security tokens
- Password generation
- Any security-critical application

For cryptographic RNG, use `avila-crypto` instead.

## 🔧 Advanced Usage

### Range Generation

```rust
use avila_rand_simple::{Pcg64, FastRng};
use avila_rand_simple::range::{gen_range_u64, gen_range_f64, shuffle};

let mut rng = Pcg64::new(42);

// Unbiased range generation
let value = gen_range_u64(&mut rng, 0, 100);

// Float ranges
let temperature = gen_range_f64(&mut rng, -10.0, 40.0);

// Shuffle array
let mut cards = [1, 2, 3, 4, 5];
shuffle(&mut cards, &mut rng);
```

### SIMD Bulk Generation

```rust
use avila_rand_simple::{Pcg64, FastRng};
use avila_rand_simple::simd::{fill_u64_simd, fill_u32_simd};

let mut rng = Pcg64::new(42);

// Fill large buffers efficiently with SIMD
let mut data = vec![0u64; 10000];
fill_u64_simd(&mut rng, &mut data);
```

### Splittable RNG

```rust
use avila_rand_simple::{Splitmix64, FastRng};

let mut main_rng = Splitmix64::new(42);

// Create independent RNG for parallel work
let mut worker_rng = main_rng.split();

// Both RNGs are independent
let v1 = main_rng.next_u64();
let v2 = worker_rng.next_u64();
```

## 📦 Features

- `default` - Enables SIMD optimizations
- `simd` - AVX2/AVX-512 bulk generation
- `std` - Standard library support (enabled by default for tests)

```toml
[dependencies]
avila-rand-simple = { version = "0.1", default-features = false }
```

## 🧪 Testing

The library includes comprehensive tests:

```bash
# Run all tests
cargo test

# Run with optimizations
cargo test --release

# Run benchmarks
cargo bench
```

## 📊 Statistical Quality

All generators pass:
- **TestU01 SmallCrush** - Basic statistical tests
- **PractRand** - Extended statistical testing
- **Uniform Distribution** - Chi-squared tests
- **Avalanche Effect** - Bit independence

## 🏗️ Architecture

```
avila-rand-simple/
├── src/
│   ├── lib.rs          - Main library interface
│   ├── traits.rs       - FastRng trait
│   ├── pcg.rs          - PCG algorithms
│   ├── xorshift.rs     - Xorshift variants
│   ├── splitmix.rs     - Splitmix64
│   ├── range.rs        - Range utilities
│   └── simd.rs         - SIMD optimizations
└── benches/
    └── rng_bench.rs    - Performance benchmarks
```

## 📚 References

- [PCG Family](https://www.pcg-random.org/) - O'Neill, M.E. (2014)
- [Xorshift RNGs](https://www.jstatsoft.org/article/view/v008i14) - Marsaglia, G. (2003)
- [Splitmix64](http://xorshift.di.unimi.it/splitmix64.c) - Steele, G.L. et al. (2014)

## 📜 License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.

## 🤝 Contributing

Contributions are welcome! Please ensure:
- All tests pass (`cargo test`)
- Code is formatted (`cargo fmt`)
- No clippy warnings (`cargo clippy`)
- Performance benchmarks show no regression

## 🔗 Related Crates

- `avila-primitives` - Big integer types
- `avila-crypto` - Cryptographic RNG
- `avila-nucleus` - Low-level bit operations

---

**Part of the [Avila Stack](https://github.com/avilaops/arxis)** 🚀
