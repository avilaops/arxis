# Equações Fundamentais do Reconhecimento Facial

## 📸 Física da Formação da Imagem

### 1. Projeção Perspectiva

A transformação de um ponto 3D `P = (X, Y, Z)` no espaço para um ponto 2D `p = (x, y)` na imagem:

```
┌─────┐     ┌─────────────┐   ┌───┐
│ x' │     │ f   0    0  │   │ X │
│ y' │  =  │ 0   f    0  │ × │ Y │
│ z' │     │ 0   0    1  │   │ Z │
└─────┘     └─────────────┘   └───┘

x = x'/z' = fX/Z
y = y'/z' = fY/Z
```

**Variáveis:**
- `f` = distância focal (mm)
- `Z` = profundidade do objeto (mm)
- Maior `f` → maior zoom
- Maior `Z` → objeto mais distante aparece menor

---

### 2. Modelo de Iluminação Lambertiana

Superfícies difusas (como pele) seguem a lei de Lambert:

```
L = ρ × (n̂ · l̂) × E₀

onde:
ρ = albedo da superfície [0,1]
n̂ = vetor normal unitário da superfície
l̂ = vetor unitário na direção da luz
E₀ = irradiância da fonte (W/m²)
```

**Interpretação Física:**
- `(n̂ · l̂)` = cos(θ), onde θ é o ângulo entre normal e luz
- θ = 0° (luz perpendicular) → máxima iluminação
- θ = 90° (luz paralela) → sem iluminação
- θ > 90° → superfície não iluminada

**Valores típicos de albedo:**
| Material     | Albedo (ρ) |
| ------------ | ---------- |
| Pele clara   | 0.65-0.75  |
| Pele média   | 0.50-0.65  |
| Pele escura  | 0.35-0.50  |
| Papel branco | 0.90       |
| Carvão       | 0.04       |

---

### 3. Modelo de Phong (Reflexão Especular)

Para superfícies brilhantes (olhos, suor):

```
I = Iₐkₐ + Σᵢ [Iᵢ(kd(n̂·l̂ᵢ) + kₛ(r̂ᵢ·v̂)ⁿ)]

Componentes:
• Iₐkₐ        = iluminação ambiente
• Iᵢkd(n̂·l̂ᵢ) = reflexão difusa (Lambertiana)
• Iᵢkₛ(r̂ᵢ·v̂)ⁿ = reflexão especular (brilho)

r̂ = 2(n̂·l̂)n̂ - l̂  (vetor refletido)
```

**Parâmetros:**
- `kₐ` ∈ [0,1]: coeficiente ambiente
- `kd` ∈ [0,1]: coeficiente difuso
- `kₛ` ∈ [0,1]: coeficiente especular
- `n` ∈ [1,∞): expoente de Phong (shininess)
  - n=1 → brilho difuso
  - n=10 → brilho moderado
  - n=100+ → brilho metálico

---

## 📐 Geometria Diferencial

### 4. Normal de Superfície

Dado três pontos não-colineares `P₀, P₁, P₂`:

```
v₁ = P₁ - P₀
v₂ = P₂ - P₀

n = v₁ × v₂  (produto vetorial)

n̂ = n/‖n‖   (normalização)
```

**Produto Vetorial em 3D:**
```
       │ i    j    k  │
v₁×v₂ =│ v₁ₓ  v₁ᵧ  v₁ᵤ│ = i(v₁ᵧv₂ᵤ - v₁ᵤv₂ᵧ)
       │ v₂ₓ  v₂ᵧ  v₂ᵤ│   - j(v₁ₓv₂ᵤ - v₁ᵤv₂ₓ)
                         + k(v₁ₓv₂ᵧ - v₁ᵧv₂ₓ)
```

---

### 5. Curvatura de Superfície

**Curvatura Gaussiana** (K):
```
K = κ₁ × κ₂

onde κ₁ e κ₂ são as curvaturas principais
```

**Interpretação:**
- `K > 0`: superfície elíptica (como uma esfera)
- `K = 0`: superfície cilíndrica ou plana
- `K < 0`: superfície hiperbólica (como uma sela)

