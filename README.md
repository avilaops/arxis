# 🚀 Arxis - Advanced Physics & Mathematics Library

> **Research-grade Rust library for General Relativity, Gravitational Waves, and Advanced Mathematics**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![Tests](https://img.shields.io/badge/tests-77%20passing-brightgreen.svg)](https://github.com/avilaops/arxis)

**🌌 NASA/LISA Mission Ready** - See [NASA_LISA_INTEGRATION.md](NASA_LISA_INTEGRATION.md) for collaboration opportunities.

---

## 📞 Contact

**Project Lead**: Nicolas Ávila
**Email**: nicolas@avila.inc
**WhatsApp**: +55 17 99781-1471
**GitHub**: https://github.com/avilaops/arxis

---

## ☁️ AVL Cloud Platform

**Arxis** é desenvolvido nativamente para a **AVL (Avila Cloud Platform)** - nossa infraestrutura proprietária de computação científica de alta performance.

- **Avx Computing**: Processamento paralelo otimizado para cálculos tensoriais e simulações astrofísicas
- **AVL Storage**: Armazenamento distribuído para grandes volumes de dados científicos (ondas gravitacionais, imagens astronômicas)
- **Avila ML**: Pipeline de machine learning integrado com suporte nativo a tensores 4D
- **AVL Deploy**: Deployment automatizado de modelos científicos e APIs de física computacional

*Diferente de AWS, Azure ou GCP, a AVL é otimizada especificamente para cargas de trabalho científicas e de engenharia.*

---

Biblioteca Rust completa incluindo **física relativística** (ondas gravitacionais, buracos negros, cosmologia), quaternions, tensores generalizados e **geometria 4D**.

## Características Principais

### 🌊 Física de Ondas Gravitacionais (NASA/LIGO/LISA)
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

**Total: 39 tests passing across all LISA modules**

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
- **Strong lensing**: Múltiplas imagens, anéis de Einstein
- **Weak lensing**: Shear, convergência, cosmic shear
- **Microlensing**: Curvas de luz, detecção de exoplanetas
- **Modelos**: Point Mass, SIS, NFW

### 🌌 Cosmologia
- **Modelo FLRW**: Universo em expansão (Planck 2018)
- **Distâncias**: Luminosa, angular, comóvel
- **Evolução**: H(z), q(z), idade do universo
- **Observáveis**: CMB, supernovas, estrutura em larga escala

### 🔄 Quaternions 3D (`Quat3D`)
- Representação de rotações em 3D usando álgebra de quaternions
- Operações: multiplicação, conjugado, inverso, normalização
- Conversão de/para ângulos de Euler e eixo-ângulo
- Rotação de vetores 3D
- Interpolação esférica (SLERP)
- Conversão para matriz de rotação 3×3

### 🌀 Quaternions Duplos (`DualQuat`)
- Representação de rotações e translações combinadas
- Suporte para transformações rígidas (rigid body transformations)
- Interpolação linear de dual quaternions (DLB)
- Extração de componentes de rotação e translação

### 🔮 SO(4) - Rotações 4D (`SO4Rotation`)
- Representação isomórfica ao produto S³ × S³
- Rotações independentes "left" e "right"
- Aplicação de rotação usando a fórmula: **q₁ × v × q₂\***
- Composição de rotações SO(4)
- Conversão para matriz de rotação 4×4
- Decomposição isoclínica

### 📐 Geometria 4D (`geometry4d`)
- **Espaço Euclidiano ℝ⁴**: Pontos, vetores, distâncias, produto escalar
- **Transformações 4D**: Matrizes 4×4 com rotações em 6 planos (XY, XZ, YZ, XW, YW, ZW)
- **Politopos Regulares 4D**:
  - Tesserato (Hipercubo): 16 vértices, 32 arestas, 24 faces, 8 células
  - 24-Cell: 24 vértices, 96 arestas (autodual, sem análogo 3D!)
  - Simplex 4D (5-cell): 5 vértices, 10 arestas
- **Projeções 4D→3D**: Ortográfica, perspectiva, estereográfica
- **Visualização ASCII**: Renderizador 3D para projeções de objetos 4D
- **Cinemática 4D**: Corpos rígidos com posição, velocidade e rotação em 4D

### 📊 Tensores Generalizados
- **Ordem 0**: Escalares
- **Ordem 1**: Vetores (produto escalar, norma, produto vetorial)
- **Ordem 2**: Matrizes (multiplicação, determinante, traço, transposição)
- **Ordem 3**: Tensores 3D (convolução 3D, fatias 2D)
- **Ordem 4**: Tensores 4D (batch de imagens, contração, produto externo)

### 🧠 Machine Learning
- Operações de batch processing
- Funções de ativação: ReLU, Sigmoid, Tanh
- Pooling: Max pooling, Average pooling
- Convolução 2D
- Batch normalization
- Operações para backpropagation

### 🌌 Relatividade Geral
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
rand = "0.8"  # necessário para dropout
```

## Teoria Matemática

### Tensores Generalizados
Um tensor de ordem (rank) N é uma generalização de:
- **Ordem 0**: Escalar (número)
- **Ordem 1**: Vetor (array unidimensional)
- **Ordem 2**: Matriz (array bidimensional)
- **Ordem 3**: Tensor 3D (cubos de dados)
- **Ordem 4**: Tensor 4D (hipercubos, batches de imagens)

## Exemplos de Uso

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

Para ver exemplos detalhados de todas as funcionalidades:

```bash
# Física: Aplicações práticas integradas (LIGO, LISA, Cosmologia)
cargo run --example practical_astrophysics

# NASA/ESA LISA Mission (Space-based gravitational waves)
cargo run --example lisa_example

# Ondas gravitacionais
cargo run --example gravitational_example

# Lentes gravitacionais
cargo run --example lensing_example

# Cosmologia
cargo run --example cosmology_example

# Geodésicas e órbitas
cargo run --example geodesic_example

# Métricas de Einstein
cargo run --example einstein_example

# Quaternions e rotações 3D/4D
cargo run --example rotations_example

# Tensores generalizados
cargo run --example tensors_example

# Relatividade geral
cargo run --example relativity_example

# Machine learning
cargo run --example ml_example

# Geometria 4D e visualizações
cargo run --example geometry4d_example
```

## Testes

Execute os testes unitários:

```bash
cargo test
```

## Estrutura do Projeto

```
arxis_quaternions/
├── lib.rs                      # Módulo principal
├── quaternion3d.rs             # Quaternions 3D
├── dual_quaternion.rs          # Quaternions duplos e SO(4)
├── tensor.rs                   # Tensores generalizados (ordem 0-2)
├── tensor4d.rs                 # Tensores 3D e 4D
├── relativity.rs               # Relatividade geral
├── geometry4d.rs               # Geometria 4D e politopos
├── examples/
│   ├── rotations_example.rs    # Exemplos de quaternions
│   ├── tensors_example.rs      # Exemplos de tensores
│   ├── relativity_example.rs   # Exemplos de relatividade
│   ├── ml_example.rs           # Exemplos de ML
│   └── geometry4d_example.rs   # Exemplos de geometria 4D
├── TENSOR_DOCUMENTATION.md     # Documentação de tensores
├── GEOMETRY4D_GUIDE.md         # Guia completo de geometria 4D
└── Cargo.toml
```

## Aplicações

Esta biblioteca é útil para:

### ☁️ AVL Cloud & Avx Computing
- **Pipelines de análise em AVL**: Processamento distribuído de dados LIGO/LISA
- **Avx Tensor Processing**: Aceleração de operações tensoriais para ML
- **AVL Scientific Compute**: Simulações de N-corpos e evolução cosmológica
- **Avila Data Lake**: Armazenamento e query de dados astronômicos em petabyte-scale

### 🚀 Astrofísica e NASA/LISA
- **Análise de dados LIGO/Virgo/KAGRA** (ondas gravitacionais terrestres)
- **Preparação para missão LISA** (ondas gravitacionais espaciais)
- **Modelagem de fontes**: SMBHs, EMRIs, binários galácticos
- **Estimativa de parâmetros**: massas, distâncias, spins
- **LISA Data Challenge** participation
- **Pulsar timing** (PSR B1913+16, NANOGrav)

### 🔭 Cosmologia Observacional
- Supernovas Tipo Ia (velas padrão)
- Lentes gravitacionais (Einstein Cross, arcos)
- Distâncias cosmológicas (redshift → Mpc)
- Evolução do universo (H₀, q₀, idade)
- Standard sirens (GW + EM)

### Computação Gráfica e Animação
- Rotações suaves e interpolação de câmera
- Transformações de corpos rígidos
- Visualização de objetos 4D (tesseracts, 24-cell, etc.)
- Cinemática inversa
- Projeções de 4D para 3D/2D

### Geometria 4D e Visualização
- Estudo de politopos regulares (tesserato, 24-cell, simplex 4D)
- Rotações em 6 planos independentes (XY, XZ, YZ, XW, YW, ZW)
- Projeções perspectivas, ortográficas e estereográficas
- Renderização ASCII de objetos 4D
- Simulação de física em 4 dimensões

### Machine Learning e Deep Learning
- Redes neurais convolucionais (CNNs)
- Processamento de imagens em batch
- Camadas fully connected
- Operações de pooling e normalização

### Física e Relatividade
- Transformações de Lorentz
- Cálculos de intervalos espaço-temporais
- Tensor energia-momento
- Curvatura do espaço-tempo

### Processamento de Imagens
- Filtros convolucionais
- Detecção de bordas
- Redimensionamento de imagens
- Data augmentation

### Robótica
- Cinemática e controle de orientação
- Planejamento de trajetórias
- Manipulação de transformações espaciais

### Matemática e Geometria
- Estudo de grupos de Lie (SO(3), SO(4))
- Álgebra geométrica em 4D
- Geometria diferencial
- Análise tensorial

## Referências

### Quaternions
- **Quaternions e Rotações**: [Ken Shoemake, "Animating Rotation with Quaternion Curves"](https://www.cs.cmu.edu/~kiranb/animation/p245-shoemake.pdf)
- **Dual Quaternions**: [Kavan et al., "Skinning with Dual Quaternions"](https://users.cs.utah.edu/~ladislav/kavan07skinning/kavan07skinning.pdf)
- **SO(4) e S³×S³**: [John Baez, "Visualizing the Hopf Fibration"](https://math.ucr.edu/home/baez/hopf.pdf)

### Tensores e Machine Learning
- **Deep Learning**: Goodfellow, Bengio, Courville - "Deep Learning" (MIT Press)
- **Tensor Analysis**: Synge & Schild - "Tensor Calculus"
- **Neural Networks**: Bishop - "Pattern Recognition and Machine Learning"

### Relatividade Geral
- **General Relativity**: Misner, Thorne, Wheeler - "Gravitation"
- **Spacetime Physics**: Taylor & Wheeler - "Spacetime Physics"
- **Mathematical Methods**: Schutz - "A First Course in General Relativity"

## Licença

MIT License

## Autor

Avila Framework / Arxis
