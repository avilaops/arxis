# Arquitetura Científica do Arxis para LISA

## 📚 Visão Geral

Este documento descreve a arquitetura científica completa do **Arxis** para processar dados da missão LISA (Laser Interferometer Space Antenna) da NASA/ESA.

---

## 🏗️ 4.1. Camada de Entrada (Input Layer) - ✅ IMPLEMENTADO

A camada de entrada é responsável por ingerir e validar dados de ondas gravitacionais provenientes de múltiplas fontes.

### Fontes de Dados Suportadas

#### 1. **Dados Oficiais da ESA**

##### LISACode
- **Descrição**: Simulador oficial de alta fidelidade desenvolvido pela ESA
- **Formato**: HDF5 (planejado), ASCII (implementado)
- **Uso**: Geração de formas de onda completas no domínio do tempo
- **Referência**: https://gitlab.in2p3.fr/LISA/LISACode

##### LISANode
- **Descrição**: Simulador simplificado para testes rápidos
- **Formato**: ASCII
- **Uso**: Prototipagem e validação de algoritmos

##### LISA Data Challenge (LDC)
- **Descrição**: Formato oficial para competições científicas da ESA
- **Formato**: HDF5 (planejado), ASCII (implementado)
- **Canais TDI**: A, E, T (Time-Delay Interferometry)
- **Uso**: Benchmark de algoritmos de detecção e estimação de parâmetros

#### 2. **Dados Sintéticos Gerados no Arxis**

O Arxis possui seu próprio simulador interno para geração rápida de dados:

##### Formas de Onda Disponíveis

1. **Binário Monocromático** (`monochromatic_binary`)
   - Frequência constante (órbitas circulares estáveis)
   - Uso: Binárias galácticas, testes de calibração
   - Exemplo: Anãs brancas ultra-compactas

2. **Binário com Chirp** (`chirping_binary`)
   - Frequência crescente (inspiral)
   - Uso: SMBHs em fusão, EMRIs
   - Física: Perda de energia por radiação gravitacional

3. **Ruído Gaussiano** (`gaussian_noise`)
   - Simula ruído do detector LISA
   - Uso: Testes de SNR, algoritmos de detecção
   - Método: Transformada Box-Muller

4. **Sinal + Ruído** (`signal_plus_noise`)
   - Observação realística
   - Uso: Treinamento de ML, validação de algoritmos

### Estruturas de Dados

#### `StrainTimeSeries`
```rust
pub struct StrainTimeSeries {
    pub time: Vec<f64>,        // Tempo GPS (segundos)
    pub h_plus: Vec<f64>,      // Polarização +
    pub h_cross: Vec<f64>,     // Polarização ×
    pub sampling_rate: f64,    // Taxa de amostragem (Hz)
    pub duration: f64,         // Duração (segundos)
}
```

**Métodos**:
- `rms_strain()`: Strain RMS (root-mean-square)
- `peak_strain()`: Amplitude máxima
- `len()`: Número de amostras

#### `LDCData`
```rust
pub struct LDCData {
    pub source_id: String,           // Identificador (ex: "SMBHB_001")
    pub version: String,              // Versão do dataset
    pub channel_a: StrainTimeSeries,  // Canal TDI A
    pub channel_e: StrainTimeSeries,  // Canal TDI E
    pub channel_t: StrainTimeSeries,  // Canal TDI T
    pub metadata: LDCMetadata,        // Metadados científicos
}
```

**Métodos**:
- `from_ascii()`: Carrega dados de arquivo ASCII
- `to_ascii()`: Salva dados em formato ASCII
- `from_hdf5()`: Carrega dados HDF5 (requer crate `hdf5`)
- `summary()`: Estatísticas resumidas do dataset

#### `LDCMetadata`
```rust
pub struct LDCMetadata {
    pub source_type: String,              // Tipo (SMBH, EMRI, etc.)
    pub parameters: Vec<(String, f64)>,   // Parâmetros físicos
    pub created: String,                  // Timestamp
    pub software: String,                 // Software gerador
    pub software_version: String,         // Versão
}
```

### Validação de Dados

#### `DataValidator`

O validador executa verificações de qualidade nos dados:

1. **Validação de Formato** (`validate_ldc`)
   - Consistência de comprimento entre canais TDI
   - Ausência de valores NaN/Inf
   - Taxa de amostragem razoável (0.01-10 Hz)
   - Dados não-vazios

