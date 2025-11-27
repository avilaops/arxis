//! HTTP/2 stream management

use crate::bytes::Bytes;

/// HTTP/2 stream state machine (RFC 7540 Section 5.1)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StreamState {
    /// Stream not yet created
    Idle,
    /// Stream open for both sides
    Open,
    /// Local side closed
    HalfClosedLocal,
    /// Remote side closed
    HalfClosedRemote,
    /// Stream fully closed
    Closed,
}

/// HTTP/2 stream
pub struct Stream {
    /// Stream identifier
    pub id: u32,
    /// Current state
    pub state: StreamState,
    /// Send window size (flow control)
    pub send_window: i32,
    /// Receive window size (flow control)
    pub recv_window: i32,
    /// Received headers
    pub headers: Option<Vec<(String, String)>>,
    /// Received data chunks
    pub data: Vec<Bytes>,
    /// Total bytes received
    pub bytes_received: usize,
}

impl Stream {
    /// Create new stream
    pub fn new(id: u32, initial_window_size: u32) -> Self {
        Self {
            id,
            state: StreamState::Idle,
            send_window: initial_window_size as i32,
            recv_window: initial_window_size as i32,
            headers: None,
            data: Vec::new(),
            bytes_received: 0,
        }
    }

    /// Check if stream can send data
    pub fn can_send(&self) -> bool {
        matches!(self.state, StreamState::Open | StreamState::HalfClosedRemote)
    }

    /// Check if stream can receive data
    pub fn can_receive(&self) -> bool {
        matches!(self.state, StreamState::Open | StreamState::HalfClosedLocal)
    }

    /// Transition stream state on send end
    pub fn close_local(&mut self) {
        match self.state {
            StreamState::Open => self.state = StreamState::HalfClosedLocal,
            StreamState::HalfClosedRemote => self.state = StreamState::Closed,
            _ => {}
        }
    }

    /// Transition stream state on receive end
    pub fn close_remote(&mut self) {
        match self.state {
            StreamState::Open => self.state = StreamState::HalfClosedRemote,
            StreamState::HalfClosedLocal => self.state = StreamState::Closed,
            _ => {}
        }
    }

    /// Add received data
    pub fn add_data(&mut self, data: Bytes) {
        self.bytes_received += data.len();
        self.data.push(data);
    }

    /// Get all received data as single buffer
    pub fn get_body(&self) -> Bytes {
        if self.data.is_empty() {
            return Bytes::new();
        }

        if self.data.len() == 1 {
            return self.data[0].clone();
        }

        // Concatenate all chunks
        let total_len: usize = self.data.iter().map(|b| b.len()).sum();
        let mut result = Vec::with_capacity(total_len);

        for chunk in &self.data {
            result.extend_from_slice(chunk.as_slice());
        }

        Bytes::from_vec(result)
    }

    /// Update send window (for flow control)
    pub fn update_send_window(&mut self, increment: i32) {
        self.send_window += increment;
    }

    /// Update receive window (for flow control)
    pub fn update_recv_window(&mut self, increment: i32) {
        self.recv_window += increment;
    }

    /// Consume send window (when sending data)
    pub fn consume_send_window(&mut self, amount: usize) -> bool {
        if self.send_window >= amount as i32 {
            self.send_window -= amount as i32;
            true
        } else {
            false
        }
    }

    /// Consume receive window (when receiving data)
    pub fn consume_recv_window(&mut self, amount: usize) {
        self.recv_window -= amount as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stream_creation() {
        let stream = Stream::new(1, 65535);
        assert_eq!(stream.id, 1);
        assert_eq!(stream.state, StreamState::Idle);
        assert_eq!(stream.send_window, 65535);
    }

    #[test]
    fn test_stream_state_transitions() {
        let mut stream = Stream::new(1, 65535);

        stream.state = StreamState::Open;
        assert!(stream.can_send());
        assert!(stream.can_receive());

        stream.close_local();
        assert_eq!(stream.state, StreamState::HalfClosedLocal);
        assert!(!stream.can_send());
        assert!(stream.can_receive());

        stream.close_remote();
        assert_eq!(stream.state, StreamState::Closed);
        assert!(!stream.can_send());
        assert!(!stream.can_receive());
    }

    #[test]
    fn test_flow_control() {
        let mut stream = Stream::new(1, 100);

        assert!(stream.consume_send_window(50));
        assert_eq!(stream.send_window, 50);

        assert!(stream.consume_send_window(50));
        assert_eq!(stream.send_window, 0);

        assert!(!stream.consume_send_window(1));
    }

    #[test]
    fn test_data_accumulation() {
        let mut stream = Stream::new(1, 65535);

        stream.add_data(Bytes::from("Hello, "));
        stream.add_data(Bytes::from("HTTP/2!"));

        let body = stream.get_body();
        assert_eq!(body.as_slice(), b"Hello, HTTP/2!");
        assert_eq!(stream.bytes_received, 14);
    }
}
