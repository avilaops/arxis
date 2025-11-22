//! Error types for avx-http

use std::fmt;

/// Result type alias for avx-http operations
pub type Result<T> = std::result::Result<T, Error>;

/// Error types for HTTP operations
#[derive(Debug)]
pub enum Error {
    /// Invalid URL provided
    InvalidUrl { url: String, reason: String },

    /// Connection failed
    ConnectionFailed { addr: String, source: std::io::Error },

    /// Request timeout
    Timeout { duration: std::time::Duration },

    /// Invalid HTTP method
    InvalidMethod { method: String },

    /// Invalid header
    InvalidHeader { name: String, value: String },

    /// HTTP status error
    StatusError { status: u16, body: String },

    /// Body read error
    BodyReadError { source: std::io::Error },

    /// JSON serialization/deserialization error
    JsonError { source: String },

    /// Invalid configuration
    InvalidConfig { message: String },

    /// Authentication error
    AuthError { message: String },

    /// Internal error
    Internal { message: String },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidUrl { url, reason } => {
                write!(f, "Invalid URL '{}': {}", url, reason)
            }
            Error::ConnectionFailed { addr, source } => {
                write!(f, "Failed to connect to {}: {}", addr, source)
            }
            Error::Timeout { duration } => {
                write!(f, "Request timed out after {:?}", duration)
            }
            Error::InvalidMethod { method } => {
                write!(f, "Invalid HTTP method: {}", method)
            }
            Error::InvalidHeader { name, value } => {
                write!(f, "Invalid header '{}': {}", name, value)
            }
            Error::StatusError { status, body } => {
                write!(f, "HTTP error {}: {}", status, body)
            }
            Error::BodyReadError { source } => {
                write!(f, "Failed to read response body: {}", source)
            }
            Error::JsonError { source } => {
                write!(f, "JSON error: {}", source)
            }
            Error::InvalidConfig { message } => {
                write!(f, "Invalid configuration: {}", message)
            }
            Error::AuthError { message } => {
                write!(f, "Authentication error: {}", message)
            }
            Error::Internal { message } => {
                write!(f, "Internal error: {}", message)
            }
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::ConnectionFailed { source, .. } => Some(source),
            Error::BodyReadError { source } => Some(source),
            _ => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::BodyReadError { source: err }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = Error::InvalidUrl {
            url: "not-a-url".to_string(),
            reason: "missing scheme".to_string(),
        };
        assert!(err.to_string().contains("Invalid URL"));
        assert!(err.to_string().contains("not-a-url"));
    }

    #[test]
    fn test_error_from_io() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let err: Error = io_err.into();
        assert!(matches!(err, Error::BodyReadError { .. }));
    }

    #[test]
    fn test_timeout_error() {
        let err = Error::Timeout {
            duration: std::time::Duration::from_secs(30),
        };
        assert!(err.to_string().contains("timed out"));
        assert!(err.to_string().contains("30s"));
    }
}
