//! HTTP/2 implementation - Pure Rust, zero dependencies
//!
//! RFC 7540 compliant HTTP/2 frame parsing, HPACK compression, multiplexing

pub mod frame;
pub mod hpack;
pub mod connection;
pub mod stream;

// Re-export submodules
pub use self::frame as frames;
pub use self::connection as conn;

pub use frame::{
    Frame, FrameHeader, DataFrame, HeadersFrame, SettingsFrame, PriorityFrame,
    Setting, SETTINGS_HEADER_TABLE_SIZE, SETTINGS_ENABLE_PUSH,
    SETTINGS_MAX_CONCURRENT_STREAMS, SETTINGS_INITIAL_WINDOW_SIZE,
    SETTINGS_MAX_FRAME_SIZE, SETTINGS_MAX_HEADER_LIST_SIZE,
};
pub use hpack::{HpackEncoder, HpackDecoder};
pub use connection::Http2Connection;
pub use stream::{Stream, StreamState};

/// HTTP/2 connection preface
pub const CONNECTION_PREFACE: &[u8] = b"PRI * HTTP/2.0\r\n\r\nSM\r\n\r\n";

/// HTTP/2 protocol error codes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCode {
    /// No error
    NoError = 0x0,
    /// Protocol error
    ProtocolError = 0x1,
    /// Internal error
    InternalError = 0x2,
    /// Flow control error
    FlowControlError = 0x3,
    /// Settings timeout
    SettingsTimeout = 0x4,
    /// Stream closed
    StreamClosed = 0x5,
    /// Frame size error
    FrameSizeError = 0x6,
    /// Refused stream
    RefusedStream = 0x7,
    /// Cancel
    Cancel = 0x8,
    /// Compression error
    CompressionError = 0x9,
    /// Connect error
    ConnectError = 0xa,
    /// Enhance your calm
    EnhanceYourCalm = 0xb,
    /// Inadequate security
    InadequateSecurity = 0xc,
    /// HTTP/1.1 required
    Http11Required = 0xd,
}

impl ErrorCode {
    /// Convert from u32
    pub fn from_u32(code: u32) -> Option<Self> {
        match code {
            0x0 => Some(ErrorCode::NoError),
            0x1 => Some(ErrorCode::ProtocolError),
            0x2 => Some(ErrorCode::InternalError),
            0x3 => Some(ErrorCode::FlowControlError),
            0x4 => Some(ErrorCode::SettingsTimeout),
            0x5 => Some(ErrorCode::StreamClosed),
            0x6 => Some(ErrorCode::FrameSizeError),
            0x7 => Some(ErrorCode::RefusedStream),
            0x8 => Some(ErrorCode::Cancel),
            0x9 => Some(ErrorCode::CompressionError),
            0xa => Some(ErrorCode::ConnectError),
            0xb => Some(ErrorCode::EnhanceYourCalm),
            0xc => Some(ErrorCode::InadequateSecurity),
            0xd => Some(ErrorCode::Http11Required),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_preface() {
        assert_eq!(CONNECTION_PREFACE.len(), 24);
        assert!(CONNECTION_PREFACE.starts_with(b"PRI * HTTP/2.0"));
    }

    #[test]
    fn test_error_code() {
        assert_eq!(ErrorCode::from_u32(0), Some(ErrorCode::NoError));
        assert_eq!(ErrorCode::from_u32(1), Some(ErrorCode::ProtocolError));
        assert_eq!(ErrorCode::from_u32(999), None);
    }
}
