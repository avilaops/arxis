//! # Ãvila Cryptography Suite
//!
//! **Sovereign cryptographic primitives with zero external dependencies.**
//!
//! ## Philosophy
//! - No compromises with government standards
//! - Mathematics over politics
//! - Battle-tested algorithms from Bitcoin/Ethereum
//! - State-of-the-art post-quantum preparation
//!
//! ## Core Principles
//! 1. **Stack-only allocation** - Zero heap usage for crypto operations
//! 2. **Fixed-size types** - Compile-time known sizes for optimization
//! 3. **Constant-time operations** - Side-channel resistant
//! 4. **SIMD-optimized** - AVX-512 native support
//! 5. **Zero dependencies** - Everything from scratch

#![no_std]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod bigint;
pub mod curves;
pub mod signatures;
pub mod hash;
pub mod encryption;
pub mod mac;

/// Version of the library
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

