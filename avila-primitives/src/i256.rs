//! 256-bit signed integer type (two's complement)

use crate::u256::U256;
use core::cmp::Ordering;
use core::ops::{Add, Sub, Mul, Div, Rem, Neg};

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
    pub const fn is_negative(&self) -> bool {
        (self.0).0[3] & (1u64 << 63) != 0
    }

    /// Check if positive
    #[inline]
    pub const fn is_positive(&self) -> bool {
        !self.is_negative() && !self.is_zero()
    }

    /// Check if zero
    #[inline]
    pub const fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    /// Absolute value
    pub fn abs(&self) -> Self {
        if self.is_negative() {
            -*self
        } else {
            *self
        }
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

    /// Create from U256 (interpret as signed)
    pub const fn from_u256(value: U256) -> Self {
        Self(value)
    }

    /// To i64 with overflow check
    pub fn to_i64(&self) -> Option<i64> {
        if self.is_negative() {
            let abs = self.abs();
            if abs.0 .0[1] == 0 && abs.0 .0[2] == 0 && abs.0 .0[3] == 0 {
                let val = abs.0 .0[0];
                if val <= i64::MAX as u64 + 1 {
                    return Some(-(val as i64));
                }
            }
            None
        } else {
            if self.0 .0[1] == 0 && self.0 .0[2] == 0 && self.0 .0[3] == 0 {
                let val = self.0 .0[0];
                if val <= i64::MAX as u64 {
                    return Some(val as i64);
                }
            }
            None
        }
    }
}

// Arithmetic operations
impl Add for I256 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl Sub for I256 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}

impl Mul for I256 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let result = self.0 * rhs.0;
        Self(result)
    }
}

impl Div for I256 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        let neg_result = self.is_negative() ^ rhs.is_negative();
        let lhs_abs = self.abs().0;
        let rhs_abs = rhs.abs().0;
        let quotient = lhs_abs / rhs_abs;

        if neg_result {
            Self(!quotient + U256::ONE)
        } else {
            Self(quotient)
        }
    }
}

impl Rem for I256 {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self {
        let lhs_neg = self.is_negative();
        let lhs_abs = self.abs().0;
        let rhs_abs = rhs.abs().0;
        let remainder = lhs_abs % rhs_abs;

        if lhs_neg {
            Self(!remainder + U256::ONE)
        } else {
            Self(remainder)
        }
    }
}

impl Neg for I256 {
    type Output = Self;

    fn neg(self) -> Self {
        Self(!self.0 + U256::ONE)
    }
}

// Comparison
impl PartialOrd for I256 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for I256 {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.is_negative(), other.is_negative()) {
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
            (true, true) => other.0.cmp(&self.0), // Both negative, reverse order
            (false, false) => self.0.cmp(&other.0), // Both positive, normal order
        }
    }
}

// Display
impl core::fmt::Debug for I256 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.is_negative() {
            write!(f, "I256(-{:?})", self.abs().0)
        } else {
            write!(f, "I256({:?})", self.0)
        }
    }
}

impl core::fmt::Display for I256 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if let Some(val) = self.to_i64() {
            write!(f, "{}", val)
        } else if self.is_negative() {
            write!(f, "-{}", self.abs().0)
        } else {
            write!(f, "{}", self.0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_i64() {
        let pos = I256::from_i64(42);
        assert_eq!(pos.to_i64(), Some(42));

        let neg = I256::from_i64(-42);
        assert_eq!(neg.to_i64(), Some(-42));

        let zero = I256::from_i64(0);
        assert_eq!(zero.to_i64(), Some(0));
    }

    #[test]
    fn test_arithmetic() {
        let a = I256::from_i64(10);
        let b = I256::from_i64(5);

        assert_eq!((a + b).to_i64(), Some(15));
        assert_eq!((a - b).to_i64(), Some(5));
        assert_eq!((a * b).to_i64(), Some(50));
        assert_eq!((a / b).to_i64(), Some(2));
    }

    #[test]
    fn test_negative_arithmetic() {
        let a = I256::from_i64(-10);
        let b = I256::from_i64(5);

        assert_eq!((a + b).to_i64(), Some(-5));
        assert_eq!((a - b).to_i64(), Some(-15));
        assert_eq!((a * b).to_i64(), Some(-50));
        assert_eq!((a / b).to_i64(), Some(-2));
    }

    #[test]
    fn test_abs_neg() {
        let pos = I256::from_i64(42);
        let neg = I256::from_i64(-42);

        assert_eq!(pos.abs().to_i64(), Some(42));
        assert_eq!(neg.abs().to_i64(), Some(42));
        assert_eq!((-pos).to_i64(), Some(-42));
        assert_eq!((-neg).to_i64(), Some(42));
    }

    #[test]
    fn test_comparison() {
        let a = I256::from_i64(10);
        let b = I256::from_i64(5);
        let c = I256::from_i64(-5);

        assert!(a > b);
        assert!(b > c);
        assert!(c < b);
        assert!(a > c);
    }
}
