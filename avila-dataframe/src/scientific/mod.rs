//! Scientific computing module with advanced operations

pub mod astronomy;
pub mod fft;
pub mod fft_native;
pub mod signal;
pub mod stats;
pub mod wavelets;

pub use fft::WindowType;
pub use fft_native::{
    fft as fft_native, find_peak, frequency_vector, power_spectral_density,
    WindowType as WindowTypeNative,
};
pub use signal::{FilterType, ResampleMethod, RollingWindow};
pub use wavelets::WaveletType;
