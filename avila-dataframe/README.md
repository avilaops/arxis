## 🔥 DESAFIO ACEITO - AvilaDF 100% Código Próprio

Você está absolutamente certo! Vou criar as instruções completas para implementar FFT DO ZERO em Rust puro.

## 📋 Instruções para Copilot: FFT 100% Próprio

### **MISSÃO: Implementar FFT Cooley-Tukey Radix-2 do Zero**

```markdown
# Contexto
Estamos construindo uma biblioteca DataFrame 100% Rust nativa chamada AvilaDF.
ZERO dependências externas para algoritmos científicos.
Foco: Performance, clareza e código educacional de qualidade produção.

# Tarefa Principal
Implementar Fast Fourier Transform (FFT) usando algoritmo Cooley-Tukey radix-2
INTEIRAMENTE do zero, sem usar rustfft, numpy, ou qualquer biblioteca externa.

## Requisitos Técnicos

### 1. Estrutura de Números Complexos Própria
Criar em `src/scientific/complex.rs`:
- Struct `Complex<T>` com campos `re` e `im`
- Implementar: Add, Sub, Mul, Div para Complex
- Métodos: magnitude(), phase(), conj(), exp()
- Suporte genérico para f32 e f64
- Traits: Clone, Copy, Debug, Default

### 2. Implementação FFT Core
Criar em `src/scientific/fft_pure.rs`:

**Algoritmos a implementar:**

a) **Cooley-Tukey Radix-2 DFT (Decimation in Time)**
   - Função: `fft_radix2(input: &[Complex<f64>]) -> Vec<Complex<f64>>`
   - Recursivo ou iterativo bit-reversal
   - Complexidade: O(N log N)
   - Validar que N é potência de 2
   - Se não for, fazer zero-padding automático

b) **Bit-Reversal Permutation**
   - Função auxiliar: `bit_reverse_copy(data: &mut [Complex<f64>])`
   - Necessário para FFT iterativa eficiente

c) **Twiddle Factors (Fatores de Rotação)**
   - Pre-calcular: `W_N^k = e^(-2πi k/N)`
   - Cache para reutilização
   - Função: `compute_twiddle_factors(n: usize) -> Vec<Complex<f64>>`

d) **FFT Inversa (IFFT)**
   - Função: `ifft_radix2(input: &[Complex<f64>]) -> Vec<Complex<f64>>`
   - Usar conjugado + FFT + conjugado + normalização
   - Dividir resultado por N

e) **FFT para Sinais Reais (otimizado)**
   - Função: `rfft(signal: &[f64]) -> Vec<Complex<f64>>`
   - Retornar apenas frequências positivas (N/2 + 1)
   - Função: `irfft(spectrum: &[Complex<f64>], n: usize) -> Vec<f64>`

### 3. Window Functions (Janelas)
Manter implementação atual em `fft_native.rs` mas renomear arquivo:
- Hann, Hamming, Blackman, Kaiser (adicionar)
- Todas com fórmulas matemáticas explícitas
- Sem dependências externas

### 4. Funções de Análise Espectral
```rust
// Power Spectral Density
pub fn psd(signal: &[f64], sample_rate: f64, window: WindowType) -> Vec<f64>

// Spectrogram (STFT)
pub fn spectrogram(
    signal: &[f64],
    window_size: usize,
    hop_size: usize,
    window: WindowType
) -> (Vec<Vec<f64>>, Vec<f64>, Vec<f64>) // (spectro, freqs, times)

// Cross-correlation via FFT
pub fn xcorr_fft(signal1: &[f64], signal2: &[f64]) -> Vec<f64>

// Convolution via FFT
pub fn convolve_fft(signal1: &[f64], signal2: &[f64]) -> Vec<f64>
```

### 5. Integrações com DataFrame
```rust
// Adicionar em series_native.rs
impl Series {
    pub fn fft(&self) -> Result<Vec<Complex<f64>>>
    pub fn ifft(&self, spectrum: &[Complex<f64>]) -> Result<Self>
    pub fn power_spectrum(&self, sample_rate: f64) -> Result<Self>
}

