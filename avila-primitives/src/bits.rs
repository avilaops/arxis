//! Bit-level manipulation: BitSet and BitVec

use crate::types::Size;

/// Fixed-size bit set (256 bits)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitSet {
    data: [u64; 4],
}

impl BitSet {
    /// Number of bits
    pub const NUM_BITS: usize = 256;
    
    /// Create empty bit set
    pub const fn new() -> Self {
        Self { data: [0; 4] }
    }
    
    /// Create with all bits set
    pub const fn all_set() -> Self {
        Self { data: [u64::MAX; 4] }
    }
    
    /// Set a bit
    #[inline]
    pub fn set(&mut self, index: usize) {
        if index < Self::NUM_BITS {
            let word = index / 64;
            let bit = index % 64;
            self.data[word] |= 1u64 << bit;
        }
    }
    
    /// Clear a bit
    #[inline]
    pub fn clear(&mut self, index: usize) {
        if index < Self::NUM_BITS {
            let word = index / 64;
            let bit = index % 64;
            self.data[word] &= !(1u64 << bit);
        }
    }
    
    /// Test a bit
    #[inline]
    pub fn test(&self, index: usize) -> bool {
        if index < Self::NUM_BITS {
            let word = index / 64;
            let bit = index % 64;
            (self.data[word] & (1u64 << bit)) != 0
        } else {
            false
        }
    }
    
    /// Toggle a bit
    #[inline]
    pub fn toggle(&mut self, index: usize) {
        if index < Self::NUM_BITS {
            let word = index / 64;
            let bit = index % 64;
            self.data[word] ^= 1u64 << bit;
        }
    }
    
    /// Count set bits (population count)
    pub fn count_ones(&self) -> usize {
        let mut count = 0;
        let mut i = 0;
        while i < 4 {
            count += self.data[i].count_ones() as usize;
            i += 1;
        }
        count
    }
    
    /// Count clear bits
    pub fn count_zeros(&self) -> usize {
        Self::NUM_BITS - self.count_ones()
    }
    
    /// Check if all bits are clear
    pub fn is_empty(&self) -> bool {
        self.data[0] == 0 && self.data[1] == 0 && self.data[2] == 0 && self.data[3] == 0
    }
    
    /// Check if all bits are set
    pub fn is_full(&self) -> bool {
        self.data[0] == u64::MAX && self.data[1] == u64::MAX && 
        self.data[2] == u64::MAX && self.data[3] == u64::MAX
    }
    
    /// Bitwise AND
    pub fn and(&self, other: &Self) -> Self {
        let mut result = Self::new();
        let mut i = 0;
        while i < 4 {
            result.data[i] = self.data[i] & other.data[i];
            i += 1;
        }
        result
    }
    
    /// Bitwise OR
    pub fn or(&self, other: &Self) -> Self {
        let mut result = Self::new();
        let mut i = 0;
        while i < 4 {
            result.data[i] = self.data[i] | other.data[i];
            i += 1;
        }
        result
    }
    
    /// Bitwise XOR
    pub fn xor(&self, other: &Self) -> Self {
        let mut result = Self::new();
        let mut i = 0;
        while i < 4 {
            result.data[i] = self.data[i] ^ other.data[i];
            i += 1;
        }
        result
    }
    
    /// Bitwise NOT
    pub fn not(&self) -> Self {
        let mut result = Self::new();
        let mut i = 0;
        while i < 4 {
            result.data[i] = !self.data[i];
            i += 1;
        }
        result
    }
}

impl Default for BitSet {
    fn default() -> Self {
        Self::new()
    }
}

/// Dynamic bit vector (simplified, using array for no_std compatibility)
pub struct BitVec {
    data: [u64; 16], // 1024 bits max
    len: usize,      // Number of bits actually used
}

impl BitVec {
    /// Maximum capacity in bits
    pub const MAX_BITS: usize = 1024;
    
    /// Create new empty bit vector
    pub const fn new() -> Self {
        Self {
            data: [0; 16],
            len: 0,
        }
    }
    
