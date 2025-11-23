# Avila LinAlg - Copilot Instructions

**Projeto**: avila-linalg
**Descrição**: Pure Rust Linear Algebra Library - Vectors, Matrices, SVD, Eigenvalues
**Status**: v0.1.0 Released (foundation complete)
**Filosofia**: Clareza > Performance prematura. Correção > Otimização.

---

## 🎯 REGRAS CRÍTICAS - NUNCA VIOLAR

### 1. Zero Dependencies Externas (Exceto num-traits)
```toml
# ✅ PERMITIDO (apenas traits)
num-traits = "0.2"   # Float, Zero, One traits genéricos
rayon = { version = "1.10", optional = true }  # Paralelização (opt-in)

# ❌ PROIBIDO (qualquer outra dependência)
- nalgebra = "..."   # Somos a alternativa!
- ndarray = "..."    # Somos mais leves!
- blas-src = "..."   # Implementação nativa!
- lapack-src = "..." # Implementação nativa!
```

**Motivo**: Controle total sobre algoritmos, portabilidade (WASM, embedded), aprendizado.

### 2. Correção Numérica > Performance
```rust
// ❌ ERRADO: Rápido mas instável
fn determinant_naive(m: &Matrix3x3<f64>) -> f64 {
    m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
    - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
    + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])
}

// ✅ CORRETO: LU decomposition (estável)
fn determinant_lu(m: &Matrix3x3<f64>) -> f64 {
    let lu = m.lu_decompose();
    lu.l.diagonal_product() * lu.u.diagonal_product()
}
```

**Regra**: Use algoritmos numericamente estáveis mesmo que sejam 2x mais lentos.

### 3. API Genérica Desde o Início
```rust
// ✅ CORRETO: Genérico sobre tipo escalar
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Float> Vector3<T> {
    pub fn norm(&self) -> T {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}

// Funciona com f32, f64, Complex, etc.
let v_f32 = Vector3::new(1.0f32, 2.0, 3.0);
let v_f64 = Vector3::new(1.0f64, 2.0, 3.0);
```

### 4. Testes com Ground Truth Matemática
```rust
#[test]
fn test_svd_reconstruction() {
    let m = Matrix3x3::from_rows([
        [1.0, 2.0, 3.0],
        [4.0, 5.0, 6.0],
        [7.0, 8.0, 9.0],
    ]);

    let svd = m.svd();
    let reconstructed = svd.u * svd.s * svd.vt;

    // Validar A = U * Σ * V^T (erro < 1e-10)
    for i in 0..3 {
        for j in 0..3 {
            assert!((m[i][j] - reconstructed[i][j]).abs() < 1e-10);
        }
    }
}
```

**Obrigatório**: Todo algoritmo deve ter testes provando correção matemática.

---

## 📐 Arquitetura do Projeto

```
avila-linalg/
├── src/
│   ├── lib.rs              # Exports públicos + prelude
│   ├── vector.rs           # Vector2, Vector3, Vector4, VectorN
│   ├── matrix.rs           # Matrix2x2, Matrix3x3, Matrix4x4, MatrixMxN
│   ├── ops.rs              # Traits genéricos (Add, Mul, etc.)
│   ├── decomposition/
│   │   ├── mod.rs
│   │   ├── svd.rs          # Singular Value Decomposition
│   │   ├── eigen.rs        # Eigenvalues/Eigenvectors
│   │   ├── qr.rs           # QR Decomposition
│   │   ├── lu.rs           # LU Decomposition
│   │   └── cholesky.rs     # Cholesky Decomposition
│   ├── solve/
│   │   ├── mod.rs
│   │   ├── linear.rs       # Solve Ax = b
│   │   ├── least_squares.rs # Least squares fitting
│   │   └── iterative.rs    # Conjugate Gradient, GMRES
│   └── special/
│       ├── mod.rs
│       ├── orthogonal.rs   # Gram-Schmidt, QR
│       └── symmetric.rs    # Symmetric matrix ops
├── benches/
│   ├── vector_bench.rs
│   ├── matrix_bench.rs
│   └── decomposition_bench.rs
├── examples/
│   ├── basic_ops.rs
│   ├── svd_image_compression.rs
│   ├── least_squares_fitting.rs
│   └── eigenfaces.rs
└── tests/
    ├── vector_test.rs
    ├── matrix_test.rs
    └── decomposition_test.rs
```

