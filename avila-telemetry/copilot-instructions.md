# Avila Telemetry - Copilot Instructions

**Projeto**: avila-telemetry
**Descrição**: Time Series Analysis, ARIMA Forecasting, Anomaly Detection for NASA-grade Observability
**Status**: Produção (22 testes passando)
**Filosofia**: Essência antes de escala. Profundidade antes de velocidade.

---

## 🎯 REGRAS CRÍTICAS - NUNCA VIOLAR

### 1. Zero External Dependencies (Exceto Fundamentais)
```toml
# ✅ PERMITIDO (fundamentos matemáticos)
ndarray = "0.15"      # Arrays N-dimensionais
statrs = "0.16"       # Estatística
rustfft = "6.1"       # FFT (opcional)
chrono = "0.4"        # Timestamps

# ❌ PROIBIDO (dependências externas)
- prometheus = "..."   # Implementar métricas próprias
- influxdb = "..."     # Implementar storage próprio
- opentelemetry = "..." # Implementar tracing próprio
```

**Motivo**: Missões científicas (LISA, LIGO) exigem controle total sobre algoritmos. Uma biblioteca externa errada pode invalidar anos de pesquisa.

### 2. Correção Matemática > Performance
```rust
// ❌ ERRADO: Rápido mas impreciso
fn mean(data: &[f64]) -> f64 {
    data.iter().sum::<f64>() / data.len() as f64
}

// ✅ CORRETO: Algoritmo numericamente estável (Welford)
fn mean(data: &[f64]) -> f64 {
    let mut m = 0.0;
    for (i, &x) in data.iter().enumerate() {
        m += (x - m) / (i + 1) as f64;
    }
    m
}
```

**Regra**: Todo algoritmo estatístico deve usar métodos numericamente estáveis (Welford, Kahan, Compensated summation).

### 3. Testes com Dados Reais NASA
```rust
#[test]
fn test_arima_nasa_gw_data() {
    // Dados reais de ondas gravitacionais (GW150914)
    let signal = load_ligo_h1_strain();
    let model = ARIMA::new(2, 1, 2);
    let forecast = model.fit_predict(&signal, 100);

    // Validar com paper original
    assert!((forecast.mse() - 0.023).abs() < 1e-3);
}
```

**Obrigatório**: Validar contra datasets científicos reais (LIGO, Kepler, NOAA).

### 4. Documentação Científica Completa
```rust
/// Detecta anomalias usando Z-score modificado (Iglewicz & Hoaglin, 1993)
///
/// # Algoritmo
/// ```text
/// MAD = median(|x_i - median(x)|)
/// M_i = 0.6745 * (x_i - median(x)) / MAD
/// anomaly if |M_i| > threshold
/// ```
///
/// # Referências
/// - Iglewicz, B., & Hoaglin, D. (1993). "How to Detect and Handle Outliers"
/// - NASA GCP Data Quality Guidelines (2018)
///
/// # Complexidade
/// - Tempo: O(n log n) (devido a mediana)
/// - Espaço: O(n)
pub fn detect_zscore_modified(&self, threshold: f64) -> Vec<usize> {
    // Implementação...
}
```

---

## 📐 Arquitetura do Projeto

```
avila-telemetry/
├── src/
│   ├── lib.rs                 # Exports públicos
│   ├── time_series.rs         # TimeSeries struct + operações
│   ├── statistics.rs          # Stats, correlação, distribuições
│   ├── anomaly.rs             # Detecção de anomalias
│   ├── forecasting/
│   │   ├── mod.rs
│   │   ├── arima.rs           # ARIMA(p,d,q)
│   │   ├── exponential.rs     # Simple/Double/Triple ES
│   │   └── prophet.rs         # (Futuro) Facebook Prophet
│   ├── transforms/
│   │   ├── mod.rs
│   │   ├── fft.rs             # FFT, IFFT, PSD
│   │   ├── wavelets.rs        # Haar, Daubechies, Morlet
│   │   └── filters.rs         # Butterworth, Chebyshev
│   ├── quality/
│   │   ├── mod.rs
│   │   ├── metrics.rs         # Accuracy, completeness, consistency
│   │   └── nasa_gcp.rs        # NASA Good Coding Practices
│   └── observability/
│       ├── mod.rs
│       ├── metrics.rs         # Counter, Gauge, Histogram
│       ├── tracing.rs         # Span, Event, Context
│       └── logging.rs         # Structured logs
├── benches/
│   └── time_series_bench.rs  # Criterion benchmarks
├── examples/
│   ├── basic_operations.rs
│   ├── anomaly_detection.rs
│   ├── forecasting.rs
│   └── nasa_gcp_observability.rs
└── tests/
    ├── integration_test.rs
    └── nasa_datasets_test.rs
