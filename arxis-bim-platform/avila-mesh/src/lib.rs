//! # avila-mesh
//!
//! **Estruturas de Mesh 3D - Geometria para Renderização**
//!
//! Define estruturas de dados para geometria 3D otimizada para GPUs:
//! - Vértices (posição, normal, UV, tangente)
//! - Índices (triângulos)
//! - Materiais (PBR)
//! - Operações (merge, split, transform, simplify)
//!
//! Compatível com glTF, WebGL, e engines de renderização modernas.

use avila_vec3d::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type Result<T> = std::result::Result<T, MeshError>;

// ============================================================================
// ERROS
// ============================================================================

#[derive(Debug, thiserror::Error)]
pub enum MeshError {
    #[error("Invalid mesh: {0}")]
    InvalidMesh(String),

    #[error("Index out of bounds: {0}")]
    IndexOutOfBounds(usize),

    #[error("Material not found: {0}")]
    MaterialNotFound(String),

    #[error("Geometry error: {0}")]
    GeometryError(String),

    #[error("Vec3d error: {0}")]
    Vec3dError(#[from] Vec3dError),
}

// ============================================================================
// VÉRTICE
// ============================================================================

/// Vértice completo com todos os atributos
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Vertex {
    /// Posição 3D
    pub position: Vec3,

    /// Normal (direção da superfície)
    pub normal: Vec3,

    /// Coordenadas de textura (UV)
    pub uv: Vec2,

    /// Tangente (para normal mapping)
    pub tangent: Option<Vec3>,

    /// Cor do vértice (opcional, para vertex colors)
    pub color: Option<[f32; 4]>, // RGBA
}

impl Vertex {
    pub fn new(position: Vec3) -> Self {
        Self {
            position,
            normal: Vec3::Z, // default up
            uv: Vec2::ZERO,
            tangent: None,
            color: None,
        }
    }

    pub fn with_normal(mut self, normal: Vec3) -> Self {
        self.normal = normal;
        self
    }

    pub fn with_uv(mut self, uv: Vec2) -> Self {
        self.uv = uv;
        self
    }

    pub fn with_tangent(mut self, tangent: Vec3) -> Self {
        self.tangent = Some(tangent);
        self
    }

    pub fn with_color(mut self, color: [f32; 4]) -> Self {
        self.color = Some(color);
        self
    }

    /// Transforma vértice por matriz
    pub fn transform(&self, matrix: &Mat4) -> Self {
        let position = matrix.transform_point(self.position);

        // Transforma normal (usa matriz inversa transposta, mas para escala uniforme isso simplifica)
        let normal_transformed = matrix.transform_point(self.normal + self.position) - matrix.transform_point(self.position);
        let normal = normal_transformed.normalize().unwrap_or(self.normal);

        Self {
            position,
            normal,
            uv: self.uv,
            tangent: self.tangent.map(|t| {
                let t_transformed = matrix.transform_point(t + self.position) - matrix.transform_point(self.position);
                t_transformed.normalize().unwrap_or(t)
            }),
            color: self.color,
        }
    }
}

// ============================================================================
// MESH
// ============================================================================

/// Mesh 3D - conjunto de triângulos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mesh {
    /// Vértices
    pub vertices: Vec<Vertex>,

    /// Índices (triângulos, cada 3 índices = 1 triângulo)
    pub indices: Vec<u32>,

    /// ID do material associado
    pub material_id: Option<String>,