2. **Validação de Strain** (`check_strain_range`)
   - Amplitude realística para LISA
   - Sensibilidade: 10⁻²⁵ a 10⁻¹⁵
   - Warnings para sinais muito fracos/fortes

3. **Pipeline Completo** (`validate_all`)
   - Executa todas as validações
   - Retorna lista de avisos/erros
   - Uso: QA antes de análise

### Gerador de Dados Sintéticos

#### `SyntheticDataGenerator`

```rust
let gen = SyntheticDataGenerator::new(
    0.1,        // Sampling rate (Hz)
    86400.0     // Duration (seconds = 1 day)
);
```

**Casos de Uso**:

1. **Testes Rápidos**
   ```rust
   let signal = gen.monochromatic_binary(0.003, 1e-21, 0.0);
   ```

2. **LISA Data Challenge Submission**
   ```rust
   let ldc = gen.generate_ldc_data("SMBH_001", 0.003, 1e-21);
   ldc.to_ascii(Path::new("submission.txt"))?;
   ```

3. **Treinamento de Machine Learning**
   ```rust
   for i in 0..1000 {
       let f = 0.001 + i as f64 * 0.00001;
       let signal = gen.monochromatic_binary(f, 1e-21, 0.0);
       // Salvar para dataset de treinamento
   }
   ```

4. **Validação de Algoritmos**
   ```rust
   let signal = gen.chirping_binary(0.001, 0.01, 1e-21);
   let noisy = gen.signal_plus_noise(&signal, 1e-22);
   // Aplicar algoritmo de detecção
   ```

### Integração com Módulo LISA

A camada de entrada integra-se perfeitamente com o módulo `lisa.rs`:

```rust
// Criar fonte SMBH com parâmetros físicos
let smbh = LISASource::smbh(1e6, 5e5, 1.0, 0.05);

// Gerar forma de onda correspondente
let freq = smbh.gw_frequency();
let amp = smbh.characteristic_strain();
let waveform = generator.monochromatic_binary(freq, amp, 0.0);

// Verificar detectabilidade
let mission = LISAMission::standard();
if mission.is_detectable(&smbh) {
    println!("SNR: {:.1}", smbh.lisa_snr());
}
```

### Formatos de Arquivo

#### ASCII Format (Implementado)
```
# LISA Data Challenge Format (Arxis)
# Source: SMBHB_001
# Version: LDC2a-001
# Sampling_rate: 0.1
# Duration: 31536000
# Software: Arxis
# Software_version: 0.2.0
#
# Columns: time h_plus h_cross
0.000000e+00 1.234567e-21 4.567890e-21
1.000000e-01 1.234568e-21 4.567891e-21
...
```

**Compatibilidade**: LISACode, Python, MATLAB, Mathematica

#### HDF5 Format (Planejado)
- Hierárquico e eficiente
- Suporte nativo da ESA
- Metadata completo
- Compressão integrada

**Implementação futura**:
```rust
// Requer adicionar ao Cargo.toml:
// hdf5 = "0.8"

let ldc = LDCData::from_hdf5(Path::new("official_data.h5"))?;
```

### Estatísticas e Métricas

Para qualquer dataset carregado ou gerado:

```rust
println!("{}", ldc.summary());
```

**Output**:
```
LDC Data Summary
================
Source: ARXIS_SMBH_001
Version: Arxis-Synthetic
Samples: 8640
Duration: 1.0 days
Sampling rate: 0.1 Hz

Channel A:
- RMS strain: 1.00e-21
- Peak strain: 1.00e-21

Channel E:
- RMS strain: 1.00e-21
- Peak strain: 1.00e-21

Channel T:
- RMS strain: 1.00e-21
- Peak strain: 1.00e-21
```

---

## 🔄 4.2. Camada de Processamento (Processing Layer) - ✅ IMPLEMENTADO

A camada de processamento prepara dados brutos para análise científica, aplicando técnicas de condicionamento de sinal e análise espectral.

### Componentes Principais

#### 1. **Análise Espectral**

##### FFT (Fast Fourier Transform)
```rust
let processor = DataProcessor::new(512); // FFT size
let spectrum = processor.compute_fft(&signal);
```

**Recursos**:
- Transformada de Fourier discreta (DFT)
- Suporte para janelas (Hann, Hamming, Blackman, Tukey)
- Espectro de magnitude e fase
- Espectro de potência

