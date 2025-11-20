# Migração de Código Legacy Python → Rust

**Data**: 20 de Novembro de 2025
**Motivo**: Consolidação do ecossistema AVX em Rust puro

## ⚠️ Código Arquivado

O diretório `Avila-Engine/` (Python) foi **arquivado** para `deprecated/prototypes/`.

### Razões do Arquivamento

1. **Duplicação**: Funcionalidades já implementadas em Rust (avila-math, avx-quantum-render)
2. **Performance**: Python não atende requisitos de simulações científicas (QED, CFD)
3. **Padrão AVX**: Todo ecossistema deve ser Rust para consistência e zero-cost abstractions
4. **Manutenção**: Duas bases de código criavam confusão e overhead

## 📦 Conteúdo Arquivado

### Avila-Engine (Python)
- `kernel/` - Estruturas matemáticas 3D
  - Vector3, Vector4, Matrix4x4, Quaternion, AABB
- `memory/` - Memory allocators customizados
  - PoolAllocator, ArenaAllocator, StackAllocator
- `examples/` - Exemplos educacionais

## ✅ Equivalentes em Rust (Produção)

| Python (Deprecated)    | Rust (Ativo)                          | Status             |
| ---------------------- | ------------------------------------- | ------------------ |
| `kernel/vector.py`     | `avila-math/src/vector.rs`            | ✅ Implementado     |
| `kernel/matrix.py`     | `avila-math/src/matrix.rs`            | ✅ Implementado     |
| `kernel/quaternion.py` | `arxis_quaternions/src/quaternion.rs` | ✅ Implementado     |
| `kernel/aabb.py`       | `avila-math/src/bounds.rs`            | ⚠️ Verificar        |
| `memory/pool.py`       | **TODO**: `avx-memory/`               | 🔴 Não implementado |
| `memory/arena.py`      | **TODO**: `avx-memory/`               | 🔴 Não implementado |
| `memory/stack.py`      | **TODO**: `avx-memory/`               | 🔴 Não implementado |

## 🚀 Próximos Passos

### Funcionalidades Faltantes

Se precisar dos memory allocators customizados:

```rust
// Criar: avx-memory/src/lib.rs
pub struct PoolAllocator<T> {
    blocks: Vec<Option<T>>,
    free_list: Vec<usize>,
}

pub struct ArenaAllocator {
    buffer: Vec<u8>,
    offset: usize,
}

pub struct StackAllocator {
    buffer: Vec<u8>,
    markers: Vec<usize>,
}
```

### AABB em Rust

Se não existir em `avila-math`:

```rust
// Adicionar em: avila-math/src/bounds.rs
#[derive(Debug, Clone, Copy)]
pub struct AABB {
    pub min: Vector3,
    pub max: Vector3,
}

impl AABB {
    pub fn from_center_size(center: Vector3, size: Vector3) -> Self { ... }
    pub fn intersects(&self, other: &AABB) -> bool { ... }
    pub fn contains_point(&self, point: Vector3) -> bool { ... }
}
```

## 📚 Referências

- **Readme1.md**: Visão científica completa (QED, CFD, ótica quântica)
- **avx-quantum-render**: Primeira implementação real seguindo roadmap
- **avila-math**: Core matemático em Rust

## 🔒 Política

- **Não modificar** código em `deprecated/` - apenas consulta histórica
- **Toda nova funcionalidade** deve ser implementada em Rust no padrão AVX
- **Performance crítica** sempre em Rust com SIMD quando possível

---

**Arquivado por**: GitHub Copilot (Claude Sonnet 4.5)
**Aprovado por**: Nicolas @ Avila Inc.
