//! Módulo de Geometria - Análise de Superfície 3D
//! 
//! Implementa conceitos de geometria diferencial aplicados a faces:
//! - Cálculo de normais de superfície
//! - Curvatura gaussiana e média
//! - Distâncias métricas
//! - Transformações geométricas

use nalgebra::{Point3, Vector3, Matrix3};

/// Calcula distância euclidiana entre dois pontos
/// 
/// d = ||p₁ - p₂|| = √[(x₁-x₂)² + (y₁-y₂)² + (z₁-z₂)²]
pub fn euclidean_distance(p1: &Point3<f32>, p2: &Point3<f32>) -> f32 {
    (p2 - p1).norm()
}

/// Calcula normal da superfície usando produto vetorial
/// 
/// n = (p₁ - p₀) × (p₂ - p₀)
/// n̂ = n / ||n||
/// 
/// A normal aponta na direção perpendicular ao plano
pub fn compute_surface_normal(
    center: &Point3<f32>,
    neighbor1: &Point3<f32>,
    neighbor2: &Point3<f32>,
) -> Vector3<f32> {
    let v1 = neighbor1 - center;
    let v2 = neighbor2 - center;
    
    // Produto vetorial
    let normal = v1.cross(&v2);
    
    // Normaliza
    normal.normalize()
}

/// Estima curvatura gaussiana em um ponto
/// 
/// K = κ₁ × κ₂
/// 
/// onde κ₁ e κ₂ são as curvaturas principais
/// 
/// Aproximação usando círculo osculador:
/// κ ≈ 2 × sin(θ/2) / ||p₁ - p₂||
pub fn estimate_curvature(
    center: &Point3<f32>,
    neighbor1: &Point3<f32>,
    neighbor2: &Point3<f32>,
) -> f32 {
    let v1 = (neighbor1 - center).normalize();
    let v2 = (neighbor2 - center).normalize();
    
    // Ângulo entre vetores
    let cos_angle = v1.dot(&v2).clamp(-1.0, 1.0);
    let angle = cos_angle.acos();
    
    // Distância média
    let d1 = euclidean_distance(center, neighbor1);
    let d2 = euclidean_distance(center, neighbor2);
    let avg_dist = (d1 + d2) / 2.0;
    
    // Curvatura aproximada
    if avg_dist > 1e-6 {
        2.0 * (angle / 2.0).sin() / avg_dist
    } else {
        0.0
    }
}

/// Calcula curvatura média usando Laplaciano discreto
/// 
/// H = (1/2) Δx = (1/2n) Σ(xᵢ - x₀)
pub fn mean_curvature(center: &Point3<f32>, neighbors: &[Point3<f32>]) -> f32 {
    if neighbors.is_empty() {
        return 0.0;
    }
    
    let sum: Vector3<f32> = neighbors.iter()
        .map(|n| n - center)
        .sum();
    
    let laplacian = sum / neighbors.len() as f32;
    laplacian.norm() / 2.0
}

/// Estrutura para análise de landmarks faciais
pub struct FacialLandmarks {
    pub left_eye: Point3<f32>,
    pub right_eye: Point3<f32>,
    pub nose_tip: Point3<f32>,
    pub left_mouth: Point3<f32>,
    pub right_mouth: Point3<f32>,
    pub chin: Point3<f32>,
}

impl FacialLandmarks {
    /// Calcula proporções geométricas faciais
    pub fn compute_proportions(&self) -> FaceProportions {
        FaceProportions {
            eye_distance: euclidean_distance(&self.left_eye, &self.right_eye),
            nose_to_mouth: euclidean_distance(&self.nose_tip, 
                &Point3::new(
                    (self.left_mouth.x + self.right_mouth.x) / 2.0,
                    (self.left_mouth.y + self.right_mouth.y) / 2.0,
                    (self.left_mouth.z + self.right_mouth.z) / 2.0,
                )),
            face_width: euclidean_distance(&self.left_eye, &self.right_eye),
            face_length: euclidean_distance(
                &Point3::new(
                    (self.left_eye.x + self.right_eye.x) / 2.0,
                    (self.left_eye.y + self.right_eye.y) / 2.0,
                    (self.left_eye.z + self.right_eye.z) / 2.0,
                ),
                &self.chin
            ),
        }
    }
    
