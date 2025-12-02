//! 1024-bit unsigned integer type

/// 1024-bit unsigned integer (16 x u64)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct U1024(pub [u64; 16]);

impl U1024 {
    /// Zero value
    pub const ZERO: Self = Self([0; 16]);
    
    /// One value
    pub const ONE: Self = Self([1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    
    /// Maximum value
    pub const MAX: Self = Self([u64::MAX; 16]);
}

// TODO: Implement full arithmetic
