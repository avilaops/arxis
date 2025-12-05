//! # Avila Time - Layer 1 Temporal Operations
//!
//! Complete time library without chrono or std::time
//! 
//! ## Features
//! - `Duration` (nanosecond resolution)
//! - `Instant` (monotonic time)
//! - `SystemTime` (wall clock)
//! - `DateTime` (UTC and local)
//! - ISO 8601 parsing/formatting
//! - Temporal arithmetic operations
//!
//! ## Absolute Rules
//! - ❌ NO `chrono`
//! - ❌ NO `std::time`
//! - ✅ `#![no_std]` compatible
//! - ✅ Direct syscalls (platform-specific)

#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]
#![allow(clippy::all)]

#[cfg(feature = "std")]
extern crate std;

#[cfg(not(feature = "std"))]
extern crate core as std;

mod duration;
mod instant;
mod system_time;
mod datetime;
mod parser;
mod syscalls;
mod error;

pub use duration::Duration;
pub use instant::Instant;
pub use system_time::SystemTime;
pub use datetime::{DateTime, TimeZone};
pub use error::{TimeError, Result};

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::{Duration, Instant, SystemTime, DateTime, TimeZone};
    pub use crate::error::{TimeError, Result};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duration_creation() {
        let d = Duration::from_secs(10);
        assert_eq!(d.as_secs(), 10);
    }

    #[test]
    fn test_duration_add() {
        let d1 = Duration::from_secs(10);
        let d2 = Duration::from_secs(5);
        let d3 = d1 + d2;
        assert_eq!(d3.as_secs(), 15);
    }

    #[test]
    fn test_instant_ordering() {
        let t1 = Instant::now();
        let t2 = Instant::now();
        assert!(t2 >= t1);
    }

    #[test]
    fn test_system_time_creation() {
        let st = SystemTime::now();
        let duration = st.duration_since_unix_epoch();
        assert!(duration.as_secs() > 0);
    }

    #[test]
    fn test_datetime_creation() {
        let dt = DateTime::from_timestamp(1640995200, 0);
        assert_eq!(dt.timestamp(), 1640995200);
    }

    #[test]
    fn test_datetime_formatting() {
        let dt = DateTime::from_timestamp(0, 0); // Unix epoch
        let formatted = dt.to_iso8601();
        assert!(formatted.contains("1970"));
    }
}
