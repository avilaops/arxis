//! Elliptic curve implementations
//!
//! Sovereign curve selection based on mathematical transparency,
//! not government approval.

pub mod secp256k1;
pub mod curve25519;

/// Point on an elliptic curve
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point<T> {
    /// X coordinate
    pub x: T,
    /// Y coordinate
    pub y: T,
    /// Infinity flag
    pub infinity: bool,
}

/// Elliptic curve trait
pub trait EllipticCurve {
    /// Field element type
    type Field: Copy + Clone;

    /// Scalar type
    type Scalar: Copy + Clone;

    /// Generator point
    fn generator() -> Point<Self::Field>;

    /// Point addition
    fn add(p: &Point<Self::Field>, q: &Point<Self::Field>) -> Point<Self::Field>;

    /// Point doubling
    fn double(p: &Point<Self::Field>) -> Point<Self::Field>;

    /// Scalar multiplication
    fn scalar_mul(k: &Self::Scalar, p: &Point<Self::Field>) -> Point<Self::Field>;
}
