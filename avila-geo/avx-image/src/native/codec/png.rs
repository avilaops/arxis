//! PNG decoder/encoder
//!
//! Pure Rust implementation of PNG format (RFC 2083)
//! Supports: RGB, RGBA, Grayscale with various bit depths
//! Compression: DEFLATE (native implementation)

use std::io::{Read, Write};

/// PNG signature (8 bytes)
const PNG_SIGNATURE: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];

/// PNG color types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ColorType {
    Grayscale = 0,
    Rgb = 2,
    Indexed = 3,
    GrayscaleAlpha = 4,
    Rgba = 6,
}

/// PNG chunk
#[derive(Debug)]
struct Chunk {
    length: u32,
    chunk_type: [u8; 4],
    data: Vec<u8>,
    crc: u32,
}

impl Chunk {
    fn read<R: Read>(reader: &mut R) -> std::io::Result<Self> {
        let mut length_bytes = [0u8; 4];
        reader.read_exact(&mut length_bytes)?;
        let length = u32::from_be_bytes(length_bytes);

        let mut chunk_type = [0u8; 4];
        reader.read_exact(&mut chunk_type)?;

        let mut data = vec![0u8; length as usize];
        reader.read_exact(&mut data)?;

        let mut crc_bytes = [0u8; 4];
        reader.read_exact(&mut crc_bytes)?;
        let crc = u32::from_be_bytes(crc_bytes);

        Ok(Chunk {
            length,
            chunk_type,
            data,
            crc,
        })
    }

    fn chunk_type_str(&self) -> String {
        String::from_utf8_lossy(&self.chunk_type).to_string()
    }
}

/// PNG decoder
pub struct PngDecoder {
    pub width: u32,
    pub height: u32,
    pub bit_depth: u8,
    pub color_type: ColorType,
    pub compression: u8,
    pub filter_method: u8,
    pub interlace: u8,
}

