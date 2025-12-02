//! 1024-bit unsigned integer type

use avila_nucleus::bits::u1024_ops::*;
use core::cmp::Ordering;
use core::ops::{Add, Sub, Mul, Div, Rem, BitAnd, BitOr, BitXor, Not, Shl, Shr};

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

    /// Create from u64
    pub const fn from_u64(n: u64) -> Self {
        let mut limbs = [0u64; 16];
        limbs[0] = n;
        Self(limbs)
    }

    /// Check if zero
    pub const fn is_zero(&self) -> bool {
        is_zero1024(&self.0)
    }

    /// Leading zeros count
    pub const fn leading_zeros(&self) -> u32 {
        leading_zeros1024(&self.0)
    }
}

// Arithmetic traits
impl Add for U1024 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let (result, _) = add1024(&self.0, &rhs.0);
        Self(result)
    }
}

impl Sub for U1024 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let (result, _) = sub1024(&self.0, &rhs.0);
        Self(result)
    }
}

impl Mul for U1024 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let result = mul1024x1024(&self.0, &rhs.0);
        let mut limbs = [0u64; 16];
        limbs.copy_from_slice(&result[0..16]);
        Self(limbs)
    }
}

impl Div for U1024 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        let (quotient, _) = div1024(&self.0, &rhs.0);
        Self(quotient)
    }
}

impl Rem for U1024 {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self {
        let (_, remainder) = div1024(&self.0, &rhs.0);
        Self(remainder)
    }
}

// Bitwise operations
impl BitAnd for U1024 {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        let mut result = [0u64; 16];
        for i in 0..16 {
            result[i] = self.0[i] & rhs.0[i];
        }
        Self(result)
    }
}

impl BitOr for U1024 {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        let mut result = [0u64; 16];
        for i in 0..16 {
            result[i] = self.0[i] | rhs.0[i];
        }
        Self(result)
    }
}

impl BitXor for U1024 {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self {
        let mut result = [0u64; 16];
        for i in 0..16 {
            result[i] = self.0[i] ^ rhs.0[i];
        }
        Self(result)
    }
}

impl Not for U1024 {
    type Output = Self;

    fn not(self) -> Self {
        let mut result = [0u64; 16];
        for i in 0..16 {
            result[i] = !self.0[i];
        }
        Self(result)
    }
}

// Shift operations
impl Shl<u32> for U1024 {
    type Output = Self;

    fn shl(self, rhs: u32) -> Self {
        Self(shl1024(&self.0, rhs))
    }
}

impl Shr<u32> for U1024 {
    type Output = Self;

    fn shr(self, rhs: u32) -> Self {
        Self(shr1024(&self.0, rhs))
    }
}

// Ordering traits
impl PartialOrd for U1024 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for U1024 {
    fn cmp(&self, other: &Self) -> Ordering {
        if lt1024(&self.0, &other.0) {
            Ordering::Less
        } else if eq1024(&self.0, &other.0) {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    }
}

// Display
impl core::fmt::Debug for U1024 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "U1024([{:016x}", self.0[15])?;
        for i in (0..15).rev() {
            write!(f, ", {:016x}", self.0[i])?;
        }
        write!(f, "])")
    }
}

impl core::fmt::Display for U1024 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "U1024({})", self.0[0])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_arithmetic() {
        let a = U1024::from_u64(100);
        let b = U1024::from_u64(50);

        let sum = a + b;
        assert_eq!(sum.0[0], 150);

        let diff = a - b;
        assert_eq!(diff.0[0], 50);

        let prod = a * b;
        assert_eq!(prod.0[0], 5000);
    }

    #[test]
    fn test_division() {
        let a = U1024::from_u64(107);
        let b = U1024::from_u64(10);

        let quotient = a / b;
        assert_eq!(quotient.0[0], 10);

        let remainder = a % b;
        assert_eq!(remainder.0[0], 7);
    }

    #[test]
    fn test_comparison() {
        let a = U1024::from_u64(100);
        let b = U1024::from_u64(50);

        assert!(a > b);
        assert!(b < a);
        assert_eq!(a, a);
    }

    #[test]
    fn test_bitwise() {
        let a = U1024::from_u64(0xFF);
        let b = U1024::from_u64(0x0F);

        let and = a & b;
        assert_eq!(and.0[0], 0x0F);

        let or = a | b;
        assert_eq!(or.0[0], 0xFF);

        let xor = a ^ b;
        assert_eq!(xor.0[0], 0xF0);
    }
}
