//! Compression utilities using avila-compress

use crate::error::{AvilaError, Result};

/// Compression level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionLevel {
    /// No compression
    None,
    /// Fast compression (level 1-3)
    Fast,
    /// Balanced compression (level 4-6)
    Balanced,
    /// Best compression (level 7-11)
    Best,
}

impl CompressionLevel {
    /// Convert to Brotli quality level (0-11)
    pub fn to_brotli_quality(&self) -> u32 {
        match self {
            CompressionLevel::None => 0,
            CompressionLevel::Fast => 3,
            CompressionLevel::Balanced => 6,
            CompressionLevel::Best => 11,
        }
    }
}

impl Default for CompressionLevel {
    fn default() -> Self {
        CompressionLevel::Balanced
    }
}

/// Compress data using Brotli
pub fn compress(data: &[u8], level: CompressionLevel) -> Result<Vec<u8>> {
    if level == CompressionLevel::None {
        return Ok(data.to_vec());
    }

    let quality = level.to_brotli_quality();

    let mut output = Vec::new();
    let mut compressor = brotli::CompressorReader::new(
        data,
        4096, // buffer size
        quality,
        22, // lgwin (window size)
    );

    std::io::copy(&mut compressor, &mut output)
        .map_err(|e| AvilaError::Compression(e.to_string()))?;

    Ok(output)
}

/// Decompress Brotli data
pub fn decompress(data: &[u8]) -> Result<Vec<u8>> {
    let mut output = Vec::new();
    let mut decompressor = brotli::Decompressor::new(
        data,
        4096, // buffer size
    );

    std::io::copy(&mut decompressor, &mut output)
        .map_err(|e| AvilaError::Compression(e.to_string()))?;

    Ok(output)
}

/// Calculate compression ratio
pub fn compression_ratio(original_size: usize, compressed_size: usize) -> f64 {
    if compressed_size == 0 {
        return 0.0;
    }
    original_size as f64 / compressed_size as f64
}

/// Compression statistics
#[derive(Debug, Clone)]
pub struct CompressionStats {
    pub original_size: usize,
    pub compressed_size: usize,
    pub ratio: f64,
    pub compression_time_us: u64,
}

/// Compress with statistics tracking
pub fn compress_with_stats(data: &[u8], level: CompressionLevel) -> Result<(Vec<u8>, CompressionStats)> {
    let start = std::time::Instant::now();
    let original_size = data.len();

    let compressed = compress(data, level)?;
    let compressed_size = compressed.len();

    let stats = CompressionStats {
        original_size,
        compressed_size,
        ratio: compression_ratio(original_size, compressed_size),
        compression_time_us: start.elapsed().as_micros() as u64,
    };

    Ok((compressed, stats))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compress_decompress() {
        let data = b"Hello, World! This is a test of compression.";
        let compressed = compress(data, CompressionLevel::Balanced).unwrap();
        let decompressed = decompress(&compressed).unwrap();

        assert_eq!(data.as_slice(), decompressed.as_slice());
    }

    #[test]
    fn test_compression_levels() {
        let data = b"A".repeat(1000);

        let fast = compress(&data, CompressionLevel::Fast).unwrap();
        let balanced = compress(&data, CompressionLevel::Balanced).unwrap();
        let best = compress(&data, CompressionLevel::Best).unwrap();

        // Best compression should produce smallest output
        assert!(best.len() <= balanced.len());
        assert!(balanced.len() <= fast.len());
    }

    #[test]
    fn test_compression_ratio() {
        let ratio = compression_ratio(1000, 500);
        assert_eq!(ratio, 2.0);

        let ratio = compression_ratio(1000, 1000);
        assert_eq!(ratio, 1.0);
    }

    #[test]
    fn test_compress_with_stats() {
        let data = b"Hello, World!".repeat(100);
        let (compressed, stats) = compress_with_stats(&data, CompressionLevel::Balanced).unwrap();

        assert_eq!(stats.original_size, data.len());
        assert_eq!(stats.compressed_size, compressed.len());
        assert!(stats.ratio > 1.0);
        assert!(stats.compression_time_us > 0);
    }

    #[test]
    fn test_no_compression() {
        let data = b"Hello, World!";
        let compressed = compress(data, CompressionLevel::None).unwrap();
        assert_eq!(data.as_slice(), compressed.as_slice());
    }
}
