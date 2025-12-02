//! # avila-zkp - Zero-Knowledge Proofs
extern crate alloc;
use alloc::vec::Vec;

pub struct Proof {
    pub commitment: Vec<u8>,
    pub challenge: Vec<u8>,
    pub response: Vec<u8>,
}

impl Proof {
    pub fn new(commitment: Vec<u8>) -> Self {
        Self {
            commitment,
            challenge: Vec::new(),
            response: Vec::new(),
        }
    }
    
    pub fn verify(&self) -> bool {
        !self.commitment.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_proof() {
        let proof = Proof::new(vec![1, 2, 3]);
        assert!(proof.verify());
    }
}
