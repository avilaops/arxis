# avila-linalg - Roadmap Técnico Detalhado

## v0.2.0 - Decomposições Fundamentais (Próxima Release)

### 1. SVD (Singular Value Decomposition) - PRIORIDADE ALTA

**Objetivo:** Decompor matriz M (m×n) em U Σ V^T

**Algoritmo a Implementar:** Golub-Reinsch (two-sided Jacobi como alternativa)

**Estrutura:**
```rust
pub struct SVD<T> {
    pub u: MatrixMxN<T>,           // m×m (ou m×min(m,n) versão thin)
    pub singular_values: Vec<T>,   // min(m,n) valores singulares
    pub vt: MatrixMxN<T>,          // n×n (ou min(m,n)×n versão thin)
}

impl<T: Float> MatrixMxN<T> {
    pub fn svd(&self) -> SVD<T> {
        // Implementação Golub-Reinsch
    }

    pub fn svd_thin(&self) -> SVD<T> {
        // Versão econômica (mais eficiente)
    }
}
```

**Passos:**
1. Bidiagonalização (via Householder reflections)
2. Diagonalização iterativa (Givens rotations)
3. Convergência quando off-diagonal < epsilon

**Testes:**
- [ ] Matriz 3×3 conhecida
- [ ] Matriz 5×3 (tall)
- [ ] Matriz 3×5 (wide)
- [ ] Reconstrução: M = U Σ V^T
- [ ] Valores singulares em ordem decrescente

**Aplicações:**
- PCA (análise de componentes principais)
- Pseudoinversa (Moore-Penrose)
- Compressão de imagens
- Sistemas least-squares

---

### 2. Eigenvalues/Eigenvectors - PRIORIDADE ALTA

**Objetivo:** Encontrar λ e v tais que Av = λv

**Algoritmos a Implementar:**

#### 2.1. Power Iteration (mais simples)
```rust
impl<T: Float> Matrix3x3<T> {
    pub fn dominant_eigenvalue(&self, max_iter: usize) -> (T, Vector3<T>) {
        // Retorna maior eigenvalue e seu eigenvector
    }
}
```

#### 2.2. QR Algorithm (mais completo)
```rust
pub struct EigenDecomposition<T> {
    pub eigenvalues: Vec<T>,
    pub eigenvectors: MatrixMxN<T>,  // Cada coluna é um eigenvector
}

impl<T: Float> MatrixMxN<T> {
    pub fn eigen(&self) -> EigenDecomposition<T> {
        // QR Algorithm com shifts
    }
}
```

**Passos (QR Algorithm):**
1. Reduzir a forma Hessenberg (Householder)
2. Iterar: A_k = Q_k R_k, A_{k+1} = R_k Q_k
3. Convergência quando triangular superior
4. Eigenvalues na diagonal

**Testes:**
- [ ] Matriz identidade (eigenvalues = 1, 1, 1)
- [ ] Matriz diagonal (eigenvalues = valores diagonais)
- [ ] Matriz simétrica 3×3
- [ ] Verificar: A v_i = λ_i v_i

**Aplicações:**
- PCA
- Análise de estabilidade
- Física quântica
- Processamento de sinais

---

### 3. QR Decomposition - PRIORIDADE MÉDIA

**Objetivo:** Decompor M = QR (Q ortogonal, R triangular superior)

**Algoritmos:**

#### 3.1. Gram-Schmidt (mais simples)
```rust
pub struct QR<T> {
    pub q: MatrixMxN<T>,  // Matriz ortogonal
    pub r: MatrixMxN<T>,  // Triangular superior
}

impl<T: Float> MatrixMxN<T> {
    pub fn qr(&self) -> QR<T> {
        // Gram-Schmidt modificado (mais estável)
    }
}
```

#### 3.2. Householder Reflections (mais estável)
```rust
impl<T: Float> MatrixMxN<T> {
    pub fn qr_householder(&self) -> QR<T> {
        // Mais estável numericamente
    }
}
```

**Testes:**
- [ ] Matriz 3×3
- [ ] Verificar Q^T Q = I (ortogonalidade)
- [ ] Verificar M = QR
- [ ] R é triangular superior

**Aplicações:**
- Resolver sistemas lineares
- Implementar eigenvalue algorithm
- Least squares

---

### 4. Operações Matriciais Adicionais

#### 4.1. Inversa de Matrix4x4
```rust
impl<T: Float> Matrix4x4<T> {
    pub fn inverse(&self) -> Option<Self> {
        // Via adjunta (similar a 3×3)
        // Ou via Gauss-Jordan
    }
}
```

