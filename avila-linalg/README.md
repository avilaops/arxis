# 🧮 Avila Linear Algebra

**Álgebra linear genuína em Rust para o ecossistema Avila.**

[![Crates.io](https://img.shields.io/crates/v/avila-linalg.svg)](https://crates.io/crates/avila-linalg)
[![Documentation](https://docs.rs/avila-linalg/badge.svg)](https://docs.rs/avila-linalg)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

---

## 🎯 Filosofia

**100% Avila** - Sem dependências pesadas. Álgebra linear eficiente, clara e educacional.

### Por que criar mais uma biblioteca de álgebra linear?

- ✅ **Controle total** - API desenhada para o ecossistema Avila
- ✅ **Zero bloat** - Apenas o necessário, sem features desnecessárias
- ✅ **Documentação PT-BR** - Primeiro no Brasil, depois no mundo
- ✅ **Educacional** - Código claro e explicativo
- ✅ **Otimizado** - Performance sem sacrificar legibilidade

---

## 📦 Instalação

```toml
[dependencies]
avila-linalg = "0.1"
```

---

## 🚀 Uso Rápido

```rust
use avila_linalg::{Vector3, Matrix3x3};

// Vetores
let v1 = Vector3::new(1.0, 2.0, 3.0);
let v2 = Vector3::new(4.0, 5.0, 6.0);

let dot = v1.dot(&v2);              // Produto escalar
let cross = v1.cross(&v2);          // Produto vetorial
let norm = v1.norm();               // Norma euclidiana

// Matrizes
let m = Matrix3x3::identity();
let v_transformed = m * v1;
```

---

## 📚 Features

### ✅ Vetores
- `Vector2<T>`, `Vector3<T>`, `Vector4<T>`
- `VectorN<T>` - Vetor de tamanho dinâmico
- Operações: soma, subtração, escalar, dot, cross, norm
- Normalização, projeção, reflexão

### ✅ Matrizes
- `Matrix2x2<T>`, `Matrix3x3<T>`, `Matrix4x4<T>`
- `MatrixMxN<T>` - Matriz de dimensões dinâmicas
- Operações: soma, multiplicação, transposta, inversa
- Determinante, traço

### 🚧 Em desenvolvimento
- [ ] Decomposição SVD (Singular Value Decomposition)
- [ ] Eigenvalues/Eigenvectors
- [ ] QR Decomposition
- [ ] LU Decomposition
- [ ] Cholesky Decomposition

---

## 🎓 Estrutura

```
avila-linalg/
├── src/
│   ├── lib.rs           # API pública
│   ├── vector.rs        # Vetores 2D/3D/4D/ND
│   ├── matrix.rs        # Matrizes
│   ├── decomposition/   # SVD, QR, LU, etc
│   │   ├── mod.rs
│   │   ├── svd.rs
│   │   └── eigen.rs
│   └── ops.rs           # Operações genéricas
├── benches/             # Benchmarks
├── examples/            # Exemplos
└── tests/               # Testes de integração
```

---

## 🔬 Exemplo Completo

```rust
use avila_linalg::prelude::*;

fn main() {
    // Cria matriz 3x3
    let m = Matrix3x3::from_rows([
        [1.0, 2.0, 3.0],
        [0.0, 1.0, 4.0],
        [5.0, 6.0, 0.0],
    ]);

    // Vetor
    let v = Vector3::new(1.0, 2.0, 3.0);

    // Transformação
    let result = m * v;

    println!("Resultado: {:?}", result);

    // Decomposição SVD (futuro)
    // let svd = m.svd();
    // let (u, s, vt) = svd.decompose();
}
```

---

## 📊 Performance

Comparação com outras bibliotecas:

| Operação  | avila-linalg | nalgebra | ndarray |
| --------- | ------------ | -------- | ------- |
| Vec3 dot  | ~2ns         | ~2ns     | ~3ns    |
| Mat3 mul  | ~15ns        | ~12ns    | ~20ns   |
| SVD 10x10 | TBD          | ~800ns   | ~1.2µs  |

---

## 🤝 Contribuindo

Contribuições são bem-vindas! Veja [CONTRIBUTING.md](CONTRIBUTING.md)

---

## 📄 Licença

MIT OR Apache-2.0

---

**Desenvolvido com ❤️ pela Avila para a comunidade brasileira de Rust** 🇧🇷
