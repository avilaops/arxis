//! # Condvar - Condition Variable nativa
//!
//! Implementação usando atomics e futex
//! - Wait/Notify pattern
//! - Funciona com Mutex

use std::sync::atomic::{AtomicU32, Ordering};

/// Condition Variable
pub struct Condvar {
    futex: AtomicU32,
}

impl Condvar {
    /// Cria nova condition variable.
    pub fn new() -> Self {
        Self {
            futex: AtomicU32::new(0),
        }
    }

    /// Wait on condition, releasing mutex atomically
    pub fn wait<'a, T>(&self, guard: crate::sync::mutex::MutexGuard<'a, T>) -> crate::sync::mutex::MutexGuard<'a, T> {
        let mutex = guard.mutex();
        let _seq = self.futex.load(Ordering::Relaxed);

        // Release mutex
        drop(guard);

        // Wait on futex
        #[cfg(target_os = "linux")]
        {
            const SYS_FUTEX: i64 = 202;
            const FUTEX_WAIT: i32 = 0;
            const FUTEX_PRIVATE_FLAG: i32 = 128;

            unsafe {
                let ret: i64;
                std::arch::asm!(
                    "syscall",
                    inlateout("rax") SYS_FUTEX => ret,
                    in("rdi") &self.futex as *const AtomicU32,
                    in("rsi") FUTEX_WAIT | FUTEX_PRIVATE_FLAG,
                    in("rdx") seq,
                    in("r10") 0 as *const u8,
                    lateout("rcx") _,
                    lateout("r11") _,
                );
            }
        }

        #[cfg(not(target_os = "linux"))]
        std::thread::yield_now();

        // Reacquire mutex
        mutex.lock()
    }

    /// Notify one waiting thread
    pub fn notify_one(&self) {
        self.futex.fetch_add(1, Ordering::Release);

        #[cfg(target_os = "linux")]
        {
            const SYS_FUTEX: i64 = 202;
            const FUTEX_WAKE: i32 = 1;
            const FUTEX_PRIVATE_FLAG: i32 = 128;

            unsafe {
                let ret: i64;
                std::arch::asm!(
                    "syscall",
                    inlateout("rax") SYS_FUTEX => ret,
                    in("rdi") &self.futex as *const AtomicU32,
                    in("rsi") FUTEX_WAKE | FUTEX_PRIVATE_FLAG,
                    in("rdx") 1,
                    lateout("rcx") _,
                    lateout("r11") _,
                );
            }
        }
    }

    /// Notify all waiting threads
    pub fn notify_all(&self) {
        self.futex.fetch_add(1, Ordering::Release);

        #[cfg(target_os = "linux")]
        {
            const SYS_FUTEX: i64 = 202;
            const FUTEX_WAKE: i32 = 1;
            const FUTEX_PRIVATE_FLAG: i32 = 128;

            unsafe {
                let ret: i64;
                std::arch::asm!(
                    "syscall",
                    inlateout("rax") SYS_FUTEX => ret,
                    in("rdi") &self.futex as *const AtomicU32,
                    in("rsi") FUTEX_WAKE | FUTEX_PRIVATE_FLAG,
                    in("rdx") i32::MAX,
                    lateout("rcx") _,
                    lateout("r11") _,
                );
            }
        }
    }
}

impl Default for Condvar {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for Condvar {}
unsafe impl Sync for Condvar {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_notify() {
        let mutex = Arc::new(Mutex::new(false));
        let condvar = Arc::new(Condvar::new());

        let m2 = mutex.clone();
        let c2 = condvar.clone();

        let handle = thread::spawn(move || {
            let mut guard = m2.lock();
            while !*guard {
                guard = c2.wait(guard);
            }
        });

        {
            let mut guard = mutex.lock();
            *guard = true;
            condvar.notify_one();
        }

        handle.join().unwrap();
    }
}
