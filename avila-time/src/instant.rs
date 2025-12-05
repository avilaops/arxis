//! Monotonic time type for measuring elapsed time

use crate::duration::Duration;
use crate::syscalls;
use crate::error::{TimeError, Result};
use core::ops::{Add, Sub};

/// Monotonic, non-decreasing clock for measuring elapsed time
///
/// Unlike SystemTime, Instant is guaranteed to never go backwards and
/// is not affected by system clock adjustments.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Instant {
    secs: u64,
    nanos: u32,
}

impl Instant {
    /// Get the current instant from the monotonic clock
    pub fn now() -> Instant {
        let ts = syscalls::get_monotonic_time()
            .expect("Failed to get monotonic time");
        
        Instant {
            secs: ts.tv_sec as u64,
            nanos: ts.tv_nsec as u32,
        }
    }

    /// Duration since an earlier instant
    ///
    /// Returns None if `earlier` is after `self`
    pub fn duration_since(&self, earlier: Instant) -> Option<Duration> {
        if self < &earlier {
            return None;
        }

        let mut secs = self.secs - earlier.secs;
        let mut nanos = self.nanos as i64 - earlier.nanos as i64;

        if nanos < 0 {
            if secs == 0 {
                return None;
            }
            secs -= 1;
            nanos += 1_000_000_000;
        }

        Some(Duration::new(secs, nanos as u32))
    }

    /// Duration since an earlier instant, saturating to zero
    pub fn saturating_duration_since(&self, earlier: Instant) -> Duration {
        self.duration_since(earlier).unwrap_or(Duration::ZERO)
    }

    /// Checked addition
    pub fn checked_add(&self, duration: Duration) -> Result<Instant> {
        let mut secs = self.secs.checked_add(duration.as_secs())
            .ok_or(TimeError::Overflow)?;
        let mut nanos = self.nanos + duration.subsec_nanos();

        if nanos >= 1_000_000_000 {
            nanos -= 1_000_000_000;
            secs = secs.checked_add(1).ok_or(TimeError::Overflow)?;
        }

        Ok(Instant { secs, nanos })
    }

    /// Checked subtraction
    pub fn checked_sub(&self, duration: Duration) -> Result<Instant> {
        let mut secs = self.secs.checked_sub(duration.as_secs())
            .ok_or(TimeError::Underflow)?;
        let mut nanos = self.nanos as i64 - duration.subsec_nanos() as i64;

        if nanos < 0 {
            if secs == 0 {
                return Err(TimeError::Underflow);
            }
            secs -= 1;
            nanos += 1_000_000_000;
        }

        Ok(Instant { secs, nanos: nanos as u32 })
    }

    /// Duration elapsed since this instant
    pub fn elapsed(&self) -> Duration {
        Instant::now().saturating_duration_since(*self)
    }
}

impl Add<Duration> for Instant {
    type Output = Instant;

    fn add(self, rhs: Duration) -> Instant {
        self.checked_add(rhs).expect("Instant overflow")
    }
}

impl Sub<Duration> for Instant {
    type Output = Instant;

    fn sub(self, rhs: Duration) -> Instant {
        self.checked_sub(rhs).expect("Instant underflow")
    }
}

impl Sub<Instant> for Instant {
    type Output = Duration;

    fn sub(self, rhs: Instant) -> Duration {
        self.duration_since(rhs).expect("Instant subtraction")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instant_now() {
        let t = Instant::now();
        assert!(t.secs > 0 || t.nanos > 0);
    }

    #[test]
    fn test_instant_ordering() {
        let t1 = Instant::now();
        // Small busy loop to ensure time passes
        for _ in 0..1000 {
            core::hint::black_box(());
        }
        let t2 = Instant::now();
        assert!(t2 >= t1);
    }

    #[test]
    fn test_duration_since() {
        let t1 = Instant::now();
        for _ in 0..10000 {
            core::hint::black_box(());
        }
        let t2 = Instant::now();
        let duration = t2.duration_since(t1);
        assert!(duration.is_some());
    }

    #[test]
    fn test_instant_add() {
        let t1 = Instant::now();
        let d = Duration::from_secs(10);
        let t2 = t1 + d;
        let diff = t2.duration_since(t1).unwrap();
        assert_eq!(diff.as_secs(), 10);
    }

    #[test]
    fn test_instant_sub() {
        let t1 = Instant::now();
        let d = Duration::from_secs(10);
        let t2 = t1 + d;
        let diff = t2 - t1;
        assert_eq!(diff.as_secs(), 10);
    }

    #[test]
    fn test_elapsed() {
        let start = Instant::now();
        for _ in 0..100000 {
            core::hint::black_box(());
        }
        let elapsed = start.elapsed();
        // Should have some elapsed time
        assert!(elapsed.as_nanos() > 0);
    }
}
