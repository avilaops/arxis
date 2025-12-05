//! Duration type with nanosecond resolution

use core::ops::{Add, Sub, Mul, Div};
use crate::error::{TimeError, Result};

/// Represents a duration with nanosecond precision
/// 
/// Internally stores seconds and nanoseconds separately for efficiency
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Duration {
    secs: u64,
    nanos: u32, // Always < 1_000_000_000
}

impl Duration {
    /// Maximum value for nanoseconds
    pub const NANOS_PER_SEC: u32 = 1_000_000_000;
    
    /// One second duration
    pub const SECOND: Duration = Duration { secs: 1, nanos: 0 };
    
    /// One millisecond duration
    pub const MILLISECOND: Duration = Duration { secs: 0, nanos: 1_000_000 };
    
    /// One microsecond duration
    pub const MICROSECOND: Duration = Duration { secs: 0, nanos: 1_000 };
    
    /// One nanosecond duration
    pub const NANOSECOND: Duration = Duration { secs: 0, nanos: 1 };
    
    /// Zero duration
    pub const ZERO: Duration = Duration { secs: 0, nanos: 0 };

    /// Create a new duration from seconds and nanoseconds
    #[inline]
    pub const fn new(secs: u64, nanos: u32) -> Duration {
        let extra_secs = nanos / Self::NANOS_PER_SEC;
        let nanos = nanos % Self::NANOS_PER_SEC;
        Duration {
            secs: secs + extra_secs as u64,
            nanos,
        }
    }

    /// Create duration from seconds
    #[inline]
    pub const fn from_secs(secs: u64) -> Duration {
        Duration { secs, nanos: 0 }
    }

    /// Create duration from milliseconds
    #[inline]
    pub const fn from_millis(millis: u64) -> Duration {
        Duration {
            secs: millis / 1000,
            nanos: ((millis % 1000) * 1_000_000) as u32,
        }
    }

    /// Create duration from microseconds
    #[inline]
    pub const fn from_micros(micros: u64) -> Duration {
        Duration {
            secs: micros / 1_000_000,
            nanos: ((micros % 1_000_000) * 1_000) as u32,
        }
    }

    /// Create duration from nanoseconds
    #[inline]
    pub const fn from_nanos(nanos: u64) -> Duration {
        Duration {
            secs: nanos / Self::NANOS_PER_SEC as u64,
            nanos: (nanos % Self::NANOS_PER_SEC as u64) as u32,
        }
    }

    /// Get total seconds (truncated)
    #[inline]
    pub const fn as_secs(&self) -> u64 {
        self.secs
    }

    /// Get total milliseconds
    #[inline]
    pub const fn as_millis(&self) -> u128 {
        self.secs as u128 * 1000 + (self.nanos / 1_000_000) as u128
    }

    /// Get total microseconds
    #[inline]
    pub const fn as_micros(&self) -> u128 {
        self.secs as u128 * 1_000_000 + (self.nanos / 1_000) as u128
    }

    /// Get total nanoseconds
    #[inline]
    pub const fn as_nanos(&self) -> u128 {
        self.secs as u128 * Self::NANOS_PER_SEC as u128 + self.nanos as u128
    }

    /// Get subsecond nanoseconds
    #[inline]
    pub const fn subsec_nanos(&self) -> u32 {
        self.nanos
    }

    /// Get subsecond milliseconds
    #[inline]
    pub const fn subsec_millis(&self) -> u32 {
        self.nanos / 1_000_000
    }

    /// Get subsecond microseconds
    #[inline]
    pub const fn subsec_micros(&self) -> u32 {
        self.nanos / 1_000
    }

    /// Checked addition
    pub fn checked_add(self, rhs: Duration) -> Result<Duration> {
        let mut secs = self.secs.checked_add(rhs.secs)
            .ok_or(TimeError::Overflow)?;
        let mut nanos = self.nanos + rhs.nanos;
        
        if nanos >= Self::NANOS_PER_SEC {
            nanos -= Self::NANOS_PER_SEC;
            secs = secs.checked_add(1).ok_or(TimeError::Overflow)?;
        }
        
        Ok(Duration { secs, nanos })
    }

