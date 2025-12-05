//! # avila-atom
//!
//! **Atomic Operations and Lock-Free Primitives - Layer 1 Foundation**
//!
//! This library provides low-level atomic operations and primitives for building
//! concurrent systems without relying on `std::sync::atomic`. All operations are
//! implemented using inline assembly for maximum control and portability.
//!
//! ## Features
//!
//! - `AtomicByte` - 8-bit atomic integer
//! - `AtomicWord` - 16-bit atomic integer (u16)
//! - `AtomicQWord` - 64-bit atomic integer (u64)
//! - `AtomicBool` - Boolean atomic
//! - `AtomicPtr<T>` - Pointer atomic
//! - Memory ordering: Relaxed, Acquire, Release, SeqCst
//! - Compare-and-swap operations
//! - Basic spinlocks
//!
//! ## Rules
//!
//! - ❌ NO `std::sync::atomic`
//! - ✅ `#![no_std]` mandatory
//! - ✅ Inline assembly only
//!
//! ## Example
//!
//! ```rust
//! use avila_atom::{AtomicByte, Ordering};
//!
//! let atomic = AtomicByte::new(0);
//! atomic.store(42, Ordering::Relaxed);
//! let value = atomic.load(Ordering::Relaxed);
//! assert_eq!(value, 42);
//! ```

#![no_std]
#![feature(asm_const)]
#![warn(missing_docs)]
#![allow(clippy::missing_safety_doc)]

use core::arch::asm;

/// Memory ordering for atomic operations
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ordering {
    /// No ordering constraints, only atomicity is guaranteed
    Relaxed = 0,
    /// When loading, all subsequent reads and writes cannot be reordered before this load
    Acquire = 1,
    /// When storing, all previous reads and writes cannot be reordered after this store
    Release = 2,
    /// Full sequential consistency - strongest ordering
    SeqCst = 3,
}

/// 8-bit atomic integer
#[repr(C, align(1))]
pub struct AtomicByte {
    value: u8,
}

unsafe impl Sync for AtomicByte {}
unsafe impl Send for AtomicByte {}

impl AtomicByte {
    /// Creates a new atomic byte
    #[inline(always)]
    pub const fn new(value: u8) -> Self {
        Self { value }
    }

    /// Loads a value from the atomic byte
    #[inline(always)]
    pub fn load(&self, order: Ordering) -> u8 {
        match order {
            Ordering::Relaxed => unsafe {
                let result: u8;
                asm!(
                    "mov {}, byte ptr [{}]",
                    out(reg_byte) result,
                    in(reg) &self.value,
                    options(readonly, nostack, preserves_flags)
                );
                result
            },
            Ordering::Acquire | Ordering::SeqCst => unsafe {
                let result: u8;
                asm!(
                    "mov {}, byte ptr [{}]",
                    "lock or dword ptr [rsp], 0",  // Memory fence
                    out(reg_byte) result,
                    in(reg) &self.value,
                    options(readonly, nostack)
                );
                result
            },
            Ordering::Release => panic!("Invalid ordering for load"),
        }
    }

    /// Stores a value into the atomic byte
    #[inline(always)]
    pub fn store(&self, value: u8, order: Ordering) {
        match order {
            Ordering::Relaxed => unsafe {
                asm!(
                    "mov byte ptr [{}], {}",
                    in(reg) &self.value,
                    in(reg_byte) value,
                    options(nostack, preserves_flags)
                );
            },
            Ordering::Release | Ordering::SeqCst => unsafe {
                asm!(
                    "xchg byte ptr [{}], {}",
                    in(reg) &self.value,
                    in(reg_byte) value,
                    options(nostack)
                );
            },
            Ordering::Acquire => panic!("Invalid ordering for store"),
        }
    }

