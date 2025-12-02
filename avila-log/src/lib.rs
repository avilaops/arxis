//! Avila Tracing - Sistema de logging nativo
//! Substitui tracing/tracing-subscriber

use std::sync::Mutex;

static LOGGER: Mutex<Option<Box<dyn Logger + Send>>> = Mutex::new(None);

pub trait Logger {
    fn log(&self, level: Level, message: &str);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl Level {
    pub fn as_str(&self) -> &'static str {
        match self {
            Level::Trace => "TRACE",
            Level::Debug => "DEBUG",
            Level::Info => "INFO",
            Level::Warn => "WARN",
            Level::Error => "ERROR",
        }
    }

    pub fn color(&self) -> &'static str {
        match self {
            Level::Trace => "\x1b[90m",   // Gray
            Level::Debug => "\x1b[36m",   // Cyan
            Level::Info => "\x1b[32m",    // Green
            Level::Warn => "\x1b[33m",    // Yellow
            Level::Error => "\x1b[31m",   // Red
        }
    }
}

pub struct ConsoleLogger {
    min_level: Level,
    colored: bool,
}

impl ConsoleLogger {
    pub fn new(min_level: Level) -> Self {
        Self {
            min_level,
            colored: true,
        }
    }

    pub fn with_colors(mut self, colored: bool) -> Self {
        self.colored = colored;
        self
    }
}

impl Logger for ConsoleLogger {
    fn log(&self, level: Level, message: &str) {
        if level < self.min_level {
            return;
        }

        let timestamp = avila_time::DateTime::now();

        if self.colored {
            println!(
                "{}[{}]\x1b[0m {} {}",
                level.color(),
                level.as_str(),
                timestamp.format("%Y-%m-%d %H:%M:%S"),
                message
            );
        } else {
            println!(
                "[{}] {} {}",
                level.as_str(),
                timestamp.format("%Y-%m-%d %H:%M:%S"),
                message
            );
        }
    }
}

pub fn init(logger: impl Logger + Send + 'static) {
    let mut guard = LOGGER.lock().unwrap();
    *guard = Some(Box::new(logger));
}

pub fn trace(message: &str) {
    log(Level::Trace, message);
}

pub fn debug(message: &str) {
    log(Level::Debug, message);
}

pub fn info(message: &str) {
    log(Level::Info, message);
}

pub fn warn(message: &str) {
    log(Level::Warn, message);
}

pub fn error(message: &str) {
    log(Level::Error, message);
}

fn log(level: Level, message: &str) {
    let guard = LOGGER.lock().unwrap();
    if let Some(logger) = guard.as_ref() {
        logger.log(level, message);
    } else {
        // Fallback: print to stderr if no logger initialized
        eprintln!("[{}] {}", level.as_str(), message);
    }
}

#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        $crate::trace(&format!($($arg)*))
    };
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::debug(&format!($($arg)*))
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::info(&format!($($arg)*))
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::warn(&format!($($arg)*))
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::error(&format!($($arg)*))
    };
}

pub struct Span {
    name: String,
    start: std::time::Instant,
}

impl Span {
    pub fn new(name: &str) -> Self {
        debug!("→ Entering span: {}", name);
        Self {
            name: name.to_string(),
            start: std::time::Instant::now(),
        }
    }

    pub fn enter(&self) {
        debug!("→ {}", self.name);
    }

    pub fn exit(&self) {
        let elapsed = self.start.elapsed();
        debug!("← {} ({:?})", self.name, elapsed);
    }
}

impl Drop for Span {
    fn drop(&mut self) {
        self.exit();
    }
}

#[macro_export]
macro_rules! span {
    ($name:expr) => {
        $crate::Span::new($name)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestLogger {
        logs: Mutex<Vec<(Level, String)>>,
    }

    impl TestLogger {
        fn new() -> Self {
            Self {
                logs: Mutex::new(Vec::new()),
            }
        }

        fn get_logs(&self) -> Vec<(Level, String)> {
            self.logs.lock().unwrap().clone()
        }
    }

    impl Logger for TestLogger {
        fn log(&self, level: Level, message: &str) {
            self.logs.lock().unwrap().push((level, message.to_string()));
        }
    }

    #[test]
    fn test_console_logger() {
        let logger = ConsoleLogger::new(Level::Info);
        logger.log(Level::Info, "test message");
        logger.log(Level::Debug, "should not appear");
    }

    #[test]
    fn test_levels() {
        assert!(Level::Error > Level::Warn);
        assert!(Level::Info > Level::Debug);
        assert_eq!(Level::Info.as_str(), "INFO");
    }

    #[test]
    fn test_span() {
        let _span = Span::new("test_operation");
        // Span should log on creation and drop
    }
}