---

## 🚀 Roadmap de Implementação

### Fase 1: Foundation (v0.1.0) ✅ COMPLETO
- [x] Vector2, Vector3, Vector4, VectorN
- [x] Matrix2x2, Matrix3x3, Matrix4x4, MatrixMxN
- [x] Operações básicas: add, sub, mul, dot, cross
- [x] Transposta, determinante (2x2, 3x3)
- [x] Traits genéricos (From, Into, Index, Add, Mul)

### Fase 2: Decompositions (v0.2.0) - Semanas 1-4
```rust
// TODO: Implementar SVD (Singular Value Decomposition)
pub struct SVD<T> {
    /// Left singular vectors (orthogonal)
    pub u: MatrixMxN<T>,
    /// Singular values (diagonal, sorted descending)
    pub s: VectorN<T>,
    /// Right singular vectors transposed (orthogonal)
    pub vt: MatrixMxN<T>,
}

impl<T: Float> MatrixMxN<T> {
    /// Compute SVD via Golub-Reinsch algorithm
    pub fn svd(&self) -> SVD<T> {
        // 1. Bidiagonalization (Householder reflections)
        // 2. Diagonalization (QR algorithm iterativo)
        // 3. Sort singular values descending
    }
}

// Aplicação: Compressão de imagens
pub fn compress_image(img: &MatrixMxN<f64>, k: usize) -> MatrixMxN<f64> {
    let svd = img.svd();

    // Keep only k largest singular values
    let u_k = svd.u.slice_cols(0, k);
    let s_k = svd.s.slice(0, k);
    let vt_k = svd.vt.slice_rows(0, k);

    u_k * Matrix::from_diagonal(&s_k) * vt_k
}
```

**Algoritmo SVD**:
1. **Bidiagonalization**: Householder reflections (O(mn²))
2. **Diagonalization**: QR algorithm iterativo (O(n³))
3. **Sorting**: Sort singular values

**Paper de referência**: Golub & Reinsch (1970) - "Singular Value Decomposition and Least Squares Solutions"

```rust
// TODO: Implementar Eigenvalues/Eigenvectors
pub struct Eigen<T> {
    /// Eigenvalues (podem ser complexos!)
    pub values: Vec<Complex<T>>,
    /// Eigenvectors (colunas da matriz)
    pub vectors: MatrixMxN<Complex<T>>,
}

impl<T: Float> MatrixMxN<T> {
    /// Compute eigenvalues via QR algorithm
    pub fn eigenvalues(&self) -> Vec<Complex<T>> {
        assert!(self.is_square(), "Matrix must be square");

        // 1. Reduce to Hessenberg form (Householder)
        // 2. QR algorithm com shifts
        // 3. Extract eigenvalues from quasi-triangular form
    }

    /// Compute eigenvectors (needs eigenvalues first)
    pub fn eigen(&self) -> Eigen<T> {
        let values = self.eigenvalues();
        let vectors = self.compute_eigenvectors(&values);
        Eigen { values, vectors }
    }
}

// Aplicação: PCA (Principal Component Analysis)
pub fn pca(data: &MatrixMxN<f64>, k: usize) -> MatrixMxN<f64> {
    // 1. Center data (subtract mean)
    let centered = data.center_columns();

    // 2. Covariance matrix
    let cov = (centered.t() * &centered) / (data.rows() - 1) as f64;

    // 3. Eigendecomposition
    let eigen = cov.eigen();

    // 4. Take k largest eigenvectors
    eigen.vectors.slice_cols(0, k)
}
```

**Algoritmo Eigenvalues**:
1. **Hessenberg reduction**: Similar triangular superior
2. **QR algorithm**: Iterativo com shifts de Wilkinson
3. **Complex handling**: Eigenvalues podem ser complexos!

**Paper**: Francis (1961) - "The QR Transformation"