##### Power Spectral Density (PSD)
```rust
// Método de Welch (periodograma médio)
let psd = processor.estimate_psd(&signal, n_segments);

// Modelo teórico LISA
let lisa_psd = PowerSpectralDensity::lisa_noise_model(f_min, f_max, n_points);
```

**Aplicações**:
- Caracterização de ruído do detector
- Normalização para whitening
- Cálculo de SNR
- Matched filtering

#### 2. **Condicionamento de Sinal**

##### Whitening (Branqueamento)
```rust
let whitened = processor.whiten(&noisy_signal);
```

**Objetivo**: Transformar ruído colorido em ruído branco (espectro plano)

**Benefícios**:
- Maximiza SNR em matched filtering
- Simplifica análise estatística
- Melhora estimação de parâmetros

##### Filtragem Passa-Banda
```rust
let filtered = processor.bandpass(&signal, f_low, f_high);
```

**Aplicações**:
- Isolar banda LISA (0.1 mHz - 1 Hz)
- Remover ruído fora da banda
- Reduzir aliasing

##### Funções Janela

Disponíveis: `Rectangular`, `Hann`, `Hamming`, `Blackman`, `Tukey`

```rust
let processor = DataProcessor::new(512)
    .with_window(WindowFunction::Hann);
```

**Efeito**: Reduz vazamento espectral (spectral leakage)

#### 3. **TDI (Time-Delay Interferometry)**

TDI é essencial para LISA cancelar ruído de frequência laser.

##### Canais TDI
```rust
let tdi = TDIChannels::from_raw(&data1, &data2, &data3);

// Canal A (Michelson α)
let channel_a = tdi.channel_a;

// Canal E (Michelson ζ)
let channel_e = tdi.channel_e;

// Canal T (Sagnac - null channel)
let channel_t = tdi.channel_t;
```

**Propriedades**:
- **A e E**: Ortogonais, sensíveis a ondas gravitacionais
- **T**: Canal nulo, contém apenas ruído laser
- **Combinação ótima**: Maximiza SNR para fonte desconhecida

##### Combinação Ótima
```rust
let combined = tdi.optimal_combination(&psd_a, &psd_e);
```

Peso baseado em PSDs dos canais individuais.

#### 4. **Detecção e Remoção de Glitches**

Glitches são transientes instrumentais que podem causar falsos positivos.

##### Detecção
```rust
let detector = GlitchDetector::new(5.0); // 5-sigma threshold
let glitches = detector.detect(&signal);
```

**Algoritmo**:
- Estatísticas móveis (média e desvio padrão)
- Threshold em número de sigmas
- Duração mínima configurável

##### Remoção
```rust
let cleaned = detector.remove_glitches(&signal, &glitches);
```

**Método**: Interpolação linear entre pontos antes/depois do glitch

### Estruturas de Dados

#### `FrequencySpectrum`
```rust
pub struct FrequencySpectrum {
    pub frequencies: Vec<f64>,  // Hz
    pub real: Vec<f64>,          // Parte real
    pub imag: Vec<f64>,          // Parte imaginária
    pub df: f64,                 // Resolução
}
```

**Métodos**:
- `magnitude()`: |H(f)|
- `phase()`: arg(H(f))
- `power()`: |H(f)|²
- `to_psd()`: Converte para PSD

#### `PowerSpectralDensity`
```rust
pub struct PowerSpectralDensity {
    pub frequencies: Vec<f64>,
    pub psd: Vec<f64>,           // strain²/Hz
    pub df: f64,
}
```

**Métodos**:
- `interpolate(f)`: PSD em frequência arbitrária
- `rms_noise()`: Ruído RMS integrado
- `lisa_noise_model()`: Curva de sensibilidade LISA

#### `TDIChannels`
```rust
pub struct TDIChannels {
    pub channel_a: StrainTimeSeries,
    pub channel_e: StrainTimeSeries,
    pub channel_t: StrainTimeSeries,
}
```

#### `GlitchEvent`
```rust
pub struct GlitchEvent {
    pub time_start: f64,
    pub time_end: f64,
    pub amplitude: f64,
    pub glitch_type: String,
}
```

### Pipeline Completo

Exemplo de pipeline de processamento end-to-end:

```rust
// 1. Carregar dados
let data = LDCData::from_ascii(Path::new("data.txt"))?;

// 2. Criar processor
let processor = DataProcessor::new(512)
    .with_window(WindowFunction::Hann);

// 3. Detectar e remover glitches
let detector = GlitchDetector::new(5.0);
let glitches = detector.detect(&data.channel_a);
let cleaned = detector.remove_glitches(&data.channel_a, &glitches);

// 4. Aplicar filtro passa-banda
let filtered = processor.bandpass(&cleaned, 0.001, 0.01);

// 5. Whitening
let whitened = processor.whiten(&filtered);

// 6. FFT para análise
let spectrum = processor.compute_fft(&whitened);

// 7. Pronto para matched filtering!
```

### Modelo de Ruído LISA

O Arxis implementa o modelo analítico oficial de ruído LISA:

$$S_n(f) = \left[S_a(f) + S_x(f)\right] \times \left[1 + \left(\frac{2\text{ mHz}}{f}\right)^4\right]$$

Onde:
- $S_a(f)$: Ruído de aceleração
- $S_x(f)$: Ruído de posição
- Termo adicional: Divergência em baixas frequências

**Parâmetros**:
- Comprimento dos braços: $L = 2.5 \times 10^9$ m
- $S_a = 9 \times 10^{-30}$ m²/s⁴/Hz @ 1 mHz
- $S_x = 2.25 \times 10^{-22}$ m²/Hz @ 1 mHz

### Casos de Uso

#### 1. Análise Espectral de Sinal
```rust
let gen = SyntheticDataGenerator::new(0.1, 3600.0);
let signal = gen.chirping_binary(0.002, 0.008, 5e-21);

let processor = DataProcessor::new(512);
let spectrum = processor.compute_fft(&signal);

let power = spectrum.power();
let peak_idx = power.iter()
    .enumerate()
    .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
    .map(|(i, _)| i)
    .unwrap();

println!("Peak at {:.4} mHz", spectrum.frequencies[peak_idx] * 1000.0);
```

#### 2. Estimação de PSD
```rust
// Estimar PSD de dados reais
let psd_estimated = processor.estimate_psd(&noisy_data, 8);

// Comparar com modelo teórico
let psd_theory = PowerSpectralDensity::lisa_noise_model(1e-4, 0.05, 500);

// Verificar consistência
for i in 0..psd_estimated.frequencies.len() {
    let f = psd_estimated.frequencies[i];
    let measured = psd_estimated.psd[i];
    let expected = psd_theory.interpolate(f);
    let ratio = measured / expected;
    println!("{:.4} mHz: ratio = {:.2}", f * 1000.0, ratio);
}
```

#### 3. Processamento de TDI
```rust
// Combinar 3 data streams
let tdi = TDIChannels::from_raw(&stream1, &stream2, &stream3);

// Estimar PSDs
let psd_a = processor.estimate_psd(&tdi.channel_a, 4);
let psd_e = processor.estimate_psd(&tdi.channel_e, 4);

// Combinação ótima
let optimal = tdi.optimal_combination(&psd_a, &psd_e);

// Processar
let whitened = processor.whiten(&optimal);
```

#### 4. Limpeza de Glitches
```rust
// Detectar
let detector = GlitchDetector::new(5.0);
let glitches = detector.detect(&data);

println!("Found {} glitches", glitches.len());
for glitch in &glitches {
    println!("  {:.1}s - {:.1}s: {:.2e}",
        glitch.time_start, glitch.time_end, glitch.amplitude);
}

// Remover
let cleaned = detector.remove_glitches(&data, &glitches);
```

### Referências Técnicas

1. **TDI**: M. Tinto & S. V. Dhurandhar, *Living Rev. Relativity* **7**, 1 (2004)
2. **LISA Sensitivity**: N. Cornish & T. Robson, *arXiv:1803.01944*
3. **Spectral Analysis**: W. H. Press et al., *Numerical Recipes* (2007)
4. **Matched Filtering**: B. Allen et al., *Phys. Rev. D* **85**, 122006 (2012)

---

## 🔍 4.3. Camada de Análise (Analysis Layer) - 🚧 EM DESENVOLVIMENTO

### Objetivos

Preparar dados brutos para análise científica através de:

1. **Análise Espectral**
   - FFT (Fast Fourier Transform)
   - Periodogramas
   - Espectrogramas tempo-frequência