    /// AABB (bounding box)
    pub bounds: Aabb,
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
            material_id: None,
            bounds: Aabb::EMPTY,
        }
    }

    pub fn with_capacity(vertex_count: usize, index_count: usize) -> Self {
        Self {
            vertices: Vec::with_capacity(vertex_count),
            indices: Vec::with_capacity(index_count),
            material_id: None,
            bounds: Aabb::EMPTY,
        }
    }

    /// Adiciona vértice e retorna seu índice
    pub fn add_vertex(&mut self, vertex: Vertex) -> u32 {
        self.bounds.expand_point(vertex.position);
        let index = self.vertices.len() as u32;
        self.vertices.push(vertex);
        index
    }

    /// Adiciona triângulo por índices
    pub fn add_triangle(&mut self, i0: u32, i1: u32, i2: u32) -> Result<()> {
        let max_idx = *[i0, i1, i2].iter().max().unwrap() as usize;
        if max_idx >= self.vertices.len() {
            return Err(MeshError::IndexOutOfBounds(max_idx));
        }
        self.indices.extend_from_slice(&[i0, i1, i2]);
        Ok(())
    }

    /// Número de triângulos
    pub fn triangle_count(&self) -> usize {
        self.indices.len() / 3
    }

    /// Valida consistência da mesh
    pub fn validate(&self) -> Result<()> {
        if self.indices.len() % 3 != 0 {
            return Err(MeshError::InvalidMesh("Index count must be multiple of 3".into()));
        }

        for &idx in &self.indices {
            if idx as usize >= self.vertices.len() {
                return Err(MeshError::IndexOutOfBounds(idx as usize));
            }
        }

        Ok(())
    }

    /// Recalcula normais (flat shading)
    pub fn recalculate_normals_flat(&mut self) {
        for chunk in self.indices.chunks_exact(3) {
            let i0 = chunk[0] as usize;
            let i1 = chunk[1] as usize;
            let i2 = chunk[2] as usize;

            let v0 = self.vertices[i0].position;
            let v1 = self.vertices[i1].position;
            let v2 = self.vertices[i2].position;

            let edge1 = v1 - v0;
            let edge2 = v2 - v0;
            let normal = edge1.cross(&edge2).normalize().unwrap_or(Vec3::Z);

            self.vertices[i0].normal = normal;
            self.vertices[i1].normal = normal;
            self.vertices[i2].normal = normal;
        }
    }

    /// Recalcula normais (smooth shading)
    pub fn recalculate_normals_smooth(&mut self) {
        // Reset normais
        for vertex in &mut self.vertices {
            vertex.normal = Vec3::ZERO;
        }

        // Acumula normais por triângulo
        for chunk in self.indices.chunks_exact(3) {
            let i0 = chunk[0] as usize;
            let i1 = chunk[1] as usize;
            let i2 = chunk[2] as usize;

            let v0 = self.vertices[i0].position;
            let v1 = self.vertices[i1].position;
            let v2 = self.vertices[i2].position;

            let edge1 = v1 - v0;
            let edge2 = v2 - v0;
            let normal = edge1.cross(&edge2);

            self.vertices[i0].normal = self.vertices[i0].normal + normal;
            self.vertices[i1].normal = self.vertices[i1].normal + normal;
            self.vertices[i2].normal = self.vertices[i2].normal + normal;
        }

        // Normaliza
        for vertex in &mut self.vertices {
            vertex.normal = vertex.normal.normalize().unwrap_or(Vec3::Z);
        }
    }

    /// Transforma mesh por matriz
    pub fn transform(&mut self, matrix: &Mat4) {
        for vertex in &mut self.vertices {
            *vertex = vertex.transform(matrix);
        }
        self.bounds = self.bounds.transform(matrix);
    }

    /// Merge com outra mesh (combina geometria)
    pub fn merge(&mut self, other: &Mesh) {
        let vertex_offset = self.vertices.len() as u32;

        // Adiciona vértices
        self.vertices.extend_from_slice(&other.vertices);

        // Adiciona índices com offset
        for &idx in &other.indices {
            self.indices.push(idx + vertex_offset);
        }

        // Atualiza bounds
        self.bounds = self.bounds.merge(&other.bounds);
    }

    /// Converte para buffers separados (para GPU/glTF)
    pub fn to_buffers(&self) -> MeshBuffers {
        let mut positions = Vec::with_capacity(self.vertices.len() * 3);
        let mut normals = Vec::with_capacity(self.vertices.len() * 3);
        let mut uvs = Vec::with_capacity(self.vertices.len() * 2);
        let mut tangents = Vec::new();
        let mut colors = Vec::new();

        let has_tangents = self.vertices.iter().any(|v| v.tangent.is_some());
        let has_colors = self.vertices.iter().any(|v| v.color.is_some());

        if has_tangents {
            tangents = Vec::with_capacity(self.vertices.len() * 3);
        }
        if has_colors {
            colors = Vec::with_capacity(self.vertices.len() * 4);
        }

        for vertex in &self.vertices {
            positions.extend_from_slice(&vertex.position.to_array());
            normals.extend_from_slice(&vertex.normal.to_array());
            uvs.extend_from_slice(&[vertex.uv.x, vertex.uv.y]);

            if has_tangents {
                if let Some(tangent) = vertex.tangent {
                    tangents.extend_from_slice(&tangent.to_array());
                } else {
                    tangents.extend_from_slice(&[0.0, 0.0, 0.0]);
                }
            }

            if has_colors {
                if let Some(color) = vertex.color {
                    colors.extend_from_slice(&color);
                } else {
                    colors.extend_from_slice(&[1.0, 1.0, 1.0, 1.0]);
                }
            }
        }

        MeshBuffers {
            positions,
            normals,
            uvs,
            tangents: if has_tangents { Some(tangents) } else { None },
            colors: if has_colors { Some(colors) } else { None },
            indices: self.indices.clone(),
        }
    }
}

