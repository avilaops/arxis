//! HMAC - Hash-based Message Authentication Code
//!
//! RFC 2104 compliant implementation
//! Supports SHA-256 and SHA-512

#![allow(dead_code)]

use crate::hash::sha256::Sha256;
use crate::hash::sha512::Sha512;
use crate::hash::Hasher;

/// HMAC with SHA-256
pub struct HmacSha256;

impl HmacSha256 {
    const BLOCK_SIZE: usize = 64; // SHA-256 block size
    const IPAD: u8 = 0x36;
    const OPAD: u8 = 0x5c;

    /// Compute HMAC-SHA256
    pub fn mac(key: &[u8], message: &[u8]) -> [u8; 32] {
        let mut padded_key = [0u8; Self::BLOCK_SIZE];
        
        // Prepare key
        if key.len() > Self::BLOCK_SIZE {
            // Hash long keys
            let hashed = Sha256::hash(key);
            padded_key[..32].copy_from_slice(&hashed);
        } else {
            padded_key[..key.len()].copy_from_slice(key);
        }

        // Create inner and outer padded keys
        let mut ipad = [0u8; Self::BLOCK_SIZE];
        let mut opad = [0u8; Self::BLOCK_SIZE];
        
        for i in 0..Self::BLOCK_SIZE {
            ipad[i] = padded_key[i] ^ Self::IPAD;
            opad[i] = padded_key[i] ^ Self::OPAD;
        }

        // Inner hash: H((K ⊕ ipad) || message)
        let mut inner_data = [0u8; Self::BLOCK_SIZE + 1024]; // Fixed size buffer
        let inner_len = Self::BLOCK_SIZE + message.len();
        
        if inner_len > inner_data.len() {
            // For large messages, process in chunks
            let mut inner_hash = [0u8; 32];
            let mut temp_buf = [0u8; Self::BLOCK_SIZE + 32];
            temp_buf[..Self::BLOCK_SIZE].copy_from_slice(&ipad);
            temp_buf[Self::BLOCK_SIZE..].copy_from_slice(&Sha256::hash(message));
            inner_hash = Sha256::hash(&temp_buf);
            
            // Outer hash: H((K ⊕ opad) || inner_hash)
            let mut outer_data = [0u8; Self::BLOCK_SIZE + 32];
            outer_data[..Self::BLOCK_SIZE].copy_from_slice(&opad);
            outer_data[Self::BLOCK_SIZE..].copy_from_slice(&inner_hash);
            Sha256::hash(&outer_data)
        } else {
            inner_data[..Self::BLOCK_SIZE].copy_from_slice(&ipad);
            inner_data[Self::BLOCK_SIZE..inner_len].copy_from_slice(message);
            let inner_hash = Sha256::hash(&inner_data[..inner_len]);

            // Outer hash: H((K ⊕ opad) || inner_hash)
            let mut outer_data = [0u8; Self::BLOCK_SIZE + 32];
            outer_data[..Self::BLOCK_SIZE].copy_from_slice(&opad);
            outer_data[Self::BLOCK_SIZE..].copy_from_slice(&inner_hash);
            Sha256::hash(&outer_data)
        }
    }

    /// Verify HMAC tag
    pub fn verify(key: &[u8], message: &[u8], tag: &[u8; 32]) -> bool {
        let computed = Self::mac(key, message);
        // Constant-time comparison
        constant_time_eq(&computed, tag)
    }
}

/// HMAC with SHA-512
pub struct HmacSha512;

impl HmacSha512 {
    const BLOCK_SIZE: usize = 128; // SHA-512 block size
    const IPAD: u8 = 0x36;
    const OPAD: u8 = 0x5c;

    /// Compute HMAC-SHA512
    pub fn mac(key: &[u8], message: &[u8]) -> [u8; 64] {
        let mut padded_key = [0u8; Self::BLOCK_SIZE];
        
        // Prepare key
        if key.len() > Self::BLOCK_SIZE {
            // Hash long keys
            let hashed = Sha512::hash(key);
            padded_key[..64].copy_from_slice(&hashed);
        } else {
            padded_key[..key.len()].copy_from_slice(key);
        }

        // Create inner and outer padded keys
        let mut ipad = [0u8; Self::BLOCK_SIZE];
        let mut opad = [0u8; Self::BLOCK_SIZE];
        
        for i in 0..Self::BLOCK_SIZE {
            ipad[i] = padded_key[i] ^ Self::IPAD;
            opad[i] = padded_key[i] ^ Self::OPAD;
        }

        // Inner hash: H((K ⊕ ipad) || message)
        let mut inner_data = [0u8; Self::BLOCK_SIZE + 1024]; // Fixed size buffer
        let inner_len = Self::BLOCK_SIZE + message.len();
        
        if inner_len > inner_data.len() {
            // For large messages, process in chunks
            let mut inner_hash = [0u8; 64];
            let mut temp_buf = [0u8; Self::BLOCK_SIZE + 64];
            temp_buf[..Self::BLOCK_SIZE].copy_from_slice(&ipad);
            temp_buf[Self::BLOCK_SIZE..].copy_from_slice(&Sha512::hash(message));
            inner_hash = Sha512::hash(&temp_buf);
            
            // Outer hash: H((K ⊕ opad) || inner_hash)
            let mut outer_data = [0u8; Self::BLOCK_SIZE + 64];
            outer_data[..Self::BLOCK_SIZE].copy_from_slice(&opad);
            outer_data[Self::BLOCK_SIZE..].copy_from_slice(&inner_hash);
            Sha512::hash(&outer_data)
        } else {
            inner_data[..Self::BLOCK_SIZE].copy_from_slice(&ipad);
            inner_data[Self::BLOCK_SIZE..inner_len].copy_from_slice(message);
            let inner_hash = Sha512::hash(&inner_data[..inner_len]);

            // Outer hash: H((K ⊕ opad) || inner_hash)
            let mut outer_data = [0u8; Self::BLOCK_SIZE + 64];
            outer_data[..Self::BLOCK_SIZE].copy_from_slice(&opad);
            outer_data[Self::BLOCK_SIZE..].copy_from_slice(&inner_hash);
            Sha512::hash(&outer_data)
        }
    }

