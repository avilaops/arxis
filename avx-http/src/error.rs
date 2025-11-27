//! Error types for avx-http (pure std, no dependencies)

use std::fmt;
use std::io;

/// Result type alias for avx-http operations
pub type Result<T> = std::result::Result<T, Error>;

/// Error types for HTTP operations
#[derive(Debug)]
pub enum Error {
    /// IO error
    Io(io::Error),

    /// Invalid URL provided
    InvalidUrl {
        /// The invalid URL
        url: String,
        /// Reason why it's invalid
        reason: String,
    },

    /// Connection failed
    ConnectionFailed {
        /// Host that failed to connect
        host: String,
        /// Error message
        reason: String,
    },

    /// Connection timeout
    Timeout {
        /// Duration that was exceeded in seconds
        seconds: u64,
    },

    /// Invalid HTTP method
    InvalidMethod {
        /// The invalid method
        method: String,
    },

    /// Invalid status code
    InvalidStatusCode {
        /// The invalid code
        code: u16,
    },

    /// Parse error
    ParseError {
        /// Error message
        message: String,
    },

    /// Unexpected end of file
    UnexpectedEof,

    /// Invalid UTF-8
    InvalidUtf8 {
        /// Error message
        message: String,
    },

    /// Internal error
    Internal {
        /// Error message
        message: String,
    },

    /// JSON parsing error
    JsonError {
        /// Error message
        message: String,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "IO error: {}", e),
            Error::InvalidUrl { url, reason } => {
                write!(f, "Invalid URL '{}': {}", url, reason)
            }
            Error::ConnectionFailed { host, reason } => {
                write!(f, "Failed to connect to {}: {}", host, reason)
            }
            Error::Timeout { seconds } => {
                write!(f, "Operation timed out after {}s", seconds)
            }
            Error::InvalidMethod { method } => {
                write!(f, "Invalid HTTP method: {}", method)
            }
            Error::InvalidStatusCode { code } => {
                write!(f, "Invalid status code: {}", code)
            }
            Error::ParseError { message } => {
                write!(f, "Parse error: {}", message)
            }
            Error::UnexpectedEof => {
                write!(f, "Unexpected end of file")
            }
            Error::InvalidUtf8 { message } => {
                write!(f, "Invalid UTF-8: {}", message)
            }
            Error::Internal { message } => {
                write!(f, "Internal error: {}", message)
            }
            Error::JsonError { message } => {
                write!(f, "JSON error: {}", message)
            }
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
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
        let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let err: Error = io_err.into();
        assert!(matches!(err, Error::Io(_)));
    }

    #[test]
    fn test_timeout_error() {
        let err = Error::Timeout { seconds: 30 };
        assert!(err.to_string().contains("timed out"));
        assert!(err.to_string().contains("30"));
    }
}
