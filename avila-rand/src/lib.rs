//! # Avila Rand - Cryptographically Secure Random Number Generator
//!
//! A `no_std` compatible random number generator library without external dependencies.
//! Implements multiple PRNG algorithms including ChaCha20, Xoshiro256**, and OS-based CSPRNG.
//!
//! ## Features
//! - ChaCha20 PRNG (cryptographically secure)
//! - Xoshiro256** (fast, high-quality non-cryptographic)
//! - OS entropy source (CSPRNG)
//! - Multiple distributions (uniform, normal, exponential)
//! - `no_std` compatible
//! - Zero external dependencies

#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]

#[cfg(feature = "std")]
extern crate std;

#[cfg(not(feature = "std"))]
extern crate core as std;

mod chacha;
mod csprng;
mod distributions;
mod traits;
mod xoshiro;

pub use chacha::ChaCha20Rng;
pub use csprng::OsRng;
pub use distributions::{Distribution, Uniform};
#[cfg(feature = "std")]
pub use distributions::{Exponential, Normal, Bernoulli, Gamma};
pub use traits::{CryptoRng, Rng, SeedableRng};
pub use xoshiro::Xoshiro256StarStar;

/// Default thread-local RNG (only available with std)
#[cfg(feature = "std")]
pub mod thread_rng {
    use super::*;
    use std::cell::RefCell;

    thread_local! {
        static THREAD_RNG: RefCell<ChaCha20Rng> = RefCell::new(ChaCha20Rng::from_entropy());
    }

    /// Get thread-local random value
    pub fn random<T: Random>() -> T {
        THREAD_RNG.with(|rng| T::random(&mut *rng.borrow_mut()))
    }

    /// Get thread-local random value in range
    pub fn random_range<T: RandomRange>(range: core::ops::Range<T>) -> T {
        THREAD_RNG.with(|rng| T::random_range(&mut *rng.borrow_mut(), range))
    }
}

#[cfg(feature = "std")]
pub use thread_rng::{random, random_range};

/// Trait for types that can be randomly generated
pub trait Random: Sized {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Self;
}

/// Trait for types that can be generated in a range
pub trait RandomRange: Sized {
    fn random_range<R: Rng + ?Sized>(rng: &mut R, range: core::ops::Range<Self>) -> Self;
}

// Implementations for basic types
impl Random for u8 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        rng.next_u32() as u8
    }
}

impl Random for u16 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        rng.next_u32() as u16
    }
}

impl Random for u32 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        rng.next_u32()
    }
}

impl Random for u64 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        rng.next_u64()
    }
}

impl Random for u128 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        let hi = rng.next_u64() as u128;
        let lo = rng.next_u64() as u128;
        (hi << 64) | lo
    }
}

impl Random for i8 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        rng.next_u32() as i8
    }
}

impl Random for i16 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        rng.next_u32() as i16
    }
}

impl Random for i32 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        rng.next_u32() as i32
    }
}

impl Random for i64 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        rng.next_u64() as i64
    }
}

impl Random for i128 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        u128::random(rng) as i128
    }
}

impl Random for bool {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        (rng.next_u32() & 1) == 1
    }
}

impl Random for f32 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        // Generate a float in [0, 1)
        let bits = rng.next_u32() >> 8; // Use 24 bits
        let scale = 1.0 / ((1u32 << 24) as f32);
        bits as f32 * scale
    }
}

impl Random for f64 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        // Generate a float in [0, 1)
        let bits = rng.next_u64() >> 11; // Use 53 bits
        let scale = 1.0 / ((1u64 << 53) as f64);
        bits as f64 * scale
    }
}

// Range implementations
impl RandomRange for u8 {
    fn random_range<R: Rng + ?Sized>(rng: &mut R, range: core::ops::Range<Self>) -> Self {
        let span = range.end.wrapping_sub(range.start);
        range.start.wrapping_add((rng.next_u32() % span as u32) as u8)
    }
}

impl RandomRange for u16 {
    fn random_range<R: Rng + ?Sized>(rng: &mut R, range: core::ops::Range<Self>) -> Self {
        let span = range.end.wrapping_sub(range.start);
        range.start.wrapping_add((rng.next_u32() % span as u32) as u16)
    }
}

impl RandomRange for u32 {
    fn random_range<R: Rng + ?Sized>(rng: &mut R, range: core::ops::Range<Self>) -> Self {
        let span = range.end.wrapping_sub(range.start);
        range.start.wrapping_add(rng.next_u32() % span)
    }
}

impl RandomRange for u64 {
    fn random_range<R: Rng + ?Sized>(rng: &mut R, range: core::ops::Range<Self>) -> Self {
        let span = range.end.wrapping_sub(range.start);
        range.start.wrapping_add(rng.next_u64() % span)
    }
}

impl RandomRange for usize {
    fn random_range<R: Rng + ?Sized>(rng: &mut R, range: core::ops::Range<Self>) -> Self {
        let span = range.end.wrapping_sub(range.start);
        range.start.wrapping_add((rng.next_u64() as usize) % span)
    }
}

impl RandomRange for i32 {
    fn random_range<R: Rng + ?Sized>(rng: &mut R, range: core::ops::Range<Self>) -> Self {
        let span = range.end.wrapping_sub(range.start) as u32;
        range.start.wrapping_add((rng.next_u32() % span) as i32)
    }
}

impl RandomRange for i64 {
    fn random_range<R: Rng + ?Sized>(rng: &mut R, range: core::ops::Range<Self>) -> Self {
        let span = range.end.wrapping_sub(range.start) as u64;
        range.start.wrapping_add((rng.next_u64() % span) as i64)
    }
}

impl RandomRange for f32 {
    fn random_range<R: Rng + ?Sized>(rng: &mut R, range: core::ops::Range<Self>) -> Self {
        let span = range.end - range.start;
        range.start + f32::random(rng) * span
    }
}

impl RandomRange for f64 {
    fn random_range<R: Rng + ?Sized>(rng: &mut R, range: core::ops::Range<Self>) -> Self {
        let span = range.end - range.start;
        range.start + f64::random(rng) * span
    }
}

/// Fill a byte slice with random data
pub fn fill_bytes<R: Rng + ?Sized>(rng: &mut R, dest: &mut [u8]) {
    rng.fill_bytes(dest);
}

/// Shuffle a slice in place
pub fn shuffle<T, R: Rng + ?Sized>(rng: &mut R, slice: &mut [T]) {
    for i in (1..slice.len()).rev() {
        let j = usize::random_range(rng, 0..i + 1);
        slice.swap(i, j);
    }
}
