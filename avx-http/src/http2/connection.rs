//! HTTP/2 connection management

use crate::bytes::Bytes;
use crate::error::{Error, Result};
use crate::http2::{Frame, FrameHeader, DataFrame, HeadersFrame, SettingsFrame, Setting};
use crate::http2::{SETTINGS_MAX_CONCURRENT_STREAMS, SETTINGS_INITIAL_WINDOW_SIZE};
use crate::http2::CONNECTION_PREFACE;
use crate::http2::hpack::{HpackEncoder, HpackDecoder};
use crate::http2::stream::{Stream, StreamState};
use crate::net::TcpStream;
use std::collections::HashMap;

/// HTTP/2 connection settings
pub struct ConnectionSettings {
    pub header_table_size: u32,
    pub enable_push: bool,
    pub max_concurrent_streams: u32,
    pub initial_window_size: u32,
    pub max_frame_size: u32,
    pub max_header_list_size: u32,
}

impl Default for ConnectionSettings {
    fn default() -> Self {
        Self {
            header_table_size: 4096,
            enable_push: true,
            max_concurrent_streams: 100,
            initial_window_size: 65535,
            max_frame_size: 16384,
            max_header_list_size: 8192,
        }
    }
}

/// HTTP/2 connection
pub struct Http2Connection {
    stream: TcpStream,
    streams: HashMap<u32, Stream>,
    next_stream_id: u32,
    settings: ConnectionSettings,
    hpack_encoder: HpackEncoder,
    hpack_decoder: HpackDecoder,
    connection_window: i32,
}

impl Http2Connection {
    /// Create new HTTP/2 client connection
    pub fn new_client(mut stream: TcpStream) -> Result<Self> {
        // Send connection preface
        stream.write_all(CONNECTION_PREFACE)?;

        // Send initial SETTINGS frame
        let settings_frame = SettingsFrame::new()
            .with_setting(SETTINGS_MAX_CONCURRENT_STREAMS, 100)
            .with_setting(SETTINGS_INITIAL_WINDOW_SIZE, 65535);

        Self::send_settings_frame(&mut stream, &settings_frame)?;

        // Wait for server SETTINGS
        // TODO: Read and process server SETTINGS

        Ok(Http2Connection {
            stream,
            streams: HashMap::new(),
            next_stream_id: 1, // Client uses odd IDs
            settings: ConnectionSettings::default(),
            hpack_encoder: HpackEncoder::new(),
            hpack_decoder: HpackDecoder::new(),
            connection_window: 65535,
        })
    }

    /// Send SETTINGS frame
    fn send_settings_frame(stream: &mut TcpStream, settings: &SettingsFrame) -> Result<()> {
        // Frame header
        let payload_len = settings.settings.len() * 6;
        let header = FrameHeader {
            length: payload_len as u32,
            frame_type: 0x04, // SETTINGS
            flags: 0,
            stream_id: 0,
        };

        stream.write_all(&header.to_bytes())?;

        // Settings payload
        for setting in &settings.settings {
            let mut buf = [0u8; 6];
            buf[0] = ((setting.id >> 8) & 0xFF) as u8;
            buf[1] = (setting.id & 0xFF) as u8;
            buf[2] = ((setting.value >> 24) & 0xFF) as u8;
            buf[3] = ((setting.value >> 16) & 0xFF) as u8;
            buf[4] = ((setting.value >> 8) & 0xFF) as u8;
            buf[5] = (setting.value & 0xFF) as u8;
            stream.write_all(&buf)?;
        }

        stream.flush()?;
        Ok(())
    }

    /// Create new stream and send request
    pub fn request(
        &mut self,
        method: &str,
        path: &str,
        authority: &str,
        headers: Vec<(String, String)>,
        body: Option<Bytes>,
    ) -> Result<u32> {
        let stream_id = self.next_stream_id;
        self.next_stream_id += 2; // Skip even IDs (server-initiated)

        // Create stream
        let stream = Stream::new(stream_id, self.settings.initial_window_size);
        self.streams.insert(stream_id, stream);

        // Build pseudo-headers (must come first)
        let mut all_headers = vec![
            (":method".to_string(), method.to_string()),
            (":path".to_string(), path.to_string()),
            (":scheme".to_string(), "https".to_string()),
            (":authority".to_string(), authority.to_string()),
        ];
        all_headers.extend(headers);

        // Encode headers with HPACK
        let encoded_headers = self.hpack_encoder.encode(&all_headers)?;

        // Send HEADERS frame
        let mut flags = 0x04; // END_HEADERS
        if body.is_none() {
            flags |= 0x01; // END_STREAM
        }

        let header = FrameHeader {
            length: encoded_headers.len() as u32,
            frame_type: 0x01, // HEADERS
            flags,
            stream_id,
        };

        self.stream.write_all(&header.to_bytes())?;
        self.stream.write_all(&encoded_headers)?;

        // Send DATA frame if body exists
        if let Some(body_data) = body {
            self.send_data(stream_id, body_data, true)?;
        }

        self.stream.flush()?;
        Ok(stream_id)
    }

    /// Send DATA frame
    fn send_data(&mut self, stream_id: u32, data: Bytes, end_stream: bool) -> Result<()> {
        let mut flags = 0u8;
        if end_stream {
            flags |= 0x01; // END_STREAM
        }

        let header = FrameHeader {
            length: data.len() as u32,
            frame_type: 0x00, // DATA
            flags,
            stream_id,
        };

        self.stream.write_all(&header.to_bytes())?;
        self.stream.write_all(&data)?;

        Ok(())
    }

    /// Read and process next frame
    pub fn read_frame(&mut self) -> Result<Option<(u32, Frame)>> {
        // Read frame header (9 bytes)
        let mut header_buf = [0u8; 9];
        if let Err(_) = self.stream.read_exact(&mut header_buf) {
            return Ok(None); // Connection closed
        }

        let header = FrameHeader::parse(&header_buf)?;

        // Read payload
        let mut payload = vec![0u8; header.length as usize];
        self.stream.read_exact(&mut payload)?;

        // Parse frame based on type
        let frame = match header.frame_type {
            0x00 => {
                // DATA frame
                let data_frame = DataFrame::parse(&header, &payload)?;
                Frame::Data(data_frame)
            }
            0x01 => {
                // HEADERS frame
                let headers_frame = HeadersFrame {
                    stream_id: header.stream_id,
                    header_block: Bytes::from_vec(payload),
                    end_stream: (header.flags & 0x01) != 0,
                    end_headers: (header.flags & 0x04) != 0,
                    padded: (header.flags & 0x08) != 0,
                    priority: None,
                };
                Frame::Headers(headers_frame)
            }
            0x04 => {
                // SETTINGS frame
                Frame::Settings(SettingsFrame {
                    ack: (header.flags & 0x01) != 0,
                    settings: Vec::new(),
                })
            }
            _ => {
                // Unknown frame type - skip
                return Ok(None);
            }
        };

        Ok(Some((header.stream_id, frame)))
    }

    /// Get stream by ID
    pub fn get_stream(&self, stream_id: u32) -> Option<&Stream> {
        self.streams.get(&stream_id)
    }

    /// Get mutable stream by ID
    pub fn get_stream_mut(&mut self, stream_id: u32) -> Option<&mut Stream> {
        self.streams.get_mut(&stream_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_settings_default() {
        let settings = ConnectionSettings::default();
        assert_eq!(settings.max_concurrent_streams, 100);
        assert_eq!(settings.initial_window_size, 65535);
    }

    #[test]
    fn test_connection_preface() {
        assert_eq!(CONNECTION_PREFACE.len(), 24);
    }
}
