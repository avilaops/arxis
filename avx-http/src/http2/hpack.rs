//! HPACK header compression (RFC 7541)
//!
//! Zero-copy implementation for HTTP/2 header compression

use crate::bytes::Bytes;
use crate::error::{Error, Result};
use std::collections::VecDeque;

/// HPACK static table (RFC 7541 Appendix A)
const STATIC_TABLE: &[(&str, &str)] = &[
    (":authority", ""),
    (":method", "GET"),
    (":method", "POST"),
    (":path", "/"),
    (":path", "/index.html"),
    (":scheme", "http"),
    (":scheme", "https"),
    (":status", "200"),
    (":status", "204"),
    (":status", "206"),
    (":status", "304"),
    (":status", "400"),
    (":status", "404"),
    (":status", "500"),
    ("accept-charset", ""),
    ("accept-encoding", "gzip, deflate"),
    ("accept-language", ""),
    ("accept-ranges", ""),
    ("accept", ""),
    ("access-control-allow-origin", ""),
    ("age", ""),
    ("allow", ""),
    ("authorization", ""),
    ("cache-control", ""),
    ("content-disposition", ""),
    ("content-encoding", ""),
    ("content-language", ""),
    ("content-length", ""),
    ("content-location", ""),
    ("content-range", ""),
    ("content-type", ""),
    ("cookie", ""),
    ("date", ""),
    ("etag", ""),
    ("expect", ""),
    ("expires", ""),
    ("from", ""),
    ("host", ""),
    ("if-match", ""),
    ("if-modified-since", ""),
    ("if-none-match", ""),
    ("if-range", ""),
    ("if-unmodified-since", ""),
    ("last-modified", ""),
    ("link", ""),
    ("location", ""),
    ("max-forwards", ""),
    ("proxy-authenticate", ""),
    ("proxy-authorization", ""),
    ("range", ""),
    ("referer", ""),
    ("refresh", ""),
    ("retry-after", ""),
    ("server", ""),
    ("set-cookie", ""),
    ("strict-transport-security", ""),
    ("transfer-encoding", ""),
    ("user-agent", ""),
    ("vary", ""),
    ("via", ""),
    ("www-authenticate", ""),
];

/// HPACK encoder
pub struct HpackEncoder {
    dynamic_table: DynamicTable,
}

impl HpackEncoder {
    /// Create new encoder
    pub fn new() -> Self {
        Self {
            dynamic_table: DynamicTable::new(4096),
        }
    }

    /// Create encoder with custom table size
    pub fn with_max_size(max_size: usize) -> Self {
        Self {
            dynamic_table: DynamicTable::new(max_size),
        }
    }

    /// Encode headers to HPACK format
    pub fn encode(&mut self, headers: &[(String, String)]) -> Result<Bytes> {
        let mut output = Vec::new();

        for (name, value) in headers {
            // Try to find in static table
            if let Some(index) = self.find_in_static_table(name, value) {
                // Indexed Header Field (RFC 7541 Section 6.1)
                self.encode_integer(index, 7, 0b1000_0000, &mut output);
            }
            // Try to find in dynamic table
            else if let Some(index) = self.dynamic_table.find(name, value) {
                let idx = index + STATIC_TABLE.len();
                self.encode_integer(idx, 7, 0b1000_0000, &mut output);
            }
            // Literal with incremental indexing
            else {
                // Add to dynamic table
                self.dynamic_table.insert(name.clone(), value.clone());

                // Encode as literal (RFC 7541 Section 6.2.1)
                output.push(0b0100_0000); // Literal with incremental indexing

                // Encode name length and name
                self.encode_integer(name.len(), 7, 0, &mut output);
                output.extend_from_slice(name.as_bytes());

                // Encode value length and value
                self.encode_integer(value.len(), 7, 0, &mut output);
                output.extend_from_slice(value.as_bytes());
            }
        }

        Ok(Bytes::from_vec(output))
    }

