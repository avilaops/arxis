//! Digital signature schemes
//!
//! Sovereign selection: battle-tested > government-approved

pub mod ecdsa;
pub mod schnorr;
// pub mod eddsa; // TODO: Fix compiler ICE

/// Signature verification result
pub type VerifyResult = Result<(), SignatureError>;

/// Signature errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignatureError {
    /// Invalid signature format
    InvalidFormat,
    /// Signature verification failed
    VerificationFailed,
    /// Invalid public key
    InvalidPublicKey,
}
