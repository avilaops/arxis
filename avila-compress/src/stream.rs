//! Streaming compression API for incremental processing
//!
//! This module provides stateful encoders and decoders that can process
//! data in chunks, making it suitable for:
//! - Network streaming (HTTP chunked transfer)
//! - Large file processing (avoid loading entire file)
//! - Real-time compression (logs, telemetry)
//!
//! ## Example
//! ```rust
//! use avila_compress::stream::Lz4Encoder;
//!
//! let mut encoder = Lz4Encoder::new();
//!
//! // Process data in chunks
//! let chunk1 = encoder.write(b"Hello, ").unwrap();
//! let chunk2 = encoder.write(b"World!").unwrap();
//! let final_chunk = encoder.finish().unwrap();
//!
//! // Combine all compressed chunks
//! let mut compressed = Vec::new();
//! compressed.extend_from_slice(&chunk1);
//! compressed.extend_from_slice(&chunk2);
//! compressed.extend_from_slice(&final_chunk);
//! ```

use crate::{Error, Result};

const BLOCK_SIZE: usize = 64 * 1024; // 64 KB blocks
const MIN_MATCH: usize = 4;
const HASH_LOG: usize = 12;
const HASH_TABLE_SIZE: usize = 1 << HASH_LOG;
const MAX_DISTANCE: usize = 65535;

/// Streaming LZ4 encoder
///
/// Processes data incrementally without loading everything into memory.
/// Maintains compression state across multiple `write()` calls.
pub struct Lz4Encoder {
    /// Hash table for match finding (persisted across chunks)
    hash_table: Vec<i32>,
    /// Input buffer (accumulates until block size reached)
    input_buffer: Vec<u8>,
    /// Output buffer for compressed data
    output_buffer: Vec<u8>,
    /// Total bytes processed (for position tracking)
    total_processed: usize,
    /// Original size accumulator
    original_size: usize,
}

impl Lz4Encoder {
    /// Create a new streaming encoder
    pub fn new() -> Self {
        Self {
            hash_table: vec![-1; HASH_TABLE_SIZE],
            input_buffer: Vec::with_capacity(BLOCK_SIZE),
            output_buffer: Vec::with_capacity(BLOCK_SIZE + 1024),
            total_processed: 0,
            original_size: 0,
        }
    }

    /// Write a chunk of data to compress
    ///
    /// Returns compressed output when internal buffer is full.
    /// May return empty Vec if not enough data accumulated yet.
    ///
    /// # Arguments
    /// * `data` - Input chunk to compress
    ///
    /// # Returns
    /// Compressed bytes (may be empty if buffering)
    pub fn write(&mut self, data: &[u8]) -> Result<Vec<u8>> {
        if data.is_empty() {
            return Ok(Vec::new());
        }

        self.original_size += data.len();
        self.input_buffer.extend_from_slice(data);

        // Process full blocks
        if self.input_buffer.len() >= BLOCK_SIZE {
            let result = self.compress_buffered()?;
            Ok(result)
        } else {
            Ok(Vec::new())
        }
    }

    /// Finish compression and flush remaining data
    ///
    /// Must be called after all `write()` calls to ensure all data is compressed.
    ///
    /// # Returns
    /// Final compressed bytes including header
    pub fn finish(&mut self) -> Result<Vec<u8>> {
        // Compress any remaining buffered data
        if !self.input_buffer.is_empty() {
            self.compress_buffered()?;
        }

        // Prepend header with original size
        let mut result = Vec::with_capacity(self.output_buffer.len() + 4);
        result.extend_from_slice(&(self.original_size as u32).to_le_bytes());
        result.append(&mut self.output_buffer);

        Ok(result)
    }

