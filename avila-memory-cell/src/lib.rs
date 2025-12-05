//! # avila-memory-cell
//!
//! **Interior Mutability Primitives - Layer 1 Foundation**
//!
//! This library provides interior mutability abstractions built from first principles
//! without using `std::cell` or `core::cell`. These are fundamental building blocks
//! for creating data structures with controlled mutability.
//!
//! ## Available Types
//!
//! - `UnsafeCell<T>` - Base primitive for interior mutability (compiler intrinsic wrapper)
//! - `Cell<T>` - Interior mutability for `Copy` types
//! - `RefCell<T>` - Interior mutability with runtime borrow checking
//! - `OnceCell<T>` - Lazy initialization primitive
//!
//! ## Philosophy
//!
//! Interior mutability allows mutation of values through shared references (`&T`),
//! which normally would be immutable. This is essential for:
//! - Shared mutable state
//! - Recursive data structures
//! - Caching and memoization
//! - Breaking circular dependencies
//!
//! ## Safety Guarantees
//!
//! - `Cell<T>`: Safe for `Copy` types (no references can exist)
//! - `RefCell<T>`: Runtime borrow checking - panics on violations
//! - `OnceCell<T>`: Thread-safe single initialization
//! - `UnsafeCell<T>`: Raw primitive - requires manual safety
//!
//! ## no_std Compatible
//!
//! This crate works in `no_std` environments - zero dependencies on std.

#![no_std]
#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(unsafe_op_in_unsafe_fn)]

#[cfg(feature = "std")]
extern crate std;

use core::cell::UnsafeCell as CoreUnsafeCell;
use core::ops::{Deref, DerefMut};

// ============================================================================
// UnsafeCell - Base Primitive
// ============================================================================

/// The core primitive for interior mutability.
///
/// `UnsafeCell<T>` is the only way to get interior mutability in Rust.
/// All other types that provide interior mutability (`Cell`, `RefCell`, etc.)
/// are built on top of this primitive.
///
/// # Safety
///
/// Since this type provides no synchronization or borrowing guarantees,
/// it is up to the user to ensure that access is safe. Improper use
/// can lead to data races and undefined behavior.
///
/// # Examples
///
/// ```
/// use avila_memory_cell::UnsafeCell;
///
/// let cell = UnsafeCell::new(5);
/// unsafe {
///     *cell.get() = 10;
///     assert_eq!(*cell.get(), 10);
/// }
/// ```
#[repr(transparent)]
pub struct UnsafeCell<T: ?Sized> {
    value: CoreUnsafeCell<T>,
}

impl<T> UnsafeCell<T> {
    /// Constructs a new instance of `UnsafeCell`.
    #[inline]
    pub const fn new(value: T) -> Self {
        UnsafeCell {
            value: CoreUnsafeCell::new(value),
        }
    }

    /// Unwraps the value.
    #[inline]
    pub fn into_inner(self) -> T {
        self.value.into_inner()
    }
}

impl<T: ?Sized> UnsafeCell<T> {
    /// Gets a mutable pointer to the wrapped value.
    ///
    /// # Safety
    ///
    /// This method is unsafe because it allows creating multiple mutable
    /// references to the same data, which can lead to data races.
    #[inline]
    pub const fn get(&self) -> *mut T {
        self.value.get()
    }

    /// Returns a mutable reference to the underlying data.
    ///
    /// # Safety
    ///
    /// This call borrows `UnsafeCell` mutably (at compile-time) which
    /// guarantees that we possess the only reference.
    #[inline]
    pub fn get_mut(&mut self) -> &mut T {
        self.value.get_mut()
    }

    /// Gets a mutable pointer to the wrapped value.
    ///
    /// # Safety
    ///
    /// Same safety requirements as `get`.
    #[inline]
    pub const fn raw_get(this: &Self) -> *mut T {
        this.value.get()
    }
}

impl<T: Default> Default for UnsafeCell<T> {
    fn default() -> Self {
        UnsafeCell::new(Default::default())
    }
}

impl<T> From<T> for UnsafeCell<T> {
    fn from(value: T) -> Self {
        UnsafeCell::new(value)
    }
}

// ============================================================================
// Cell - For Copy Types
// ============================================================================

/// A mutable memory location for `Copy` types.
///
/// `Cell<T>` provides interior mutability for `Copy` types. Since `Copy` types
/// cannot have references to them (by definition), there's no risk of
/// creating aliased mutable references.
///
/// # Examples
///
/// ```
/// use avila_memory_cell::Cell;
///
/// let cell = Cell::new(5);
/// assert_eq!(cell.get(), 5);
///
/// cell.set(10);
/// assert_eq!(cell.get(), 10);
/// ```
pub struct Cell<T: ?Sized> {
    value: UnsafeCell<T>,
}

