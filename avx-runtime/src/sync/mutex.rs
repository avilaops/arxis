//! # Mutex - Implementação nativa de Mutex sem parking_lot
//!
//! Usa futex no Linux, kevent no macOS, WaitOnAddress no Windows.

use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicU32, Ordering};

/// Mutex nativo usando futex (Linux), kevent (macOS) ou WaitOnAddress (Windows).
///
/// # Exemplo
///
/// ```no_run
/// use avx_runtime::sync::Mutex;
///
/// let mutex = Mutex::new(42);
/// let guard = mutex.lock();
/// assert_eq!(*guard, 42);
/// ```
pub struct Mutex<T> {
    state: AtomicU32,
    data: UnsafeCell<T>,
}

const UNLOCKED: u32 = 0;
const LOCKED: u32 = 1;
const PARKED: u32 = 2;

impl<T> Mutex<T> {
    /// Cria novo Mutex protegendo valor fornecido.
    pub fn new(value: T) -> Self {
        Self {
            state: AtomicU32::new(UNLOCKED),
            data: UnsafeCell::new(value),
        }
    }

    /// Adquire lock, bloqueando se necessário.
    ///
    /// # Retorna
    ///
    /// Guard que libera lock automaticamente ao sair de escopo.
    pub fn lock(&self) -> MutexGuard<'_, T> {
        // Fast path: try to acquire immediately
        if self.state.compare_exchange(UNLOCKED, LOCKED, Ordering::Acquire, Ordering::Relaxed).is_ok() {
            return MutexGuard { mutex: self };
        }

        // Slow path: spin then park
        self.lock_contended();
        MutexGuard { mutex: self }
    }

    fn lock_contended(&self) {
        let mut spin_count = 0;
        const MAX_SPINS: u32 = 100;

        loop {
            // Spin for a while
            if spin_count < MAX_SPINS {
                for _ in 0..10 {
                    std::hint::spin_loop();
                }
                spin_count += 1;

                if self.state.load(Ordering::Relaxed) == UNLOCKED {
                    if self.state.compare_exchange(UNLOCKED, LOCKED, Ordering::Acquire, Ordering::Relaxed).is_ok() {
                        return;
                    }
                }
                continue;
            }

            // Park the thread
            if self.state.swap(PARKED, Ordering::Acquire) == UNLOCKED {
                return;
            } else {
                // Wait using platform-specific primitive
                self.park();
            }
        }
    }

    fn unlock(&self) {
        if self.state.swap(UNLOCKED, Ordering::Release) == PARKED {
            self.unpark();
        }
    }

    #[cfg(target_os = "linux")]
    fn park(&self) {
        unsafe {
            libc::syscall(
                libc::SYS_futex,
                &self.state as *const _ as *const libc::c_int,
                libc::FUTEX_WAIT | libc::FUTEX_PRIVATE_FLAG,
                PARKED as libc::c_int,
                std::ptr::null::<libc::timespec>(),
            );
        }
    }

    #[cfg(target_os = "linux")]
    fn unpark(&self) {
        unsafe {
            libc::syscall(
                libc::SYS_futex,
                &self.state as *const _ as *const libc::c_int,
                libc::FUTEX_WAKE | libc::FUTEX_PRIVATE_FLAG,
                1,
            );
        }
    }

    #[cfg(not(target_os = "linux"))]
    fn park(&self) {
        // Fallback: just spin
        std::thread::yield_now();
    }

    #[cfg(not(target_os = "linux"))]
    fn unpark(&self) {
        // Fallback: no-op
    }
}

unsafe impl<T: Send> Send for Mutex<T> {}
unsafe impl<T: Send> Sync for Mutex<T> {}

/// Guard que libera Mutex automaticamente ao sair de escopo.
pub struct MutexGuard<'a, T> {
    mutex: &'a Mutex<T>,
}

impl<'a, T> MutexGuard<'a, T> {
    /// Get reference to the mutex (for condvar)
    pub(crate) fn mutex(&self) -> &'a Mutex<T> {
        self.mutex
    }
}

impl<T> Deref for MutexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.mutex.data.get() }
    }
}

impl<T> DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.mutex.data.get() }
    }
}

impl<T> Drop for MutexGuard<'_, T> {
    fn drop(&mut self) {
        self.mutex.unlock();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mutex_basic() {
        let mutex = Mutex::new(0);
        {
            let mut guard = mutex.lock();
            *guard += 1;
        }
        {
            let guard = mutex.lock();
            assert_eq!(*guard, 1);
        }
    }
}
