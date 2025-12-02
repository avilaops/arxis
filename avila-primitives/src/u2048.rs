//! 2048-bit unsigned integer type

/// 2048-bit unsigned integer (32 x u64)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct U2048(pub [u64; 32]);

impl U2048 {
    /// Zero value
    pub const ZERO: Self = Self([0; 32]);
    
    /// One value  
    pub const ONE: Self = {
        let mut arr = [0u64; 32];
        arr[0] = 1;
        Self(arr)
    };
    
    /// Maximum value
    pub const MAX: Self = Self([u64::MAX; 32]);
}

// TODO: Implement full arithmetic