### Fase 3: Linear Solvers (v0.3.0) - Semanas 5-8
```rust
// TODO: Solve Ax = b
pub enum SolverMethod {
    LU,          // LU decomposition (dense)
    QR,          // QR decomposition (overdetermined)
    Cholesky,    // Symmetric positive definite
    Iterative,   // Conjugate gradient (sparse)
}

impl<T: Float> MatrixMxN<T> {
    /// Solve Ax = b
    pub fn solve(&self, b: &VectorN<T>, method: SolverMethod) -> VectorN<T> {
        match method {
            SolverMethod::LU => self.solve_lu(b),
            SolverMethod::QR => self.solve_qr(b),
            SolverMethod::Cholesky => self.solve_cholesky(b),
            SolverMethod::Iterative => self.solve_cg(b),
        }
    }

    /// LU solve: PA = LU
    fn solve_lu(&self, b: &VectorN<T>) -> VectorN<T> {
        let lu = self.lu_decompose();

        // 1. Solve Ly = Pb (forward substitution)
        let y = lu.l.forward_substitute(&lu.permute(b));

        // 2. Solve Ux = y (backward substitution)
        lu.u.backward_substitute(&y)
    }

    /// QR solve: A = QR, então x = R^(-1) * Q^T * b
    fn solve_qr(&self, b: &VectorN<T>) -> VectorN<T> {
        let qr = self.qr_decompose();
        let qt_b = qr.q.t() * b;
        qr.r.backward_substitute(&qt_b)
    }
}

// TODO: Least Squares (overdetermined system)
pub fn least_squares<T: Float>(a: &MatrixMxN<T>, b: &VectorN<T>) -> VectorN<T> {
    // Solve (A^T A) x = A^T b
    let ata = a.t() * a;
    let atb = a.t() * b;
    ata.solve(&atb, SolverMethod::Cholesky)
}

// Aplicação: Linear regression
pub fn linear_regression(x: &[f64], y: &[f64]) -> (f64, f64) {
    // Fit y = mx + b
    let n = x.len();

    // Design matrix: [1, x]
    let mut a = MatrixMxN::new(n, 2);
    for i in 0..n {
        a[[i, 0]] = 1.0;
        a[[i, 1]] = x[i];
    }

    let b_vec = VectorN::from_slice(y);
    let coeffs = least_squares(&a, &b_vec);

    (coeffs[1], coeffs[0])  // (slope, intercept)
}
```

**Métodos de Solver**:
- **LU**: Dense, general purpose (O(n³))
- **QR**: Overdetermined, least squares (O(mn²))
- **Cholesky**: Symmetric positive definite (O(n³/3), mais rápido)
- **Conjugate Gradient**: Sparse, iterativo (O(kn), k << n)

### Fase 4: Otimizações SIMD (v0.4.0) - Semanas 9-12
```rust
// TODO: Vetorização com SIMD (AVX2, AVX-512, NEON)
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

impl Vector4<f64> {
    /// Dot product com AVX2 (4x f64 simultâneos)
    #[cfg(target_feature = "avx2")]
    pub fn dot_simd(&self, other: &Self) -> f64 {
        unsafe {
            let a = _mm256_loadu_pd(&self.x as *const f64);
            let b = _mm256_loadu_pd(&other.x as *const f64);
            let prod = _mm256_mul_pd(a, b);

            // Horizontal sum
            let sum = _mm256_hadd_pd(prod, prod);
            let hi = _mm256_extractf128_pd(sum, 1);
            let lo = _mm256_castpd256_pd128(sum);
            let final_sum = _mm_add_pd(hi, lo);

            _mm_cvtsd_f64(final_sum)
        }
    }
}

impl MatrixMxN<f64> {
    /// Matrix multiplication com blocking + SIMD
    pub fn matmul_optimized(&self, other: &Self) -> Self {
        const BLOCK_SIZE: usize = 64;  // L1 cache-friendly

        let mut result = Self::zeros(self.rows, other.cols);

        // Blocked multiplication (cache-friendly)
        for i in (0..self.rows).step_by(BLOCK_SIZE) {
            for j in (0..other.cols).step_by(BLOCK_SIZE) {
                for k in (0..self.cols).step_by(BLOCK_SIZE) {
                    let i_end = (i + BLOCK_SIZE).min(self.rows);
                    let j_end = (j + BLOCK_SIZE).min(other.cols);
                    let k_end = (k + BLOCK_SIZE).min(self.cols);

                    self.matmul_block(&mut result, other, i, i_end, j, j_end, k, k_end);
                }
            }
        }

        result
    }
}
```

