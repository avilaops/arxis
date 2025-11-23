//! Scientific computing module with advanced operations

pub mod astronomy;
pub mod complex;
pub mod fft;
pub mod fft_native;
pub mod fft_pure;
pub mod signal;
pub mod spectrogram;
pub mod stats;
pub mod wavelets;

pub use complex::Complex;
pub use fft::WindowType;
pub use fft_native::{
    fft as fft_native, find_peak, frequency_vector, power_spectral_density,
    WindowType as WindowTypeNative,
};
pub use fft_pure::{
    convolve_fft, fft_cooley_tukey, fft_frequencies, ifft, power_spectral_density as psd_pure,
    rfft, xcorr_fft,
};
pub use signal::{FilterType, ResampleMethod, RollingWindow};
pub use spectrogram::{
    log_spectrogram, mel_spectrogram, power_spectrogram, stft, WindowType as WindowTypeSpec,
};
pub use wavelets::WaveletType;