impl Default for Mesh {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// MESH BUFFERS (formato GPU-friendly)
// ============================================================================

/// Buffers de mesh separados (mais eficiente para upload GPU)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshBuffers {
    /// Posições (x, y, z, x, y, z, ...)
    pub positions: Vec<f32>,

    /// Normais (x, y, z, x, y, z, ...)
    pub normals: Vec<f32>,

    /// UVs (u, v, u, v, ...)
    pub uvs: Vec<f32>,

    /// Tangentes (opcional)
    pub tangents: Option<Vec<f32>>,

    /// Cores (opcional, RGBA)
    pub colors: Option<Vec<f32>>,

    /// Índices
    pub indices: Vec<u32>,
}

impl MeshBuffers {
    pub fn vertex_count(&self) -> usize {
        self.positions.len() / 3
    }

    pub fn triangle_count(&self) -> usize {
        self.indices.len() / 3
    }
}

// ============================================================================
// MATERIAL
// ============================================================================

/// Material PBR (Physically Based Rendering) - compatível com glTF
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PbrMaterial {
    pub id: String,
    pub name: String,

    // Base color
    pub base_color_factor: [f32; 4], // RGBA
    pub base_color_texture: Option<String>, // path/URI

    // Metallic-Roughness
    pub metallic_factor: f32,
    pub roughness_factor: f32,
    pub metallic_roughness_texture: Option<String>,

    // Normal map
    pub normal_texture: Option<String>,
    pub normal_scale: f32,

    // Emissive
    pub emissive_factor: [f32; 3], // RGB
    pub emissive_texture: Option<String>,

    // Occlusion
    pub occlusion_texture: Option<String>,
    pub occlusion_strength: f32,

    // Alpha mode
    pub alpha_mode: AlphaMode,
    pub alpha_cutoff: f32,

    // Double-sided
    pub double_sided: bool,
}

