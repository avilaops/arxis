//! Kernel scheduler

/// Kernel scheduling strategy
#[derive(Debug, Clone, Copy)]
pub enum SchedulingStrategy {
    Fifo,
    Priority,
    RoundRobin,
}

/// Kernel scheduler
pub struct Scheduler {
    strategy: SchedulingStrategy,
}

impl Scheduler {
    pub fn new(strategy: SchedulingStrategy) -> Self {
        Self { strategy }
    }
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new(SchedulingStrategy::Fifo)
    }
}
