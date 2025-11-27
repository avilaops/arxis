//! # Semaphore - Semáforo nativo
//!
//! Implementação usando atomics e futex
//! - Contador de recursos
//! - Acquire/Release operations

use std::sync::atomic::{AtomicUsize, Ordering};

/// Semáforo para contagem de recursos.
///
/// Implementa padrão acquire/release para controle de recursos limitados.
///
/// # Exemplo
///
/// ```no_run
/// use avx_runtime::sync::Semaphore;
///
/// let sem = Semaphore::new(3); // 3 recursos disponíveis
/// sem.acquire(); // Pega 1 recurso
/// sem.release(); // Devolve recurso
/// ```
pub struct Semaphore {
    count: AtomicUsize,
}

impl Semaphore {
    /// Cria novo semáforo com contagem inicial.
    ///
    /// # Argumentos
    ///
    /// * `initial` - Número inicial de recursos disponíveis
    pub fn new(initial: usize) -> Self {
        Self {
            count: AtomicUsize::new(initial),
        }
    }

    /// Adquire um permit, bloqueando se necessário.
    pub fn acquire(&self) {
        loop {
            let count = self.count.load(Ordering::Relaxed);

            if count == 0 {
                self.wait();
                continue;
            }

            if self.count.compare_exchange_weak(
                count,
                count - 1,
                Ordering::Acquire,
                Ordering::Relaxed,
            ).is_ok() {
                return;
            }
        }
    }

    /// Tenta adquirir permit sem bloquear.
    ///
    /// # Retorna
    ///
    /// `true` se adquiriu permit, `false` se nenhum disponível.
    pub fn try_acquire(&self) -> bool {
        let mut count = self.count.load(Ordering::Relaxed);

        loop {
            if count == 0 {
                return false;
            }

            match self.count.compare_exchange_weak(
                count,
                count - 1,
                Ordering::Acquire,
                Ordering::Relaxed,
            ) {
                Ok(_) => return true,
                Err(c) => count = c,
            }
        }
    }

    /// Release one permit
    pub fn release(&self) {
        self.count.fetch_add(1, Ordering::Release);

        // Acorda uma thread esperando
        #[cfg(target_os = "linux")]
        self.futex_wake(1);
    }

    /// Release multiple permits
    pub fn release_many(&self, n: usize) {
        self.count.fetch_add(n, Ordering::Release);

        // Acorda N threads esperando
        #[cfg(target_os = "linux")]
        self.futex_wake(n as i32);
    }

    fn wait(&self) {
        // Spin por algumas iterações
        for _ in 0..100 {
            std::hint::spin_loop();
            if self.count.load(Ordering::Relaxed) > 0 {
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
                in("rdi") &self.count as *const AtomicUsize,
                in("rsi") FUTEX_WAIT | FUTEX_PRIVATE_FLAG,
                in("rdx") 0,
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
                in("rdi") &self.count as *const AtomicUsize,
                in("rsi") FUTEX_WAKE | FUTEX_PRIVATE_FLAG,
                in("rdx") n,
                lateout("rcx") _,
                lateout("r11") _,
            );
        }
    }
}

/// RAII guard for semaphore
pub struct SemaphoreGuard<'a> {
    sem: &'a Semaphore,
}

impl<'a> Drop for SemaphoreGuard<'a> {
    fn drop(&mut self) {
        self.sem.release();
    }
}

unsafe impl Send for Semaphore {}
unsafe impl Sync for Semaphore {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_acquire_release() {
        let sem = Semaphore::new(2);

        sem.acquire();
        sem.acquire();

        assert!(!sem.try_acquire());

        sem.release();
        assert!(sem.try_acquire());
    }
}