    /// Compress currently buffered data
    fn compress_buffered(&mut self) -> Result<Vec<u8>> {
        if self.input_buffer.is_empty() {
            return Ok(Vec::new());
        }

        let input = &self.input_buffer;
        let mut anchor = 0;
        let mut pos = 0;
        let input_end = input.len();
        let input_limit = if input_end > 5 { input_end - 5 } else { 0 };

        while pos < input_limit {
            let mut match_found = false;
            let mut match_pos = 0;
            let mut match_len = 0;

            // Try to find match
            if pos + MIN_MATCH <= input_end {
                let hash = self.hash4(&input[pos..]);
                let candidate = self.hash_table[hash];

                if candidate >= 0 {
                    let candidate_pos = candidate as usize;
                    let distance = (self.total_processed + pos) - candidate_pos;

                    if distance > 0 && distance <= MAX_DISTANCE {
                        let max_match = input_end - pos;
                        let len = self.count_match(&input[candidate_pos..], &input[pos..], max_match);

                        if len >= MIN_MATCH {
                            match_found = true;
                            match_pos = candidate_pos;
                            match_len = len;
                        }
                    }
                }

                self.hash_table[hash] = (self.total_processed + pos) as i32;
            }

            if match_found {
                // Emit literal + match
                let literal_len = pos - anchor;

                let lit_token = if literal_len >= 15 { 15 } else { literal_len };
                let match_token = if match_len >= MIN_MATCH + 15 {
                    15
                } else {
                    match_len - MIN_MATCH
                };
                self.output_buffer.push(((lit_token << 4) | match_token) as u8);

                if literal_len >= 15 {
                    let mut remaining = literal_len - 15;
                    while remaining >= 255 {
                        self.output_buffer.push(255);
                        remaining -= 255;
                    }
                    self.output_buffer.push(remaining as u8);
                }

                self.output_buffer.extend_from_slice(&input[anchor..pos]);

                let offset = ((self.total_processed + pos) - match_pos) as u16;
                self.output_buffer.extend_from_slice(&offset.to_le_bytes());

                if match_len >= MIN_MATCH + 15 {
                    let mut remaining = match_len - MIN_MATCH - 15;
                    while remaining >= 255 {
                        self.output_buffer.push(255);
                        remaining -= 255;
                    }
                    self.output_buffer.push(remaining as u8);
                }

                pos += match_len;
                anchor = pos;
            } else {
                pos += 1;
            }
        }

        // Emit remaining literals
        let final_literals = input_end - anchor;
        if final_literals > 0 {
            let lit_token = if final_literals >= 15 { 15 } else { final_literals };
            self.output_buffer.push((lit_token << 4) as u8);

            if final_literals >= 15 {
                let mut remaining = final_literals - 15;
                while remaining >= 255 {
                    self.output_buffer.push(255);
                    remaining -= 255;
                }
                self.output_buffer.push(remaining as u8);
            }

            self.output_buffer.extend_from_slice(&input[anchor..]);
        }

        self.total_processed += input.len();
        self.input_buffer.clear();

        Ok(Vec::new()) // Data buffered in output_buffer
    }

    #[inline]
    fn hash4(&self, data: &[u8]) -> usize {
        if data.len() < 4 {
            return 0;
        }
        let value = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
        ((value.wrapping_mul(2654435761)) >> (32 - HASH_LOG)) as usize
    }

    #[inline]
    fn count_match(&self, a: &[u8], b: &[u8], max_len: usize) -> usize {
        let limit = a.len().min(b.len()).min(max_len);
        let mut len = 0;
        while len < limit && a[len] == b[len] {
            len += 1;
        }
        len
    }
}

impl Default for Lz4Encoder {
    fn default() -> Self {
        Self::new()
    }
}

/// Streaming LZ4 decoder
///
/// Decompresses data incrementally as it arrives.
pub struct Lz4Decoder {
    /// Output buffer
    output: Vec<u8>,
    /// Input buffer for incomplete tokens
    input_buffer: Vec<u8>,
    /// Expected original size (from header)
    expected_size: Option<usize>,
    /// Current decoding position
    pos: usize,
}

impl Lz4Decoder {
    /// Create a new streaming decoder
    pub fn new() -> Self {
        Self {
            output: Vec::new(),
            input_buffer: Vec::new(),
            expected_size: None,
            pos: 0,
        }
    }

    /// Write compressed data chunk
    ///
    /// Returns decompressed bytes when available.
    ///
    /// # Arguments
    /// * `data` - Compressed chunk
    ///
    /// # Returns
    /// Decompressed bytes (may be empty if buffering)
    pub fn write(&mut self, data: &[u8]) -> Result<Vec<u8>> {
        if data.is_empty() {
            return Ok(Vec::new());
        }

        self.input_buffer.extend_from_slice(data);

        // Read header if not yet read
        if self.expected_size.is_none() {
            if self.input_buffer.len() < 4 {
                return Ok(Vec::new()); // Wait for header
            }

            let size = u32::from_le_bytes([
                self.input_buffer[0],
                self.input_buffer[1],
                self.input_buffer[2],
                self.input_buffer[3],
            ]) as usize;

            self.expected_size = Some(size);
            self.output.reserve(size);
            self.input_buffer.drain(0..4);
            self.pos = 0;
        }

        // Decompress available data
        self.decompress_buffered()?;

        // Return decompressed data
        if !self.output.is_empty() {
            let result = self.output.clone();
            Ok(result)
        } else {
            Ok(Vec::new())
        }
    }

    /// Finish decompression
    ///
    /// Returns final decompressed data and validates size.
    pub fn finish(self) -> Result<Vec<u8>> {
        if let Some(expected) = self.expected_size {
            if self.output.len() != expected {
                return Err(Error::CorruptedData(format!(
                    "Size mismatch: expected {}, got {}",
                    expected,
                    self.output.len()
                )));
            }
        }

        Ok(self.output)
    }