// Adicionar em dataframe_native.rs
impl DataFrame {
    pub fn fft_column(&self, column: &str) -> Result<Self>
    pub fn spectrogram_column(
        &self,
        column: &str,
        window_size: usize,
        hop_size: usize
    ) -> Result<Self>
}
```

### 6. Performance Otimizations
- Usar iteradores Rust nativos
- Inline crítico: #[inline(always)] em operações Complex
- SIMD onde possível (std::arch) - opcional
- Cache de twiddle factors com lazy_static OU once_cell
- Paralelização com rayon para múltiplas FFTs

### 7. Testes Completos
```rust
#[cfg(test)]
mod tests {
    // Testar contra valores conhecidos (DFT naive)
    // Testar identidade: ifft(fft(x)) == x
    // Testar Parseval's theorem (conservação energia)
    // Testar linearidade
    // Benchmark vs rustfft para validação
}
```

## Estrutura de Arquivos

```
src/scientific/
├── complex.rs          # Números complexos próprios
├── fft_pure.rs         # FFT 100% próprio (novo)
├── fft_native.rs       # Wrapper e utilities (refatorar)
├── spectrogram.rs      # STFT e análise tempo-freq (novo)
├── windows.rs          # Janelas separadas (extrair)
└── mod.rs              # Re-exports
```

## Algoritmo FFT - Pseudocódigo de Referência

```rust
// Cooley-Tukey Radix-2 DIT (Decimation in Time)
fn fft_recursive(x: &[Complex<f64>]) -> Vec<Complex<f64>> {
    let n = x.len();

    if n == 1 {
        return x.to_vec();
    }

    // Dividir em pares e ímpares
    let even: Vec<_> = x.iter().step_by(2).copied().collect();
    let odd: Vec<_> = x.iter().skip(1).step_by(2).copied().collect();

    // Recursão
    let fft_even = fft_recursive(&even);
    let fft_odd = fft_recursive(&odd);

    // Combinar
    let mut result = vec![Complex::default(); n];
    for k in 0..(n/2) {
        let t = Complex::from_polar(1.0, -2.0 * PI * k as f64 / n as f64) * fft_odd[k];
        result[k] = fft_even[k] + t;
        result[k + n/2] = fft_even[k] - t;
    }

    result
}
```

## Critérios de Sucesso
✅ Passar em todos os testes unitários
✅ Performance dentro de 2x de rustfft (aceitável)
✅ Código limpo, bem documentado
✅ Zero dependências externas (exceto std)
✅ Exemplos funcionais atualizados

## Documentação
- Comentários explicando matemática
- Docstrings com LaTeX para equações
- README.md explicando algoritmo
- Comparação de performance vs outras libs
```

---

## 🎯 Quer que eu IMPLEMENTE ISSO AGORA?

Posso criar:
1. ✅ `src/scientific/complex.rs` - Números complexos do zero
2. ✅ `src/scientific/fft_pure.rs` - FFT Cooley-Tukey completo
3. ✅ `src/scientific/spectrogram.rs` - STFT do zero
4. ✅ Integração com DataFrame/Series
5. ✅ Testes completos
6. ✅ Benchmarks vs rustfft

Começo agora? 🚀---

## 🎯 Quer que eu IMPLEMENTE ISSO AGORA?

Posso criar:
1. ✅ `src/scientific/complex.rs` - Números complexos do zero
2. ✅ `src/scientific/fft_pure.rs` - FFT Cooley-Tukey completo
3. ✅ `src/scientific/spectrogram.rs` - STFT do zero
4. ✅ Integração com DataFrame/Series
5. ✅ Testes completos
6. ✅ Benchmarks vs rustfft

Começo agora? 🚀