    /// Compare-and-swap operation
    #[inline(always)]
    pub fn compare_and_swap(&self, current: u8, new: u8, _order: Ordering) -> u8 {
        unsafe {
            let result: u8;
            asm!(
                "lock cmpxchg byte ptr [{}], {}",
                in(reg) &self.value,
                in(reg_byte) new,
                inout("al") current => result,
                options(nostack)
            );
            result
        }
    }

    /// Atomically adds to the current value and returns the previous value
    #[inline(always)]
    pub fn fetch_add(&self, value: u8, _order: Ordering) -> u8 {
        unsafe {
            let result: u8;
            asm!(
                "lock xadd byte ptr [{}], {}",
                in(reg) &self.value,
                inout(reg_byte) value => result,
                options(nostack)
            );
            result
        }
    }

    /// Atomically subtracts from the current value and returns the previous value
    #[inline(always)]
    pub fn fetch_sub(&self, value: u8, order: Ordering) -> u8 {
        self.fetch_add((value as i8).wrapping_neg() as u8, order)
    }

    /// Gets the inner value (not atomic)
    #[inline(always)]
    pub fn into_inner(self) -> u8 {
        self.value
    }

    /// Gets a mutable reference to the inner value
    #[inline(always)]
    pub fn get_mut(&mut self) -> &mut u8 {
        &mut self.value
    }
}

/// 16-bit atomic integer
#[repr(C, align(2))]
pub struct AtomicWord {
    value: u16,
}

unsafe impl Sync for AtomicWord {}
unsafe impl Send for AtomicWord {}

impl AtomicWord {
    /// Creates a new atomic word
    #[inline(always)]
    pub const fn new(value: u16) -> Self {
        Self { value }
    }

    /// Loads a value from the atomic word
    #[inline(always)]
    pub fn load(&self, order: Ordering) -> u16 {
        match order {
            Ordering::Relaxed => unsafe {
                let result: u16;
                asm!(
                    "mov {}, word ptr [{}]",
                    out(reg) result,
                    in(reg) &self.value,
                    options(readonly, nostack, preserves_flags)
                );
                result
            },
            Ordering::Acquire | Ordering::SeqCst => unsafe {
                let result: u16;
                asm!(
                    "mov {}, word ptr [{}]",
                    "lock or dword ptr [rsp], 0",
                    out(reg) result,
                    in(reg) &self.value,
                    options(readonly, nostack)
                );
                result
            },
            Ordering::Release => panic!("Invalid ordering for load"),
        }
    }

    /// Stores a value into the atomic word
    #[inline(always)]
    pub fn store(&self, value: u16, order: Ordering) {
        match order {
            Ordering::Relaxed => unsafe {
                asm!(
                    "mov word ptr [{}], {}",
                    in(reg) &self.value,
                    in(reg) value,
                    options(nostack, preserves_flags)
                );
            },
            Ordering::Release | Ordering::SeqCst => unsafe {
                asm!(
                    "xchg word ptr [{}], {}",
                    in(reg) &self.value,
                    in(reg) value,
                    options(nostack)
                );
            },
            Ordering::Acquire => panic!("Invalid ordering for store"),
        }
    }

    /// Compare-and-swap operation
    #[inline(always)]
    pub fn compare_and_swap(&self, current: u16, new: u16, _order: Ordering) -> u16 {
        unsafe {
            let result: u16;
            asm!(
                "lock cmpxchg word ptr [{}], {}",
                in(reg) &self.value,
                in(reg) new,
                inout("ax") current => result,
                options(nostack)
            );
            result
        }
    }

    /// Atomically adds to the current value and returns the previous value
    #[inline(always)]
    pub fn fetch_add(&self, value: u16, _order: Ordering) -> u16 {
        unsafe {
            let result: u16;
            asm!(
                "lock xadd word ptr [{}], {}",
                in(reg) &self.value,
                inout(reg) value => result,
                options(nostack)
            );
            result
        }
    }

    /// Atomically subtracts from the current value and returns the previous value
    #[inline(always)]
    pub fn fetch_sub(&self, value: u16, order: Ordering) -> u16 {
        self.fetch_add((value as i16).wrapping_neg() as u16, order)
    }

