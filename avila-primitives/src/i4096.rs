//! 4096-bit signed integer type (two's complement)

use crate::u4096::U4096;

/// 4096-bit signed integer (two's complement)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct I4096(pub U4096);

impl I4096 {
    /// Zero value
    pub const ZERO: Self = Self(U4096::ZERO);
    
    /// One value
    pub const ONE: Self = Self(U4096::ONE);
    
    /// Check if negative
    #[inline]
    pub fn is_negative(&self) -> bool {
        (self.0).0[63] & (1u64 << 63) != 0
    }
}

// TODO: Implement full arithmetic
