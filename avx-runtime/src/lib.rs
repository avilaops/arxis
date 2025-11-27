//! # AVX Runtime - Native Async Runtime
//!
//! Complete async runtime built from scratch to replace Tokio.
//! Implements work-stealing scheduler, event loop, and zero-overhead futures.
//!
//! ## Architecture
//!
//! ```text
//! ┌────────────────────────────────────────┐
//! │       AVX Runtime Architecture         │
//! ├────────────────────────────────────────┤
//! │                                        │
//! │  ┌──────────────────────────────────┐ │
//! │  │   Work-Stealing Scheduler        │ │
//! │  │  (Chase-Lev Deque + Threads)     │ │
//! │  └──────────────────────────────────┘ │
//! │              ↕                         │
//! │  ┌──────────────────────────────────┐ │
//! │  │   Event Loop (Reactor)           │ │
//! │  │  (Epoll/Kqueue/IOCP via mio)     │ │
//! │  └──────────────────────────────────┘ │
//! │              ↕                         │
//! │  ┌──────────────────────────────────┐ │
//! │  │   Waker + Future Executor        │ │
//! │  │  (Task queue + Poll mechanism)   │ │
//! │  └──────────────────────────────────┘ │
//! │              ↕                         │
//! │  ┌──────────────────────────────────┐ │
//! │  │   Lock-Free MPSC Queue           │ │
//! │  │  (Crossbeam channels)            │ │
//! │  └──────────────────────────────────┘ │
//! │              ↕                         │
//! │  ┌──────────────────────────────────┐ │
//! │  │   Timer Wheel                    │ │
//! │  │  (Hierarchical timeout system)   │ │
//! │  └──────────────────────────────────┘ │
//! │                                        │
//! └────────────────────────────────────────┘
//! ```
//!
//! ## Features
//!
//! - **Work-Stealing Scheduler**: Balanceia tarefas entre threads automaticamente
//! - **Event Loop**: Multiplexação de I/O via epoll/kqueue/IOCP
//! - **Zero-Overhead Futures**: Futures sem alocação heap
//! - **Lock-Free Queues**: Comunicação entre threads sem locks
//! - **Timer Wheel**: Sistema hierárquico de timeouts
//! - **Budget System**: Previne monopolização de CPU
//!
//! ## Example
//!
//! ```rust,no_run
//! use avx_runtime::Runtime;
//!
//! let runtime = Runtime::new().unwrap();
//!
//! runtime.block_on(async {
//!     println!("Hello from AVX Runtime!");
//! });
//! ```

#![warn(missing_docs)]
#![warn(rust_2018_idioms)]

pub mod runtime;
pub mod scheduler;
pub mod task;
pub mod reactor;
pub mod timer;
pub mod waker;
pub mod budget;
pub mod queue;

// Native implementations - 100% Rust, zero external dependencies!
pub mod atomic;
pub mod deque;
pub mod sync;
pub mod channel;

// Re-exports
pub use runtime::{Runtime, RuntimeConfig, Handle};
pub use task::{JoinHandle, Task};

/// Prelude com tipos mais comuns
pub mod prelude {
    pub use crate::runtime::{Runtime, RuntimeConfig, Handle};
    pub use crate::task::JoinHandle;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_creation() {
        let runtime = Runtime::new().unwrap();
        assert!(runtime.handle().is_some());
    }
}
