//! ChaCha20 stream cipher
//!
//! Faster than AES in software
//! Constant-time (side-channel resistant)
//! No timing attacks possible

/// ChaCha20 cipher state
pub struct ChaCha20 {
    state: [u32; 16],
}

impl ChaCha20 {
    /// Create new ChaCha20 cipher
    /// key: 32-byte key
    /// nonce: 12-byte nonce
    /// counter: 32-bit block counter
    pub fn new(key: &[u8; 32], nonce: &[u8; 12], counter: u32) -> Self {
        let mut state = [0u32; 16];
        
        // Constants 'expand 32-byte k'
        state[0] = 0x61707865;
        state[1] = 0x3320646e;
        state[2] = 0x79622d32;
        state[3] = 0x6b206574;
        
        // Key (8 words = 32 bytes)
        for i in 0..8 {
            state[4 + i] = u32::from_le_bytes([
                key[i * 4],
                key[i * 4 + 1],
                key[i * 4 + 2],
                key[i * 4 + 3],
            ]);
        }
        
        // Counter
        state[12] = counter;
        
        // Nonce (3 words = 12 bytes)
        for i in 0..3 {
            state[13 + i] = u32::from_le_bytes([
                nonce[i * 4],
                nonce[i * 4 + 1],
                nonce[i * 4 + 2],
                nonce[i * 4 + 3],
            ]);
        }
        
        Self { state }
    }
    
    /// Encrypt/decrypt data (XOR with keystream)
    pub fn apply_keystream(&mut self, data: &mut [u8]) {
        let mut offset = 0;
        
        while offset < data.len() {
            let keystream = self.block();
            let to_xor = core::cmp::min(64, data.len() - offset);
            
            for i in 0..to_xor {
                data[offset + i] ^= keystream[i];
            }
            
            offset += to_xor;
            self.state[12] = self.state[12].wrapping_add(1);
        }
    }
    
    /// Generate one 64-byte keystream block
    fn block(&self) -> [u8; 64] {
        let mut working = self.state;
        
        // 20 rounds (10 column rounds + 10 diagonal rounds)
        for _ in 0..10 {
            // Column round
            Self::quarter_round(&mut working, 0, 4, 8, 12);
            Self::quarter_round(&mut working, 1, 5, 9, 13);
            Self::quarter_round(&mut working, 2, 6, 10, 14);
            Self::quarter_round(&mut working, 3, 7, 11, 15);
            
            // Diagonal round
            Self::quarter_round(&mut working, 0, 5, 10, 15);
            Self::quarter_round(&mut working, 1, 6, 11, 12);
            Self::quarter_round(&mut working, 2, 7, 8, 13);
            Self::quarter_round(&mut working, 3, 4, 9, 14);
        }
        
        // Add original state
        for i in 0..16 {
            working[i] = working[i].wrapping_add(self.state[i]);
        }
        
        // Serialize to bytes (little-endian)
        let mut output = [0u8; 64];
        for i in 0..16 {
            let bytes = working[i].to_le_bytes();
            output[i * 4] = bytes[0];
            output[i * 4 + 1] = bytes[1];
            output[i * 4 + 2] = bytes[2];
            output[i * 4 + 3] = bytes[3];
        }
        
        output
    }
    
    /// ChaCha20 quarter round
    #[inline]
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
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_chacha20_basic() {
        // Simple encryption/decryption test
        let key = [0u8; 32];
        let nonce = [0u8; 12];
        
        let plaintext = b"Hello, ChaCha20!";
        let mut encrypted = *plaintext;
        
        // Encrypt
        let mut cipher = ChaCha20::new(&key, &nonce, 0);
        cipher.apply_keystream(&mut encrypted);
        
        // Verify it's different
        assert_ne!(&encrypted[..], &plaintext[..]);
        
        // Decrypt
        let mut cipher2 = ChaCha20::new(&key, &nonce, 0);
        cipher2.apply_keystream(&mut encrypted);
        
        // Verify we get back original
        assert_eq!(&encrypted[..], &plaintext[..]);
    }
}