**Metas de Performance**:
- Vector3 dot product: <2ns (AVX2)
- Matrix3x3 multiplication: <10ns (AVX2)
- Matrix 1000x1000 multiplication: <200ms (blocked + SIMD)
- SVD 100x100: <10ms

### Fase 5: Sparse Matrices (v0.5.0) - Semanas 13-16
```rust
// TODO: Compressed Sparse Row (CSR) format
pub struct SparseMatrix<T> {
    /// Values (non-zero elements)
    values: Vec<T>,
    /// Column indices
    col_indices: Vec<usize>,
    /// Row pointers
    row_ptrs: Vec<usize>,
    rows: usize,
    cols: usize,
}

impl<T: Float> SparseMatrix<T> {
    /// Sparse matrix-vector multiplication
    pub fn mul_vec(&self, x: &VectorN<T>) -> VectorN<T> {
        let mut y = VectorN::zeros(self.rows);

        for i in 0..self.rows {
            let start = self.row_ptrs[i];
            let end = self.row_ptrs[i + 1];

            for j in start..end {
                let col = self.col_indices[j];
                let val = self.values[j];
                y[i] = y[i] + val * x[col];
            }
        }

        y
    }

    /// Conjugate Gradient solver (para matrizes sparse simétricas)
    pub fn solve_cg(&self, b: &VectorN<T>, max_iter: usize, tol: T)
        -> VectorN<T> {
        let mut x = VectorN::zeros(self.cols);
        let mut r = b.clone();
        let mut p = r.clone();
        let mut rs_old = r.dot(&r);

        for _ in 0..max_iter {
            let ap = self.mul_vec(&p);
            let alpha = rs_old / p.dot(&ap);

            x = x + &p * alpha;
            r = r - &ap * alpha;

            let rs_new = r.dot(&r);
            if rs_new.sqrt() < tol {
                break;
            }

            let beta = rs_new / rs_old;
            p = &r + &p * beta;
            rs_old = rs_new;
        }

        x
    }
}
```

**Aplicações Sparse**:
- Finite Element Analysis (FEA)
- Graph algorithms (PageRank, etc.)
- Large-scale linear systems (millions of equations)

---

## 🧪 Testes Obrigatórios

### 1. Unit Tests (Correção Algorítmica)
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_vector_dot_product() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(4.0, 5.0, 6.0);

        let dot = v1.dot(&v2);
        assert_relative_eq!(dot, 32.0, epsilon = 1e-10);
    }

    #[test]
    fn test_matrix_determinant() {
        let m = Matrix3x3::from_rows([
            [1.0, 2.0, 3.0],
            [0.0, 1.0, 4.0],
            [5.0, 6.0, 0.0],
        ]);

        let det = m.determinant();
        assert_relative_eq!(det, 1.0, epsilon = 1e-10);
    }

    #[test]
    fn test_svd_orthogonality() {
        let m = Matrix3x3::identity();
        let svd = m.svd();

        // U^T * U = I
        let utu = svd.u.t() * &svd.u;
        for i in 0..3 {
            for j in 0..3 {
                let expected = if i == j { 1.0 } else { 0.0 };
                assert_relative_eq!(utu[[i, j]], expected, epsilon = 1e-10);
            }
        }
    }
}
```

### 2. Property-Based Tests
```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_matrix_mul_associative(
        a in matrix_3x3(),
        b in matrix_3x3(),
        c in matrix_3x3()
    ) {
        let left = (a.clone() * b.clone()) * c.clone();
        let right = a * (b * c);

        assert_matrices_equal(&left, &right, 1e-10);
    }

    #[test]
    fn test_svd_reconstruction(m in matrix_5x5()) {
        let svd = m.svd();
        let reconstructed = svd.u * svd.s * svd.vt;

        assert_matrices_equal(&m, &reconstructed, 1e-10);
    }
}
```

### 3. Benchmarks (Performance)
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_vector_ops(c: &mut Criterion) {
    let v1 = Vector3::new(1.0, 2.0, 3.0);
    let v2 = Vector3::new(4.0, 5.0, 6.0);

    c.bench_function("vector3_dot", |b| {
        b.iter(|| black_box(v1.dot(&v2)))
    });

    c.bench_function("vector3_cross", |b| {
        b.iter(|| black_box(v1.cross(&v2)))
    });
}

fn bench_matrix_mul(c: &mut Criterion) {
    let m1 = Matrix3x3::identity();
    let m2 = Matrix3x3::identity();

    c.bench_function("matrix3x3_mul", |b| {
        b.iter(|| black_box(&m1 * &m2))
    });
}

criterion_group!(benches, bench_vector_ops, bench_matrix_mul);
criterion_main!(benches);
```

