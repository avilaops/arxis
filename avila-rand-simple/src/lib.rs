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
//! use avila_rand_simple::{Pcg64, FastRng, gen_range_u64};
//!
//! let mut rng = Pcg64::new(12345);
//! let random_number = rng.next_u64();
//! let in_range = gen_range_u64(&mut rng, 1, 100);
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

// Additional comprehensive tests
#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_all_rngs_produce_different_seeds() {
        let seed = 42;
        let mut pcg = Pcg64::new(seed);
        let mut xor = Xorshift128Plus::new(seed);
        let mut split = Splitmix64::new(seed);
        
        // All RNGs should produce different sequences
        let p1 = pcg.next_u64();
        let x1 = xor.next_u64();
        let s1 = split.next_u64();
        
        // Values should be different (very high probability)
        assert!(p1 != x1 || x1 != s1);
    }

    #[test]
    fn test_f64_in_bounds() {
        let mut rng = Pcg64::new(12345);
        for _ in 0..1000 {
            let f = rng.next_f64();
            assert!(f >= 0.0 && f < 1.0, "f64 {} out of bounds", f);
        }
    }

    #[test]
    fn test_f32_in_bounds() {
        let mut rng = Pcg64::new(12345);
        for _ in 0..1000 {
            let f = rng.next_f32();
            assert!(f >= 0.0 && f < 1.0, "f32 {} out of bounds", f);
        }
    }

    #[test]
    fn test_bool_distribution() {
        let mut rng = Pcg64::new(12345);
        let mut true_count = 0;
        let iterations = 10000;
        
        for _ in 0..iterations {
            if rng.next_bool() {
                true_count += 1;
            }
        }
        
        // Should be roughly 50/50 (within 10%)
        assert!(true_count > 4000 && true_count < 6000);
    }

    #[test]
    fn test_fill_bytes_zero_length() {
        let mut rng = Pcg64::new(42);
        let mut buf = [];
        rng.fill_bytes(&mut buf); // Should not panic
    }

    #[test]
    fn test_fill_bytes_odd_length() {
        let mut rng = Pcg64::new(42);
        let mut buf = [0u8; 13];
        rng.fill_bytes(&mut buf);
        
        // Should fill all bytes
        let all_zero = buf.iter().all(|&x| x == 0);
        assert!(!all_zero);
    }

    #[test]
    fn test_pcg32_u64_generation() {
        let mut rng = Pcg32::new(42);
        let v1 = rng.next_u64();
        let v2 = rng.next_u64();
        
        // PCG32 generates u64 from two u32 calls
        assert_ne!(v1, v2);
    }

    #[test]
    fn test_pcg64_statistics() {
        let mut rng = Pcg64::new(12345);
        let mut buckets = [0u32; 16];
        for _ in 0..16000 {
            let val = rng.next_u64();
            let bucket = (val % 16) as usize;
            buckets[bucket] += 1;
        }
        
        // Each bucket should have roughly 1000 items (±40%)
        for count in buckets.iter() {
            assert!(*count > 600 && *count < 1400);
        }
    }

    #[test]
    fn test_xorshift128plus_statistics() {
        let mut rng = Xorshift128Plus::new(12345);
        let mut buckets = [0u32; 16];
        for _ in 0..16000 {
            let val = rng.next_u64();
            let bucket = (val % 16) as usize;
            buckets[bucket] += 1;
        }
        
        // Each bucket should have roughly 1000 items (±40%)
        for count in buckets.iter() {
            assert!(*count > 600 && *count < 1400);
        }
    }

    #[test]
    fn test_splitmix64_statistics() {
        let mut rng = Splitmix64::new(12345);
        let mut buckets = [0u32; 16];
        for _ in 0..16000 {
            let val = rng.next_u64();
            let bucket = (val % 16) as usize;
            buckets[bucket] += 1;
        }
        
        // Each bucket should have roughly 1000 items (±40%)
        for count in buckets.iter() {
            assert!(*count > 600 && *count < 1400);
        }
    }
}
