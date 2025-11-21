# Operações Avançadas de Álgebra Linear - avila-linalg

## 🚀 Features para Engines AAA

### ✅ IMPLEMENTADO (v0.1.1)

#### 1. **Quaternions** (Rotações Avançadas)
```rust
use avila_linalg::Quaternion;

// Criação
let q = Quaternion::from_axis_angle(axis, angle);  // Eixo-ângulo
let q = Quaternion::from_euler(roll, pitch, yaw); // Ângulos de Euler
let q = Quaternion::identity();                    // Sem rotação

// Operações
q.normalize()              // Torna unitário
q.conjugate()              // Inverte rotação
q.rotate_vector(v)         // Rotaciona vetor 3D
q1 * q2                    // Composição de rotações

// Conversão
q.to_matrix3()             // → Matrix3x3
q.to_matrix4()             // → Matrix4x4 (homogêneas)

// Interpolação
q1.slerp(&q2, t)          // Interpolação esférica (animações)
q1.lerp(&q2, t)           // Interpolação linear (mais rápida)
```

**Por que Quaternions?**
- ✅ Evita **gimbal lock** (problema dos Euler angles)
- ✅ Interpolação **geodésica suave** (SLERP)
- ✅ **Composição eficiente** de rotações
- ✅ Apenas **4 floats** vs 9 da matriz 3×3
- ✅ Usado em **Unity, Unreal, Godot**, todas as engines AAA

---

#### 2. **Transformações 4D** (Coordenadas Homogêneas)

```rust
use avila_linalg::Matrix4x4;

// Transformações básicas
let t = Matrix4x4::translation(x, y, z);     // Translação
let s = Matrix4x4::scale(factor);            // Escala uniforme
let s = Matrix4x4::scale_xyz(x, y, z);       // Escala não-uniforme

// Rotações (Euler)
let rx = Matrix4x4::rotation_x(angle);       // Roll
let ry = Matrix4x4::rotation_y(angle);       // Pitch
let rz = Matrix4x4::rotation_z(angle);       // Yaw

// Composição TRS (ordem importa!)
let model = t * r * s;  // Translate → Rotate → Scale
```

**Coordenadas Homogêneas:**
```rust
// Ponto vs Direção
let point = Vector4::from_point(v3);         // w = 1
let direction = Vector4::from_direction(v3); // w = 0

// Transformação
let transformed = matrix * point;

// De volta para 3D
let v3 = transformed.to_vector3();  // Divide por w
```

---

#### 3. **Câmera & Projeção**

```rust
// Look-At Matrix (câmera)
let view = Matrix4x4::look_at(
    eye,     // Posição da câmera
    target,  // Ponto que está olhando
    up       // Vetor "para cima" (geralmente (0,1,0))
);

// Projeção Perspectiva
let projection = Matrix4x4::perspective(
    fovy,    // Campo de visão vertical (radianos)
    aspect,  // Aspect ratio (width/height)
    near,    // Plano near
    far      // Plano far
);

// Projeção Ortográfica (UI/HUD)
let ortho = Matrix4x4::orthographic(
    left, right,
    bottom, top,
    near, far
);
```

---

#### 4. **Pipeline Gráfico Completo**

```rust
// 1. Model Space → World Space
let world_vertex = model_matrix * local_vertex;

// 2. World Space → View Space (câmera)
let view_vertex = view_matrix * world_vertex;

// 3. View Space → Clip Space
let clip_vertex = projection_matrix * view_vertex;

// 4. Clip Space → NDC (Normalized Device Coordinates)
let ndc = Vector3::new(
    clip_vertex.x() / clip_vertex.w(),
    clip_vertex.y() / clip_vertex.w(),
    clip_vertex.z() / clip_vertex.w(),
);
// NDC ∈ [-1, 1] para X, Y, Z
```

---

#### 5. **Operações com Vector4**

