//! Error types for AVL Queue

use thiserror::Error;

/// Result type for AVL Queue operations
pub type Result<T> = std::result::Result<T, QueueError>;

/// Error types for queue operations
#[derive(Debug, Error)]
pub enum QueueError {
    /// Connection error
    #[error("Connection error: {0}")]
    Connection(String),

    /// Topic not found
    #[error("Topic not found: {0}")]
    TopicNotFound(String),

    /// Message too large
    #[error("Message size {0} exceeds maximum {1} bytes")]
    MessageTooLarge(usize, usize),

    /// Compression error
    #[error("Compression error: {0}")]
    Compression(String),

    /// Storage error
    #[error("Storage error: {0}")]
    Storage(String),

    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// Invalid message
    #[error("Invalid message: {0}")]
    InvalidMessage(String),

    /// Timeout error
    #[error("Operation timed out")]
    Timeout,

    /// Channel closed
    #[error("Channel closed")]
    ChannelClosed,

    /// Dead letter queue error
    #[error("Dead letter queue error: {0}")]
    DeadLetter(String),

    /// Message field error
    #[error("Message field error: {0}")]
    MessageField(String),

    /// Generic error
    #[error("Queue error: {0}")]
    Other(String),
}

impl From<anyhow::Error> for QueueError {
    fn from(err: anyhow::Error) -> Self {
        QueueError::Other(err.to_string())
    }
}
