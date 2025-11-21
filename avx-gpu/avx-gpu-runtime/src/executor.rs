//! Kernel executor

/// Execution mode
#[derive(Debug, Clone, Copy)]
pub enum ExecutionMode {
    Sync,
    Async,
}

/// Kernel executor
pub struct Executor {
    mode: ExecutionMode,
}

impl Executor {
    pub fn new(mode: ExecutionMode) -> Self {
        Self { mode }
    }
}

impl Default for Executor {
    fn default() -> Self {
        Self::new(ExecutionMode::Sync)
    }
}