#### 4.2. Resolver Sistemas Lineares
```rust
impl<T: Float> MatrixMxN<T> {
    /// Resolve Ax = b
    pub fn solve(&self, b: &[T]) -> Option<Vec<T>> {
        // Via LU decomposition ou QR
    }
}
```

**Testes:**
- [ ] Sistema 3×3 com solução única
- [ ] Sistema inconsistente (sem solução)
- [ ] Sistema indeterminado

---

## v0.3.0 - Performance & Otimizações

### 1. Paralelização com Rayon

**Feature flag:** `parallel`

```rust
#[cfg(feature = "parallel")]
use rayon::prelude::*;

impl<T: Float + Send + Sync> MatrixMxN<T> {
    pub fn mul_parallel(&self, other: &Self) -> Self {
        // Multiplicação paralela de matrizes
    }
}
```

**Otimizações:**
- [ ] Matrix multiplication paralela
- [ ] SVD paralelo (bidiagonalização)
- [ ] Operações vetoriais em lotes

### 2. SIMD Intrinsics

**Plataformas:** x86_64 (AVX2, AVX512), ARM (NEON)

```rust
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

impl Vector4<f32> {
    #[target_feature(enable = "avx2")]
    unsafe fn dot_simd(&self, other: &Self) -> f32 {
        // Produto escalar SIMD
    }
}
```

**Operações SIMD:**
- [ ] Dot product (4 floats paralelos)
- [ ] Matrix × Vector (4×4)
- [ ] Normalize (via rsqrt)

### 3. Benchmarks

```rust
// benches/matrix_ops.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use avila_linalg::{Matrix3x3, MatrixMxN};

fn bench_matrix_multiply(c: &mut Criterion) {
    let m1 = Matrix3x3::identity();
    let m2 = Matrix3x3::identity();

    c.bench_function("matrix3x3_mul", |b| {
        b.iter(|| black_box(m1 * m2))
    });
}
```

**Benchmarks a Criar:**
- [ ] Vector operations (dot, cross, norm)
- [ ] Matrix multiplication (3×3, 4×4, 100×100)
- [ ] SVD (diferentes tamanhos)
- [ ] Eigenvalues

---

## v0.4.0 - Integração & Validação

### 1. Substituir nalgebra no facial-recognition

**Arquivos a Modificar:**
- `optics.rs`: Vector3, Matrix3x3
- `geometry.rs`: Matrix operations
- `features.rs`: Convolução matricial
- `recognition.rs`: **PCA com nosso SVD!**

**Mudanças Críticas:**
```rust
// Antes (nalgebra)
use nalgebra::{DMatrix, DVector};
let svd = covariance_matrix.svd(true, true);

// Depois (avila-linalg)
use avila_linalg::MatrixMxN;
let svd = covariance_matrix.svd();
let eigenfaces = svd.u;  // Componentes principais
```

### 2. Validação com Dados Reais

**Datasets:**
- [ ] AT&T Face Database (400 imagens)
- [ ] LFW (Labeled Faces in the Wild)
- [ ] Synthetic data (variações controladas)

**Métricas:**
- [ ] Precisão do reconhecimento
- [ ] Tempo de treinamento
- [ ] Tempo de inferência
- [ ] Uso de memória

### 3. Comparação de Performance

| Operação          | avila-linalg | nalgebra | ndarray |
| ----------------- | ------------ | -------- | ------- |
| Matrix 3×3 mul    | ?            | ?        | ?       |
| SVD 100×100       | ?            | ?        | ?       |
| Eigenvalues 10×10 | ?            | ?        | ?       |
| Memory usage      | ?            | ?        | ?       |

---

## v0.5.0 - Decomposições Avançadas

### 1. LU Decomposition
```rust
pub struct LU<T> {
    pub l: MatrixMxN<T>,  // Lower triangular
    pub u: MatrixMxN<T>,  // Upper triangular
    pub p: Vec<usize>,    // Permutation vector
}
```

### 2. Cholesky Decomposition (matrizes definidas positivas)
```rust
impl<T: Float> MatrixMxN<T> {
    pub fn cholesky(&self) -> Option<MatrixMxN<T>> {
        // L L^T = A
    }
}
```

### 3. Schur Decomposition
```rust
pub struct Schur<T> {
    pub q: MatrixMxN<T>,  // Ortogonal
    pub t: MatrixMxN<T>,  // Triangular
}
```

---

## v1.0.0 - Produção Ready

### Checklist para Release

