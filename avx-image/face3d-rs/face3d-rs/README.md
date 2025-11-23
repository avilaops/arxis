# face3d-rs 🎭

Biblioteca Rust para modelagem 3D de rostos, suportando os principais modelos paramétricos:

- **3DMM** (3D Morphable Models) - Modelos paramétricos lineares baseados em PCA
- **FLAME** - Faces Learned with an Articulated Model and Expressions
- **BFM** - Basel Face Model

## 🚀 Instalação

Adicione ao seu `Cargo.toml`:

```toml
[dependencies]
face3d-rs = "0.1.0"

# Com features opcionais:
face3d-rs = { version = "0.1.0", features = ["full"] }
```

## 📦 Features

- `default` - Funcionalidades básicas (3DMM, FLAME, BFM)
- `scientific-io` - Suporte para carregar arquivos HDF5, NPY
- `mesh-export` - Exportar meshes para OBJ, PLY
- `full` - Todas as features habilitadas

## 🎯 Exemplo Rápido

```rust
use face3d_rs::models::MorphableModel;
use nalgebra as na;

// Criar modelo 3DMM
let model = MorphableModel::new(
    na::DVector::zeros(300),
    na::DMatrix::zeros(300, 199),
    na::DMatrix::zeros(300, 199),
    na::DVector::zeros(300),
    vec![],
);

// Gerar face com parâmetros
let shape_params = na::DVector::from_element(199, 0.1);
let texture_params = na::DVector::from_element(199, 0.1);

let (shape, texture) = model.generate_face(&shape_params, &texture_params)?;
```

## 📚 Exemplos

Execute os exemplos incluídos:

```bash
# 3DMM básico
cargo run --example threedmm_basic

# FLAME com expressões
cargo run --example flame_expressions

# Basel Face Model
cargo run --example bfm_generation

# Projeção 3D → 2D
cargo run --example projection_demo
```

## 🧬 Modelos Suportados

### 3DMM (3D Morphable Model)

Modelo linear simples baseado em PCA:

```rust
use face3d_rs::models::MorphableModel;

let (shape, texture) = model.generate_face(&shape_params, &texture_params)?;
let vertices = model.shape_to_vertices(&shape);
```

**Características:**
- Linear e rápido
- Baseado em PCA
- Ideal para faces frontais

### FLAME

Modelo articulado com skeleton e expressões:

```rust
use face3d_rs::models::flame::{FlameModel, FlameBuilder};

let model = FlameBuilder::new()
    .n_vertices(5023)
    .n_shape_components(300)
    .n_expression_components(100)
    .n_joints(5)
    .build_empty();

let vertices = model.forward(&shape_params, &expr_params, &pose_params)?;
```

**Características:**
- Articulações (neck, jaw, eyeballs)
- Expressões faciais (100 componentes)
- Linear Blend Skinning (LBS)
- Ideal para animação

### Basel Face Model (BFM)

Modelo de alta qualidade baseado em scans 3D:

```rust
use face3d_rs::models::bfm::{BaselFaceModel, BfmBuilder};

let model = BfmBuilder::new()
    .n_vertices(53149)
    .n_shape_components(199)
    .n_color_components(199)
    .n_expression_components(100)
    .build_empty();

let (vertices, colors) = model.generate(&shape_coeffs, &color_coeffs, &expr_coeffs)?;
```

**Características:**
- 53k vértices, 105k triângulos
- Shape + Color + Expression
- Landmarks faciais
- Alta qualidade

## 📷 Projeção 3D → 2D

```rust
use face3d_rs::utils::projection::{PerspectiveCamera, perspective_projection};

let camera = PerspectiveCamera::new(1000.0, 640, 480);
let points_2d = perspective_projection(&points_3d, &camera);
```

Suporta:
- **Projeção perspectiva** (câmera pinhole)
- **Weak perspective** (ortográfica com escala)
- Matriz intrínseca K
- Look-at e view matrix

## 🛠️ Utilitários

### Fórmula de Rodrigues (axis-angle ↔ rotation matrix)

```rust
use face3d_rs::utils::rodrigues::axis_angle_to_matrix;
use nalgebra as na;

let axis_angle = na::DVector::from_vec(vec![0.0, 0.5, 0.0]);
let rotation_matrix = axis_angle_to_matrix(&axis_angle.as_slice());
```

## 📊 Comparação de Modelos

| Modelo    | Vértices | Parâmetros           | Articulado | Uso Principal            |
| --------- | -------- | -------------------- | ---------- | ------------------------ |
| **3DMM**  | Variável | Shape + Texture      | ❌          | Geração simples, fitting |
| **FLAME** | 5,023    | Shape + Expr + Pose  | ✅          | Animação, tracking       |
| **BFM**   | 53,149   | Shape + Color + Expr | ❌          | Alta qualidade, pesquisa |

## 🧪 Testes

Execute os testes:

```bash
# Todos os testes
cargo test

# Testes específicos
cargo test --lib models::threedmm
cargo test --lib models::flame
cargo test --lib models::bfm
```

## 📖 Documentação

Gere a documentação local:

```bash
cargo doc --open
```

## 🤝 Contribuindo

Contribuições são bem-vindas! Por favor:

1. Fork o repositório
2. Crie uma branch para sua feature (`git checkout -b feature/nova-feature`)
3. Commit suas mudanças (`git commit -am 'Adiciona nova feature'`)
4. Push para a branch (`git push origin feature/nova-feature`)
5. Abra um Pull Request

## 📝 Licença

MIT License - veja [LICENSE](LICENSE) para detalhes.

## 📚 Referências

- **3DMM**: Blanz, V., & Vetter, T. (1999). A morphable model for the synthesis of 3D faces.
- **FLAME**: Li, T., et al. (2017). Learning a model of facial shape and expression from 4D scans.
- **BFM**: Paysan, P., et al. (2009). A 3D face model for pose and illumination invariant face recognition.

## 🌟 Roadmap

- [ ] Carregar modelos FLAME oficiais (.pkl)
- [ ] Carregar BFM 2017/2019 (.h5)
- [ ] Fitting 2D → 3D (otimização)
- [ ] Renderização básica
- [ ] Suporte para GPU (wgpu)
- [ ] Python bindings (PyO3)

---

**Desenvolvido para AVL Cloud Platform** 🇧🇷
