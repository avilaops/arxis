//! Zero-copy bytes buffer
//!
//! Similar to bytes crate but with zero external dependencies

use std::ops::{Deref, Range};
use std::sync::Arc;

/// Reference-counted byte buffer with zero-copy slicing
#[derive(Clone)]
pub struct Bytes {
    data: Arc<Vec<u8>>,
    offset: usize,
    len: usize,
}

impl Bytes {
    /// Create empty bytes
    pub fn new() -> Self {
        Self {
            data: Arc::new(Vec::new()),
            offset: 0,
            len: 0,
        }
    }

    /// Create from vector
    pub fn from_vec(vec: Vec<u8>) -> Self {
        let len = vec.len();
        Self {
            data: Arc::new(vec),
            offset: 0,
            len,
        }
    }

    /// Create from slice (copies data)
    pub fn copy_from_slice(slice: &[u8]) -> Self {
        Self::from_vec(slice.to_vec())
    }

    /// Get length
    pub fn len(&self) -> usize {
        self.len
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Zero-copy slice
    pub fn slice(&self, range: Range<usize>) -> Self {
        assert!(range.end <= self.len, "Range out of bounds");

        Self {
            data: Arc::clone(&self.data),
            offset: self.offset + range.start,
            len: range.end - range.start,
        }
    }

    /// Get as slice
    pub fn as_slice(&self) -> &[u8] {
        &self.data[self.offset..self.offset + self.len]
    }

    /// Convert to vector (copies if shared)
    pub fn to_vec(&self) -> Vec<u8> {
        self.as_slice().to_vec()
    }
}

impl Deref for Bytes {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl From<Vec<u8>> for Bytes {
    fn from(vec: Vec<u8>) -> Self {
        Self::from_vec(vec)
    }
}

impl From<&[u8]> for Bytes {
    fn from(slice: &[u8]) -> Self {
        Self::copy_from_slice(slice)
    }
}

impl From<String> for Bytes {
    fn from(s: String) -> Self {
        Self::from_vec(s.into_bytes())
    }
}

impl From<&str> for Bytes {
    fn from(s: &str) -> Self {
        Self::copy_from_slice(s.as_bytes())
    }
}

impl Default for Bytes {
    fn default() -> Self {
        Self::new()
    }
}

impl AsRef<[u8]> for Bytes {
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl PartialEq for Bytes {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl Eq for Bytes {}

impl std::fmt::Debug for Bytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Bytes({} bytes)", self.len)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bytes_new() {
        let bytes = Bytes::new();
        assert_eq!(bytes.len(), 0);
        assert!(bytes.is_empty());
    }

    #[test]
    fn test_bytes_from_vec() {
        let vec = vec![1, 2, 3, 4];
        let bytes = Bytes::from_vec(vec);
        assert_eq!(bytes.len(), 4);
        assert_eq!(bytes.as_slice(), &[1, 2, 3, 4]);
    }

    #[test]
    fn test_bytes_slice() {
        let bytes = Bytes::from_vec(vec![1, 2, 3, 4, 5]);
        let slice = bytes.slice(1..4);

        assert_eq!(slice.len(), 3);
        assert_eq!(slice.as_slice(), &[2, 3, 4]);
    }

    #[test]
    fn test_bytes_zero_copy() {
        let original = Bytes::from_vec(vec![1, 2, 3, 4, 5]);
        let slice = original.slice(1..4);

        // Both should share the same underlying data
        assert_eq!(Arc::strong_count(&original.data), 2);
    }

    #[test]
    fn test_bytes_from_string() {
        let bytes = Bytes::from("hello");
        assert_eq!(bytes.as_slice(), b"hello");
    }
}
