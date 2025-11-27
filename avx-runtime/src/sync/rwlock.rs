//! # RwLock - Reader-Writer Lock nativo
//!
//! Implementação usando atomics e futex (Linux) / park/unpark (outros)
//! - Múltiplos readers simultâneos
//! - Writer exclusivo

use std::sync::atomic::{AtomicU32, Ordering};
use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};

const UNLOCKED: u32 = 0;
const WRITER: u32 = 1 << 31;
const READER: u32 = 1;
const MAX_READERS: u32 = WRITER - 1;

/// Reader-Writer Lock nativo.
///
/// Permite múltiplos leitores simultâneos OU um escritor exclusivo.
///
/// # Exemplo
///
/// ```no_run
/// use avx_runtime::sync::RwLock;
///
/// let lock = RwLock::new(vec![1, 2, 3]);
///
/// // Múltiplos readers
/// let r1 = lock.read();
/// let r2 = lock.read();
///
/// // Writer exclusivo
/// let mut w = lock.write();
/// w.push(4);
/// ```
pub struct RwLock<T> {
    state: AtomicU32,
    data: UnsafeCell<T>,
}

impl<T> RwLock<T> {
    /// Cria novo RwLock protegendo valor fornecido.
    pub fn new(value: T) -> Self {
        Self {
            state: AtomicU32::new(UNLOCKED),
            data: UnsafeCell::new(value),
        }
    }

    /// Adquire lock de leitura (compartilhado).
    ///
    /// Bloqueia se houver escritor ativo.
    pub fn read(&self) -> RwLockReadGuard<'_, T> {
        loop {
            let state = self.state.load(Ordering::Relaxed);

            // Se há writer, espera
            if state & WRITER != 0 {
                self.wait_for_readers();
                continue;
            }

            // Tenta incrementar readers
            if state >= MAX_READERS {
                panic!("Too many readers");
            }

            if self.state.compare_exchange_weak(
                state,
                state + READER,
                Ordering::Acquire,
                Ordering::Relaxed,
            ).is_ok() {
                return RwLockReadGuard { lock: self };
            }
        }
    }

    /// Adquire lock de escrita (exclusivo).
    ///
    /// Bloqueia se houver leitores ou escritor ativo.
    pub fn write(&self) -> RwLockWriteGuard<'_, T> {
        loop {
            let state = self.state.load(Ordering::Relaxed);

            // Se há readers ou writer, espera
            if state != UNLOCKED {
                self.wait_for_writers();
                continue;
            }

            // Tenta marcar como writer
            if self.state.compare_exchange_weak(
                UNLOCKED,
                WRITER,
                Ordering::Acquire,
                Ordering::Relaxed,
            ).is_ok() {
                return RwLockWriteGuard { lock: self };
            }
        }
    }

    fn wait_for_readers(&self) {
        // Spin por algumas iterações
        for _ in 0..100 {
            std::hint::spin_loop();
            if self.state.load(Ordering::Relaxed) & WRITER == 0 {
                return;
            }
        }

        // Park thread
        #[cfg(target_os = "linux")]
        self.futex_wait();

        #[cfg(not(target_os = "linux"))]
        std::thread::yield_now();
    }

    fn wait_for_writers(&self) {
        // Spin por algumas iterações
        for _ in 0..100 {
            std::hint::spin_loop();
            if self.state.load(Ordering::Relaxed) == UNLOCKED {
                return;
            }
        }

        // Park thread
        #[cfg(target_os = "linux")]
        self.futex_wait();

        #[cfg(not(target_os = "linux"))]
        std::thread::yield_now();
    }

    #[cfg(target_os = "linux")]
    fn futex_wait(&self) {
        const SYS_FUTEX: i64 = 202;
        const FUTEX_WAIT: i32 = 0;
        const FUTEX_PRIVATE_FLAG: i32 = 128;

        unsafe {
            let ret: i64;
            std::arch::asm!(
                "syscall",
                inlateout("rax") SYS_FUTEX => ret,
                in("rdi") &self.state as *const AtomicU32,
                in("rsi") FUTEX_WAIT | FUTEX_PRIVATE_FLAG,
                in("rdx") self.state.load(Ordering::Relaxed),
                in("r10") 0 as *const u8,
                lateout("rcx") _,
                lateout("r11") _,
            );
        }
    }

    #[cfg(target_os = "linux")]
    fn futex_wake(&self, n: i32) {
        const SYS_FUTEX: i64 = 202;
        const FUTEX_WAKE: i32 = 1;
        const FUTEX_PRIVATE_FLAG: i32 = 128;

        unsafe {
            let ret: i64;
            std::arch::asm!(
                "syscall",
                inlateout("rax") SYS_FUTEX => ret,
                in("rdi") &self.state as *const AtomicU32,
                in("rsi") FUTEX_WAKE | FUTEX_PRIVATE_FLAG,
                in("rdx") n,
                lateout("rcx") _,
                lateout("r11") _,
            );
        }
    }
}

/// Guard de leitura que libera lock automaticamente ao sair de escopo.
pub struct RwLockReadGuard<'a, T> {
    lock: &'a RwLock<T>,
}

impl<'a, T> Drop for RwLockReadGuard<'a, T> {
    fn drop(&mut self) {
        let old = self.lock.state.fetch_sub(READER, Ordering::Release);

        // Se era o último reader, acorda writers
        if old == READER {
            #[cfg(target_os = "linux")]
            self.lock.futex_wake(1);
        }
    }
}

impl<'a, T> Deref for RwLockReadGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.lock.data.get() }
    }
}

/// Guard de escrita para RwLock.
pub struct RwLockWriteGuard<'a, T> {
    lock: &'a RwLock<T>,
}

impl<'a, T> Drop for RwLockWriteGuard<'a, T> {
    fn drop(&mut self) {
        self.lock.state.store(UNLOCKED, Ordering::Release);

        // Acorda todos esperando
        #[cfg(target_os = "linux")]
        self.lock.futex_wake(i32::MAX);
    }
}

impl<'a, T> Deref for RwLockWriteGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.lock.data.get() }
    }
}

impl<'a, T> DerefMut for RwLockWriteGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.lock.data.get() }
    }
}

unsafe impl<T: Send> Send for RwLock<T> {}
unsafe impl<T: Send + Sync> Sync for RwLock<T> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiple_readers() {
        let lock = RwLock::new(42);

        let r1 = lock.read();
        let r2 = lock.read();

        assert_eq!(*r1, 42);
        assert_eq!(*r2, 42);
    }

    #[test]
    fn test_writer() {
        let lock = RwLock::new(0);

        {
            let mut w = lock.write();
            *w = 100;
        }

        let r = lock.read();
        assert_eq!(*r, 100);
    }
}
