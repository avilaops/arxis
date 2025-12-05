//! PCG (Permuted Congruential Generator) family of RNGs
//!
//! PCG is a family of high-quality, fast random number generators.
//! They offer excellent statistical properties and speed.
//!
//! References:
//! - https://www.pcg-random.org/
//! - O'Neill, M.E. (2014). PCG: A Family of Simple Fast Space-Efficient 
//!   Statistically Good Algorithms for Random Number Generation

use crate::traits::FastRng;

/// PCG-XSH-RR 64-bit state, 32-bit output
#[derive(Debug, Clone)]
pub struct Pcg32 {
    state: u64,
    inc: u64,
}

impl Pcg32 {
    /// Create new PCG32 with seed
    #[inline]
    pub fn new(seed: u64) -> Self {
        let mut rng = Self {
            state: 0,
            inc: (seed << 1) | 1, // Must be odd
        };
        // Warm up the generator
        rng.state = rng.state.wrapping_add(seed);
        let _ = rng.next_u32();
        rng
    }

    /// Generate next 32-bit random number
    #[inline]
    pub fn next_u32(&mut self) -> u32 {
        let oldstate = self.state;
        // Advance internal state
        self.state = oldstate
            .wrapping_mul(6364136223846793005u64)
            .wrapping_add(self.inc);
        
        // Calculate output function (XSH-RR)
        let xorshifted = (((oldstate >> 18) ^ oldstate) >> 27) as u32;
        let rot = (oldstate >> 59) as u32;
        xorshifted.rotate_right(rot)
    }
}

impl FastRng for Pcg32 {
    #[inline]
    fn next_u64(&mut self) -> u64 {
        let high = self.next_u32() as u64;
        let low = self.next_u32() as u64;
        (high << 32) | low
    }
    
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.next_u32()
    }
}

/// PCG-XSL-RR 128-bit state, 64-bit output
#[derive(Debug, Clone)]
pub struct Pcg64 {
    state: u128,
    inc: u128,
}

impl Pcg64 {
    /// Create new PCG64 with seed
    #[inline]
    pub fn new(seed: u64) -> Self {
        let seed_128 = (seed as u128) | ((seed as u128) << 64);
        let mut rng = Self {
            state: 0,
            inc: (seed_128 << 1) | 1, // Must be odd
        };
        // Warm up the generator
        rng.state = rng.state.wrapping_add(seed_128);
        let _ = rng.next_u64();
        rng
    }

    /// Create new PCG64 with 128-bit seed
    #[inline]
    pub fn new_u128(seed: u128) -> Self {
        let mut rng = Self {
            state: 0,
            inc: (seed << 1) | 1, // Must be odd
        };
        // Warm up the generator
        rng.state = rng.state.wrapping_add(seed);
        let _ = rng.next_u64();
        rng
    }
}

impl FastRng for Pcg64 {
    #[inline]
    fn next_u64(&mut self) -> u64 {
        let oldstate = self.state;
        // Advance internal state (LCG)
        self.state = oldstate
            .wrapping_mul(47026247687942121848144207491837523525u128)
            .wrapping_add(self.inc);
        
        // Calculate output function (XSL-RR)
        let xorshifted = ((oldstate >> 64) ^ oldstate) as u64;
        let rot = (oldstate >> 122) as u32;
        xorshifted.rotate_right(rot)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pcg32_creation() {
        let _rng = Pcg32::new(12345);
    }

    #[test]
    fn test_pcg32_output() {
        let mut rng = Pcg32::new(42);
        let v1 = rng.next_u32();
        let v2 = rng.next_u32();
        assert_ne!(v1, v2);
    }

    #[test]
    fn test_pcg64_creation() {
        let _rng = Pcg64::new(12345);
    }

    #[test]
    fn test_pcg64_output() {
        let mut rng = Pcg64::new(42);
        let v1 = rng.next_u64();
        let v2 = rng.next_u64();
        assert_ne!(v1, v2);
    }

    #[test]
    fn test_pcg64_reproducibility() {
        let mut rng1 = Pcg64::new(999);
        let mut rng2 = Pcg64::new(999);
        
        for _ in 0..1000 {
            assert_eq!(rng1.next_u64(), rng2.next_u64());
        }
    }

    #[test]
    fn test_pcg32_reproducibility() {
        let mut rng1 = Pcg32::new(999);
        let mut rng2 = Pcg32::new(999);
        
        for _ in 0..1000 {
            assert_eq!(rng1.next_u32(), rng2.next_u32());
        }
    }

    #[test]
    fn test_pcg64_distribution() {
        let mut rng = Pcg64::new(42);
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

    #[test]
    fn test_pcg64_u128_seed() {
        let seed: u128 = 0x123456789ABCDEF0_FEDCBA9876543210;
        let mut rng = Pcg64::new_u128(seed);
        let v = rng.next_u64();
        assert_ne!(v, 0);
    }
}
