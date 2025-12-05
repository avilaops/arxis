//! Statistical distributions for random number generation
//!
//! Provides various probability distributions including uniform, normal, and exponential.
//!
//! Note: Distributions requiring floating-point math (Normal, Exponential, Gamma) are only
//! available with the `std` feature enabled.

use crate::traits::Rng;
use crate::{Random, RandomRange};

/// Trait for sampling from a probability distribution
pub trait Distribution<T> {
    /// Sample a value from this distribution
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> T;
}

/// Uniform distribution over a range
#[derive(Clone, Copy, Debug)]
pub struct Uniform<T> {
    low: T,
    high: T,
}

impl<T> Uniform<T> {
    /// Create a uniform distribution over [low, high)
    pub fn new(low: T, high: T) -> Self {
        Self { low, high }
    }
}

impl Distribution<f64> for Uniform<f64> {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        let val = f64::random(rng);
        self.low + val * (self.high - self.low)
    }
}

impl Distribution<f32> for Uniform<f32> {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f32 {
        let val = f32::random(rng);
        self.low + val * (self.high - self.low)
    }
}

impl Distribution<u32> for Uniform<u32> {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> u32 {
        u32::random_range(rng, self.low..self.high)
    }
}

impl Distribution<u64> for Uniform<u64> {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> u64 {
        u64::random_range(rng, self.low..self.high)
    }
}

impl Distribution<i32> for Uniform<i32> {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> i32 {
        i32::random_range(rng, self.low..self.high)
    }
}

impl Distribution<i64> for Uniform<i64> {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> i64 {
        i64::random_range(rng, self.low..self.high)
    }
}

// Advanced distributions require std for floating point math
#[cfg(feature = "std")]
mod float_distributions {
    use super::*;

    /// Standard normal distribution (mean=0, std=1)
    #[derive(Clone, Copy, Debug, Default)]
    pub struct Normal {
        mean: f64,
        std_dev: f64,
    }

    impl Normal {
        /// Create a new normal distribution with given mean and standard deviation
        pub fn new(mean: f64, std_dev: f64) -> Self {
            Self { mean, std_dev }
        }

        /// Standard normal distribution (mean=0, std=1)
        pub fn standard() -> Self {
            Self {
                mean: 0.0,
                std_dev: 1.0,
            }
        }
    }

    impl Distribution<f64> for Normal {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
            // Box-Muller transform
            let u1 = f64::random(rng);
            let u2 = f64::random(rng);

            // Avoid log(0)
            let u1 = if u1 < 1e-10 { 1e-10 } else { u1 };

            let r = (-2.0 * u1.ln()).sqrt();
            let theta = 2.0 * core::f64::consts::PI * u2;

