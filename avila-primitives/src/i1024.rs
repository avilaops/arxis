//! 1024-bit signed integer type (two's complement)

use crate::u1024::U1024;

/// 1024-bit signed integer (two's complement)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct I1024(pub U1024);

impl I1024 {
    /// Zero value
    pub const ZERO: Self = Self(U1024::ZERO);

    /// One value
    pub const ONE: Self = Self(U1024::ONE);

    /// Check if negative
    #[inline]
    pub fn is_negative(&self) -> bool {
        (self.0).0[15] & (1u64 << 63) != 0
    }
}

// TODO: Implement full arithmetic
