use avila_dataframe::prelude::*;
use avila_dataframe::scientific::fft::{fft, power_spectral_density, WindowType};

fn main() -> Result<()> {
    println!("=== AvilaDF FFT Example ===\n");

    // Generate sample gravitational wave signal
    let sample_rate = 4096.0; // Hz
    let duration = 1.0; // seconds
    let n_samples = (sample_rate * duration) as usize;

    let mut signal = Vec::with_capacity(n_samples);
    for i in 0..n_samples {
        let t = i as f64 / sample_rate;

        // Simulate gravitational wave chirp (simplified)
        let f0 = 35.0; // Hz
        let chirp_rate = 100.0;
        let freq = f0 + chirp_rate * t;

        let amplitude = 1e-21 * (1.0 + 2.0 * t); // Increasing amplitude
        let phase = 2.0 * std::f64::consts::PI * freq * t;

        signal.push(amplitude * phase.sin());
    }

    println!("Generated {} samples at {} Hz", n_samples, sample_rate);
    println!("Signal duration: {:.2} seconds\n", duration);

    // Apply FFT with Hann window
    println!("Applying FFT with Hann window...");
    let spectrum = fft(&signal, Some(WindowType::Hann))?;

    println!("FFT output length: {}", spectrum.len());
    println!("Nyquist frequency: {:.2} Hz\n", sample_rate / 2.0);

    // Calculate Power Spectral Density
    println!("Computing Power Spectral Density...");
    let psd = power_spectral_density(&signal, sample_rate, Some(WindowType::Hann))?;

    // Find peak frequency
    let mut max_power = 0.0;
    let mut peak_idx = 0;

    for i in 0..psd.len() {
        if psd[i] > max_power {
            max_power = psd[i];
            peak_idx = i;
        }
    }

    let freq_resolution = sample_rate / n_samples as f64;
    let peak_freq = peak_idx as f64 * freq_resolution;

    println!("Peak frequency: {:.2} Hz", peak_freq);
    println!("Peak power: {:.2e}\n", max_power);

    // Create DataFrame with results
    let frequencies: Vec<f64> = (0..256).map(|i| i as f64 * freq_resolution).collect();

    let powers: Vec<f64> = psd.iter().take(256).copied().collect();

    let df = DataFrame::new(vec![
        Series::new("frequency_hz", frequencies),
        Series::new("power", powers),
    ])?;

    println!("Power Spectrum (first 256 bins):");
    println!("{}", df);

    // Show statistics
    let stats = df.describe()?;
    println!("\nStatistics:");
    println!("{}", stats);

    println!("\n✅ FFT analysis complete!");
    println!("🚀 AvilaDF: Built for gravitational wave astronomy");

    Ok(())
}
