//! Numeric wrapper types and semantic types

/// Single byte (8 bits)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Byte(pub u8);

impl Byte {
    /// Zero value
    pub const ZERO: Self = Self(0);
    
    /// Maximum value
    pub const MAX: Self = Self(u8::MAX);
    
    /// Minimum value
    pub const MIN: Self = Self(u8::MIN);
    
    /// Create from u8
    #[inline]
    pub const fn new(value: u8) -> Self {
        Self(value)
    }
    
    /// Get inner value
    #[inline]
    pub const fn get(self) -> u8 {
        self.0
    }
    
    /// Create from bits
    #[inline]
    pub const fn from_bits(bits: [bool; 8]) -> Self {
        let mut value = 0u8;
        let mut i = 0;
        while i < 8 {
            if bits[i] {
                value |= 1 << i;
            }
            i += 1;
        }
        Self(value)
    }
}

/// Word (16 bits)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Word(pub u16);

impl Word {
    /// Zero value
    pub const ZERO: Self = Self(0);
    
    /// Maximum value
    pub const MAX: Self = Self(u16::MAX);
    
    /// Minimum value
    pub const MIN: Self = Self(u16::MIN);
    
    /// Create from u16
    #[inline]
    pub const fn new(value: u16) -> Self {
        Self(value)
    }
    
    /// Get inner value
    #[inline]
    pub const fn get(self) -> u16 {
        self.0
    }
    
    /// Split into bytes (little-endian)
    #[inline]
    pub const fn to_bytes_le(self) -> [Byte; 2] {
        [
            Byte((self.0 & 0xFF) as u8),
            Byte((self.0 >> 8) as u8),
        ]
    }
    
    /// Create from bytes (little-endian)
    #[inline]
    pub const fn from_bytes_le(bytes: [Byte; 2]) -> Self {
        Self((bytes[0].0 as u16) | ((bytes[1].0 as u16) << 8))
    }
}

/// Double word (32 bits)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct DWord(pub u32);

impl DWord {
    /// Zero value
    pub const ZERO: Self = Self(0);
    
    /// Maximum value
    pub const MAX: Self = Self(u32::MAX);
    
    /// Minimum value
    pub const MIN: Self = Self(u32::MIN);
    
    /// Create from u32
    #[inline]
    pub const fn new(value: u32) -> Self {
        Self(value)
    }
    
    /// Get inner value
    #[inline]
    pub const fn get(self) -> u32 {
        self.0
    }
    
    /// Split into words (little-endian)
    #[inline]
    pub const fn to_words_le(self) -> [Word; 2] {
        [
            Word((self.0 & 0xFFFF) as u16),
            Word((self.0 >> 16) as u16),
        ]
    }
    
    /// Create from words (little-endian)
    #[inline]
    pub const fn from_words_le(words: [Word; 2]) -> Self {
        Self((words[0].0 as u32) | ((words[1].0 as u32) << 16))
    }
}

/// Quad word (64 bits)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct QWord(pub u64);

impl QWord {
    /// Zero value
    pub const ZERO: Self = Self(0);
    
    /// Maximum value
    pub const MAX: Self = Self(u64::MAX);
    
    /// Minimum value
    pub const MIN: Self = Self(u64::MIN);
    
    /// Create from u64
    #[inline]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }
    
    /// Get inner value
    #[inline]
    pub const fn get(self) -> u64 {
        self.0
    }
    
    /// Split into dwords (little-endian)
    #[inline]
    pub const fn to_dwords_le(self) -> [DWord; 2] {
        [
            DWord((self.0 & 0xFFFFFFFF) as u32),
            DWord((self.0 >> 32) as u32),
        ]
    }
    
    /// Create from dwords (little-endian)
    #[inline]
    pub const fn from_dwords_le(dwords: [DWord; 2]) -> Self {
        Self((dwords[0].0 as u64) | ((dwords[1].0 as u64) << 32))
    }
}

/// Index type - for array/vector indexing
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Index(pub usize);

impl Index {
    /// Zero index
    pub const ZERO: Self = Self(0);
    
    /// Create from usize
    #[inline]
    pub const fn new(value: usize) -> Self {
        Self(value)
    }
    
    /// Get inner value
    #[inline]
    pub const fn get(self) -> usize {
        self.0
    }
    
    /// Increment by one
    #[inline]
    pub const fn next(self) -> Self {
        Self(self.0.wrapping_add(1))
    }
    
