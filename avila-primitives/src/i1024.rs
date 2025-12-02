//! 1024-bit signed integer type (two's complement)

use crate::u1024::U1024;
use core::cmp::Ordering;
use core::ops::{Add, Sub, Mul, Div, Rem, Neg};

/// 1024-bit signed integer (two's complement)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct I1024(pub U1024);

impl I1024 {
    /// Zero value
    pub const ZERO: Self = Self(U1024::ZERO);

    /// One value
    pub const ONE: Self = Self(U1024::ONE);

    /// Negative one
    pub const NEG_ONE: Self = Self(U1024::MAX);

    /// Check if negative
    #[inline]
    pub const fn is_negative(&self) -> bool {
        (self.0).0[15] & (1u64 << 63) != 0
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
        if self.is_negative() { -*self } else { *self }
    }

    /// Create from i64
    pub fn from_i64(value: i64) -> Self {
        if value >= 0 {
            Self(U1024::from_u64(value as u64))
        } else {
            let abs = value.unsigned_abs();
            Self(!U1024::from_u64(abs) + U1024::ONE)
        }
    }
}

impl Add for I1024 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self { Self(self.0 + rhs.0) }
}

impl Sub for I1024 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self { Self(self.0 - rhs.0) }
}

impl Mul for I1024 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self { Self(self.0 * rhs.0) }
}

impl Div for I1024 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        let neg = self.is_negative() ^ rhs.is_negative();
        let q = self.abs().0 / rhs.abs().0;
        if neg { Self(!q + U1024::ONE) } else { Self(q) }
    }
}

impl Rem for I1024 {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self {
        let r = self.abs().0 % rhs.abs().0;
        if self.is_negative() { Self(!r + U1024::ONE) } else { Self(r) }
    }
}

impl Neg for I1024 {
    type Output = Self;
    fn neg(self) -> Self { Self(!self.0 + U1024::ONE) }
}

impl PartialOrd for I1024 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

impl Ord for I1024 {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.is_negative(), other.is_negative()) {
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
            (true, true) => other.0.cmp(&self.0),
            (false, false) => self.0.cmp(&other.0),
        }
    }
}

impl core::fmt::Debug for I1024 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.is_negative() {
            write!(f, "I1024(-)")
        } else {
            write!(f, "I1024(+)")
        }
    }
}

impl core::fmt::Display for I1024 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "I1024({})", self.0 .0[0])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arithmetic() {
        let a = I1024::from_i64(10);
        let b = I1024::from_i64(-5);
        assert_eq!((a + b).0 .0[0], 5);
    }
}
