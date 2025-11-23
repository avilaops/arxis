//! Cryptographic utilities and key management

use crate::error::{AuthError, Result};
use rand::RngCore;
use sha2::{Digest, Sha256};
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};

pub struct CryptoManager;

impl CryptoManager {
    pub fn new() -> Self {
        Self
    }

    /// Generate a cryptographically secure random token
    pub fn generate_token(&self, length: usize) -> String {
        let mut bytes = vec![0u8; length];
        rand::thread_rng().fill_bytes(&mut bytes);
        URL_SAFE_NO_PAD.encode(&bytes)
    }

    /// Generate RSA key pair for JWT signing
    pub fn generate_rsa_keypair(&self, bits: usize) -> Result<(String, String)> {
        use rsa::{RsaPrivateKey, RsaPublicKey};
        use rsa::pkcs8::{EncodePrivateKey, EncodePublicKey};

        let mut rng = rand::thread_rng();
        let private_key = RsaPrivateKey::new(&mut rng, bits)
            .map_err(|e| AuthError::CryptoError(e.to_string()))?;
        let public_key = RsaPublicKey::from(&private_key);

        let private_pem = private_key
            .to_pkcs8_pem(rsa::pkcs8::LineEnding::LF)
            .map_err(|e| AuthError::CryptoError(e.to_string()))?
            .to_string();

        let public_pem = public_key
            .to_public_key_pem(rsa::pkcs8::LineEnding::LF)
            .map_err(|e| AuthError::CryptoError(e.to_string()))?;

        Ok((private_pem, public_pem))
    }

    /// Generate EC key pair for JWT signing
    pub fn generate_ec_keypair(&self) -> Result<(String, String)> {
        use p256::SecretKey;
        use p256::pkcs8::{EncodePrivateKey, EncodePublicKey};

        let secret_key = SecretKey::random(&mut rand::thread_rng());
        let public_key = secret_key.public_key();

        let private_pem = secret_key
            .to_pkcs8_pem(p256::pkcs8::LineEnding::LF)
            .map_err(|e| AuthError::CryptoError(e.to_string()))?
            .to_string();

        let public_pem = public_key
            .to_public_key_pem(p256::pkcs8::LineEnding::LF)
            .map_err(|e| AuthError::CryptoError(e.to_string()))?;

        Ok((private_pem, public_pem))
    }

    /// Hash data with SHA-256
    pub fn hash_sha256(&self, data: &[u8]) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }

    /// Generate a secure password reset token
    pub fn generate_reset_token(&self) -> String {
        self.generate_token(32)
    }

    /// Generate email verification token
    pub fn generate_verification_token(&self) -> String {
        self.generate_token(32)
    }

    /// Constant-time string comparison
    pub fn constant_time_compare(&self, a: &str, b: &str) -> bool {
        use subtle::ConstantTimeEq;
        a.as_bytes().ct_eq(b.as_bytes()).into()
    }

    /// Encrypt sensitive data (AES-256-GCM)
    pub fn encrypt(&self, plaintext: &[u8], key: &[u8]) -> Result<Vec<u8>> {
        use aes_gcm::{
            aead::{Aead, AeadCore, KeyInit, OsRng},
            Aes256Gcm,
        };

        let cipher = Aes256Gcm::new_from_slice(key)
            .map_err(|e| AuthError::CryptoError(e.to_string()))?;

        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

        let mut ciphertext = cipher
            .encrypt(&nonce, plaintext)
            .map_err(|e| AuthError::CryptoError(e.to_string()))?;

        // Prepend nonce to ciphertext
        let mut result = nonce.to_vec();
        result.append(&mut ciphertext);

        Ok(result)
    }

    /// Decrypt sensitive data (AES-256-GCM)
    pub fn decrypt(&self, ciphertext: &[u8], key: &[u8]) -> Result<Vec<u8>> {
        use aes_gcm::{
            aead::{Aead, KeyInit},
            Aes256Gcm,
        };
        use aes_gcm::aead::generic_array::GenericArray;

        if ciphertext.len() < 12 {
            return Err(AuthError::CryptoError("Invalid ciphertext".to_string()));
        }

        let cipher = Aes256Gcm::new_from_slice(key)
            .map_err(|e| AuthError::CryptoError(e.to_string()))?;

        let (nonce_bytes, ciphertext_bytes) = ciphertext.split_at(12);
        let nonce = GenericArray::from_slice(nonce_bytes);

        cipher
            .decrypt(nonce, ciphertext_bytes)
            .map_err(|e| AuthError::CryptoError(e.to_string()))
    }

    /// Generate device fingerprint
    pub fn generate_device_fingerprint(&self, user_agent: &str, ip: &str) -> String {
        let data = format!("{}:{}", user_agent, ip);
        let hash = self.hash_sha256(data.as_bytes());
        URL_SAFE_NO_PAD.encode(&hash)
    }
}

impl Default for CryptoManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_token() {
        let crypto = CryptoManager::new();
        let token = crypto.generate_token(32);
        assert!(!token.is_empty());
    }

    #[test]
    fn test_constant_time_compare() {
        let crypto = CryptoManager::new();
        assert!(crypto.constant_time_compare("hello", "hello"));
        assert!(!crypto.constant_time_compare("hello", "world"));
    }

    #[test]
    fn test_encrypt_decrypt() {
        let crypto = CryptoManager::new();
        let key = &[0u8; 32]; // 256-bit key
        let plaintext = b"secret data";

        let ciphertext = crypto.encrypt(plaintext, key).unwrap();
        let decrypted = crypto.decrypt(&ciphertext, key).unwrap();

        assert_eq!(plaintext, &decrypted[..]);
    }
}
