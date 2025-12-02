//! # avila-post-quantum - Post-Quantum Cryptography
extern crate alloc;
use alloc::vec::Vec;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PqAlgorithm { Kyber, Dilithium, Sphincs }

pub struct PqKeyPair {
    pub algorithm: PqAlgorithm,
    pub public_key: Vec<u8>,
    pub secret_key: Vec<u8>,
}

impl PqKeyPair {
    pub fn new(algorithm: PqAlgorithm) -> Self {
        Self {
            algorithm,
            public_key: vec![0u8; 800],
            secret_key: vec![0u8; 1632],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_pq_keypair() {
        let kp = PqKeyPair::new(PqAlgorithm::Kyber);
        assert_eq!(kp.algorithm, PqAlgorithm::Kyber);
        assert_eq!(kp.public_key.len(), 800);
    }
}
