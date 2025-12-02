//! 512-bit signed integer type (two's complement)

use crate::u512::U512;
use core::cmp::Ordering;
use core::ops::{Add, Sub, Mul, Div, Rem, Neg};

/// 512-bit signed integer (two's complement)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct I512(pub U512);

impl I512 {
    /// Zero value
    pub const ZERO: Self = Self(U512::ZERO);

    /// One value
    pub const ONE: Self = Self(U512::ONE);

    /// Negative one
    pub const NEG_ONE: Self = Self(U512::MAX);

    /// Minimum value
    pub const MIN: Self = Self(U512([0, 0, 0, 0, 0, 0, 0, 1u64 << 63]));

    /// Maximum value
    pub const MAX: Self = Self(U512([u64::MAX, u64::MAX, u64::MAX, u64::MAX, u64::MAX, u64::MAX, u64::MAX, (1u64 << 63) - 1]));

    /// Check if negative
    #[inline]
    pub const fn is_negative(&self) -> bool {
        (self.0).0[7] & (1u64 << 63) != 0
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
            Self(U512::from_u64(value as u64))
        } else {
            let abs = value.unsigned_abs();
            let mut result = U512::from_u64(abs);
            result = !result;
            result = result + U512::ONE;
            Self(result)
        }
    }
}

impl Add for I512 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl Sub for I512 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}

impl Mul for I512 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self(self.0 * rhs.0)
    }
}

impl Div for I512 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        let neg_result = self.is_negative() ^ rhs.is_negative();
        let lhs_abs = self.abs().0;
        let rhs_abs = rhs.abs().0;
        let quotient = lhs_abs / rhs_abs;
        if neg_result {
            Self(!quotient + U512::ONE)
        } else {
            Self(quotient)
        }
    }
}

impl Rem for I512 {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self {
        let lhs_neg = self.is_negative();
        let lhs_abs = self.abs().0;
        let rhs_abs = rhs.abs().0;
        let remainder = lhs_abs % rhs_abs;
        if lhs_neg {
            Self(!remainder + U512::ONE)
        } else {
            Self(remainder)
        }
    }
}

impl Neg for I512 {
    type Output = Self;
    fn neg(self) -> Self {
        Self(!self.0 + U512::ONE)
    }
}

impl PartialOrd for I512 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for I512 {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.is_negative(), other.is_negative()) {
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
            (true, true) => other.0.cmp(&self.0),
            (false, false) => self.0.cmp(&other.0),
        }
    }
}

impl core::fmt::Debug for I512 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.is_negative() {
            write!(f, "I512(-{:?})", self.abs().0)
        } else {
            write!(f, "I512({:?})", self.0)
        }
    }
}

impl core::fmt::Display for I512 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.is_negative() {
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
    fn test_arithmetic() {
        let a = I512::from_i64(10);
        let b = I512::from_i64(5);
        let c = I512::from_i64(-5);

        assert_eq!((a + b).0 .0[0], 15);
        assert_eq!((a - b).0 .0[0], 5);
        assert_eq!((a + c).0 .0[0], 5);
    }

    #[test]
    fn test_comparison() {
        let a = I512::from_i64(10);
        let b = I512::from_i64(-10);
        assert!(a > b);
        assert!(b < a);
    }
}