```

---

## 🚀 Roadmap de Implementação

### Fase 1: Fundamentos (Semanas 1-2) ✅ COMPLETO
- [x] TimeSeries struct básico
- [x] Operações: moving_average, exponential_smoothing
- [x] Estatísticas: mean, std, min, max, percentiles
- [x] Anomaly detection: Z-score, IQR
- [x] Forecasting: Simple/Double ES
- [x] 22 testes passando

### Fase 2: ARIMA & Wavelets (Semanas 3-5)
```rust
// TODO: Implementar ARIMA(p,d,q) completo
pub struct ARIMA {
    p: usize,  // AR order
    d: usize,  // Differencing
    q: usize,  // MA order
}

impl ARIMA {
    /// Fit ARIMA usando Maximum Likelihood Estimation
    pub fn fit(&mut self, data: &TimeSeries) -> Result<ARIMAModel> {
        // 1. Stationarity test (Augmented Dickey-Fuller)
        // 2. ACF/PACF analysis
        // 3. Parameter estimation (MLE via Newton-Raphson)
        // 4. Residual diagnostics (Ljung-Box test)
    }

    /// Previsão multi-step ahead
    pub fn predict(&self, model: &ARIMAModel, steps: usize) -> Vec<f64> {
        // Recursive forecasting com intervalos de confiança
    }
}

// TODO: Implementar Wavelet Transform
pub enum WaveletType {
    Haar,
    Daubechies(usize),  // db2, db4, db8
    Morlet,
    Mexican,
}

pub struct WaveletTransform {
    wavelet: WaveletType,
    levels: usize,
}

impl WaveletTransform {
    /// Decomposição wavelet (DWT)
    pub fn decompose(&self, signal: &[f64]) -> Vec<Vec<f64>> {
        // Retorna coeficientes por nível
    }

    /// Reconstrução a partir de coeficientes
    pub fn reconstruct(&self, coeffs: &[Vec<f64>]) -> Vec<f64> {
        // Inverse DWT
    }
}
```

**Casos de Uso**:
- ARIMA: Previsão de métricas de servidor (CPU, memória, latência)
- Wavelets: Remoção de ruído em sinais gravitacionais (LIGO/LISA)

### Fase 3: Transformadas de Fourier (Semanas 6-7)
```rust
// TODO: Implementar FFT otimizada (Cooley-Tukey)
pub struct FFT {
    size: usize,
    twiddle_factors: Vec<Complex<f64>>,
}

impl FFT {
    /// FFT 1D (radix-2, in-place)
    pub fn fft(&self, signal: &mut [Complex<f64>]) {
        // Bit-reversal permutation
        // Butterfly operations
    }

    /// Power Spectral Density (Welch's method)
    pub fn psd(&self, signal: &[f64], window_size: usize) -> Vec<f64> {
        // Overlapping windows + averaging
    }

