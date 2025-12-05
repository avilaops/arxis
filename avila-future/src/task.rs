//! Task context and pinning support

use crate::waker::Waker;

/// Re-export Pin from core
pub use core::pin::Pin;

/// Re-export Unpin from core
pub use core::marker::Unpin;

/// The context of an asynchronous task
///
/// A `Context` contains a `Waker` that can be used to wake up the task
/// when it should be polled again.
pub struct Context<'a> {
    waker: &'a Waker,
}

impl<'a> Context<'a> {
    /// Creates a new `Context` from a `Waker`
    pub fn from_waker(waker: &'a Waker) -> Self {
        Context { waker }
    }

    /// Returns a reference to the `Waker` for the current task
    pub fn waker(&self) -> &'a Waker {
        self.waker
    }
}