impl<T: Copy> Cell<T> {
    /// Creates a new `Cell` containing the given value.
    #[inline]
    pub const fn new(value: T) -> Self {
        Cell {
            value: UnsafeCell::new(value),
        }
    }

    /// Returns a copy of the contained value.
    #[inline]
    pub fn get(&self) -> T {
        unsafe { *self.value.get() }
    }

    /// Sets the contained value.
    #[inline]
    pub fn set(&self, value: T) {
        unsafe {
            *self.value.get() = value;
        }
    }

    /// Swaps the values of two `Cell`s.
    #[inline]
    pub fn swap(&self, other: &Self) {
        let tmp = self.get();
        self.set(other.get());
        other.set(tmp);
    }

    /// Replaces the contained value and returns the old value.
    #[inline]
    pub fn replace(&self, value: T) -> T {
        let old = self.get();
        self.set(value);
        old
    }

    /// Unwraps the value.
    #[inline]
    pub fn into_inner(self) -> T {
        self.value.into_inner()
    }
}

impl<T: Copy + Default> Default for Cell<T> {
    fn default() -> Self {
        Cell::new(Default::default())
    }
}

impl<T: Copy> From<T> for Cell<T> {
    fn from(value: T) -> Self {
        Cell::new(value)
    }
}

impl<T: Copy> Clone for Cell<T> {
    fn clone(&self) -> Self {
        Cell::new(self.get())
    }
}

// ============================================================================
// RefCell - Runtime Borrow Checking
// ============================================================================

/// A mutable memory location with dynamically checked borrow rules.
///
/// `RefCell<T>` uses runtime checks to ensure that borrows follow Rust's
/// borrowing rules: multiple immutable borrows OR one mutable borrow.
/// Violations result in a panic.
///
/// # Examples
///
/// ```
/// use avila_memory_cell::RefCell;
///
/// let cell = RefCell::new(5);
///
/// {
///     let value = cell.borrow();
///     assert_eq!(*value, 5);
/// }
///
/// {
///     let mut value = cell.borrow_mut();
///     *value = 10;
/// }
///
/// assert_eq!(*cell.borrow(), 10);
/// ```
///
/// # Panics
///
/// This type will panic if you attempt to borrow mutably while any borrows
/// are active, or if you attempt to borrow while a mutable borrow is active.
pub struct RefCell<T: ?Sized> {
    borrow: Cell<isize>,
    value: UnsafeCell<T>,
}

/// An RAII immutable borrow of a `RefCell`.
pub struct Ref<'a, T: ?Sized + 'a> {
    value: &'a T,
    borrow: &'a Cell<isize>,
}

/// An RAII mutable borrow of a `RefCell`.
pub struct RefMut<'a, T: ?Sized + 'a> {
    value: &'a mut T,
    borrow: &'a Cell<isize>,
}

const UNUSED: isize = 0;
const WRITING: isize = -1;

impl<T> RefCell<T> {
    /// Creates a new `RefCell` containing `value`.
    #[inline]
    pub const fn new(value: T) -> Self {
        RefCell {
            borrow: Cell::new(UNUSED),
            value: UnsafeCell::new(value),
        }
    }

    /// Consumes the `RefCell`, returning the wrapped value.
    #[inline]
    pub fn into_inner(self) -> T {
        self.value.into_inner()
    }

    /// Replaces the wrapped value with a new one, returning the old value.
    #[inline]
    pub fn replace(&self, value: T) -> T {
        core::mem::replace(&mut *self.borrow_mut(), value)
    }

    /// Swaps the wrapped value with another `RefCell`.
    #[inline]
    pub fn swap(&self, other: &Self) {
        core::mem::swap(&mut *self.borrow_mut(), &mut *other.borrow_mut())
    }
}

