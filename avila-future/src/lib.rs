//! # Avila Future - Complete Async Runtime
//!
//! A complete async runtime without tokio, featuring:
//! - Custom Future trait
//! - Multi-threaded work-stealing executor
//! - Async/await syntax support
//! - Task scheduler
//! - Timers and intervals (with `timers` feature)
//! - Channels (mpsc, oneshot)
//! - `#![no_std]` support (without `std` feature)
//!
//! ## Example
//!
//! ```no_run
//! use avila_future::{Runtime, spawn};
//!
//! async fn hello() -> u32 {
//!     42
//! }
//!
//! let runtime = Runtime::new();
//! let result = runtime.block_on(async {
//!     let handle = spawn(hello());
//!     handle.await
//! });
//! ```

#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]
#![allow(clippy::all)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

// Core future and task primitives
pub mod future;
pub mod task;
pub mod waker;

// Executor and runtime
#[cfg(feature = "std")]
pub mod executor;
#[cfg(feature = "std")]
pub mod runtime;

// Channels for communication
pub mod channel;

// Timers (requires std)
#[cfg(feature = "timers")]
pub mod time;

// Re-exports
pub use future::{Future, Poll, Ready, ready, pending};
pub use task::{Context, Pin};
pub use waker::Waker;

#[cfg(feature = "std")]
pub use executor::Executor;

#[cfg(feature = "std")]
pub use runtime::{Runtime, spawn, block_on, JoinHandle};

#[cfg(feature = "std")]
pub use channel::{mpsc, oneshot};

#[cfg(feature = "timers")]
pub use time::{sleep, timeout, interval, Duration};

/// Prelude module for convenient imports
pub mod prelude {
    //! Common imports for async programming
    pub use crate::future::{Future, Poll, Ready, ready, pending};
    pub use crate::task::{Context, Pin};
    
    #[cfg(feature = "std")]
    pub use crate::runtime::{Runtime, spawn, block_on, JoinHandle};
    
    #[cfg(feature = "std")]
    pub use crate::channel::{mpsc, oneshot};
    
    #[cfg(feature = "timers")]
    pub use crate::time::{sleep, timeout, interval, Duration};
}