    /// Gets the inner value (not atomic)
    #[inline(always)]
    pub fn into_inner(self) -> u16 {
        self.value
    }

    /// Gets a mutable reference to the inner value
    #[inline(always)]
    pub fn get_mut(&mut self) -> &mut u16 {
        &mut self.value
    }
}

/// 64-bit atomic integer
#[repr(C, align(8))]
pub struct AtomicQWord {
    value: u64,
}

unsafe impl Sync for AtomicQWord {}
unsafe impl Send for AtomicQWord {}

impl AtomicQWord {
    /// Creates a new atomic qword
    #[inline(always)]
    pub const fn new(value: u64) -> Self {
        Self { value }
    }

    /// Loads a value from the atomic qword
    #[inline(always)]
    pub fn load(&self, order: Ordering) -> u64 {
        match order {
            Ordering::Relaxed => unsafe {
                let result: u64;
                asm!(
                    "mov {}, qword ptr [{}]",
                    out(reg) result,
                    in(reg) &self.value,
                    options(readonly, nostack, preserves_flags)
                );
                result
            },
            Ordering::Acquire | Ordering::SeqCst => unsafe {
                let result: u64;
                asm!(
                    "mov {}, qword ptr [{}]",
                    "lock or dword ptr [rsp], 0",
                    out(reg) result,
                    in(reg) &self.value,
                    options(readonly, nostack)
                );
                result
            },
            Ordering::Release => panic!("Invalid ordering for load"),
        }
    }

    /// Stores a value into the atomic qword
    #[inline(always)]
    pub fn store(&self, value: u64, order: Ordering) {
        match order {
            Ordering::Relaxed => unsafe {
                asm!(
                    "mov qword ptr [{}], {}",
                    in(reg) &self.value,
                    in(reg) value,
                    options(nostack, preserves_flags)
                );
            },
            Ordering::Release | Ordering::SeqCst => unsafe {
                asm!(
                    "xchg qword ptr [{}], {}",
                    in(reg) &self.value,
                    in(reg) value,
                    options(nostack)
                );
            },
            Ordering::Acquire => panic!("Invalid ordering for store"),
        }
    }

    /// Compare-and-swap operation
    #[inline(always)]
    pub fn compare_and_swap(&self, current: u64, new: u64, _order: Ordering) -> u64 {
        unsafe {
            let result: u64;
            asm!(
                "lock cmpxchg qword ptr [{}], {}",
                in(reg) &self.value,
                in(reg) new,
                inout("rax") current => result,
                options(nostack)
            );
            result
        }
    }

    /// Atomically adds to the current value and returns the previous value
    #[inline(always)]
    pub fn fetch_add(&self, value: u64, _order: Ordering) -> u64 {
        unsafe {
            let result: u64;
            asm!(
                "lock xadd qword ptr [{}], {}",
                in(reg) &self.value,
                inout(reg) value => result,
                options(nostack)
            );
            result
        }
    }

    /// Atomically subtracts from the current value and returns the previous value
    #[inline(always)]
    pub fn fetch_sub(&self, value: u64, order: Ordering) -> u64 {
        self.fetch_add((value as i64).wrapping_neg() as u64, order)
    }

    /// Gets the inner value (not atomic)
    #[inline(always)]
    pub fn into_inner(self) -> u64 {
        self.value
    }

    /// Gets a mutable reference to the inner value
    #[inline(always)]
    pub fn get_mut(&mut self) -> &mut u64 {
        &mut self.value
    }
}

/// Atomic boolean
#[repr(C, align(1))]
pub struct AtomicBool {
    value: u8,
}

unsafe impl Sync for AtomicBool {}
unsafe impl Send for AtomicBool {}

impl AtomicBool {
    /// Creates a new atomic boolean
    #[inline(always)]
    pub const fn new(value: bool) -> Self {
        Self { value: value as u8 }
    }