    /// Checked subtraction
    pub fn checked_sub(self, rhs: Duration) -> Result<Duration> {
        if self < rhs {
            return Err(TimeError::Underflow);
        }
        
        let mut secs = self.secs - rhs.secs;
        let mut nanos = self.nanos as i64 - rhs.nanos as i64;
        
        if nanos < 0 {
            if secs == 0 {
                return Err(TimeError::Underflow);
            }
            secs -= 1;
            nanos += Self::NANOS_PER_SEC as i64;
        }
        
        Ok(Duration { secs, nanos: nanos as u32 })
    }

    /// Checked multiplication
    pub fn checked_mul(self, rhs: u32) -> Result<Duration> {
        let total_nanos = self.nanos as u64 * rhs as u64;
        let extra_secs = total_nanos / Self::NANOS_PER_SEC as u64;
        let nanos = (total_nanos % Self::NANOS_PER_SEC as u64) as u32;
        
        let secs = self.secs.checked_mul(rhs as u64)
            .and_then(|s| s.checked_add(extra_secs))
            .ok_or(TimeError::Overflow)?;
        
        Ok(Duration { secs, nanos })
    }

    /// Checked division
    pub fn checked_div(self, rhs: u32) -> Result<Duration> {
        if rhs == 0 {
            return Err(TimeError::Overflow);
        }
        
        let total_nanos = self.as_nanos();
        let result_nanos = total_nanos / rhs as u128;
        
        Ok(Duration::from_nanos(result_nanos as u64))
    }

    /// Check if duration is zero
    #[inline]
    pub const fn is_zero(&self) -> bool {
        self.secs == 0 && self.nanos == 0
    }
}

impl Add for Duration {
    type Output = Duration;

    fn add(self, rhs: Duration) -> Duration {
        self.checked_add(rhs).expect("Duration overflow")
    }
}

impl Sub for Duration {
    type Output = Duration;

    fn sub(self, rhs: Duration) -> Duration {
        self.checked_sub(rhs).expect("Duration underflow")
    }
}

impl Mul<u32> for Duration {
    type Output = Duration;

    fn mul(self, rhs: u32) -> Duration {
        self.checked_mul(rhs).expect("Duration overflow")
    }
}

impl Div<u32> for Duration {
    type Output = Duration;

    fn div(self, rhs: u32) -> Duration {
        self.checked_div(rhs).expect("Duration division")
    }
}

impl Default for Duration {
    fn default() -> Self {
        Duration::ZERO
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duration_creation() {
        let d = Duration::from_secs(10);
        assert_eq!(d.as_secs(), 10);
        assert_eq!(d.subsec_nanos(), 0);
    }

    #[test]
    fn test_duration_from_millis() {
        let d = Duration::from_millis(1500);
        assert_eq!(d.as_secs(), 1);
        assert_eq!(d.subsec_millis(), 500);
    }

    #[test]
    fn test_duration_from_nanos() {
        let d = Duration::from_nanos(1_500_000_000);
        assert_eq!(d.as_secs(), 1);
        assert_eq!(d.subsec_nanos(), 500_000_000);
    }

    #[test]
    fn test_duration_add() {
        let d1 = Duration::from_secs(10);
        let d2 = Duration::from_millis(500);
        let d3 = d1 + d2;
        assert_eq!(d3.as_secs(), 10);
        assert_eq!(d3.subsec_millis(), 500);
    }

    #[test]
    fn test_duration_sub() {
        let d1 = Duration::from_millis(1500);
        let d2 = Duration::from_millis(500);
        let d3 = d1 - d2;
        assert_eq!(d3.as_millis(), 1000);
    }

    #[test]
    fn test_duration_mul() {
        let d = Duration::from_secs(10);
        let d2 = d * 3;
        assert_eq!(d2.as_secs(), 30);
    }

    #[test]
    fn test_duration_div() {
        let d = Duration::from_secs(30);
        let d2 = d / 3;
        assert_eq!(d2.as_secs(), 10);
    }

    #[test]
    fn test_duration_overflow() {
        let d = Duration::from_secs(u64::MAX);
        assert!(d.checked_add(Duration::from_secs(1)).is_err());
    }

    #[test]
    fn test_duration_underflow() {
        let d1 = Duration::from_secs(5);
        let d2 = Duration::from_secs(10);
        assert!(d1.checked_sub(d2).is_err());
    }
}
