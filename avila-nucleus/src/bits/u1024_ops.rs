//! Operações U1024 (16 limbs × 64-bit)
//!
//! Aritmética de 1024-bit para RSA-1024.

use super::u64_ops::*;

/// Adição U1024: a + b → (soma, carry)
#[inline]
pub const fn add1024(a: &[u64; 16], b: &[u64; 16]) -> ([u64; 16], u64) {
    let mut result = [0u64; 16];
    let mut carry = 0u64;

    let mut i = 0;
    while i < 16 {
        let (r, c) = adc(a[i], b[i], carry);
        result[i] = r;
        carry = c;
        i += 1;
    }

    (result, carry)
}

/// Subtração U1024: a - b → (diferença, borrow)
#[inline]
pub const fn sub1024(a: &[u64; 16], b: &[u64; 16]) -> ([u64; 16], u64) {
    let mut result = [0u64; 16];
    let mut borrow = 0u64;

    let mut i = 0;
    while i < 16 {
        let (r, b) = sbb(a[i], b[i], borrow);
        result[i] = r;
        borrow = b;
        i += 1;
    }

    (result, borrow)
}

/// Multiplicação U1024 × U64 → U1088
#[inline]
pub const fn mul1024x64(a: &[u64; 16], b: u64) -> [u64; 17] {
    let mut result = [0u64; 17];
    let mut carry = 0u64;

    let mut i = 0;
    while i < 16 {
        let (lo, hi) = mul_wide(a[i], b);
        let (sum, c) = adc(lo, carry, 0);
        result[i] = sum;
        carry = hi + c;
        i += 1;
    }

    result[16] = carry;
    result
}

/// Shift left U1024
#[inline]
pub const fn shl1024(a: &[u64; 16], bits: u32) -> [u64; 16] {
    if bits == 0 {
        return *a;
    }

    let mut result = [0u64; 16];
    let shift_right = 64 - bits;

    result[0] = a[0] << bits;

    let mut i = 1;
    while i < 16 {
        result[i] = (a[i] << bits) | (a[i - 1] >> shift_right);
        i += 1;
    }

    result
}

/// Shift right U1024
#[inline]
pub const fn shr1024(a: &[u64; 16], bits: u32) -> [u64; 16] {
    if bits == 0 {
        return *a;
    }

    let mut result = [0u64; 16];
    let shift_left = 64 - bits;

    result[15] = a[15] >> bits;

    let mut i = 14;
    loop {
        result[i] = (a[i] >> bits) | (a[i + 1] << shift_left);
        if i == 0 {
            break;
        }
        i -= 1;
    }

    result
}

/// Comparações U1024
#[inline]
pub const fn eq1024(a: &[u64; 16], b: &[u64; 16]) -> bool {
    let mut i = 0;
    while i < 16 {
        if a[i] != b[i] {
            return false;
        }
        i += 1;
    }
    true
}

#[inline]
pub const fn lt1024(a: &[u64; 16], b: &[u64; 16]) -> bool {
    let mut i = 15;
    loop {
        if a[i] < b[i] {
            return true;
        }
        if a[i] > b[i] {
            return false;
        }
        if i == 0 {
            break;
        }
        i -= 1;
    }
    false
}

#[inline]
pub const fn is_zero1024(a: &[u64; 16]) -> bool {
    let mut acc = 0u64;
    let mut i = 0;
    while i < 16 {
        acc |= a[i];
        i += 1;
    }
    acc == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add1024() {
        let a = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let b = [2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let (sum, carry) = add1024(&a, &b);
        assert_eq!(sum[0], 3);
        assert_eq!(carry, 0);
    }

    #[test]
    fn test_comparisons() {
        let a = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let b = [2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        assert!(lt1024(&a, &b));
        assert!(eq1024(&a, &a));
    }
}
