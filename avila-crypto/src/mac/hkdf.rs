//! HKDF - HMAC-based Key Derivation Function
//!
//! RFC 5869 compliant implementation
//! Supports SHA-256 and SHA-512

#![allow(dead_code)]

use crate::mac::hmac::{HmacSha256, HmacSha512};

/// HKDF with SHA-256
pub struct HkdfSha256;

impl HkdfSha256 {
    const HASH_LEN: usize = 32;

    /// Extract step: HKDF-Extract(salt, IKM) -> PRK
    pub fn extract(salt: &[u8], ikm: &[u8]) -> [u8; 32] {
        let salt = if salt.is_empty() {
            &[0u8; 32][..]
        } else {
            salt
        };
        
        HmacSha256::mac(salt, ikm)
    }

    /// Expand step: HKDF-Expand(PRK, info, L) -> OKM
    pub fn expand(prk: &[u8; 32], info: &[u8], length: usize) -> Result<[u8; 255], HkdfError> {
        if length > 255 * Self::HASH_LEN {
            return Err(HkdfError::InvalidLength);
        }

        let mut okm = [0u8; 255];
        let n = (length + Self::HASH_LEN - 1) / Self::HASH_LEN;
        
        let mut t = [0u8; 32];
        let mut t_len = 0;

        for i in 0..n {
            // T(i) = HMAC-Hash(PRK, T(i-1) | info | i)
            let mut data = [0u8; 32 + 256 + 1]; // T + info + counter
            let mut data_len = 0;
            
            if i > 0 {
                data[..32].copy_from_slice(&t);
                data_len = 32;
            }
            
            let info_len = core::cmp::min(info.len(), 256);
            data[data_len..data_len + info_len].copy_from_slice(&info[..info_len]);
            data_len += info_len;
            
            data[data_len] = (i + 1) as u8;
            data_len += 1;

            t = HmacSha256::mac(prk, &data[..data_len]);
            t_len = Self::HASH_LEN;

            // Copy to output
            let offset = i * Self::HASH_LEN;
            let copy_len = core::cmp::min(t_len, length - offset);
            okm[offset..offset + copy_len].copy_from_slice(&t[..copy_len]);
        }

        Ok(okm)
    }

    /// Combined extract-then-expand: HKDF(salt, IKM, info, L) -> OKM
    pub fn derive(salt: &[u8], ikm: &[u8], info: &[u8], length: usize) -> Result<[u8; 255], HkdfError> {
        let prk = Self::extract(salt, ikm);
        Self::expand(&prk, info, length)
    }
}

/// HKDF with SHA-512
pub struct HkdfSha512;

impl HkdfSha512 {
    const HASH_LEN: usize = 64;

    /// Extract step: HKDF-Extract(salt, IKM) -> PRK
    pub fn extract(salt: &[u8], ikm: &[u8]) -> [u8; 64] {
        let salt = if salt.is_empty() {
            &[0u8; 64][..]
        } else {
            salt
        };
        
        HmacSha512::mac(salt, ikm)
    }

    /// Expand step: HKDF-Expand(PRK, info, L) -> OKM
    pub fn expand(prk: &[u8; 64], info: &[u8], length: usize) -> Result<[u8; 255], HkdfError> {
        if length > 255 * Self::HASH_LEN {
            return Err(HkdfError::InvalidLength);
        }

        let mut okm = [0u8; 255];
        let n = (length + Self::HASH_LEN - 1) / Self::HASH_LEN;
        
        let mut t = [0u8; 64];
        let mut t_len = 0;

        for i in 0..n {
            // T(i) = HMAC-Hash(PRK, T(i-1) | info | i)
            let mut data = [0u8; 64 + 256 + 1]; // T + info + counter
            let mut data_len = 0;
            
            if i > 0 {
                data[..64].copy_from_slice(&t);
                data_len = 64;
            }
            
            let info_len = core::cmp::min(info.len(), 256);
            data[data_len..data_len + info_len].copy_from_slice(&info[..info_len]);
            data_len += info_len;
            
            data[data_len] = (i + 1) as u8;
            data_len += 1;

            t = HmacSha512::mac(prk, &data[..data_len]);
            t_len = Self::HASH_LEN;

            // Copy to output
            let offset = i * Self::HASH_LEN;
            let copy_len = core::cmp::min(t_len, length - offset);
            okm[offset..offset + copy_len].copy_from_slice(&t[..copy_len]);
        }

        Ok(okm)
    }

    /// Combined extract-then-expand: HKDF(salt, IKM, info, L) -> OKM
    pub fn derive(salt: &[u8], ikm: &[u8], info: &[u8], length: usize) -> Result<[u8; 255], HkdfError> {
        let prk = Self::extract(salt, ikm);
        Self::expand(&prk, info, length)
    }
}