    /// Loads a value from the atomic boolean
    #[inline(always)]
    pub fn load(&self, order: Ordering) -> bool {
        let val = match order {
            Ordering::Relaxed => unsafe {
                let result: u8;
                asm!(
                    "mov {}, byte ptr [{}]",
                    out(reg_byte) result,
                    in(reg) &self.value,
                    options(readonly, nostack, preserves_flags)
                );
                result
            },
            Ordering::Acquire | Ordering::SeqCst => unsafe {
                let result: u8;
                asm!(
                    "mov {}, byte ptr [{}]",
                    "lock or dword ptr [rsp], 0",
                    out(reg_byte) result,
                    in(reg) &self.value,
                    options(readonly, nostack)
                );
                result
            },
            Ordering::Release => panic!("Invalid ordering for load"),
        };
        val != 0
    }

    /// Stores a value into the atomic boolean
    #[inline(always)]
    pub fn store(&self, value: bool, order: Ordering) {
        let val = value as u8;
        match order {
            Ordering::Relaxed => unsafe {
                asm!(
                    "mov byte ptr [{}], {}",
                    in(reg) &self.value,
                    in(reg_byte) val,
                    options(nostack, preserves_flags)
                );
            },
            Ordering::Release | Ordering::SeqCst => unsafe {
                asm!(
                    "xchg byte ptr [{}], {}",
                    in(reg) &self.value,
                    in(reg_byte) val,
                    options(nostack)
                );
            },
            Ordering::Acquire => panic!("Invalid ordering for store"),
        }
    }

    /// Compare-and-swap operation
    #[inline(always)]
    pub fn compare_and_swap(&self, current: bool, new: bool, _order: Ordering) -> bool {
        unsafe {
            let result: u8;
            asm!(
                "lock cmpxchg byte ptr [{}], {}",
                in(reg) &self.value,
                in(reg_byte) new as u8,
                inout("al") current as u8 => result,
                options(nostack)
            );
            result != 0
        }
    }

    /// Gets the inner value (not atomic)
    #[inline(always)]
    pub fn into_inner(self) -> bool {
        self.value != 0
    }

    /// Gets a mutable reference to the inner value
    #[inline(always)]
    pub fn get_mut(&mut self) -> &mut bool {
        unsafe { &mut *((&mut self.value) as *mut u8 as *mut bool) }
    }
}

/// Atomic pointer
#[repr(C)]
pub struct AtomicPtr<T> {
    ptr: *mut T,
}

unsafe impl<T> Sync for AtomicPtr<T> {}
unsafe impl<T> Send for AtomicPtr<T> {}

impl<T> AtomicPtr<T> {
    /// Creates a new atomic pointer
    #[inline(always)]
    pub const fn new(ptr: *mut T) -> Self {
        Self { ptr }
    }

    /// Loads a value from the atomic pointer
    #[inline(always)]
    pub fn load(&self, order: Ordering) -> *mut T {
        match order {
            Ordering::Relaxed => unsafe {
                let result: *mut T;
                asm!(
                    "mov {}, qword ptr [{}]",
                    out(reg) result,
                    in(reg) &self.ptr,
                    options(readonly, nostack, preserves_flags)
                );
                result
            },
            Ordering::Acquire | Ordering::SeqCst => unsafe {
                let result: *mut T;
                asm!(
                    "mov {}, qword ptr [{}]",
                    "lock or dword ptr [rsp], 0",
                    out(reg) result,
                    in(reg) &self.ptr,
                    options(readonly, nostack)
                );
                result
            },
            Ordering::Release => panic!("Invalid ordering for load"),
        }
    }