2. **Filtragem e Condicionamento**
   - Whitening (branqueamento)
   - Filtros passa-banda
   - Remoção de linhas espectrais

3. **Combinações TDI**
   - Canal A (Michelson α)
   - Canal E (Michelson ζ)
   - Canal T (Sagnac)
   - Combinações ortogonais

4. **Identificação de Glitches**
   - Detecção de anomalias
   - Remoção de artefatos instrumentais
   - Preenchimento de gaps

### Estrutura Planejada

```rust
pub struct DataProcessor {
    pub fft_size: usize,
    pub window_type: WindowFunction,
}

impl DataProcessor {
    pub fn compute_fft(&self, data: &StrainTimeSeries) -> FrequencySpectrum;
    pub fn whiten(&self, data: &StrainTimeSeries) -> StrainTimeSeries;
    pub fn bandpass(&self, data: &StrainTimeSeries, f_low: f64, f_high: f64) -> StrainTimeSeries;
    pub fn detect_glitches(&self, data: &StrainTimeSeries) -> Vec<GlitchEvent>;
}
```

---

## 🔍 4.3. Camada de Análise (Analysis Layer) - 🚧 EM DESENVOLVIMENTO

### Objetivos

Extrair informação física dos dados processados:

1. **Matched Filtering**
   - Banco de templates
   - Maximização de SNR
   - Detecção de candidatos

2. **Estimação de Parâmetros**
   - Maximum Likelihood Estimation (MLE)
   - Fisher Information Matrix
   - Incertezas paramétricas

3. **Inferência Bayesiana**
   - MCMC (Markov Chain Monte Carlo)
   - Nested Sampling
   - Posterior distributions

4. **Caracterização de Fontes**
   - Classificação (SMBH vs EMRI vs Galactic)
   - Propriedades físicas
   - Localização no céu

### Estrutura Planejada

```rust
pub struct MatchedFilter {
    pub template_bank: TemplateBank,
    pub psd: PowerSpectralDensity,
}

pub struct ParameterEstimator {
    pub method: EstimationMethod,
    pub priors: ParameterPriors,
}

pub struct BayesianInference {
    pub sampler: SamplerType,
    pub n_samples: usize,
}
```

---

## 📊 4.4. Camada de Saída (Output Layer) - 🚧 EM DESENVOLVIMENTO

### Objetivos

Visualizar e comunicar resultados científicos:

1. **Visualização de Dados**
   - Séries temporais
   - Espectrogramas
   - Gráficos tempo-frequência

2. **Gráficos Estatísticos**
   - Corner plots (distribuições posteriores)
   - Confidence regions
   - Parameter correlations

3. **Mapas Celestes**
   - Localização de fontes
   - Sky localization contours
   - Overlay com catálogos EM

4. **Figuras para Publicação**
   - Alta resolução
   - Formatação científica
   - Export SVG/PDF/PNG

### Estrutura Planejada

```rust
pub struct Visualizer {
    pub backend: PlotBackend,
    pub style: PlotStyle,
}

impl Visualizer {
    pub fn plot_timeseries(&self, data: &StrainTimeSeries) -> Plot;
    pub fn plot_spectrogram(&self, data: &FrequencySpectrum) -> Plot;
    pub fn plot_corner(&self, samples: &MCMCSamples) -> Plot;
    pub fn plot_skymap(&self, localization: &SkyLocalization) -> Plot;
}
```

---

## 🚀 Roadmap de Implementação

### ✅ Fase 1: Input Layer (CONCLUÍDA)
- [x] Estruturas de dados (StrainTimeSeries, LDCData)
- [x] Gerador sintético (monochromatic, chirp, noise)
- [x] Validação de dados
- [x] I/O ASCII
- [x] Integração com módulo LISA
- [x] Exemplo completo (`lisa_data_input_example.rs`)

### 🔄 Fase 2: Processing Layer (PRÓXIMA)
- [ ] FFT e análise espectral
- [ ] Whitening
- [ ] Combinações TDI
- [ ] Detecção de glitches
- [ ] Exemplo: `lisa_processing_example.rs`

### 📋 Fase 3: Analysis Layer
- [ ] Template bank generation
- [ ] Matched filtering
- [ ] Maximum likelihood
- [ ] MCMC sampling
- [ ] Exemplo: `lisa_analysis_example.rs`