    /// Spectrogram (STFT)
    pub fn spectrogram(&self, signal: &[f64], window: usize, hop: usize)
        -> Array2<f64> {
        // Time-frequency representation
    }
}
```

**Aplicações**:
- Análise espectral de ondas gravitacionais
- Detecção de periodicidades em dados astronômicos
- Compressão de sinais (via DCT)

### Fase 4: Métricas NASA GCP (Semanas 8-9)
```rust
// TODO: Implementar NASA Good Coding Practices (GCP)
pub struct DataQualityMetrics {
    pub accuracy: f64,      // 0-1 (% correto)
    pub completeness: f64,  // % não-null
    pub consistency: f64,   // % sem contradições
    pub timeliness: f64,    // latência vs SLA
    pub validity: f64,      // % dentro de ranges
}

impl DataQualityMetrics {
    /// Calcula score geral (média ponderada)
    pub fn overall_score(&self) -> f64 {
        let weights = [0.3, 0.2, 0.2, 0.15, 0.15];
        let scores = [
            self.accuracy,
            self.completeness,
            self.consistency,
            self.timeliness,
            self.validity,
        ];
        weights.iter().zip(&scores).map(|(w, s)| w * s).sum()
    }

    /// Verifica se atende padrões NASA (≥0.95)
    pub fn meets_nasa_standards(&self) -> bool {
        self.overall_score() >= 0.95
    }
}

// TODO: Observability estruturada
pub struct TelemetryTracer {
    spans: Vec<Span>,
    events: Vec<Event>,
}

impl TelemetryTracer {
    pub fn span(&mut self, name: &str) -> SpanGuard {
        // Trace hierárquico (OpenTelemetry-compatible)
    }

    pub fn event(&mut self, level: Level, msg: &str) {
        // Structured logging (JSON)
    }

    pub fn export_otlp(&self) -> Vec<u8> {
        // Export para Jaeger, Tempo, etc.
    }
}
```

**Requisitos NASA**:
- Data quality score ≥ 0.95
- 100% uptime em missões críticas
- Rastreabilidade completa (every decision logged)

### Fase 5: Otimização & SIMD (Semanas 10-12)
```rust
// TODO: Vetorização com SIMD (AVX2, AVX-512, NEON)
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

pub fn moving_average_simd(data: &[f64], window: usize) -> Vec<f64> {
    #[cfg(target_feature = "avx2")]
    unsafe {
        // Process 4 doubles at once with AVX2
        let mut result = vec![0.0; data.len() - window + 1];
        for i in 0..result.len() {
            let mut sum = _mm256_setzero_pd();
            for j in 0..window {
                let val = _mm256_loadu_pd(&data[i + j] as *const f64);
                sum = _mm256_add_pd(sum, val);
            }
            result[i] = _mm256_reduce_add_pd(sum) / window as f64;
        }
        result
    }

    #[cfg(not(target_feature = "avx2"))]
    {
        // Fallback scalar implementation
        data.windows(window)
            .map(|w| w.iter().sum::<f64>() / window as f64)
            .collect()
    }
}
```

**Metas de Performance**:
- Moving average: 500 MB/s (AVX2), 1 GB/s (AVX-512)
- FFT 1M pontos: <10ms
- ARIMA fitting 10K pontos: <100ms

---

## 🧪 Testes Obrigatórios

### 1. Unit Tests (Correção Algorítmica)
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_moving_average_correctness() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let ts = TimeSeries::new(data);
        let ma = ts.moving_average(3).unwrap();

        assert_relative_eq!(ma.data[0], 2.0, epsilon = 1e-10);
        assert_relative_eq!(ma.data[1], 3.0, epsilon = 1e-10);
        assert_relative_eq!(ma.data[2], 4.0, epsilon = 1e-10);
    }

    #[test]
    fn test_arima_fits_ar1() {
        // AR(1): x_t = 0.7 * x_{t-1} + ε
        let data = generate_ar1(1000, 0.7, 1.0);
        let model = ARIMA::new(1, 0, 0);
        let fitted = model.fit(&TimeSeries::new(data)).unwrap();

        // Validar que coeficiente AR ≈ 0.7
        assert_relative_eq!(fitted.ar_coefs[0], 0.7, epsilon = 0.05);
    }
}
```

