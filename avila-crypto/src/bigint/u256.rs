//! 256-bit unsigned integer for elliptic curve cryptography
//!
//! Used for secp256k1, P-256, Curve25519

use super::{BigInt, U512};

/// 256-bit unsigned integer (4 x u64 limbs)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(align(32))]
pub struct U256 {
    /// Little-endian limbs
    pub limbs: [u64; 4],
}

impl BigInt for U256 {
    const LIMBS: usize = 4;
    const BITS: usize = 256;
    const ZERO: Self = Self { limbs: [0; 4] };
    const ONE: Self = Self { limbs: [1, 0, 0, 0] };

    fn from_bytes_be(bytes: &[u8]) -> Self {
        assert!(bytes.len() <= 32, "Input too large for U256");

        let mut limbs = [0u64; 4];
        let mut offset = bytes.len();

        for i in 0..4 {
            if offset == 0 {
                break;
            }

            let chunk_size = core::cmp::min(8, offset);
            let start = offset - chunk_size;

            let mut limb = 0u64;
            for j in 0..chunk_size {
                limb = (limb << 8) | (bytes[start + j] as u64);
            }

            limbs[i] = limb;
            offset = start;
        }

        Self { limbs }
    }

    fn add_mod(&self, other: &Self, modulus: &Self) -> Self {
        self.add_wide(other).reduce(modulus)
    }

    fn mul_mod(&self, other: &Self, modulus: &Self) -> Self {
        // Full 512-bit multiplication then reduce
        let product = self.mul_wide(other);
        product.reduce(modulus)
    }

    fn pow_mod(&self, exp: &Self, modulus: &Self) -> Self {
        let mut result = Self::ONE;
        let mut base = *self;
        let mut e = *exp;

        while !e.is_zero() {
            if e.limbs[0] & 1 == 1 {
                result = result.mul_mod(&base, modulus);
            }
            base = base.mul_mod(&base, modulus);
            e = e.shr(1);
        }

        result
    }

    fn inv_mod(&self, modulus: &Self) -> Option<Self> {
        if self.is_zero() || modulus.is_zero() {
            return None;
        }

        // Fermat's little theorem: a^(p-1) ≡ 1 (mod p) for prime p
        // Therefore a^(p-2) ≡ a^{-1} (mod p) when gcd(a, p) = 1.
        let two = Self { limbs: [2, 0, 0, 0] };
        if matches!(modulus.cmp(&two), core::cmp::Ordering::Less | core::cmp::Ordering::Equal) {
            return None;
        }

        let exponent = modulus.sub(&two);
        Some(self.pow_mod(&exponent, modulus))
    }
}

impl U256 {
    /// Addition with overflow detection
    pub fn add(&self, other: &Self) -> Self {
        let mut result = Self::ZERO;
        let mut carry = 0u64;

        for i in 0..4 {
            let (sum, c1) = self.limbs[i].overflowing_add(other.limbs[i]);
            let (sum, c2) = sum.overflowing_add(carry);
            result.limbs[i] = sum;
            carry = (c1 as u64) + (c2 as u64);
        }

        result
    }

    /// Subtraction
    pub fn sub(&self, other: &Self) -> Self {
        let mut result = Self::ZERO;
        let mut borrow = 0u64;

        for i in 0..4 {
            let (diff, b1) = self.limbs[i].overflowing_sub(other.limbs[i]);
            let (diff, b2) = diff.overflowing_sub(borrow);
            result.limbs[i] = diff;
            borrow = (b1 as u64) + (b2 as u64);
        }

        result
    }

    /// Multiplication (returns lower 256 bits)
    pub fn mul(&self, other: &Self) -> Self {
        let mut result = Self::ZERO;

        for i in 0..4 {
            let mut carry = 0u128;
            for j in 0..4 {
                if i + j < 4 {
                    let product = (self.limbs[i] as u128) * (other.limbs[j] as u128)
                                + (result.limbs[i + j] as u128)
                                + carry;
                    result.limbs[i + j] = product as u64;
                    carry = product >> 64;
                }
            }
        }

        result
    }

    /// Division with remainder
    pub fn div_rem(&self, divisor: &Self) -> (Self, Self) {
        if divisor.is_zero() {
            panic!("division by zero");
        }
        let mut remainder = Self::ZERO;
        let mut quotient = Self::ZERO;
        for i in (0..256).rev() {
            remainder = remainder.shl(1);
            let limb_idx = i / 64;
            let bit_idx = i % 64;
            if (self.limbs[limb_idx] & (1u64 << bit_idx)) != 0 {
                remainder.limbs[0] |= 1;
            }
            if remainder.cmp(divisor) != core::cmp::Ordering::Less {
                remainder = remainder.sub(divisor);
                quotient.limbs[limb_idx] |= 1u64 << bit_idx;
            }
        }
        (quotient, remainder)
    }

