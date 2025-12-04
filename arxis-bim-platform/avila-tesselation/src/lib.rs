//! # avila-tesselation
//!
//! **Engine de Tesselação - Conversão de Sólidos IFC em Triângulos**
//!
//! Converte representações geométricas de alto nível (IFC) em meshes trianguladas:
//! - Extruded Solids (perfis extrudados)
//! - BRep (Boundary Representation)
//! - CSG (Constructive Solid Geometry)
//! - Swept Solids
//!
//! Pipeline: IFC Geometry → Tesselação → Mesh 3D

use avila_vec3d::*;
use avila_mesh::*;
use serde::{Deserialize, Serialize};

pub type Result<T> = std::result::Result<T, TesselationError>;

// ============================================================================
// ERROS
// ============================================================================

#[derive(Debug, thiserror::Error)]
pub enum TesselationError {
    #[error("Invalid geometry: {0}")]
    InvalidGeometry(String),

    #[error("Unsupported geometry type: {0}")]
    UnsupportedGeometry(String),

    #[error("Tesselation failed: {0}")]
    TesselationFailed(String),

    #[error("Vec3d error: {0}")]
    Vec3dError(#[from] Vec3dError),

    #[error("Mesh error: {0}")]
    MeshError(#[from] avila_mesh::MeshError),
}

// ============================================================================
// GEOMETRIA IFC (representações de alto nível)
// ============================================================================

/// Tipos de geometria IFC
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IfcGeometry {
    /// Sólido extrudado (perfil 2D + direção/distância)
    ExtrudedAreaSolid {
        profile: Vec<Vec2>,
        extrusion_direction: Vec3,
        depth: f32,
    },

    /// Caixa (box)
    Box {
        center: Vec3,
        size: Vec3,
    },

    /// Cilindro
    Cylinder {
        base_center: Vec3,
        radius: f32,
        height: f32,
    },

    /// Esfera
    Sphere {
        center: Vec3,
        radius: f32,
    },

    /// Malha triangulada (já tesselada)
    TriangulatedMesh {
        vertices: Vec<Vec3>,
        indices: Vec<u32>,
    },

    /// BRep (Boundary Representation) - faces, edges, vertices
    Brep {
        faces: Vec<BrepFace>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrepFace {
    pub outer_bound: Vec<Vec3>,
    pub inner_bounds: Vec<Vec<Vec3>>,
}

// ============================================================================
// TESSELATOR PRINCIPAL
// ============================================================================

pub struct Tesselator {
    tolerance: f32, // Tolerância para curvas/aproximações
}

impl Tesselator {
    pub fn new() -> Self {
        Self { tolerance: 0.01 }
    }

    pub fn with_tolerance(tolerance: f32) -> Self {
        Self { tolerance }
    }

    /// Converte geometria IFC em mesh
    pub fn tesselate(&self, geometry: &IfcGeometry) -> Result<Mesh> {
        match geometry {
            IfcGeometry::ExtrudedAreaSolid { profile, extrusion_direction, depth } => {
                self.tesselate_extruded_solid(profile, *extrusion_direction, *depth)
            }
            IfcGeometry::Box { center, size } => {
                Ok(self.tesselate_box(*center, *size))
            }
            IfcGeometry::Cylinder { base_center, radius, height } => {
                Ok(self.tesselate_cylinder(*base_center, *radius, *height))
            }
            IfcGeometry::Sphere { center, radius } => {
                Ok(self.tesselate_sphere(*center, *radius))
            }
            IfcGeometry::TriangulatedMesh { vertices, indices } => {
                self.tesselate_from_triangles(vertices, indices)
            }
            IfcGeometry::Brep { faces } => {
                self.tesselate_brep(faces)
            }
        }
    }

    // ========================================================================
    // EXTRUDED SOLID
    // ========================================================================

    fn tesselate_extruded_solid(&self, profile: &[Vec2], direction: Vec3, depth: f32) -> Result<Mesh> {
        if profile.len() < 3 {
            return Err(TesselationError::InvalidGeometry(
                "Profile must have at least 3 points".into()
            ));
        }

        let mut mesh = Mesh::new();

        // Normalizar direção
        let dir = direction.normalize()?;

        // Criar perfil bottom e top
        let bottom_verts: Vec<Vec3> = profile.iter()
            .map(|p| Vec3::new(p.x, 0.0, p.y))
            .collect();

        let top_verts: Vec<Vec3> = bottom_verts.iter()
            .map(|v| *v + dir * depth)
            .collect();

        // Adicionar vértices
        let bottom_indices: Vec<u32> = bottom_verts.iter()
            .map(|&v| {
                let normal = -dir; // Normal para baixo
                mesh.add_vertex(Vertex::new(v).with_normal(normal))
            })
            .collect();

        let top_indices: Vec<u32> = top_verts.iter()
            .map(|&v| {
                let normal = dir; // Normal para cima
                mesh.add_vertex(Vertex::new(v).with_normal(normal))
            })
            .collect();

        // Triangular bottom face (fan triangulation)
        for i in 1..bottom_indices.len() - 1 {
            mesh.add_triangle(bottom_indices[0], bottom_indices[i + 1], bottom_indices[i])?;
        }

        // Triangular top face
        for i in 1..top_indices.len() - 1 {
            mesh.add_triangle(top_indices[0], top_indices[i], top_indices[i + 1])?;
        }

        // Faces laterais
        for i in 0..profile.len() {
            let next = (i + 1) % profile.len();

            let b0 = bottom_verts[i];
            let b1 = bottom_verts[next];
            let t0 = top_verts[i];
            let t1 = top_verts[next];

            // Calcular normal da face lateral
            let edge1 = t0 - b0;
            let edge2 = b1 - b0;
            let normal = edge1.cross(&edge2).normalize().unwrap_or(Vec3::X);

            // Adicionar 4 vértices da face lateral (quad)
            let v0 = mesh.add_vertex(Vertex::new(b0).with_normal(normal));
            let v1 = mesh.add_vertex(Vertex::new(b1).with_normal(normal));
            let v2 = mesh.add_vertex(Vertex::new(t1).with_normal(normal));
            let v3 = mesh.add_vertex(Vertex::new(t0).with_normal(normal));

            // 2 triângulos
            mesh.add_triangle(v0, v1, v2)?;
            mesh.add_triangle(v0, v2, v3)?;
        }

        Ok(mesh)
    }

    // ========================================================================
    // BOX
    // ========================================================================

    fn tesselate_box(&self, center: Vec3, size: Vec3) -> Mesh {
        let mut mesh = primitives::cube(1.0);

        // Escalar e transladar
        let transform = Mat4::translation(center).mul_mat4(&Mat4::scale(size));
        mesh.transform(&transform);

        mesh
    }

    // ========================================================================
    // CYLINDER
    // ========================================================================

    fn tesselate_cylinder(&self, base_center: Vec3, radius: f32, height: f32) -> Mesh {
        let mut mesh = Mesh::new();

        let segments = 32;

        // Bottom circle
        let bottom_center_idx = mesh.add_vertex(
            Vertex::new(base_center).with_normal(Vec3::new(0.0, -1.0, 0.0))
        );

        for i in 0..segments {
            let theta = 2.0 * std::f32::consts::PI * i as f32 / segments as f32;
            let x = radius * theta.cos();
            let z = radius * theta.sin();
            let pos = base_center + Vec3::new(x, 0.0, z);
            mesh.add_vertex(Vertex::new(pos).with_normal(Vec3::new(0.0, -1.0, 0.0)));
        }

        // Bottom triangles
        for i in 0..segments {
            let next = (i + 1) % segments;
            mesh.add_triangle(bottom_center_idx, (i + 1) as u32, (next + 1) as u32).unwrap();
        }

        // Top circle
        let top_center = base_center + Vec3::new(0.0, height, 0.0);
        let top_center_idx = mesh.add_vertex(
            Vertex::new(top_center).with_normal(Vec3::new(0.0, 1.0, 0.0))
        );

        let top_start = mesh.vertices.len() as u32;
        for i in 0..segments {
            let theta = 2.0 * std::f32::consts::PI * i as f32 / segments as f32;
            let x = radius * theta.cos();
            let z = radius * theta.sin();
            let pos = top_center + Vec3::new(x, 0.0, z);
            mesh.add_vertex(Vertex::new(pos).with_normal(Vec3::new(0.0, 1.0, 0.0)));
        }

        // Top triangles
        for i in 0..segments {
            let next = (i + 1) % segments;
            mesh.add_triangle(top_center_idx, (top_start + next), (top_start + i)).unwrap();
        }

        // Side faces
        for i in 0..segments {
            let next = (i + 1) % segments;

            let b0 = (i + 1) as u32;
            let b1 = (next + 1) as u32;
            let t0 = top_start + i;
            let t1 = top_start + next;

            mesh.add_triangle(b0, b1, t1).unwrap();
            mesh.add_triangle(b0, t1, t0).unwrap();
        }

        mesh.recalculate_normals_smooth();
        mesh
    }

    // ========================================================================
    // SPHERE
    // ========================================================================

    fn tesselate_sphere(&self, center: Vec3, radius: f32) -> Mesh {
        let mut mesh = primitives::sphere(radius, 2);
        mesh.transform(&Mat4::translation(center));
        mesh
    }

    // ========================================================================
    // TRIANGULATED MESH
    // ========================================================================

    fn tesselate_from_triangles(&self, vertices: &[Vec3], indices: &[u32]) -> Result<Mesh> {
        let mut mesh = Mesh::with_capacity(vertices.len(), indices.len());

        for &v in vertices {
            mesh.add_vertex(Vertex::new(v));
        }

        for chunk in indices.chunks_exact(3) {
            mesh.add_triangle(chunk[0], chunk[1], chunk[2])?;
        }

        mesh.recalculate_normals_smooth();
        Ok(mesh)
    }

    // ========================================================================
    // BREP (simplificado - triangula faces planares)
    // ========================================================================

    fn tesselate_brep(&self, faces: &[BrepFace]) -> Result<Mesh> {
        let mut mesh = Mesh::new();

        for face in faces {
            // Triangular outer bound (ear clipping simplificado)
            let outer = &face.outer_bound;
            if outer.len() < 3 {
                continue;
            }

            // Fan triangulation (assume face convexa)
            let base_idx = mesh.add_vertex(Vertex::new(outer[0]));
            for i in 1..outer.len() - 1 {
                let v1 = mesh.add_vertex(Vertex::new(outer[i]));
                let v2 = mesh.add_vertex(Vertex::new(outer[i + 1]));
                mesh.add_triangle(base_idx, v1, v2)?;
            }

            // TODO: inner bounds (furos)
        }

        mesh.recalculate_normals_smooth();
        Ok(mesh)
    }
}

impl Default for Tesselator {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// TESTES
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tesselate_box() {
        let tesselator = Tesselator::new();
        let geometry = IfcGeometry::Box {
            center: Vec3::ZERO,
            size: Vec3::ONE,
        };

        let mesh = tesselator.tesselate(&geometry).unwrap();
        assert!(mesh.validate().is_ok());
        assert!(mesh.triangle_count() > 0);
    }

    #[test]
    fn test_tesselate_cylinder() {
        let tesselator = Tesselator::new();
        let geometry = IfcGeometry::Cylinder {
            base_center: Vec3::ZERO,
            radius: 1.0,
            height: 2.0,
        };

        let mesh = tesselator.tesselate(&geometry).unwrap();
        assert!(mesh.validate().is_ok());
        assert!(mesh.triangle_count() > 0);
    }

    #[test]
    fn test_tesselate_extruded_solid() {
        let tesselator = Tesselator::new();

        // Perfil quadrado
        let profile = vec![
            Vec2::new(-1.0, -1.0),
            Vec2::new(1.0, -1.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(-1.0, 1.0),
        ];

        let geometry = IfcGeometry::ExtrudedAreaSolid {
            profile,
            extrusion_direction: Vec3::Y,
            depth: 3.0,
        };

        let mesh = tesselator.tesselate(&geometry).unwrap();
        assert!(mesh.validate().is_ok());
        assert!(mesh.triangle_count() > 0);
    }
}