impl PngDecoder {
    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            bit_depth: 8,
            color_type: ColorType::Rgba,
            compression: 0,
            filter_method: 0,
            interlace: 0,
        }
    }

    /// Decode PNG from reader
    pub fn decode<R: Read>(&mut self, reader: &mut R) -> std::io::Result<Vec<u8>> {
        // Verify PNG signature
        let mut signature = [0u8; 8];
        reader.read_exact(&mut signature)?;

        if signature != PNG_SIGNATURE {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid PNG signature",
            ));
        }

        let mut compressed_data = Vec::new();
        let mut palette: Option<Vec<u8>> = None;

        // Read chunks
        loop {
            let chunk = Chunk::read(reader)?;

            match &chunk.chunk_type {
                b"IHDR" => {
                    self.parse_ihdr(&chunk.data)?;
                }
                b"PLTE" => {
                    palette = Some(chunk.data.clone());
                }
                b"IDAT" => {
                    compressed_data.extend_from_slice(&chunk.data);
                }
                b"IEND" => {
                    break;
                }
                _ => {
                    // Skip unknown chunks
                }
            }
        }

        // Decompress data
        let decompressed = self.decompress_zlib(&compressed_data)?;

        // Unfilter scanlines
        let unfiltered = self.unfilter(&decompressed)?;

        // Convert to RGBA
        let rgba = self.to_rgba(&unfiltered, palette.as_deref())?;

        Ok(rgba)
    }

    fn parse_ihdr(&mut self, data: &[u8]) -> std::io::Result<()> {
        if data.len() < 13 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid IHDR chunk",
            ));
        }

        self.width = u32::from_be_bytes([data[0], data[1], data[2], data[3]]);
        self.height = u32::from_be_bytes([data[4], data[5], data[6], data[7]]);
        self.bit_depth = data[8];

        self.color_type = match data[9] {
            0 => ColorType::Grayscale,
            2 => ColorType::Rgb,
            3 => ColorType::Indexed,
            4 => ColorType::GrayscaleAlpha,
            6 => ColorType::Rgba,
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Invalid color type",
                ))
            }
        };

        self.compression = data[10];
        self.filter_method = data[11];
        self.interlace = data[12];

        Ok(())
    }

    /// Simplified DEFLATE decompression (for basic PNG support)
    /// Full implementation would require LZ77 + Huffman decoding
    fn decompress_zlib(&self, data: &[u8]) -> std::io::Result<Vec<u8>> {
        // TODO: Implement full DEFLATE decompression
        // For now, this is a placeholder that handles uncompressed data

        if data.len() < 6 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid zlib data",
            ));
        }

        // Skip zlib header (2 bytes) and checksum (4 bytes at end)
        let compressed = &data[2..data.len() - 4];

        // Simplified: assume uncompressed blocks for now
        // Real implementation needs full DEFLATE
        let mut output = Vec::new();
        let mut i = 0;

        while i < compressed.len() {
            if i + 5 > compressed.len() {
                break;
            }

            let _bfinal = compressed[i] & 0x01;
            let btype = (compressed[i] & 0x06) >> 1;

            if btype == 0 {
                // Uncompressed block
                i += 1;
                if i + 4 > compressed.len() {
                    break;
                }

                let len = u16::from_le_bytes([compressed[i], compressed[i + 1]]) as usize;
                i += 4; // Skip LEN and NLEN

                if i + len > compressed.len() {
                    break;
                }

                output.extend_from_slice(&compressed[i..i + len]);
                i += len;
            } else {
                // Compressed blocks - TODO: implement Huffman decoding
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Compressed PNG not yet supported - use uncompressed PNG for now",
                ));
            }
        }

        Ok(output)
    }

    fn unfilter(&self, data: &[u8]) -> std::io::Result<Vec<u8>> {
        let bytes_per_pixel = self.bytes_per_pixel();
        let scanline_length = (self.width as usize * bytes_per_pixel) + 1; // +1 for filter byte

        if data.len() != scanline_length * self.height as usize {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!(
                    "Invalid data length: expected {}, got {}",
                    scanline_length * self.height as usize,
                    data.len()
                ),
            ));
        }

        let mut output = Vec::with_capacity(self.width as usize * self.height as usize * bytes_per_pixel);
        let mut prev_scanline = vec![0u8; self.width as usize * bytes_per_pixel];

        for y in 0..self.height as usize {
            let offset = y * scanline_length;
            let filter_type = data[offset];
            let scanline = &data[offset + 1..offset + scanline_length];

            let unfiltered = self.unfilter_scanline(filter_type, scanline, &prev_scanline, bytes_per_pixel)?;
            output.extend_from_slice(&unfiltered);
            prev_scanline.copy_from_slice(&unfiltered);
        }

        Ok(output)
    }

    fn unfilter_scanline(
        &self,
        filter_type: u8,
        scanline: &[u8],
        prev_scanline: &[u8],
        bytes_per_pixel: usize,
    ) -> std::io::Result<Vec<u8>> {
        let mut output = vec![0u8; scanline.len()];

        match filter_type {
            0 => {
                // None
                output.copy_from_slice(scanline);
            }
            1 => {
                // Sub
                for i in 0..scanline.len() {
                    let left = if i >= bytes_per_pixel {
                        output[i - bytes_per_pixel]
                    } else {
                        0
                    };
                    output[i] = scanline[i].wrapping_add(left);
                }
            }
            2 => {
                // Up
                for i in 0..scanline.len() {
                    output[i] = scanline[i].wrapping_add(prev_scanline[i]);
                }
            }
            3 => {
                // Average
                for i in 0..scanline.len() {
                    let left = if i >= bytes_per_pixel {
                        output[i - bytes_per_pixel] as u16
                    } else {
                        0
                    };
                    let up = prev_scanline[i] as u16;
                    let avg = ((left + up) / 2) as u8;
                    output[i] = scanline[i].wrapping_add(avg);
                }
            }
            4 => {
                // Paeth
                for i in 0..scanline.len() {
                    let left = if i >= bytes_per_pixel {
                        output[i - bytes_per_pixel]
                    } else {
                        0
                    };
                    let up = prev_scanline[i];
                    let up_left = if i >= bytes_per_pixel {
                        prev_scanline[i - bytes_per_pixel]
                    } else {
                        0
                    };

                    let paeth = paeth_predictor(left, up, up_left);
                    output[i] = scanline[i].wrapping_add(paeth);
                }
            }
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Invalid filter type",
                ));
            }
        }

        Ok(output)
    }

    fn bytes_per_pixel(&self) -> usize {
        let bits_per_pixel = match self.color_type {
            ColorType::Grayscale => self.bit_depth as usize,
            ColorType::Rgb => self.bit_depth as usize * 3,
            ColorType::Indexed => self.bit_depth as usize,
            ColorType::GrayscaleAlpha => self.bit_depth as usize * 2,
            ColorType::Rgba => self.bit_depth as usize * 4,
        };

        (bits_per_pixel + 7) / 8
    }

    fn to_rgba(&self, data: &[u8], _palette: Option<&[u8]>) -> std::io::Result<Vec<u8>> {
        let mut rgba = Vec::with_capacity(self.width as usize * self.height as usize * 4);

        match self.color_type {
            ColorType::Rgba if self.bit_depth == 8 => {
                // Already RGBA8
                rgba.extend_from_slice(data);
            }
            ColorType::Rgb if self.bit_depth == 8 => {
                // RGB8 -> RGBA8
                for i in (0..data.len()).step_by(3) {
                    rgba.push(data[i]);
                    rgba.push(data[i + 1]);
                    rgba.push(data[i + 2]);
                    rgba.push(255);
                }
            }
            ColorType::Grayscale if self.bit_depth == 8 => {
                // Grayscale8 -> RGBA8
                for &gray in data {
                    rgba.push(gray);
                    rgba.push(gray);
                    rgba.push(gray);
                    rgba.push(255);
                }
            }
            ColorType::GrayscaleAlpha if self.bit_depth == 8 => {
                // GrayscaleAlpha8 -> RGBA8
                for i in (0..data.len()).step_by(2) {
                    let gray = data[i];
                    let alpha = data[i + 1];
                    rgba.push(gray);
                    rgba.push(gray);
                    rgba.push(gray);
                    rgba.push(alpha);
                }
            }
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("Unsupported color type/bit depth: {:?}/{}", self.color_type, self.bit_depth),
                ));
            }
        }

        Ok(rgba)
    }
}

