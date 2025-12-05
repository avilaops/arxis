//! Error types for avila-time

use core::fmt;

/// Result type for time operations
pub type Result<T> = core::result::Result<T, TimeError>;

/// Time-related errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeError {
    /// Invalid timestamp value
    InvalidTimestamp,
    /// Invalid date component (year, month, day)
    InvalidDate,
    /// Invalid time component (hour, minute, second)
    InvalidTime,
    /// Parse error
    ParseError,
    /// Overflow in arithmetic operation
    Overflow,
    /// Underflow in arithmetic operation
    Underflow,
    /// System call failed
    SystemCallFailed,
}

impl fmt::Display for TimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TimeError::InvalidTimestamp => write!(f, "Invalid timestamp"),
            TimeError::InvalidDate => write!(f, "Invalid date"),
            TimeError::InvalidTime => write!(f, "Invalid time"),
            TimeError::ParseError => write!(f, "Parse error"),
            TimeError::Overflow => write!(f, "Overflow"),
            TimeError::Underflow => write!(f, "Underflow"),
            TimeError::SystemCallFailed => write!(f, "System call failed"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for TimeError {}