    /// Right shift
    pub fn shr(&self, bits: u32) -> Self {
        if bits >= 256 {
            return Self::ZERO;
        }

        let limb_shift = (bits / 64) as usize;
        let bit_shift = bits % 64;

        let mut result = Self::ZERO;

        for i in 0..(4 - limb_shift) {
            result.limbs[i] = self.limbs[i + limb_shift] >> bit_shift;
            if bit_shift > 0 && i + limb_shift + 1 < 4 {
                result.limbs[i] |= self.limbs[i + limb_shift + 1] << (64 - bit_shift);
            }
        }

        result
    }

    /// Check if zero
    pub fn is_zero(&self) -> bool {
        self.limbs.iter().all(|&x| x == 0)
    }

    /// Check if even
    pub fn is_even(&self) -> bool {
        (self.limbs[0] & 1) == 0
    }

    /// Left shift
    pub fn shl(&self, bits: u32) -> Self {
        if bits >= 256 {
            return Self::ZERO;
        }

        let limb_shift = (bits / 64) as usize;
        let bit_shift = bits % 64;
        let mut result = Self::ZERO;

        for i in limb_shift..4 {
            result.limbs[i] = self.limbs[i - limb_shift] << bit_shift;
            if bit_shift > 0 && i > limb_shift {
                result.limbs[i] |= self.limbs[i - limb_shift - 1] >> (64 - bit_shift);
            }
        }

        result
    }

    /// Wide multiplication (returns full 512 bits as U512)
    fn mul_wide(&self, other: &Self) -> U512 {
        let mut result = [0u64; 8];

        for i in 0..4 {
            let mut carry = 0u128;
            for j in 0..4 {
                let index = i + j;
                let product = (self.limbs[i] as u128) * (other.limbs[j] as u128)
                    + (result[index] as u128)
                    + carry;
                result[index] = product as u64;
                carry = product >> 64;
            }

            let mut index = i + 4;
            while carry != 0 && index < 8 {
                let sum = (result[index] as u128) + carry;
                result[index] = sum as u64;
                carry = sum >> 64;
                index += 1;
            }
        }

        U512 { limbs: result }
    }

    /// Wide addition (returns 512-bit sum)
    fn add_wide(&self, other: &Self) -> U512 {
        let mut result = [0u64; 8];
        let mut carry = 0u128;

        for i in 0..4 {
            let sum = self.limbs[i] as u128 + other.limbs[i] as u128 + carry;
            result[i] = sum as u64;
            carry = sum >> 64;
        }

        result[4] = carry as u64;
        U512 { limbs: result }
    }

    /// Comparison
    pub fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        for i in (0..4).rev() {
            match self.limbs[i].cmp(&other.limbs[i]) {
                core::cmp::Ordering::Equal => continue,
                ord => return ord,
            }
        }
        core::cmp::Ordering::Equal
    }

    pub fn normalize(mut self, modulus: &Self) -> Self {
        while self.cmp(modulus) != core::cmp::Ordering::Less {
            self = self.sub(modulus);
        }
        self
    }

    /// Wrapping addition
    pub fn wrapping_add(&self, other: &Self) -> Self {
        self.add(other) // Already handles overflow
    }

    /// Wrapping subtraction
    pub fn wrapping_sub(&self, other: &Self) -> Self {
        self.sub(other) // Already handles underflow
    }

    /// Wrapping multiplication (lower 256 bits)
    pub fn wrapping_mul(&self, other: &Self) -> Self {
        self.mul(other)
    }

    /// To little-endian bytes
    pub fn to_bytes_le(&self) -> [u8; 32] {
        let mut bytes = [0u8; 32];
        for i in 0..4 {
            let limb = self.limbs[i];
            for j in 0..8 {
                bytes[i * 8 + j] = (limb >> (j * 8)) as u8;
            }
        }
        bytes
    }

    /// From little-endian bytes
    pub fn from_bytes_le(bytes: &[u8; 32]) -> Self {
        let mut limbs = [0u64; 4];
        for i in 0..4 {
            for j in 0..8 {
                limbs[i] |= (bytes[i * 8 + j] as u64) << (j * 8);
            }
        }
        Self { limbs }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_bytes_be() {
        let bytes = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
        ];
        let n = U256::from_bytes_be(&bytes);
        assert_eq!(n, U256::ONE);
    }

    #[test]
    fn test_addition() {
        let a = U256::ONE;
        let b = U256::ONE;
        let c = a.add(&b);
        assert_eq!(c.limbs[0], 2);
    }
}

