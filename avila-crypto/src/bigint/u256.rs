//! 256-bit unsigned integer for elliptic curve cryptography
//!
//! Used for secp256k1, P-256, Curve25519

use super::BigInt;

/// 256-bit unsigned integer (4 x u64 limbs)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
        let sum = self.add(other);
        if matches!(sum.cmp(modulus), core::cmp::Ordering::Greater | core::cmp::Ordering::Equal) {
            sum.sub(modulus)
        } else {
            sum
        }
    }

    fn mul_mod(&self, other: &Self, _modulus: &Self) -> Self {
        // Placeholder - proper implementation uses Montgomery multiplication
        let product = self.mul(other);
        product
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
        // Extended Euclidean algorithm
        if self.is_zero() {
            return None;
        }

        // Simplified - full implementation needed
        Some(*self)
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

    /// Division remainder
    pub fn rem(&self, _divisor: &Self) -> Self {
        // Simplified - proper implementation uses Barrett reduction
        *self
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