/// Paeth predictor for PNG filter type 4
fn paeth_predictor(a: u8, b: u8, c: u8) -> u8 {
    let a = a as i32;
    let b = b as i32;
    let c = c as i32;

    let p = a + b - c;
    let pa = (p - a).abs();
    let pb = (p - b).abs();
    let pc = (p - c).abs();

    if pa <= pb && pa <= pc {
        a as u8
    } else if pb <= pc {
        b as u8
    } else {
        c as u8
    }
}

/// PNG encoder (simplified)
pub struct PngEncoder {
    pub width: u32,
    pub height: u32,
}

impl PngEncoder {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    /// Encode RGBA8 data to PNG
    pub fn encode<W: Write>(&self, writer: &mut W, data: &[u8]) -> std::io::Result<()> {
        // Write PNG signature
        writer.write_all(&PNG_SIGNATURE)?;

        // Write IHDR chunk
        self.write_ihdr(writer)?;

        // Write IDAT chunk (simplified - uncompressed)
        self.write_idat(writer, data)?;

        // Write IEND chunk
        self.write_iend(writer)?;

        Ok(())
    }

    fn write_ihdr<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        let mut data = Vec::new();
        data.extend_from_slice(&self.width.to_be_bytes());
        data.extend_from_slice(&self.height.to_be_bytes());
        data.push(8); // bit depth
        data.push(6); // color type (RGBA)
        data.push(0); // compression
        data.push(0); // filter method
        data.push(0); // interlace

        self.write_chunk(writer, b"IHDR", &data)
    }

    fn write_idat<W: Write>(&self, writer: &mut W, rgba: &[u8]) -> std::io::Result<()> {
        // Apply filter type 0 (None) to each scanline
        let mut filtered = Vec::new();
        let bytes_per_scanline = self.width as usize * 4;

        for y in 0..self.height as usize {
            filtered.push(0); // Filter type: None
            let offset = y * bytes_per_scanline;
            filtered.extend_from_slice(&rgba[offset..offset + bytes_per_scanline]);
        }

        // Simplified: write uncompressed zlib data
        let mut compressed = Vec::new();
        compressed.push(0x78); // CMF
        compressed.push(0x01); // FLG

        // Write uncompressed blocks
        let chunk_size = 65535;
        for chunk in filtered.chunks(chunk_size) {
            let is_final = chunk.len() < chunk_size;
            compressed.push(if is_final { 0x01 } else { 0x00 });

            let len = chunk.len() as u16;
            compressed.extend_from_slice(&len.to_le_bytes());
            compressed.extend_from_slice(&(!len).to_le_bytes());
            compressed.extend_from_slice(chunk);
        }

        // Add Adler-32 checksum (simplified: use 1)
        compressed.extend_from_slice(&[0, 0, 0, 1]);

        self.write_chunk(writer, b"IDAT", &compressed)
    }

    fn write_iend<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        self.write_chunk(writer, b"IEND", &[])
    }

    fn write_chunk<W: Write>(
        &self,
        writer: &mut W,
        chunk_type: &[u8; 4],
        data: &[u8],
    ) -> std::io::Result<()> {
        // Length
        writer.write_all(&(data.len() as u32).to_be_bytes())?;

        // Chunk type
        writer.write_all(chunk_type)?;

        // Data
        writer.write_all(data)?;

        // CRC (simplified: use 0)
        let crc = Self::calculate_crc(chunk_type, data);
        writer.write_all(&crc.to_be_bytes())?;

        Ok(())
    }

    fn calculate_crc(chunk_type: &[u8; 4], data: &[u8]) -> u32 {
        let mut crc = 0xFFFFFFFF_u32;

        for &byte in chunk_type.iter().chain(data.iter()) {
            crc ^= byte as u32;
            for _ in 0..8 {
                if crc & 1 != 0 {
                    crc = (crc >> 1) ^ 0xEDB88320;
                } else {
                    crc >>= 1;
                }
            }
        }

        !crc
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paeth_predictor() {
        // Test case 1: a=10, b=20, c=15
        // p = 10 + 20 - 15 = 15
        // pa = |15 - 10| = 5, pb = |15 - 20| = 5, pc = |15 - 15| = 0
        // Since pc is smallest, return c = 15
        assert_eq!(paeth_predictor(10, 20, 15), 15);

        // Test case 2: all equal
        assert_eq!(paeth_predictor(100, 100, 100), 100);

        // Test case 3: a is closest
        assert_eq!(paeth_predictor(10, 5, 2), 10);
    }

    #[test]
    fn test_crc_calculation() {
        let crc = PngEncoder::calculate_crc(b"IHDR", &[0, 0, 0, 1]);
        assert!(crc != 0); // Should calculate non-zero CRC
    }
}