    /// Encode integer using variable-length encoding
    fn encode_integer(&self, mut value: usize, prefix_bits: u8, prefix_mask: u8, output: &mut Vec<u8>) {
        let max_prefix = (1 << prefix_bits) - 1;

        if value < max_prefix {
            output.push(prefix_mask | (value as u8));
        } else {
            output.push(prefix_mask | max_prefix as u8);
            value -= max_prefix;

            while value >= 128 {
                output.push(((value % 128) + 128) as u8);
                value /= 128;
            }
            output.push(value as u8);
        }
    }

    /// Find header in static table
    fn find_in_static_table(&self, name: &str, value: &str) -> Option<usize> {
        STATIC_TABLE
            .iter()
            .position(|(n, v)| n.eq_ignore_ascii_case(name) && *v == value)
            .map(|idx| idx + 1) // HPACK indexing starts at 1
    }
}

impl Default for HpackEncoder {
    fn default() -> Self {
        Self::new()
    }
}

/// HPACK decoder
pub struct HpackDecoder {
    dynamic_table: DynamicTable,
}

impl HpackDecoder {
    /// Create new decoder
    pub fn new() -> Self {
        Self {
            dynamic_table: DynamicTable::new(4096),
        }
    }

    /// Decode HPACK-encoded headers
    pub fn decode(&mut self, buf: &[u8]) -> Result<Vec<(String, String)>> {
        let mut headers = Vec::new();
        let mut pos = 0;

        while pos < buf.len() {
            let byte = buf[pos];

            // Indexed Header Field (bit pattern: 1xxxxxxx)
            if byte & 0x80 != 0 {
                let (index, consumed) = self.decode_integer(&buf[pos..], 7)?;
                pos += consumed;

                let (name, value) = self.get_indexed(index)?;
                headers.push((name, value));
            }
            // Literal Header Field with Incremental Indexing (bit pattern: 01xxxxxx)
            else if byte & 0x40 != 0 {
                pos += 1;

                // Decode name
                let (name_len, consumed) = self.decode_integer(&buf[pos..], 7)?;
                pos += consumed;
                let name = String::from_utf8(buf[pos..pos + name_len].to_vec())
                    .map_err(|_| Error::InvalidUtf8 {
                        message: "Invalid header name".to_string(),
                    })?;
                pos += name_len;

                // Decode value
                let (value_len, consumed) = self.decode_integer(&buf[pos..], 7)?;
                pos += consumed;
                let value = String::from_utf8(buf[pos..pos + value_len].to_vec())
                    .map_err(|_| Error::InvalidUtf8 {
                        message: "Invalid header value".to_string(),
                    })?;
                pos += value_len;

                self.dynamic_table.insert(name.clone(), value.clone());
                headers.push((name, value));
            }
            // Literal Header Field without Indexing (bit pattern: 0000xxxx)
            else {
                pos += 1;

                // Similar to incremental, but don't add to dynamic table
                let (name_len, consumed) = self.decode_integer(&buf[pos..], 7)?;
                pos += consumed;
                let name = String::from_utf8(buf[pos..pos + name_len].to_vec())
                    .map_err(|_| Error::InvalidUtf8 {
                        message: "Invalid header name".to_string(),
                    })?;
                pos += name_len;

                let (value_len, consumed) = self.decode_integer(&buf[pos..], 7)?;
                pos += consumed;
                let value = String::from_utf8(buf[pos..pos + value_len].to_vec())
                    .map_err(|_| Error::InvalidUtf8 {
                        message: "Invalid header value".to_string(),
                    })?;
                pos += value_len;

                headers.push((name, value));
            }
        }

        Ok(headers)
    }

    /// Decode variable-length integer
    fn decode_integer(&self, buf: &[u8], prefix_bits: u8) -> Result<(usize, usize)> {
        if buf.is_empty() {
            return Err(Error::ParseError {
                message: "Empty buffer for integer decode".to_string(),
            });
        }

        let max_prefix = (1 << prefix_bits) - 1;
        let first_byte = buf[0] & max_prefix as u8;

        if first_byte < max_prefix as u8 {
            return Ok((first_byte as usize, 1));
        }

        let mut value = max_prefix;
        let mut pos = 1;
        let mut shift = 0;

        loop {
            if pos >= buf.len() {
                return Err(Error::ParseError {
                    message: "Incomplete integer encoding".to_string(),
                });
            }

            let byte = buf[pos];
            pos += 1;

            value += ((byte & 0x7F) as usize) << shift;
            shift += 7;

            if byte & 0x80 == 0 {
                break;
            }
        }

        Ok((value, pos))
    }