### 2. Integration Tests (NASA Datasets)
```rust
#[test]
fn test_ligo_h1_strain_analysis() {
    // Dados reais: LIGO Hanford detector (GW150914)
    let strain = load_ligo_h1_strain();

    // 1. Whitening (remove noise spectrum)
    let whitened = whiten_strain(&strain);

    // 2. Matched filtering
    let template = generate_bbh_template(36.0, 29.0); // Massas solares
    let snr = matched_filter(&whitened, &template);

    // 3. Validar SNR máximo ≈ 24 (paper original)
    assert!((snr.max() - 24.0).abs() < 1.0);
}
```

### 3. Benchmarks (Performance)
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_moving_average(c: &mut Criterion) {
    let data: Vec<f64> = (0..1_000_000).map(|x| x as f64).collect();
    let ts = TimeSeries::new(data);

    c.bench_function("moving_average_1M_w100", |b| {
        b.iter(|| black_box(ts.moving_average(100).unwrap()))
    });
}

criterion_group!(benches, bench_moving_average);
criterion_main!(benches);
```

**Targets**:
- Moving average 1M pontos: <2ms
- FFT 1M pontos: <10ms
- ARIMA fit 10K pontos: <100ms

---

## 📊 Integração com AVL Platform

### 1. AvilaDB Backend
```rust
use aviladb::{AvilaClient, Collection};

pub struct TelemetryStore {
    db: AvilaClient,
    metrics: Collection,
}

impl TelemetryStore {
    pub async fn store_metric(&self, name: &str, value: f64, timestamp: i64) {
        self.metrics.insert(document! {
            "name": name,
            "value": value,
            "timestamp": timestamp,
            "host": hostname(),
        }).await.unwrap();
    }