    /// Decrement by one
    #[inline]
    pub const fn prev(self) -> Self {
        Self(self.0.wrapping_sub(1))
    }
    
    /// Add offset
    #[inline]
    pub const fn offset_by(self, offset: Offset) -> Self {
        Self(self.0.wrapping_add(offset.0))
    }
}

/// Offset type - for memory offsets and displacements
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Offset(pub usize);

impl Offset {
    /// Zero offset
    pub const ZERO: Self = Self(0);
    
    /// Create from usize
    #[inline]
    pub const fn new(value: usize) -> Self {
        Self(value)
    }
    
    /// Get inner value
    #[inline]
    pub const fn get(self) -> usize {
        self.0
    }
    
    /// Add offset
    #[inline]
    pub const fn add(self, other: Offset) -> Self {
        Self(self.0.wrapping_add(other.0))
    }
    
    /// Subtract offset
    #[inline]
    pub const fn sub(self, other: Offset) -> Self {
        Self(self.0.wrapping_sub(other.0))
    }
}

/// Size type - for memory sizes and lengths
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Size(pub usize);

impl Size {
    /// Zero size
    pub const ZERO: Self = Self(0);
    
    /// Create from usize
    #[inline]
    pub const fn new(value: usize) -> Self {
        Self(value)
    }
    
    /// Get inner value
    #[inline]
    pub const fn get(self) -> usize {
        self.0
    }
    
    /// Check if zero
    #[inline]
    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }
    
    /// Add sizes
    #[inline]
    pub const fn add(self, other: Size) -> Self {
        Self(self.0.wrapping_add(other.0))
    }
    
    /// Subtract sizes
    #[inline]
    pub const fn sub(self, other: Size) -> Self {
        Self(self.0.wrapping_sub(other.0))
    }
    
    /// Multiply by scalar
    #[inline]
    pub const fn mul(self, factor: usize) -> Self {
        Self(self.0.wrapping_mul(factor))
    }
    
    /// Convert to offset
    #[inline]
    pub const fn as_offset(self) -> Offset {
        Offset(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_byte_basic() {
        let b = Byte::new(42);
        assert_eq!(b.get(), 42);
        assert_eq!(Byte::ZERO.get(), 0);
        assert_eq!(Byte::MAX.get(), 255);
    }

    #[test]
    fn test_byte_from_bits() {
        let bits = [true, false, true, false, true, false, true, false];
        let b = Byte::from_bits(bits);
        assert_eq!(b.get(), 0b10101010);
    }

    #[test]
    fn test_word_bytes() {
        let w = Word::new(0x1234);
        let bytes = w.to_bytes_le();
        assert_eq!(bytes[0].get(), 0x34);
        assert_eq!(bytes[1].get(), 0x12);
        
        let w2 = Word::from_bytes_le(bytes);
        assert_eq!(w, w2);
    }

    #[test]
    fn test_dword_words() {
        let dw = DWord::new(0x12345678);
        let words = dw.to_words_le();
        assert_eq!(words[0].get(), 0x5678);
        assert_eq!(words[1].get(), 0x1234);
        
        let dw2 = DWord::from_words_le(words);
        assert_eq!(dw, dw2);
    }

    #[test]
    fn test_qword_dwords() {
        let qw = QWord::new(0x123456789ABCDEF0);
        let dwords = qw.to_dwords_le();
        assert_eq!(dwords[0].get(), 0x9ABCDEF0);
        assert_eq!(dwords[1].get(), 0x12345678);
        
        let qw2 = QWord::from_dwords_le(dwords);
        assert_eq!(qw, qw2);
    }

    #[test]
    fn test_index_operations() {
        let idx = Index::new(10);
        assert_eq!(idx.next().get(), 11);
        assert_eq!(idx.prev().get(), 9);
        
        let offset = Offset::new(5);
        assert_eq!(idx.offset_by(offset).get(), 15);
    }

    #[test]
    fn test_offset_operations() {
        let off1 = Offset::new(10);
        let off2 = Offset::new(5);
        assert_eq!(off1.add(off2).get(), 15);
        assert_eq!(off1.sub(off2).get(), 5);
    }

    #[test]
    fn test_size_operations() {
        let s1 = Size::new(10);
        let s2 = Size::new(5);
        assert_eq!(s1.add(s2).get(), 15);
        assert_eq!(s1.sub(s2).get(), 5);
        assert_eq!(s1.mul(3).get(), 30);
        assert!(!s1.is_zero());
        assert!(Size::ZERO.is_zero());
    }
}
