//! Core trait for fast random number generators

/// Fast RNG trait providing common interface for all generators
pub trait FastRng {
    /// Generate next random u64
    fn next_u64(&mut self) -> u64;
    
    /// Generate random u32
    #[inline]
    fn next_u32(&mut self) -> u32 {
        (self.next_u64() >> 32) as u32
    }
    
    /// Generate random u16
    #[inline]
    fn next_u16(&mut self) -> u16 {
        (self.next_u64() >> 48) as u16
    }
    
    /// Generate random u8
    #[inline]
    fn next_u8(&mut self) -> u8 {
        (self.next_u64() >> 56) as u8
    }
    
    /// Generate random bool
    #[inline]
    fn next_bool(&mut self) -> bool {
        (self.next_u64() & 1) == 1
    }
    
    /// Generate random float in [0, 1)
    #[inline]
    fn next_f64(&mut self) -> f64 {
        // Use 53 bits of precision (IEEE 754 double has 52-bit mantissa)
        let val = self.next_u64() >> 11;
        (val as f64) * (1.0 / ((1u64 << 53) as f64))
    }
    
    /// Generate random float in [0, 1)
    #[inline]
    fn next_f32(&mut self) -> f32 {
        // Use 24 bits of precision (IEEE 754 float has 23-bit mantissa)
        let val = self.next_u32() >> 8;
        (val as f32) * (1.0 / ((1u32 << 24) as f32))
    }
    
    /// Generate value in range [min, max)
    #[inline]
    fn gen_range(&mut self, min: u64, max: u64) -> u64 {
        assert!(min < max, "min must be less than max");
        let range = max - min;
        min + (self.next_u64() % range)
    }
    
    /// Fill buffer with random bytes
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        let mut i = 0;
        while i + 8 <= dest.len() {
            let val = self.next_u64();
            dest[i..i+8].copy_from_slice(&val.to_le_bytes());
            i += 8;
        }
        
        // Handle remaining bytes
        if i < dest.len() {
            let val = self.next_u64();
            let bytes = val.to_le_bytes();
            let remaining = dest.len() - i;
            dest[i..].copy_from_slice(&bytes[..remaining]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Pcg64;

    #[test]
    fn test_next_u32() {
        let mut rng = Pcg64::new(42);
        let v = rng.next_u32();
        assert!(v <= u32::MAX);
    }

    #[test]
    fn test_next_bool() {
        let mut rng = Pcg64::new(42);
        let _b1 = rng.next_bool();
        let _b2 = rng.next_bool();
        // Just ensure it doesn't panic
    }

    #[test]
    fn test_next_f64() {
        let mut rng = Pcg64::new(42);
        let f = rng.next_f64();
        assert!(f >= 0.0 && f < 1.0);
    }

    #[test]
    fn test_gen_range() {
        let mut rng = Pcg64::new(42);
        for _ in 0..100 {
            let v = rng.gen_range(10, 20);
            assert!(v >= 10 && v < 20);
        }
    }

    #[test]
    fn test_fill_bytes() {
        let mut rng = Pcg64::new(42);
        let mut buf = [0u8; 100];
        rng.fill_bytes(&mut buf);
        
        // Check that buffer was actually filled (not all zeros)
        let all_zero = buf.iter().all(|&x| x == 0);
        assert!(!all_zero);
    }

    #[test]
    #[should_panic(expected = "min must be less than max")]
    fn test_gen_range_invalid() {
        let mut rng = Pcg64::new(42);
        let _ = rng.gen_range(20, 10);
    }
}