### 📊 Fase 4: Visualization Layer
- [ ] Time-frequency plots
- [ ] Corner plots
- [ ] Sky maps
- [ ] Publication figures
- [ ] Exemplo: `lisa_visualization_example.rs`

### 🌐 Fase 5: Integration & Deployment
- [ ] Python bindings (PyO3)
- [ ] Web API (REST/GraphQL)
- [ ] Cloud deployment (AVL Platform)
- [ ] Real-time processing
- [ ] Dashboard interativo

---

## 📖 Como Usar

### Instalação

```toml
[dependencies]
arxis_quaternions = "0.2.0"
```

### Exemplo Rápido

```rust
use arxis_quaternions::physics::*;

fn main() {
    // 1. Criar gerador
    let gen = SyntheticDataGenerator::new(0.1, 86400.0);

    // 2. Gerar sinal
    let signal = gen.monochromatic_binary(0.003, 1e-21, 0.0);

    // 3. Adicionar ruído
    let noisy = gen.signal_plus_noise(&signal, 1e-22);

    // 4. Criar dataset LDC
    let ldc = gen.generate_ldc_data("TEST_001", 0.003, 1e-21);

    // 5. Validar
    let warnings = DataValidator::validate_all(&ldc);

    // 6. Salvar
    ldc.to_ascii(Path::new("output.txt"))?;
}
```

### Executar Exemplo Completo

```bash
cd arxis
cargo run --example lisa_data_input_example
```

---

## 🔬 Casos de Uso Científicos

### 1. Preparação para LISA Data Challenge
```rust
let gen = SyntheticDataGenerator::new(0.1, 31536000.0); // 1 year
let ldc = gen.generate_ldc_data("SMBHB_001", 0.003, 1e-21);
ldc.to_ascii(Path::new("ldc_submission.txt"))?;
```

### 2. Teste de Algoritmos de Detecção
```rust
let signal = gen.chirping_binary(0.001, 0.01, 5e-21);
let noisy = gen.signal_plus_noise(&signal, 1e-22);
let snr = signal.rms_strain() / 1e-22;
println!("SNR teórico: {:.1}", snr);
```

### 3. Validação de Dados Externos
```rust
let data = LDCData::from_ascii(Path::new("external_data.txt"))?;
let warnings = DataValidator::validate_all(&data);
for w in warnings {
    println!("{}", w);
}
```

### 4. Geração de Dataset para ML
```rust
for i in 0..10000 {
    let f = 0.0001 + i as f64 * 0.00001;
    let amp = 1e-21 + (i % 100) as f64 * 1e-23;
    let signal = gen.monochromatic_binary(f, amp, 0.0);
    let noisy = gen.signal_plus_noise(&signal, 1e-22);
    // Salvar para dataset de treinamento
    save_to_ml_dataset(&noisy, f, amp)?;
}
```

---

## 📚 Referências Científicas

### Documentos Oficiais da Missão LISA

1. **LISA Mission Proposal**
   arXiv:1702.00786
   https://arxiv.org/abs/1702.00786

2. **LISA Data Challenges**
   https://lisa-ldc.lal.in2p3.fr/

3. **LISA Sensitivity Curve**
   arXiv:1803.01944
   https://arxiv.org/abs/1803.01944

4. **Time-Delay Interferometry (TDI)**
   Living Reviews in Relativity 7, 1 (2004)

5. **LISA Data Analysis**
   arXiv:1806.01772
   https://arxiv.org/abs/1806.01772

### Software e Ferramentas

- **LISA Official Site**: https://lisa.nasa.gov/
- **ESA LISA**: https://www.cosmos.esa.int/lisa
- **LISACode**: https://gitlab.in2p3.fr/LISA/LISACode
- **LISA Data Challenges**: https://lisa-ldc.lal.in2p3.fr/

---

## 🤝 Contribuindo

O Arxis é desenvolvido pela **Avila** (avilaops) e está em desenvolvimento ativo.

**Contato**: nicolas@avila.inc
**GitHub**: https://github.com/avilaops/arxis

---

## 📄 Licença

MIT License - Ver arquivo LICENSE

---

## 🎯 Status Atual

**Versão**: 0.2.0
**Status Input Layer**: ✅ Pronto para produção
**Status Processing Layer**: 🚧 Em desenvolvimento
**Status Analysis Layer**: 📋 Planejado
**Status Visualization Layer**: 📋 Planejado

**Última Atualização**: Novembro 2025
