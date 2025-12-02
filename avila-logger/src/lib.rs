//! # avila-logger - Structured Logging
//!
//! Fast structured logging with compile-time optimization.

#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]

#[cfg(feature = "std")]
use std::sync::Mutex;

/// Log level
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    /// Error messages
    Error = 1,
    /// Warning messages
    Warn = 2,
    /// Info messages
    Info = 3,
    /// Debug messages
    Debug = 4,
    /// Trace messages
    Trace = 5,
}

impl Level {
    /// Returns level name
    pub const fn as_str(&self) -> &'static str {
        match self {
            Level::Error => "ERROR",
            Level::Warn => "WARN",
            Level::Info => "INFO",
            Level::Debug => "DEBUG",
            Level::Trace => "TRACE",
        }
    }
}

/// Log record
#[derive(Debug)]
pub struct Record<'a> {
    /// Log level
    pub level: Level,
    /// Message
    pub message: &'a str,
    /// Module path
    pub module: &'a str,
    /// File name
    pub file: &'a str,
    /// Line number
    pub line: u32,
}

/// Logger trait
pub trait Logger: Send + Sync {
    /// Logs a record
    fn log(&self, record: &Record);
    /// Checks if level is enabled
    fn enabled(&self, level: Level) -> bool;
}

/// Simple console logger
#[cfg(feature = "std")]
pub struct ConsoleLogger {
    level: Mutex<Level>,
}

#[cfg(feature = "std")]
impl ConsoleLogger {
    /// Creates new console logger
    pub fn new(level: Level) -> Self {
        Self {
            level: Mutex::new(level),
        }
    }

    /// Sets log level
    pub fn set_level(&self, level: Level) {
        *self.level.lock().unwrap() = level;
    }
}

#[cfg(feature = "std")]
impl Logger for ConsoleLogger {
    fn log(&self, record: &Record) {
        if self.enabled(record.level) {
            println!(
                "[{}] {}:{} - {}",
                record.level.as_str(),
                record.module,
                record.line,
                record.message
            );
        }
    }

    fn enabled(&self, level: Level) -> bool {
        level <= *self.level.lock().unwrap()
    }
}

#[cfg(feature = "std")]
static LOGGER: Mutex<Option<&'static dyn Logger>> = Mutex::new(None);

/// Sets global logger
#[cfg(feature = "std")]
pub fn set_logger(logger: &'static dyn Logger) {
    *LOGGER.lock().unwrap() = Some(logger);
}

/// Gets global logger
#[cfg(feature = "std")]
fn get_logger() -> Option<&'static dyn Logger> {
    *LOGGER.lock().unwrap()
}

/// Logs a message
#[cfg(feature = "std")]
pub fn log(level: Level, module: &str, file: &str, line: u32, message: &str) {
    if let Some(logger) = get_logger() {
        let record = Record {
            level,
            message,
            module,
            file,
            line,
        };
        logger.log(&record);
    }
}

/// Error log macro
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        #[cfg(feature = "std")]
        $crate::log(
            $crate::Level::Error,
            module_path!(),
            file!(),
            line!(),
            &format!($($arg)*)
        )
    };
}

/// Info log macro
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        #[cfg(feature = "std")]
        $crate::log(
            $crate::Level::Info,
            module_path!(),
            file!(),
            line!(),
            &format!($($arg)*)
        )
    };
}

/// Debug log macro
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        #[cfg(feature = "std")]
        $crate::log(
            $crate::Level::Debug,
            module_path!(),
            file!(),
            line!(),
            &format!($($arg)*)
        )
    };
}

/// Prelude
pub mod prelude {
    pub use crate::{Level, Logger};
    #[cfg(feature = "std")]
    pub use crate::{ConsoleLogger, set_logger};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level_ordering() {
        assert!(Level::Error < Level::Warn);
        assert!(Level::Warn < Level::Info);
        assert!(Level::Info < Level::Debug);
    }

    #[test]
    #[cfg(feature = "std")]
    fn test_console_logger() {
        let logger = ConsoleLogger::new(Level::Info);
        assert!(logger.enabled(Level::Error));
        assert!(logger.enabled(Level::Info));
        assert!(!logger.enabled(Level::Debug));
    }
}