```rust
// Operadores aritméticos
v1 + v2                // Adição
v1 - v2                // Subtração
v * scalar             // Multiplicação escalar

// Produto escalar
v1.dot(&v2)            // 4D dot product

// Norma
v.norm()               // ||v|| = √(x² + y² + z² + w²)
v.normalize()          // v̂ = v / ||v||
```

---

## 🎮 Casos de Uso para Engines AAA

### 1. **Sistema de Animação**
```rust
// Interpolar rotações suavemente
let current_rotation = Quaternion::from_euler(0.0, 0.0, 0.0);
let target_rotation = Quaternion::from_euler(0.0, PI, 0.0);

// A cada frame
let t = time / duration;  // 0.0 → 1.0
let interpolated = current_rotation.slerp(&target_rotation, t);

// Aplicar ao objeto
let rotation_matrix = interpolated.to_matrix4();
```

### 2. **Hierarquia de Transforms (Scene Graph)**
```rust
// Parent transform
let parent = Matrix4x4::translation(10.0, 0.0, 0.0) *
             Matrix4x4::rotation_y(PI / 4.0);

// Child local transform
let child_local = Matrix4x4::scale(0.5);

// World transform do child
let child_world = parent * child_local;
```

### 3. **Sistema de Câmera Third-Person**
```rust
let player_pos = Vector3::new(0.0, 0.0, 0.0);
let camera_offset = Quaternion::from_euler(0.0, yaw, 0.0)
    .rotate_vector(Vector3::new(0.0, 2.0, 5.0));

let camera_pos = player_pos + camera_offset;
let view_matrix = Matrix4x4::look_at(camera_pos, player_pos, UP);
```

### 4. **Física e Cinemática**
```rust
// Movimento de projétil
let velocity = Vector3::new(10.0, 15.0, 0.0);
let gravity = Vector3::new(0.0, -9.81, 0.0);

let new_velocity = velocity + gravity * dt;
let new_position = position + new_velocity * dt;
```

### 5. **Frustum Culling**
```rust
// Extrai planos do frustum da matriz view-projection
let vp = projection * view;

// Testa se AABB está no frustum
fn is_in_frustum(aabb_min: Vector3, aabb_max: Vector3, vp: Matrix4x4) -> bool {
    // Testar 8 vértices do AABB contra 6 planos
    // ...
}
```

---

## 📊 Comparação com Engines Comerciais

| Feature        | avila-linalg | glm (C++) | Unity | Unreal |
| -------------- | ------------ | --------- | ----- | ------ |
| Quaternions    | ✅            | ✅         | ✅     | ✅      |
| SLERP          | ✅            | ✅         | ✅     | ✅      |
| Matrix 4×4     | ✅            | ✅         | ✅     | ✅      |
| Look-At        | ✅            | ✅         | ✅     | ✅      |
| Perspective    | ✅            | ✅         | ✅     | ✅      |
| Composição TRS | ✅            | ✅         | ✅     | ✅      |
| Zero deps      | ✅            | ❌         | ❌     | ❌      |
| 100% Rust      | ✅            | ❌         | ❌     | ❌      |

---

## 🔬 Operações Muito Avançadas (Futuras)

### v0.2.0 - Decomposições
- [ ] **SVD** - Singular Value Decomposition (PCA, compressão)
- [ ] **QR Decomposition** - Sistemas lineares, eigenvalues
- [ ] **Eigenvalues/Eigenvectors** - Análise modal
- [ ] **Inversa 4×4 completa** - Via Gauss-Jordan ou LU

### v0.3.0 - Física Avançada
- [ ] **Tensor de inércia** - Física de corpo rígido
- [ ] **Dual Quaternions** - Rotação + translação unificadas
- [ ] **Exponential map** - so(3) ↔ SO(3)
- [ ] **Lie algebra** - Velocidades angulares

### v0.4.0 - Gráficos Avançados
- [ ] **Spherical harmonics** - Iluminação global
- [ ] **Matrix skinning** - Animação skeletal (blend de N matrizes)
- [ ] **Tangent space** - Normal mapping
- [ ] **Frustum extraction** - Culling automático