    /// Stores a value into the atomic pointer
    #[inline(always)]
    pub fn store(&self, ptr: *mut T, order: Ordering) {
        match order {
            Ordering::Relaxed => unsafe {
                asm!(
                    "mov qword ptr [{}], {}",
                    in(reg) &self.ptr,
                    in(reg) ptr,
                    options(nostack, preserves_flags)
                );
            },
            Ordering::Release | Ordering::SeqCst => unsafe {
                asm!(
                    "xchg qword ptr [{}], {}",
                    in(reg) &self.ptr,
                    in(reg) ptr,
                    options(nostack)
                );
            },
            Ordering::Acquire => panic!("Invalid ordering for store"),
        }
    }

    /// Compare-and-swap operation
    #[inline(always)]
    pub fn compare_and_swap(&self, current: *mut T, new: *mut T, _order: Ordering) -> *mut T {
        unsafe {
            let result: *mut T;
            asm!(
                "lock cmpxchg qword ptr [{}], {}",
                in(reg) &self.ptr,
                in(reg) new,
                inout("rax") current => result,
                options(nostack)
            );
            result
        }
    }

    /// Gets the inner value (not atomic)
    #[inline(always)]
    pub fn into_inner(self) -> *mut T {
        self.ptr
    }

    /// Gets a mutable reference to the inner value
    #[inline(always)]
    pub fn get_mut(&mut self) -> &mut *mut T {
        &mut self.ptr
    }
}

/// Basic spinlock implementation using atomic operations
pub struct SpinLock {
    locked: AtomicBool,
}

impl SpinLock {
    /// Creates a new unlocked spinlock
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            locked: AtomicBool::new(false),
        }
    }

    /// Attempts to acquire the lock
    #[inline(always)]
    pub fn try_lock(&self) -> bool {
        !self.locked.compare_and_swap(false, true, Ordering::Acquire)
    }

    /// Acquires the lock, spinning until successful
    pub fn lock(&self) {
        while !self.try_lock() {
            // Spin with hint to CPU
            core::hint::spin_loop();
        }
    }

    /// Releases the lock
    #[inline(always)]
    pub fn unlock(&self) {
        self.locked.store(false, Ordering::Release);
    }
}

/// Memory fence - ensures ordering of memory operations
#[inline(always)]
pub fn fence(order: Ordering) {
    match order {
        Ordering::Relaxed => {},
        Ordering::Acquire | Ordering::Release | Ordering::SeqCst => unsafe {
            asm!("mfence", options(nostack, preserves_flags));
        },
    }
}

