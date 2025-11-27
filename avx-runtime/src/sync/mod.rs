//! # Sync - Primitivas de sincronização
//!
//! Implementações nativas sem dependências externas.

pub mod mutex;
pub mod rwlock;
pub mod semaphore;
pub mod condvar;

pub use mutex::{Mutex, MutexGuard};
pub use rwlock::{RwLock, RwLockReadGuard, RwLockWriteGuard};
pub use semaphore::{Semaphore, SemaphoreGuard};
pub use condvar::Condvar;
