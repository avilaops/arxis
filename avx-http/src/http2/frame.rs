//! HTTP/2 frame parsing and serialization
//!
//! RFC 7540 Section 4 - Frame Format

use crate::error::{Error, Result};
use crate::bytes::Bytes;

/// Frame types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameType {
    Data = 0x0,
    Headers = 0x1,
    Priority = 0x2,
    RstStream = 0x3,
    Settings = 0x4,
    PushPromise = 0x5,
    Ping = 0x6,
    GoAway = 0x7,
    WindowUpdate = 0x8,
    Continuation = 0x9,
}

impl FrameType {
    pub fn from_u8(byte: u8) -> Option<Self> {
        match byte {
            0x0 => Some(FrameType::Data),
            0x1 => Some(FrameType::Headers),
            0x2 => Some(FrameType::Priority),
            0x3 => Some(FrameType::RstStream),
            0x4 => Some(FrameType::Settings),
            0x5 => Some(FrameType::PushPromise),
            0x6 => Some(FrameType::Ping),
            0x7 => Some(FrameType::GoAway),
            0x8 => Some(FrameType::WindowUpdate),
            0x9 => Some(FrameType::Continuation),
            _ => None,
        }
    }
}

/// HTTP/2 frame
#[derive(Debug, Clone)]
pub enum Frame {
    Data(DataFrame),
    Headers(HeadersFrame),
    Priority(PriorityFrame),
    RstStream(RstStreamFrame),
    Settings(SettingsFrame),
    PushPromise(PushPromiseFrame),
    Ping(PingFrame),
    GoAway(GoAwayFrame),
    WindowUpdate(WindowUpdateFrame),
    Continuation(ContinuationFrame),
}

/// Frame header (9 bytes)
#[derive(Debug, Clone)]
pub struct FrameHeader {
    /// Payload length (24 bits)
    pub length: u32,
    /// Frame type (8 bits)
    pub frame_type: u8,
    /// Flags (8 bits)
    pub flags: u8,
    /// Stream identifier (31 bits, R=0)
    pub stream_id: u32,
}

impl FrameHeader {
    /// Parse frame header from bytes
    pub fn parse(buf: &[u8]) -> Result<Self> {
        if buf.len() < 9 {
            return Err(Error::ParseError {
                message: "Frame header too short".to_string(),
            });
        }

        // Length (3 bytes, big-endian)
        let length = ((buf[0] as u32) << 16) | ((buf[1] as u32) << 8) | (buf[2] as u32);

        let frame_type = buf[3];
        let flags = buf[4];

        // Stream ID (4 bytes, big-endian, ignore R bit)
        let stream_id = ((buf[5] as u32) << 24)
            | ((buf[6] as u32) << 16)
            | ((buf[7] as u32) << 8)
            | (buf[8] as u32);
        let stream_id = stream_id & 0x7FFFFFFF; // Clear R bit

        Ok(FrameHeader {
            length,
            frame_type,
            flags,
            stream_id,
        })
    }

    /// Serialize frame header to bytes
    pub fn to_bytes(&self) -> [u8; 9] {
        let mut buf = [0u8; 9];

        // Length (3 bytes)
        buf[0] = ((self.length >> 16) & 0xFF) as u8;
        buf[1] = ((self.length >> 8) & 0xFF) as u8;
        buf[2] = (self.length & 0xFF) as u8;

        buf[3] = self.frame_type;
        buf[4] = self.flags;

        // Stream ID (4 bytes)
        buf[5] = ((self.stream_id >> 24) & 0xFF) as u8;
        buf[6] = ((self.stream_id >> 16) & 0xFF) as u8;
        buf[7] = ((self.stream_id >> 8) & 0xFF) as u8;
        buf[8] = (self.stream_id & 0xFF) as u8;

        buf
    }
}

/// DATA frame
#[derive(Debug, Clone)]
pub struct DataFrame {
    pub stream_id: u32,
    pub data: Bytes,
    pub end_stream: bool,
    pub padded: bool,
    pub pad_length: u8,
}

impl DataFrame {
    pub fn new(stream_id: u32, data: Bytes) -> Self {
        Self {
            stream_id,
            data,
            end_stream: false,
            padded: false,
            pad_length: 0,
        }
    }

    pub fn with_end_stream(mut self) -> Self {
        self.end_stream = true;
        self
    }

    pub fn parse(header: &FrameHeader, payload: &[u8]) -> Result<Self> {
        let end_stream = (header.flags & 0x01) != 0;
        let padded = (header.flags & 0x08) != 0;

        let mut pos = 0;
        let pad_length = if padded {
            if payload.is_empty() {
                return Err(Error::ParseError {
                    message: "Padded DATA frame with no pad length".to_string(),
                });
            }
            let len = payload[0];
            pos += 1;
            len
        } else {
            0
        };

        let data_len = payload.len() - pos - pad_length as usize;
        let data = Bytes::copy_from_slice(&payload[pos..pos + data_len]);

        Ok(DataFrame {
            stream_id: header.stream_id,
            data,
            end_stream,
            padded,
            pad_length,
        })
    }
}

