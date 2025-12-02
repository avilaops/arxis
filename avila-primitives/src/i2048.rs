//! 2048-bit signed integer type (two's complement)

use crate::u2048::U2048;

/// 2048-bit signed integer (two's complement)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct I2048(pub U2048);

impl I2048 {
    /// Zero value
    pub const ZERO: Self = Self(U2048::ZERO);

    /// One value
    pub const ONE: Self = Self(U2048::ONE);

    /// Check if negative
    #[inline]
    pub fn is_negative(&self) -> bool {
        (self.0).0[31] & (1u64 << 63) != 0
    }
}

// TODO: Implement full arithmetic
