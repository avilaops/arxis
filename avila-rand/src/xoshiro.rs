//! Xoshiro256** - Fast, high-quality non-cryptographic PRNG
//!
//! Xoshiro256** is a fast all-purpose PRNG with excellent statistical quality.
//! Not suitable for cryptographic purposes but ideal for simulations and games.

use crate::traits::{Rng, SeedableRng};

/// Xoshiro256** random number generator
#[derive(Clone)]
pub struct Xoshiro256StarStar {
    s: [u64; 4],
}

impl Xoshiro256StarStar {
    /// Create a new Xoshiro256** from a seed
    pub fn new(seed: [u8; 32]) -> Self {
        let mut s = [0u64; 4];
        for (i, chunk) in seed.chunks_exact(8).enumerate() {
            s[i] = u64::from_le_bytes([
                chunk[0], chunk[1], chunk[2], chunk[3],
                chunk[4], chunk[5], chunk[6], chunk[7],
            ]);
        }
        
        // Ensure state is not all zeros
        if s == [0, 0, 0, 0] {
            s[0] = 1;
        }
        
        Self { s }
    }

    /// Jump function - equivalent to 2^128 calls to next()
    /// Useful for generating non-overlapping sequences
    pub fn jump(&mut self) {
        const JUMP: [u64; 4] = [
            0x180ec6d33cfd0aba,
            0xd5a61266f0c9392c,
            0xa9582618e03fc9aa,
            0x39abdc4529b1661c,
        ];

        let mut s0 = 0;
        let mut s1 = 0;
        let mut s2 = 0;
        let mut s3 = 0;

        for j in &JUMP {
            for b in 0..64 {
                if (j & (1u64 << b)) != 0 {
                    s0 ^= self.s[0];
                    s1 ^= self.s[1];
                    s2 ^= self.s[2];
                    s3 ^= self.s[3];
                }
                self.next_u64();
            }
        }

        self.s[0] = s0;
        self.s[1] = s1;
        self.s[2] = s2;
        self.s[3] = s3;
    }

    /// Long jump function - equivalent to 2^192 calls to next()
    pub fn long_jump(&mut self) {
        const LONG_JUMP: [u64; 4] = [
            0x76e15d3efefdcbbf,
            0xc5004e441c522fb3,
            0x77710069854ee241,
            0x39109bb02acbe635,
        ];

        let mut s0 = 0;
        let mut s1 = 0;
        let mut s2 = 0;
        let mut s3 = 0;

        for j in &LONG_JUMP {
            for b in 0..64 {
                if (j & (1u64 << b)) != 0 {
                    s0 ^= self.s[0];
                    s1 ^= self.s[1];
                    s2 ^= self.s[2];
                    s3 ^= self.s[3];
                }
                self.next_u64();
            }
        }

        self.s[0] = s0;
        self.s[1] = s1;
        self.s[2] = s2;
        self.s[3] = s3;
    }
}

impl Rng for Xoshiro256StarStar {
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    fn next_u64(&mut self) -> u64 {
        let result = self.s[1].wrapping_mul(5).rotate_left(7).wrapping_mul(9);
        let t = self.s[1] << 17;

        self.s[2] ^= self.s[0];
        self.s[3] ^= self.s[1];
        self.s[1] ^= self.s[2];
        self.s[0] ^= self.s[3];

        self.s[2] ^= t;
        self.s[3] = self.s[3].rotate_left(45);

        result
    }
}

impl SeedableRng for Xoshiro256StarStar {
    type Seed = [u8; 32];

    fn from_seed(seed: Self::Seed) -> Self {
        Self::new(seed)
    }
}

impl Default for Xoshiro256StarStar {
    fn default() -> Self {
        Self::seed_from_u64(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xoshiro_basic() {
        let seed = [1u8; 32];
        let mut rng = Xoshiro256StarStar::new(seed);
        let _val = rng.next_u64();
    }

    #[test]
    fn test_xoshiro_deterministic() {
        let seed = [42u8; 32];
        let mut rng1 = Xoshiro256StarStar::new(seed);
        let mut rng2 = Xoshiro256StarStar::new(seed);

        for _ in 0..100 {
            assert_eq!(rng1.next_u64(), rng2.next_u64());
        }
    }

    #[test]
    fn test_xoshiro_jump() {
        let seed = [7u8; 32];
        let mut rng1 = Xoshiro256StarStar::new(seed);
        let mut rng2 = Xoshiro256StarStar::new(seed);

        // Generate 2^128 values in one go
        rng1.jump();

        // Generate them manually (just a few for testing)
        for _ in 0..1000 {
            rng2.next_u64();
        }

        // They should be different
        assert_ne!(rng1.next_u64(), rng2.next_u64());
    }

    #[test]
    fn test_xoshiro_zero_seed_handling() {
        let seed = [0u8; 32];
        let mut rng = Xoshiro256StarStar::new(seed);
        
        // State should not be all zeros (it was corrected)
        assert_ne!(rng.s, [0, 0, 0, 0]);
        
        // Should be able to generate values
        let mut has_nonzero = false;
        for _ in 0..10 {
            if rng.next_u64() != 0 {
                has_nonzero = true;
                break;
            }
        }
        assert!(has_nonzero);
    }
}
