//! 512-bit signed integer type (two's complement)

use crate::u512::U512;

/// 512-bit signed integer (two's complement)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct I512(pub U512);

impl I512 {
    /// Zero value
    pub const ZERO: Self = Self(U512::ZERO);
    
    /// One value
    pub const ONE: Self = Self(U512::ONE);
    
    /// Check if negative
    #[inline]
    pub fn is_negative(&self) -> bool {
        (self.0).0[7] & (1u64 << 63) != 0
    }
}

// TODO: Implement full arithmetic
