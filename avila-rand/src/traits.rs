//! Core traits for random number generators

/// A random number generator
pub trait Rng {
    /// Generate a random u32
    fn next_u32(&mut self) -> u32;

    /// Generate a random u64
    fn next_u64(&mut self) -> u64 {
        let hi = self.next_u32() as u64;
        let lo = self.next_u32() as u64;
        (hi << 32) | lo
    }

    /// Fill a byte slice with random data
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        let mut left = dest;
        while left.len() >= 8 {
            let (chunk, rest) = left.split_at_mut(8);
            let rand = self.next_u64();
            chunk.copy_from_slice(&rand.to_le_bytes());
            left = rest;
        }
        if !left.is_empty() {
            let rand = self.next_u64();
            let bytes = rand.to_le_bytes();
            left.copy_from_slice(&bytes[..left.len()]);
        }
    }

    /// Generate a random value of type T
    fn gen<T: crate::Random>(&mut self) -> T {
        T::random(self)
    }

    /// Generate a random value in a range
    fn gen_range<T: crate::RandomRange>(&mut self, range: core::ops::Range<T>) -> T {
        T::random_range(self, range)
    }
}

/// A marker trait for cryptographically secure RNGs
pub trait CryptoRng {}

/// A random number generator that can be seeded
pub trait SeedableRng: Sized {
    /// Seed type
    type Seed: Default + AsMut<[u8]>;

    /// Create a new RNG from a seed
    fn from_seed(seed: Self::Seed) -> Self;

    /// Create a new RNG from entropy (requires std or getrandom feature)
    #[cfg(any(feature = "std", feature = "getrandom"))]
    fn from_entropy() -> Self {
        let mut seed = Self::Seed::default();
        crate::csprng::fill_entropy(seed.as_mut());
        Self::from_seed(seed)
    }

    /// Seed from a u64 value
    fn seed_from_u64(state: u64) -> Self {
        let mut seed = Self::Seed::default();
        let seed_slice = seed.as_mut();
        let mut hasher = SplitMix64::new(state);
        
        for chunk in seed_slice.chunks_mut(8) {
            let val = hasher.next_u64();
            let bytes = val.to_le_bytes();
            chunk.copy_from_slice(&bytes[..chunk.len()]);
        }
        
        Self::from_seed(seed)
    }
}

/// SplitMix64 - used for seeding
pub(crate) struct SplitMix64 {
    state: u64,
}

impl SplitMix64 {
    pub(crate) fn new(state: u64) -> Self {
        Self { state }
    }

    pub(crate) fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    }
}

impl Rng for SplitMix64 {
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    fn next_u64(&mut self) -> u64 {
        SplitMix64::next_u64(self)
    }
}
