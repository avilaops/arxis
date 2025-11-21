# avila-convexa3d

**Processamento de dados 3D em Rust puro 🎬**

Biblioteca 100% Rust para processamento de dados tridimensionais: vídeos (sequências temporais) e volumes (dados espaciais 3D). Parte do ecossistema **Arxis** para computação científica de alto desempenho.

[![Crates.io](https://img.shields.io/crates/v/avila-convexa3d)](https://crates.io/crates/avila-convexa3d)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](./LICENSE)

## 🎯 Características

### 📹 Processamento de Vídeo (Temporal)
- **VideoFrame**: Wrapper para frames individuais com timestamp
- **Video**: Gerenciamento de sequências de frames com FPS configurável
- **VideoProcessor**:
  - Estatísticas temporais (média, desvio padrão, energia)
  - Diferença entre frames consecutivos
  - Conversão para escala de cinza
  - Extração de sub-vídeos

### 🎬 Optical Flow e Motion Detection
- **OpticalFlow**:
  - Block matching para estimativa de movimento
  - Gradient-based (Lucas-Kanade simplificado)
  - FlowField com vetores de movimento 2D
- **MotionDetector**:
  - Detecção por diferença de frames
  - Energia de movimento temporal
  - Identificação de frames com movimento significativo

### 🧊 Volumes 3D (Espacial)
- **Volume3D**: Dados 4D (profundidade × altura × largura × canais)
- **VolumeProcessor**:
  - Estatísticas espaciais (média, desvio padrão)
  - Slicing ao longo de eixos (X, Y, Z)
  - Threshold e binarização
  - Downsampling 3D
  - Cálculo de gradiente 3D
  - Geração de formas geométricas (esferas)

### 🔧 Filtros e Convolução 3D
- **ConvolutionKernel3D**: Kernels 3×3×3 e 5×5×5
  - Média 3D
  - Gaussiano 3D
  - Laplaciano 3D
  - Sobel 3D (X, Y, Z)
- **Filter3D**: Aplicação eficiente de filtros
  - Gaussian blur 3D
  - Edge detection 3D
  - Filtro de média

### 📐 Geometria 3D
- **Point3D**: Pontos com distâncias euclidianas e Manhattan
- **Size3D**: Dimensões com cálculo de volume
- **BoundingBox3D**: Volumes delimitadores com testes de contenção e interseção

## 🚀 Instalação

```toml
[dependencies]
avila-convexa3d = "0.1"
```

## 📚 Exemplos

### Processamento de Vídeo

```rust
use avila_convexa3d::video::VideoProcessor;

// Criar vídeo de teste (100x100, 30 frames, 30 fps)
let video = VideoProcessor::create_test_video(100, 100, 30, 30.0);

// Estatísticas temporais
let mean = VideoProcessor::temporal_mean(&video);
let energy = VideoProcessor::temporal_energy(&video);

// Converter para escala de cinza
let gray_video = VideoProcessor::to_grayscale(&video);

// Extrair sub-vídeo
let subvideo = video.subvideo(10, 20);
```

### Optical Flow

```rust
use avila_convexa3d::motion::{OpticalFlow, MotionDetector};

let frame1 = video.get_frame(0).unwrap();
let frame2 = video.get_frame(1).unwrap();

// Block matching
let flow = OpticalFlow::block_matching(frame1, frame2, 10, 5);
println!("Magnitude média: {:.4}", flow.average_magnitude());

// Gradient-based
let flow_grad = OpticalFlow::gradient_based(frame1, frame2);

// Detecção de movimento
let motion_mask = MotionDetector::frame_difference_threshold(frame1, frame2, 30.0);
```

### Volumes 3D

```rust
use avila_convexa3d::volume::{Volume3D, VolumeProcessor};
use avila_convexa3d::common::Axis3D;

// Criar volume de teste
let volume = VolumeProcessor::create_test_volume(50, 50, 50);

// Estatísticas espaciais
let mean = VolumeProcessor::spatial_mean(&volume, 0);
let std = VolumeProcessor::spatial_std(&volume, 0, mean);

// Extrair slice 2D
let slice = volume.slice_at(Axis3D::Z, 25, 0);

// Threshold
let binary = VolumeProcessor::threshold(&volume, 0.5);

// Downsampling
let downsampled = VolumeProcessor::downsample(&volume, 2);
```

### Filtros 3D

```rust
use avila_convexa3d::filters::Filter3D;
use avila_convexa3d::volume::VolumeProcessor;

let sphere = VolumeProcessor::create_sphere(10);

// Gaussian blur
let blurred = Filter3D::gaussian_blur(&sphere, 0);

// Detecção de bordas (Laplaciano)
let edges = Filter3D::laplacian(&sphere, 0);

// Sobel X
let sobel = Filter3D::sobel_x(&sphere, 0);
```

## 🏗️ Arquitetura

```
avila-convexa3d/
├── src/
│   ├── lib.rs           # API principal
│   ├── common.rs        # Point3D, Size3D, BoundingBox3D, Axis3D
│   ├── video.rs         # VideoFrame, Video, VideoProcessor
│   ├── volume.rs        # Volume3D, VolumeProcessor
│   ├── filters.rs       # ConvolutionKernel3D, Filter3D
│   ├── motion.rs        # OpticalFlow, MotionDetector
│   └── main.rs          # Exemplo demonstrativo
├── Cargo.toml           # Metadados do pacote
├── README.md            # Este arquivo
└── LICENSE              # Licença MIT
```

## 🧪 Testes

```bash
cargo test
```

**27 testes passando** cobrindo:
- Geometria 3D (4 testes)
- Vídeo e processamento temporal (6 testes)
- Volumes e operações espaciais (7 testes)
- Filtros e convolução 3D (4 testes)
- Optical flow e motion detection (6 testes)

## 📊 Estatísticas

- **~2000 linhas** de código Rust puro
- **27 testes unitários** (100% passando)
- **Zero dependências unsafe**
- **5 módulos especializados**

## 🔗 Dependências

- `ndarray`: Arrays multidimensionais eficientes
- `serde`: Serialização de estruturas
- `serde_json`: JSON support
- `num-traits`: Operações numéricas genéricas

## 📖 Casos de Uso

### Visão Computacional
- Análise de sequências de vídeo
- Tracking de objetos
- Estimativa de movimento
- Detecção de atividades

### Medical Imaging
- Processamento de volumes CT/MRI
- Segmentação 3D
- Análise de estruturas anatômicas
- Registro de imagens

### Computação Científica
- Visualização de dados 3D
- Simulações físicas
- Análise de fluidos
- Processamento de dados atmosféricos

## 🎓 Família Convexa

- **[avila-convexa1d](https://crates.io/crates/avila-convexa1d)**: Dados sequenciais 1D (áudio, texto)
- **[avila-convexa2d](https://crates.io/crates/avila-convexa2d)**: Dados bidimensionais (imagens, matrizes)
- **avila-convexa3d**: Dados tridimensionais (vídeos, volumes) ← você está aqui

## 🏢 Ecossistema Arxis

Parte do **Arxis**, framework de computação científica de alto desempenho da AVL Cloud Platform:

- 🎵 **avila-convexa1d**: Processamento 1D
- 🖼️ **avila-convexa2d**: Processamento 2D
- 🎬 **avila-convexa3d**: Processamento 3D
- 🔢 **avila-math**: Matemática numérica
- 📊 **avila-dataframe**: Manipulação de dados
- 🤖 **avila-ml**: Machine learning

## 📄 Licença

MIT License - veja [LICENSE](./LICENSE) para detalhes.

## 👨‍💻 Autor

Desenvolvido com ❤️ pela equipe da **AVL Cloud Platform** 🇧🇷

Para mais informações, visite: [avila.cloud](https://avila.cloud)

---

**avila-convexa3d** - Processamento 3D genuíno da AVL Cloud Platform! 🎬🧊