---

## 📊 Comparação com Alternativas

| Feature | avila-linalg | nalgebra | ndarray |
|---------|--------------|----------|---------|
| **Dependencies** | 1 (num-traits) | 20+ | 10+ |
| **Binary size** | ~500KB | ~3MB | ~2MB |
| **Compile time** | <5s | ~30s | ~20s |
| **WASM support** | ✅ Full | ✅ Partial | ⚠️ Limited |
| **Embedded** | ✅ no_std | ⚠️ Difficult | ❌ std only |
| **API simplicity** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐ |
| **PT-BR docs** | ✅ Full | ❌ No | ❌ No |
| **Educational** | ✅ Clear code | ⚠️ Complex | ⚠️ Complex |

**Quando usar avila-linalg**:
- Projetos educacionais (código claro)
- Embedded/WASM (minimal deps)
- Controle total sobre algoritmos
- PT-BR documentation needed

**Quando usar nalgebra**:
- Production (battle-tested)
- GPU acceleration needed
- Maximum performance critical

---

## 🎓 Recursos de Aprendizado

### Livros Essenciais
1. **"Matrix Computations"** - Golub & Van Loan (1996)
   - Bíblia da álgebra linear numérica
   - SVD, eigenvalues, solvers

2. **"Numerical Linear Algebra"** - Trefethen & Bau (1997)
   - Abordagem prática
   - Stability analysis

3. **"Introduction to Linear Algebra"** - Strang (2016)
   - Intuitivo e visual
   - MIT OpenCourseWare

### Papers Importantes
- Golub & Reinsch (1970) - SVD algorithm
- Francis (1961) - QR algorithm for eigenvalues
- Hestenes & Stiefel (1952) - Conjugate Gradient
- Householder (1958) - Householder reflections

### Cursos Online
- MIT 18.06 - Linear Algebra (Gilbert Strang)
- Coursera - Matrix Methods in Data Analysis

---

## ⚠️ Erros Comuns a Evitar

### 1. Division by Zero
```rust
// ❌ ERRADO: Panic em vetor nulo
impl<T: Float> Vector3<T> {
    pub fn normalize(&self) -> Self {
        let norm = self.norm();
        Self {
            x: self.x / norm,  // Panic se norm == 0!
            y: self.y / norm,
            z: self.z / norm,
        }
    }
}

// ✅ CORRETO: Retornar Result
impl<T: Float> Vector3<T> {
    pub fn normalize(&self) -> Result<Self, &'static str> {
        let norm = self.norm();
        if norm.is_zero() {
            return Err("Cannot normalize zero vector");
        }
        Ok(Self {
            x: self.x / norm,
            y: self.y / norm,
            z: self.z / norm,
        })
    }
}
```