/// Compiler fence - prevents compiler reordering
#[inline(always)]
pub fn compiler_fence(_order: Ordering) {
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atomic_byte_basic() {
        let atomic = AtomicByte::new(0);
        assert_eq!(atomic.load(Ordering::Relaxed), 0);
        
        atomic.store(42, Ordering::Relaxed);
        assert_eq!(atomic.load(Ordering::Relaxed), 42);
    }

    #[test]
    fn test_atomic_byte_compare_and_swap() {
        let atomic = AtomicByte::new(10);
        
        // Successful CAS
        let old = atomic.compare_and_swap(10, 20, Ordering::SeqCst);
        assert_eq!(old, 10);
        assert_eq!(atomic.load(Ordering::SeqCst), 20);
        
        // Failed CAS
        let old = atomic.compare_and_swap(10, 30, Ordering::SeqCst);
        assert_eq!(old, 20);
        assert_eq!(atomic.load(Ordering::SeqCst), 20);
    }

    #[test]
    fn test_atomic_byte_fetch_add() {
        let atomic = AtomicByte::new(5);
        
        let old = atomic.fetch_add(10, Ordering::Relaxed);
        assert_eq!(old, 5);
        assert_eq!(atomic.load(Ordering::Relaxed), 15);
    }

    #[test]
    fn test_atomic_byte_fetch_sub() {
        let atomic = AtomicByte::new(20);
        
        let old = atomic.fetch_sub(5, Ordering::Relaxed);
        assert_eq!(old, 20);
        assert_eq!(atomic.load(Ordering::Relaxed), 15);
    }

    #[test]
    fn test_atomic_word_basic() {
        let atomic = AtomicWord::new(0);
        assert_eq!(atomic.load(Ordering::Relaxed), 0);
        
        atomic.store(1000, Ordering::Relaxed);
        assert_eq!(atomic.load(Ordering::Relaxed), 1000);
    }

    #[test]
    fn test_atomic_word_compare_and_swap() {
        let atomic = AtomicWord::new(100);
        
        let old = atomic.compare_and_swap(100, 200, Ordering::SeqCst);
        assert_eq!(old, 100);
        assert_eq!(atomic.load(Ordering::SeqCst), 200);
    }

    #[test]
    fn test_atomic_qword_basic() {
        let atomic = AtomicQWord::new(0);
        assert_eq!(atomic.load(Ordering::Relaxed), 0);
        
        atomic.store(0xDEADBEEF_CAFEBABE, Ordering::Relaxed);
        assert_eq!(atomic.load(Ordering::Relaxed), 0xDEADBEEF_CAFEBABE);
    }

    #[test]
    fn test_atomic_qword_compare_and_swap() {
        let atomic = AtomicQWord::new(1000);
        
        let old = atomic.compare_and_swap(1000, 2000, Ordering::SeqCst);
        assert_eq!(old, 1000);
        assert_eq!(atomic.load(Ordering::SeqCst), 2000);
    }

    #[test]
    fn test_atomic_bool_basic() {
        let atomic = AtomicBool::new(false);
        assert_eq!(atomic.load(Ordering::Relaxed), false);
        
        atomic.store(true, Ordering::Relaxed);
        assert_eq!(atomic.load(Ordering::Relaxed), true);
    }

    #[test]
    fn test_atomic_bool_compare_and_swap() {
        let atomic = AtomicBool::new(false);
        
        // Successful CAS
        let old = atomic.compare_and_swap(false, true, Ordering::SeqCst);
        assert_eq!(old, false);
        assert_eq!(atomic.load(Ordering::SeqCst), true);
        
        // Failed CAS
        let old = atomic.compare_and_swap(false, false, Ordering::SeqCst);
        assert_eq!(old, true);
        assert_eq!(atomic.load(Ordering::SeqCst), true);
    }

    #[test]
    fn test_atomic_ptr_basic() {
        let mut value = 42i32;
        let atomic = AtomicPtr::new(&mut value as *mut i32);
        
        let ptr = atomic.load(Ordering::Relaxed);
        assert!(!ptr.is_null());
        assert_eq!(unsafe { *ptr }, 42);
    }

    #[test]
    fn test_atomic_ptr_compare_and_swap() {
        let mut value1 = 10i32;
        let mut value2 = 20i32;
        let ptr1 = &mut value1 as *mut i32;
        let ptr2 = &mut value2 as *mut i32;
        
        let atomic = AtomicPtr::new(ptr1);
        
        let old = atomic.compare_and_swap(ptr1, ptr2, Ordering::SeqCst);
        assert_eq!(old, ptr1);
        assert_eq!(atomic.load(Ordering::SeqCst), ptr2);
    }

    #[test]
    fn test_spinlock_basic() {
        let lock = SpinLock::new();
        
        // Lock should be initially unlocked
        assert!(lock.try_lock());
        
        // Second attempt should fail
        assert!(!lock.try_lock());
        
        // Unlock
        lock.unlock();
        
        // Should be able to lock again
        assert!(lock.try_lock());
        lock.unlock();
    }

    #[test]
    fn test_ordering_types() {
        // Test that all ordering types exist
        let _ = Ordering::Relaxed;
        let _ = Ordering::Acquire;
        let _ = Ordering::Release;
        let _ = Ordering::SeqCst;
    }

    #[test]
    fn test_memory_ordering_semantics() {
        let atomic = AtomicByte::new(0);
        
        // Test different orderings work
        atomic.store(1, Ordering::Relaxed);
        assert_eq!(atomic.load(Ordering::Relaxed), 1);
        
        atomic.store(2, Ordering::Release);
        assert_eq!(atomic.load(Ordering::Acquire), 2);
        
        atomic.store(3, Ordering::SeqCst);
        assert_eq!(atomic.load(Ordering::SeqCst), 3);
    }

    #[test]
    fn test_fence() {
        fence(Ordering::SeqCst);
        fence(Ordering::Acquire);
        fence(Ordering::Release);
        fence(Ordering::Relaxed);
    }

    #[test]
    fn test_compiler_fence() {
        compiler_fence(Ordering::SeqCst);
        compiler_fence(Ordering::Acquire);
        compiler_fence(Ordering::Release);
        compiler_fence(Ordering::Relaxed);
    }

    // Stress tests
    #[test]
    fn stress_test_atomic_byte() {
        let atomic = AtomicByte::new(0);
        
        for i in 0..255 {
            atomic.store(i, Ordering::SeqCst);
            assert_eq!(atomic.load(Ordering::SeqCst), i);
        }
    }

    #[test]
    fn stress_test_fetch_add() {
        let atomic = AtomicByte::new(0);
        
        for _ in 0..100 {
            atomic.fetch_add(1, Ordering::SeqCst);
        }
        
        assert_eq!(atomic.load(Ordering::SeqCst), 100);
    }

    #[test]
    fn stress_test_compare_and_swap() {
        let atomic = AtomicQWord::new(0);
        
        for i in 0..1000 {
            loop {
                let current = atomic.load(Ordering::Acquire);
                let old = atomic.compare_and_swap(current, i, Ordering::SeqCst);
                if old == current {
                    break;
                }
            }
        }
        
        assert_eq!(atomic.load(Ordering::SeqCst), 999);
    }

    #[test]
    fn stress_test_spinlock() {
        let lock = SpinLock::new();
        
        for _ in 0..1000 {
            lock.lock();
            // Critical section
            lock.unlock();
        }
    }

    #[test]
    fn test_atomic_byte_overflow() {
        let atomic = AtomicByte::new(255);
        atomic.fetch_add(1, Ordering::SeqCst);
        assert_eq!(atomic.load(Ordering::SeqCst), 0); // Wraps around
    }

    #[test]
    fn test_atomic_byte_underflow() {
        let atomic = AtomicByte::new(0);
        atomic.fetch_sub(1, Ordering::SeqCst);
        assert_eq!(atomic.load(Ordering::SeqCst), 255); // Wraps around
    }

    #[test]
    fn test_atomic_word_large_values() {
        let atomic = AtomicWord::new(0);
        atomic.store(u16::MAX, Ordering::SeqCst);
        assert_eq!(atomic.load(Ordering::SeqCst), u16::MAX);
    }

    #[test]
    fn test_atomic_qword_large_values() {
        let atomic = AtomicQWord::new(0);
        atomic.store(u64::MAX, Ordering::SeqCst);
        assert_eq!(atomic.load(Ordering::SeqCst), u64::MAX);
    }

    #[test]
    fn test_null_pointer() {
        let atomic: AtomicPtr<i32> = AtomicPtr::new(core::ptr::null_mut());
        assert!(atomic.load(Ordering::Relaxed).is_null());
    }

    #[test]
    fn test_spinlock_nested() {
        let lock1 = SpinLock::new();
        let lock2 = SpinLock::new();
        
        lock1.lock();
        lock2.lock();
        
        lock2.unlock();
        lock1.unlock();
        
        assert!(lock1.try_lock());
        assert!(lock2.try_lock());
    }

    #[test]
    fn test_into_inner() {
        let atomic = AtomicByte::new(42);
        assert_eq!(atomic.into_inner(), 42);
        
        let atomic = AtomicBool::new(true);
        assert_eq!(atomic.into_inner(), true);
    }

    #[test]
    fn test_get_mut() {
        let mut atomic = AtomicByte::new(10);
        *atomic.get_mut() = 20;
        assert_eq!(atomic.load(Ordering::Relaxed), 20);
    }
}
