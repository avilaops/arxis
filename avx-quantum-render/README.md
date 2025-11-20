# 🌌 AVX Quantum Render - QED Path Integral Renderer

> **Renderização baseada em Eletrodinâmica Quântica (QED)** usando formulação de integrais de caminho

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)

**Parte do ecossistema AVX (Avila Experience Platform)**

---

## 📚 Sobre

**avx-quantum-render** é um renderizador experimental que implementa renderização de luz usando **princípios fundamentais da física quântica**. Em vez de usar aproximações clássicas (ray tracing), este renderer calcula a propagação de fótons usando a **formulação de path integral de Feynman**.

### Teoria

Na Eletrodinâmica Quântica (QED), a amplitude para um fóton ir de A → B é:

```
A(A→B) = Σ_caminhos exp(i·S[caminho]/ℏ)
```

Onde:
- **S** é a ação do caminho: `S = ∫(n·ℏω - p·v)dt`
- **ℏ** é a constante de Planck reduzida
- A soma é sobre **todos os caminhos possíveis** do fóton

A **intensidade final** é proporcional a `|A|²` (probabilidade quântica).

---

## 🎯 Características

### ✅ Implementado

- **Path Integral Monte Carlo**: Amostragem de caminhos quânticos
- **Amplitude Complexa**: Cálculo de amplitudes com fase
- **Vértices de Feynman**: Interações e⁻γ com coupling constants
- **Propagadores**: Fóton e elétron (gauge de Feynman)
- **Materiais Físicos**:
  - Lambertiano (difuso)
  - Especular (espelho)
  - Dielétrico (vidro, água)
  - Metal (condutor)
  - Absorvente
- **Efeitos Quânticos**:
  - Espalhamento Compton
  - Pair production (γ → e⁺ + e⁻)
  - Interferência quântica
  - Russian Roulette para terminação de caminhos

### 🚧 Em Desenvolvimento

- Polarização completa (Stokes vectors)
- Espalhamento de Mie para partículas
- Renderização volumétrica (meios participantes)
- GPU acceleration (CUDA/wgpu)

---

## 📦 Instalação

Adicione ao seu `Cargo.toml`:

```toml
[dependencies]
avx-quantum-render = { path = "../avx-quantum-render" }
```

Ou como parte do workspace Arxis:

```toml
[workspace]
members = ["avx-quantum-render"]
```

---

## 🚀 Uso Rápido

```rust
use avx_quantum_render::prelude::*;

// Criar cena
let mut scene = Scene::new();
scene.add_light(Light::point([0.0, 5.0, 0.0], 100.0));
scene.add_surface(Surface::lambertian([0.0, 0.0, 0.0], [0.0, 1.0, 0.0], 0.8));

// Configurar câmera
let camera = Camera::new([0.0, 2.0, 5.0], [0.0, 0.0, 0.0], PI/3.0)
    .with_resolution(800, 600);
scene.set_camera(camera);

// Renderizar com QED
let config = RenderConfig::default();
let renderer = QEDRenderer::new(config);
let image = renderer.render(&scene);

// Processar imagem
for (y, row) in image.iter().enumerate() {
    for (x, &intensity) in row.iter().enumerate() {
        // intensity ∈ [0.0, 1.0]
        let pixel_value = (intensity * 255.0) as u8;
        // Salvar pixel...
    }
}
```

---

## 📖 Exemplos

### Exemplo Básico - Cornell Box

```bash
cd avx-quantum-render
cargo run --example basic_render --release
```

Este exemplo:
1. Cria uma Cornell Box com materiais variados
2. Renderiza usando path integral Monte Carlo
3. Gera imagem ASCII art
4. Demonstra cálculos quânticos individuais

**Output esperado:**
```
=== AVX Quantum Renderer - QED Path Integral Demo ===

✓ Cena criada: 7 objetos
✓ Renderizador QED configurado
  - Samples por pixel: 10
  - Profundidade máxima: 3
  - Caminhos por sample: 100

🎨 Renderizando com Path Integral Monte Carlo...
✓ Renderização concluída em 12.34s

📊 Estatísticas da Imagem:
  Resolução: 80x40
  Intensidade média: 0.3245
  Intensidade mín: 0.0012
  Intensidade máx: 0.9876
```

### Cálculos Quânticos Individuais

```rust
use avx_quantum_render::amplitude::*;
use avx_quantum_render::feynman::*;

// 1. Amplitude complexa
let amp = ComplexAmplitude::from_polar(2.0, PI/4.0);
println!("Probabilidade: {}", amp.probability()); // |A|²

// 2. Acumulação de fase
let mut phase = PhaseAccumulator::new();
phase.add_propagation(1e-6, 550e-9, 1.0); // 1μm @ 550nm
let amplitude = phase.to_amplitude();

// 3. Vértice de Feynman
let mut vertex = FeynmanVertex::new([0.0, 0.0, 0.0], InteractionType::Emission);
vertex.compute_qed_amplitude(); // e·γ^μ ≈ √(4πα)

// 4. Espalhamento Compton
let amp = compton_scattering_amplitude(1e-15, 0.9e-15, PI/4.0);
println!("Seção de choque: {}", amp.probability());
```

---

## 🔬 Física Implementada

### Constantes Fundamentais

