//! Hash functions
//!
//! Selection criteria: Speed, security, ZK-friendliness

pub mod blake3;
pub mod sha3;
pub mod sha256;
pub mod sha512;

/// Hasher trait
pub trait Hasher {
    /// Output size in bytes
    const OUTPUT_SIZE: usize;

    /// Output type
    type Output;

    /// Hash data
    fn hash(data: &[u8]) -> Self::Output;
}
