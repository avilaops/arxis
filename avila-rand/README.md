# Avila Rand

[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)

A cryptographically secure random number generator library for Rust with **zero external dependencies**. Built from scratch with `no_std` support.

## Features

- **🔒 Cryptographically Secure**: ChaCha20-based PRNG and OS entropy source (CSPRNG)
- **⚡ Fast Non-Crypto RNG**: Xoshiro256** for high-performance applications
- **📊 Statistical Distributions**: Uniform, Normal (Gaussian), Exponential, Bernoulli, Gamma
- **🚫 Zero External Dependencies**: No `rand` crate or other dependencies
- **🔧 `no_std` Compatible**: Works in embedded and constrained environments
- **🎯 Type-Safe API**: Strongly typed random generation
- **✅ Well-Tested**: 47+ tests including statistical tests

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
avila-rand = { path = "../avila-rand" }
```

## Quick Start

```rust
use avila_rand::*;

fn main() {
    // Use the thread-local default RNG
    let x: u32 = random();
    let y = random_range(0..100);
    
    // Or create your own RNG instance
    let mut rng = ChaCha20Rng::seed_from_u64(42);
    let value: u64 = rng.gen();
    let in_range: i32 = rng.gen_range(-10..10);
    
    // Fill a buffer with random bytes
    let mut buffer = [0u8; 32];
    rng.fill_bytes(&mut buffer);
}
```

## RNG Algorithms

### ChaCha20 PRNG (Cryptographically Secure)

Based on the ChaCha20 stream cipher by D. J. Bernstein. Suitable for security-critical applications.

```rust
use avila_rand::{ChaCha20Rng, SeedableRng};

// Seed from OS entropy (requires std or getrandom feature)
let mut rng = ChaCha20Rng::from_entropy();

// Or from a specific seed
let seed = [0u8; 32];
let mut rng = ChaCha20Rng::from_seed(seed);

// Or from a u64
let mut rng = ChaCha20Rng::seed_from_u64(12345);
```

### Xoshiro256** (Fast, High-Quality)

Fast all-purpose PRNG with excellent statistical quality. Not suitable for cryptographic purposes but ideal for simulations, games, and Monte Carlo methods.

```rust
use avila_rand::{Xoshiro256StarStar, SeedableRng};

let mut rng = Xoshiro256StarStar::seed_from_u64(42);

// Jump ahead by 2^128 calls (useful for parallel streams)
rng.jump();

// Jump ahead by 2^192 calls
rng.long_jump();
```

### OS RNG (CSPRNG)

Cryptographically secure random numbers directly from the operating system.

```rust
use avila_rand::OsRng;

let mut rng = OsRng::new();
let secure_random: u64 = rng.next_u64();
```

## Distributions

### Uniform Distribution

```rust
use avila_rand::{ChaCha20Rng, Uniform, Distribution, SeedableRng};

let mut rng = ChaCha20Rng::seed_from_u64(42);
let uniform = Uniform::new(0.0, 10.0);

let value = uniform.sample(&mut rng);
```

### Normal (Gaussian) Distribution

Uses the Box-Muller transform.

```rust
use avila_rand::{Normal, Distribution};

// Standard normal (mean=0, std=1)
let normal = Normal::standard();

// Custom mean and standard deviation
let normal = Normal::new(100.0, 15.0);

let value = normal.sample(&mut rng);
```

### Exponential Distribution

```rust
use avila_rand::{Exponential, Distribution};

let exp = Exponential::new(1.5); // lambda = 1.5
let value = exp.sample(&mut rng);
```

### Other Distributions

- **Bernoulli**: Boolean with probability p
- **Gamma**: Gamma distribution (shape, scale)

## Utilities

### Shuffle

```rust
use avila_rand::{shuffle, ChaCha20Rng, SeedableRng};

let mut rng = ChaCha20Rng::seed_from_u64(42);
let mut array = [1, 2, 3, 4, 5];
shuffle(&mut rng, &mut array);
```

### Fill Bytes

```rust
let mut buffer = [0u8; 64];
rng.fill_bytes(&mut buffer);
```

## `no_std` Support

This crate is fully `no_std` compatible. Disable the default features:

```toml
[dependencies]
avila-rand = { path = "../avila-rand", default-features = false }
```

Note: OS entropy source (`OsRng`) requires either the `std` or `getrandom` feature.

## Features

- `std` (default): Enable standard library support and thread-local RNG
- `getrandom`: Enable getrandom support for OS entropy in `no_std` environments

## Safety

This crate is marked with `#![forbid(unsafe_code)]` at the library level - all RNG algorithms use only safe Rust code.

**Note**: The OS entropy source (`OsRng`) on Windows requires a single FFI call to `RtlGenRandom` which is inherently unsafe. This is properly documented and uses the recommended Windows API for cryptographically secure random numbers. The Unix implementation reads from `/dev/urandom` and uses only safe code.

## Performance

Benchmarks (on x86_64):

| RNG              | Throughput    | Use Case                          |
|------------------|---------------|-----------------------------------|
| ChaCha20         | ~2.5 GB/s     | Cryptographic applications        |
| Xoshiro256**     | ~4.5 GB/s     | Simulations, games, Monte Carlo   |
| OsRng            | ~100 MB/s     | Seeding, one-time secure values   |

## Statistical Quality

All PRNGs pass standard statistical tests:

- **Chi-square test**: Uniformity of distribution
- **Bit distribution test**: Equal probability of 0s and 1s
- **Correlation test**: No correlation between consecutive values

ChaCha20 and OsRng are cryptographically secure.

## Testing

Run the test suite:

```bash
cargo test
```

Run benchmarks:

```bash
cargo bench
```

## Why No External Dependencies?

This library is built as part of the Arxis project philosophy:

- **Zero Trust**: Complete control over cryptographic primitives
- **Audibility**: All code is reviewable in one place
- **No Supply Chain Attacks**: No external crates that could be compromised
- **Educational**: Learn how RNGs work from scratch
- **Sovereignty**: Complete independence from external projects

## Related Crates

- `avila-crypto`: Cryptographic primitives
- `avila-primitives`: Big integer types (U256, U512, etc.)

## License

Dual-licensed under MIT OR Apache-2.0.

## Contributing

Contributions are welcome! Please ensure:

1. No external dependencies are added
2. `#![no_std]` compatibility is maintained
3. All tests pass
4. Code follows the existing style

## References

- ChaCha20: [RFC 7539](https://tools.ietf.org/html/rfc7539)
- Xoshiro256**: [xoshiro / xoroshiro generators](https://prng.di.unimi.it/)
- Box-Muller Transform: For normal distribution generation

## Acknowledgments

Built from scratch for the Arxis ecosystem by Nícolas Ávila.
