//! ChaCha20-Poly1305 AEAD - RFC 8439

extern crate alloc;
use alloc::vec::Vec;

use super::chacha20::ChaCha20;
use crate::mac::poly1305::Poly1305;

/// ChaCha20-Poly1305 AEAD cipher
pub struct ChaCha20Poly1305 {
    key: [u8; 32],
}

impl ChaCha20Poly1305 {
    /// Create new AEAD cipher with key
    pub fn new(key: &[u8; 32]) -> Self {
        Self { key: *key }
    }
    
    /// Encrypt and authenticate, returns (ciphertext, tag)
    pub fn encrypt(&self, nonce: &[u8; 12], plaintext: &[u8], aad: &[u8]) -> (Vec<u8>, [u8; 16]) {
        let mut poly_key = [0u8; 32];
        let mut chacha = ChaCha20::new(&self.key, nonce, 0);
        chacha.apply_keystream(&mut poly_key);
        
        let mut ciphertext = plaintext.to_vec();
        let mut chacha = ChaCha20::new(&self.key, nonce, 1);
        chacha.apply_keystream(&mut ciphertext);
        
        let tag = self.compute_tag(&poly_key, aad, &ciphertext);
        (ciphertext, tag)
    }
    
    /// Decrypt and verify, returns Some(plaintext) if auth succeeds
    pub fn decrypt(&self, nonce: &[u8; 12], ciphertext: &[u8], aad: &[u8], tag: &[u8; 16]) -> Option<Vec<u8>> {
        let mut poly_key = [0u8; 32];
        let mut chacha = ChaCha20::new(&self.key, nonce, 0);
        chacha.apply_keystream(&mut poly_key);
        
        let computed = self.compute_tag(&poly_key, aad, ciphertext);
        if computed != *tag {
            return None;
        }
        
        let mut plaintext = ciphertext.to_vec();
        let mut chacha = ChaCha20::new(&self.key, nonce, 1);
        chacha.apply_keystream(&mut plaintext);
        Some(plaintext)
    }
    
    fn compute_tag(&self, key: &[u8; 32], aad: &[u8], ciphertext: &[u8]) -> [u8; 16] {
        let mut poly = Poly1305::new(key);
        
        poly.update(aad);
        let aad_pad = (16 - (aad.len() % 16)) % 16;
        if aad_pad > 0 {
            poly.update(&[0u8; 16][..aad_pad]);
        }
        
        poly.update(ciphertext);
        let ct_pad = (16 - (ciphertext.len() % 16)) % 16;
        if ct_pad > 0 {
            poly.update(&[0u8; 16][..ct_pad]);
        }
        
        let mut lengths = [0u8; 16];
        lengths[0..8].copy_from_slice(&(aad.len() as u64).to_le_bytes());
        lengths[8..16].copy_from_slice(&(ciphertext.len() as u64).to_le_bytes());
        poly.update(&lengths);
        
        poly.finalize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_aead_roundtrip() {
        let key = [0x42; 32];
        let nonce = [0x07; 12];
        let plaintext = b"Hello AEAD!";
        let aad = b"metadata";
        
        let aead = ChaCha20Poly1305::new(&key);
        let (ct, tag) = aead.encrypt(&nonce, plaintext, aad);
        
        assert_ne!(ct.as_slice(), plaintext);
        
        let pt = aead.decrypt(&nonce, &ct, aad, &tag).expect("Should decrypt");
        assert_eq!(pt.as_slice(), plaintext);
    }
    
    #[test]
    fn test_aead_auth_failure() {
        let key = [0x42; 32];
        let nonce = [0x07; 12];
        let aead = ChaCha20Poly1305::new(&key);
        
        let (ct, mut tag) = aead.encrypt(&nonce, b"secret", b"meta");
        tag[0] ^= 1;
        
        assert!(aead.decrypt(&nonce, &ct, b"meta", &tag).is_none());
    }
}