/// HEADERS frame
#[derive(Debug, Clone)]
pub struct HeadersFrame {
    pub stream_id: u32,
    pub header_block: Bytes,
    pub end_stream: bool,
    pub end_headers: bool,
    pub padded: bool,
    pub priority: Option<Priority>,
}

impl HeadersFrame {
    pub fn new(stream_id: u32, header_block: Bytes) -> Self {
        Self {
            stream_id,
            header_block,
            end_stream: false,
            end_headers: true,
            padded: false,
            priority: None,
        }
    }
}

/// Stream priority
#[derive(Debug, Clone)]
pub struct Priority {
    pub exclusive: bool,
    pub stream_dependency: u32,
    pub weight: u8,
}

/// PRIORITY frame
#[derive(Debug, Clone)]
pub struct PriorityFrame {
    pub stream_id: u32,
    pub priority: Priority,
}

/// RST_STREAM frame
#[derive(Debug, Clone)]
pub struct RstStreamFrame {
    pub stream_id: u32,
    pub error_code: u32,
}

/// SETTINGS frame
#[derive(Debug, Clone)]
pub struct SettingsFrame {
    pub ack: bool,
    pub settings: Vec<Setting>,
}

impl SettingsFrame {
    pub fn new() -> Self {
        Self {
            ack: false,
            settings: Vec::new(),
        }
    }

    pub fn with_setting(mut self, id: u16, value: u32) -> Self {
        self.settings.push(Setting { id, value });
        self
    }
}

impl Default for SettingsFrame {
    fn default() -> Self {
        Self::new()
    }
}

/// Single setting
#[derive(Debug, Clone)]
pub struct Setting {
    pub id: u16,
    pub value: u32,
}

// Setting identifiers (RFC 7540 Section 6.5.2)
pub const SETTINGS_HEADER_TABLE_SIZE: u16 = 0x1;
pub const SETTINGS_ENABLE_PUSH: u16 = 0x2;
pub const SETTINGS_MAX_CONCURRENT_STREAMS: u16 = 0x3;
pub const SETTINGS_INITIAL_WINDOW_SIZE: u16 = 0x4;
pub const SETTINGS_MAX_FRAME_SIZE: u16 = 0x5;
pub const SETTINGS_MAX_HEADER_LIST_SIZE: u16 = 0x6;

/// PUSH_PROMISE frame
#[derive(Debug, Clone)]
pub struct PushPromiseFrame {
    pub stream_id: u32,
    pub promised_stream_id: u32,
    pub header_block: Bytes,
}

/// PING frame
#[derive(Debug, Clone)]
pub struct PingFrame {
    pub ack: bool,
    pub opaque_data: [u8; 8],
}

/// GOAWAY frame
#[derive(Debug, Clone)]
pub struct GoAwayFrame {
    pub last_stream_id: u32,
    pub error_code: u32,
    pub debug_data: Bytes,
}

/// WINDOW_UPDATE frame
#[derive(Debug, Clone)]
pub struct WindowUpdateFrame {
    pub stream_id: u32,
    pub window_size_increment: u32,
}

/// CONTINUATION frame
#[derive(Debug, Clone)]
pub struct ContinuationFrame {
    pub stream_id: u32,
    pub header_block: Bytes,
    pub end_headers: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frame_header_parse() {
        let buf = [
            0x00, 0x00, 0x0A, // Length: 10
            0x00, // Type: DATA
            0x01, // Flags: END_STREAM
            0x00, 0x00, 0x00, 0x01, // Stream ID: 1
        ];

        let header = FrameHeader::parse(&buf).unwrap();
        assert_eq!(header.length, 10);
        assert_eq!(header.frame_type, 0);
        assert_eq!(header.flags, 1);
        assert_eq!(header.stream_id, 1);
    }

    #[test]
    fn test_frame_header_serialize() {
        let header = FrameHeader {
            length: 256,
            frame_type: 1,
            flags: 4,
            stream_id: 3,
        };

        let bytes = header.to_bytes();
        assert_eq!(bytes[0], 0x00);
        assert_eq!(bytes[1], 0x01);
        assert_eq!(bytes[2], 0x00);
        assert_eq!(bytes[3], 1);
        assert_eq!(bytes[8], 3);
    }

    #[test]
    fn test_data_frame_new() {
        let data = Bytes::from("Hello, HTTP/2!");
        let frame = DataFrame::new(1, data.clone());

        assert_eq!(frame.stream_id, 1);
        assert_eq!(frame.data.len(), 14);
        assert!(!frame.end_stream);
    }

    #[test]
    fn test_settings_frame() {
        let frame = SettingsFrame::new()
            .with_setting(SETTINGS_MAX_CONCURRENT_STREAMS, 100)
            .with_setting(SETTINGS_INITIAL_WINDOW_SIZE, 65535);

        assert_eq!(frame.settings.len(), 2);
        assert_eq!(frame.settings[0].id, SETTINGS_MAX_CONCURRENT_STREAMS);
        assert_eq!(frame.settings[0].value, 100);
    }
}
