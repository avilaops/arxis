//! Range generation utilities for random numbers

use crate::traits::FastRng;

/// Generate random u64 in range [min, max)
#[inline]
pub fn gen_range_u64<R: FastRng>(rng: &mut R, min: u64, max: u64) -> u64 {
    assert!(min < max, "min must be less than max");
    
    let range = max - min;
    
    // Use rejection sampling to avoid modulo bias
    // This ensures uniform distribution
    if range.is_power_of_two() {
        // Fast path for power of 2
        min + (rng.next_u64() & (range - 1))
    } else {
        // Lemire's nearly divisionless algorithm
        let mut m = (rng.next_u64() as u128) * (range as u128);
        let mut leftover = (m & 0xFFFFFFFFFFFFFFFF) as u64;
        
        if leftover < range {
            let threshold = range.wrapping_neg() % range;
            while leftover < threshold {
                m = (rng.next_u64() as u128) * (range as u128);
                leftover = (m & 0xFFFFFFFFFFFFFFFF) as u64;
            }
        }
        
        min + (m >> 64) as u64
    }
}

/// Generate random u32 in range [min, max)
#[inline]
pub fn gen_range_u32<R: FastRng>(rng: &mut R, min: u32, max: u32) -> u32 {
    assert!(min < max, "min must be less than max");
    
    let range = max - min;
    
    if range.is_power_of_two() {
        min + (rng.next_u32() & (range - 1))
    } else {
        // Lemire's algorithm for u32
        let mut m = (rng.next_u32() as u64) * (range as u64);
        let mut leftover = (m & 0xFFFFFFFF) as u32;
        
        if leftover < range {
            let threshold = range.wrapping_neg() % range;
            while leftover < threshold {
                m = (rng.next_u32() as u64) * (range as u64);
                leftover = (m & 0xFFFFFFFF) as u32;
            }
        }
        
        min + (m >> 32) as u32
    }
}

/// Generate random usize in range [min, max)
#[inline]
pub fn gen_range_usize<R: FastRng>(rng: &mut R, min: usize, max: usize) -> usize {
    #[cfg(target_pointer_width = "64")]
    {
        gen_range_u64(rng, min as u64, max as u64) as usize
    }
    
    #[cfg(target_pointer_width = "32")]
    {
        gen_range_u32(rng, min as u32, max as u32) as usize
    }
}

/// Generate random float in range [min, max)
#[inline]
pub fn gen_range_f64<R: FastRng>(rng: &mut R, min: f64, max: f64) -> f64 {
    assert!(min < max, "min must be less than max");
    let t = rng.next_f64();
    min + t * (max - min)
}

/// Generate random float in range [min, max)
#[inline]
pub fn gen_range_f32<R: FastRng>(rng: &mut R, min: f32, max: f32) -> f32 {
    assert!(min < max, "min must be less than max");
    let t = rng.next_f32();
    min + t * (max - min)
}

/// Shuffle a slice in place
#[inline]
pub fn shuffle<T, R: FastRng>(slice: &mut [T], rng: &mut R) {
    let len = slice.len();
    for i in (1..len).rev() {
        let j = gen_range_usize(rng, 0, i + 1);
        slice.swap(i, j);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Pcg64;

    #[test]
    fn test_gen_range_u64() {
        let mut rng = Pcg64::new(42);
        for _ in 0..1000 {
            let v = gen_range_u64(&mut rng, 10, 20);
            assert!(v >= 10 && v < 20);
        }
    }

    #[test]
    fn test_gen_range_u64_power_of_two() {
        let mut rng = Pcg64::new(42);
        for _ in 0..1000 {
            let v = gen_range_u64(&mut rng, 0, 256);
            assert!(v < 256);
        }
    }

    #[test]
    fn test_gen_range_u32() {
        let mut rng = Pcg64::new(42);
        for _ in 0..1000 {
            let v = gen_range_u32(&mut rng, 100, 200);
            assert!(v >= 100 && v < 200);
        }
    }

    #[test]
    fn test_gen_range_usize() {
        let mut rng = Pcg64::new(42);
        for _ in 0..1000 {
            let v = gen_range_usize(&mut rng, 5, 15);
            assert!(v >= 5 && v < 15);
        }
    }

    #[test]
    fn test_gen_range_f64() {
        let mut rng = Pcg64::new(42);
        for _ in 0..1000 {
            let v = gen_range_f64(&mut rng, 1.0, 10.0);
            assert!(v >= 1.0 && v < 10.0);
        }
    }

    #[test]
    fn test_gen_range_f32() {
        let mut rng = Pcg64::new(42);
        for _ in 0..1000 {
            let v = gen_range_f32(&mut rng, 0.5, 5.5);
            assert!(v >= 0.5 && v < 5.5);
        }
    }

    #[test]
    fn test_shuffle() {
        let mut rng = Pcg64::new(42);
        let mut arr = [1, 2, 3, 4, 5];
        let original = arr;
        
        shuffle(&mut arr, &mut rng);
        
        // Array should be modified
        assert_ne!(arr, original);
        
        // All elements should still be present
        arr.sort();
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }

    #[test]
    #[should_panic(expected = "min must be less than max")]
    fn test_gen_range_u64_invalid() {
        let mut rng = Pcg64::new(42);
        let _ = gen_range_u64(&mut rng, 20, 10);
    }

    #[test]
    fn test_gen_range_u64_uniform() {
        let mut rng = Pcg64::new(42);
        let mut buckets = [0u32; 10];
        
        for _ in 0..10000 {
            let v = gen_range_u64(&mut rng, 0, 10);
            buckets[v as usize] += 1;
        }
        
        // Each bucket should have roughly 1000 items (±30%)
        for count in buckets.iter() {
            assert!(*count > 700 && *count < 1300);
        }
    }
}
