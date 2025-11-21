# Changelog - avila-linalg

Todas as mudanças notáveis neste projeto serão documentadas aqui.

O formato é baseado em [Keep a Changelog](https://keepachangelog.com/pt-BR/1.0.0/),
e este projeto adere ao [Semantic Versioning](https://semver.org/lang/pt-BR/).

## [Unreleased]

## [0.1.1] - 2025-11-21

### ✨ Added (Novo)

#### Transformações Avançadas (módulo `transform.rs`)
- **Quaternions** para rotações sem gimbal lock
  - `Quaternion::from_axis_angle()` - Rotação por eixo-ângulo
  - `Quaternion::from_euler()` - Conversão de ângulos de Euler
  - `Quaternion::rotate_vector()` - Rotaciona Vector3 diretamente
  - `Quaternion::to_matrix3()` / `to_matrix4()` - Conversão para matrizes
  - `Quaternion::slerp()` - Interpolação esférica (animações suaves)
  - `Quaternion::lerp()` - Interpolação linear (mais rápida)
  - Multiplicação de quaternions (composição de rotações)
  - `conjugate()` e `normalize()`

#### Matrizes 4×4 Avançadas
- `Matrix4x4::translation()` - Matriz de translação
- `Matrix4x4::scale()` / `scale_xyz()` - Matrizes de escala
- `Matrix4x4::rotation_x/y/z()` - Rotações em cada eixo
- `Matrix4x4::look_at()` - Matriz de câmera (view matrix)
- `Matrix4x4::perspective()` - Projeção perspectiva (FOV-based)
- `Matrix4x4::orthographic()` - Projeção ortográfica (UI/HUD)
- `Matrix4x4 * Matrix4x4` - Multiplicação de matrizes 4×4
- `Matrix4x4 * Vector4` - Transformação de vetores 4D

#### Coordenadas Homogêneas (Vector4)
- `Vector4::from_point()` - Converte Vector3 em ponto (w=1)
- `Vector4::from_direction()` - Converte Vector3 em direção (w=0)
- `Vector4::to_vector3()` - Converte de volta (divide por w)
- Operadores aritméticos para Vector4 (+, -, * escalar)

#### Documentação
- `ADVANCED.md` - Guia completo de operações avançadas para engines AAA
- `examples/engine_aaa.rs` - Demo completo de pipeline gráfico 3D/4D

### 🔧 Changed (Mudanças)

- `Matrix3x3` e `Matrix4x4` agora expõem método `data()` para acesso aos dados internos
- Módulo `transform` integrado ao `prelude` para fácil importação

### 📚 Documentation

- Documentação completa em português para todas as novas features
- Exemplos de uso em engines AAA (Unity, Unreal, Godot)
- Comparações com bibliotecas concorrentes (glm, nalgebra)
- Guias de performance e boas práticas

### 🧪 Tests

- 3 novos testes para quaternions (identity, rotation, translation)
- Total: 12 testes passando

---

## [0.1.0] - 2025-11-21

### ✨ Added (Inicial)

#### Vetores
- `Vector2<T>`, `Vector3<T>`, `Vector4<T>` - Vetores genéricos de tamanho fixo
- `VectorN<T>` - Vetor de tamanho dinâmico
- Operações: `dot()`, `cross()` (3D), `norm()`, `normalize()`
- Operadores aritméticos: `Add`, `Sub`, `Mul<T>`

#### Matrizes
- `Matrix2x2<T>`, `Matrix3x3<T>`, `Matrix4x4<T>` - Matrizes genéricas
- `MatrixMxN<T>` - Matriz de tamanho dinâmico
- Operações: `transpose()`, `det()` (2×2, 3×3), `trace()`, `inverse()` (3×3)
- Multiplicação: `Matrix × Matrix`, `Matrix × Vector`

#### Operações Genéricas
- Traits: `Norm`, `Normalize`, `Dot`, `Cross`
- Funções utilitárias: `distance()`, `lerp()`, `clamp()`

#### Documentação
- `README.md` - Filosofia 100% Avila, zero bloat
- `STATUS.md` - Status detalhado do projeto
- `MIGRATION.md` - Guia de migração nalgebra → avila-linalg
- `ROADMAP.md` - Roadmap técnico v0.1 → v1.0
- `SUMMARY.md` - Resumo executivo
- `examples/basic_usage.rs` - Exemplo funcional

### 🎯 Features

- **Zero bloat**: apenas 1 dependência (num-traits)
- **Genérico**: funciona com f32, f64, e outros tipos numéricos
- **Testado**: 9 testes unitários passando
- **Documentado**: 3000+ linhas de documentação em PT-BR
- **Educacional**: código legível e bem comentado

---

## Tipos de Mudanças

- `Added` - Novas features
- `Changed` - Mudanças em features existentes
- `Deprecated` - Features que serão removidas
- `Removed` - Features removidas
- `Fixed` - Correções de bugs
- `Security` - Correções de segurança

[Unreleased]: https://github.com/avilaops/arxis/compare/avila-linalg-v0.1.1...HEAD
[0.1.1]: https://github.com/avilaops/arxis/compare/avila-linalg-v0.1.0...avila-linalg-v0.1.1
[0.1.0]: https://github.com/avilaops/arxis/releases/tag/avila-linalg-v0.1.0
