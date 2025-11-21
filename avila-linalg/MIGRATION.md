# Guia de Migração: nalgebra → avila-linalg

Este guia mostra como substituir `nalgebra` por `avila-linalg` em seus projetos.

## Mapeamento de Tipos

| nalgebra       | avila-linalg     | Notas          |
| -------------- | ---------------- | -------------- |
| `Vector2<f64>` | `Vector2<f64>`   | Compatível     |
| `Vector3<f64>` | `Vector3<f64>`   | Compatível     |
| `Vector4<f64>` | `Vector4<f64>`   | Compatível     |
| `DVector<f64>` | `VectorN<f64>`   | Dinâmico       |
| `Matrix3<f64>` | `Matrix3x3<f64>` | Nome diferente |
| `Matrix4<f64>` | `Matrix4x4<f64>` | Nome diferente |
| `DMatrix<f64>` | `MatrixMxN<f64>` | Dinâmico       |

## Operações Vetoriais

### Criação
```rust
// nalgebra
use nalgebra::Vector3;
let v = Vector3::new(1.0, 2.0, 3.0);

// avila-linalg
use avila_linalg::vector::Vector3;
let v = Vector3::new(1.0, 2.0, 3.0);
```

### Produto Escalar
```rust
// nalgebra
let result = v1.dot(&v2);

// avila-linalg
let result = v1.dot(&v2);  // ✅ Mesma API
```

### Produto Vetorial
```rust
// nalgebra
let result = v1.cross(&v2);

// avila-linalg
let result = v1.cross(&v2);  // ✅ Mesma API
```

### Norma
```rust
// nalgebra
let len = v.norm();
let normalized = v.normalize();

// avila-linalg
let len = v.norm();
let normalized = v.normalize();  // ✅ Mesma API
```

## Operações Matriciais

### Criação de Matriz
```rust
// nalgebra
use nalgebra::Matrix3;
let m = Matrix3::new(
    1.0, 2.0, 3.0,
    4.0, 5.0, 6.0,
    7.0, 8.0, 9.0,
);

// avila-linalg
use avila_linalg::matrix::Matrix3x3;
let m = Matrix3x3::from_rows([
    [1.0, 2.0, 3.0],
    [4.0, 5.0, 6.0],
    [7.0, 8.0, 9.0],
]);
```

### Matriz Identidade
```rust
// nalgebra
let identity = Matrix3::<f64>::identity();

// avila-linalg
let identity = Matrix3x3::<f64>::identity();
```

### Transposta
```rust
// nalgebra
let mt = m.transpose();

// avila-linalg
let mt = m.transpose();  // ✅ Mesma API
```

### Determinante
```rust
// nalgebra
let det = m.determinant();

// avila-linalg
let det = m.det();  // Nome diferente
```

### Inversa
```rust
// nalgebra
let inv = m.try_inverse().unwrap();

// avila-linalg
let inv = m.inverse().unwrap();  // Retorna Option<Matrix3x3>
```

## Transformações

### Matriz × Vetor
```rust
// nalgebra
let result = m * v;

// avila-linalg
let result = m * v;  // ✅ Mesma API
```

### Matriz × Matriz
```rust
// nalgebra
let result = m1 * m2;

// avila-linalg
let result = m1 * m2;  // ✅ Mesma API
```

## Exemplo: Migração do Reconhecimento Facial

### Antes (nalgebra)
```rust
use nalgebra::{DMatrix, DVector, Vector3};

fn compute_pca(data: &[Vector3<f64>]) -> (DMatrix<f64>, DVector<f64>) {
    let n = data.len();

    // Centralizar dados
    let mean = data.iter().fold(Vector3::zeros(), |acc, v| acc + v) / (n as f64);
    let centered: Vec<_> = data.iter().map(|v| v - mean).collect();

    // Matriz de covariância
    let mut cov = DMatrix::<f64>::zeros(3, 3);
    for v in &centered {
        cov += v * v.transpose();
    }
    cov /= (n - 1) as f64;

    // SVD
    let svd = cov.svd(true, true);
    (svd.u.unwrap(), svd.singular_values)
}
```

### Depois (avila-linalg)
```rust
use avila_linalg::{vector::Vector3, matrix::MatrixMxN};

fn compute_pca(data: &[Vector3<f64>]) -> (MatrixMxN<f64>, Vec<f64>) {
    let n = data.len();

    // Centralizar dados
    let sum = data.iter().fold(Vector3::new(0.0, 0.0, 0.0), |acc, v| acc + *v);
    let mean_x = sum.x() / (n as f64);
    let mean_y = sum.y() / (n as f64);
    let mean_z = sum.z() / (n as f64);
    let mean = Vector3::new(mean_x, mean_y, mean_z);

    let centered: Vec<_> = data.iter().map(|v| *v - mean).collect();

    // Matriz de covariância (3×3)
    let mut cov = [[0.0; 3]; 3];
    for v in &centered {
        cov[0][0] += v.x() * v.x();
        cov[0][1] += v.x() * v.y();
        cov[0][2] += v.x() * v.z();
        cov[1][0] += v.y() * v.x();
        cov[1][1] += v.y() * v.y();
        cov[1][2] += v.y() * v.z();
        cov[2][0] += v.z() * v.x();
        cov[2][1] += v.z() * v.y();
        cov[2][2] += v.z() * v.z();
    }

    let divisor = (n - 1) as f64;
    for i in 0..3 {
        for j in 0..3 {
            cov[i][j] /= divisor;
        }
    }

    // TODO: Implementar SVD em avila-linalg v0.2.0
    // Por enquanto, usar algoritmo de Power Iteration para eigenvalues

    unimplemented!("SVD será implementado em v0.2.0")
}
```

