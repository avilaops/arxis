use super::complex::Complex;
use std::f64::consts::PI;

/// Calcula os twiddle factors (fatores de rotação) para FFT
/// W_N^k = e^(-2πi k/N) = cos(-2πk/N) + i*sin(-2πk/N)
fn compute_twiddle_factors(n: usize) -> Vec<Complex<f64>> {
    let mut twiddles = Vec::with_capacity(n / 2);
    for k in 0..(n / 2) {
        let angle = -2.0 * PI * k as f64 / n as f64;
        twiddles.push(Complex::from_polar(1.0, angle));
    }
    twiddles
}

/// Calcula o índice bit-reversed
/// Exemplo: para n=8 (log_n=3), i=6 (110b) -> 3 (011b)
fn bit_reverse_index(mut i: usize, log_n: u32) -> usize {
    let mut result = 0;
    for _ in 0..log_n {
        result = (result << 1) | (i & 1);
        i >>= 1;
    }
    result
}

/// Aplica bit-reversal permutation in-place
fn bit_reverse_copy(data: &mut [Complex<f64>]) {
    let n = data.len();
    let log_n = (n as f64).log2() as u32;

    for i in 0..n {
        let j = bit_reverse_index(i, log_n);
        if i < j {
            data.swap(i, j);
        }
    }
}

/// Verifica se um número é potência de 2
#[inline]
fn is_power_of_two(n: usize) -> bool {
    n > 0 && (n & (n - 1)) == 0
}

/// Calcula o próximo número que é potência de 2
#[inline]
fn next_power_of_two(n: usize) -> usize {
    if n == 0 {
        return 1;
    }
    let mut power = 1;
    while power < n {
        power <<= 1;
    }
    power
}

/// FFT Cooley-Tukey Radix-2 (Decimation in Time) - Versão Iterativa
///
/// Algoritmo:
/// 1. Bit-reversal permutation
/// 2. Iteração sobre estágios (log₂N)
/// 3. Para cada estágio, aplicar butterflies
///
/// Complexidade: O(N log N)
pub fn fft_cooley_tukey(input: &[Complex<f64>]) -> Vec<Complex<f64>> {
    let n = input.len();

    if n == 0 {
        return vec![];
    }

    if n == 1 {
        return input.to_vec();
    }

    // Validar e aplicar zero-padding se necessário
    let padded_n = if is_power_of_two(n) {
        n
    } else {
        next_power_of_two(n)
    };

    let mut data = if padded_n == n {
        input.to_vec()
    } else {
        let mut padded = input.to_vec();
        padded.resize(padded_n, Complex::zero());
        padded
    };

    let n = padded_n;
    let log_n = (n as f64).log2() as u32;

    // Bit-reversal permutation
    bit_reverse_copy(&mut data);

    // FFT iterativa
    for s in 1..=log_n {
        let m = 1 << s; // 2^s
        let half_m = m >> 1; // m/2

        // Twiddle factor para este estágio
        let angle = -2.0 * PI / m as f64;
        let wm = Complex::from_polar(1.0, angle);

        for k in (0..n).step_by(m) {
            let mut w = Complex::one();

            for j in 0..half_m {
                let t = w * data[k + j + half_m];
                let u = data[k + j];

                data[k + j] = u + t;
                data[k + j + half_m] = u - t;

                w *= wm;
            }
        }
    }

    data
}

/// FFT Inversa (IFFT)
///
/// Algoritmo:
/// 1. Conjugar entrada
/// 2. Aplicar FFT
/// 3. Conjugar resultado
/// 4. Dividir por N
pub fn ifft(input: &[Complex<f64>]) -> Vec<Complex<f64>> {
    let n = input.len();

    if n == 0 {
        return vec![];
    }

    if n == 1 {
        return input.to_vec();
    }

    // 1. Conjugar
    let conjugated: Vec<Complex<f64>> = input.iter().map(|z| z.conj()).collect();

    // 2. FFT
    let mut result = fft_cooley_tukey(&conjugated);

    // 3. Conjugar resultado e normalizar
    let n_inv = 1.0 / n as f64;
    for z in result.iter_mut() {
        *z = z.conj() * n_inv;
    }

    result
}

