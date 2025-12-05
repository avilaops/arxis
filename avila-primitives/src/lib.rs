//! # Avila Primitives - Layer 1 Foundation
//!
//! **100% from scratch primitive types with ZERO dependencies**
//!
//! This crate provides the absolute foundation for the Avila ecosystem:
//!
//! ## Numeric Wrapper Types
//! - **Byte, Word, DWord, QWord** - Semantic wrappers around primitive integers
//!
//! ## Semantic Types
//! - **Index, Offset, Size** - Type-safe numeric types for different purposes
//!
//! ## Bit Manipulation
//! - **BitSet** - Fixed-size bit operations
//! - **BitVec** - Dynamic bit vector
//!
//! ## Collections (from scratch)
//! - **Vec<T>** - Dynamic array with custom allocator
//! - **String** - UTF-8 string built on Vec<u8>
//! - **HashMap<K,V>** - Hash table with open addressing
//!
//! ## Memory Management
//! - **BumpAllocator** - Simple bump allocator for no_std environments
//!
//! ## Philosophy
//! - ❌ **ZERO** std, core, alloc
//! - ❌ **ZERO** external dependencies
//! - ✅ Only Rust primitives (u8, i32, etc)
//! - ✅ Inline assembly when needed
//! - ✅ 100% no_std compatible

#![no_std]
#![warn(missing_docs)]
#![warn(clippy::all)]
#![allow(internal_features)]
#![feature(core_intrinsics)]
#![feature(lang_items)]
#![feature(alloc_error_handler)]

// Core modules
pub mod types;
pub mod bits;
pub mod alloc;
pub mod vec;
pub mod string;
pub mod hashmap;

// Re-export main types
pub use types::{Byte, Word, DWord, QWord, Index, Offset, Size};
pub use bits::{BitSet, BitVec};
pub use vec::Vec;
pub use string::String;
pub use hashmap::HashMap;
pub use alloc::BumpAllocator;

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::types::*;
    pub use crate::bits::*;
    pub use crate::vec::Vec;
    pub use crate::string::String;
    pub use crate::hashmap::HashMap;
    pub use crate::alloc::BumpAllocator;
}

// Panic handler for no_std
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

// Language items required for no_std
#[cfg(not(test))]
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[cfg(not(test))]
#[alloc_error_handler]
fn alloc_error(_layout: core::alloc::Layout) -> ! {
    loop {}
}
