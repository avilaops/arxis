//! Fixed-size big integer types for cryptography
//!
//! All types are stack-allocated with compile-time known sizes.

mod u256;
mod u384;
mod u512;
mod u2048;
mod u4096;
mod ops;

pub use u256::U256;
pub use u384::U384;
pub use u512::U512;
pub use u2048::U2048;
pub use u4096::U4096;

/// Common operations for all big integer types
pub trait BigInt: Sized + Clone + Copy {
    /// Number of 64-bit limbs
    const LIMBS: usize;

    /// Number of bits
    const BITS: usize;

    /// Zero value
    const ZERO: Self;

    /// One value
    const ONE: Self;

    /// Creates from big-endian bytes
    fn from_bytes_be(bytes: &[u8]) -> Self;

    /// Modular addition: (self + other) mod m
    fn add_mod(&self, other: &Self, modulus: &Self) -> Self;

    /// Modular multiplication: (self * other) mod m
    fn mul_mod(&self, other: &Self, modulus: &Self) -> Self;

    /// Modular exponentiation: self^exp mod m
    fn pow_mod(&self, exp: &Self, modulus: &Self) -> Self;

    /// Modular inverse: self^(-1) mod m
    fn inv_mod(&self, modulus: &Self) -> Option<Self>;
}
