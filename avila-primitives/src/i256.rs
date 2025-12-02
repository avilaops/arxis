//! 256-bit signed integer type (two's complement)

use crate::u256::U256;

/// 256-bit signed integer (two's complement)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct I256(pub U256);

impl I256 {
    /// Zero value
    pub const ZERO: Self = Self(U256::ZERO);
    
    /// One value
    pub const ONE: Self = Self(U256::ONE);
    
    /// Negative one
    pub const NEG_ONE: Self = Self(U256::MAX);
    
    /// Minimum value
    pub const MIN: Self = Self(U256([0, 0, 0, 1u64 << 63]));
    
    /// Maximum value
    pub const MAX: Self = Self(U256([u64::MAX, u64::MAX, u64::MAX, (1u64 << 63) - 1]));
    
    /// Check if negative
    #[inline]
    pub fn is_negative(&self) -> bool {
        (self.0).0[3] & (1u64 << 63) != 0
    }
    
    /// Create from i64
    pub fn from_i64(value: i64) -> Self {
        if value >= 0 {
            Self(U256::from_u64(value as u64))
        } else {
            // Two's complement for negative numbers
            let abs = value.unsigned_abs();
            let mut result = U256::from_u64(abs);
            result = !result;
            result = result + U256::ONE;
            Self(result)
        }
    }
}

// TODO: Implement full arithmetic with sign handling
