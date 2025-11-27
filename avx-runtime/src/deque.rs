//! # Chase-Lev Work-Stealing Deque
//!
//! Implementação nativa do algoritmo Chase-Lev para work-stealing.
//! Baseado no paper "Dynamic Circular Work-Stealing Deque" (2005).
//!
//! Características:
//! - Lock-free para o owner thread
//! - O(1) para push/pop local
//! - O(log n) para steal de outras threads
//! - Usa array circular dinâmico

use std::sync::atomic::{AtomicIsize, AtomicPtr, Ordering};
use std::ptr;
use std::alloc::{alloc, dealloc, Layout};

const INITIAL_CAPACITY: usize = 256;

/// Chase-Lev Deque - Owner pode push/pop do topo, stealers roubam da base
pub struct WorkStealingDeque<T> {
    top: AtomicIsize,
    bottom: AtomicIsize,
    array: AtomicPtr<CircularArray<T>>,
}

struct CircularArray<T> {
    capacity: usize,
    mask: usize,
    buffer: *mut T,
}

impl<T> CircularArray<T> {
    fn new(capacity: usize) -> *mut Self {
        assert!(capacity.is_power_of_two());

        unsafe {
            let layout = Layout::new::<CircularArray<T>>();
            let ptr = alloc(layout) as *mut CircularArray<T>;

            let buffer_layout = Layout::array::<T>(capacity).unwrap();
            let buffer = alloc(buffer_layout) as *mut T;

            ptr::write(ptr, CircularArray {
                capacity,
                mask: capacity - 1,
                buffer,
            });

            ptr
        }
    }

    unsafe fn get(&self, index: isize) -> *mut T {
        self.buffer.add((index as usize) & self.mask)
    }

    unsafe fn put(&self, index: isize, value: T) {
        let ptr = self.get(index);
        ptr::write(ptr, value);
    }

    unsafe fn grow(old: *mut Self, top: isize, bottom: isize) -> *mut Self {
        let old_capacity = (*old).capacity;
        let new_capacity = old_capacity * 2;
        let new_array = Self::new(new_capacity);

        // Copy elements
        for i in top..bottom {
            let ptr = (*old).get(i);
            let value = ptr::read(ptr);
            (*new_array).put(i, value);
        }

        new_array
    }
}

impl<T> Drop for CircularArray<T> {
    fn drop(&mut self) {
        unsafe {
            let buffer_layout = Layout::array::<T>(self.capacity).unwrap();
            dealloc(self.buffer as *mut u8, buffer_layout);
        }
    }
}

impl<T> WorkStealingDeque<T> {
    /// Cria novo deque vazio com capacidade inicial.
    pub fn new() -> Self {
        Self {
            top: AtomicIsize::new(0),
            bottom: AtomicIsize::new(0),
            array: AtomicPtr::new(CircularArray::new(INITIAL_CAPACITY)),
        }
    }

    /// Push no topo (somente owner thread).
    ///
    /// O(1) - sem contenção.
    pub fn push(&self, value: T) {
        let bottom = self.bottom.load(Ordering::Relaxed);
        let top = self.top.load(Ordering::Acquire);
        let array = self.array.load(Ordering::Relaxed);

        let size = bottom - top;

        unsafe {
            // Grow if needed
            if size >= (*array).capacity as isize {
                let new_array = CircularArray::grow(array, top, bottom);
                self.array.store(new_array, Ordering::Release);
                // Old array will be dropped eventually (epoch-based reclamation would go here)
            }

            let array = self.array.load(Ordering::Relaxed);
            (*array).put(bottom, value);
        }

        // Memory fence
        std::sync::atomic::fence(Ordering::Release);
        self.bottom.store(bottom + 1, Ordering::Relaxed);
    }

    /// Pop do topo (owner thread only)
    pub fn pop(&self) -> Option<T> {
        let bottom = self.bottom.load(Ordering::Relaxed) - 1;
        let array = self.array.load(Ordering::Relaxed);
        self.bottom.store(bottom, Ordering::Relaxed);

        std::sync::atomic::fence(Ordering::SeqCst);

        let top = self.top.load(Ordering::Relaxed);

        if bottom < top {
            // Empty
            self.bottom.store(top, Ordering::Relaxed);
            return None;
        }

        unsafe {
            let value = ptr::read((*array).get(bottom));

            if bottom > top {
                // More than one element
                return Some(value);
            }

            // Last element - race with stealers
            if self.top.compare_exchange(
                top,
                top + 1,
                Ordering::SeqCst,
                Ordering::Relaxed,
            ).is_err() {
                // Lost race
                std::mem::forget(value);
                self.bottom.store(top + 1, Ordering::Relaxed);
                return None;
            }

            self.bottom.store(top + 1, Ordering::Relaxed);
            Some(value)
        }
    }

    /// Steal da base (other threads)
    pub fn steal(&self) -> Steal<T> {
        let top = self.top.load(Ordering::Acquire);
        std::sync::atomic::fence(Ordering::SeqCst);
        let bottom = self.bottom.load(Ordering::Acquire);

        if top >= bottom {
            return Steal::Empty;
        }

        let array = self.array.load(Ordering::Acquire);

        unsafe {
            let value = ptr::read((*array).get(top));

            if self.top.compare_exchange(
                top,
                top + 1,
                Ordering::SeqCst,
                Ordering::Relaxed,
            ).is_err() {
                // Lost race
                std::mem::forget(value);
                return Steal::Retry;
            }

            Steal::Success(value)
        }
    }

    /// Retorna se está vazio (aproximação)
    pub fn is_empty(&self) -> bool {
        let bottom = self.bottom.load(Ordering::Relaxed);
        let top = self.top.load(Ordering::Relaxed);
        bottom <= top
    }
}

/// Resultado de steal operation.
pub enum Steal<T> {
    /// Deque está vazio
    Empty,
    /// Sucesso - valor roubado
    Success(T),
    /// Conflito - tentar novamente
    Retry,
}

unsafe impl<T: Send> Send for WorkStealingDeque<T> {}
unsafe impl<T: Send> Sync for WorkStealingDeque<T> {}

impl<T> Default for WorkStealingDeque<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for WorkStealingDeque<T> {
    fn drop(&mut self) {
        // Pop all remaining elements
        while self.pop().is_some() {}

        // Free array
        unsafe {
            let array = self.array.load(Ordering::Relaxed);
            if !array.is_null() {
                let layout = Layout::new::<CircularArray<T>>();
                dealloc(array as *mut u8, layout);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_pop() {
        let deque = WorkStealingDeque::new();
        deque.push(1);
        deque.push(2);
        deque.push(3);

        assert_eq!(deque.pop(), Some(3));
        assert_eq!(deque.pop(), Some(2));
        assert_eq!(deque.pop(), Some(1));
        assert_eq!(deque.pop(), None);
    }

    #[test]
    fn test_steal() {
        let deque = WorkStealingDeque::new();
        deque.push(1);
        deque.push(2);
        deque.push(3);

        match deque.steal() {
            Steal::Success(v) => assert_eq!(v, 1),
            _ => panic!("Expected success"),
        }

        assert_eq!(deque.pop(), Some(3));
        assert_eq!(deque.pop(), Some(2));
    }
}