**Curvatura Média** (H):
```
H = (κ₁ + κ₂)/2

Aproximação discreta (Laplaciano):
H ≈ (1/2n)Σⁿᵢ₌₁(Pᵢ - P₀)
```

**Aplicação:** Faces têm regiões com diferentes curvaturas:
- Nariz: K > 0 (convexa)
- Olhos (órbita): K < 0 (côncava)
- Testa: K ≈ 0 (quase plana)

---

## 🔍 Processamento de Imagem

### 6. Gradientes (Derivadas Parciais)

```
∇I(x,y) = [∂I/∂x, ∂I/∂y]ᵀ

Magnitude: |∇I| = √((∂I/∂x)² + (∂I/∂y)²)

Direção: θ = arctan(∂I/∂y, ∂I/∂x)
```

**Aproximação por Diferenças Finitas:**
```
∂I/∂x ≈ (I(x+1,y) - I(x-1,y))/2
∂I/∂y ≈ (I(x,y+1) - I(x,y-1))/2
```

**Filtro de Sobel (mais robusto ao ruído):**
```
Gₓ = 1/8 × [-1  0  1]     Gᵧ = 1/8 × [-1 -2 -1]
            [-2  0  2]                 [ 0  0  0]
            [-1  0  1]                 [ 1  2  1]
```

---

### 7. Local Binary Patterns (LBP)

Para cada pixel central `gc` e seus 8 vizinhos `gp`:

```
LBP(x,y) = Σ⁷ₚ₌₀ s(gp - gc)×2ᵖ

onde s(x) = { 1, se x ≥ 0
            { 0, se x < 0
```

**Exemplo:**
```
Vizinhança:        Threshold:       Binário:
50  60  70         0  1  1          0×2⁷ + 1×2⁶ + 1×2⁵ +
40 [55] 80  →      0 [55] 1   →     0×2⁴ + 1×2³ + 1×2² +
30  45  75         0  0  1          0×2¹ + 0×2⁰ + 1×2⁰

LBP = 64 + 32 + 8 + 4 + 1 = 109
```

**LBP Uniforme:** Padrões com no máximo 2 transições 0→1 ou 1→0
- Reduz 256 padrões → 59 padrões uniformes
- Mais robustos e significativos

---

### 8. Filtro de Gabor

```
G(x,y;λ,θ,ψ,σ,γ) = exp(-(x'²+γ²y'²)/(2σ²)) × cos(2πx'/λ + ψ)

Rotação:
x' = x cos(θ) + y sin(θ)
y' = -x sin(θ) + y cos(θ)
```

**Parâmetros:**
- `λ`: comprimento de onda (pixels) → controla frequência
- `θ`: orientação (radianos) → direção das "listras"
- `ψ`: fase (radianos) → deslocamento da onda
- `σ`: desvio padrão → largura do envelope gaussiano
- `γ`: aspect ratio → alongamento do envelope

**Banco de Filtros de Gabor:**
```
Frequências: λ ∈ {4, 8, 16, 32} pixels
Orientações: θ ∈ {0°, 45°, 90°, 135°}
Total: 4×4 = 16 filtros
```

---

## 🧮 Álgebra Linear para Reconhecimento

### 9. PCA (Principal Component Analysis)

**Objetivo:** Reduzir dimensionalidade preservando variância

**Passos:**

1. **Centralização dos dados:**
```
X̃ = X - μ
onde μ = (1/n)Σⁿᵢ₌₁ xᵢ (média)
```

2. **Matriz de Covariância:**
```
C = (1/n)X̃ᵀX̃ = (1/n)Σⁿᵢ₌₁(xᵢ-μ)(xᵢ-μ)ᵀ

Dimensão: C ∈ ℝᵈˣᵈ (d = número de pixels)
```

3. **Decomposição Espectral:**
```
C = UΛUᵀ

U = [u₁ u₂ ... ud]  (autovetores)
Λ = diag(λ₁, λ₂, ..., λd)  (autovalores)

onde λ₁ ≥ λ₂ ≥ ... ≥ λd ≥ 0
```

4. **Projeção (Eigenfaces):**
```
Seleciona k << d componentes principais:
Uk = [u₁ u₂ ... uk]

Projeta nova face:
y = Uᵏᵀ(x - μ) ∈ ℝᵏ
```

