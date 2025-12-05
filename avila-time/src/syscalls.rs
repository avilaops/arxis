//! Platform-specific syscalls for time operations
//!
//! Direct syscalls without using std::time

use crate::error::{TimeError, Result};

/// Timespec structure for syscalls
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

impl Timespec {
    pub const fn zero() -> Self {
        Timespec {
            tv_sec: 0,
            tv_nsec: 0,
        }
    }
}

// Linux syscalls
#[cfg(target_os = "linux")]
mod linux {
    use super::*;

    const CLOCK_REALTIME: i32 = 0;
    const CLOCK_MONOTONIC: i32 = 1;

    extern "C" {
        fn clock_gettime(clk_id: i32, tp: *mut Timespec) -> i32;
    }

    pub fn get_monotonic_time() -> Result<Timespec> {
        unsafe {
            let mut ts = Timespec::zero();
            if clock_gettime(CLOCK_MONOTONIC, &mut ts) != 0 {
                return Err(TimeError::SystemCallFailed);
            }
            Ok(ts)
        }
    }

    pub fn get_system_time() -> Result<Timespec> {
        unsafe {
            let mut ts = Timespec::zero();
            if clock_gettime(CLOCK_REALTIME, &mut ts) != 0 {
                return Err(TimeError::SystemCallFailed);
            }
            Ok(ts)
        }
    }
}

// macOS/iOS syscalls
#[cfg(any(target_os = "macos", target_os = "ios"))]
mod macos {
    use super::*;

    const CLOCK_REALTIME: i32 = 0;
    const CLOCK_MONOTONIC: i32 = 6;

    extern "C" {
        fn clock_gettime(clk_id: i32, tp: *mut Timespec) -> i32;
    }

    pub fn get_monotonic_time() -> Result<Timespec> {
        unsafe {
            let mut ts = Timespec::zero();
            if clock_gettime(CLOCK_MONOTONIC, &mut ts) != 0 {
                return Err(TimeError::SystemCallFailed);
            }
            Ok(ts)
        }
    }

    pub fn get_system_time() -> Result<Timespec> {
        unsafe {
            let mut ts = Timespec::zero();
            if clock_gettime(CLOCK_REALTIME, &mut ts) != 0 {
                return Err(TimeError::SystemCallFailed);
            }
            Ok(ts)
        }
    }
}

// Windows syscalls
#[cfg(target_os = "windows")]
mod windows {
    use super::*;

    #[repr(C)]
    struct FILETIME {
        dwLowDateTime: u32,
        dwHighDateTime: u32,
    }

    extern "system" {
        fn GetSystemTimeAsFileTime(lpSystemTimeAsFileTime: *mut FILETIME);
        fn QueryPerformanceCounter(lpPerformanceCount: *mut i64) -> i32;
        fn QueryPerformanceFrequency(lpFrequency: *mut i64) -> i32;
    }

    // Windows FILETIME is 100-nanosecond intervals since January 1, 1601
    const FILETIME_TO_UNIX_EPOCH: i64 = 116444736000000000;

    pub fn get_monotonic_time() -> Result<Timespec> {
        unsafe {
            let mut counter: i64 = 0;
            let mut frequency: i64 = 0;

            if QueryPerformanceFrequency(&mut frequency) == 0 {
                return Err(TimeError::SystemCallFailed);
            }
            if QueryPerformanceCounter(&mut counter) == 0 {
                return Err(TimeError::SystemCallFailed);
            }

            let secs = counter / frequency;
            let nanos = ((counter % frequency) * 1_000_000_000) / frequency;

            Ok(Timespec {
                tv_sec: secs,
                tv_nsec: nanos,
            })
        }
    }

    pub fn get_system_time() -> Result<Timespec> {
        unsafe {
            let mut ft = FILETIME {
                dwLowDateTime: 0,
                dwHighDateTime: 0,
            };
            GetSystemTimeAsFileTime(&mut ft);

            let intervals = ((ft.dwHighDateTime as i64) << 32) | (ft.dwLowDateTime as i64);
            let unix_intervals = intervals - FILETIME_TO_UNIX_EPOCH;

            let secs = unix_intervals / 10_000_000;
            let nanos = (unix_intervals % 10_000_000) * 100;

            Ok(Timespec {
                tv_sec: secs,
                tv_nsec: nanos,
            })
        }
    }
}

// WASI syscalls
#[cfg(target_os = "wasi")]
mod wasi {
    use super::*;

    const CLOCKID_REALTIME: u32 = 0;
    const CLOCKID_MONOTONIC: u32 = 1;

    extern "C" {
        fn clock_time_get(id: u32, precision: u64, time: *mut u64) -> u16;
    }

    pub fn get_monotonic_time() -> Result<Timespec> {
        unsafe {
            let mut time: u64 = 0;
            if clock_time_get(CLOCKID_MONOTONIC, 1, &mut time) != 0 {
                return Err(TimeError::SystemCallFailed);
            }
            Ok(Timespec {
                tv_sec: (time / 1_000_000_000) as i64,
                tv_nsec: (time % 1_000_000_000) as i64,
            })
        }
    }

    pub fn get_system_time() -> Result<Timespec> {
        unsafe {
            let mut time: u64 = 0;
            if clock_time_get(CLOCKID_REALTIME, 1, &mut time) != 0 {
                return Err(TimeError::SystemCallFailed);
            }
            Ok(Timespec {
                tv_sec: (time / 1_000_000_000) as i64,
                tv_nsec: (time % 1_000_000_000) as i64,
            })
        }
    }
}

// Fallback for unsupported platforms (returns error)
#[cfg(not(any(
    target_os = "linux",
    target_os = "macos",
    target_os = "ios",
    target_os = "windows",
    target_os = "wasi"
)))]
mod unsupported {
    use super::*;

    pub fn get_monotonic_time() -> Result<Timespec> {
        Err(TimeError::SystemCallFailed)
    }

    pub fn get_system_time() -> Result<Timespec> {
        Err(TimeError::SystemCallFailed)
    }
}

// Public API
#[cfg(target_os = "linux")]
pub use linux::{get_monotonic_time, get_system_time};

#[cfg(any(target_os = "macos", target_os = "ios"))]
pub use macos::{get_monotonic_time, get_system_time};

#[cfg(target_os = "windows")]
pub use windows::{get_monotonic_time, get_system_time};

#[cfg(target_os = "wasi")]
pub use wasi::{get_monotonic_time, get_system_time};

#[cfg(not(any(
    target_os = "linux",
    target_os = "macos",
    target_os = "ios",
    target_os = "windows",
    target_os = "wasi"
)))]
pub use unsupported::{get_monotonic_time, get_system_time};
