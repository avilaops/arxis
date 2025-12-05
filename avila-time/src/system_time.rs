//! System wall clock time

use crate::duration::Duration;
use crate::syscalls;
use crate::error::{TimeError, Result};
use core::ops::{Add, Sub};

/// System wall clock time
///
/// Represents a point in time as measured by the system clock.
/// Unlike Instant, this can go backwards if the system clock is adjusted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SystemTime {
    secs: i64,  // Signed to allow times before Unix epoch
    nanos: u32,
}

impl SystemTime {
    /// Unix epoch: January 1, 1970 00:00:00 UTC
    pub const UNIX_EPOCH: SystemTime = SystemTime { secs: 0, nanos: 0 };

    /// Get the current system time
    pub fn now() -> SystemTime {
        let ts = syscalls::get_system_time()
            .expect("Failed to get system time");
        
        SystemTime {
            secs: ts.tv_sec,
            nanos: ts.tv_nsec as u32,
        }
    }

    /// Create a SystemTime from seconds and nanoseconds since Unix epoch
    pub const fn from_secs_nanos(secs: i64, nanos: u32) -> SystemTime {
        SystemTime { secs, nanos }
    }

    /// Duration since Unix epoch
    pub fn duration_since_unix_epoch(&self) -> Duration {
        if self.secs >= 0 {
            Duration::new(self.secs as u64, self.nanos)
        } else {
            Duration::ZERO
        }
    }

    /// Duration since an earlier time
    ///
    /// Returns error if `earlier` is after `self`
    pub fn duration_since(&self, earlier: SystemTime) -> Result<Duration> {
        if self < &earlier {
            return Err(TimeError::Underflow);
        }

        let mut secs = self.secs - earlier.secs;
        let mut nanos = self.nanos as i64 - earlier.nanos as i64;

        if nanos < 0 {
            if secs == 0 {
                return Err(TimeError::Underflow);
            }
            secs -= 1;
            nanos += 1_000_000_000;
        }

        if secs < 0 {
            return Err(TimeError::Underflow);
        }

        Ok(Duration::new(secs as u64, nanos as u32))
    }

    /// Duration elapsed since this time
    pub fn elapsed(&self) -> Result<Duration> {
        SystemTime::now().duration_since(*self)
    }

    /// Checked addition
    pub fn checked_add(&self, duration: Duration) -> Result<SystemTime> {
        let mut secs = self.secs.checked_add(duration.as_secs() as i64)
            .ok_or(TimeError::Overflow)?;
        let mut nanos = self.nanos + duration.subsec_nanos();

        if nanos >= 1_000_000_000 {
            nanos -= 1_000_000_000;
            secs = secs.checked_add(1).ok_or(TimeError::Overflow)?;
        }

        Ok(SystemTime { secs, nanos })
    }

    /// Checked subtraction
    pub fn checked_sub(&self, duration: Duration) -> Result<SystemTime> {
        let mut secs = self.secs.checked_sub(duration.as_secs() as i64)
            .ok_or(TimeError::Underflow)?;
        let mut nanos = self.nanos as i64 - duration.subsec_nanos() as i64;

        if nanos < 0 {
            secs = secs.checked_sub(1).ok_or(TimeError::Underflow)?;
            nanos += 1_000_000_000;
        }

        Ok(SystemTime { secs, nanos: nanos as u32 })
    }

    /// Get the Unix timestamp (seconds since epoch)
    pub fn unix_timestamp(&self) -> i64 {
        self.secs
    }

    /// Get subsecond nanoseconds
    pub fn subsec_nanos(&self) -> u32 {
        self.nanos
    }
}

impl Add<Duration> for SystemTime {
    type Output = SystemTime;

    fn add(self, rhs: Duration) -> SystemTime {
        self.checked_add(rhs).expect("SystemTime overflow")
    }
}

impl Sub<Duration> for SystemTime {
    type Output = SystemTime;

    fn sub(self, rhs: Duration) -> SystemTime {
        self.checked_sub(rhs).expect("SystemTime underflow")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_time_now() {
        let st = SystemTime::now();
        let duration = st.duration_since_unix_epoch();
        // Should be after year 2000 (946684800 seconds since epoch)
        assert!(duration.as_secs() > 946684800);
    }

    #[test]
    fn test_unix_epoch() {
        let epoch = SystemTime::UNIX_EPOCH;
        assert_eq!(epoch.unix_timestamp(), 0);
        assert_eq!(epoch.subsec_nanos(), 0);
    }

    #[test]
    fn test_duration_since() {
        let st1 = SystemTime::now();
        let st2 = st1 + Duration::from_secs(10);
        let diff = st2.duration_since(st1).unwrap();
        assert_eq!(diff.as_secs(), 10);
    }

    #[test]
    fn test_system_time_add() {
        let st = SystemTime::UNIX_EPOCH;
        let d = Duration::from_secs(3600);
        let st2 = st + d;
        assert_eq!(st2.unix_timestamp(), 3600);
    }

    #[test]
    fn test_system_time_sub() {
        let st = SystemTime::from_secs_nanos(3600, 0);
        let d = Duration::from_secs(1800);
        let st2 = st - d;
        assert_eq!(st2.unix_timestamp(), 1800);
    }

    #[test]
    fn test_before_epoch() {
        let st = SystemTime::from_secs_nanos(-3600, 0);
        assert_eq!(st.unix_timestamp(), -3600);
    }

    #[test]
    fn test_elapsed() {
        let st = SystemTime::now();
        // Small delay
        for _ in 0..100000 {
            core::hint::black_box(());
        }
        let elapsed = st.elapsed().unwrap();
        assert!(elapsed.as_nanos() > 0);
    }
}