/// FFT para sinais reais (otimizado)
/// Retorna apenas as frequências positivas (0 a Nyquist)
/// Tamanho da saída: N/2 + 1
pub fn rfft(signal: &[f64]) -> Vec<Complex<f64>> {
    // Converter para complex
    let complex_input: Vec<Complex<f64>> = signal
        .iter()
        .map(|&x| Complex::new(x, 0.0))
        .collect();

    // FFT completa
    let full_fft = fft_cooley_tukey(&complex_input);

    // Retornar apenas metade + bin DC e Nyquist
    let n = full_fft.len();
    full_fft[0..=(n / 2)].to_vec()
}

/// IFFT para sinais reais
/// Reconstrói a metade negativa do espectro por simetria
pub fn irfft(spectrum: &[Complex<f64>], n: usize) -> Vec<f64> {
    if spectrum.is_empty() {
        return vec![];
    }

    // Reconstruir espectro completo usando simetria hermitiana
    let mut full_spectrum = vec![Complex::zero(); n];

    // Copiar frequências positivas
    let len = spectrum.len().min(n / 2 + 1);
    full_spectrum[0..len].copy_from_slice(&spectrum[0..len]);

    // Simetria: X[k] = conj(X[N-k]) para k > 0
    for k in 1..(n / 2) {
        full_spectrum[n - k] = full_spectrum[k].conj();
    }

    // IFFT
    let result = ifft(&full_spectrum);

    // Extrair parte real
    result.iter().map(|z| z.re).collect()
}

/// DFT naive (O(N²)) - útil para testes e validação
pub fn dft_naive(input: &[Complex<f64>]) -> Vec<Complex<f64>> {
    let n = input.len();
    let mut output = vec![Complex::zero(); n];

    for k in 0..n {
        let mut sum = Complex::zero();
        for (t, &x_t) in input.iter().enumerate() {
            let angle = -2.0 * PI * k as f64 * t as f64 / n as f64;
            let twiddle = Complex::from_polar(1.0, angle);
            sum += x_t * twiddle;
        }
        output[k] = sum;
    }

    output
}

/// Convolução usando FFT (método rápido)
/// conv(x, y) = IFFT(FFT(x) * FFT(y))
pub fn convolve_fft(signal1: &[f64], signal2: &[f64]) -> Vec<f64> {
    if signal1.is_empty() || signal2.is_empty() {
        return vec![];
    }

    // Tamanho da convolução
    let n = signal1.len() + signal2.len() - 1;
    let padded_n = next_power_of_two(n);

    // Padding
    let mut padded1 = signal1.to_vec();
    let mut padded2 = signal2.to_vec();
    padded1.resize(padded_n, 0.0);
    padded2.resize(padded_n, 0.0);

    // FFT de ambos
    let fft1 = rfft(&padded1);
    let fft2 = rfft(&padded2);

    // Multiplicação no domínio da frequência
    let mut product: Vec<Complex<f64>> = fft1
        .iter()
        .zip(fft2.iter())
        .map(|(a, b)| *a * *b)
        .collect();

    // IFFT
    let result = irfft(&product, padded_n);

    // Retornar apenas os n primeiros elementos
    result[0..n].to_vec()
}

/// Correlação cruzada usando FFT
/// xcorr(x, y) = IFFT(FFT(x) * conj(FFT(y)))
pub fn xcorr_fft(signal1: &[f64], signal2: &[f64]) -> Vec<f64> {
    if signal1.is_empty() || signal2.is_empty() {
        return vec![];
    }

    let n = signal1.len() + signal2.len() - 1;
    let padded_n = next_power_of_two(n);

    // Padding
    let mut padded1 = signal1.to_vec();
    let mut padded2 = signal2.to_vec();
    padded1.resize(padded_n, 0.0);
    padded2.resize(padded_n, 0.0);

    // FFT de ambos
    let fft1 = rfft(&padded1);
    let fft2 = rfft(&padded2);

    // Multiplicação com conjugado
    let mut product: Vec<Complex<f64>> = fft1
        .iter()
        .zip(fft2.iter())
        .map(|(a, b)| *a * b.conj())
        .collect();

    // IFFT
    let result = irfft(&product, padded_n);

    result[0..n].to_vec()
}

