//! Scoped thread spawning

use std::marker::PhantomData;
use std::thread;

/// A scope for spawning threads
pub struct Scope<'env> {
    _marker: PhantomData<&'env ()>,
}

impl<'env> Scope<'env> {
    /// Spawn a scoped thread
    pub fn spawn<F, T>(&self, f: F) -> ScopedJoinHandle<'env, T>
    where
        F: FnOnce(&Scope<'env>) -> T + Send + 'env,
        T: Send + 'env,
    {
        // Note: This is a simplified implementation
        // A full implementation would use thread::scope from std
        let handle = thread::spawn(move || {
            let scope = Scope {
                _marker: PhantomData,
            };
            f(&scope)
        });

        ScopedJoinHandle {
            handle: Some(handle),
            _marker: PhantomData,
        }
    }
}

/// A handle to a scoped thread
pub struct ScopedJoinHandle<'env, T> {
    handle: Option<thread::JoinHandle<T>>,
    _marker: PhantomData<&'env ()>,
}

impl<'env, T> ScopedJoinHandle<'env, T> {
    /// Wait for the thread to finish
    pub fn join(mut self) -> thread::Result<T> {
        self.handle.take().unwrap().join()
    }
}

/// Create a scope for spawning threads
pub fn scope<'env, F, R>(f: F) -> R
where
    F: FnOnce(&Scope<'env>) -> R,
{
    let scope = Scope {
        _marker: PhantomData,
    };
    f(&scope)
}
