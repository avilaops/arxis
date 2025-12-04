//! EdDSA - Edwards-curve Digital Signature Algorithm (Ed25519)
//!
//! Features:
//! - Deterministic signatures (no random nonce needed)
//! - Fast verification
//! - Small signatures (64 bytes)
//! - Small public keys (32 bytes)
//! - Constant-time operations

use crate::curves::curve25519::{Curve25519, EdwardsPoint};
use crate::bigint::U256;
use crate::hash::sha512::Sha512;

/// Chave privada Ed25519 (32 bytes)
#[derive(Debug, Clone)]
pub struct SecretKey {
    seed: [u8; 32],
    scalar: U256,
    prefix: [u8; 32],
}

/// Chave pública Ed25519 (32 bytes)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PublicKey {
    point: EdwardsPoint,
    bytes: [u8; 32],
}

/// Assinatura Ed25519 (64 bytes)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Signature {
    /// R component (32 bytes)
    pub r: [u8; 32],
    /// S component (32 bytes)
    pub s: [u8; 32],
}

impl SecretKey {
    /// Gera chave privada de seed (32 bytes)
    pub fn from_seed(seed: [u8; 32]) -> Self {
        // Hash do seed com SHA-512
        let h = Sha512::hash(&seed);

        // Primeiros 32 bytes = scalar (clamped)
        let mut scalar_bytes = [0u8; 32];
        scalar_bytes.copy_from_slice(&h[..32]);

        // Clamping (RFC 8032 section 5.1.5)
        scalar_bytes[0] &= 248; // Clear 3 LSB
        scalar_bytes[31] &= 127; // Clear MSB
        scalar_bytes[31] |= 64; // Set second MSB

        let scalar = U256::from_bytes_le(&scalar_bytes);

        // Últimos 32 bytes = prefix para nonce
        let mut prefix = [0u8; 32];
        prefix.copy_from_slice(&h[32..]);

        Self {
            seed,
            scalar,
            prefix,
        }
    }

    /// Deriva chave pública
    pub fn public_key(&self) -> PublicKey {
        let point = EdwardsPoint::BASE.scalar_mul(&self.scalar);
        let bytes = point.compress();

        PublicKey { point, bytes }
    }

    /// Assina mensagem
    pub fn sign(&self, message: &[u8]) -> Signature {
        let public_key = self.public_key();

        // r = H(prefix || message) mod L
        let r = {
            let mut hasher = Sha512::new();
            hasher.update(&self.prefix);
            hasher.update(message);
            let h = hasher.finalize();
            let r_bytes = [
                h[0], h[1], h[2], h[3], h[4], h[5], h[6], h[7],
                h[8], h[9], h[10], h[11], h[12], h[13], h[14], h[15],
                h[16], h[17], h[18], h[19], h[20], h[21], h[22], h[23],
                h[24], h[25], h[26], h[27], h[28], h[29], h[30], h[31],
            ];
            reduce_scalar(&U256::from_bytes_le(&r_bytes))
        };

        // R = rG (ponto na curva)
        let big_r = EdwardsPoint::BASE.scalar_mul(&r);
        let r_bytes = big_r.compress();

        // k = H(R || A || message) mod L
        let k = {
            let mut hasher = Sha512::new();
            hasher.update(&r_bytes);
            hasher.update(&public_key.bytes);
            hasher.update(message);
            let h = hasher.finalize();
            let k_bytes = [
                h[0], h[1], h[2], h[3], h[4], h[5], h[6], h[7],
                h[8], h[9], h[10], h[11], h[12], h[13], h[14], h[15],
                h[16], h[17], h[18], h[19], h[20], h[21], h[22], h[23],
                h[24], h[25], h[26], h[27], h[28], h[29], h[30], h[31],
            ];
            reduce_scalar(&U256::from_bytes_le(&k_bytes))
        };

        // s = (r + k * a) mod L
        let s = {
            let ka = mul_mod_l(&k, &self.scalar);
            add_mod_l(&r, &ka)
        };

        Signature {
            r: r_bytes,
            s: s.to_bytes_le(),
        }
    }

    /// Retorna bytes do seed
    pub fn to_bytes(&self) -> [u8; 32] {
        self.seed
    }
}