    /// Decompress buffered input
    fn decompress_buffered(&mut self) -> Result<()> {
        while self.pos < self.input_buffer.len() {
            // Read token
            let token = self.input_buffer[self.pos];
            self.pos += 1;

            let mut literal_len = (token >> 4) as usize;
            let mut match_len = (token & 0x0F) as usize;

            // Extended literal length
            if literal_len == 15 {
                loop {
                    if self.pos >= self.input_buffer.len() {
                        return Ok(()); // Wait for more data
                    }
                    let extra = self.input_buffer[self.pos] as usize;
                    self.pos += 1;
                    literal_len += extra;
                    if extra != 255 {
                        break;
                    }
                }
            }

            // Copy literals
            if literal_len > 0 {
                if self.pos + literal_len > self.input_buffer.len() {
                    return Ok(()); // Wait for more data
                }
                self.output.extend_from_slice(&self.input_buffer[self.pos..self.pos + literal_len]);
                self.pos += literal_len;
            }

            // Check if done
            if self.pos >= self.input_buffer.len() || match_len == 0 {
                continue;
            }

            // Read match offset
            if self.pos + 2 > self.input_buffer.len() {
                return Ok(()); // Wait for more data
            }
            let offset = u16::from_le_bytes([
                self.input_buffer[self.pos],
                self.input_buffer[self.pos + 1],
            ]) as usize;
            self.pos += 2;

            if offset == 0 || offset > self.output.len() {
                return Err(Error::CorruptedData(format!(
                    "Invalid match offset: {}",
                    offset
                )));
            }

            // Extended match length
            match_len += MIN_MATCH;
            if match_len == MIN_MATCH + 15 {
                loop {
                    if self.pos >= self.input_buffer.len() {
                        return Ok(()); // Wait for more data
                    }
                    let extra = self.input_buffer[self.pos] as usize;
                    self.pos += 1;
                    match_len += extra;
                    if extra != 255 {
                        break;
                    }
                }
            }

            // Copy match
            let match_start = self.output.len() - offset;
            for i in 0..match_len {
                let byte = self.output[match_start + i];
                self.output.push(byte);
            }
        }

        Ok(())
    }
}

impl Default for Lz4Decoder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encoder_single_chunk() {
        let mut encoder = Lz4Encoder::new();
        let data = b"Hello, World!";

        encoder.write(data).unwrap();
        let compressed = encoder.finish().unwrap();

        // Should be able to decompress with regular function
        let decompressed = crate::lz4::decompress(&compressed).unwrap();
        assert_eq!(data, &decompressed[..]);
    }

    #[test]
    fn test_encoder_multiple_chunks() {
        let mut encoder = Lz4Encoder::new();

        encoder.write(b"Hello, ").unwrap();
        encoder.write(b"World! ").unwrap();
        encoder.write(b"This is a test.").unwrap();

        let compressed = encoder.finish().unwrap();
        let decompressed = crate::lz4::decompress(&compressed).unwrap();

        assert_eq!(b"Hello, World! This is a test.", &decompressed[..]);
    }

    #[test]
    fn test_encoder_large_data() {
        let mut encoder = Lz4Encoder::new();
        let chunk = vec![b'A'; 10000];

        for _ in 0..10 {
            encoder.write(&chunk).unwrap();
        }

        let compressed = encoder.finish().unwrap();
        let decompressed = crate::lz4::decompress(&compressed).unwrap();

        let expected = vec![b'A'; 100000];
        assert_eq!(expected, decompressed);
    }

    #[test]
    fn test_decoder_single_chunk() {
        let original = b"Hello, World!";
        let compressed = crate::lz4::compress(original).unwrap();

        let mut decoder = Lz4Decoder::new();
        decoder.write(&compressed).unwrap();
        let decompressed = decoder.finish().unwrap();

        assert_eq!(original, &decompressed[..]);
    }

    #[test]
    fn test_decoder_multiple_chunks() {
        let original = b"Hello, World! This is streaming decompression.";
        let compressed = crate::lz4::compress(original).unwrap();

        let mut decoder = Lz4Decoder::new();

        // Split compressed data into chunks
        let mid = compressed.len() / 2;
        decoder.write(&compressed[..mid]).unwrap();
        decoder.write(&compressed[mid..]).unwrap();

        let decompressed = decoder.finish().unwrap();
        assert_eq!(original, &decompressed[..]);
    }

    #[test]
    fn test_round_trip_streaming() {
        let mut encoder = Lz4Encoder::new();

        encoder.write(b"Chunk 1: ").unwrap();
        encoder.write(b"Chunk 2: ").unwrap();
        encoder.write(b"Chunk 3").unwrap();

        let compressed = encoder.finish().unwrap();

        let mut decoder = Lz4Decoder::new();
        decoder.write(&compressed).unwrap();
        let decompressed = decoder.finish().unwrap();

        assert_eq!(b"Chunk 1: Chunk 2: Chunk 3", &decompressed[..]);
    }
}