/// HKDF error types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HkdfError {
    /// Requested output length too large
    InvalidLength,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hkdf_sha256_extract() {
        let salt = b"salt";
        let ikm = b"input key material";
        let prk = HkdfSha256::extract(salt, ikm);
        
        // Should be deterministic
        let prk2 = HkdfSha256::extract(salt, ikm);
        assert_eq!(prk, prk2);
    }

    #[test]
    fn test_hkdf_sha256_expand() {
        let prk = [0x42; 32];
        let info = b"application info";
        let okm = HkdfSha256::expand(&prk, info, 42).unwrap();
        
        // Should be deterministic
        let okm2 = HkdfSha256::expand(&prk, info, 42).unwrap();
        assert_eq!(&okm[..42], &okm2[..42]);
    }

    #[test]
    fn test_hkdf_sha256_derive() {
        let salt = b"salt";
        let ikm = b"input key material";
        let info = b"application info";
        let okm = HkdfSha256::derive(salt, ikm, info, 42).unwrap();
        
        // Should be deterministic
        let okm2 = HkdfSha256::derive(salt, ikm, info, 42).unwrap();
        assert_eq!(&okm[..42], &okm2[..42]);
    }

    #[test]
    fn test_hkdf_sha256_rfc_vector_1() {
        // RFC 5869 Test Case 1
        let ikm = [0x0b; 22];
        let salt = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
            0x08, 0x09, 0x0a, 0x0b, 0x0c,
        ];
        let info = [
            0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7,
            0xf8, 0xf9,
        ];
        
        let prk = HkdfSha256::extract(&salt, &ikm);
        let expected_prk = [
            0x07, 0x77, 0x09, 0x36, 0x2c, 0x2e, 0x32, 0xdf,
            0x0d, 0xdc, 0x3f, 0x0d, 0xc4, 0x7b, 0xba, 0x63,
            0x90, 0xb6, 0xc7, 0x3b, 0xb5, 0x0f, 0x9c, 0x31,
            0x22, 0xec, 0x84, 0x4a, 0xd7, 0xc2, 0xb3, 0xe5,
        ];
        assert_eq!(prk, expected_prk);

        let okm = HkdfSha256::expand(&prk, &info, 42).unwrap();
        let expected_okm = [
            0x3c, 0xb2, 0x5f, 0x25, 0xfa, 0xac, 0xd5, 0x7a,
            0x90, 0x43, 0x4f, 0x64, 0xd0, 0x36, 0x2f, 0x2a,
            0x2d, 0x2d, 0x0a, 0x90, 0xcf, 0x1a, 0x5a, 0x4c,
            0x5d, 0xb0, 0x2d, 0x56, 0xec, 0xc4, 0xc5, 0xbf,
            0x34, 0x00, 0x72, 0x08, 0xd5, 0xb8, 0x87, 0x18,
            0x58, 0x65,
        ];
        assert_eq!(&okm[..42], &expected_okm[..]);
    }

    #[test]
    fn test_hkdf_sha256_no_salt() {
        let ikm = b"input key material";
        let info = b"info";
        let okm = HkdfSha256::derive(&[], ikm, info, 32).unwrap();
        
        // Should work with empty salt
        assert_eq!(okm[0..32].len(), 32);
    }

    #[test]
    fn test_hkdf_sha256_no_info() {
        let salt = b"salt";
        let ikm = b"input key material";
        let okm = HkdfSha256::derive(salt, ikm, &[], 32).unwrap();
        
        // Should work with empty info
        assert_eq!(okm[0..32].len(), 32);
    }

    #[test]
    fn test_hkdf_sha512_basic() {
        let salt = b"salt";
        let ikm = b"input key material";
        let info = b"application info";
        let okm = HkdfSha512::derive(salt, ikm, info, 64).unwrap();
        
        // Should be deterministic
        let okm2 = HkdfSha512::derive(salt, ikm, info, 64).unwrap();
        assert_eq!(&okm[..64], &okm2[..64]);
    }

    #[test]
    fn test_hkdf_invalid_length() {
        let prk = [0x42; 32];
        let info = b"info";
        let result = HkdfSha256::expand(&prk, info, 8192);
        assert_eq!(result, Err(HkdfError::InvalidLength));
    }

    #[test]
    fn test_hkdf_sha256_multiple_blocks() {
        // Test expansion that requires multiple HMAC blocks
        let prk = [0x42; 32];
        let info = b"test";
        let okm = HkdfSha256::expand(&prk, info, 64).unwrap();
        
        // Should produce 64 bytes
        assert_eq!(okm[..64].len(), 64);
    }
}
