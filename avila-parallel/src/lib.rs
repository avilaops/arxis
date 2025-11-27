//! # avila-parallel
//!
//! Data parallelism library for AVL Platform.
//!
//! ## Features
//!
//! - Thread pool with work stealing
//! - Parallel iterators
//! - Scoped threads
//! - Zero overhead abstractions
//!
//! ## Quick Start
//!
//! ```rust
//! use avila_parallel::prelude::*;
//!
//! let numbers: Vec<i32> = (0..1000).collect();
//! let sum: i32 = numbers.par_iter().sum();
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]

pub mod thread_pool;
pub mod parallel;
pub mod scope;

pub use thread_pool::ThreadPool;
pub use parallel::{ParallelIterator, IntoParallelIterator};

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::parallel::{ParallelIterator, IntoParallelIterator};
    pub use crate::scope::scope;
}

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Create a scope for spawning threads
pub fn scope<'env, F, R>(f: F) -> R
where
    F: FnOnce(&scope::Scope<'env>) -> R,
{
    scope::scope(f)
}

#[cfg(test)]
mod tests {
    use super::*;
    use prelude::*;

    #[test]
    fn test_parallel_sum() {
        let numbers: Vec<i32> = (0..100).collect();
        let sum: i32 = numbers.par_iter().sum();
        assert_eq!(sum, 4950);
    }

    #[test]
    fn test_parallel_map() {
        let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
        let doubled: Vec<i32> = numbers
            .par_iter()
            .map(|&x| x * 2)
            .collect();
        assert_eq!(doubled, vec![2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_thread_pool() {
        let pool = ThreadPool::new(2);
        let mut results = Vec::new();

        for i in 0..10 {
            results.push(pool.execute(move || i * i));
        }

        pool.wait();
    }
}
