//! 2048-bit unsigned integer type

use avila_nucleus::bits::u2048_ops::*;
use core::cmp::Ordering;
use core::ops::{Add, Sub, Mul, Div, Rem, BitAnd, BitOr, BitXor, Not, Shl, Shr};

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

    /// Create from u64
    pub const fn from_u64(n: u64) -> Self {
        let mut limbs = [0u64; 32];
        limbs[0] = n;
        Self(limbs)
    }

    /// Check if zero
    pub const fn is_zero(&self) -> bool {
        is_zero2048(&self.0)
    }

    /// Leading zeros count
    pub const fn leading_zeros(&self) -> u32 {
        leading_zeros2048(&self.0)
    }
}

// Arithmetic traits
impl Add for U2048 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let (result, _) = add2048(&self.0, &rhs.0);
        Self(result)
    }
}

impl Sub for U2048 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let (result, _) = sub2048(&self.0, &rhs.0);
        Self(result)
    }
}

impl Mul for U2048 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let result = mul2048x2048(&self.0, &rhs.0);
        let mut limbs = [0u64; 32];
        limbs.copy_from_slice(&result[0..32]);
        Self(limbs)
    }
}

impl Div for U2048 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        let (quotient, _) = div2048(&self.0, &rhs.0);
        Self(quotient)
    }
}

impl Rem for U2048 {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self {
        let (_, remainder) = div2048(&self.0, &rhs.0);
        Self(remainder)
    }
}

// Bitwise operations
impl BitAnd for U2048 {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        let mut result = [0u64; 32];
        for i in 0..32 {
            result[i] = self.0[i] & rhs.0[i];
        }
        Self(result)
    }
}

impl BitOr for U2048 {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        let mut result = [0u64; 32];
        for i in 0..32 {
            result[i] = self.0[i] | rhs.0[i];
        }
        Self(result)
    }
}

impl BitXor for U2048 {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self {
        let mut result = [0u64; 32];
        for i in 0..32 {
            result[i] = self.0[i] ^ rhs.0[i];
        }
        Self(result)
    }
}

impl Not for U2048 {
    type Output = Self;

    fn not(self) -> Self {
        let mut result = [0u64; 32];
        for i in 0..32 {
            result[i] = !self.0[i];
        }
        Self(result)
    }
}

// Shift operations
impl Shl<u32> for U2048 {
    type Output = Self;

    fn shl(self, rhs: u32) -> Self {
        Self(shl2048(&self.0, rhs))
    }
}

impl Shr<u32> for U2048 {
    type Output = Self;

    fn shr(self, rhs: u32) -> Self {
        Self(shr2048(&self.0, rhs))
    }
}

// Ordering traits
impl PartialOrd for U2048 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for U2048 {
    fn cmp(&self, other: &Self) -> Ordering {
        if lt2048(&self.0, &other.0) {
            Ordering::Less
        } else if eq2048(&self.0, &other.0) {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    }
}

// Display
impl core::fmt::Debug for U2048 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "U2048([{:016x}", self.0[31])?;
        for i in (0..31).rev() {
            write!(f, ", {:016x}", self.0[i])?;
        }
        write!(f, "])")
    }
}

impl core::fmt::Display for U2048 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "U2048({})", self.0[0])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_arithmetic() {
        let a = U2048::from_u64(100);
        let b = U2048::from_u64(50);

        let sum = a + b;
        assert_eq!(sum.0[0], 150);

        let diff = a - b;
        assert_eq!(diff.0[0], 50);

        let prod = a * b;
        assert_eq!(prod.0[0], 5000);
    }

    #[test]
    fn test_division() {
        let a = U2048::from_u64(107);
        let b = U2048::from_u64(10);

        let quotient = a / b;
        assert_eq!(quotient.0[0], 10);

        let remainder = a % b;
        assert_eq!(remainder.0[0], 7);
    }

    #[test]
    fn test_comparison() {
        let a = U2048::from_u64(100);
        let b = U2048::from_u64(50);

        assert!(a > b);
        assert!(b < a);
        assert_eq!(a, a);
    }
}
