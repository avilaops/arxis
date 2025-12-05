//! Splitmix64 RNG
//!
//! Splitmix64 is a fast splittable pseudorandom number generator.
//! It's often used to seed other PRNGs because of its excellent avalanche properties.
//!
//! References:
//! - Steele, G.L., Lea, D., Flood, C.H. (2014). Fast splittable pseudorandom number generators.
//! - http://xorshift.di.unimi.it/splitmix64.c

use crate::traits::FastRng;

/// Splitmix64 - 64-bit state RNG with excellent quality
#[derive(Debug, Clone)]
pub struct Splitmix64 {
    state: u64,
}

impl Splitmix64 {
    /// Create new Splitmix64 with seed
    #[inline]
    pub fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    /// Split the generator into a new independent generator
    #[inline]
    pub fn split(&mut self) -> Self {
        // Generate a new seed using the current state
        let new_seed = self.next_u64();
        Self::new(new_seed)
    }
}

impl FastRng for Splitmix64 {
    #[inline]
    fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_splitmix64_creation() {
        let _rng = Splitmix64::new(12345);
    }

    #[test]
    fn test_splitmix64_output() {
        let mut rng = Splitmix64::new(42);
        let v1 = rng.next_u64();
        let v2 = rng.next_u64();
        assert_ne!(v1, v2);
    }

    #[test]
    fn test_splitmix64_reproducibility() {
        let mut rng1 = Splitmix64::new(999);
        let mut rng2 = Splitmix64::new(999);
        
        for _ in 0..1000 {
            assert_eq!(rng1.next_u64(), rng2.next_u64());
        }
    }

    #[test]
    fn test_splitmix64_zero_seed() {
        let mut rng = Splitmix64::new(0);
        let v = rng.next_u64();
        // Splitmix64 with seed 0 should still produce output
        assert_ne!(v, 0);
    }

    #[test]
    fn test_splitmix64_split() {
        let mut rng = Splitmix64::new(42);
        let mut split_rng = rng.split();
        
        // The split RNG should produce different values
        let v1 = rng.next_u64();
        let v2 = split_rng.next_u64();
        assert_ne!(v1, v2);
    }

    #[test]
    fn test_splitmix64_avalanche() {
        // Test that similar seeds produce very different outputs
        let mut rng1 = Splitmix64::new(1000);
        let mut rng2 = Splitmix64::new(1001);
        
        let v1 = rng1.next_u64();
        let v2 = rng2.next_u64();
        
        // Count different bits
        let xor = v1 ^ v2;
        let different_bits = xor.count_ones();
        
        // Should differ in many bits (expect ~32 out of 64)
        assert!(different_bits > 20);
    }

    #[test]
    fn test_splitmix64_distribution() {
        let mut rng = Splitmix64::new(42);
        let mut buckets = [0u32; 10];
        
        for _ in 0..10000 {
            let val = rng.next_u64();
            let bucket = (val % 10) as usize;
            buckets[bucket] += 1;
        }
        
        // Each bucket should have roughly 1000 items (±30%)
        for count in buckets.iter() {
            assert!(*count > 700 && *count < 1300);
        }
    }
}
