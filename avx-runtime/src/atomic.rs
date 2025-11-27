//! # Atomic Operations - Operações atômicas nativas em Rust puro
//!
//! Implementa primitivas atômicas sem dependências externas.

use std::sync::atomic::{AtomicBool, AtomicUsize, AtomicPtr, Ordering};
use std::ptr;

/// Ponteiro atômico para valor heap-allocated.
///
/// Gerencia memória automaticamente ao atualizar.
pub struct AtomicBox<T> {
    ptr: AtomicPtr<T>,
}

impl<T> AtomicBox<T> {
    /// Cria novo AtomicBox com valor inicial.
    pub fn new(value: T) -> Self {
        let boxed = Box::new(value);
        Self {
            ptr: AtomicPtr::new(Box::into_raw(boxed)),
        }
    }

    /// Cria AtomicBox nulo.
    pub fn null() -> Self {
        Self {
            ptr: AtomicPtr::new(ptr::null_mut()),
        }
    }

    /// Carrega referência ao valor, se existir.
    pub fn load(&self, order: Ordering) -> Option<&T> {
        let ptr = self.ptr.load(order);
        if ptr.is_null() {
            None
        } else {
            unsafe { Some(&*ptr) }
        }
    }

    /// Armazena novo valor, dropando anterior.
    pub fn store(&self, value: T, order: Ordering) {
        let new_ptr = Box::into_raw(Box::new(value));
        let old_ptr = self.ptr.swap(new_ptr, order);
        if !old_ptr.is_null() {
            unsafe { drop(Box::from_raw(old_ptr)); }
        }
    }

    /// Troca valor, retornando anterior.
    pub fn swap(&self, value: T, order: Ordering) -> Option<Box<T>> {
        let new_ptr = Box::into_raw(Box::new(value));
        let old_ptr = self.ptr.swap(new_ptr, order);
        if old_ptr.is_null() {
            None
        } else {
            unsafe { Some(Box::from_raw(old_ptr)) }
        }
    }

    /// Compare-and-swap condicional.
    pub fn compare_exchange(
        &self,
        current: *mut T,
        new: T,
        success: Ordering,
        failure: Ordering,
    ) -> Result<*mut T, *mut T> {
        let new_ptr = Box::into_raw(Box::new(new));
        match self.ptr.compare_exchange(current, new_ptr, success, failure) {
            Ok(ptr) => Ok(ptr),
            Err(ptr) => {
                // Falhou, libera new_ptr
                unsafe { drop(Box::from_raw(new_ptr)); }
                Err(ptr)
            }
        }
    }
}

impl<T> Drop for AtomicBox<T> {
    fn drop(&mut self) {
        let ptr = self.ptr.load(Ordering::Acquire);
        if !ptr.is_null() {
            unsafe { drop(Box::from_raw(ptr)); }
        }
    }
}

unsafe impl<T: Send> Send for AtomicBox<T> {}
unsafe impl<T: Send> Sync for AtomicBox<T> {}

/// Contador atômico thread-safe.
pub struct AtomicCounter {
    count: AtomicUsize,
}

impl AtomicCounter {
    /// Cria contador com valor inicial.
    pub fn new(initial: usize) -> Self {
        Self {
            count: AtomicUsize::new(initial),
        }
    }

    /// Incrementa contador, retornando valor anterior.
    pub fn increment(&self) -> usize {
        self.count.fetch_add(1, Ordering::SeqCst)
    }

    /// Decrementa contador, retornando valor anterior.
    pub fn decrement(&self) -> usize {
        self.count.fetch_sub(1, Ordering::SeqCst)
    }

    /// Carrega valor atual.
    pub fn load(&self) -> usize {
        self.count.load(Ordering::SeqCst)
    }

    /// Armazena novo valor.
    pub fn store(&self, value: usize) {
        self.count.store(value, Ordering::SeqCst);
    }
}

/// Flag booleana atômica.
pub struct AtomicFlag {
    flag: AtomicBool,
}

impl AtomicFlag {
    /// Cria flag com valor inicial.
    pub fn new(initial: bool) -> Self {
        Self {
            flag: AtomicBool::new(initial),
        }
    }

    /// Marca flag como ativa.
    pub fn set(&self) {
        self.flag.store(true, Ordering::SeqCst);
    }

    /// Marca flag como inativa.
    pub fn clear(&self) {
        self.flag.store(false, Ordering::SeqCst);
    }

    /// Verifica se flag está ativa.
    pub fn is_set(&self) -> bool {
        self.flag.load(Ordering::SeqCst)
    }

    /// Testa e marca flag, retornando valor anterior.
    pub fn test_and_set(&self) -> bool {
        self.flag.swap(true, Ordering::SeqCst)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atomic_counter() {
        let counter = AtomicCounter::new(0);
        assert_eq!(counter.increment(), 0);
        assert_eq!(counter.increment(), 1);
        assert_eq!(counter.load(), 2);
        assert_eq!(counter.decrement(), 2);
        assert_eq!(counter.load(), 1);
    }

    #[test]
    fn test_atomic_flag() {
        let flag = AtomicFlag::new(false);
        assert!(!flag.is_set());
        flag.set();
        assert!(flag.is_set());
        flag.clear();
        assert!(!flag.is_set());
    }
}
