# AvilaDB DataFrame 🚀

**Revolutionary DataFrame library for scientific computing with native astrophysics support**

[![License: Apache-2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![Status](https://img.shields.io/badge/status-in%20development-yellow.svg)]()

> **Status**: 🚧 Em desenvolvimento ativo. Core funcional, features avançadas em progresso.

## 🎯 Missão

Criar a biblioteca de DataFrame **mais rápida e poderosa do mundo** para computação científica, com capacidades únicas que ninguém mais tem - especialmente otimizada para **pesquisa brasileira** (LIGO-BR, LISA, INPE) e integrada nativamente com **AvilaDB**.

## ⚡ Por Que AvilaDB DataFrame?

### 🏆 Diferenciais Únicos (Ninguém Tem Isso!)

1. **⚛️ Quaternions Nativos** - Rotações 3D/4D para espaçonaves, robótica, física
2. **🌌 Tensor4D** - Dados espaço-temporais (x, y, z, t) nativos
3. **🔬 Weyl Spinors** - Física de partículas built-in
4. **🪐 Coordenadas Geodésicas** - Relatividade geral computacional
5. **🎵 FFT/Wavelets Built-in** - Processamento de sinal sem deps externas
6. **⚡ GPU-First** - Aceleração CUDA/ROCm transparente
7. **🇧🇷 Brazilian Cloud Native** - Integrado com AvilaDB (40-60% mais barato)

### 📊 Targets de Performance

| Operação | Polars | Pandas | **avila-dataframe** |
|----------|--------|--------|---------------------|
| Group By (100M rows) | 2.3s | 45s | **< 1.5s** ⚡ |
| Join (10M × 10M) | 1.8s | 120s | **< 1.0s** ⚡ |
| FFT (1M samples) | N/A | 3.2s | **< 0.5s** ⚡ |
| Parquet read (10GB) | 8.5s | N/A | **< 6.0s** ⚡ |

## 🚀 Quick Start

### Instalação

```toml
[dependencies]
avila-dataframe = "0.1"

# Ou com todas as features
avila-dataframe = { version = "0.1", features = ["full"] }
```

### Exemplo Básico

```rust
use avila_dataframe::prelude::*;

fn main() -> Result<()> {
    // Criar DataFrame com dados de ondas gravitacionais (LIGO/LISA)
    let df = DataFrame::new(vec![
        Series::new("timestamp", vec![0.0, 0.001, 0.002]),
        Series::new("strain_h", vec![1.2e-21, 1.5e-21, 1.1e-21]),
        Series::new("snr", vec![8.5, 12.3, 9.1]),
    ])?;

    // Query fluente
    let result = df
        .filter(col("snr") > 10.0)?
        .with_column((col("strain_h") * lit(1e21)).alias("scaled"))?
        .select(&["timestamp", "scaled"])?;

    println!("{}", result);
    Ok(())
}
```

### Executar Exemplos

```powershell
# Exemplo básico
cargo run --example basic_usage

# Tipos científicos (Quaternions, Spinors, Geodésicas)
cargo run --example scientific_types

# Todos os exemplos
.\scripts\run-examples.ps1
```

## 🔬 Features Científicas

### Tipos de Dados Únicos

```rust
// Quaternions para rotações 3D
let q = Quaternion::from_axis_angle([0.0, 0.0, 1.0], PI/4.0);
let rot_matrix = q.normalize().to_rotation_matrix();

// Spinors de Weyl para física de partículas
let spinor = SpinorWeyl::new(Complex::new(1.0, 0.0), Complex::new(0.0, 1.0));
let boosted = spinor.boost(0.5); // Lorentz boost

// Coordenadas geodésicas (Relatividade Geral)
let coord = GeodesicCoord::new(0.0, 10.0, PI/2.0, 0.0);
let gtt = coord.schwarzschild_gtt(1.0); // Métrica de Schwarzschild
```

### Operações Científicas (Em Progresso)

```rust
// FFT & Análise Espectral
df.fft("signal", window=WindowType::Hann)?
  .power_spectral_density()?;

// Wavelets
df.wavelet_transform("strain", wavelet="morlet", scales=128)?;

// Astronomia
df.redshift_correction("wavelength", z=0.5)?
  .luminosity_distance("redshift")?;
```

## 📁 Estrutura do Projeto

```
avila-dataframe/
├── src/
│   ├── core/          # DataFrame, Series, tipos científicos
│   ├── ops/           # Operações (filter, group_by, join)
│   ├── io/            # Parquet, CSV, Arrow, HDF5
│   ├── lazy/          # Lazy evaluation
│   ├── scientific/    # FFT, wavelets, astronomy
│   └── gpu/           # CUDA/ROCm acceleration
├── examples/          # Exemplos de uso
├── benches/           # Benchmarks vs Polars
├── scripts/           # Scripts de desenvolvimento
└── tests/             # Testes de integração
```

## 🛠️ Desenvolvimento

### Build

```powershell
# Build padrão
cargo build

# Build release com todas as features
cargo build --release --all-features

# Script completo (build + testes)
.\scripts\build.ps1
```

### Testes

```powershell
# Todos os testes
cargo test --all-features

# Teste específico
cargo test test_dataframe_creation
```

### Benchmarks

```powershell
# Executar benchmarks
.\scripts\benchmark.ps1

# Resultados em target/criterion/report/index.html
```

### Checks de Qualidade

```powershell
# Formatar, lint, testar, docs
.\scripts\check.ps1

# Auto-fix de problemas
.\scripts\check.ps1 -Fix
```

## 📚 Documentação

- [**Guia de Desenvolvimento**](DEVELOPMENT.md) - Roadmap detalhado e arquitetura
- [**Como Contribuir**](CONTRIBUTING.md) - Guia para contribuidores
- [**Changelog**](CHANGELOG.md) - Histórico de mudanças
- [**Exemplos**](./examples/) - Código de exemplo
- [**API Docs**](https://docs.rs/avila-dataframe) - Documentação da API

## 🎯 Casos de Uso

### Astrofísica & Astronomia
- 🌌 **LIGO/LISA** - Análise de ondas gravitacionais
- 🛰️ **Telescópios** - Processamento de dados astronômicos
- ⭐ **Catálogos estelares** - Redshift, distância, magnitudes

### Ciência de Dados
- 📊 **Séries temporais** - Finanças, IoT, sensores
- 🤖 **Machine Learning** - Feature engineering, treino/teste
- 📈 **Analytics** - Agregações, pivots, joins

### Física & Engenharia
- ⚛️ **Física de partículas** - Análise de colisões, spinors
- 🚀 **Aeroespacial** - Orientação quaternion, trajetórias
- 🤖 **Robótica** - Rotações 3D, cinemática

### Pesquisa Brasileira
- 🇧🇷 **LIGO-BR** - Detector de ondas gravitacionais brasileiro
- 🛰️ **INPE** - Dados climáticos e ambientais
- 🔬 **Universidades** - Pesquisa científica nacional

## 🤝 Contribuindo

Contribuições são muito bem-vindas! Veja [CONTRIBUTING.md](CONTRIBUTING.md) para:

- Como configurar o ambiente
- Padrões de código
- Como submeter PRs
- Issues e features prioritárias

### Issues Prioritárias

- [ ] Implementar filter/group_by completo
- [ ] Joins (inner, left, right, outer)
- [ ] I/O Parquet (leitura/escrita)
- [ ] FFT com rustfft
- [ ] Aceleração GPU

## 📊 Status do Projeto

### ✅ Completo
- [x] Estrutura base do projeto
- [x] Core DataFrame/Series com Arrow
- [x] Tipos científicos (Quaternion, SpinorWeyl, GeodesicCoord)
- [x] Sistema de expressões
- [x] Operações básicas (select, with_column)
- [x] Estatísticas básicas (mean, std, sum)
- [x] Exemplos funcionais

### 🚧 Em Progresso
- [ ] Filter/group_by implementation
- [ ] I/O Parquet/CSV
- [ ] Lazy evaluation
- [ ] SQL engine

### 🔮 Planejado
- [ ] FFT/Wavelets
- [ ] GPU acceleration
- [ ] Distributed computing
- [ ] Python bindings

## 📜 Licença

Apache License 2.0 - Veja [LICENSE](LICENSE)

## 🔗 Links

- [**AvilaDB**](https://avila.cloud/aviladb) - Database nativo integrado
- [**AVL Platform**](https://avila.cloud) - Plataforma cloud brasileira
- [**Documentação**](https://docs.avila.cloud/dataframe)

---

**Construído com 🇧🇷 no Brasil pela AVL Cloud Platform**

*Destruindo a concorrência, um DataFrame por vez* 🔥

---

## **Especificações Técnicas Completas (Para Referência)**

## **Requisitos Core Avançados:**

### **1. Storage Engine de Última Geração**
- **Columnar storage** com Apache Arrow como base
- **Compression**: Zstd, LZ4, Snappy, Delta encoding, RLE
- **Zero-copy**: Compartilhamento de memória via Arrow IPC
- **Memory-mapped I/O**: Datasets maiores que RAM
- **Partitioning**: Horizontal (sharding) e vertical (column pruning)
- **Lazy evaluation**: Query optimizer antes de executar

### **2. Tipos de Dados Científicos Nativos**
```rust
pub enum DType {
    // Básicos
    Int8, Int16, Int32, Int64, UInt8, UInt16, UInt32, UInt64,
    Float32, Float64, Complex64, Complex128,
    Bool, String, Binary,

    // Avançados (nosso diferencial)
    Quaternion,           // Rotações 3D/4D
    Tensor4D(Shape),      // Dados espaço-temporais
    SpinorWeyl,           // Física de partículas
    Geodesic,             // Coordenadas em variedades curvas
    TimeSeries(TimeZone), // Séries temporais com timezone
    Categorical(Dict),    // Categóricos otimizados
    Nested(Schema),       // DataFrames aninhados
    Graph(GraphType),     // Grafos como coluna
}
```

### **3. Query Engine Revolucionário**
```rust
// SQL completo + extensions científicas
df.sql(r#"
    SELECT
        timestamp,
        strain_h,
        FFT(strain_h, window='hann') as spectrum,
        DETECT_ANOMALIES(strain_h, method='zscore') as anomalies,
        WAVELET_TRANSFORM(strain_h, wavelet='morlet') as cwt
    FROM lisa_data
    WHERE abs(strain_h) > 1e-21
    ORDER BY timestamp
    LIMIT 1000
"#)?;

// Query builder fluente
df.filter(col("mass1") > 30.0)
  .with_column(col("mass1") + col("mass2"), "total_mass")
  .group_by(&["event_type"])
  .agg(&[
      col("snr").mean().alias("mean_snr"),
      col("snr").std().alias("std_snr"),
      col("distance").median().alias("median_distance"),
  ])
  .sort("mean_snr", SortOrder::Desc)?;
```

### **4. Operações Científicas Built-in**
```rust
// FFT/Spectral Analysis
df.fft("signal", window=WindowType::Hann)?
  .power_spectral_density()?
  .spectrogram(nperseg=256)?;

// Wavelets
df.wavelet_transform("strain", wavelet="morlet", scales=128)?;

// Signal Processing
df.filter_butterworth("signal", cutoff=10.0, order=4)?
  .resample("1ms", method=ResampleMethod::Linear)?
  .rolling_window(100).apply(|w| w.std())?;

// Statistical Tests
df.kolmogorov_smirnov("distribution1", "distribution2")?;
df.chi_square_test("observed", "expected")?;
df.anderson_darling("sample")?;

// Time Series
df.autocorrelation("signal", max_lag=100)?
  .cross_correlation("signal1", "signal2")?
  .seasonal_decompose(period=365)?;

// Astronomy/Astrophysics
df.redshift_correction("wavelength", z=0.5)?
  .luminosity_distance("redshift")?
  .angular_separation("ra1", "dec1", "ra2", "dec2")?;
```

### **5. Machine Learning Integration**
```rust
// Train/Test Split
let (train, test) = df.train_test_split(0.8, stratify="label")?;

// Feature Engineering
df.one_hot_encode(&["category1", "category2"])?
  .standardize(&["mass1", "mass2", "distance"])?
  .polynomial_features(&["x", "y"], degree=3)?
  .target_encode("category", "target")?;

// Feature Selection
df.mutual_information("features", "target")?
  .correlation_matrix(method=CorrelationMethod::Spearman)?
  .variance_threshold(0.1)?;

// Native avila-ml integration
let model = df.train_linear_regression("target", features)?;
let predictions = df.predict(model)?;
```

### **6. Distributed Computing (Big Data)**
```rust
// Dask/Spark-like distributed processing
use avila_dataframe::distributed::*;

let cluster = Cluster::connect("avila://cluster:8786")?;

let df = LazyFrame::scan_parquet("s3://bucket/lisa_data/*.parquet")?
    .filter(col("snr") > 8.0)
    .group_by("event_type")
    .agg(col("mass1").mean())
    .collect_distributed(&cluster)?;

// Streaming para dados infinitos (LISA real-time)
let stream = Stream::connect("kafka://lisa-topic")?
    .window(Duration::seconds(60))
    .aggregate(|window| window.mean("strain_h"))
    .write_to("aviladb://anomalies")?;
```

### **7. I/O Avançado**
```rust
// Formatos suportados
df.write_parquet("data.parquet", compression=Compression::Zstd)?;
df.write_arrow_ipc("data.arrow")?;
df.write_hdf5("data.h5", dataset="lisa")?;
df.write_fits("data.fits")?;  // Astronomy standard
df.write_netcdf("data.nc")?;  // Climate science
df.write_aviladb("aviladb://collection")?;  // Native

// Streaming I/O
df.scan_csv_chunked("huge.csv", chunk_size=1_000_000)?
  .process_chunks(|chunk| {
      chunk.filter(col("valid") == true)?.collect()
  })?;

// Cloud-native
df.read_s3("s3://bucket/data.parquet")?;
df.read_azure("azure://container/data.parquet")?;
df.read_gcs("gs://bucket/data.parquet")?;
```

### **8. Visualização Integrada**
```rust
// Plots direto do DataFrame
df.plot()
  .scatter("mass1", "mass2", color="snr")
  .title("Black Hole Mergers")
  .save("plot.png")?;

df.plot().histogram("snr", bins=50)?;
df.plot().heatmap("correlation_matrix")?;
df.plot().time_series("timestamp", "strain_h")?;
```

### **9. GPU Acceleration (CUDA/ROCm)**
```rust
// Operações em GPU
df.to_gpu()?
  .filter(col("x") > 0.0)  // Executa na GPU
  .group_by("category")
  .agg(col("value").sum())
  .to_cpu()?;  // Volta pra CPU

// cuDF compatibility
let cudf = df.to_cudf()?;
let rapids_result = cudf.rapids_operation()?;
```

### **10. Query Optimization Avançado**
- **Predicate pushdown**: Filtros aplicados no scan
- **Projection pushdown**: Lê só colunas necessárias
- **Join reordering**: Otimiza ordem de joins
- **Common subexpression elimination**: Remove cálculos duplicados
- **Vectorized execution**: SIMD automático
- **Adaptive query execution**: Muda estratégia em runtime

---

## **Arquitetura:**
```
avila-dataframe/
├── src/
│   ├── lib.rs
│   ├── core/
│   │   ├── dataframe.rs
│   │   ├── series.rs
│   │   ├── chunked_array.rs
│   │   ├── dtype.rs
│   │   └── schema.rs
│   ├── lazy/
│   │   ├── logical_plan.rs
│   │   ├── physical_plan.rs
│   │   ├── optimizer.rs
│   │   └── executor.rs
│   ├── io/
│   │   ├── parquet.rs
│   │   ├── arrow.rs
│   │   ├── csv.rs
│   │   ├── hdf5.rs
│   │   ├── fits.rs
│   │   └── aviladb.rs
│   ├── ops/
│   │   ├── filter.rs
│   │   ├── group_by.rs
│   │   ├── join.rs
│   │   ├── pivot.rs
│   │   └── window.rs
│   ├── scientific/
│   │   ├── fft.rs
│   │   ├── wavelets.rs
│   │   ├── signal.rs
│   │   ├── stats.rs
│   │   └── astronomy.rs
│   ├── distributed/
│   │   ├── cluster.rs
│   │   ├── scheduler.rs
│   │   └── streaming.rs
│   ├── sql/
│   │   ├── parser.rs
│   │   ├── planner.rs
│   │   └── extensions.rs
│   ├── gpu/
│   │   ├── cuda.rs
│   │   └── rocm.rs
│   └── prelude.rs
```

---

## **Benchmarks Obrigatórios (deve ser mais rápido):**
```
| Operation            | Polars | Pandas | avila-dataframe |
| -------------------- | ------ | ------ | --------------- |
| Group By (100M rows) | 2.3s   | 45s    | < 1.5s          |
| Join (10M x 10M)     | 1.8s   | 120s   | < 1.0s          |
| FFT (1M samples)     | N/A    | 3.2s   | < 0.5s (SIMD)   |
| Parquet read (10GB)  | 8.5s   | N/A    | < 6.0s          |
```

---

## **Diferenciais Únicos (que ninguém tem):**

1. **Tensor4D como dtype nativo** - Datasets 4D nativos
2. **Quaternions/Spinors** - Física avançada
3. **FFT/Wavelets built-in** - Sem deps externas
4. **Geodesic coordinates** - Relatividade geral
5. **AvilaDB native** - Integração zero-copy
6. **SQL científico** - Funções astronomy/physics no SQL
7. **Streaming real-time** - LISA data ingestion
8. **GPU por padrão** - Transparente CUDA/ROCm

**Essa biblioteca vai destruir a concorrência. Crie com tudo isso!**" 🚀