#### Funcionalidades
- [x] Vetores 2D/3D/4D/ND
- [x] Matrizes 2×2, 3×3, 4×4, M×N
- [ ] SVD
- [ ] Eigenvalues/Eigenvectors
- [ ] QR decomposition
- [ ] LU decomposition
- [ ] Inversa de matrizes (todas)
- [ ] Resolver sistemas lineares

#### Performance
- [ ] SIMD para operações críticas
- [ ] Paralelização com rayon
- [ ] Benchmarks completos
- [ ] Otimizações de cache

#### Qualidade
- [ ] Cobertura de testes > 90%
- [ ] Todos os doc tests passando
- [ ] Documentação completa (PT-BR)
- [ ] Exemplos para cada feature
- [ ] CI/CD configurado

#### Distribuição
- [ ] Publicado no crates.io
- [ ] Versionamento semântico
- [ ] Changelog.md
- [ ] API estável (sem breaking changes)
- [ ] License files
- [ ] Contributing guidelines

---

## Além do v1.0.0

### Possíveis Features Futuras

#### 1. Matrizes Esparsas
```rust
pub struct SparseMatrix<T> {
    rows: usize,
    cols: usize,
    data: HashMap<(usize, usize), T>,  // COO format
}
```

#### 2. GPU Acceleration (via CUDA/OpenCL)
```rust
#[cfg(feature = "gpu")]
impl MatrixMxN<f32> {
    pub fn mul_gpu(&self, other: &Self) -> Self {
        // Multiplicação na GPU
    }
}
```

#### 3. Complex Numbers
```rust
use num_complex::Complex;

type Matrix3x3c = Matrix3x3<Complex<f64>>;
```

#### 4. Automatic Differentiation (para ML)
```rust
pub struct Dual<T> {
    value: T,
    gradient: T,
}
```

#### 5. Integração com avila-ml
```rust
// Em avila-ml
use avila_linalg::MatrixMxN;

pub struct NeuralLayer {
    weights: MatrixMxN<f32>,
    biases: Vec<f32>,
}
```

---

## Cronograma Estimado

| Versão | Features          | Prazo Estimado | Status      |
| ------ | ----------------- | -------------- | ----------- |
| v0.1.0 | Base + testes     | -              | ✅ Concluído |
| v0.2.0 | SVD + Eigenvalues | 2 semanas      | 🔨 Próximo   |
| v0.3.0 | Performance       | 1 semana       | 📋 Planejado |
| v0.4.0 | Integração        | 1 semana       | 📋 Planejado |
| v0.5.0 | LU/Cholesky       | 1 semana       | 📋 Planejado |
| v1.0.0 | Production        | 1 semana       | 📋 Planejado |

**Total:** ~6-8 semanas para v1.0.0

---

## Priorização de Features

### Must Have (v1.0.0)
1. ✅ Vetores e matrizes básicas
2. 🔨 SVD
3. 🔨 Eigenvalues
4. 📋 QR decomposition
5. 📋 Resolver sistemas lineares

### Should Have (v1.x)
- LU decomposition
- Cholesky
- Performance otimizada (SIMD)
- Paralelização

### Could Have (v2.x)
- Matrizes esparsas
- GPU acceleration
- Complex numbers

### Won't Have (fora de escopo)
- Symbolic math (use SymPy/Mathematica)
- Arbitrary precision (use rug crate)
- Gráficos (use plotters)

---

## Decisões de Design

### Por que Golub-Reinsch para SVD?
✅ Numericamente estável
✅ Amplamente validado
✅ Convergência garantida
❌ Mais complexo que Jacobi

**Alternativa:** Jacobi SVD (mais simples, um pouco mais lento)

### Por que QR Algorithm para Eigenvalues?
✅ Funciona para matrizes não-simétricas
✅ O(n³) razoável
✅ Pode ser paralelizado
❌ Requer QR decomposition primeiro

**Alternativa:** Jacobi eigenvalue (apenas simétricas)

### Por que Householder para QR?
✅ Mais estável que Gram-Schmidt
✅ Menos operações que Givens
✅ Paralelizável
❌ Implementação mais complexa

---

## Recursos de Referência

### Papers
- Golub & Van Loan: "Matrix Computations" (4th ed)
- Watkins: "Fundamentals of Matrix Computations"
- Trefethen & Bau: "Numerical Linear Algebra"

### Implementações de Referência
- LAPACK (Fortran, gold standard)
- Eigen (C++, bem otimizado)
- nalgebra (Rust, completo)
- ndarray (Rust, arrays N-D)

### Benchmarks
- SuiteSparse Matrix Collection
- NIST Matrix Market
- Own synthetic benchmarks

---

**Autor:** Nícolas Ávila
**Última atualização:** 2024
**Versão do documento:** 1.0