impl PublicKey {
    /// Cria de bytes comprimidos
    pub fn from_bytes(bytes: [u8; 32]) -> Option<Self> {
        let point = EdwardsPoint::decompress(&bytes)?;
        Some(Self { point, bytes })
    }

    /// Verifica assinatura
    pub fn verify(&self, message: &[u8], signature: &Signature) -> bool {
        // Decodifica R
        let big_r = match EdwardsPoint::decompress(&signature.r) {
            Some(r) => r,
            None => return false,
        };

        // Decodifica S
        let s = U256::from_bytes_le(&signature.s);

        // Verifica S < L
        if s >= Curve25519::L {
            return false;
        }

        // k = H(R || A || message) mod L
        let k = {
            let mut hasher = Sha512::new();
            hasher.update(&signature.r);
            hasher.update(&self.bytes);
            hasher.update(message);
            let h = hasher.finalize();
            let k_bytes = [
                h[0], h[1], h[2], h[3], h[4], h[5], h[6], h[7],
                h[8], h[9], h[10], h[11], h[12], h[13], h[14], h[15],
                h[16], h[17], h[18], h[19], h[20], h[21], h[22], h[23],
                h[24], h[25], h[26], h[27], h[28], h[29], h[30], h[31],
            ];
            reduce_scalar(&U256::from_bytes_le(&k_bytes))
        };

        // Verifica: sG = R + kA
        let sg = EdwardsPoint::BASE.scalar_mul(&s);
        let ka = self.point.scalar_mul(&k);
        let r_plus_ka = big_r.add(&ka);

        sg == r_plus_ka
    }

    /// Retorna bytes da chave
    pub fn to_bytes(&self) -> [u8; 32] {
        self.bytes
    }
}

impl Signature {
    /// Cria de bytes (64 bytes)
    pub fn from_bytes(bytes: &[u8; 64]) -> Self {
        let mut r = [0u8; 32];
        let mut s = [0u8; 32];
        r.copy_from_slice(&bytes[..32]);
        s.copy_from_slice(&bytes[32..]);
        Self { r, s }
    }

    /// Converte para bytes (64 bytes)
    pub fn to_bytes(&self) -> [u8; 64] {
        let mut result = [0u8; 64];
        result[..32].copy_from_slice(&self.r);
        result[32..].copy_from_slice(&self.s);
        result
    }
}

// Helper functions

fn reduce_scalar(n: &U256) -> U256 {
    // n mod L
    n.wrapping_rem(&Curve25519::L)
}

fn add_mod_l(a: &U256, b: &U256) -> U256 {
    let sum = a.wrapping_add(b);
    if sum >= Curve25519::L {
        sum.wrapping_sub(&Curve25519::L)
    } else {
        sum
    }
}

fn mul_mod_l(a: &U256, b: &U256) -> U256 {
    a.wrapping_mul(b).wrapping_rem(&Curve25519::L)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keygen() {
        let seed = [0x42; 32];
        let sk = SecretKey::from_seed(seed);
        let pk = sk.public_key();

        assert_eq!(pk.to_bytes().len(), 32);
    }

    #[test]
    fn test_sign_verify() {
        let seed = [0x42; 32];
        let sk = SecretKey::from_seed(seed);
        let pk = sk.public_key();

        let message = b"Hello, Ed25519!";
        let signature = sk.sign(message);

        assert!(pk.verify(message, &signature));
    }

    #[test]
    fn test_invalid_signature() {
        let seed = [0x42; 32];
        let sk = SecretKey::from_seed(seed);
        let pk = sk.public_key();

        let message = b"Hello, Ed25519!";
        let signature = sk.sign(message);

        // Modifica mensagem
        let wrong_message = b"Wrong message!";
        assert!(!pk.verify(wrong_message, &signature));
    }

    #[test]
    fn test_signature_serialization() {
        let seed = [0x42; 32];
        let sk = SecretKey::from_seed(seed);
        let message = b"Test";

        let sig1 = sk.sign(message);
        let bytes = sig1.to_bytes();
        let sig2 = Signature::from_bytes(&bytes);

        assert_eq!(sig1, sig2);
    }
}