### 2. Index Out of Bounds
```rust
// ❌ ERRADO: Sem validação
impl<T> Matrix3x3<T> {
    pub fn get(&self, i: usize, j: usize) -> &T {
        &self.data[i * 3 + j]  // Panic se i >= 3 ou j >= 3
    }
}

// ✅ CORRETO: Panic with message or checked access
impl<T> Matrix3x3<T> {
    pub fn get(&self, i: usize, j: usize) -> &T {
        assert!(i < 3 && j < 3, "Index ({}, {}) out of bounds", i, j);
        &self.data[i * 3 + j]
    }

    pub fn get_checked(&self, i: usize, j: usize) -> Option<&T> {
        if i < 3 && j < 3 {
            Some(&self.data[i * 3 + j])
        } else {
            None
        }
    }
}
```

### 3. Singular Matrix Inversion
```rust
// ❌ ERRADO: Assume matrix is invertible
impl Matrix3x3<f64> {
    pub fn inverse(&self) -> Self {
        let det = self.determinant();
        let adj = self.adjugate();
        adj / det  // Division by zero if det == 0!
    }
}

// ✅ CORRETO: Check determinant
impl Matrix3x3<f64> {
    pub fn inverse(&self) -> Result<Self, &'static str> {
        let det = self.determinant();
        if det.abs() < 1e-10 {
            return Err("Matrix is singular (non-invertible)");
        }
        let adj = self.adjugate();
        Ok(adj / det)
    }
}
```

---

## 🏆 Checklist de Qualidade

Antes de fazer PR:

- [ ] **Correção**: Testes unitários com ground truth matemática
- [ ] **Stability**: Algoritmos numericamente estáveis
- [ ] **Generics**: API genérica sobre tipo escalar (Float trait)
- [ ] **Docs**: Docstrings com:
  - Descrição do algoritmo
  - Complexidade (tempo e espaço)
  - Exemplo de uso
  - Referências (papers, livros)
- [ ] **Error Handling**: Result<T, E> para operações que podem falhar
- [ ] **Zero Deps**: Apenas num-traits (+ rayon opcional)
- [ ] **Benchmarks**: Performance validada
- [ ] **Clippy Clean**: `cargo clippy -- -D warnings`
- [ ] **Formatting**: `cargo fmt`
- [ ] **Coverage**: ≥90% code coverage

---

## 🚀 Como Começar

### Setup
```bash
cd arxis/avila-linalg
cargo build
cargo test
cargo bench
```

### Implementar Feature
1. Escolha task do Roadmap (Fase 2, 3, 4, 5)
2. Leia papers de referência listados
3. Implemente com testes
4. Valide com benchmarks
5. Documente completamente

### Exemplo: Implementar QR Decomposition
```rust
// 1. Ler: Householder (1958), Golub & Van Loan Cap. 5

// 2. Implementar
pub struct QR<T> {
    pub q: MatrixMxN<T>,  // Orthogonal
    pub r: MatrixMxN<T>,  // Upper triangular
}

impl<T: Float> MatrixMxN<T> {
    pub fn qr_decompose(&self) -> QR<T> {
        // Householder reflections...
    }
}

// 3. Testar
#[test]
fn test_qr_orthogonality() {
    let m = Matrix3x3::from_rows([[1.0, 2.0], [3.0, 4.0], [5.0, 6.0]]);
    let qr = m.qr_decompose();

    // Q^T * Q = I
    let qtq = qr.q.t() * &qr.q;
    assert_matrices_equal(&qtq, &Matrix2x2::identity(), 1e-10);

    // Q * R = A
    let reconstructed = &qr.q * &qr.r;
    assert_matrices_equal(&m, &reconstructed, 1e-10);
}

// 4. Benchmark
criterion_benchmark!(qr_100x100, qr_1000x1000);

// 5. Documentar
/// QR decomposition via Householder reflections
///
/// # Algorithm
/// ```text
/// A = Q * R
/// onde Q é ortogonal (Q^T Q = I)
/// e R é triangular superior
/// ```
///
/// # Complexity
/// - Time: O(mn²)
/// - Space: O(mn)
///
/// # References
/// - Householder (1958). "Unitary Triangularization"
/// - Golub & Van Loan (1996). Ch. 5
pub fn qr_decompose(&self) -> QR<T> { ... }
```

---

**Lembre-se**: Álgebra linear é fundação de ML, física, gráficos 3D. Correção aqui é crítica.

**Avila LinAlg** - Clareza e Correção 🧮✨
