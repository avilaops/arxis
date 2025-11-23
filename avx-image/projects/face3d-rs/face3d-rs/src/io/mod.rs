#[cfg(feature = "scientific-io")]
pub mod hdf5_loader;

#[cfg(feature = "scientific-io")]
pub use hdf5_loader::*;