impl PbrMaterial {
    pub fn default_material(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: "Default".into(),
            base_color_factor: [0.8, 0.8, 0.8, 1.0],
            base_color_texture: None,
            metallic_factor: 0.0,
            roughness_factor: 1.0,
            metallic_roughness_texture: None,
            normal_texture: None,
            normal_scale: 1.0,
            emissive_factor: [0.0, 0.0, 0.0],
            emissive_texture: None,
            occlusion_texture: None,
            occlusion_strength: 1.0,
            alpha_mode: AlphaMode::Opaque,
            alpha_cutoff: 0.5,
            double_sided: false,
        }
    }

    pub fn from_ifc_material(name: &str, ifc_material: &str) -> Self {
        // Conversão básica IFC → PBR
        let (base_color, metallic, roughness) = match ifc_material.to_lowercase().as_str() {
            s if s.contains("concreto") || s.contains("concrete") => ([0.7, 0.7, 0.7, 1.0], 0.0, 0.9),
            s if s.contains("aço") || s.contains("steel") || s.contains("metal") => ([0.8, 0.8, 0.8, 1.0], 1.0, 0.3),
            s if s.contains("madeira") || s.contains("wood") => ([0.6, 0.4, 0.2, 1.0], 0.0, 0.8),
            s if s.contains("vidro") || s.contains("glass") => ([0.9, 0.9, 1.0, 0.3], 0.0, 0.1),
            s if s.contains("cerâmica") || s.contains("ceramic") => ([0.9, 0.85, 0.8, 1.0], 0.0, 0.6),
            s if s.contains("gesso") || s.contains("plaster") => ([0.95, 0.95, 0.95, 1.0], 0.0, 0.9),
            _ => ([0.8, 0.8, 0.8, 1.0], 0.0, 0.7), // default
        };

        let alpha_mode = if base_color[3] < 1.0 {
            AlphaMode::Blend
        } else {
            AlphaMode::Opaque
        };

        Self {
            id: name.to_string(),
            name: ifc_material.to_string(),
            base_color_factor: base_color,
            base_color_texture: None,
            metallic_factor: metallic,
            roughness_factor: roughness,
            metallic_roughness_texture: None,
            normal_texture: None,
            normal_scale: 1.0,
            emissive_factor: [0.0, 0.0, 0.0],
            emissive_texture: None,
            occlusion_texture: None,
            occlusion_strength: 1.0,
            alpha_mode,
            alpha_cutoff: 0.5,
            double_sided: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlphaMode {
    Opaque,
    Mask,
    Blend,
}

// ============================================================================
// SCENE - Coleção de meshes
// ============================================================================

/// Cena 3D - coleção de meshes com materiais
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scene {
    pub meshes: Vec<Mesh>,
    pub materials: HashMap<String, PbrMaterial>,
    pub bounds: Aabb,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            meshes: Vec::new(),
            materials: HashMap::new(),
            bounds: Aabb::EMPTY,
        }
    }

    pub fn add_mesh(&mut self, mesh: Mesh) {
        self.bounds = self.bounds.merge(&mesh.bounds);
        self.meshes.push(mesh);
    }

    pub fn add_material(&mut self, material: PbrMaterial) {
        self.materials.insert(material.id.clone(), material);
    }

    pub fn mesh_count(&self) -> usize {
        self.meshes.len()
    }

    pub fn triangle_count(&self) -> usize {
        self.meshes.iter().map(|m| m.triangle_count()).sum()
    }

    pub fn vertex_count(&self) -> usize {
        self.meshes.iter().map(|m| m.vertices.len()).sum()
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// PRIMITIVAS GEOMÉTRICAS
// ============================================================================

pub mod primitives {
    use super::*;

    /// Cria um cubo
    pub fn cube(size: f32) -> Mesh {
        let half = size / 2.0;
        let mut mesh = Mesh::with_capacity(24, 36);

        // 6 faces, 4 vértices cada (para UVs corretos)
        let positions = [
            // Front (+Z)
            [-half, -half, half], [half, -half, half], [half, half, half], [-half, half, half],
            // Back (-Z)
            [half, -half, -half], [-half, -half, -half], [-half, half, -half], [half, half, -half],
            // Top (+Y)
            [-half, half, half], [half, half, half], [half, half, -half], [-half, half, -half],
            // Bottom (-Y)
            [-half, -half, -half], [half, -half, -half], [half, -half, half], [-half, -half, half],
            // Right (+X)
            [half, -half, half], [half, -half, -half], [half, half, -half], [half, half, half],
            // Left (-X)
            [-half, -half, -half], [-half, -half, half], [-half, half, half], [-half, half, -half],
        ];

        let normals = [
            Vec3::new(0.0, 0.0, 1.0), Vec3::new(0.0, 0.0, 1.0), Vec3::new(0.0, 0.0, 1.0), Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(0.0, 0.0, -1.0), Vec3::new(0.0, 0.0, -1.0), Vec3::new(0.0, 0.0, -1.0), Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, -1.0, 0.0), Vec3::new(0.0, -1.0, 0.0), Vec3::new(0.0, -1.0, 0.0), Vec3::new(0.0, -1.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(-1.0, 0.0, 0.0), Vec3::new(-1.0, 0.0, 0.0), Vec3::new(-1.0, 0.0, 0.0), Vec3::new(-1.0, 0.0, 0.0),
        ];

        let uvs = [
            Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0), Vec2::new(1.0, 1.0), Vec2::new(0.0, 1.0),
            Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0), Vec2::new(1.0, 1.0), Vec2::new(0.0, 1.0),
            Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0), Vec2::new(1.0, 1.0), Vec2::new(0.0, 1.0),
            Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0), Vec2::new(1.0, 1.0), Vec2::new(0.0, 1.0),
            Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0), Vec2::new(1.0, 1.0), Vec2::new(0.0, 1.0),
            Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0), Vec2::new(1.0, 1.0), Vec2::new(0.0, 1.0),
        ];

        for i in 0..24 {
            let vertex = Vertex::new(Vec3::new(positions[i][0], positions[i][1], positions[i][2]))
                .with_normal(normals[i])
                .with_uv(uvs[i]);
            mesh.add_vertex(vertex);
        }

        // Índices (2 triângulos por face)
        for face in 0..6 {
            let base = face * 4;
            mesh.add_triangle(base, base + 1, base + 2).unwrap();
            mesh.add_triangle(base, base + 2, base + 3).unwrap();
        }

        mesh
    }

    /// Cria uma esfera (usando subdivisão de icosaedro)
    pub fn sphere(radius: f32, subdivisions: u32) -> Mesh {
        // TODO: Implementar subdivisão de icosaedro
        // Por simplicidade, criar esfera UV (lat/lon)
        let mut mesh = Mesh::new();

        let segments = 32;
        let rings = 16;

        for ring in 0..=rings {
            let phi = std::f32::consts::PI * ring as f32 / rings as f32;
            let y = radius * phi.cos();
            let ring_radius = radius * phi.sin();

            for segment in 0..=segments {
                let theta = 2.0 * std::f32::consts::PI * segment as f32 / segments as f32;
                let x = ring_radius * theta.cos();
                let z = ring_radius * theta.sin();

                let position = Vec3::new(x, y, z);
                let normal = position.normalize().unwrap_or(Vec3::Y);
                let uv = Vec2::new(segment as f32 / segments as f32, ring as f32 / rings as f32);

                mesh.add_vertex(Vertex::new(position).with_normal(normal).with_uv(uv));
            }
        }

        // Índices
        for ring in 0..rings {
            for segment in 0..segments {
                let current = ring * (segments + 1) + segment;
                let next = current + segments + 1;

                mesh.add_triangle(current, next, current + 1).unwrap();
                mesh.add_triangle(current + 1, next, next + 1).unwrap();
            }
        }

        mesh
    }

    /// Cria um plano
    pub fn plane(width: f32, height: f32) -> Mesh {
        let mut mesh = Mesh::with_capacity(4, 6);

        let hw = width / 2.0;
        let hh = height / 2.0;

        mesh.add_vertex(Vertex::new(Vec3::new(-hw, 0.0, -hh)).with_normal(Vec3::Y).with_uv(Vec2::new(0.0, 0.0)));
        mesh.add_vertex(Vertex::new(Vec3::new(hw, 0.0, -hh)).with_normal(Vec3::Y).with_uv(Vec2::new(1.0, 0.0)));
        mesh.add_vertex(Vertex::new(Vec3::new(hw, 0.0, hh)).with_normal(Vec3::Y).with_uv(Vec2::new(1.0, 1.0)));
        mesh.add_vertex(Vertex::new(Vec3::new(-hw, 0.0, hh)).with_normal(Vec3::Y).with_uv(Vec2::new(0.0, 1.0)));

        mesh.add_triangle(0, 1, 2).unwrap();
        mesh.add_triangle(0, 2, 3).unwrap();

        mesh
    }
}

