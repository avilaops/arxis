# avila-convexa4d

**Processamento de dados 4D em Rust puro 🎯**

Biblioteca 100% Rust para processamento de dados tetradimensionais: tensores espaço-temporais, sequências de volumes, vídeos volumétricos. Parte do ecossistema **Arxis** para computação científica de alto desempenho.

[![Crates.io](https://img.shields.io/crates/v/avila-convexa4d)](https://crates.io/crates/avila-convexa4d)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](./LICENSE)

## 🎯 Características

### 🎯 Tensores 4D
- **Tensor4D**: Estrutura para dados 4D (tempo × profundidade × altura × largura × canais)
- **TensorOps**:
  - Média e desvio padrão espaço-temporais
  - Colapso temporal (média no tempo)
  - Colapso espacial (média no espaço)
  - Threshold e binarização 4D
  - Downsampling 4D com fator configurável

### 📹 Sequências de Volumes
- **VolumeFrame**: Frames volumétricos individuais com timestamp
- **VolumeSequence**: Gerenciamento de sequências temporais de volumes
- **VolumeSequenceProcessor**:
  - Criação de sequências de teste
  - Diferença temporal entre frames
  - Média temporal de sequências
  - Energia espaço-temporal
  - Conversão para Tensor4D

### 🔧 Filtros Espaço-Temporais 4D
- **ConvolutionKernel4D**: Kernels 3×3×3×3 para convolução 4D
  - Média 4D
  - Gaussiano 4D
  - Laplaciano 4D (bordas espaço-temporais)
- **Filter4D**: Aplicação eficiente de filtros
  - Blur espaço-temporal
  - Detecção de bordas em 4 dimensões

### ⚡ Processamento Espaço-Temporal
- **SpatioTemporalProcessor**:
  - Correlação temporal com lag configurável
  - Derivada temporal (gradiente no tempo)
  - Derivada espacial (gradiente em X, Y, Z)
  - Magnitude do gradiente 4D
- **MotionAnalyzer**:
  - Detecção de movimento volumétrico
  - Taxa de movimento em sequências
  - Detecção de transições abruptas

### 📐 Geometria 4D
- **Point4D**: Pontos 4D (t, z, y, x) com distâncias euclidianas e Manhattan
- **Size4D**: Dimensões 4D com cálculo de hipervolume e volume espacial
- **BoundingBox4D**: Hipercubos delimitadores com testes de contenção e interseção

## 🚀 Instalação

```toml
[dependencies]
avila-convexa4d = "0.1"
```

## 📚 Exemplos

### Tensores 4D

```rust
use avila_convexa4d::tensor::{Tensor4D, TensorOps};

// Criar tensor 4D (10 frames × 15 profundidade × 20 altura × 25 largura)
let tensor = TensorOps::create_test_tensor(10, 15, 20, 25);

// Estatísticas espaço-temporais
let mean = TensorOps::mean(&tensor, 0);
let std = TensorOps::std(&tensor, 0, mean);

// Colapso temporal (média ao longo do tempo)
let temporal_mean = TensorOps::temporal_mean(&tensor);

// Colapso espacial (média ao longo do espaço)
let spatial_mean = TensorOps::spatial_mean(&tensor);

// Threshold
let binary = TensorOps::threshold(&tensor, 0.5);

// Downsampling
let downsampled = TensorOps::downsample(&tensor, 2);
```

### Sequências de Volumes

```rust
use avila_convexa4d::sequence::VolumeSequenceProcessor;

// Criar sequência de volumes (8 frames, 10×10×10)
let sequence = VolumeSequenceProcessor::create_test_sequence(8, 10, 10, 10);

// Média temporal
let mean = VolumeSequenceProcessor::temporal_mean(&sequence);

// Energia espaço-temporal
let energy = VolumeSequenceProcessor::spatiotemporal_energy(&sequence);

// Converter para tensor 4D
let tensor = sequence.to_tensor();
```

### Filtros 4D

```rust
use avila_convexa4d::filters::{Filter4D, ConvolutionKernel4D};
use avila_convexa4d::tensor::TensorOps;

let tensor = TensorOps::create_test_tensor(7, 7, 7, 7);

// Filtro de média 4D
let mean_filtered = Filter4D::mean_filter(&tensor, 0);

// Gaussian blur 4D
let blurred = Filter4D::gaussian_blur(&tensor, 0);

// Detecção de bordas espaço-temporais
let edges = Filter4D::laplacian(&tensor, 0);
```

### Análise Espaço-Temporal

```rust
use avila_convexa4d::processor::{SpatioTemporalProcessor, MotionAnalyzer};

// Correlação temporal
let corr = SpatioTemporalProcessor::temporal_correlation(&tensor, 1, 0);

// Derivadas
let dt = SpatioTemporalProcessor::temporal_derivative(&tensor);
let dx = SpatioTemporalProcessor::spatial_derivative(&tensor, 2);

// Magnitude do gradiente 4D
let grad_mag = SpatioTemporalProcessor::gradient_magnitude_4d(&tensor);

// Análise de movimento
let motion_frames = MotionAnalyzer::detect_volumetric_motion(&sequence, 10.0);
let motion_rate = MotionAnalyzer::motion_rate(&motion_frames);
let transitions = MotionAnalyzer::detect_transitions(&sequence, 100.0);
```

## 🏗️ Arquitetura

```
avila-convexa4d/
├── src/
│   ├── lib.rs           # API principal
│   ├── common.rs        # Point4D, Size4D, BoundingBox4D, Axis4D
│   ├── tensor.rs        # Tensor4D, TensorOps
│   ├── sequence.rs      # VolumeFrame, VolumeSequence, VolumeSequenceProcessor
│   ├── filters.rs       # ConvolutionKernel4D, Filter4D
│   ├── processor.rs     # SpatioTemporalProcessor, MotionAnalyzer
│   └── main.rs          # Exemplo demonstrativo
├── Cargo.toml           # Metadados do pacote
├── README.md            # Este arquivo
└── LICENSE              # Licença MIT
```

## 🧪 Testes

```bash
cargo test
```

**26 testes passando** cobrindo:
- Geometria 4D (4 testes)
- Tensores 4D (6 testes)
- Sequências de volumes (5 testes)
- Filtros 4D (4 testes)
- Processamento espaço-temporal (7 testes)

## 📊 Estatísticas

- **~1600 linhas** de código Rust puro
- **26 testes unitários** (100% passando)
- **Zero dependências unsafe**
- **5 módulos especializados**

## 🔗 Dependências

- `ndarray`: Arrays multidimensionais eficientes (Array5 para 4D + canais)
- `serde`: Serialização de estruturas
- `serde_json`: JSON support
- `num-traits`: Operações numéricas genéricas

## 📖 Casos de Uso

### Medical Imaging 4D
- Sequências de CT/MRI ao longo do tempo
- Análise de perfusão cardíaca
- Monitoramento de tumores temporalmente
- Registro de imagens 4D

### Visão Computacional Avançada
- Vídeos volumétricos (hologramas, light fields)
- Tracking 3D + tempo
- Análise de movimento volumétrico
- Reconstrução 4D

### Computação Científica
- Simulações espaço-temporais
- Dinâmica de fluidos 4D
- Dados atmosféricos/oceanográficos temporais
- Análise de fenômenos físicos 4D

### Realidade Virtual/Aumentada
- Captura volumétrica em tempo real
- Streaming de volumes 3D
- Compressão espaço-temporal
- Processamento de point clouds temporais

## 🎓 Família Convexa

- **[avila-convexa1d](https://crates.io/crates/avila-convexa1d)**: Dados sequenciais 1D (áudio, texto)
- **[avila-convexa2d](https://crates.io/crates/avila-convexa2d)**: Dados bidimensionais (imagens, matrizes)
- **[avila-convexa3d](https://crates.io/crates/avila-convexa3d)**: Dados tridimensionais (vídeos, volumes)
- **avila-convexa4d**: Dados tetradimensionais (tensores espaço-temporais) ← você está aqui

## 🏢 Ecossistema Arxis

Parte do **Arxis**, framework de computação científica de alto desempenho da AVL Cloud Platform:

- 🎵 **avila-convexa1d**: Processamento 1D
- 🖼️ **avila-convexa2d**: Processamento 2D
- 🎬 **avila-convexa3d**: Processamento 3D
- 🎯 **avila-convexa4d**: Processamento 4D
- 🔢 **avila-math**: Matemática numérica
- 📊 **avila-dataframe**: Manipulação de dados
- 🤖 **avila-ml**: Machine learning

## 📄 Licença

MIT License - veja [LICENSE](./LICENSE) para detalhes.

## 👨‍💻 Autor

Desenvolvido com ❤️ pela equipe da **AVL Cloud Platform** 🇧🇷

Para mais informações, visite: [avila.cloud](https://avila.cloud)

---

**avila-convexa4d** - Processamento 4D genuíno da AVL Cloud Platform! 🎯✨