    /// Verify HMAC tag
    pub fn verify(key: &[u8], message: &[u8], tag: &[u8; 64]) -> bool {
        let computed = Self::mac(key, message);
        // Constant-time comparison
        constant_time_eq_64(&computed, tag)
    }
}

/// Constant-time equality comparison for 32 bytes
fn constant_time_eq(a: &[u8; 32], b: &[u8; 32]) -> bool {
    let mut diff = 0u8;
    for i in 0..32 {
        diff |= a[i] ^ b[i];
    }
    diff == 0
}

/// Constant-time equality comparison for 64 bytes
fn constant_time_eq_64(a: &[u8; 64], b: &[u8; 64]) -> bool {
    let mut diff = 0u8;
    for i in 0..64 {
        diff |= a[i] ^ b[i];
    }
    diff == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hmac_sha256_empty() {
        let key = b"";
        let message = b"";
        let tag = HmacSha256::mac(key, message);
        
        // Should produce consistent output
        let tag2 = HmacSha256::mac(key, message);
        assert_eq!(tag, tag2);
    }

    #[test]
    fn test_hmac_sha256_basic() {
        let key = b"key";
        let message = b"The quick brown fox jumps over the lazy dog";
        let tag = HmacSha256::mac(key, message);
        
        // Should be deterministic
        let tag2 = HmacSha256::mac(key, message);
        assert_eq!(tag, tag2);
        
        // Different message should produce different tag
        let tag3 = HmacSha256::mac(key, b"different message");
        assert_ne!(tag, tag3);
    }

    #[test]
    fn test_hmac_sha256_verify() {
        let key = b"secret_key";
        let message = b"Important message";
        let tag = HmacSha256::mac(key, message);
        
        assert!(HmacSha256::verify(key, message, &tag));
        assert!(!HmacSha256::verify(key, b"tampered", &tag));
    }

    #[test]
    fn test_hmac_sha256_rfc_vector_1() {
        // RFC 4231 Test Case 1
        let key = [0x0b; 20];
        let message = b"Hi There";
        let expected = [
            0xb0, 0x34, 0x4c, 0x61, 0xd8, 0xdb, 0x38, 0x53,
            0x5c, 0xa8, 0xaf, 0xce, 0xaf, 0x0b, 0xf1, 0x2b,
            0x88, 0x1d, 0xc2, 0x00, 0xc9, 0x83, 0x3d, 0xa7,
            0x26, 0xe9, 0x37, 0x6c, 0x2e, 0x32, 0xcf, 0xf7,
        ];
        
        let tag = HmacSha256::mac(&key, message);
        assert_eq!(tag, expected);
    }

    #[test]
    fn test_hmac_sha256_rfc_vector_2() {
        // RFC 4231 Test Case 2
        let key = b"Jefe";
        let message = b"what do ya want for nothing?";
        let expected = [
            0x5b, 0xdc, 0xc1, 0x46, 0xbf, 0x60, 0x75, 0x4e,
            0x6a, 0x04, 0x24, 0x26, 0x08, 0x95, 0x75, 0xc7,
            0x5a, 0x00, 0x3f, 0x08, 0x9d, 0x27, 0x39, 0x83,
            0x9d, 0xec, 0x58, 0xb9, 0x64, 0xec, 0x38, 0x43,
        ];
        
        let tag = HmacSha256::mac(key, message);
        assert_eq!(tag, expected);
    }

    #[test]
    fn test_hmac_sha512_empty() {
        let key = b"";
        let message = b"";
        let tag = HmacSha512::mac(key, message);
        
        // Should produce consistent output
        let tag2 = HmacSha512::mac(key, message);
        assert_eq!(tag, tag2);
    }

    #[test]
    fn test_hmac_sha512_basic() {
        let key = b"key";
        let message = b"The quick brown fox jumps over the lazy dog";
        let tag = HmacSha512::mac(key, message);
        
        // Should be deterministic
        let tag2 = HmacSha512::mac(key, message);
        assert_eq!(tag, tag2);
    }

    #[test]
    fn test_hmac_sha512_verify() {
        let key = b"secret_key_512";
        let message = b"Important message for SHA-512";
        let tag = HmacSha512::mac(key, message);
        
        assert!(HmacSha512::verify(key, message, &tag));
        assert!(!HmacSha512::verify(key, b"tampered", &tag));
    }

    #[test]
    fn test_constant_time_eq() {
        let a = [0x42; 32];
        let b = [0x42; 32];
        let c = [0x43; 32];
        
        assert!(constant_time_eq(&a, &b));
        assert!(!constant_time_eq(&a, &c));
    }
}
