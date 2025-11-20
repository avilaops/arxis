<div align="center">

# 🏛️ Arxis

### *The Mathematical Citadel. The Computational Engine.*

**ARX** (fortress) + **AXIS** (engine) = **ARXIS**

> *Where solid foundations meet powerful engines*

[![CI](https://github.com/avilaops/arxis/workflows/CI/badge.svg)](https://github.com/avilaops/arxis/actions)
[![Crates.io](https://img.shields.io/crates/v/arxis_quaternions.svg)](https://crates.io/crates/arxis_quaternions)
[![Documentation](https://docs.rs/arxis_quaternions/badge.svg)](https://docs.rs/arxis_quaternions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-CE422B.svg)](https://www.rust-lang.org/)
[![Tests](https://img.shields.io/badge/tests-101%20passing-00C853.svg)](https://github.com/avilaops/arxis)
[![AVL Cloud](https://img.shields.io/badge/AVL-Cloud%20Platform-00d4ff)](https://avila.cloud)

**🌌 NASA/LISA Mission Ready** | **🏛️ Research-Grade** | **📚 [Documentation](https://docs.rs/arxis_quaternions)** | **💬 [Discussions](https://github.com/avilaops/arxis/discussions)**

---

**Research-grade Rust library for General Relativity, Gravitational Waves, and Advanced Mathematics**

*From the fortress of quaternions to the axis of gravitational waves*

</div>

---

## 🎯 What is Arxis?

**Arxis** is the mathematical citadel and computational engine for scientific computing in Rust.

The name combines:
- 🏛️ **ARX** (Latin: *fortress, citadel*) - The unshakeable foundation providing solid mathematical primitives
- ⚙️ **AXIS** (*engine, center*) - The central motor that drives complex systems and simulations

**In every project, Arxis is the engine** - the mathematical core that provides:
- 🏛️ **Solid Foundations**: Rigorous mathematics (quaternions, tensors, 4D geometry)
- ⚙️ **Central Engine**: Computational power driving complex simulations
- 🛡️ **Robust Protection**: Rust's type safety, rigorous testing, reliable APIs
- 🚀 **Propulsion**: Native performance for mission-critical calculations

---

## 📞 Contact

**Project Lead**: Nicolas Ávila
**Email**: nicolas@avila.inc
**WhatsApp**: +55 17 99781-1471
**GitHub**: https://github.com/avilaops/arxis

---

## 🏗️ Architecture Overview

Arxis is built as a modular ecosystem - the fortress that protects and the axis that rotates:

### 📦 Core Crates

#### **avila-math** - Mathematical Kernel
Standalone mathematical library providing foundational primitives:
- **Geometry**: 3D quaternions, dual quaternions, 4D rotations (SO(4))
- **Tensors**: N-dimensional arrays (0D-4D) with ML operations
- **Linear Algebra**: Vectors, matrices, transformations
- **26 tests passing** ✅
- Shared across entire Avila ecosystem (vision, engine, arxis)

#### **avila-telemetry** - Time Series & Analytics
Time series analysis and observability for scientific data:
- **Time Series**: Moving averages, EMA, differencing, statistics
- **Anomaly Detection**: Z-score, IQR methods for glitch detection
- **Forecasting**: ARIMA, Exponential Smoothing models
- **Quality Metrics**: NASA-standard data quality assessment
- **Observability**: Structured logging, alerts, performance tracking
- **22 tests passing** ✅

#### **arxis_quaternions** - Physics Engine
Main library integrating math + telemetry for astrophysics:
- Complete LISA scientific pipeline (Phases 0-6)
- Gravitational waves, relativity, cosmology
- **101 tests passing** (39 LISA + 62 physics) ✅

```
arxis/
├── avila-math/           ← Shared mathematical kernel
│   ├── src/geometry/     (Quat3D, DualQuat, SO4, 4D geometry)
│   └── src/tensor/       (Tensor, Matrix, Vector, ML ops)
│
├── avila-telemetry/      ← Time series & observability
│   ├── src/time_series.rs
│   ├── src/anomaly.rs
│   ├── src/forecasting.rs
│   └── src/observability.rs
│
└── src/                  ← Main physics library
    ├── geometry/         → re-exports avila-math::geometry
    ├── tensor/           → re-exports avila-math::tensor
    └── physics/
        ├── lisa_*.rs     (Complete LISA pipeline)
        └── lisa_telemetry.rs → re-exports avila-telemetry
```

---

## ☁️ AVL Cloud Platform

**Arxis** é a fundação da **AVL (Avila Cloud Platform)** - nossa infraestrutura proprietária de computação científica.

Como a cidadela protege a cidade, Arxis protege seus cálculos:
- **Avx Computing**: Processamento paralelo otimizado (o motor central)
- **AVL Storage**: Armazenamento distribuído para dados científicos (a fortaleza)
- **Avila ML**: Pipeline de ML com tensores 4D nativos (o eixo de rotação)
- **AVL Deploy**: Deployment automatizado (a plataforma de lançamento)

*Diferente de AWS, Azure ou GCP, a AVL é construída sobre as fundações sólidas do Arxis - otimizada especificamente para cargas de trabalho científicas e de engenharia.*

---

## 🌟 Features

Biblioteca Rust completa - a cidadela matemática que inclui **física relativística** (ondas gravitacionais, buracos negros, cosmologia), quaternions, tensores generalizados e **geometria 4D**.

## Características Principais

### 🧮 avila-math - Mathematical Kernel (26 tests)
Shared mathematical primitives for the entire Avila ecosystem:

#### Quaternions & Rotations
- **3D Quaternions (`Quat3D`)**: Rotations, SLERP interpolation, axis-angle
- **Dual Quaternions (`DualQuat`)**: Rigid body transformations
- **SO(4) Rotations**: 4D rotations using S³ × S³ representation

#### Geometry 4D
- **Euclidean ℝ⁴**: Points, vectors, distances, dot products
- **4D Transformations**: 6-plane rotations (XY, XZ, YZ, XW, YW, ZW)
- **Regular Polytopes**:
  - Tesseract (hypercube): 16 vertices, 32 edges, 24 faces, 8 cells
  - 24-Cell: 24 vertices, 96 edges (self-dual, no 3D analog!)
  - 4-Simplex (5-cell): 5 vertices, 10 edges
- **Projections**: Orthographic, perspective, stereographic (4D→3D)

#### Tensors
- **N-Dimensional Arrays**: Scalars (0D), vectors (1D), matrices (2D), 3D, 4D
- **Linear Algebra**: Matrix multiplication, determinant, transpose, inverse
- **ML Operations**: Convolution, pooling, batch normalization, activations

### 📊 avila-telemetry - Time Series & Analytics (22 tests)
Scientific data analysis and quality control:

#### Time Series Analysis
- **Operations**: Moving average, exponential smoothing, differencing
- **Statistics**: Mean, std dev, min, max, percentiles
- **Transformations**: Slicing, resampling, windowing

#### Anomaly Detection
- **Statistical Methods**: Z-score (configurable threshold)
- **Robust Methods**: IQR (Interquartile Range) detection
- **Use Cases**: Glitch detection in LISA data, instrumental artifacts

#### Forecasting
- **ARIMA Models**: AutoRegressive Integrated Moving Average
- **Exponential Smoothing**: Simple, double, triple exponential
- **Applications**: Observation planning, trend prediction

#### Data Quality (NASA Standards)
- **Quality Metrics**: Accuracy, completeness, consistency, validity
- **Scoring**: Overall quality score (0-1), NASA threshold (≥0.95)
- **Observability**: Structured logging, alerts, performance tracking

---
- **Sistemas binários compactos**: Fusões de buracos negros e estrelas de nêutrons
- **Formas de onda**: Inspiração, coalescência, ringdown
- **Detecção**: Cálculo de SNR para LIGO, Virgo, LISA
- **Análise**: Estimativa de parâmetros (massas, distâncias, spins)
- **Validação**: GW150914, PSR B1913+16

#### 🛰️ LISA Scientific Pipeline (Complete)
**Phase 0: Mathematical Kernel** ✅
- Tensor operations, quaternion algebra, relativity framework

**Phase 1: Input Layer** ✅ (`lisa_data.rs`)
- LDC format ingestion (HDF5, JSON)
- Synthetic data generation (MBHB, EMRI, Galactic binaries)
- Data validation and quality checks
- 6 tests passing

**Phase 2: Processing Layer** ✅ (`lisa_processing.rs`)
- FFT/IFFT for frequency domain analysis
- Power Spectral Density (PSD) estimation
- Data whitening and conditioning
- Time-Delay Interferometry (TDI)
- Glitch detection and mitigation
- 6 tests passing

**Phase 3: Analysis Layer** ✅ (`lisa_analysis.rs`)
- Matched filtering for signal detection
- Template bank generation (MBHB, EMRI, Galactic)
- Parameter estimation (chirp mass, mass ratio, distance)
- Event detection with SNR threshold
- Signal consistency checks
- 10 tests passing

**Phase 4: Visualization Layer** ✅ (`lisa_visualization.rs`)
- Time series plots (ASCII art rendering)
- Spectrograms (STFT-based, grayscale colormap)
- SNR plots with peak detection
- Template bank coverage visualization
- Sky maps with event localization
- 5 tests passing

**Phase 5: Event Catalog & Reporting** ✅ (`lisa_catalog.rs`)
- Event database (CRUD operations)
- Source classification (MBHB/EMRI/Galactic)
- Metadata management (GPS/UTC time, confidence, FAR)
- Query filters (by source, SNR, time)
- Statistics computation (counts, distributions)
- Export formats (JSON, CSV)
- Report generation (formatted analysis)
- 6 tests passing

**Phase 6: Bayesian Parameter Estimation** ✅ (`lisa_inference.rs`)
- MCMC sampling (Metropolis-Hastings algorithm)
- Prior distributions (Uniform, LogUniform, Gaussian, Fixed)
- Likelihood calculation for GW signals
- Posterior inference with uncertainties
- Chain diagnostics (ESS, acceptance rate, convergence)
- Credible intervals (5%-95%) and medians
- Parameter recovery assessment
- 6 tests passing

**Total: 101 tests passing** (39 LISA + 62 physics modules)

**Integration with avila-telemetry:**
- Time series analysis of strain data (moving averages, statistics)
- Anomaly detection for glitch identification (Z-score, IQR)
- Data quality assessment (NASA standards compliance)
- Performance monitoring and observability

**Examples:**
- `lisa_data_example.rs` - Data generation and LDC format
- `lisa_processing_example.rs` - Signal processing pipeline
- `lisa_analysis_example.rs` - Matched filtering search
- `lisa_visualization_example.rs` - Multi-plot demonstration
- `lisa_catalog_example.rs` - Complete end-to-end pipeline
- `lisa_inference_example.rs` - Bayesian parameter estimation with MCMC


### 🕳️ Relatividade Geral
- **Métricas**: Schwarzschild, Kerr, FLRW, Minkowski
- **Geodésicas**: Trajetórias de partículas, órbitas relativísticas
- **Curvatura**: Tensor de Riemann, tensor de Einstein
- **Efeitos**: Precessão periélica, redshift gravitacional, deflexão de luz

### 🔭 Lentes Gravitacionais
Einstein rings, arcs, microlensing events:
- **Strong lensing**: Múltiplas imagens, anéis de Einstein
- **Weak lensing**: Shear, convergência, cosmic shear
- **Microlensing**: Curvas de luz, detecção de exoplanetas
- **Modelos**: Point Mass, SIS, NFW

### 🌌 Cosmologia
FLRW universe, Planck 2018 parameters:
- **Modelo FLRW**: Universo em expansão (Planck 2018)
- **Distâncias**: Luminosa, angular, comóvel
- **Evolução**: H(z), q(z), idade do universo
- **Observáveis**: CMB, supernovas, estrutura em larga escala

### 🔄 Mathematical Primitives (from avila-math)

#### Quaternions 3D (`Quat3D`)
- Representação de rotações em 3D usando álgebra de quaternions
- Operações: multiplicação, conjugado, inverso, normalização
- Conversão de/para ângulos de Euler e eixo-ângulo
- Rotação de vetores 3D
- Interpolação esférica (SLERP)
- Conversão para matriz de rotação 3×3

#### 🌀 Quaternions Duplos (`DualQuat`)
- Representação de rotações e translações combinadas
- Suporte para transformações rígidas (rigid body transformations)
- Interpolação linear de dual quaternions (DLB)
- Extração de componentes de rotação e translação

#### 🔮 SO(4) - Rotações 4D (`SO4Rotation`)
- Representação isomórfica ao produto S³ × S³
- Rotações independentes "left" e "right"
- Aplicação de rotação usando a fórmula: **q₁ × v × q₂\***
- Composição de rotações SO(4)
- Conversão para matriz de rotação 4×4
- Decomposição isoclínica

#### 📐 Geometria 4D (`geometry4d`)
- **Espaço Euclidiano ℝ⁴**: Pontos, vetores, distâncias, produto escalar
- **Transformações 4D**: Matrizes 4×4 com rotações em 6 planos (XY, XZ, YZ, XW, YW, ZW)
- **Politopos Regulares 4D**:
  - Tesserato (Hipercubo): 16 vértices, 32 arestas, 24 faces, 8 células
  - 24-Cell: 24 vértices, 96 arestas (autodual, sem análogo 3D!)
  - Simplex 4D (5-cell): 5 vértices, 10 arestas
- **Projeções 4D→3D**: Ortográfica, perspectiva, estereográfica
- **Visualização ASCII**: Renderizador 3D para projeções de objetos 4D
- **Cinemática 4D**: Corpos rígidos com posição, velocidade e rotação em 4D

#### 📊 Tensores Generalizados
- **Ordem 0**: Escalares
- **Ordem 1**: Vetores (produto escalar, norma, produto vetorial)
- **Ordem 2**: Matrizes (multiplicação, determinante, traço, transposição)
- **Ordem 3**: Tensores 3D (convolução 3D, fatias 2D)
- **Ordem 4**: Tensores 4D (batch de imagens, contração, produto externo)

#### 🧠 Machine Learning
- Operações de batch processing
- Funções de ativação: ReLU, Sigmoid, Tanh
- Pooling: Max pooling, Average pooling
- Convolução 2D
- Batch normalization
- Operações para backpropagation

#### 🌌 Relatividade Geral
- Métrica de Minkowski
- Transformações de Lorentz (boosts e rotações)
- Tensor energia-momento
- Tensor de Riemann (curvatura)
- Intervalos espaço-temporais
- Classificação de vetores (tipo tempo, tipo luz, tipo espaço)

## Teoria Matemática

### Quaternions 3D
Um quaternion é representado como:
```
q = w + xi + yj + zk
```
onde i² = j² = k² = ijk = -1

Para rotacionar um vetor v por um quaternion unitário q:
```
v' = q × v × q*
```
onde q* é o conjugado de q.

### SO(4) e S³ × S³
O grupo de rotações SO(4) é isomórfico ao produto de duas esferas 3D:
```
SO(4) ≅ (S³ × S³) / {±1}
```

Isso permite representar rotações 4D usando dois quaternions (q₁, q₂):
```
v' = q₁ × v × q₂*
```

Esta representação permite:
- **Rotações left**: apenas q₁ varia (q₂ = identidade)
- **Rotações right**: apenas q₂ varia (q₁ = identidade)
- **Rotações duplas**: ambos q₁ e q₂ variam independentemente

## Instalação

Adicione ao seu `Cargo.toml`:
```toml
[dependencies]
arxis_quaternions = "0.2.0"
avila-math = { git = "https://github.com/avilaops/arxis", branch = "main" }
avila-telemetry = { git = "https://github.com/avilaops/arxis", branch = "main" }
rand = "0.8"
chrono = "0.4"
```

Para uso standalone das crates:
```toml
# Apenas kernel matemático
[dependencies]
avila-math = { git = "https://github.com/avilaops/arxis", branch = "main" }

# Apenas telemetria e time series
[dependencies]
avila-telemetry = { git = "https://github.com/avilaops/arxis", branch = "main" }
chrono = "0.4"
```

---

## Teoria Matemática

### Tensores Generalizados
Um tensor de ordem (rank) N é uma generalização de:
- **Ordem 0**: Escalar (número)
- **Ordem 1**: Vetor (array unidimensional)
- **Ordem 2**: Matriz (array bidimensional)
- **Ordem 3**: Tensor 3D (cubos de dados)
- **Ordem 4**: Tensor 4D (hipercubos, batches de imagens)

## Exemplos de Uso

### Complete LISA Pipeline with Telemetry
```rust
use arxis_quaternions::physics::*;
use avila_telemetry::{TimeSeries, anomaly::AnomalyDetector};

// Phase 1: Generate synthetic LISA data
let gen = SyntheticDataGenerator::new(0.1, 10000.0);
let source = LISASource::smbh(1e6, 5e5, 3e25, 1.0);
let signal = gen.monochromatic_binary(
    source.gw_frequency(),
    source.characteristic_strain(),
    0.0
);
let data = gen.signal_plus_noise(&signal, 1e-22);

// Phase 2: Time series analysis with avila-telemetry
let ts = TimeSeries::new(data.strain.clone());
let ma = ts.moving_average(10)?; // 10-point moving average
let stats = ts.statistics();
println!("Mean: {:.2e}, Std: {:.2e}", stats.mean, stats.std_dev);

// Detect anomalies (glitches)
let detector = AnomalyDetector::new(3.0, 1.5); // 3-sigma, 1.5 IQR
let anomalies = detector.detect_zscore(&ts)?;
println!("Found {} anomalies", anomalies.len());

// Phase 3: Process data
let spectrum = data.fft();
let psd = PowerSpectralDensity::from_data(&data, 1024, 512);
let whitened = data.whiten(&psd);

// Phase 4: Matched filtering search
let mut bank = TemplateBank::new(0.97);
bank.generate_mbhb_grid((5e5, 3e6), (2e5, 1e6), 5, 4, 3e25, 10000.0, 0.1);
let mf = MatchedFilter::new(bank, psd, 7.0);
let results = mf.search(&whitened);
let events = mf.cluster_events(&results, 500.0);

// Phase 5: Catalog and quality assessment
let mut catalog = EventCatalog::new(
    "LISA-O1".to_string(),
    "1.0.0".to_string(),
    "arxis-0.2.0".to_string(),
);

for (i, result) in events.iter().enumerate() {
    let event = CatalogEvent {
        id: format!("LISA-GW-{:06}", 240120 + i),
        snr: result.snr,
        source_type: SourceClassification::from_mass_ratio(
            result.parameters.mass_ratio,
            result.parameters.total_mass,
        ),
        data_quality: DataQuality {
            glitches: anomalies.len(),
            gaps: 0,
            score: 0.95
        },
        // ... other fields
    };
    catalog.add_event(event);
}

println!("{}", catalog.generate_report());
```

### avila-math: Quaternions & 4D Geometry
```rust
use avila_math::geometry::{Quat3D, SO4Rotation, Geometry4D};
use std::f64::consts::PI;

// 3D rotation with quaternions
let q = Quat3D::from_axis_angle([0.0, 0.0, 1.0], PI / 2.0);
let v = [1.0, 0.0, 0.0];
let rotated = q.rotate_vector(v); // ≈ [0.0, 1.0, 0.0]

// 4D rotation with SO(4)
let q_left = Quat3D::from_axis_angle([1.0, 0.0, 0.0], PI / 6.0);
let q_right = Quat3D::from_axis_angle([0.0, 1.0, 0.0], PI / 4.0);
let so4 = SO4Rotation::new(q_left, q_right);
let v4 = [1.0, 0.0, 0.0, 0.0];
let rotated_4d = so4.rotate_vector_4d(v4);

// 4D polytopes
let tesseract = Geometry4D::tesseract();
println!("Vertices: {}", tesseract.vertices.len()); // 16
```

### avila-telemetry: Time Series & Forecasting
```rust
use avila_telemetry::{
    TimeSeries,
    forecasting::ExponentialSmoothing,
    models::ARIMA,
    observability::DataQualityAssessment,
};

// Time series operations
let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 4.0, 3.0, 2.0];
let ts = TimeSeries::new(data);
let ma = ts.moving_average(3)?;
let diff = ts.diff();
let stats = ts.statistics();

// Forecasting with ARIMA
let mut arima = ARIMA::new(1, 1, 1); // p=1, d=1, q=1
arima.fit(&ts)?;
let forecast = arima.predict(5)?; // Forecast 5 steps ahead

// Data quality assessment (NASA standards)
let mut quality = DataQualityAssessment {
    accuracy: 0.98,
    completeness: 0.95,
    consistency: 0.92,
    timeliness_ms: 100,
    validity: 0.97,
    overall_score: 0.0,
};
quality.calculate_overall();
println!("Meets NASA standards: {}", quality.meets_nasa_standards());
```

### LISA Complete Pipeline
```rust
use arxis_quaternions::physics::*;

// Phase 1: Generate synthetic LISA data
let gen = SyntheticDataGenerator::new(0.1, 10000.0);
let source = LISASource::smbh(1e6, 5e5, 3e25, 1.0);
let signal = gen.monochromatic_binary(
    source.gw_frequency(),
    source.characteristic_strain(),
    0.0
);
let data = gen.signal_plus_noise(&signal, 1e-22);

// Phase 2: Process data
let spectrum = data.fft();
let psd = PowerSpectralDensity::from_data(&data, 1024, 512);
let whitened = data.whiten(&psd);

// Phase 3: Matched filtering search
let mut bank = TemplateBank::new(0.97);
bank.generate_mbhb_grid((5e5, 3e6), (2e5, 1e6), 5, 4, 3e25, 10000.0, 0.1);
let mf = MatchedFilter::new(bank, psd, 7.0);
let results = mf.search(&whitened);
let events = mf.cluster_events(&results, 500.0);

// Phase 4: Visualize
let plot = TimeSeriesPlot::from_strain(&whitened);
println!("{}", plot.to_ascii(80, 20));

let snr_plot = SNRPlot::new(
    results.iter().map(|r| r.time).collect(),
    results.iter().map(|r| r.snr).collect(),
    7.0
);
println!("{}", snr_plot.to_ascii(80, 20));

// Phase 5: Catalog events
let mut catalog = EventCatalog::new(
    "LISA-O1".to_string(),
    "1.0.0".to_string(),
    "arxis-0.2.0".to_string(),
);

for (i, result) in events.iter().enumerate() {
    let event = CatalogEvent {
        id: format!("LISA-GW-{:06}", 240120 + i),
        gps_time: result.time,
        utc_time: format!("2024-01-20T{:02}:{:02}:00Z", i/60, i%60),
        snr: result.snr,
        far: 1e-6,
        false_alarm_prob: 0.01,
        confidence: if result.snr > 15.0 { 0.95 } else { 0.80 },
        source_type: SourceClassification::from_mass_ratio(
            result.parameters.mass_ratio,
            result.parameters.total_mass,
        ),
        parameters: result.parameters.clone(),
        sky_location: None,
        data_quality: DataQuality { glitches: 0, gaps: 0, score: 0.95 },
        metadata: HashMap::new(),
        pipeline_version: "arxis-0.2.0".to_string(),
    };
    catalog.add_event(event);
}

// Generate reports and export
println!("{}", catalog.generate_report());
catalog.export_json("catalog.json")?;
catalog.export_csv("catalog.csv")?;

// Query catalog
let mbhb_events = catalog.filter_by_source(SourceClassification::MBHB);
let high_snr = catalog.filter_by_snr(10.0);
let stats = catalog.statistics();
```

### Ondas Gravitacionais (LIGO/LISA)
```rust
use arxis_quaternions::physics::*;

// Criar sistema binário (tipo GW150914)
let binary = CompactBinary::new(
    36.0,  // M1 em massas solares
    29.0,  // M2 em massas solares
    350.0, // separação em km
    427e6 * 3.086e22, // distância em metros
    0.0    // excentricidade
);

// Gerar forma de onda
let wave = binary.generate_wave();
let frequency = binary.gravitational_wave_frequency();

// Calcular SNR para LIGO
let ligo = Detector::ligo();
let snr = ligo.signal_to_noise_ratio(&wave, 0.2);
println!("SNR: {:.1} (detectável se > 8)", snr);
```


### Lentes Gravitacionais
```rust
use arxis_quaternions::physics::*;

// Lente tipo Einstein Cross
let lens = GravitationalLens::point_mass(
    1e11,  // massa da galáxia em M☉
    d_lens_pc,
    d_source_pc,
);

// Raio de Einstein
let theta_e = lens.einstein_radius_arcsec();

// Múltiplas imagens
let images = lens.image_positions(source_beta);
for &theta in &images {
    let mag = lens.magnification(theta);
    println!("Imagem: θ={:.3}\" μ={:.1}×", theta * 206265.0, mag);
}
```

### Cosmologia
```rust
use arxis_quaternions::physics::*;

// Universo com parâmetros Planck 2018
let universe = FLRWUniverse::standard();

// Distância a supernova
let z = 0.5;
let d_L = universe.luminosity_distance(z);
let distance_modulus = universe.distance_modulus(z);

// Parâmetros de evolução
let H_z = universe.hubble_parameter(z);
let q_z = universe.deceleration_parameter(z);
let age = universe.age_of_universe() / (365.25 * 24.0 * 3600.0 * 1e9);
println!("Idade do universo: {:.2} Gyr", age);
```

### Missão LISA (NASA/ESA)
```rust
use arxis_quaternions::physics::lisa::*;

// Supermassive black hole binary
let smbh = LISASource::smbh(
    1e6,   // 1 million solar masses
    5e5,   // 500,000 solar masses
    1.0,   // redshift z=1
    0.05   // 0.05 AU separation
);

// Verificar detectabilidade
let mission = LISAMission::standard();
if mission.is_detectable(&smbh) {
    println!("SNR: {:.1}", smbh.lisa_snr());
    println!("Cycles: {:.0}", smbh.observable_cycles());
}

// Extreme mass ratio inspiral (EMRI)
let emri = LISASource::emri(1e6, 10.0, 0.5, 10.0);
println!("{}", emri.summary());
```

### Tensores Básicos
```rust
use arxis_quaternions::tensor::{Vector, Matrix, Tensor};

// Vetor
let v = Vector::from_slice(&[1.0, 2.0, 3.0]);
let norm = v.norm();

// Matriz
let m = Matrix::new(3, 3);
let identity = Matrix::identity(3);
let product = m.matmul(&identity).unwrap();

// Tensor 4D para imagens
use arxis_quaternions::tensor4d::Tensor4D;
let batch = Tensor4D::from_images(8, 3, 32, 32); // batch, channels, height, width
```

### Operações de Machine Learning
```rust
use arxis_quaternions::tensor4d::{Tensor4D, image_ops};

// Convolução 2D
let output = image_ops::conv2d(&input, &kernel, stride, padding)?;

// Pooling
let pooled = input.max_pool_2d(2, 2)?;

// Ativações
let activated = input.relu();
let normalized = input.batch_normalize(1e-5);
```

### Relatividade
```rust
use arxis_quaternions::relativity::{MinkowskiMetric, LorentzTransform};

// Métrica de Minkowski
let metric = MinkowskiMetric::new();
let interval = metric.interval(&four_vector);

// Transformação de Lorentz (boost)
let boost = LorentzTransform::boost_x(0.6)?; // 60% da velocidade da luz
let transformed = boost.transform(&event)?;
```

### Rotação 3D básica
```rust
use arxis_quaternions::Quat3D;
use std::f64::consts::PI;

// Cria rotação de 90° em torno do eixo Z
let q = Quat3D::from_axis_angle([0.0, 0.0, 1.0], PI / 2.0);

// Rotaciona vetor
let v = [1.0, 0.0, 0.0];
let v_rotated = q.rotate_vector(v);
// v_rotated ≈ [0.0, 1.0, 0.0]
```

### Interpolação de rotações (SLERP)
```rust
use arxis_quaternions::Quat3D;

let q_start = Quat3D::identity();
let q_end = Quat3D::from_axis_angle([0.0, 1.0, 0.0], PI / 2.0);

// Interpola 50% entre as duas rotações
let q_mid = q_start.slerp(&q_end, 0.5);
```

### Transformações rígidas com Dual Quaternions
```rust
use arxis_quaternions::{Quat3D, DualQuat};

let rotation = Quat3D::from_axis_angle([0.0, 0.0, 1.0], PI / 4.0);
let translation = [5.0, 3.0, 0.0];

let dq = DualQuat::from_rotation_translation(rotation, translation);

let point = [1.0, 0.0, 0.0];
let transformed = dq.transform_point(point);
```

### Rotações 4D com SO(4)
```rust
use arxis_quaternions::{Quat3D, SO4Rotation};
use std::f64::consts::PI;

// Cria rotação SO(4) com componentes left e right
let q_left = Quat3D::from_axis_angle([1.0, 0.0, 0.0], PI / 6.0);
let q_right = Quat3D::from_axis_angle([0.0, 1.0, 0.0], PI / 4.0);
let so4 = SO4Rotation::new(q_left, q_right);

// Rotaciona vetor 4D usando q1 * v * q2*
let v4 = [1.0, 0.0, 0.0, 0.0];
let v4_rotated = so4.rotate_vector_4d(v4);
```

### Rotações independentes Left/Right
```rust
use arxis_quaternions::{Quat3D, SO4Rotation};

// Rotação apenas no componente left
let so4_left = SO4Rotation::from_left(
    Quat3D::from_axis_angle([0.0, 0.0, 1.0], PI / 3.0)
);

// Rotação apenas no componente right
let so4_right = SO4Rotation::from_right(
    Quat3D::from_axis_angle([1.0, 0.0, 0.0], PI / 4.0)
);

// Composição de rotações
let so4_composed = so4_left.compose(&so4_right);
```

## Executar Exemplos

Todos os exemplos estão no diretório `examples/`:

```bash
# Complete LISA pipeline (all phases 0-6)
cargo run --example lisa_example

# LISA data processing and telemetry
cargo run --example lisa_catalog_example

# Bayesian inference with MCMC
cargo run --example lisa_inference_example

# Física: Aplicações práticas integradas
cargo run --example practical_astrophysics

# avila-math examples
cargo run --example rotations_example      # Quaternions 3D/4D
cargo run --example geometry4d_example     # 4D polytopes & projections
cargo run --example tensors_example        # Tensor operations

# Physics examples
cargo run --example gravitational_example  # Gravitational waves
cargo run --example lensing_example        # Gravitational lensing
cargo run --example cosmology_example      # FLRW cosmology
cargo run --example geodesic_example       # Orbits & geodesics
cargo run --example einstein_example       # Metrics & curvature
cargo run --example relativity_example     # Lorentz transforms

# Machine learning
cargo run --example ml_example             # Neural networks with tensors
```

---

## Testes

Execute os testes unitários:

```bash
# Todos os testes (101 total)
cargo test

# Por módulo específico
cargo test --lib                    # Biblioteca principal (101 tests)
cargo test -p avila-math           # Kernel matemático (26 tests)
cargo test -p avila-telemetry      # Telemetria (22 tests)

# Testes LISA específicos
cargo test lisa                     # Todos os testes LISA (39 tests)
cargo test lisa_data                # Phase 1: Input (6 tests)
cargo test lisa_processing          # Phase 2: Processing (6 tests)
cargo test lisa_analysis            # Phase 3: Analysis (10 tests)
cargo test lisa_visualization       # Phase 4: Visualization (5 tests)
cargo test lisa_catalog             # Phase 5: Catalog (6 tests)
cargo test lisa_inference           # Phase 6: Bayesian (6 tests)

# Testes de física
cargo test gravitational_waves      # GW tests
cargo test relativity               # Relativity tests
cargo test cosmology                # Cosmology tests
```

---

## Estrutura do Projeto

```
arxis/
├── avila-math/                     # Mathematical kernel (26 tests)
│   ├── src/
│   │   ├── geometry/               # Quaternions, SO(4), 4D geometry
│   │   │   ├── quaternion3d.rs     # 3D rotations
│   │   │   ├── dual_quaternion.rs  # Rigid body transforms
│   │   │   ├── geometry4d.rs       # 4D polytopes & projections
│   │   │   └── mod.rs
│   │   ├── tensor/                 # N-dimensional arrays
│   │   │   ├── tensor.rs           # Vectors, matrices
│   │   │   ├── tensor4d.rs         # 3D/4D tensors, ML ops
│   │   │   └── mod.rs
│   │   └── lib.rs
│   └── Cargo.toml
│
├── avila-telemetry/                # Time series & analytics (22 tests)
│   ├── src/
│   │   ├── time_series.rs          # TimeSeries core
│   │   ├── anomaly.rs              # Anomaly detection
│   │   ├── forecasting.rs          # Forecasters interface
│   │   ├── models/
│   │   │   ├── arima.rs            # ARIMA implementation
│   │   │   └── mod.rs
│   │   ├── features.rs             # Feature engineering
│   │   ├── decomposition.rs        # Trend/seasonal decomposition
│   │   ├── observability.rs        # NASA quality control
│   │   └── lib.rs
│   ├── examples/                   # Usage examples
│   ├── benches/                    # Performance benchmarks
│   └── Cargo.toml
│
├── src/                            # Main arxis library (101 tests)
│   ├── lib.rs                      # Module root
│   ├── geometry/                   # Re-exports avila-math
│   │   └── mod.rs
│   ├── tensor/                     # Re-exports avila-math
│   │   └── mod.rs
│   └── physics/                    # Physics implementations
│       ├── mod.rs
│       ├── gravitational_waves.rs  # GW fundamentals
│       ├── relativity.rs           # Lorentz, spacetime
│       ├── cosmology.rs            # FLRW universe
│       ├── geodesic.rs             # Orbits, trajectories
│       ├── gravitational_lensing.rs
│       ├── einstein.rs             # Metrics, curvature
│       ├── lisa.rs                 # Phase 0: LISA fundamentals
│       ├── lisa_data.rs            # Phase 1: Data ingestion
│       ├── lisa_processing.rs      # Phase 2: Signal processing
│       ├── lisa_analysis.rs        # Phase 3: Matched filtering
│       ├── lisa_visualization.rs   # Phase 4: Plotting
│       ├── lisa_catalog.rs         # Phase 5: Event catalog
│       ├── lisa_inference.rs       # Phase 6: Bayesian MCMC
│       └── lisa_telemetry.rs       # Re-exports avila-telemetry
│
├── examples/                       # Usage examples
│   ├── lisa_example.rs
│   ├── lisa_catalog_example.rs
│   ├── lisa_inference_example.rs
│   ├── practical_astrophysics.rs
│   ├── rotations_example.rs
│   ├── geometry4d_example.rs
│   ├── tensors_example.rs
│   └── ... (15+ examples)
│
├── docs/                           # Documentation
│   ├── ECOSYSTEM_SYNC.md           # Integration strategy
│   ├── TENSOR_DOCUMENTATION.md
│   ├── VISUALIZATION_GUIDE.md
│   └── NASA_LISA_INTEGRATION.md
│
└── Cargo.toml                      # Workspace configuration
```

---

## Aplicações

### 🚀 Astrofísica e NASA/LISA
- **Análise de dados LIGO/Virgo/KAGRA** (ondas gravitacionais terrestres)
- **Preparação para missão LISA** (ondas gravitacionais espaciais, 2030s)
- **Complete scientific pipeline**: Data ingestion → Processing → Analysis → Visualization → Cataloging → Bayesian inference
- **Modelagem de fontes**: SMBHs, EMRIs, binários galácticos, stochastic background
- **Estimativa de parâmetros**: massas, distâncias, spins com MCMC
- **LISA Data Challenge** participation ready
- **Pulsar timing** (PSR B1913+16, NANOGrav)
- **Standard sirens** for cosmology (H₀ measurement)

### 📊 Scientific Data Analysis (avila-telemetry)
- **Time series analysis**: Sensor data, telemetry streams, astronomical time series
- **Anomaly detection**: Instrumental glitches, transient events, quality control
- **Forecasting**: Observation planning, resource allocation, trend prediction
- **Quality assessment**: NASA-standard data quality metrics for missions
- **Observability**: Monitoring scientific pipelines, performance tracking

### 🧮 Mathematical Computing (avila-math)
- **Computational geometry**: 3D/4D transformations, polytopes, projections
- **Linear algebra**: High-performance matrix operations for simulations
- **Quaternion algebra**: Spacecraft attitude control, robotics orientation
- **Tensor operations**: Shared kernel for ML, physics, computer graphics

### 🎮 Computer Graphics & Animation
- Smooth rotations and camera interpolation (SLERP)
- Rigid body transformations (dual quaternions)
- 4D object visualization (tesseracts, 24-cell)
- Inverse kinematics
- 4D→3D→2D projection pipelines

### 🤖 Robotics & Control Systems
- Spacecraft attitude control (quaternion-based)
- Robot arm kinematics and orientation
- Trajectory planning with smooth interpolation
- Sensor fusion for orientation estimation

### 🧠 Machine Learning
- Convolutional Neural Networks (CNNs)
- Batch image processing (Tensor4D)
- Custom activation functions and layers
- Tensor operations for scientific ML

### 📡 Signal Processing & Telemetry
- Real-time anomaly detection in sensor streams
- Time series forecasting for system monitoring
- Data quality assessment for mission-critical systems
- Performance monitoring and observability

---

## Referências

### Gravitational Waves & LISA
- **LISA Mission**: [ESA/NASA LISA Overview](https://lisa.nasa.gov/)
- **LIGO Scientific Collaboration**: [LIGO Lab Caltech](https://www.ligo.caltech.edu/)
- **GW Data Analysis**: Cutler & Flanagan, "Gravitational Waves from Merging Compact Binaries"
- **Matched Filtering**: Allen et al., "FINDCHIRP: An Algorithm for Detection of Gravitational Waves"
- **LISA Sensitivity**: Robson, Cornish & Liu, "The Construction and Use of LISA Sensitivity Curves"

### Time Series & Anomaly Detection
- **Time Series Analysis**: Box, Jenkins, Reinsel - "Time Series Analysis: Forecasting and Control"
- **Anomaly Detection**: Chandola, Banerjee, Kumar - "Anomaly Detection: A Survey" (ACM Computing Surveys)
- **ARIMA Models**: Hyndman & Athanasopoulos - "Forecasting: Principles and Practice"
- **NASA Quality Standards**: NASA-STD-8739.8A - "Software Assurance and Software Safety Standard"

### Quaternions & Rotations
- **Quaternions e Rotações**: [Ken Shoemake, "Animating Rotation with Quaternion Curves"](https://www.cs.cmu.edu/~kiranb/animation/p245-shoemake.pdf)
- **Dual Quaternions**: [Kavan et al., "Skinning with Dual Quaternions"](https://users.cs.utah.edu/~ladislav/kavan07skinning/kavan07skinning.pdf)
- **SO(4) e S³×S³**: [John Baez, "Visualizing the Hopf Fibration"](https://math.ucr.edu/home/baez/hopf.pdf)

### Tensores e Machine Learning
- **Deep Learning**: Goodfellow, Bengio, Courville - "Deep Learning" (MIT Press)
- **Tensor Analysis**: Synge & Schild - "Tensor Calculus"
- **Neural Networks**: Bishop - "Pattern Recognition and Machine Learning"

### Relativity & Cosmology
- **General Relativity**: Misner, Thorne, Wheeler - "Gravitation" (MTW)
- **Cosmology**: Ryden - "Introduction to Cosmology"
- **Spacetime Physics**: Taylor & Wheeler - "Spacetime Physics"
- **Mathematical Methods**: Schutz - "A First Course in General Relativity"
- **Planck Results**: Planck Collaboration 2018 - "Cosmological Parameters"

---

## Status & Roadmap

### ✅ Completed (v0.2.0)
- **avila-math**: Mathematical kernel (26 tests)
- **avila-telemetry**: Time series & analytics (22 tests)
- **LISA Pipeline**: Complete Phases 0-6 (39 tests)
- **Physics Modules**: GW, relativity, cosmology, lensing (62 tests)
- **Total**: 101 tests passing

### 🚧 In Progress
- **avila-vision**: Computer vision integration (YOLO, tracking)
- **AvilaDB Integration**: Database backend for catalogs
- **Performance optimization**: SIMD, parallel processing

### 🔮 Future Work
- **LISA Data Challenge**: Official LDC participation
- **Real LIGO Data**: Integration with GWOSC datasets
- **GPU Acceleration**: CUDA/ROCm for FFT and matched filtering
- **Distributed Computing**: Multi-node MCMC and template bank generation
- **AVL Cloud Deploy**: Production deployment on Avila Cloud Platform

---

## Contributing

Contributions are welcome! Please:
1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

For major changes, please open an issue first to discuss what you would like to change.

---

## Citation

If you use Arxis in your research, please cite:

```bibtex
@software{arxis2024,
  author = {Ávila, Nicolas},
  title = {Arxis: The Mathematical Citadel - Research-grade Physics \& Mathematics in Rust},
  year = {2024},
  url = {https://github.com/avilaops/arxis},
  version = {0.2.0},
  note = {ARX (fortress) + AXIS (engine) = ARXIS}
}
```

---

## Licença

Dual-licensed under MIT OR Apache-2.0 - See [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE) for details.

---

## 🏛️ Built by Avila

**Arxis** - *The Mathematical Citadel*
Developed with ❤️ for the scientific community

🏛️ **Solid as a fortress**
⚙️ **Powerful as an engine**
🚀 **Built in Rust**

Contact: nicolas@avila.inc
GitHub: [@avilaops](https://github.com/avilaops)
Platform: [avila.cloud](https://avila.cloud)
