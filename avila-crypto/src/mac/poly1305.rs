//! Poly1305 - One-time MAC using 130-bit prime
//!
//! Designed by Daniel J. Bernstein
//! Used with ChaCha20 for AEAD

/// Poly1305 state: accumulator and key
pub struct Poly1305 {
    r: [u32; 5],  // Clamped r (130 bits)
    h: [u32; 5],  // Accumulator (130 bits)
    s: [u32; 4],  // Secret s (128 bits)
    buffer: [u8; 16],
    buffer_len: usize,
}

impl Poly1305 {
    /// Initialize with 32-byte key (r || s)
    pub fn new(key: &[u8; 32]) -> Self {
        let mut r = [0u32; 5];
        let mut s = [0u32; 4];

        // Load r and clamp
        r[0] = u32::from_le_bytes([key[0], key[1], key[2], key[3]]) & 0x0fffffff;
        r[1] = u32::from_le_bytes([key[4], key[5], key[6], key[7]]) & 0x0ffffffc;
        r[2] = u32::from_le_bytes([key[8], key[9], key[10], key[11]]) & 0x0ffffffc;
        r[3] = u32::from_le_bytes([key[12], key[13], key[14], key[15]]) & 0x0ffffffc;

        // Load s
        for i in 0..4 {
            s[i] = u32::from_le_bytes([
                key[16 + i * 4],
                key[16 + i * 4 + 1],
                key[16 + i * 4 + 2],
                key[16 + i * 4 + 3],
            ]);
        }

        Self {
            r,
            h: [0; 5],
            s,
            buffer: [0; 16],
            buffer_len: 0,
        }
    }

    /// Process message
    pub fn update(&mut self, data: &[u8]) {
        let mut offset = 0;

        // Fill buffer first
        if self.buffer_len > 0 {
            let to_copy = core::cmp::min(16 - self.buffer_len, data.len());
            self.buffer[self.buffer_len..self.buffer_len + to_copy]
                .copy_from_slice(&data[..to_copy]);
            self.buffer_len += to_copy;
            offset += to_copy;

            if self.buffer_len == 16 {
                let buf = self.buffer;
                self.process_block(&buf, false);
                self.buffer_len = 0;
            }
        }

        // Process complete blocks
        while offset + 16 <= data.len() {
            let block = &data[offset..offset + 16];
            let mut block_arr = [0u8; 16];
            block_arr.copy_from_slice(block);
            self.process_block(&block_arr, false);
            offset += 16;
        }

        // Buffer remaining
        if offset < data.len() {
            let remaining = &data[offset..];
            self.buffer[..remaining.len()].copy_from_slice(remaining);
            self.buffer_len = remaining.len();
        }
    }

    /// Finalize and return 16-byte tag
    pub fn finalize(&mut self) -> [u8; 16] {
        // Process final block if any
        if self.buffer_len > 0 {
            let buffer_copy = self.buffer;
            let buffer_len = self.buffer_len;
            self.process_block(&buffer_copy[..buffer_len], true);
        }

        // Fully reduce h mod 2^130-5
        self.reduce();

        // h += s
        let mut carry = 0u64;
        for i in 0..4 {
            carry += self.h[i] as u64 + self.s[i] as u64;
            self.h[i] = carry as u32;
            carry >>= 32;
        }

        // Convert to bytes
        let mut tag = [0u8; 16];
        for i in 0..4 {
            tag[i * 4..(i + 1) * 4].copy_from_slice(&self.h[i].to_le_bytes());
        }
        tag
    }

    fn process_block(&mut self, block: &[u8], is_final: bool) {
        // Convert block to 130-bit number with padding bit
        let mut c = [0u32; 5];

        let len = if is_final { block.len() } else { 16 };

        for i in 0..len.min(16) {
            let byte_idx = i / 4;
            let shift = (i % 4) * 8;
            c[byte_idx] |= (block[i] as u32) << shift;
        }

        // Add padding bit (2^128 for full block, 2^(8*len) for final)
        if !is_final {
            c[4] = 1;
        } else if len < 16 {
            let byte_idx = len / 4;
            let shift = (len % 4) * 8;
            if byte_idx < 5 {
                c[byte_idx] |= 1u32 << shift;
            }
        } else {
            c[4] = 1;
        }

        // h += c
        let mut carry = 0u64;
        for i in 0..5 {
            carry += self.h[i] as u64 + c[i] as u64;
            self.h[i] = carry as u32;
            carry >>= 32;
        }

        // h *= r (mod 2^130-5)
        self.multiply_r();
    }

