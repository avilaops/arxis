//! Exemplo FFT nativo - 100% Rust
//!
//! Demonstra análise espectral de ondas gravitacionais usando FFT nativo

use avila_dataframe::prelude::*;
use avila_dataframe::scientific::fft_native::{
    fft as fft_native, find_peak, frequency_vector, power_spectral_density,
    WindowTypeNative as WindowType,
};
use std::f64::consts::PI;

fn main() -> Result<()> {
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║   AvilaDF - FFT Nativo (Fast Fourier Transform)        ║");
    println!("║   100% Rust • Zero Arrow • Máxima Performance 🚀        ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");

    // ========================================
    // 1. Gerar sinal de onda gravitacional simulado
    // ========================================
    println!("🌌 Gerando sinal de onda gravitacional simulada...\n");

    let sample_rate = 4096.0; // Hz (taxa LIGO)
    let duration = 1.0; // segundos
    let n_samples = (sample_rate * duration) as usize;

    println!("   Parâmetros:");
    println!("   • Taxa de amostragem: {} Hz", sample_rate);
    println!("   • Duração: {:.2} s", duration);
    println!("   • Amostras: {}", n_samples);
    println!("   • Frequência Nyquist: {:.1} Hz\n", sample_rate / 2.0);

    // Gerar chirp (onda gravitacional simplificada)
    let mut signal = Vec::with_capacity(n_samples);
    let f0 = 35.0; // Hz inicial
    let chirp_rate = 100.0; // Hz/s

    for i in 0..n_samples {
        let t = i as f64 / sample_rate;

        // Frequência crescente (chirp)
        let freq = f0 + chirp_rate * t;

        // Amplitude crescente (fusão se aproximando)
        let amplitude = 1e-21 * (1.0 + 2.0 * t);

        // Fase acumulada
        let phase = 2.0 * PI * (f0 * t + 0.5 * chirp_rate * t * t);

        signal.push(amplitude * phase.sin());
    }

    println!("✅ Sinal gerado: {} pontos", signal.len());
    println!("   • Frequência inicial: {} Hz", f0);
    println!(
        "   • Frequência final: {:.1} Hz\n",
        f0 + chirp_rate * duration
    );

    // ========================================
    // 2. Calcular FFT com diferentes janelas
    // ========================================
    println!("📊 Calculando FFT com janelas diferentes...\n");

    let windows = vec![
        ("Sem janela", None),
        ("Hann", Some(WindowType::Hann)),
        ("Hamming", Some(WindowType::Hamming)),
        ("Blackman", Some(WindowType::Blackman)),
    ];

    for (name, window) in &windows {
        let spectrum = fft_native(&signal, *window)?;
        let (peak_idx, peak_freq, peak_mag) = find_peak(&spectrum, sample_rate, n_samples);

        println!("   {} Window:", name);
        println!("   • Bins espectrais: {}", spectrum.len());
        println!("   • Pico em: {:.2} Hz (bin {})", peak_freq, peak_idx);
        println!("   • Magnitude: {:.2e}\n", peak_mag);
    }

    // ========================================
    // 3. Calcular Power Spectral Density
    // ========================================
    println!("⚡ Calculando Power Spectral Density (PSD)...\n");

    let psd = power_spectral_density(&signal, sample_rate, Some(WindowType::Hann))?;
    let (peak_idx, peak_freq, peak_power) = find_peak(&psd, sample_rate, n_samples);

    println!("   PSD Statistics:");
    println!("   • Bins: {}", psd.len());
    println!(
        "   • Pico de potência: {:.2e} @ {:.2} Hz",
        peak_power, peak_freq
    );
    println!(
        "   • Resolução: {:.3} Hz/bin\n",
        sample_rate / n_samples as f64
    );

    // ========================================
    // 4. Criar DataFrame com resultados
    // ========================================
    println!("📋 Criando DataFrame com espectro...\n");

    let frequencies = frequency_vector(n_samples, sample_rate);
    let spectrum_final = fft_native(&signal, Some(WindowType::Hann))?;

    // Pegar apenas primeiros 256 bins (até ~256 Hz)
    let n_bins = 256.min(frequencies.len());
    let freq_subset: Vec<f64> = frequencies.iter().take(n_bins).copied().collect();
    let spec_subset: Vec<f64> = spectrum_final.iter().take(n_bins).copied().collect();
    let psd_subset: Vec<f64> = psd.iter().take(n_bins).copied().collect();

    let df = DataFrame::from_series(vec![
        Series::new_float("frequency_hz", freq_subset),
        Series::new_float("magnitude", spec_subset),
        Series::new_float("power_density", psd_subset),
    ])?;

    println!("   Espectro de Frequências (primeiros 256 bins):");
    println!("{}\n", df.head(10)?);

    // ========================================
    // 5. Estatísticas do espectro
    // ========================================
    println!("📈 Estatísticas do Espectro:\n");
    let stats = df.describe()?;
    println!("{}\n", stats);

    // ========================================
    // 6. Filtrar frequências de interesse (30-150 Hz - banda LIGO)
    // ========================================
    println!("🔍 Filtrando banda de interesse (30-150 Hz)...\n");

    let filtered_df = df.filter(|row| {
        if let Some(freq) = row.get("frequency_hz") {
            match freq {
                Value::Float(f) => *f >= 30.0 && *f <= 150.0,
                _ => false,
            }
        } else {
            false
        }
    })?;

    println!("   Banda filtrada:");
    println!("{}\n", filtered_df.head(10)?);
    println!("   Total de bins na banda: {}", filtered_df.shape().0);

    // Encontrar pico na banda filtrada
    let mut max_power_filtered = 0.0;
    let mut peak_freq_filtered = 0.0;

    for row in filtered_df.rows() {
        if let (Some(Value::Float(freq)), Some(Value::Float(power))) =
            (row.get("frequency_hz"), row.get("power_density"))
        {
            if *power > max_power_filtered {
                max_power_filtered = *power;
                peak_freq_filtered = *freq;
            }
        }
    }

    println!("\n   🎯 Pico detectado na banda:");
    println!("   • Frequência: {:.2} Hz", peak_freq_filtered);
    println!("   • Potência: {:.2e}", max_power_filtered);

    // ========================================
    // 7. Resumo final
    // ========================================
    println!("\n╔══════════════════════════════════════════════════════════╗");
    println!("║                   RESUMO DA ANÁLISE                     ║");
    println!("╠══════════════════════════════════════════════════════════╣");
    println!("║  Sinal:          Onda gravitacional (chirp)            ║");
    println!(
        "║  Amostras:       {}                                  ║",
        n_samples
    );
    println!(
        "║  Taxa:           {} Hz                                ║",
        sample_rate as usize
    );
    println!(
        "║  FFT bins:       {}                                   ║",
        spectrum_final.len()
    );
    println!(
        "║  Pico global:    {:.1} Hz                             ║",
        peak_freq
    );
    println!(
        "║  Pico na banda:  {:.1} Hz (30-150 Hz)                ║",
        peak_freq_filtered
    );
    println!("╚══════════════════════════════════════════════════════════╝\n");

    println!("✅ Análise FFT completa!");
    println!("🚀 AvilaDF: 100% Rust nativo • Zero overhead • Brasil 🇧🇷\n");

    Ok(())
}
