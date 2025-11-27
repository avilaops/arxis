//! Operações U4096 (64 limbs × 64-bit)
//!
//! Aritmética de 4096-bit para RSA-4096 e aplicações avançadas.

use super::u64_ops::*;

/// Adição U4096
#[inline]
pub fn add4096(a: &[u64; 64], b: &[u64; 64]) -> ([u64; 64], u64) {
    let mut result = [0u64; 64];
    let mut carry = 0u64;

    for i in 0..64 {
        let (r, c) = adc(a[i], b[i], carry);
        result[i] = r;
        carry = c;
    }

    (result, carry)
}

/// Subtração U4096
#[inline]
pub fn sub4096(a: &[u64; 64], b: &[u64; 64]) -> ([u64; 64], u64) {
    let mut result = [0u64; 64];
    let mut borrow = 0u64;

    for i in 0..64 {
        let (r, b) = sbb(a[i], b[i], borrow);
        result[i] = r;
        borrow = b;
    }

    (result, borrow)
}

/// Multiplicação U4096 × U64
#[inline]
pub fn mul4096x64(a: &[u64; 64], b: u64) -> [u64; 65] {
    let mut result = [0u64; 65];
    let mut carry = 0u64;

    for i in 0..64 {
        let (lo, hi) = mul_wide(a[i], b);
        let (sum, c) = adc(lo, carry, 0);
        result[i] = sum;
        carry = hi + c;
    }

    result[64] = carry;
    result
}

/// Comparações U4096
#[inline]
pub fn eq4096(a: &[u64; 64], b: &[u64; 64]) -> bool {
    a.iter().zip(b.iter()).all(|(x, y)| x == y)
}

#[inline]
pub fn lt4096(a: &[u64; 64], b: &[u64; 64]) -> bool {
    for i in (0..64).rev() {
        if a[i] < b[i] {
            return true;
        }
        if a[i] > b[i] {
            return false;
        }
    }
    false
}

#[inline]
pub fn is_zero4096(a: &[u64; 64]) -> bool {
    a.iter().all(|&x| x == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add4096() {
        let mut a = [0u64; 64];
        let mut b = [0u64; 64];
        a[0] = 1;
        b[0] = 2;

        let (sum, carry) = add4096(&a, &b);
        assert_eq!(sum[0], 3);
        assert_eq!(carry, 0);
    }
}