**Variância Explicada:**
```
Proporção = Σᵏᵢ₌₁λᵢ / Σᵈᵢ₌₁λᵢ

Típico: k=50 explica >95% da variância
```

---

### 10. Métricas de Distância

#### Distância Euclidiana
```
d_E(x,y) = ‖x - y‖₂ = √(Σⁿᵢ₌₁(xᵢ - yᵢ)²)
```

**Propriedades:**
- Simétrica: d(x,y) = d(y,x)
- Não-negativa: d(x,y) ≥ 0
- Identidade: d(x,x) = 0
- Desigualdade triangular: d(x,z) ≤ d(x,y) + d(y,z)

#### Distância de Cosseno
```
sim(x,y) = (x·y)/(‖x‖‖y‖) = cos(θ)

d_cos(x,y) = 1 - sim(x,y) = 1 - cos(θ)
```

**Vantagem:** Invariante à magnitude (só considera direção)

#### Distância de Mahalanobis
```
d_M(x,y) = √((x-y)ᵀΣ⁻¹(x-y))

onde Σ = matriz de covariância
```

**Vantagem:** Considera correlações entre features

---

### 11. Transformada de Procrustes

Alinha dois conjuntos de pontos `{pᵢ}` e `{qᵢ}` minimizando:

```
E = Σⁿᵢ₌₁‖s·R·pᵢ + t - qᵢ‖²

Solução:
1. Centroides: p̄ = (1/n)Σpᵢ, q̄ = (1/n)Σqᵢ

2. Matriz H: H = Σⁿᵢ₌₁(pᵢ-p̄)(qᵢ-q̄)ᵀ

3. SVD: H = UΣVᵀ

4. Rotação ótima: R = VUᵀ

5. Escala: s = tr(RH)/tr(PPᵀ)

6. Translação: t = q̄ - s·R·p̄
```

---

## 📊 Métricas de Avaliação

### 12. Matriz de Confusão

```
                  Predito
                Pos    Neg
Real  Pos  │   TP  │  FN  │
      Neg  │   FP  │  TN  │
```

**Métricas derivadas:**

```
Acurácia = (TP + TN)/(TP + TN + FP + FN)

Precisão = TP/(TP + FP)

Recall = TP/(TP + FN)

F1-Score = 2×(Precisão×Recall)/(Precisão+Recall)
```

**Para reconhecimento facial:**
- **FAR** (False Accept Rate) = FP/(FP+TN)
- **FRR** (False Reject Rate) = FN/(FN+TP)
- **EER** (Equal Error Rate) = ponto onde FAR = FRR

---

## 🎯 Pipeline Matemático Completo

```
Imagem Raw (1920×1080×3)
    ↓
Conversão Grayscale: Y = 0.299R + 0.587G + 0.114B
    ↓ (1920×1080)
Detecção de Face (Viola-Jones/CNN)
    ↓ (crop + align)
Face Alinhada (128×128)
    ↓ (16,384 pixels)
Extração de Features:
  • HOG: 324 dimensões
  • LBP: 59 dimensões
  • Gabor: 32 dimensões
    ↓ (415 dimensões)
PCA (Eigenfaces): y = Uᵏᵀ(x-μ)
    ↓ (50 dimensões)
Classificação: argminᵢ d(y, yᵢ)
    ↓
Person ID + Confidence
```

---

## 🔬 Constantes Físicas Relevantes

| Constante                      | Valor               | Uso                 |
| ------------------------------ | ------------------- | ------------------- |
| Velocidade da luz              | c = 3×10⁸ m/s       | Propagação EM       |
| Planck                         | h = 6.626×10⁻³⁴ J·s | Energia fóton: E=hf |
| Comprimento de onda (vermelho) | λ ≈ 700 nm          | Limite difração     |
| Comprimento de onda (azul)     | λ ≈ 450 nm          | Resolução óptica    |
| Índice refração (vidro)        | n ≈ 1.5             | Lentes              |

**Limite de Difração (critério de Rayleigh):**
```
θ_min = 1.22 λ/D

onde D = diâmetro da abertura
```

---

Desenvolvido como material educacional para física e matemática aplicada! 🎓
