//! 4096-bit unsigned integer type

/// 4096-bit unsigned integer (64 x u64)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct U4096(pub [u64; 64]);

impl U4096 {
    /// Zero value
    pub const ZERO: Self = Self([0; 64]);
    
    /// One value
    pub const ONE: Self = {
        let mut arr = [0u64; 64];
        arr[0] = 1;
        Self(arr)
    };
    
    /// Maximum value
    pub const MAX: Self = Self([u64::MAX; 64]);
}

// TODO: Implement full arithmetic
