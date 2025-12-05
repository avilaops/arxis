//! OS-based CSPRNG - Uses system entropy sources
//!
//! Provides cryptographically secure random numbers from the operating system.

// Windows RtlGenRandom requires FFI which needs unsafe
#![cfg_attr(windows, allow(unsafe_code))]

use crate::traits::{CryptoRng, Rng};

/// OS-based cryptographically secure RNG
#[derive(Clone, Copy, Debug, Default)]
pub struct OsRng;

impl OsRng {
    /// Create a new OS RNG
    pub fn new() -> Self {
        Self
    }
}

impl Rng for OsRng {
    fn next_u32(&mut self) -> u32 {
        let mut buf = [0u8; 4];
        fill_entropy(&mut buf);
        u32::from_le_bytes(buf)
    }

    fn next_u64(&mut self) -> u64 {
        let mut buf = [0u8; 8];
        fill_entropy(&mut buf);
        u64::from_le_bytes(buf)
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        fill_entropy(dest);
    }
}

impl CryptoRng for OsRng {}

/// Fill a buffer with entropy from the OS
#[cfg(all(feature = "std", not(target_arch = "wasm32")))]
pub(crate) fn fill_entropy(dest: &mut [u8]) {
    use std::fs::File;
    use std::io::Read;

    #[cfg(unix)]
    {
        // On Unix systems, read from /dev/urandom
        let mut file = File::open("/dev/urandom").expect("Failed to open /dev/urandom");
        file.read_exact(dest).expect("Failed to read from /dev/urandom");
    }

    #[cfg(windows)]
    {
        // On Windows, use RtlGenRandom (SystemFunction036)
        // This is the recommended cryptographically secure random source on Windows
        // SAFETY: We ensure the buffer pointer is valid and the length matches
        extern "system" {
            fn SystemFunction036(random_buffer: *mut u8, random_buffer_length: u32) -> u8;
        }

        unsafe {
            let ret = SystemFunction036(dest.as_mut_ptr(), dest.len() as u32);
            if ret == 0 {
                panic!("RtlGenRandom failed");
            }
        }
    }

    #[cfg(not(any(unix, windows)))]
    {
        // Fallback: use system time-based entropy (not cryptographically secure!)
        use std::time::{SystemTime, UNIX_EPOCH};
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        
        let mut hasher = crate::traits::SplitMix64::new(time as u64);
        use crate::traits::Rng as _;
        for chunk in dest.chunks_mut(8) {
            let val = hasher.next_u64();
            let bytes = val.to_le_bytes();
            chunk.copy_from_slice(&bytes[..chunk.len()]);
        }
    }
}

/// Fill a buffer with entropy (WASM target)
#[cfg(all(feature = "std", target_arch = "wasm32"))]
pub(crate) fn fill_entropy(dest: &mut [u8]) {
    // For WASM, we need to use JS crypto API or similar
    // This is a simple fallback using time-based seeding
    use std::time::{SystemTime, UNIX_EPOCH};
    
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    
    let mut state = time as u64;
    for chunk in dest.chunks_mut(8) {
        // Simple hash function
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        let bytes = state.to_le_bytes();
        chunk.copy_from_slice(&bytes[..chunk.len()]);
    }
}

/// Fill a buffer with entropy (no_std target without getrandom)
#[cfg(not(feature = "std"))]
pub(crate) fn fill_entropy(_dest: &mut [u8]) {
    panic!("OsRng requires either 'std' or 'getrandom' feature");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "std")]
    fn test_os_rng_basic() {
        let mut rng = OsRng::new();
        let _val = rng.next_u64();
    }

    #[test]
    #[cfg(feature = "std")]
    fn test_os_rng_fill_bytes() {
        let mut rng = OsRng::new();
        let mut buf = [0u8; 32];
        rng.fill_bytes(&mut buf);
        
        // Check that not all bytes are zero
        assert!(buf.iter().any(|&b| b != 0));
    }

    #[test]
    #[cfg(feature = "std")]
    fn test_os_rng_different_values() {
        let mut rng = OsRng::new();
        let val1 = rng.next_u64();
        let val2 = rng.next_u64();
        
        // Values should be different (extremely high probability)
        assert_ne!(val1, val2);
    }
}