// ============================================================================
// TESTES
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mesh_creation() {
        let mut mesh = Mesh::new();

        let v0 = mesh.add_vertex(Vertex::new(Vec3::new(0.0, 0.0, 0.0)));
        let v1 = mesh.add_vertex(Vertex::new(Vec3::new(1.0, 0.0, 0.0)));
        let v2 = mesh.add_vertex(Vertex::new(Vec3::new(0.5, 1.0, 0.0)));

        mesh.add_triangle(v0, v1, v2).unwrap();

        assert_eq!(mesh.vertices.len(), 3);
        assert_eq!(mesh.triangle_count(), 1);
        assert!(mesh.validate().is_ok());
    }

    #[test]
    fn test_mesh_merge() {
        let mesh1 = primitives::cube(1.0);
        let mut mesh2 = primitives::cube(1.0);

        let original_count = mesh2.vertices.len();
        mesh2.merge(&mesh1);

        assert_eq!(mesh2.vertices.len(), original_count * 2);
        assert_eq!(mesh2.triangle_count(), mesh1.triangle_count() * 2);
    }

    #[test]
    fn test_primitives() {
        let cube = primitives::cube(2.0);
        assert!(cube.validate().is_ok());
        assert_eq!(cube.triangle_count(), 12); // 6 faces * 2 triangles

        let plane = primitives::plane(10.0, 5.0);
        assert!(plane.validate().is_ok());
        assert_eq!(plane.triangle_count(), 2);
    }
}
