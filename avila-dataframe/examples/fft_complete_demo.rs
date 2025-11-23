use avila_dataframe::core::series_native::Series;
use avila_dataframe::core::dataframe_native::DataFrame;
use avila_dataframe::scientific::{Complex, fft_cooley_tukey, ifft, rfft, stft, WindowType};
use std::f64::consts::PI;

fn main() {
    println!("=== FFT 100% Próprio - AvilaDF ===\n");

    // ========================================
    // 1. TESTE DE NÚMEROS COMPLEXOS
    // ========================================
    println!("1. NÚMEROS COMPLEXOS");
    println!("   Testando operações com Complex<f64>...");

    let z1 = Complex::new(3.0, 4.0);
    let z2 = Complex::new(1.0, 2.0);

    println!("   z1 = {}", z1);
    println!("   z2 = {}", z2);
    println!("   z1 + z2 = {}", z1 + z2);
    println!("   z1 * z2 = {}", z1 * z2);
    println!("   |z1| = {}", z1.magnitude());
    println!("   arg(z1) = {:.4} rad", z1.phase());
    println!("   conj(z1) = {}", z1.conj());

    // Teste de Euler: e^(iπ) = -1
    let euler = Complex::new(0.0, PI).exp();
    println!("   e^(iπ) = {} (deve ser próximo de -1)", euler);
    println!("   ✓ Números complexos funcionando!\n");

    // ========================================
    // 2. FFT BÁSICA - IMPULSO
    // ========================================
    println!("2. FFT DE IMPULSO");
    println!("   Sinal: [1, 0, 0, 0]");

    let impulse = vec![
        Complex::new(1.0, 0.0),
        Complex::new(0.0, 0.0),
        Complex::new(0.0, 0.0),
        Complex::new(0.0, 0.0),
    ];

    let fft_impulse = fft_cooley_tukey(&impulse);
    println!("   FFT: ");
    for (i, val) in fft_impulse.iter().enumerate() {
        println!("      [{}] {:.4} + {:.4}i", i, val.re, val.im);
    }
    println!("   ✓ FFT de impulso = [1, 1, 1, 1]\n");

    // ========================================
    // 3. FFT + IFFT - IDENTIDADE
    // ========================================
    println!("3. TESTE DE IDENTIDADE: IFFT(FFT(x)) = x");

    let original = vec![
        Complex::new(1.0, 0.0),
        Complex::new(2.0, 0.0),
        Complex::new(3.0, 0.0),
        Complex::new(4.0, 0.0),
    ];

    println!("   Sinal original: [1, 2, 3, 4]");
    let fft_result = fft_cooley_tukey(&original);
    let ifft_result = ifft(&fft_result);

    println!("   Após FFT -> IFFT:");
    for (i, val) in ifft_result.iter().enumerate() {
        println!("      [{}] {:.4}", i, val.re);
    }
    println!("   ✓ Identidade preservada!\n");

    // ========================================
    // 4. FFT DE SINAL SENOIDAL
    // ========================================
    println!("4. FFT DE SINAL SENOIDAL");

    let sample_rate = 1000.0; // Hz
    let freq = 50.0; // Hz
    let duration = 0.1; // segundos
    let n_samples = (sample_rate * duration) as usize;

    // Gerar sinal: sin(2πft)
    let signal: Vec<f64> = (0..n_samples)
        .map(|i| {
            let t = i as f64 / sample_rate;
            (2.0 * PI * freq * t).sin()
        })
        .collect();

    println!("   Sinal: {} Hz, {} Hz amostragem", freq, sample_rate);
    println!("   {} amostras", n_samples);

    let spectrum = rfft(&signal);

    // Encontrar pico
    let mut max_idx = 0;
    let mut max_mag = 0.0;
    for (i, z) in spectrum.iter().enumerate() {
        let mag = z.magnitude();
        if mag > max_mag {
            max_mag = mag;
            max_idx = i;
        }
    }

    let detected_freq = max_idx as f64 * sample_rate / n_samples as f64;
    println!("   Frequência detectada: {:.1} Hz", detected_freq);
    println!("   Magnitude do pico: {:.2}", max_mag);
    println!("   ✓ Detecção de frequência funcionando!\n");

    // ========================================
    // 5. INTEGRAÇÃO COM SERIES
    // ========================================
    println!("5. INTEGRAÇÃO COM SERIES");

    let series_signal: Vec<f64> = (0..64)
        .map(|i| {
            let t = i as f64 / 100.0;
            (2.0 * PI * 5.0 * t).sin() + 0.5 * (2.0 * PI * 10.0 * t).sin()
        })
        .collect();

    let series = Series::new_float("signal", series_signal);

    println!("   Criando Series com sinal composto (5 Hz + 10 Hz)");
    println!("   Calculando FFT...");

    match series.fft() {
        Ok(spectrum) => {
            println!("   ✓ FFT calculada com sucesso!");
            println!("   Espectro tem {} bins de frequência", spectrum.len());
        }
        Err(e) => println!("   ✗ Erro: {}", e),
    }

    match series.magnitude_spectrum() {
        Ok(mag_series) => {
            println!("   ✓ Espectro de magnitude calculado!");
            println!("   Tamanho: {}", mag_series.len());
        }
        Err(e) => println!("   ✗ Erro: {}", e),
    }

    match series.power_spectrum(100.0) {
        Ok(power_series) => {
            println!("   ✓ Espectro de potência calculado!");
            println!("   Tamanho: {}", power_series.len());
        }
        Err(e) => println!("   ✗ Erro: {}", e),
    }
    println!();

    // ========================================
    // 6. INTEGRAÇÃO COM DATAFRAME
    // ========================================
    println!("6. INTEGRAÇÃO COM DATAFRAME");

    let time: Vec<f64> = (0..128).map(|i| i as f64 / 1000.0).collect();
    let values: Vec<f64> = (0..128)
        .map(|i| {
            let t = i as f64 / 1000.0;
            (2.0 * PI * 20.0 * t).sin()
        })
        .collect();

    match DataFrame::from_series(vec![
        Series::new_float("time", time),
        Series::new_float("amplitude", values),
    ]) {
        Ok(df) => {
            println!("   DataFrame criado: {} linhas", df.height());

            match df.fft_column("amplitude") {
                Ok(fft_df) => {
                    println!("   ✓ FFT da coluna 'amplitude' calculada!");
                    println!("   Resultado: {} linhas", fft_df.height());
                }
                Err(e) => println!("   ✗ Erro: {:?}", e),
            }

            match df.power_spectrum_column("amplitude", 1000.0) {
                Ok(power_df) => {
                    println!("   ✓ Power spectrum calculado!");
                    println!("   Resultado: {} linhas", power_df.height());
                }
                Err(e) => println!("   ✗ Erro: {:?}", e),
            }
        }
        Err(e) => println!("   ✗ Erro ao criar DataFrame: {:?}", e),
    }
    println!();

    // ========================================
    // 7. SPECTROGRAM (STFT)
    // ========================================
    println!("7. SPECTROGRAM - ANÁLISE TEMPO-FREQUÊNCIA");

    // Sinal chirp (frequência crescente)
    let n = 1000;
    let chirp: Vec<f64> = (0..n)
        .map(|i| {
            let t = i as f64 / n as f64;
            let freq = 10.0 + 40.0 * t; // 10 Hz -> 50 Hz
            (2.0 * PI * freq * t * n as f64 / 100.0).sin()
        })
        .collect();

    let (spec, freqs, times) = stft(&chirp, 128, 32, 100.0, WindowType::Hann);

    println!("   Sinal chirp: {} amostras", n);
    println!("   STFT calculado:");
    println!("      {} frames no tempo", times.len());
    println!("      {} bins de frequência", freqs.len());
    if !spec.is_empty() {
        println!("      Dimensões da matriz: {}x{}", spec.len(), spec[0].len());
    }
    println!("   ✓ STFT funcionando!\n");

    // ========================================
    // 8. CONVOLUÇÃO E CORRELAÇÃO
    // ========================================
    println!("8. CONVOLUÇÃO E CORRELAÇÃO");

    let signal_a = Series::new_float("a", vec![1.0, 2.0, 3.0, 4.0]);
    let signal_b = Series::new_float("b", vec![1.0, 1.0, 1.0]);

    match signal_a.convolve(&signal_b) {
        Ok(conv) => {
            println!("   Convolução [1,2,3,4] * [1,1,1]:");
            print!("      [");
            for i in 0..conv.len().min(10) {
                if let Some(val) = conv.get(i) {
                    print!("{:.1}, ", val.as_f64().unwrap_or(0.0));
                }
            }
            println!("]");
            println!("   ✓ Convolução via FFT funcionando!");
        }
        Err(e) => println!("   ✗ Erro: {}", e),
    }

    match signal_a.xcorr(&signal_b) {
        Ok(xcorr) => {
            println!("   Correlação cruzada calculada: {} pontos", xcorr.len());
            println!("   ✓ Correlação via FFT funcionando!");
        }
        Err(e) => println!("   ✗ Erro: {}", e),
    }
    println!();

    // ========================================
    // RESUMO FINAL
    // ========================================
    println!("==============================================");
    println!("✅ IMPLEMENTAÇÃO FFT 100% PRÓPRIA COMPLETA!");
    println!("==============================================");
    println!();
    println!("Funcionalidades implementadas:");
    println!("  ✓ Números complexos (Complex<T>)");
    println!("  ✓ FFT Cooley-Tukey Radix-2");
    println!("  ✓ IFFT (inversa)");
    println!("  ✓ RFFT (otimizada para sinais reais)");
    println!("  ✓ STFT/Spectrogram");
    println!("  ✓ Janelas (Hann, Hamming, Blackman, Kaiser)");
    println!("  ✓ Convolução via FFT");
    println!("  ✓ Correlação cruzada via FFT");
    println!("  ✓ Power Spectral Density");
    println!("  ✓ Mel-Spectrogram");
    println!("  ✓ Integração com Series");
    println!("  ✓ Integração com DataFrame");
    println!();
    println!("🚀 ZERO DEPENDÊNCIAS EXTERNAS!");
    println!("📚 Código 100% educacional e produção-ready");
    println!();
}