    /// Get bit at index
    pub fn get(&self, index: usize) -> bool {
        if index >= self.len {
            return false;
        }
        
        let word = index / 64;
        let bit = index % 64;
        (self.data[word] & (1u64 << bit)) != 0
    }
    
    /// Set bit at index
    pub fn set(&mut self, index: usize) {
        if index >= Self::MAX_BITS {
            return;
        }
        
        if index >= self.len {
            self.len = index + 1;
        }
        
        let word = index / 64;
        let bit = index % 64;
        self.data[word] |= 1u64 << bit;
    }
    
    /// Clear bit at index
    pub fn clear(&mut self, index: usize) {
        if index >= self.len {
            return;
        }
        
        let word = index / 64;
        let bit = index % 64;
        self.data[word] &= !(1u64 << bit);
    }
    
    /// Push a bit
    pub fn push(&mut self, value: bool) {
        if self.len >= Self::MAX_BITS {
            return;
        }
        
        if value {
            self.set(self.len);
        } else {
            self.len += 1;
        }
    }
    
    /// Get length in bits
    pub const fn len(&self) -> usize {
        self.len
    }
    
    /// Check if empty
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }
    
    /// Clear all bits
    pub fn clear_all(&mut self) {
        self.data = [0; 16];
        self.len = 0;
    }
}

impl Default for BitVec {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitset_basic() {
        let mut bs = BitSet::new();
        assert!(bs.is_empty());
        
        bs.set(5);
        assert!(bs.test(5));
        assert!(!bs.test(4));
        
        bs.clear(5);
        assert!(!bs.test(5));
    }

    #[test]
    fn test_bitset_toggle() {
        let mut bs = BitSet::new();
        bs.toggle(10);
        assert!(bs.test(10));
        bs.toggle(10);
        assert!(!bs.test(10));
    }

    #[test]
    fn test_bitset_count() {
        let mut bs = BitSet::new();
        assert_eq!(bs.count_ones(), 0);
        assert_eq!(bs.count_zeros(), 256);
        
        bs.set(0);
        bs.set(1);
        bs.set(255);
        assert_eq!(bs.count_ones(), 3);
        assert_eq!(bs.count_zeros(), 253);
    }

    #[test]
    fn test_bitset_logical_ops() {
        let mut bs1 = BitSet::new();
        let mut bs2 = BitSet::new();
        
        bs1.set(1);
        bs1.set(3);
        bs2.set(2);
        bs2.set(3);
        
        let and_result = bs1.and(&bs2);
        assert!(and_result.test(3));
        assert!(!and_result.test(1));
        assert!(!and_result.test(2));
        
        let or_result = bs1.or(&bs2);
        assert!(or_result.test(1));
        assert!(or_result.test(2));
        assert!(or_result.test(3));
        
        let xor_result = bs1.xor(&bs2);
        assert!(xor_result.test(1));
        assert!(xor_result.test(2));
        assert!(!xor_result.test(3));
    }

    #[test]
    fn test_bitset_not() {
        let mut bs = BitSet::new();
        bs.set(0);
        
        let not_bs = bs.not();
        assert!(!not_bs.test(0));
        assert!(not_bs.test(1));
        assert!(not_bs.test(255));
    }

    #[test]
    fn test_bitset_full() {
        let bs = BitSet::all_set();
        assert!(bs.is_full());
        assert_eq!(bs.count_ones(), 256);
    }

    #[test]
    fn test_bitvec_basic() {
        let mut bv = BitVec::new();
        assert!(bv.is_empty());
        assert_eq!(bv.len(), 0);
        
        bv.push(true);
        bv.push(false);
        bv.push(true);
        
        assert_eq!(bv.len(), 3);
        assert!(bv.get(0));
        assert!(!bv.get(1));
        assert!(bv.get(2));
    }

    #[test]
    fn test_bitvec_set_clear() {
        let mut bv = BitVec::new();
        
        bv.set(10);
        assert!(bv.get(10));
        assert_eq!(bv.len(), 11);
        
        bv.clear(10);
        assert!(!bv.get(10));
    }
}