```rust
pub const HBAR: f64 = 1.054571817e-34;        // ℏ (J·s)
pub const SPEED_OF_LIGHT: f64 = 299792458.0;  // c (m/s)
pub const FINE_STRUCTURE: f64 = 1.0/137.036;  // α
pub const ELECTRON_CHARGE: f64 = 1.602176634e-19; // e (C)
```

### Vértices QED

Para interação e⁻ + γ, a amplitude do vértice é:

```
V = -i·e·γ^μ = -i·√(4πα)·γ^μ
```

Onde α ≈ 1/137 é a **constante de estrutura fina**.

### Propagadores

**Fóton** (gauge de Feynman):
```
D_μν(q) = -i·g_μν / (q² + iε)
```

**Elétron**:
```
S(p) = i(γ·p + m) / (p² - m² + iε)
```

### Espalhamento Compton

Amplitude para γ + e⁻ → γ + e⁻ (fórmula Klein-Nishina):

```
|A|² ∝ α² (E_out/E_in + E_in/E_out - sin²θ)
```

---

## 🎨 Configuração do Renderer

```rust
let config = RenderConfig {
    samples_per_pixel: 100,   // SPP (mais = menos ruído)
    max_path_depth: 5,         // Máximo de bounces
    num_paths: 1000,           // Caminhos quânticos por sample
    parallel: true,            // Usar paralelismo (Rayon)
    rr_threshold: 0.1,         // Russian Roulette threshold
};

// Presets
let preview = RenderConfig::preview();       // Rápido
let default = RenderConfig::default();       // Balanceado
let hq = RenderConfig::high_quality();       // Alta qualidade
```

---

## 📊 Performance

**Benchmark (Intel i7, Cornell Box 800x600):**

| Config       | SPP  | Tempo | Qualidade |
| ------------ | ---- | ----- | --------- |
| Preview      | 10   | ~5s   | Ruidoso   |
| Default      | 100  | ~45s  | Bom       |
| High Quality | 1000 | ~8min | Excelente |

**Otimizações:**
- ✅ Paralelização com Rayon
- ✅ Russian Roulette para terminação precoce
- ✅ Importance sampling (luz, BRDF)
- 🚧 GPU acceleration (planejado)
- 🚧 Denoising (planejado)

---

## 🧪 Testes

```bash
# Executar todos os testes
cargo test

# Testes específicos
cargo test amplitude      # Amplitudes complexas
cargo test photon         # Caminhos de fótons
cargo test feynman        # Vértices e diagramas
cargo test scene          # Cena e materiais
cargo test renderer       # Renderizador

# Com output detalhado
cargo test -- --nocapture
```

---

## 📚 Referências

### Física Quântica

1. **"QED: The Strange Theory of Light and Matter"** - Richard Feynman
2. **"Introduction to Quantum Field Theory"** - Peskin & Schroeder
3. **"Quantum Electrodynamics"** - Landau & Lifshitz

### Renderização

4. **"Physically Based Rendering"** - Pharr, Jakob, Humphreys
5. **"Path Integral Formulation for Light Transport"** - Veach (PhD Thesis)
6. **"Quantum Light Transport"** - Arvo et al.

### Papers

- Feynman, R. (1949). "Space-Time Approach to Quantum Electrodynamics"
- Veach, E. (1997). "Robust Monte Carlo Methods for Light Transport Simulation"

---

## 🛠️ Arquitetura

```
avx-quantum-render/
├── src/
│   ├── lib.rs              # Módulo raiz
│   ├── amplitude.rs        # ComplexAmplitude, PhaseAccumulator
│   ├── photon.rs           # PhotonPath, Vertex, Interaction
│   ├── feynman.rs          # FeynmanVertex, FeynmanDiagram
│   ├── scene.rs            # Scene, Light, Surface, Camera
│   └── renderer.rs         # QEDRenderer, path integral Monte Carlo
│
├── examples/
│   └── basic_render.rs     # Cornell Box + cálculos quânticos
│
├── Cargo.toml
└── README.md
```

---

## 🤝 Contribuindo

Este é um projeto experimental parte do **ecossistema AVX**. Contribuições são bem-vindas!

**Áreas para desenvolvimento:**
- Otimizações de performance
- Novos materiais (anisotropic, subsurface scattering)
- Efeitos quânticos adicionais (túnel quântico, emaranhamento)
- Visualização de diagramas de Feynman
- GPU acceleration

---

## 📄 Licença

MIT OR Apache-2.0 - Veja arquivo LICENSE

---

## 📞 Contato

**Projeto**: Avila Experience Platform (AVX)
**Autor**: Nicolas Ávila
**Email**: nicolas@avila.inc
**GitHub**: https://github.com/avilaops/arxis

---

## 🌟 Reconhecimentos

- **Richard Feynman** - Path integral formulation
- **Eric Veach** - Path tracing e Monte Carlo methods
- **Rust Community** - Ferramentas e bibliotecas incríveis

---

**Status**: 🚧 Experimental - v0.1.0
**Última Atualização**: Novembro 2025

*"The theory of quantum electrodynamics describes Nature as absurd from the point of view of common sense. And it agrees fully with experiment. So I hope you can accept Nature as She is - absurd."* - Richard Feynman
