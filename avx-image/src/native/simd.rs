//! SIMD-optimized operations
//!
//! Uses portable SIMD when available, with scalar fallback

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

#[cfg(target_arch = "aarch64")]
use std::arch::aarch64::*;

/// Check if AVX2 is available
#[inline]
pub fn has_avx2() -> bool {
    #[cfg(target_arch = "x86_64")]
    {
        is_x86_feature_detected!("avx2")
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        false
    }
}

/// Check if NEON is available (ARM)
#[inline]
pub fn has_neon() -> bool {
    #[cfg(target_arch = "aarch64")]
    {
        true // NEON is mandatory on AArch64
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        false
    }
}

/// Add two f32 arrays with SIMD
pub fn add_f32(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len());
    assert_eq!(a.len(), out.len());

    #[cfg(target_arch = "x86_64")]
    {
        if has_avx2() {
            unsafe { add_f32_avx2(a, b, out) };
            return;
        }
    }

    // Scalar fallback
    for i in 0..a.len() {
        out[i] = a[i] + b[i];
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
unsafe fn add_f32_avx2(a: &[f32], b: &[f32], out: &mut [f32]) {
    let len = a.len();
    let mut i = 0;

    // Process 8 elements at a time
    while i + 8 <= len {
        let va = _mm256_loadu_ps(a.as_ptr().add(i));
        let vb = _mm256_loadu_ps(b.as_ptr().add(i));
        let vout = _mm256_add_ps(va, vb);
        _mm256_storeu_ps(out.as_mut_ptr().add(i), vout);
        i += 8;
    }

    // Handle remainder
    while i < len {
        out[i] = a[i] + b[i];
        i += 1;
    }
}

/// Multiply two f32 arrays with SIMD
pub fn mul_f32(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len());
    assert_eq!(a.len(), out.len());

    #[cfg(target_arch = "x86_64")]
    {
        if has_avx2() {
            unsafe { mul_f32_avx2(a, b, out) };
            return;
        }
    }

    // Scalar fallback
    for i in 0..a.len() {
        out[i] = a[i] * b[i];
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
unsafe fn mul_f32_avx2(a: &[f32], b: &[f32], out: &mut [f32]) {
    let len = a.len();
    let mut i = 0;

    while i + 8 <= len {
        let va = _mm256_loadu_ps(a.as_ptr().add(i));
        let vb = _mm256_loadu_ps(b.as_ptr().add(i));
        let vout = _mm256_mul_ps(va, vb);
        _mm256_storeu_ps(out.as_mut_ptr().add(i), vout);
        i += 8;
    }

    while i < len {
        out[i] = a[i] * b[i];
        i += 1;
    }
}

/// Fused multiply-add: out = a * b + c
pub fn fma_f32(a: &[f32], b: &[f32], c: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len());
    assert_eq!(a.len(), c.len());
    assert_eq!(a.len(), out.len());

    #[cfg(target_arch = "x86_64")]
    {
        if has_avx2() && is_x86_feature_detected!("fma") {
            unsafe { fma_f32_avx2(a, b, c, out) };
            return;
        }
    }

    // Scalar fallback
    for i in 0..a.len() {
        out[i] = a[i].mul_add(b[i], c[i]);
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2,fma")]
unsafe fn fma_f32_avx2(a: &[f32], b: &[f32], c: &[f32], out: &mut [f32]) {
    let len = a.len();
    let mut i = 0;

    while i + 8 <= len {
        let va = _mm256_loadu_ps(a.as_ptr().add(i));
        let vb = _mm256_loadu_ps(b.as_ptr().add(i));
        let vc = _mm256_loadu_ps(c.as_ptr().add(i));
        let vout = _mm256_fmadd_ps(va, vb, vc);
        _mm256_storeu_ps(out.as_mut_ptr().add(i), vout);
        i += 8;
    }

    while i < len {
        out[i] = a[i].mul_add(b[i], c[i]);
        i += 1;
    }
}

/// Sum all elements in array (horizontal reduction)
pub fn sum_f32(data: &[f32]) -> f32 {
    #[cfg(target_arch = "x86_64")]
    {
        if has_avx2() {
            return unsafe { sum_f32_avx2(data) };
        }
    }

    // Scalar fallback
    data.iter().sum()
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
unsafe fn sum_f32_avx2(data: &[f32]) -> f32 {
    let len = data.len();
    let mut i = 0;
    let mut sum_vec = _mm256_setzero_ps();

    while i + 8 <= len {
        let v = _mm256_loadu_ps(data.as_ptr().add(i));
        sum_vec = _mm256_add_ps(sum_vec, v);
        i += 8;
    }

    // Horizontal reduction
    let mut sum_array = [0.0f32; 8];
    _mm256_storeu_ps(sum_array.as_mut_ptr(), sum_vec);
    let mut total = sum_array.iter().sum::<f32>();

    // Add remainder
    while i < len {
        total += data[i];
        i += 1;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_f32() {
        let a = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        let b = vec![9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0];
        let mut out = vec![0.0; 9];

        add_f32(&a, &b, &mut out);

        for i in 0..9 {
            assert_eq!(out[i], 10.0);
        }
    }

    #[test]
    fn test_mul_f32() {
        let a = vec![2.0; 10];
        let b = vec![3.0; 10];
        let mut out = vec![0.0; 10];

        mul_f32(&a, &b, &mut out);

        for &val in &out {
            assert_eq!(val, 6.0);
        }
    }

    #[test]
    fn test_sum_f32() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let sum = sum_f32(&data);
        assert_eq!(sum, 55.0);
    }
}
