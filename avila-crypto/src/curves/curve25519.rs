//! Curve25519 - Modern, fast, and secure
//!
//! Montgomery curve: By² = x³ + Ax² + x
//! Twisted Edwards form: -x² + y² = 1 - (121665/121666)x²y²
//!
//! Benefits:
//! - Complete addition formulas (no edge cases)
//! - Constant-time operations (side-channel resistant)
//! - Prime: 2^255 - 19 (fast arithmetic)
//! - Twist secure (dual curve is also secure)

use crate::bigint::U256;

/// Curve25519 parameters
pub struct Curve25519;

impl Curve25519 {
    /// Prime: 2^255 - 19
    pub const P: U256 = U256 {
        limbs: [
            0xFFFFFFFFFFFFFFED,
            0xFFFFFFFFFFFFFFFF,
            0xFFFFFFFFFFFFFFFF,
            0x7FFFFFFFFFFFFFFF,
        ],
    };

    /// Subgroup order
    pub const L: U256 = U256 {
        limbs: [
            0x5812631A5CF5D3ED,
            0x14DEF9DEA2F79CD6,
            0x0000000000000000,
            0x1000000000000000,
        ],
    };

    /// Montgomery coefficient A = 486662
    pub const A: U256 = U256 {
        limbs: [486662, 0, 0, 0],
    };
}
