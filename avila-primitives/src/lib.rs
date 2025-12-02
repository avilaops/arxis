//! # Avila Primitives
//!
//! Big integer primitives for cryptography and high-precision arithmetic.
//!
//! This crate provides:
//! - **U256, U512, U1024, U2048, U4096** - Unsigned big integers
//! - **I256, I512, I1024, I2048, I4096** - Signed big integers
//! - **Arithmetic operations** - Add, sub, mul, div, mod with overflow detection
//! - **Bitwise operations** - AND, OR, XOR, NOT, shifts, rotations
//! - **Constant-time operations** - For cryptographic safety
//! - **SIMD acceleration** - Optional AVX2/AVX512 support
//!
//! Built on top of `avila-nucleus` for low-level bit manipulation.

#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]
#![warn(clippy::all)]

pub mod u256;
pub mod u512;
pub mod u1024;
pub mod u2048;
pub mod u4096;

pub mod i256;
pub mod i512;
pub mod i1024;
pub mod i2048;
pub mod i4096;

pub mod traits;

pub mod prelude {
    //! Common imports for convenience
    
    pub use crate::u256::U256;
    pub use crate::u512::U512;
    pub use crate::u1024::U1024;
    pub use crate::u2048::U2048;
    pub use crate::u4096::U4096;
    
    pub use crate::i256::I256;
    pub use crate::i512::I512;
    pub use crate::i1024::I1024;
    pub use crate::i2048::I2048;
    pub use crate::i4096::I4096;
    
    pub use crate::traits::{BigInt, BigUint};
}
