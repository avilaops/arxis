use nalgebra as na;
use crate::error::{Face3dError, Result};

/// Modelo 3D Morphable básico (3DMM)
///
/// Representa um modelo paramétrico linear para geração de faces 3D
/// baseado em análise de componentes principais (PCA).
///
/// # Estrutura
/// - Shape: S = S_mean + Σ(α_i × B_shape_i)
/// - Texture: T = T_mean + Σ(β_i × B_texture_i)
#[derive(Clone, Debug)]
pub struct MorphableModel {
    /// Vértices médios (mean shape): 3N valores [x1,y1,z1, x2,y2,z2, ...]
    pub mean_shape: na::DVector<f32>,

    /// Base de formas (shape basis): PCA dos vértices
    /// Matriz 3N × n_shape_components
    pub shape_basis: na::DMatrix<f32>,

    /// Base de texturas (texture basis): PCA das cores
    /// Matriz 3N × n_texture_components
    pub texture_basis: na::DMatrix<f32>,

    /// Textura média (mean texture): 3N valores [r1,g1,b1, r2,g2,b2, ...]
    pub mean_texture: na::DVector<f32>,

    /// Topologia: faces triangulares (índices dos vértices)
    pub faces: Vec<[usize; 3]>,

    /// Número de vértices
    pub n_vertices: usize,
}

impl MorphableModel {
    /// Cria um novo modelo 3DMM
    pub fn new(
        mean_shape: na::DVector<f32>,
        shape_basis: na::DMatrix<f32>,
        texture_basis: na::DMatrix<f32>,
        mean_texture: na::DVector<f32>,
        faces: Vec<[usize; 3]>,
    ) -> Self {
        let n_vertices = mean_shape.len() / 3;
        Self {
            mean_shape,
            shape_basis,
            texture_basis,
            mean_texture,
            faces,
            n_vertices,
        }
    }

    /// Retorna o número de componentes de shape
    pub fn n_shape_components(&self) -> usize {
        self.shape_basis.ncols()
    }

    /// Retorna o número de componentes de texture
    pub fn n_texture_components(&self) -> usize {
        self.texture_basis.ncols()
    }

    /// Gera uma face 3D a partir de parâmetros
    ///
    /// # Argumentos
    /// * `shape_params` - Coeficientes α para shape (n_shape_components)
    /// * `texture_params` - Coeficientes β para texture (n_texture_components)
    ///
    /// # Retorna
    /// Tupla (shape, texture) com os vértices e cores gerados
    pub fn generate_face(
        &self,
        shape_params: &na::DVector<f32>,
        texture_params: &na::DVector<f32>,
    ) -> Result<(na::DVector<f32>, na::DVector<f32>)> {
        // Validar dimensões
        if shape_params.len() != self.n_shape_components() {
            return Err(Face3dError::InvalidShapeParams {
                expected: self.n_shape_components(),
                got: shape_params.len(),
            });
        }

        if texture_params.len() != self.n_texture_components() {
            return Err(Face3dError::InvalidShapeParams {
                expected: self.n_texture_components(),
                got: texture_params.len(),
            });
        }

        // Shape: S = S_mean + Σ(α_i × B_shape_i)
        let shape = &self.mean_shape + &self.shape_basis * shape_params;

        // Texture: T = T_mean + Σ(β_i × B_texture_i)
        let texture = &self.mean_texture + &self.texture_basis * texture_params;

        Ok((shape, texture))
    }

    /// Extrai vértice específico das coordenadas
    pub fn get_vertex(&self, shape: &na::DVector<f32>, idx: usize) -> Result<na::Point3<f32>> {
        if idx >= self.n_vertices {
            return Err(Face3dError::InvalidVertexIndex(idx));
        }

        let i = idx * 3;
        Ok(na::Point3::new(shape[i], shape[i + 1], shape[i + 2]))
    }

    /// Converte shape para lista de pontos 3D
    pub fn shape_to_vertices(&self, shape: &na::DVector<f32>) -> Vec<na::Point3<f32>> {
        (0..self.n_vertices)
            .map(|i| {
                let idx = i * 3;
                na::Point3::new(shape[idx], shape[idx + 1], shape[idx + 2])
            })
            .collect()
    }

    /// Converte texture para lista de cores RGB
    pub fn texture_to_colors(&self, texture: &na::DVector<f32>) -> Vec<na::Vector3<f32>> {
        (0..self.n_vertices)
            .map(|i| {
                let idx = i * 3;
                na::Vector3::new(texture[idx], texture[idx + 1], texture[idx + 2])
            })
            .collect()
    }

    /// Exporta mesh para formato OBJ (requer feature "mesh-export")
    #[cfg(feature = "mesh-export")]
    pub fn export_obj(&self, shape: &na::DVector<f32>, path: &str) -> Result<()> {
        use std::fs::File;
        use std::io::Write;

        let mut file = File::create(path)?;

        // Escrever vértices
        for i in 0..self.n_vertices {
            let v = self.get_vertex(shape, i)?;
            writeln!(file, "v {} {} {}", v.x, v.y, v.z)?;
        }

        // Escrever faces (OBJ usa índices 1-based)
        for face in &self.faces {
            writeln!(file, "f {} {} {}", face[0] + 1, face[1] + 1, face[2] + 1)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_create_model() {
        let model = MorphableModel::new(
            na::DVector::zeros(300),
            na::DMatrix::zeros(300, 199),
            na::DMatrix::zeros(300, 199),
            na::DVector::zeros(300),
            vec![],
        );

        assert_eq!(model.n_vertices, 100);
        assert_eq!(model.n_shape_components(), 199);
        assert_eq!(model.n_texture_components(), 199);
    }

    #[test]
    fn test_generate_face() {
        let model = MorphableModel::new(
            na::DVector::from_element(300, 1.0),
            na::DMatrix::from_element(300, 10, 0.1),
            na::DMatrix::from_element(300, 10, 0.1),
            na::DVector::from_element(300, 0.5),
            vec![],
        );

        let shape_params = na::DVector::from_element(10, 0.5);
        let texture_params = na::DVector::from_element(10, 0.5);

        let result = model.generate_face(&shape_params, &texture_params);
        assert!(result.is_ok());

        let (shape, texture) = result.unwrap();
        assert_eq!(shape.len(), 300);
        assert_eq!(texture.len(), 300);
    }

    #[test]
    fn test_get_vertex() {
        let shape = na::DVector::from_vec(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let model = MorphableModel::new(
            shape.clone(),
            na::DMatrix::zeros(6, 5),
            na::DMatrix::zeros(6, 5),
            na::DVector::zeros(6),
            vec![],
        );

        let v0 = model.get_vertex(&shape, 0).unwrap();
        assert_relative_eq!(v0.x, 1.0);
        assert_relative_eq!(v0.y, 2.0);
        assert_relative_eq!(v0.z, 3.0);

        let v1 = model.get_vertex(&shape, 1).unwrap();
        assert_relative_eq!(v1.x, 4.0);
        assert_relative_eq!(v1.y, 5.0);
        assert_relative_eq!(v1.z, 6.0);
    }

    #[test]
    fn test_invalid_params() {
        let model = MorphableModel::new(
            na::DVector::zeros(300),
            na::DMatrix::zeros(300, 10),
            na::DMatrix::zeros(300, 10),
            na::DVector::zeros(300),
            vec![],
        );

        let wrong_shape = na::DVector::zeros(5);
        let correct_texture = na::DVector::zeros(10);

        let result = model.generate_face(&wrong_shape, &correct_texture);
        assert!(result.is_err());
    }
}
