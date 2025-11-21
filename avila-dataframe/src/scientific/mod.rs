//! Scientific computing module with advanced operations

pub mod fft;
pub mod fft_native;
pub mod wavelets;
pub mod signal;
pub mod stats;
pub mod astronomy;

pub use fft::{WindowType};
pub use fft_native::{
    fft as fft_native,
    power_spectral_density,
    frequency_vector,
    find_peak,
    WindowType as WindowTypeNative,
};
pub use wavelets::{WaveletType};
pub use signal::{ResampleMethod, FilterType, RollingWindow};
