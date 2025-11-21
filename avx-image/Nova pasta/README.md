# Sistema de Reconhecimento Facial - Física e Matemática

Sistema demonstrativo em Rust que implementa os fundamentos físicos e matemáticos do reconhecimento facial.

## 📐 Fundamentos Físicos

### 1. Formação da Imagem (Óptica Geométrica)

#### Modelo Pinhole Camera
A câmera é modelada como uma câmera pinhole, onde pontos 3D são projetados no plano da imagem:

```
x' = f × (x / z)
y' = f × (y / z)
```

Onde:
- `f` = distância focal
- `(x, y, z)` = coordenadas 3D do ponto
- `(x', y')` = coordenadas 2D na imagem
- `z` = profundidade

**Implementação**: `optics::Camera::project_point()`

#### Equação de Reflectância (Lambertian)

A intensidade da luz refletida pela pele segue o modelo Lambertiano:

```
I(x,y) = ρ × (n · l) × E
```

Onde:
- `I(x,y)` = intensidade no pixel
- `ρ` = albedo da superfície (0.6-0.7 para pele)
- `n` = vetor normal da superfície
- `l` = vetor direção da luz
- `E` = irradiância da fonte de luz

**Implementação**: `optics::calculate_irradiance()`

#### Modelo de Reflectância Phong

Para superfícies com brilho (componente especular):

```
I = Ia×ka + Il×(kd×(n·l) + ks×(r·v)ⁿ)
```

Onde:
- `Ia` = luz ambiente
- `ka` = coeficiente ambiente
- `kd` = coeficiente difuso
- `ks` = coeficiente especular
- `n` = expoente de Phong (shininess)
- `r` = vetor refletido
- `v` = vetor de visão

**Implementação**: `optics::calculate_phong_reflection()`

### 2. Lei da Reflexão

```
r = d - 2(d·n)n
```

Onde `d` é o vetor incidente e `n` é a normal da superfície.

---

## 🔢 Fundamentos Matemáticos

### 1. Geometria Diferencial

#### Normal da Superfície
Calculada usando produto vetorial de dois vetores tangentes:

```
n = (p₁ - p₀) × (p₂ - p₀)
n̂ = n / ||n||
```

**Implementação**: `geometry::compute_surface_normal()`

#### Curvatura Gaussiana
Medida da curvatura da superfície:

```
K = κ₁ × κ₂
```

Onde `κ₁` e `κ₂` são as curvaturas principais.

**Implementação**: `geometry::estimate_curvature()`

#### Curvatura Média (usando Laplaciano)

```
H = (1/2)Δx = (1/2n) Σ(xᵢ - x₀)
```

**Implementação**: `geometry::mean_curvature()`

### 2. Processamento de Imagem

#### Gradientes (Filtros de Sobel)

```
∇I = [∂I/∂x, ∂I/∂y]ᵀ
```

Kernels:
```
Gₓ = [-1  0  1]     Gᵧ = [-1 -2 -1]
     [-2  0  2]          [ 0  0  0]
     [-1  0  1]          [ 1  2  1]
```

**Implementação**: `features::compute_gradients()`

#### HOG (Histogram of Oriented Gradients)

1. Calcula magnitude: `m = √(gₓ² + gᵧ²)`
2. Calcula orientação: `θ = arctan(gᵧ/gₓ)`
3. Cria histograma de orientações (9 bins, 0-180°)

**Implementação**: `features::compute_hog_features()`

#### LBP (Local Binary Patterns)

```
LBP(xc,yc) = Σₚ₌₀⁷ s(gₚ - gc)2ᵖ
```

Onde:
```
s(x) = { 1, se x ≥ 0
       { 0, caso contrário
```

**Implementação**: `features::compute_lbp_histogram()`

#### Filtros de Gabor (Análise de Frequência)

```
G(x,y;λ,θ,ψ,σ,γ) = exp(-(x'²+γ²y'²)/(2σ²)) × cos(2πx'/λ + ψ)
```

Onde:
- `x' = x cos θ + y sin θ` (rotação)
- `y' = -x sin θ + y cos θ`
- `λ` = comprimento de onda
- `θ` = orientação
- `σ` = desvio padrão
- `γ` = aspect ratio

**Implementação**: `features::compute_gabor_response()`

### 3. Reconhecimento (PCA - Eigenfaces)

#### Principal Component Analysis

1. **Face Média**:
```
μ = (1/n)Σᵢ xᵢ
```

2. **Centralização**:
```
Φᵢ = xᵢ - μ
```

3. **Matriz de Covariância**:
```
C = (1/n)ΣᵢΦᵢΦᵢᵀ
```

4. **Decomposição Espectral**:
```
C = UΛUᵀ
```
Onde `U` contém os autovetores (eigenfaces) e `Λ` os autovalores.

5. **Projeção**:
```
y = Uᵀ(x - μ)
```

**Implementação**: `recognition::FaceRecognizer::train_pca()`

#### Métricas de Distância

**Euclidiana**:
```
d = ||x - y|| = √Σᵢ(xᵢ - yᵢ)²
```

**Cosseno**:
```
sim = (x·y)/(||x|| ||y||)
dist = 1 - sim
```