    pub async fn query_time_series(&self, name: &str, start: i64, end: i64)
        -> TimeSeries {
        let cursor = self.metrics
            .query("SELECT value FROM metrics WHERE name = @name
                    AND timestamp >= @start AND timestamp <= @end
                    ORDER BY timestamp")
            .param("name", name)
            .param("start", start)
            .param("end", end)
            .execute()
            .await
            .unwrap();

        let data: Vec<f64> = cursor.collect().await.unwrap();
        TimeSeries::new(data)
    }
}
```

### 2. AVX Gateway Integration
```rust
// Endpoint para queries de métricas
#[get("/api/metrics/{name}/forecast")]
async fn forecast_metric(
    name: Path<String>,
    query: Query<ForecastParams>,
) -> Result<Json<Forecast>> {
    let ts = telemetry_store.query_time_series(&name, query.start, query.end).await?;
    let model = ARIMA::new(2, 1, 2);
    let fitted = model.fit(&ts)?;
    let forecast = fitted.predict(query.steps);

    Ok(Json(Forecast {
        values: forecast,
        confidence: 0.95,
    }))
}
```

---

## 🎓 Recursos de Aprendizado

### Papers Científicos Obrigatórios
1. **Time Series**:
   - Box & Jenkins (1976) - "Time Series Analysis: Forecasting and Control"
   - Hyndman & Athanasopoulos (2021) - "Forecasting: Principles and Practice"

2. **Wavelets**:
   - Mallat (1989) - "A Theory for Multiresolution Signal Decomposition"
   - Daubechies (1992) - "Ten Lectures on Wavelets"

3. **Anomaly Detection**:
   - Chandola et al. (2009) - "Anomaly Detection: A Survey"
   - Iglewicz & Hoaglin (1993) - "How to Detect and Handle Outliers"

4. **NASA Standards**:
   - NASA-STD-8719.13 (2013) - "Software Safety Standard"
   - NASA GCP (2018) - "Good Coding Practices for Scientific Computing"

### Datasets de Validação
- **LIGO Open Science Center**: https://www.gw-openscience.org/
- **NASA Exoplanet Archive**: https://exoplanetarchive.ipac.caltech.edu/
- **NOAA Climate Data**: https://www.ncdc.noaa.gov/

---

## ⚠️ Erros Comuns a Evitar

### 1. Instabilidade Numérica
```rust
// ❌ ERRADO: Variance usando (Σx²)/n - (Σx/n)²
fn variance_naive(data: &[f64]) -> f64 {
    let mean = data.iter().sum::<f64>() / data.len() as f64;
    let sq_diff_sum: f64 = data.iter().map(|x| (x - mean).powi(2)).sum();
    sq_diff_sum / data.len() as f64
}
// Problema: Cancellation catastrophic para dados com mean >> std

// ✅ CORRETO: Welford's online algorithm
fn variance_welford(data: &[f64]) -> f64 {
    let mut mean = 0.0;
    let mut m2 = 0.0;
    for (i, &x) in data.iter().enumerate() {
        let delta = x - mean;
        mean += delta / (i + 1) as f64;
        let delta2 = x - mean;
        m2 += delta * delta2;
    }
    m2 / data.len() as f64
}
```

### 2. Off-by-One em Window Operations
```rust
// ❌ ERRADO: Retorna tamanho errado
fn moving_average(data: &[f64], window: usize) -> Vec<f64> {
    (0..data.len()).map(|i| {
        let end = (i + window).min(data.len());
        data[i..end].iter().sum::<f64>() / window as f64
    }).collect()
}

// ✅ CORRETO: Retorna data.len() - window + 1 valores
fn moving_average(data: &[f64], window: usize) -> Vec<f64> {
    data.windows(window)
        .map(|w| w.iter().sum::<f64>() / window as f64)
        .collect()
}
```

### 3. Memória Excessiva em FFT
```rust
// ❌ ERRADO: Copia dados
fn fft(signal: &[f64]) -> Vec<Complex<f64>> {
    let mut complex: Vec<_> = signal.iter()
        .map(|&x| Complex::new(x, 0.0))
        .collect();
    fft_inplace(&mut complex);
    complex
}

// ✅ CORRETO: In-place quando possível
fn fft_inplace(signal: &mut [Complex<f64>]) {
    // Radix-2 FFT in-place (zero allocations)
}
```

---

## 🏆 Checklist de Qualidade

Antes de fazer PR, verifique:

- [ ] **Correção**: Todos os testes unitários passam
- [ ] **Validação**: Testado com NASA datasets (LIGO, Kepler)
- [ ] **Performance**: Benchmarks atendem targets
- [ ] **Documentação**: Cada função tem docstring com:
  - Descrição do algoritmo
  - Complexidade (tempo e espaço)
  - Referências científicas
  - Exemplos de uso
- [ ] **Estabilidade Numérica**: Usa algoritmos estáveis (Welford, Kahan)
- [ ] **Zero Dependencies**: Apenas ndarray, statrs, rustfft
- [ ] **Clippy Clean**: `cargo clippy -- -D warnings`
- [ ] **Formatting**: `cargo fmt`
- [ ] **Coverage**: `cargo tarpaulin` ≥ 80%

---

## 🚀 Como Começar

### Setup Inicial
```bash
cd arxis/avila-telemetry
cargo build
cargo test
cargo bench
```

### Próxima Tarefa
Consulte `TODO.md` para lista priorizada de tarefas.

### Dúvidas?
1. Leia `ONBOARDING.md` no root do workspace
2. Consulte papers científicos listados acima
3. Valide com NASA datasets antes de implementar

---

**Lembre-se**: Estamos construindo software que processa dados de bilhões de dólares. Correção > Performance > Conveniência.

**Avila Telemetry** - Observatory of the Citadel 🏛️🔭