    /// Calcula plano médio da face
    pub fn compute_face_plane(&self) -> (Point3<f32>, Vector3<f32>) {
        // Centro (média dos pontos)
        let center = Point3::new(
            (self.left_eye.x + self.right_eye.x + self.nose_tip.x) / 3.0,
            (self.left_eye.y + self.right_eye.y + self.nose_tip.y) / 3.0,
            (self.left_eye.z + self.right_eye.z + self.nose_tip.z) / 3.0,
        );
        
        // Normal do plano
        let normal = compute_surface_normal(&center, &self.left_eye, &self.right_eye);
        
        (center, normal)
    }
}

pub struct FaceProportions {
    pub eye_distance: f32,
    pub nose_to_mouth: f32,
    pub face_width: f32,
    pub face_length: f32,
}

impl FaceProportions {
    /// Calcula índice facial (altura/largura)
    pub fn facial_index(&self) -> f32 {
        self.face_length / self.face_width
    }
}

/// Calcula transformação de Procrustes entre dois conjuntos de pontos
/// 
/// Alinha dois conjuntos de pontos minimizando:
/// Σ ||R×s×pᵢ + t - qᵢ||²
/// 
/// onde R = rotação, s = escala, t = translação
pub fn procrustes_alignment(
    source: &[Point3<f32>],
    target: &[Point3<f32>],
) -> (Matrix3<f32>, Vector3<f32>, f32) {
    assert_eq!(source.len(), target.len());
    
    let n = source.len() as f32;
    
    // Centroides
    let source_center = source.iter().fold(Vector3::zeros(), |acc, p| acc + p.coords) / n;
    let target_center = target.iter().fold(Vector3::zeros(), |acc, p| acc + p.coords) / n;
    
    // Centraliza pontos
    let source_centered: Vec<Vector3<f32>> = source.iter()
        .map(|p| p.coords - source_center)
        .collect();
    let target_centered: Vec<Vector3<f32>> = target.iter()
        .map(|p| p.coords - target_center)
        .collect();
    
    // Matriz de covariância
    let mut h = Matrix3::zeros();
    for i in 0..source.len() {
        h += source_centered[i] * target_centered[i].transpose();
    }
    
    // SVD para encontrar rotação ótima
    let svd = h.svd(true, true);
    let u = svd.u.unwrap();
    let v_t = svd.v_t.unwrap();
    let rotation = v_t.transpose() * u.transpose();
    
    // Escala
    let source_scale: f32 = source_centered.iter().map(|v| v.norm_squared()).sum::<f32>().sqrt();
    let target_scale: f32 = target_centered.iter().map(|v| v.norm_squared()).sum::<f32>().sqrt();
    let scale = target_scale / source_scale;
    
    // Translação
    let translation = target_center - scale * (rotation * source_center);
    
    (rotation, translation, scale)
}

/// Calcula área de superfície triangular (fórmula de Heron)
/// 
/// s = (a + b + c) / 2
/// A = √[s(s-a)(s-b)(s-c)]
pub fn triangle_area(p1: &Point3<f32>, p2: &Point3<f32>, p3: &Point3<f32>) -> f32 {
    let a = euclidean_distance(p1, p2);
    let b = euclidean_distance(p2, p3);
    let c = euclidean_distance(p3, p1);
    
    let s = (a + b + c) / 2.0;
    (s * (s - a) * (s - b) * (s - c)).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_euclidean_distance() {
        let p1 = Point3::new(0.0, 0.0, 0.0);
        let p2 = Point3::new(3.0, 4.0, 0.0);
        assert!((euclidean_distance(&p1, &p2) - 5.0).abs() < 1e-5);
    }
    
    #[test]
    fn test_surface_normal() {
        let center = Point3::new(0.0, 0.0, 0.0);
        let p1 = Point3::new(1.0, 0.0, 0.0);
        let p2 = Point3::new(0.0, 1.0, 0.0);
        
        let normal = compute_surface_normal(&center, &p1, &p2);
        
        // Normal deve apontar para +Z
        assert!((normal.z - 1.0).abs() < 1e-5);
        assert!(normal.x.abs() < 1e-5);
        assert!(normal.y.abs() < 1e-5);
    }
}
