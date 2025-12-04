//! 512-bit unsigned integer (used internally for wide multiplication)

use super::BigInt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct U512 {
    pub limbs: [u64; 8],
}

impl BigInt for U512 {
    const LIMBS: usize = 8;
    const BITS: usize = 512;
    const ZERO: Self = Self { limbs: [0; 8] };
    const ONE: Self = Self { limbs: [1, 0, 0, 0, 0, 0, 0, 0] };

    fn from_bytes_be(_bytes: &[u8]) -> Self { Self::ZERO }
    fn add_mod(&self, _other: &Self, _modulus: &Self) -> Self { *self }
    fn mul_mod(&self, _other: &Self, _modulus: &Self) -> Self { *self }
    fn pow_mod(&self, _exp: &Self, _modulus: &Self) -> Self { *self }
    fn inv_mod(&self, _modulus: &Self) -> Option<Self> { Some(*self) }
}

impl U512 {
    /// Reduce 512-bit value modulo 256-bit modulus using multi-limb elimination.
    pub fn reduce(&self, modulus: &super::U256) -> super::U256 {
        let mut acc = self.limbs;

        for idx in (super::U256::LIMBS..Self::LIMBS).rev() {
            let limb = acc[idx];
            if limb == 0 {
                continue;
            }

            subtract_mul_shift(&mut acc, modulus, limb, idx - super::U256::LIMBS);
            acc[idx] = 0;
        }

        super::U256 {
            limbs: [
                acc[0],
                acc[1],
                acc[2],
                acc[3],
            ],
        }
        .normalize(modulus)
    }

    fn shl(&self, bits: u32) -> Self {
        if bits >= 512 {
            return Self::ZERO;
        }
        let limb_shift = (bits / 64) as usize;
        let bit_shift = bits % 64;
        let mut result = Self::ZERO;

        for i in limb_shift..8 {
            result.limbs[i] = self.limbs[i - limb_shift] << bit_shift;
            if bit_shift > 0 && i > limb_shift {
                result.limbs[i] |= self.limbs[i - limb_shift - 1] >> (64 - bit_shift);
            }
        }
        result
    }

    fn shr(&self, bits: u32) -> Self {
        if bits >= 512 {
            return Self::ZERO;
        }
        let limb_shift = (bits / 64) as usize;
        let bit_shift = bits % 64;
        let mut result = Self::ZERO;

        for i in 0..(8 - limb_shift) {
            result.limbs[i] = self.limbs[i + limb_shift] >> bit_shift;
            if bit_shift > 0 && i + limb_shift + 1 < 8 {
                result.limbs[i] |= self.limbs[i + limb_shift + 1] << (64 - bit_shift);
            }
        }
        result
    }

    fn sub(&self, other: &Self) -> Self {
        let mut result = Self::ZERO;
        let mut borrow = 0u64;

        for i in 0..8 {
            let (diff, b1) = self.limbs[i].overflowing_sub(other.limbs[i]);
            let (diff, b2) = diff.overflowing_sub(borrow);
            result.limbs[i] = diff;
            borrow = (b1 as u64) + (b2 as u64);
        }
        result
    }

    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        for i in (0..8).rev() {
            match self.limbs[i].cmp(&other.limbs[i]) {
                core::cmp::Ordering::Equal => continue,
                ord => return ord,
            }
        }
        core::cmp::Ordering::Equal
    }
}

fn subtract_mul_shift(acc: &mut [u64; U512::LIMBS], modulus: &super::U256, factor: u64, shift: usize) {
    let mut borrow = 0u128;

    for (j, &mod_limb) in modulus.limbs.iter().enumerate() {
        let idx = j + shift;
        let product = (mod_limb as u128) * (factor as u128) + borrow;
        let sub = product as u64;
        borrow = product >> 64;

        let (res, b) = acc[idx].overflowing_sub(sub);
        acc[idx] = res;
        borrow += b as u128;
    }

    let mut idx = modulus.limbs.len() + shift;
    let mut iterations = 0;
    while borrow != 0 {
        iterations += 1;
        if iterations > 100 {
            // Prevent infinite loop
            break;
        }
        debug_assert!(idx < acc.len(), "borrow exceeded accumulator length");
        let sub = borrow as u64;
        let (res, b) = acc[idx].overflowing_sub(sub);
        acc[idx] = res;
        borrow = (borrow >> 64) + (b as u128);
        idx += 1;
    }

    debug_assert!(borrow == 0, "residual borrow after reduction");
}
