//! 2048-bit signed integer type (two's complement)

use crate::u2048::U2048;
use core::cmp::Ordering;
use core::ops::{Add, Sub, Mul, Div, Rem, Neg};

/// 2048-bit signed integer (two's complement)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct I2048(pub U2048);

impl I2048 {
    /// Zero value
    pub const ZERO: Self = Self(U2048::ZERO);

    /// One value
    pub const ONE: Self = Self(U2048::ONE);

    /// Negative one
    pub const NEG_ONE: Self = Self(U2048::MAX);

    /// Check if negative
    #[inline]
    pub const fn is_negative(&self) -> bool {
        (self.0).0[31] & (1u64 << 63) != 0
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
            Self(U2048::from_u64(value as u64))
        } else {
            Self(!U2048::from_u64(value.unsigned_abs()) + U2048::ONE)
        }
    }
}

impl Add for I2048 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self { Self(self.0 + rhs.0) }
}

impl Sub for I2048 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self { Self(self.0 - rhs.0) }
}

impl Mul for I2048 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self { Self(self.0 * rhs.0) }
}

impl Div for I2048 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        let neg = self.is_negative() ^ rhs.is_negative();
        let q = self.abs().0 / rhs.abs().0;
        if neg { Self(!q + U2048::ONE) } else { Self(q) }
    }
}

impl Rem for I2048 {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self {
        let r = self.abs().0 % rhs.abs().0;
        if self.is_negative() { Self(!r + U2048::ONE) } else { Self(r) }
    }
}

impl Neg for I2048 {
    type Output = Self;
    fn neg(self) -> Self { Self(!self.0 + U2048::ONE) }
}

impl PartialOrd for I2048 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

impl Ord for I2048 {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.is_negative(), other.is_negative()) {
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
            (true, true) => other.0.cmp(&self.0),
            (false, false) => self.0.cmp(&other.0),
        }
    }
}

impl core::fmt::Debug for I2048 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "I2048")
    }
}

impl core::fmt::Display for I2048 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "I2048({})", self.0 .0[0])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let a = I2048::from_i64(10);
        let b = I2048::from_i64(-10);
        assert!(a > b);
    }
}
