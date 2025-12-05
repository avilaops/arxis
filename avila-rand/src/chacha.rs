//! ChaCha20-based PRNG - Cryptographically secure random number generator
//!
//! Based on the ChaCha20 stream cipher by D. J. Bernstein.
//! This is a cryptographically secure PRNG suitable for security-critical applications.

use crate::traits::{CryptoRng, Rng, SeedableRng};

/// ChaCha20-based random number generator
#[derive(Clone)]
pub struct ChaCha20Rng {
    state: [u32; 16],
    buffer: [u32; 16],
    index: usize,
}

impl ChaCha20Rng {
    /// Constants for ChaCha20
    const CONSTANTS: [u32; 4] = [0x61707865, 0x3320646e, 0x79622d32, 0x6b206574];

    /// Create a new ChaCha20 RNG from a 32-byte seed
    pub fn new(seed: [u8; 32]) -> Self {
        let mut key = [0u32; 8];
        for (i, chunk) in seed.chunks_exact(4).enumerate() {
            key[i] = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
        }

        let mut state = [0u32; 16];
        state[0..4].copy_from_slice(&Self::CONSTANTS);
        state[4..12].copy_from_slice(&key);
        state[12] = 0; // counter
        state[13] = 0;
        state[14] = 0; // nonce
        state[15] = 0;

        let mut rng = Self {
            state,
            buffer: [0u32; 16],
            index: 16, // Force refill on first use
        };
        rng.refill();
        rng
    }

    /// Quarter round operation (operates on indices)
    #[inline(always)]
    fn quarter_round(state: &mut [u32; 16], a: usize, b: usize, c: usize, d: usize) {
        state[a] = state[a].wrapping_add(state[b]);
        state[d] ^= state[a];
        state[d] = state[d].rotate_left(16);

        state[c] = state[c].wrapping_add(state[d]);
        state[b] ^= state[c];
        state[b] = state[b].rotate_left(12);

        state[a] = state[a].wrapping_add(state[b]);
        state[d] ^= state[a];
        state[d] = state[d].rotate_left(8);

        state[c] = state[c].wrapping_add(state[d]);
        state[b] ^= state[c];
        state[b] = state[b].rotate_left(7);
    }

    /// ChaCha20 block function (20 rounds)
    fn chacha_block(state: &[u32; 16]) -> [u32; 16] {
        let mut working = *state;

        // 10 double rounds = 20 rounds
        for _ in 0..10 {
            // Column rounds
            Self::quarter_round(&mut working, 0, 4, 8, 12);
            Self::quarter_round(&mut working, 1, 5, 9, 13);
            Self::quarter_round(&mut working, 2, 6, 10, 14);
            Self::quarter_round(&mut working, 3, 7, 11, 15);

            // Diagonal rounds
            Self::quarter_round(&mut working, 0, 5, 10, 15);
            Self::quarter_round(&mut working, 1, 6, 11, 12);
            Self::quarter_round(&mut working, 2, 7, 8, 13);
            Self::quarter_round(&mut working, 3, 4, 9, 14);
        }

        // Add original state
        for i in 0..16 {
            working[i] = working[i].wrapping_add(state[i]);
        }

        working
    }

    /// Refill the internal buffer
    fn refill(&mut self) {
        self.buffer = Self::chacha_block(&self.state);
        self.index = 0;

        // Increment counter
        self.state[12] = self.state[12].wrapping_add(1);
        if self.state[12] == 0 {
            self.state[13] = self.state[13].wrapping_add(1);
        }
    }

    /// Set the stream position (for testing/reproducibility)
    pub fn set_counter(&mut self, counter: u64) {
        self.state[12] = counter as u32;
        self.state[13] = (counter >> 32) as u32;
        self.refill();
    }
}

impl Rng for ChaCha20Rng {
    fn next_u32(&mut self) -> u32 {
        if self.index >= 16 {
            self.refill();
        }
        let value = self.buffer[self.index];
        self.index += 1;
        value
    }

    fn next_u64(&mut self) -> u64 {
        let hi = self.next_u32() as u64;
        let lo = self.next_u32() as u64;
        (hi << 32) | lo
    }
}

impl CryptoRng for ChaCha20Rng {}

impl SeedableRng for ChaCha20Rng {
    type Seed = [u8; 32];

    fn from_seed(seed: Self::Seed) -> Self {
        Self::new(seed)
    }
}

impl Default for ChaCha20Rng {
    fn default() -> Self {
        Self::seed_from_u64(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chacha20_basic() {
        let seed = [0u8; 32];
        let mut rng = ChaCha20Rng::new(seed);
        let _val = rng.next_u32();
    }

    #[test]
    fn test_chacha20_deterministic() {
        let seed = [42u8; 32];
        let mut rng1 = ChaCha20Rng::new(seed);
        let mut rng2 = ChaCha20Rng::new(seed);

        for _ in 0..100 {
            assert_eq!(rng1.next_u64(), rng2.next_u64());
        }
    }

    #[test]
    fn test_chacha20_different_seeds() {
        let seed1 = [1u8; 32];
        let seed2 = [2u8; 32];
        let mut rng1 = ChaCha20Rng::new(seed1);
        let mut rng2 = ChaCha20Rng::new(seed2);

        let val1 = rng1.next_u64();
        let val2 = rng2.next_u64();
        assert_ne!(val1, val2);
    }
}
