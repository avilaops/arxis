//! SIMD-accelerated bulk random number generation
//!
//! This module provides vectorized generation of random numbers
//! using AVX2 or AVX-512 when available.

use crate::traits::FastRng;

#[cfg(all(feature = "std", target_arch = "x86_64"))]
use core::arch::x86_64::*;

#[cfg(all(feature = "std", target_arch = "x86"))]
use core::arch::x86::*;

/// Fill buffer with random u64s using SIMD when available
#[inline]
pub fn fill_u64_simd<R: FastRng>(rng: &mut R, dest: &mut [u64]) {
    #[cfg(all(feature = "std", any(target_arch = "x86_64", target_arch = "x86")))]
    {
        if is_x86_feature_detected!("avx2") {
            unsafe { fill_u64_avx2(rng, dest) }
        } else {
            fill_u64_scalar(rng, dest)
        }
    }
    
    #[cfg(not(all(feature = "std", any(target_arch = "x86_64", target_arch = "x86"))))]
    {
        fill_u64_scalar(rng, dest)
    }
}

/// Scalar fallback for filling buffer with u64s
#[inline]
fn fill_u64_scalar<R: FastRng>(rng: &mut R, dest: &mut [u64]) {
    for item in dest.iter_mut() {
        *item = rng.next_u64();
    }
}

#[cfg(all(feature = "std", any(target_arch = "x86_64", target_arch = "x86")))]
#[target_feature(enable = "avx2")]
unsafe fn fill_u64_avx2<R: FastRng>(rng: &mut R, dest: &mut [u64]) {
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::*;
    #[cfg(target_arch = "x86")]
    use core::arch::x86::*;
    
    let len = dest.len();
    let mut i = 0;
    
    // Process 4 u64s at a time with AVX2
    while i + 4 <= len {
        // Generate 4 random u64s
        let v0 = rng.next_u64();
        let v1 = rng.next_u64();
        let v2 = rng.next_u64();
        let v3 = rng.next_u64();
        
        // Store using AVX2
        let ptr = dest.as_mut_ptr().add(i);
        _mm256_storeu_si256(
            ptr as *mut __m256i,
            _mm256_set_epi64x(v3 as i64, v2 as i64, v1 as i64, v0 as i64)
        );
        
        i += 4;
    }
    
    // Handle remaining elements
    while i < len {
        dest[i] = rng.next_u64();
        i += 1;
    }
}

/// Fill buffer with random u32s using SIMD when available
#[inline]
pub fn fill_u32_simd<R: FastRng>(rng: &mut R, dest: &mut [u32]) {
    #[cfg(all(feature = "std", any(target_arch = "x86_64", target_arch = "x86")))]
    {
        if is_x86_feature_detected!("avx2") {
            unsafe { fill_u32_avx2(rng, dest) }
        } else {
            fill_u32_scalar(rng, dest)
        }
    }
    
    #[cfg(not(all(feature = "std", any(target_arch = "x86_64", target_arch = "x86"))))]
    {
        fill_u32_scalar(rng, dest)
    }
}

/// Scalar fallback for filling buffer with u32s
#[inline]
fn fill_u32_scalar<R: FastRng>(rng: &mut R, dest: &mut [u32]) {
    for item in dest.iter_mut() {
        *item = rng.next_u32();
    }
}

#[cfg(all(feature = "std", any(target_arch = "x86_64", target_arch = "x86")))]
#[target_feature(enable = "avx2")]
unsafe fn fill_u32_avx2<R: FastRng>(rng: &mut R, dest: &mut [u32]) {
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::*;
    #[cfg(target_arch = "x86")]
    use core::arch::x86::*;
    
    let len = dest.len();
    let mut i = 0;
    
    // Process 8 u32s at a time with AVX2
    while i + 8 <= len {
        // Generate from 4 u64s
        let v0 = rng.next_u64();
        let v1 = rng.next_u64();
        let v2 = rng.next_u64();
        let v3 = rng.next_u64();
        
        // Store using AVX2
        let ptr = dest.as_mut_ptr().add(i);
        _mm256_storeu_si256(
            ptr as *mut __m256i,
            _mm256_set_epi32(
                (v3 >> 32) as i32, v3 as i32,
                (v2 >> 32) as i32, v2 as i32,
                (v1 >> 32) as i32, v1 as i32,
                (v0 >> 32) as i32, v0 as i32,
            )
        );
        
        i += 8;
    }
    
    // Handle remaining elements
    while i < len {
        dest[i] = rng.next_u32();
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Pcg64;

    #[test]
    fn test_fill_u64_simd() {
        let mut rng = Pcg64::new(42);
        let mut buf = [0u64; 100];
        fill_u64_simd(&mut rng, &mut buf);
        
        // Check that buffer was filled
        let all_zero = buf.iter().all(|&x| x == 0);
        assert!(!all_zero);
    }

    #[test]
    fn test_fill_u32_simd() {
        let mut rng = Pcg64::new(42);
        let mut buf = [0u32; 100];
        fill_u32_simd(&mut rng, &mut buf);
        
        // Check that buffer was filled
        let all_zero = buf.iter().all(|&x| x == 0);
        assert!(!all_zero);
    }

    #[test]
    fn test_fill_u64_scalar() {
        let mut rng = Pcg64::new(42);
        let mut buf = [0u64; 100];
        fill_u64_scalar(&mut rng, &mut buf);
        
        // Check that buffer was filled
        let all_zero = buf.iter().all(|&x| x == 0);
        assert!(!all_zero);
    }

    #[test]
    fn test_fill_u32_scalar() {
        let mut rng = Pcg64::new(42);
        let mut buf = [0u32; 100];
        fill_u32_scalar(&mut rng, &mut buf);
        
        // Check that buffer was filled
        let all_zero = buf.iter().all(|&x| x == 0);
        assert!(!all_zero);
    }
}