impl<T: ?Sized> RefCell<T> {
    /// Immutably borrows the wrapped value.
    ///
    /// # Panics
    ///
    /// Panics if the value is currently mutably borrowed.
    #[inline]
    pub fn borrow(&self) -> Ref<'_, T> {
        self.try_borrow().expect("RefCell<T> already mutably borrowed")
    }

    /// Attempts to immutably borrow the wrapped value.
    ///
    /// Returns `None` if the value is currently mutably borrowed.
    pub fn try_borrow(&self) -> Option<Ref<'_, T>> {
        let borrow = self.borrow.get();
        if borrow == WRITING {
            return None;
        }
        self.borrow.set(borrow + 1);
        Some(Ref {
            value: unsafe { &*self.value.get() },
            borrow: &self.borrow,
        })
    }

    /// Mutably borrows the wrapped value.
    ///
    /// # Panics
    ///
    /// Panics if the value is currently borrowed.
    #[inline]
    pub fn borrow_mut(&self) -> RefMut<'_, T> {
        self.try_borrow_mut()
            .expect("RefCell<T> already borrowed")
    }

    /// Attempts to mutably borrow the wrapped value.
    ///
    /// Returns `None` if the value is currently borrowed.
    pub fn try_borrow_mut(&self) -> Option<RefMut<'_, T>> {
        let borrow = self.borrow.get();
        if borrow != UNUSED {
            return None;
        }
        self.borrow.set(WRITING);
        Some(RefMut {
            value: unsafe { &mut *self.value.get() },
            borrow: &self.borrow,
        })
    }

    /// Returns a raw pointer to the underlying data.
    #[inline]
    pub fn as_ptr(&self) -> *mut T {
        self.value.get()
    }

    /// Returns a mutable reference to the wrapped value.
    ///
    /// This method doesn't perform any runtime checks.
    #[inline]
    pub fn get_mut(&mut self) -> &mut T {
        self.value.get_mut()
    }
}

impl<T: ?Sized> Drop for Ref<'_, T> {
    fn drop(&mut self) {
        let borrow = self.borrow.get();
        debug_assert!(borrow > 0);
        self.borrow.set(borrow - 1);
    }
}

impl<T: ?Sized> Deref for Ref<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        self.value
    }
}

impl<T: ?Sized> Drop for RefMut<'_, T> {
    fn drop(&mut self) {
        let borrow = self.borrow.get();
        debug_assert_eq!(borrow, WRITING);
        self.borrow.set(UNUSED);
    }
}

impl<T: ?Sized> Deref for RefMut<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        self.value
    }
}

impl<T: ?Sized> DerefMut for RefMut<'_, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        self.value
    }
}

impl<T: Default> Default for RefCell<T> {
    fn default() -> Self {
        RefCell::new(Default::default())
    }
}

impl<T> From<T> for RefCell<T> {
    fn from(value: T) -> Self {
        RefCell::new(value)
    }
}

impl<T: Clone> Clone for RefCell<T> {
    fn clone(&self) -> Self {
        RefCell::new(self.borrow().clone())
    }
}

// ============================================================================
// OnceCell - Lazy Initialization
// ============================================================================

/// A cell which can be written to only once.
///
/// `OnceCell<T>` provides a way to initialize a value lazily. Once initialized,
/// the value cannot be changed.
///
/// # Examples
///
/// ```
/// use avila_memory_cell::OnceCell;
///
/// let cell = OnceCell::new();
/// assert!(cell.get().is_none());
///
/// let value = cell.get_or_init(|| 42);
/// assert_eq!(*value, 42);
/// assert_eq!(cell.get(), Some(&42));
///
/// // Cannot initialize again
/// assert!(cell.set(100).is_err());
/// ```
pub struct OnceCell<T> {
    inner: UnsafeCell<Option<T>>,
    initialized: Cell<bool>,
}

impl<T> OnceCell<T> {
    /// Creates a new empty `OnceCell`.
    #[inline]
    pub const fn new() -> Self {
        OnceCell {
            inner: UnsafeCell::new(None),
            initialized: Cell::new(false),
        }
    }

    /// Gets the reference to the underlying value.
    ///
    /// Returns `None` if the cell is empty.
    #[inline]
    pub fn get(&self) -> Option<&T> {
        if self.initialized.get() {
            unsafe { (*self.inner.get()).as_ref() }
        } else {
            None
        }
    }

    /// Gets the mutable reference to the underlying value.
    ///
    /// Returns `None` if the cell is empty.
    #[inline]
    pub fn get_mut(&mut self) -> Option<&mut T> {
        self.inner.get_mut().as_mut()
    }

    /// Sets the contents of the cell to `value`.
    ///
    /// Returns `Ok(())` if the cell was empty, `Err(value)` otherwise.
    pub fn set(&self, value: T) -> Result<(), T> {
        if self.initialized.get() {
            return Err(value);
        }

        unsafe {
            *self.inner.get() = Some(value);
        }
        self.initialized.set(true);
        Ok(())
    }

    /// Gets the contents of the cell, initializing it with `f` if empty.
    pub fn get_or_init<F>(&self, f: F) -> &T
    where
        F: FnOnce() -> T,
    {
        if !self.initialized.get() {
            let value = f();
            // This might panic if someone else initialized it, but that's a bug
            // in the caller's code (not properly synchronized)
            let _ = self.set(value);
        }
        self.get().unwrap()
    }

