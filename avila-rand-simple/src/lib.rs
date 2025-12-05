//! # Avila Rand Simple
//!
//! Fast non-cryptographic random number generators for general use.
//!
//! ## Features
//!
//! - **PCG (Permuted Congruential Generator)** - High-quality, fast RNG
//! - **Xorshift variants** - Ultra-fast, simple algorithms
//! - **Splitmix64** - High-quality 64-bit generator
//! - **`#![no_std]` compatible** - Works in embedded environments
//! - **SIMD optimizations** - Bulk generation with AVX2/AVX-512
//! - **Zero dependencies** - Except avila-primitives for types
//!
//! ## Performance Target
//!
//! All algorithms target <1ns per number generated on modern hardware.
//!
//! ## Example
//!
//! ```rust
//! use avila_rand_simple::{Pcg64, FastRng};
//!
//! let mut rng = Pcg64::new(12345);
//! let random_number = rng.next_u64();
//! let in_range = rng.gen_range(1, 100);
//! ```

#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]
#![warn(clippy::all)]

#[cfg(feature = "std")]
extern crate std;

mod pcg;
mod xorshift;
mod splitmix;
mod traits;
mod range;

#[cfg(feature = "simd")]
mod simd;

pub use pcg::{Pcg32, Pcg64};
pub use xorshift::{Xorshift64, Xorshift128Plus, Xorshift128StarStar};
pub use splitmix::Splitmix64;
pub use traits::FastRng;
pub use range::{gen_range_u64, gen_range_u32, gen_range_usize, gen_range_f64, gen_range_f32, shuffle};

#[cfg(feature = "simd")]
pub use simd::{fill_u64_simd, fill_u32_simd};

/// Common imports for convenience
pub mod prelude {
    pub use crate::{FastRng, Pcg64, Xorshift128Plus, Splitmix64};
    pub use crate::range::{gen_range_u64, gen_range_u32, gen_range_usize};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pcg64_basic() {
        let mut rng = Pcg64::new(42);
        let v1 = rng.next_u64();
        let v2 = rng.next_u64();
        assert_ne!(v1, v2);
    }

    #[test]
    fn test_xorshift64_basic() {
        let mut rng = Xorshift64::new(42);
        let v1 = rng.next_u64();
        let v2 = rng.next_u64();
        assert_ne!(v1, v2);
    }

    #[test]
    fn test_splitmix64_basic() {
        let mut rng = Splitmix64::new(42);
        let v1 = rng.next_u64();
        let v2 = rng.next_u64();
        assert_ne!(v1, v2);
    }

    #[test]
    fn test_reproducibility() {
        let mut rng1 = Pcg64::new(12345);
        let mut rng2 = Pcg64::new(12345);
        
        for _ in 0..100 {
            assert_eq!(rng1.next_u64(), rng2.next_u64());
        }
    }

    #[test]
    fn test_zero_seed() {
        let mut rng = Pcg64::new(0);
        let v = rng.next_u64();
        assert_ne!(v, 0); // Should produce non-zero output
    }
}