    /// Get header by index (static or dynamic table)
    fn get_indexed(&self, index: usize) -> Result<(String, String)> {
        if index == 0 {
            return Err(Error::ParseError {
                message: "Index 0 is invalid in HPACK".to_string(),
            });
        }

        // Static table
        if index <= STATIC_TABLE.len() {
            let (name, value) = STATIC_TABLE[index - 1];
            return Ok((name.to_string(), value.to_string()));
        }

        // Dynamic table
        let dynamic_index = index - STATIC_TABLE.len() - 1;
        self.dynamic_table.get(dynamic_index)
            .ok_or_else(|| Error::ParseError {
                message: format!("Invalid dynamic table index: {}", dynamic_index),
            })
    }
}

impl Default for HpackDecoder {
    fn default() -> Self {
        Self::new()
    }
}

/// Dynamic table for HPACK
struct DynamicTable {
    entries: VecDeque<(String, String)>,
    size: usize,
    max_size: usize,
}

impl DynamicTable {
    fn new(max_size: usize) -> Self {
        Self {
            entries: VecDeque::new(),
            size: 0,
            max_size,
        }
    }

    fn insert(&mut self, name: String, value: String) {
        let entry_size = 32 + name.len() + value.len();

        // Evict entries if necessary
        while self.size + entry_size > self.max_size && !self.entries.is_empty() {
            if let Some((old_name, old_value)) = self.entries.pop_back() {
                self.size -= 32 + old_name.len() + old_value.len();
            }
        }

        self.entries.push_front((name, value));
        self.size += entry_size;
    }

    fn find(&self, name: &str, value: &str) -> Option<usize> {
        self.entries
            .iter()
            .position(|(n, v)| n.eq_ignore_ascii_case(name) && v == value)
    }

    fn get(&self, index: usize) -> Option<(String, String)> {
        self.entries.get(index).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_static_table() {
        assert_eq!(STATIC_TABLE[0], (":authority", ""));
        assert_eq!(STATIC_TABLE[1], (":method", "GET"));
        assert_eq!(STATIC_TABLE[2], (":method", "POST"));
    }

    #[test]
    fn test_encode_integer_small() {
        let encoder = HpackEncoder::new();
        let mut output = Vec::new();
        encoder.encode_integer(10, 7, 0, &mut output);

        assert_eq!(output.len(), 1);
        assert_eq!(output[0], 10);
    }

    #[test]
    fn test_encode_integer_large() {
        let encoder = HpackEncoder::new();
        let mut output = Vec::new();
        encoder.encode_integer(1337, 5, 0, &mut output);

        assert!(output.len() > 1);
    }

    #[test]
    fn test_encode_decode_headers() {
        let mut encoder = HpackEncoder::new();
        let mut decoder = HpackDecoder::new();

        let headers = vec![
            (":method".to_string(), "GET".to_string()),
            (":path".to_string(), "/".to_string()),
            (":scheme".to_string(), "https".to_string()),
        ];

        let encoded = encoder.encode(&headers).unwrap();
        let decoded = decoder.decode(&encoded).unwrap();

        assert_eq!(decoded.len(), 3);
        assert_eq!(decoded[0].0, ":method");
        assert_eq!(decoded[0].1, "GET");
    }

    #[test]
    fn test_dynamic_table_insertion() {
        let mut table = DynamicTable::new(1000);
        table.insert("custom-key".to_string(), "custom-value".to_string());

        assert_eq!(table.entries.len(), 1);
        assert!(table.find("custom-key", "custom-value").is_some());
    }

    #[test]
    fn test_dynamic_table_eviction() {
        let mut table = DynamicTable::new(100);

        // Insert many entries to trigger eviction
        for i in 0..10 {
            table.insert(format!("key{}", i), format!("value{}", i));
        }

        // Should have evicted some entries
        assert!(table.entries.len() < 10);
    }
}