### v0.5.0 - Machine Learning
- [ ] **Tensor operations** - N-dimensional arrays
- [ ] **Backpropagation** - Autodiff de matrizes
- [ ] **Batch operations** - SIMD paralelo
- [ ] **GPU compute** - Via wgpu

---

## 💡 Dicas de Performance

### 1. **Prefira Quaternions para Rotações**
```rust
// ❌ Lento: 3 matrizes × 9 multiplicações cada
let r = Matrix4x4::rotation_x(a) *
        Matrix4x4::rotation_y(b) *
        Matrix4x4::rotation_z(c);

// ✅ Rápido: Quaternion × 16 multiplicações
let q = Quaternion::from_euler(a, b, c);
let r = q.to_matrix4();
```

### 2. **Cache Transformações Constantes**
```rust
// ❌ Recalcula toda hora
fn update() {
    let view = Matrix4x4::look_at(eye, target, up);
    // ...
}

// ✅ Calcula só quando muda
struct Camera {
    view: Matrix4x4<f32>,
    dirty: bool,
}

impl Camera {
    fn get_view(&mut self) -> Matrix4x4<f32> {
        if self.dirty {
            self.view = Matrix4x4::look_at(...);
            self.dirty = false;
        }
        self.view
    }
}
```

### 3. **Use Lerp quando SLERP não é necessário**
```rust
// Para ângulos pequenos (<15°), lerp é suficiente
let angle_difference = acos(q1.dot(&q2));
if angle_difference < 0.26 {  // ~15°
    interpolated = q1.lerp(&q2, t).normalize();  // Mais rápido
} else {
    interpolated = q1.slerp(&q2, t);  // Mais preciso
}
```

### 4. **Batch Operations (futuro: SIMD)**
```rust
// v0.3.0 terá:
let matrices: [Matrix4x4; 4] = [...];
let vertices: [Vector4; 4] = [...];

// Paralelo via SIMD (4 transforms simultâneos)
let transformed = matrices.transform_batch(&vertices);
```

---

## 🎯 Roadmap de Complexidade

### Básico (v0.1) ✅
- Vetores 2D/3D/4D/ND
- Matrizes 2×2, 3×3, 4×4, M×N
- Dot, cross, norm, transpose, det

### Intermediário (v0.1.1) ✅
- **Quaternions** com SLERP
- **Transformações 4D** (TRS)
- **Projeções** (perspectiva, ortho)
- **Câmera** (look-at)

### Avançado (v0.2)
- SVD, QR, Eigenvalues
- Inversa 4×4
- LU, Cholesky

### Expert (v0.3+)
- Dual quaternions
- Tensor operations
- SIMD/GPU acceleration
- Lie algebra

---

## 📚 Referências

### Papers Fundamentais
- **Quaternions**: Shoemake, K. (1985). "Animating rotation with quaternion curves"
- **SLERP**: Dam, E.B. et al. (1998). "Quaternions, Interpolation and Animation"
- **View Matrices**: Hughes, J.F. et al. (2013). "Computer Graphics: Principles and Practice"

### Engines de Referência
- **Unity**: Usa Quaternion + Matrix4x4 (mesma estrutura)
- **Unreal**: FQuat + FMatrix (C++, similar)
- **Godot**: Quat + Transform3D (GDScript/C++)
- **Bevy**: Quat + Mat4 (Rust, glam crate)

### Livros Recomendados
- "3D Math Primer for Graphics and Game Development" - Fletcher Dunn
- "Real-Time Rendering" - Tomas Akenine-Möller
- "Game Engine Architecture" - Jason Gregory

---

**Status:** v0.1.1 ✅ Pronto para engines AAA (operações básicas e intermediárias)
**Próximo:** v0.2.0 - SVD, Eigenvalues, Decomposições Avançadas
**Autor:** Nícolas Ávila <nicolas@avila.inc>
**Data:** 21 de Novembro de 2025
