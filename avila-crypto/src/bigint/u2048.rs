//! 2048-bit unsigned integer for RSA

use super::BigInt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct U2048 {
    pub limbs: [u64; 32],
}

impl BigInt for U2048 {
    const LIMBS: usize = 32;
    const BITS: usize = 2048;
    const ZERO: Self = Self { limbs: [0; 32] };
    const ONE: Self = Self { limbs: [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] };

    fn from_bytes_be(_bytes: &[u8]) -> Self { Self::ZERO }
    fn add_mod(&self, _other: &Self, _modulus: &Self) -> Self { *self }
    fn mul_mod(&self, _other: &Self, _modulus: &Self) -> Self { *self }
    fn pow_mod(&self, _exp: &Self, _modulus: &Self) -> Self { *self }
    fn inv_mod(&self, _modulus: &Self) -> Option<Self> { Some(*self) }
}
