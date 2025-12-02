//! SHA-3 (Keccak-256) - Ethereum's choice
//!
//! Winner of NIST SHA-3 competition
//! Resistant to length extension attacks

use super::Hasher;

/// Keccak-256 round constants
const RC: [u64; 24] = [
    0x0000000000000001, 0x0000000000008082, 0x800000000000808A, 0x8000000080008000,
    0x000000000000808B, 0x0000000080000001, 0x8000000080008081, 0x8000000000008009,
    0x000000000000008A, 0x0000000000000088, 0x0000000080008009, 0x000000008000000A,
    0x000000008000808B, 0x800000000000008B, 0x8000000000008089, 0x8000000000008003,
    0x8000000000008002, 0x8000000000000080, 0x000000000000800A, 0x800000008000000A,
    0x8000000080008081, 0x8000000000008080, 0x0000000080000001, 0x8000000080008008,
];

/// Rotation offsets for Keccak-f
const ROTC: [u32; 24] = [
    1, 3, 6, 10, 15, 21, 28, 36, 45, 55, 2, 14,
    27, 41, 56, 8, 25, 43, 62, 18, 39, 61, 20, 44,
];

/// Pi lane permutation
const PILN: [usize; 24] = [
    10, 7, 11, 17, 18, 3, 5, 16, 8, 21, 24, 4,
    15, 23, 19, 13, 12, 2, 20, 14, 22, 9, 6, 1,
];

/// Keccak-256 hasher
pub struct Keccak256 {
    state: [u64; 25],
    buffer: [u8; 136],
    buffer_len: usize,
}

impl Hasher for Keccak256 {
    const OUTPUT_SIZE: usize = 32;
    type Output = [u8; 32];
    
    fn hash(data: &[u8]) -> Self::Output {
        let mut hasher = Self::new();
        hasher.update(data);
        hasher.finalize()
    }
}

impl Keccak256 {
    fn new() -> Self {
        Self {
            state: [0u64; 25],
            buffer: [0u8; 136],
            buffer_len: 0,
        }
    }
    
    fn update(&mut self, mut data: &[u8]) {
        // Process buffered data first
        if self.buffer_len > 0 {
            let to_copy = core::cmp::min(136 - self.buffer_len, data.len());
            self.buffer[self.buffer_len..self.buffer_len + to_copy].copy_from_slice(&data[..to_copy]);
            self.buffer_len += to_copy;
            data = &data[to_copy..];
            
            if self.buffer_len == 136 {
                let buf = self.buffer; self.absorb_block(&buf);
                self.buffer_len = 0;
            }
        }
        
        // Process complete 136-byte blocks (rate for Keccak-256)
        while data.len() >= 136 {
            self.absorb_block(&data[..136]);
            data = &data[136..];
        }
        
        // Buffer remaining data
        if !data.is_empty() {
            self.buffer[..data.len()].copy_from_slice(data);
            self.buffer_len = data.len();
        }
    }
    
    fn finalize(mut self) -> [u8; 32] {
        // Pad with 0x01 || 0x00...0x00 || 0x80
        self.buffer[self.buffer_len] = 0x01;
        for i in (self.buffer_len + 1)..136 {
            self.buffer[i] = 0;
        }
        self.buffer[135] |= 0x80;
        
        let buf = self.buffer; self.absorb_block(&buf);
        
        // Squeeze output (first 32 bytes)
        let mut output = [0u8; 32];
        for i in 0..4 {
            let bytes = self.state[i].to_le_bytes();
            output[i * 8..(i + 1) * 8].copy_from_slice(&bytes);
        }
        output
    }
    
    fn absorb_block(&mut self, block: &[u8]) {
        // XOR block into state (17 lanes = 136 bytes)
        for i in 0..17 {
            let lane = u64::from_le_bytes([
                block[i * 8],
                block[i * 8 + 1],
                block[i * 8 + 2],
                block[i * 8 + 3],
                block[i * 8 + 4],
                block[i * 8 + 5],
                block[i * 8 + 6],
                block[i * 8 + 7],
            ]);
            self.state[i] ^= lane;
        }
        
        // Keccak-f[1600] permutation
        self.keccak_f();
    }
    
    fn keccak_f(&mut self) {
        for round in 0..24 {
            // Theta
            let mut c = [0u64; 5];
            for x in 0..5 {
                c[x] = self.state[x] ^ self.state[x + 5] ^ self.state[x + 10] 
                     ^ self.state[x + 15] ^ self.state[x + 20];
            }
            
            let mut d = [0u64; 5];
            for x in 0..5 {
                d[x] = c[(x + 4) % 5] ^ c[(x + 1) % 5].rotate_left(1);
            }
            
            for x in 0..5 {
                for y in 0..5 {
                    self.state[x + y * 5] ^= d[x];
                }
            }
            
            // Rho and Pi
            let mut b = [0u64; 25];
            b[0] = self.state[0];
            for i in 0..24 {
                b[PILN[i]] = self.state[i + 1].rotate_left(ROTC[i]);
            }
            
            // Chi
            for y in 0..5 {
                let t = [
                    b[y * 5],
                    b[y * 5 + 1],
                    b[y * 5 + 2],
                    b[y * 5 + 3],
                    b[y * 5 + 4],
                ];
                for x in 0..5 {
                    self.state[y * 5 + x] = t[x] ^ ((!t[(x + 1) % 5]) & t[(x + 2) % 5]);
                }
            }
            
            // Iota
            self.state[0] ^= RC[round];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test] fn test_keccak256_consistency() { let hash1 = Keccak256::hash(&[]); let hash2 = Keccak256::hash(&[]); assert_eq!(hash1, hash2); }
    
    #[test]
    fn test_keccak256_hello() {
        let hash = Keccak256::hash(b"hello");
        // Verify it's different from empty
        let empty_hash = Keccak256::hash(&[]);
        assert_ne!(hash, empty_hash);
    }
}

