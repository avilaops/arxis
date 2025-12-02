//! 4096-bit unsigned integer for RSA-4096

use super::BigInt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct U4096 {
    pub limbs: [u64; 64],
}

impl BigInt for U4096 {
    const LIMBS: usize = 64;
    const BITS: usize = 4096;
    const ZERO: Self = Self { limbs: [0; 64] };
    const ONE: Self = Self { limbs: [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] };

    fn from_bytes_be(_bytes: &[u8]) -> Self { Self::ZERO }
    fn add_mod(&self, _other: &Self, _modulus: &Self) -> Self { *self }
    fn mul_mod(&self, _other: &Self, _modulus: &Self) -> Self { *self }
    fn pow_mod(&self, _exp: &Self, _modulus: &Self) -> Self { *self }
    fn inv_mod(&self, _modulus: &Self) -> Option<Self> { Some(*self) }
}
