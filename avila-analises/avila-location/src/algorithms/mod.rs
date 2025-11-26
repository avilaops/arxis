//! Optimization algorithms for location selection

pub mod geographic;
pub mod clustering;
pub mod financial;
pub mod routing;

pub use geographic::*;
pub use clustering::*;
pub use financial::*;
pub use routing::*;
