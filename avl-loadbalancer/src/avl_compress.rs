//! AVL Compress Module
//!
//! High-performance compression utilities for AVL Cloud Platform.
//! Provides gzip and Brotli compression with optimized defaults.

use anyhow::Result;
use std::io::Write;

/// Compression algorithm type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionAlgorithm {
    /// Gzip compression (widely supported, fast)
    Gzip,
    /// Brotli compression (better compression ratio, modern browsers)
    Brotli,
}

/// Compression level from 0 (no compression) to 9 (maximum compression)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompressionLevel(u8);

impl CompressionLevel {
    /// No compression
    pub const NONE: Self = Self(0);
    /// Fastest compression
    pub const FASTEST: Self = Self(1);
    /// Default compression (balanced speed/ratio)
    pub const DEFAULT: Self = Self(6);
    /// Maximum compression
    pub const MAX: Self = Self(9);

    /// Create custom compression level (0-9)
    pub fn new(level: u8) -> Self {
        Self(level.min(9))
    }

    pub fn value(&self) -> u8 {
        self.0
    }
}

impl Default for CompressionLevel {
    fn default() -> Self {
        Self::DEFAULT
    }
}

/// AVL Compression configuration
#[derive(Debug, Clone)]
pub struct AvlCompressor {
    level: CompressionLevel,
    algorithm: CompressionAlgorithm,
}

impl Default for AvlCompressor {
    fn default() -> Self {
        Self {
            level: CompressionLevel::DEFAULT,
            algorithm: CompressionAlgorithm::Gzip,
        }
    }
}

impl AvlCompressor {
    /// Create new compressor with algorithm
    pub fn new(algorithm: CompressionAlgorithm) -> Self {
        Self {
            level: CompressionLevel::DEFAULT,
            algorithm,
        }
    }

    /// Set compression level
    pub fn with_level(mut self, level: CompressionLevel) -> Self {
        self.level = level;
        self
    }

    /// Compress data with configured algorithm
    pub fn compress(&self, data: &[u8]) -> Result<Vec<u8>> {
        match self.algorithm {
            CompressionAlgorithm::Gzip => self.compress_gzip(data),
            CompressionAlgorithm::Brotli => self.compress_brotli(data),
        }
    }

    /// Compress with gzip
    fn compress_gzip(&self, data: &[u8]) -> Result<Vec<u8>> {
        use flate2::write::GzEncoder;
        use flate2::Compression;

        let mut encoder = GzEncoder::new(
            Vec::new(),
            Compression::new(self.level.value() as u32),
        );
        encoder.write_all(data)?;
        Ok(encoder.finish()?)
    }

    /// Compress with Brotli
    fn compress_brotli(&self, data: &[u8]) -> Result<Vec<u8>> {
        use brotli::enc::BrotliEncoderParams;

        let mut output = Vec::new();
        let mut params = BrotliEncoderParams::default();
        params.quality = self.level.value() as i32;

        brotli::BrotliCompress(
            &mut std::io::Cursor::new(data),
            &mut output,
            &params,
        )?;
        Ok(output)
    }

    /// Quick gzip compression with default settings
    pub fn gzip(data: &[u8]) -> Result<Vec<u8>> {
        Self::new(CompressionAlgorithm::Gzip).compress(data)
    }

    /// Quick brotli compression with default settings
    pub fn brotli(data: &[u8]) -> Result<Vec<u8>> {
        Self::new(CompressionAlgorithm::Brotli).compress(data)
    }

    /// Check if compression would be beneficial
    /// Returns true if data is large enough and compressible
    pub fn should_compress(data: &[u8], min_size: usize) -> bool {
        data.len() >= min_size
    }

    /// Compress only if beneficial (smaller result)
    pub fn compress_if_smaller(&self, data: &[u8]) -> Result<Vec<u8>> {
        let compressed = self.compress(data)?;
        if compressed.len() < data.len() {
            Ok(compressed)
        } else {
            Ok(data.to_vec())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gzip_compression() {
        let data = b"Hello, AVL Cloud Platform! ".repeat(100);
        let compressed = AvlCompressor::gzip(&data).unwrap();
        assert!(compressed.len() < data.len());
    }

    #[test]
    fn test_brotli_compression() {
        let data = b"Hello, AVL Cloud Platform! ".repeat(100);
        let compressed = AvlCompressor::brotli(&data).unwrap();
        assert!(compressed.len() < data.len());
    }

    #[test]
    fn test_compression_level() {
        let data = b"x".repeat(1000);
        
        let fast = AvlCompressor::new(CompressionAlgorithm::Gzip)
            .with_level(CompressionLevel::FASTEST)
            .compress(&data)
            .unwrap();
        
        let max = AvlCompressor::new(CompressionAlgorithm::Gzip)
            .with_level(CompressionLevel::MAX)
            .compress(&data)
            .unwrap();
        
        // Max compression should be smaller
        assert!(max.len() <= fast.len());
    }

    #[test]
    fn test_should_compress() {
        assert!(!AvlCompressor::should_compress(b"small", 1024));
        assert!(AvlCompressor::should_compress(&vec![0u8; 2048], 1024));
    }
}
