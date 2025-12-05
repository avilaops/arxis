//! Xorshift family of RNGs
//!
//! Xorshift generators are extremely fast and simple, using only XOR and shift operations.
//! They have good statistical properties for non-cryptographic applications.
//!
//! References:
//! - Marsaglia, G. (2003). Xorshift RNGs. Journal of Statistical Software.
//! - Vigna, S. (2016). An experimental exploration of Marsaglia's xorshift generators.

use crate::traits::FastRng;

/// Xorshift 64-bit state RNG
#[derive(Debug, Clone)]
pub struct Xorshift64 {
    state: u64,
}

impl Xorshift64 {
    /// Create new Xorshift64 with seed
    /// 
    /// Note: seed must not be zero
    #[inline]
    pub fn new(seed: u64) -> Self {
        let state = if seed == 0 { 0xBAD5EED } else { seed };
        Self { state }
    }
}

impl FastRng for Xorshift64 {
    #[inline]
    fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }
}

/// Xorshift128+ - High-quality 128-bit state RNG
#[derive(Debug, Clone)]
pub struct Xorshift128Plus {
    state: [u64; 2],
}

impl Xorshift128Plus {
    /// Create new Xorshift128+ with seed
    #[inline]
    pub fn new(seed: u64) -> Self {
        // Use splitmix64 to generate initial state from seed
        let mut s = seed;
        let mut gen_state = || {
            s = s.wrapping_add(0x9E3779B97F4A7C15);
            let mut z = s;
            z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
            z ^ (z >> 31)
        };
        
        let state = [gen_state(), gen_state()];
        Self { state }
    }

    /// Create new Xorshift128+ with 128-bit seed
    #[inline]
    pub fn new_u128(seed: u128) -> Self {
        let s0 = seed as u64;
        let s1 = (seed >> 64) as u64;
        Self { state: [s0, s1] }
    }
}

impl FastRng for Xorshift128Plus {
    #[inline]
    fn next_u64(&mut self) -> u64 {
        let mut s1 = self.state[0];
        let s0 = self.state[1];
        let result = s0.wrapping_add(s1);
        
        self.state[0] = s0;
        s1 ^= s1 << 23;
        self.state[1] = s1 ^ s0 ^ (s1 >> 18) ^ (s0 >> 5);
        
        result
    }
}

/// Xorshift128** - Alternative high-quality 128-bit state RNG
#[derive(Debug, Clone)]
pub struct Xorshift128StarStar {
    state: [u64; 2],
}

impl Xorshift128StarStar {
    /// Create new Xorshift128** with seed
    #[inline]
    pub fn new(seed: u64) -> Self {
        // Use splitmix64 to generate initial state from seed
        let mut s = seed;
        let mut gen_state = || {
            s = s.wrapping_add(0x9E3779B97F4A7C15);
            let mut z = s;
            z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
            z ^ (z >> 31)
        };
        
        let state = [gen_state(), gen_state()];
        Self { state }
    }

    /// Create new Xorshift128** with 128-bit seed
    #[inline]
    pub fn new_u128(seed: u128) -> Self {
        let s0 = seed as u64;
        let s1 = (seed >> 64) as u64;
        Self { state: [s0, s1] }
    }
}

impl FastRng for Xorshift128StarStar {
    #[inline]
    fn next_u64(&mut self) -> u64 {
        let mut s1 = self.state[0];
        let s0 = self.state[1];
        
        self.state[0] = s0;
        s1 ^= s1 << 23;
        self.state[1] = s1 ^ s0 ^ (s1 >> 18) ^ (s0 >> 5);
        
        // Star-star scrambler
        let result = s0.wrapping_mul(5).rotate_left(7).wrapping_mul(9);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xorshift64_creation() {
        let _rng = Xorshift64::new(12345);
    }

    #[test]
    fn test_xorshift64_zero_seed() {
        let mut rng = Xorshift64::new(0);
        let v = rng.next_u64();
        assert_ne!(v, 0);
    }

    #[test]
    fn test_xorshift64_output() {
        let mut rng = Xorshift64::new(42);
        let v1 = rng.next_u64();
        let v2 = rng.next_u64();
        assert_ne!(v1, v2);
    }

    #[test]
    fn test_xorshift128plus_creation() {
        let _rng = Xorshift128Plus::new(12345);
    }

    #[test]
    fn test_xorshift128plus_output() {
        let mut rng = Xorshift128Plus::new(42);
        let v1 = rng.next_u64();
        let v2 = rng.next_u64();
        assert_ne!(v1, v2);
    }

    #[test]
    fn test_xorshift128starstar_creation() {
        let _rng = Xorshift128StarStar::new(12345);
    }

    #[test]
    fn test_xorshift128starstar_output() {
        let mut rng = Xorshift128StarStar::new(42);
        let v1 = rng.next_u64();
        let v2 = rng.next_u64();
        assert_ne!(v1, v2);
    }

    #[test]
    fn test_xorshift64_reproducibility() {
        let mut rng1 = Xorshift64::new(999);
        let mut rng2 = Xorshift64::new(999);
        
        for _ in 0..1000 {
            assert_eq!(rng1.next_u64(), rng2.next_u64());
        }
    }

    #[test]
    fn test_xorshift128plus_reproducibility() {
        let mut rng1 = Xorshift128Plus::new(999);
        let mut rng2 = Xorshift128Plus::new(999);
        
        for _ in 0..1000 {
            assert_eq!(rng1.next_u64(), rng2.next_u64());
        }
    }

    #[test]
    fn test_xorshift128starstar_reproducibility() {
        let mut rng1 = Xorshift128StarStar::new(999);
        let mut rng2 = Xorshift128StarStar::new(999);
        
        for _ in 0..1000 {
            assert_eq!(rng1.next_u64(), rng2.next_u64());
        }
    }

    #[test]
    fn test_xorshift128plus_u128_seed() {
        let seed: u128 = 0x123456789ABCDEF0_FEDCBA9876543210;
        let mut rng = Xorshift128Plus::new_u128(seed);
        let v = rng.next_u64();
        assert_ne!(v, 0);
    }

    #[test]
    fn test_xorshift128starstar_u128_seed() {
        let seed: u128 = 0x123456789ABCDEF0_FEDCBA9876543210;
        let mut rng = Xorshift128StarStar::new_u128(seed);
        let v = rng.next_u64();
        assert_ne!(v, 0);
    }
}