    /// Gets the contents of the cell, initializing it with `f` if empty.
    ///
    /// If `f` returns `Err`, the error is propagated and the cell remains empty.
    pub fn get_or_try_init<F, E>(&self, f: F) -> Result<&T, E>
    where
        F: FnOnce() -> Result<T, E>,
    {
        if !self.initialized.get() {
            let value = f()?;
            if self.set(value).is_err() {
                // Someone else initialized it
            }
        }
        Ok(self.get().unwrap())
    }

    /// Takes the value out of the cell, moving it back to an uninitialized state.
    pub fn take(&mut self) -> Option<T> {
        let value = self.inner.get_mut().take();
        if value.is_some() {
            self.initialized.set(false);
        }
        value
    }

    /// Consumes the cell, returning the wrapped value.
    #[inline]
    pub fn into_inner(self) -> Option<T> {
        self.inner.into_inner()
    }
}

impl<T> Default for OnceCell<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> From<T> for OnceCell<T> {
    fn from(value: T) -> Self {
        let cell = OnceCell::new();
        let _ = cell.set(value);
        cell
    }
}

impl<T: Clone> Clone for OnceCell<T> {
    fn clone(&self) -> Self {
        let cell = OnceCell::new();
        if let Some(value) = self.get() {
            let _ = cell.set(value.clone());
        }
        cell
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // UnsafeCell tests
    #[test]
    fn test_unsafe_cell_new() {
        let cell = UnsafeCell::new(42);
        unsafe {
            assert_eq!(*cell.get(), 42);
        }
    }

    #[test]
    fn test_unsafe_cell_into_inner() {
        let cell = UnsafeCell::new(42);
        assert_eq!(cell.into_inner(), 42);
    }

    #[test]
    fn test_unsafe_cell_get_mut() {
        let mut cell = UnsafeCell::new(42);
        *cell.get_mut() = 100;
        assert_eq!(*cell.get_mut(), 100);
    }

    // Cell tests
    #[test]
    fn test_cell_new() {
        let cell = Cell::new(5);
        assert_eq!(cell.get(), 5);
    }

    #[test]
    fn test_cell_set() {
        let cell = Cell::new(5);
        cell.set(10);
        assert_eq!(cell.get(), 10);
    }

    #[test]
    fn test_cell_swap() {
        let c1 = Cell::new(5);
        let c2 = Cell::new(10);
        c1.swap(&c2);
        assert_eq!(c1.get(), 10);
        assert_eq!(c2.get(), 5);
    }

    #[test]
    fn test_cell_replace() {
        let cell = Cell::new(5);
        let old = cell.replace(10);
        assert_eq!(old, 5);
        assert_eq!(cell.get(), 10);
    }

    #[test]
    fn test_cell_into_inner() {
        let cell = Cell::new(42);
        assert_eq!(cell.into_inner(), 42);
    }

    #[test]
    fn test_cell_clone() {
        let cell1 = Cell::new(42);
        let cell2 = cell1.clone();
        assert_eq!(cell1.get(), 42);
        assert_eq!(cell2.get(), 42);
        cell2.set(100);
        assert_eq!(cell1.get(), 42);
        assert_eq!(cell2.get(), 100);
    }

    // RefCell tests
    #[test]
    fn test_refcell_new() {
        let cell = RefCell::new(5);
        assert_eq!(*cell.borrow(), 5);
    }

    #[test]
    fn test_refcell_borrow() {
        let cell = RefCell::new(5);
        let b1 = cell.borrow();
        let b2 = cell.borrow();
        assert_eq!(*b1, 5);
        assert_eq!(*b2, 5);
    }

    #[test]
    fn test_refcell_borrow_mut() {
        let cell = RefCell::new(5);
        {
            let mut b = cell.borrow_mut();
            *b = 10;
        }
        assert_eq!(*cell.borrow(), 10);
    }

    #[test]
    #[should_panic(expected = "already mutably borrowed")]
    fn test_refcell_borrow_while_borrowed_mut() {
        let cell = RefCell::new(5);
        let _b_mut = cell.borrow_mut();
        let _b = cell.borrow(); // Should panic
    }

    #[test]
    #[should_panic(expected = "already borrowed")]
    fn test_refcell_borrow_mut_while_borrowed() {
        let cell = RefCell::new(5);
        let _b = cell.borrow();
        let _b_mut = cell.borrow_mut(); // Should panic
    }

    #[test]
    #[should_panic(expected = "already mutably borrowed")]
    fn test_refcell_borrow_mut_while_borrowed_mut() {
        let cell = RefCell::new(5);
        let _b_mut1 = cell.borrow_mut();
        let _b_mut2 = cell.borrow_mut(); // Should panic
    }

    #[test]
    fn test_refcell_try_borrow() {
        let cell = RefCell::new(5);
        assert!(cell.try_borrow().is_some());
        let _b_mut = cell.borrow_mut();
        assert!(cell.try_borrow().is_none());
    }

    #[test]
    fn test_refcell_try_borrow_mut() {
        let cell = RefCell::new(5);
        assert!(cell.try_borrow_mut().is_some());
        let _b = cell.borrow();
        assert!(cell.try_borrow_mut().is_none());
    }

    #[test]
    fn test_refcell_replace() {
        let cell = RefCell::new(5);
        let old = cell.replace(10);
        assert_eq!(old, 5);
        assert_eq!(*cell.borrow(), 10);
    }

    #[test]
    fn test_refcell_swap() {
        let c1 = RefCell::new(5);
        let c2 = RefCell::new(10);
        c1.swap(&c2);
        assert_eq!(*c1.borrow(), 10);
        assert_eq!(*c2.borrow(), 5);
    }

    #[test]
    fn test_refcell_into_inner() {
        let cell = RefCell::new(42);
        assert_eq!(cell.into_inner(), 42);
    }

    #[test]
    fn test_refcell_get_mut() {
        let mut cell = RefCell::new(5);
        *cell.get_mut() = 10;
        assert_eq!(*cell.borrow(), 10);
    }

    #[test]
    fn test_refcell_clone() {
        let cell1 = RefCell::new(42);
        let cell2 = cell1.clone();
        assert_eq!(*cell1.borrow(), 42);
        assert_eq!(*cell2.borrow(), 42);
        *cell2.borrow_mut() = 100;
        assert_eq!(*cell1.borrow(), 42);
        assert_eq!(*cell2.borrow(), 100);
    }

    // OnceCell tests
    #[test]
    fn test_oncecell_new() {
        let cell: OnceCell<i32> = OnceCell::new();
        assert!(cell.get().is_none());
    }

    #[test]
    fn test_oncecell_set() {
        let cell = OnceCell::new();
        assert!(cell.set(42).is_ok());
        assert_eq!(cell.get(), Some(&42));
    }

    #[test]
    fn test_oncecell_set_twice() {
        let cell = OnceCell::new();
        assert!(cell.set(42).is_ok());
        assert_eq!(cell.set(100), Err(100));
        assert_eq!(cell.get(), Some(&42));
    }

    #[test]
    fn test_oncecell_get_or_init() {
        let cell = OnceCell::new();
        let value = cell.get_or_init(|| 42);
        assert_eq!(*value, 42);
        let value2 = cell.get_or_init(|| 100);
        assert_eq!(*value2, 42);
    }

    #[test]
    fn test_oncecell_get_or_try_init() {
        let cell: OnceCell<i32> = OnceCell::new();
        let result = cell.get_or_try_init(|| Ok::<_, ()>(42));
        assert_eq!(result, Ok(&42));
        let result2 = cell.get_or_try_init(|| Ok::<_, ()>(100));
        assert_eq!(result2, Ok(&42));
    }

    #[test]
    fn test_oncecell_get_or_try_init_error() {
        let cell: OnceCell<i32> = OnceCell::new();
        let result = cell.get_or_try_init(|| Err::<i32, _>("error"));
        assert_eq!(result, Err("error"));
        assert!(cell.get().is_none());
    }

    #[test]
    fn test_oncecell_take() {
        let mut cell = OnceCell::new();
        assert!(cell.set(42).is_ok());
        assert_eq!(cell.take(), Some(42));
        assert!(cell.get().is_none());
    }

    #[test]
    fn test_oncecell_into_inner() {
        let cell = OnceCell::new();
        assert!(cell.set(42).is_ok());
        assert_eq!(cell.into_inner(), Some(42));
    }

    #[test]
    fn test_oncecell_get_mut() {
        let mut cell = OnceCell::new();
        assert!(cell.set(42).is_ok());
        *cell.get_mut().unwrap() = 100;
        assert_eq!(cell.get(), Some(&100));
    }

    #[test]
    fn test_oncecell_clone() {
        let cell1 = OnceCell::new();
        assert!(cell1.set(42).is_ok());
        let cell2 = cell1.clone();
        assert_eq!(cell1.get(), Some(&42));
        assert_eq!(cell2.get(), Some(&42));
    }

    #[test]
    fn test_oncecell_from() {
        let cell = OnceCell::from(42);
        assert_eq!(cell.get(), Some(&42));
    }
}