**Mahalanobis**:
```
d = √[(x-y)ᵀΣ⁻¹(x-y)]
```

**Implementação**: `recognition::euclidean_distance()`, `cosine_distance()`, `mahalanobis_distance()`

### 4. Alinhamento de Procrustes

Minimiza:
```
Σᵢ ||R×s×pᵢ + t - qᵢ||²
```

Onde:
- `R` = matriz de rotação
- `s` = fator de escala
- `t` = vetor de translação

**Implementação**: `geometry::procrustes_alignment()`

---

## 🔬 Bibliotecas Utilizadas

| Biblioteca       | Função                                             |
| ---------------- | -------------------------------------------------- |
| `nalgebra`       | Álgebra linear (vetores, matrizes, transformações) |
| `ndarray`        | Arrays N-dimensionais (imagens como matrizes)      |
| `image`          | Carregamento/salvamento de imagens                 |
| `imageproc`      | Processamento (filtros, detecção)                  |
| `rustfft`        | Transformada de Fourier                            |
| `ndarray-linalg` | Decomposição SVD, eigenvalues                      |
| `ndarray-stats`  | Estatísticas                                       |
| `plotters`       | Visualização                                       |

---

## 🚀 Como Executar

```bash
# Build
cargo build --release

# Run
cargo run --release

# Tests
cargo test
```

---

## 📊 Arquitetura do Sistema

```
main.rs
├── optics.rs          → Física: formação de imagem, reflectância
├── geometry.rs        → Geometria: normais, curvatura, distâncias
├── features.rs        → Features: HOG, LBP, Gabor
└── recognition.rs     → Reconhecimento: PCA, matching
```

---

## 🎯 Pipeline de Reconhecimento

```
1. CAPTURA
   ↓ [Camera Model + Optics]

2. PRÉ-PROCESSAMENTO
   ↓ [Gradientes, Normalização]

3. EXTRAÇÃO DE FEATURES
   ↓ [HOG + LBP + Gabor]

4. REDUÇÃO DE DIMENSIONALIDADE
   ↓ [PCA → Eigenfaces]

5. MATCHING
   ↓ [Distance Metrics]

6. IDENTIFICAÇÃO
   └→ Person ID + Confidence
```

---

## 📈 Exemplo de Saída

```
=== Sistema de Reconhecimento Facial ===

1. FÍSICA - Formação da Imagem
  Simulando captura de imagem por câmera...
  ✓ Posição 3D: (0.0, 0.0, 500.0) mm
  ✓ Projeção 2D: pixel (960, 540)
  ✓ Irradiância: 0.534 W/m²
  ✓ Cor RGB: (0.46, 0.35, 0.29)

  Equação de Reflectância (Lambert):
  I(x,y) = ρ × (n · l) × E
  onde:
    ρ = 0.65 (albedo)
    n · l = 0.820 (produto escalar normal-luz)
    E = 1.0 (irradiância)

2. GEOMETRIA - Análise de Superfície Facial
  ✓ Distância entre olhos: 60.0 mm
  ✓ Distância nariz-boca: 30.8 mm
  ✓ Normal do nariz: (0.000, 0.000, -1.000)
  ✓ Curvatura gaussiana: 0.012458

3. MATEMÁTICA - Extração de Features
  ✓ HOG features: 324 dimensões
  ✓ LBP histogram: 256 bins
  ✓ Gabor wavelets: 4096 coeficientes

4. RECONHECIMENTO - Sistema Completo
  ✓ Database: 5 pessoas, 15 amostras
  ✓ PCA treinado: 20 eigenfaces
  ✓ Pessoa identificada: 2
  ✓ Confiança: 78.3%
```

---

## 🔍 Conceitos Avançados

### Transformada de Fourier 2D

```
F(u,v) = ∫∫ f(x,y)e^(-i2π(ux+vy)) dx dy
```

Análise de frequências espaciais na imagem.

### Convolução

```
(f * g)(x,y) = ∫∫ f(τ,σ)g(x-τ,y-σ) dτ dσ
```

Base para filtros (Sobel, Gaussian, Gabor).

### Teorema de Nyquist-Shannon

```
fs ≥ 2×fmax
```

Frequência de amostragem deve ser pelo menos o dobro da frequência máxima.

---

## 📚 Referências

1. **Turk & Pentland (1991)** - Eigenfaces for Recognition
2. **Ahonen et al. (2006)** - Face Description with LBP
3. **Dalal & Triggs (2005)** - HOG for Human Detection
4. **Phong (1975)** - Illumination for Computer Generated Pictures
5. **Hartley & Zisserman** - Multiple View Geometry

---

## 🎓 Para Aprender Mais

- **Física**: Óptica geométrica, radiometria, fotometria
- **Geometria**: Geometria diferencial, topologia de superfícies
- **Álgebra Linear**: SVD, PCA, autovalores/autovetores
- **Análise de Fourier**: Transformadas, análise de frequência
- **Machine Learning**: Classificação, métricas de distância

---

**Desenvolvido para demonstração educacional dos fundamentos de reconhecimento facial** 🔬📐