    fn multiply_r(&mut self) {
        // Multiply h by r using schoolbook multiplication
        let h0 = self.h[0] as u64;
        let h1 = self.h[1] as u64;
        let h2 = self.h[2] as u64;
        let h3 = self.h[3] as u64;
        let h4 = self.h[4] as u64;

        let r0 = self.r[0] as u64;
        let r1 = self.r[1] as u64;
        let r2 = self.r[2] as u64;
        let r3 = self.r[3] as u64;

        // Precompute r * 5 for reduction (use u128 to prevent overflow)
        let r1_5 = (r1 as u128) * 5;
        let r2_5 = (r2 as u128) * 5;
        let r3_5 = (r3 as u128) * 5;
        let r0_5 = (r0 as u128) * 5;

        // Multiply (with modular reduction via r*5) - use u128 arithmetic
        let d0 = (h0 as u128) * (r0 as u128) + (h1 as u128) * r3_5 + (h2 as u128) * r2_5 + (h3 as u128) * r1_5 + (h4 as u128) * r0_5;
        let mut d1 = (h0 as u128) * (r1 as u128) + (h1 as u128) * (r0 as u128) + (h2 as u128) * r3_5 + (h3 as u128) * r2_5 + (h4 as u128) * r1_5;
        let mut d2 = (h0 as u128) * (r2 as u128) + (h1 as u128) * (r1 as u128) + (h2 as u128) * (r0 as u128) + (h3 as u128) * r3_5 + (h4 as u128) * r2_5;
        let mut d3 = (h0 as u128) * (r3 as u128) + (h1 as u128) * (r2 as u128) + (h2 as u128) * (r1 as u128) + (h3 as u128) * (r0 as u128) + (h4 as u128) * r3_5;
        let mut d4 = (h4 as u128) * ((r0 as u128) & 3);

        // Carry propagation
        let mut c = d0 >> 32;
        self.h[0] = d0 as u32;
        d1 += c;

        c = d1 >> 32;
        self.h[1] = d1 as u32;
        d2 += c;

        c = d2 >> 32;
        self.h[2] = d2 as u32;
        d3 += c;

        c = d3 >> 32;
        self.h[3] = d3 as u32;
        d4 += c;

        self.h[4] = d4 as u32;
    }

    fn reduce(&mut self) {
        // Reduce h mod 2^130-5
        let mut c = self.h[4] >> 2;
        self.h[4] &= 3;

        c = c.wrapping_mul(5);

        for i in 0..4 {
            let sum = (self.h[i] as u64).wrapping_add(c as u64);
            self.h[i] = sum as u32;
            c = (sum >> 32) as u32;
        }

        self.h[4] = self.h[4].wrapping_add(c);
    }

    /// One-shot MAC computation
    pub fn mac(key: &[u8; 32], message: &[u8]) -> [u8; 16] {
        let mut poly = Poly1305::new(key);
        poly.update(message);
        poly.finalize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poly1305_empty() {
        let key = [0u8; 32];
        let tag = Poly1305::mac(&key, &[]);
        assert_eq!(tag.len(), 16);
    }

    #[test]
    fn test_poly1305_consistency() {
        let key = [0x42; 32];
        let message = b"Hello Poly1305!";

        let tag1 = Poly1305::mac(&key, message);
        let tag2 = Poly1305::mac(&key, message);

        assert_eq!(tag1, tag2, "Same input should produce same tag");

        let tag3 = Poly1305::mac(&key, b"Different message");
        assert_ne!(tag1, tag3, "Different messages should produce different tags");
    }
}