/// Calcula Power Spectral Density (PSD)
pub fn power_spectral_density(signal: &[f64], sample_rate: f64) -> Vec<f64> {
    let spectrum = rfft(signal);
    let n = signal.len();

    spectrum
        .iter()
        .map(|z| {
            let mag_sq = z.magnitude_squared();
            // Normalização: 2/N² para bins intermediários, 1/N² para DC e Nyquist
            mag_sq / (n * n) as f64 * 2.0
        })
        .collect()
}

/// Calcula vetor de frequências para FFT
pub fn fft_frequencies(n: usize, sample_rate: f64) -> Vec<f64> {
    let freq_bin = sample_rate / n as f64;
    (0..=(n / 2))
        .map(|k| k as f64 * freq_bin)
        .collect()
}

// ============================================================================
// TESTES
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-10;

    #[test]
    fn test_is_power_of_two() {
        assert!(is_power_of_two(1));
        assert!(is_power_of_two(2));
        assert!(is_power_of_two(4));
        assert!(is_power_of_two(8));
        assert!(is_power_of_two(1024));
        assert!(!is_power_of_two(0));
        assert!(!is_power_of_two(3));
        assert!(!is_power_of_two(5));
        assert!(!is_power_of_two(100));
    }

    #[test]
    fn test_next_power_of_two() {
        assert_eq!(next_power_of_two(0), 1);
        assert_eq!(next_power_of_two(1), 1);
        assert_eq!(next_power_of_two(2), 2);
        assert_eq!(next_power_of_two(3), 4);
        assert_eq!(next_power_of_two(5), 8);
        assert_eq!(next_power_of_two(100), 128);
    }

    #[test]
    fn test_bit_reverse_index() {
        // Para n=8 (log_n=3)
        assert_eq!(bit_reverse_index(0, 3), 0); // 000 -> 000
        assert_eq!(bit_reverse_index(1, 3), 4); // 001 -> 100
        assert_eq!(bit_reverse_index(2, 3), 2); // 010 -> 010
        assert_eq!(bit_reverse_index(3, 3), 6); // 011 -> 110
        assert_eq!(bit_reverse_index(4, 3), 1); // 100 -> 001
        assert_eq!(bit_reverse_index(5, 3), 5); // 101 -> 101
        assert_eq!(bit_reverse_index(6, 3), 3); // 110 -> 011
        assert_eq!(bit_reverse_index(7, 3), 7); // 111 -> 111
    }

    #[test]
    fn test_complex_arithmetic() {
        let z1 = Complex::new(1.0, 2.0);
        let z2 = Complex::new(3.0, 4.0);

        let sum = z1 + z2;
        assert_eq!(sum.re, 4.0);
        assert_eq!(sum.im, 6.0);

        let product = z1 * z2;
        assert_eq!(product.re, -5.0);
        assert_eq!(product.im, 10.0);
    }

    #[test]
    fn test_fft_impulse() {
        // Impulso: [1, 0, 0, 0]
        // FFT de impulso = [1, 1, 1, 1]
        let input = vec![
            Complex::new(1.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
        ];

        let result = fft_cooley_tukey(&input);

        for z in &result {
            assert!((z.re - 1.0).abs() < EPSILON);
            assert!(z.im.abs() < EPSILON);
        }
    }

    #[test]
    fn test_fft_sine_wave() {
        // Sinal: cos(2πk/N) para k=0..7
        let n = 8;
        let input: Vec<Complex<f64>> = (0..n)
            .map(|k| {
                let val = (2.0 * PI * k as f64 / n as f64).cos();
                Complex::new(val, 0.0)
            })
            .collect();

        let result = fft_cooley_tukey(&input);

        // Para coseno puro, esperamos picos em k=1 e k=N-1
        assert!(result[1].magnitude() > 3.0); // Pico
        assert!(result[7].magnitude() > 3.0); // Pico simétrico
        assert!(result[0].magnitude() < 0.1); // DC próximo de zero
        assert!(result[2].magnitude() < 0.1); // Outros bins pequenos
    }

    #[test]
    fn test_fft_ifft_identity() {
        // Testar que IFFT(FFT(x)) = x
        let input = vec![
            Complex::new(1.0, 0.0),
            Complex::new(2.0, 0.0),
            Complex::new(3.0, 0.0),
            Complex::new(4.0, 0.0),
        ];

        let fft_result = fft_cooley_tukey(&input);
        let ifft_result = ifft(&fft_result);

        for (original, recovered) in input.iter().zip(ifft_result.iter()) {
            assert!((original.re - recovered.re).abs() < EPSILON);
            assert!((original.im - recovered.im).abs() < EPSILON);
        }
    }

    #[test]
    fn test_parsevals_theorem() {
        // Teorema de Parseval: energia no tempo = energia na frequência
        // sum(|x[n]|²) = (1/N) * sum(|X[k]|²)
        let input = vec![
            Complex::new(1.0, 0.0),
            Complex::new(2.0, 0.0),
            Complex::new(3.0, 0.0),
            Complex::new(4.0, 0.0),
        ];

        let time_energy: f64 = input.iter().map(|z| z.magnitude_squared()).sum();

        let fft_result = fft_cooley_tukey(&input);
        let freq_energy: f64 = fft_result.iter().map(|z| z.magnitude_squared()).sum();
        let freq_energy_normalized = freq_energy / input.len() as f64;

        assert!((time_energy - freq_energy_normalized).abs() < EPSILON);
    }

    #[test]
    fn test_rfft_symmetry() {
        let signal = vec![1.0, 2.0, 3.0, 4.0, 3.0, 2.0, 1.0, 0.0];
        let spectrum = rfft(&signal);

        // Espectro de sinal real deve ter simetria hermitiana
        // Verificar apenas o tamanho correto
        assert_eq!(spectrum.len(), signal.len() / 2 + 1);
    }

    #[test]
    fn test_rfft_irfft_identity() {
        let signal = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let spectrum = rfft(&signal);
        let recovered = irfft(&spectrum, signal.len());

        for (original, recovered) in signal.iter().zip(recovered.iter()) {
            assert!((original - recovered).abs() < EPSILON);
        }
    }

    #[test]
    fn test_convolve_fft() {
        let signal1 = vec![1.0, 2.0, 3.0];
        let signal2 = vec![1.0, 1.0];

        // Convolução esperada: [1, 3, 5, 3]
        let result = convolve_fft(&signal1, &signal2);

        assert_eq!(result.len(), 4);
        assert!((result[0] - 1.0).abs() < EPSILON);
        assert!((result[1] - 3.0).abs() < EPSILON);
        assert!((result[2] - 5.0).abs() < EPSILON);
        assert!((result[3] - 3.0).abs() < EPSILON);
    }

    #[test]
    fn test_dft_naive_vs_fft() {
        let input = vec![
            Complex::new(1.0, 0.0),
            Complex::new(2.0, 0.0),
            Complex::new(3.0, 0.0),
            Complex::new(4.0, 0.0),
        ];

        let dft_result = dft_naive(&input);
        let fft_result = fft_cooley_tukey(&input);

        for (dft_val, fft_val) in dft_result.iter().zip(fft_result.iter()) {
            assert!((dft_val.re - fft_val.re).abs() < EPSILON);
            assert!((dft_val.im - fft_val.im).abs() < EPSILON);
        }
    }

    #[test]
    fn test_fft_zero_padding() {
        // FFT deve fazer zero-padding automático para potência de 2
        let input = vec![
            Complex::new(1.0, 0.0),
            Complex::new(2.0, 0.0),
            Complex::new(3.0, 0.0),
        ];

        let result = fft_cooley_tukey(&input);
        assert_eq!(result.len(), 4); // Padded para 4 (próxima potência de 2)
    }

    #[test]
    fn test_fft_frequencies() {
        let n = 8;
        let sample_rate = 1000.0;
        let freqs = fft_frequencies(n, sample_rate);

        assert_eq!(freqs.len(), n / 2 + 1);
        assert_eq!(freqs[0], 0.0);
        assert_eq!(freqs[1], 125.0);
        assert_eq!(freqs[4], 500.0); // Nyquist
    }

    #[test]
    fn test_power_spectral_density() {
        let signal = vec![1.0, 0.0, -1.0, 0.0, 1.0, 0.0, -1.0, 0.0];
        let psd = power_spectral_density(&signal, 1.0);

        // Verificar que não há valores negativos
        for &p in &psd {
            assert!(p >= 0.0);
        }
    }
}