            let z = r * theta.cos();
            self.mean + z * self.std_dev
        }
    }

    /// Exponential distribution
    #[derive(Clone, Copy, Debug)]
    pub struct Exponential {
        lambda: f64,
    }

    impl Exponential {
        /// Create a new exponential distribution with rate parameter lambda
        pub fn new(lambda: f64) -> Self {
            assert!(lambda > 0.0, "lambda must be positive");
            Self { lambda }
        }
    }

    impl Distribution<f64> for Exponential {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
            let u = f64::random(rng);
            // Avoid log(0)
            let u = if u < 1e-10 { 1e-10 } else { u };
            -u.ln() / self.lambda
        }
    }

    /// Bernoulli distribution (boolean with probability p)
    #[derive(Clone, Copy, Debug)]
    pub struct Bernoulli {
        p: f64,
    }

    impl Bernoulli {
        /// Create a Bernoulli distribution with success probability p
        pub fn new(p: f64) -> Self {
            assert!(p >= 0.0 && p <= 1.0, "p must be in [0, 1]");
            Self { p }
        }
    }

    impl Distribution<bool> for Bernoulli {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> bool {
            f64::random(rng) < self.p
        }
    }

    /// Gamma distribution (used for other distributions)
    #[derive(Clone, Copy, Debug)]
    pub struct Gamma {
        shape: f64,
        scale: f64,
    }

    impl Gamma {
        /// Create a new Gamma distribution
        pub fn new(shape: f64, scale: f64) -> Self {
            assert!(shape > 0.0 && scale > 0.0, "shape and scale must be positive");
            Self { shape, scale }
        }
    }

    impl Distribution<f64> for Gamma {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
            // Marsaglia and Tsang's method for shape >= 1
            if self.shape >= 1.0 {
                let d = self.shape - 1.0 / 3.0;
                let c = 1.0 / (9.0 * d).sqrt();

                loop {
                    let normal = Normal::standard();
                    let x = normal.sample(rng);
                    let v = (1.0 + c * x).powi(3);

                    if v > 0.0 {
                        let u = f64::random(rng);
                        let x2 = x * x;
                        
                        if u < 1.0 - 0.0331 * x2 * x2 {
                            return d * v * self.scale;
                        }
                        
                        if u.ln() < 0.5 * x2 + d * (1.0 - v + v.ln()) {
                            return d * v * self.scale;
                        }
                    }
                }
            } else {
                // For shape < 1, use the method of Johnk
                let exp = Exponential::new(1.0);
                loop {
                    let u = f64::random(rng);
                    let e = exp.sample(rng);
                    
                    let u_pow = u.powf(1.0 / self.shape);
                    if u_pow + e <= 1.0 {
                        return e * self.scale;
                    }
                }
            }
        }
    }
}

#[cfg(feature = "std")]
pub use float_distributions::{Bernoulli, Exponential, Gamma, Normal};

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;
    use crate::{ChaCha20Rng, traits::SeedableRng};

    #[test]
    fn test_uniform_f64() {
        let mut rng = ChaCha20Rng::seed_from_u64(123);
        let dist = Uniform::new(0.0, 10.0);

        for _ in 0..100 {
            let val = dist.sample(&mut rng);
            assert!(val >= 0.0 && val < 10.0);
        }
    }

    #[test]
    fn test_uniform_u32() {
        let mut rng = ChaCha20Rng::seed_from_u64(456);
        let dist = Uniform::new(5u32, 15u32);

        for _ in 0..100 {
            let val = dist.sample(&mut rng);
            assert!(val >= 5 && val < 15);
        }
    }

    #[test]
    fn test_normal_distribution() {
        let mut rng = ChaCha20Rng::seed_from_u64(789);
        let dist = Normal::new(10.0, 2.0);

        let mut sum = 0.0;
        let n = 1000;
        for _ in 0..n {
            let val = dist.sample(&mut rng);
            sum += val;
        }

        let mean = sum / n as f64;
        // Mean should be close to 10.0 (within 1.0 with high probability)
        assert!((mean - 10.0).abs() < 1.0);
    }

    #[test]
    fn test_exponential_distribution() {
        let mut rng = ChaCha20Rng::seed_from_u64(101112);
        let dist = Exponential::new(0.5);

        for _ in 0..100 {
            let val = dist.sample(&mut rng);
            assert!(val >= 0.0);
        }
    }

    #[test]
    fn test_bernoulli_distribution() {
        let mut rng = ChaCha20Rng::seed_from_u64(131415);
        let dist = Bernoulli::new(0.7);

        let mut count = 0;
        let n = 1000;
        for _ in 0..n {
            if dist.sample(&mut rng) {
                count += 1;
            }
        }

        let ratio = count as f64 / n as f64;
        // Should be close to 0.7 (within 0.1 with high probability)
        assert!((ratio - 0.7).abs() < 0.1);
    }

    #[test]
    fn test_gamma_distribution() {
        let mut rng = ChaCha20Rng::seed_from_u64(161718);
        let dist = Gamma::new(2.0, 1.0);

        for _ in 0..100 {
            let val = dist.sample(&mut rng);
            assert!(val >= 0.0);
        }
    }
}