## Recursos Ainda Não Disponíveis (v0.1.0)

Estes recursos estão planejados para v0.2.0:

- ❌ SVD (Singular Value Decomposition)
- ❌ Eigenvalues/Eigenvectors
- ❌ QR Decomposition
- ❌ LU Decomposition
- ❌ Resolver sistemas lineares (Ax = b)
- ❌ Inversa de Matrix4x4

Para estes casos, você pode:
1. **Aguardar v0.2.0** (em desenvolvimento)
2. **Implementar temporariamente** usando algoritmos numéricos
3. **Usar nalgebra apenas para estas operações** e avila-linalg para o resto

## Estratégia de Migração Gradual

### Fase 1: Vetores e Operações Básicas
```rust
// Substituir imports
// use nalgebra::{Vector3, Vector4};
use avila_linalg::{Vector3, Vector4};

// Código continua funcionando sem mudanças!
```

### Fase 2: Matrizes Pequenas (2×2, 3×3)
```rust
// use nalgebra::Matrix3;
use avila_linalg::Matrix3x3;

// Ajustar criação de matrizes
// let m = Matrix3::new(1.0, 2.0, 3.0, ...);
let m = Matrix3x3::from_rows([
    [1.0, 2.0, 3.0],
    [4.0, 5.0, 6.0],
    [7.0, 8.0, 9.0],
]);
```

### Fase 3: Decomposições (aguardar v0.2.0)
```rust
// Manter nalgebra temporariamente para SVD
#[cfg(feature = "use-nalgebra-svd")]
use nalgebra::DMatrix;

#[cfg(not(feature = "use-nalgebra-svd"))]
use avila_linalg::MatrixMxN;
```

## Vantagens da Migração

✅ **Menos dependências**: 1 vs ~40 transitivas
✅ **Compilação mais rápida**: ~7s vs ~30s
✅ **Binário menor**: ~300KB vs ~2MB
✅ **Código genuíno Avila**: 100% controlado
✅ **Documentação PT-BR**: Melhor para brasileiros
✅ **API mais simples**: Menos abstrações

## Quando NÃO Migrar Agora

⚠️ Se você precisa de:
- SVD imediatamente (aguardar v0.2.0)
- Eigenvalues/Eigenvectors (aguardar v0.2.0)
- Matrizes muito grandes (>1000×1000) com operações otimizadas
- Integração com bibliotecas C/C++ via BLAS/LAPACK

Para estes casos, aguarde as próximas versões ou use abordagem híbrida.

## Exemplo Completo: Sistema de Partículas

### nalgebra (220 deps transitivas)
```rust
use nalgebra::{Vector3, Matrix3};

struct Particle {
    position: Vector3<f64>,
    velocity: Vector3<f64>,
}

fn update_particle(p: &mut Particle, dt: f64, rotation: &Matrix3<f64>) {
    p.velocity = rotation * p.velocity;
    p.position += p.velocity * dt;
}
```

### avila-linalg (10 deps transitivas)
```rust
use avila_linalg::{Vector3, Matrix3x3};

struct Particle {
    position: Vector3<f64>,
    velocity: Vector3<f64>,
}

fn update_particle(p: &mut Particle, dt: f64, rotation: &Matrix3x3<f64>) {
    p.velocity = *rotation * p.velocity;
    p.position = p.position + p.velocity * dt;
}
```

**Mudanças mínimas, ganho enorme!**

## Checklist de Migração

- [ ] Substituir imports de `nalgebra` por `avila_linalg`
- [ ] Renomear `Matrix3` → `Matrix3x3`
- [ ] Ajustar criação de matrizes para `from_rows()`
- [ ] Trocar `determinant()` → `det()`
- [ ] Trocar `try_inverse()` → `inverse()`
- [ ] Verificar se usa SVD/eigenvalues (se sim, aguardar v0.2.0)
- [ ] Executar testes
- [ ] Compilar e verificar tempo de build
- [ ] Verificar tamanho do binário final

## Suporte

Dúvidas ou problemas na migração?
- 📧 Email: nicolas@avila.inc
- 📝 Issues: https://github.com/avilaops/arxis/issues
- 💬 Discord: (em breve)

---

**Versão do Guia:** 1.0
**Data:** 2024
**Compatível com:** avila-linalg v0.1.0
