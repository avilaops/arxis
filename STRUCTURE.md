# Estrutura do Projeto Arxis

```
arxis/
├── src/
│   ├── lib.rs                 # Biblioteca principal com prelude
│   │
│   ├── geometry/              # 📐 Módulo de Geometria
│   │   ├── mod.rs            
│   │   ├── quaternion3d.rs    # Quaternions 3D (Quat3D)
│   │   ├── dual_quaternion.rs # Quaternions duplos e SO(4)
│   │   └── geometry4d.rs      # Geometria 4D, politopos, projeções
│   │
│   ├── physics/               # 🔬 Módulo de Física
│   │   ├── mod.rs
│   │   └── relativity.rs      # Relatividade, Lorentz, tensores físicos
│   │
│   └── tensor/                # 📊 Módulo de Tensores
│       ├── mod.rs
│       ├── tensor.rs          # Tensores genéricos (ordem 0-2)
│       └── tensor4d.rs        # Tensores 3D e 4D, ML ops
│
├── examples/                  # Exemplos demonstrativos
│   ├── rotations_example.rs
│   ├── tensors_example.rs
│   ├── relativity_example.rs
│   ├── ml_example.rs
│   └── geometry4d_example.rs
│
├── docs/                      # Documentação
│   ├── GEOMETRY4D_GUIDE.md
│   ├── TENSOR_DOCUMENTATION.md
│   └── VISUALIZATION_GUIDE.md
│
├── Cargo.toml
├── README.md
└── .gitignore
```

## Convenções de Nomenclatura

### Módulos
- **snake_case**: `quaternion3d`, `dual_quaternion`, `geometry4d`
- Organizados por domínio: `geometry/`, `physics/`, `tensor/`

### Structs
- **PascalCase**: `Quat3D`, `DualQuat`, `Matrix4x4`, `Tesseract`
- Sufixos descritivos: `4D`, `3D`, `Metric`, `Transform`

### Funções
- **snake_case**: `rotate_vector`, `from_axis_angle`, `project_stereographic`
- Verbos descritivos para ações

### Constantes
- **SCREAMING_SNAKE_CASE**: (quando necessário)

## Imports Organizados

### Usuário final:
```rust
use arxis_quaternions::prelude::*;
```

### Específico por módulo:
```rust
use arxis_quaternions::geometry::{Quat3D, Tesseract};
use arxis_quaternions::physics::LorentzTransform;
use arxis_quaternions::tensor::{Matrix, Tensor};
```

## Benefícios da Nova Estrutura

✅ **Modular**: Cada domínio (geometria, física, tensores) separado  
✅ **Escalável**: Fácil adicionar novos módulos (ex: `physics/quantum.rs`)  
✅ **Organizado**: Imports claros refletem a hierarquia  
✅ **Documentado**: Cada módulo tem seu propósito bem definido  
✅ **Testável**: Testes organizados por módulo (32 testes passando)  
✅ **Profissional**: Segue padrões da comunidade Rust
