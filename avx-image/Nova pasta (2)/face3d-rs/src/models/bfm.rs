use nalgebra as na;
use std::collections::HashMap;
use crate::error::{Face3dError, Result};

/// Resultado com geometria e cores do BFM
pub type BfmOutput = (Vec<na::Point3<f32>>, Vec<na::Vector3<f32>>);

/// Basel Face Model (BFM) - Dataset Escanado
///
/// Modelo de alta qualidade baseado em escaneamentos 3D reais.
/// Suporta:
/// - Shape model (geometria da face)
/// - Color model (textura/cor)
/// - Expression model (expressões faciais)
///
/// BFM 2017: ~53k vértices, ~105k triângulos
#[derive(Clone, Debug)]
pub struct BaselFaceModel {
    /// Shape model - mean (μ)
    pub shape_mu: na::DVector<f32>,

    /// Shape model - eigenvalues (λ)
    pub shape_ev: na::DVector<f32>,

    /// Shape model - principal components (PCA basis)
    pub shape_pc: na::DMatrix<f32>,

    /// Color model - mean (μ)
    pub color_mu: na::DVector<f32>,

    /// Color model - eigenvalues (λ)
    pub color_ev: na::DVector<f32>,

    /// Color model - principal components
    pub color_pc: na::DMatrix<f32>,

    /// Expression model - mean (μ)
    pub expression_mu: na::DVector<f32>,

    /// Expression model - eigenvalues (λ)
    pub expression_ev: na::DVector<f32>,

    /// Expression model - principal components
    pub expression_pc: na::DMatrix<f32>,

    /// Topologia: faces triangulares
    pub faces: Vec<[usize; 3]>,

    /// Landmarks (pontos de interesse faciais)
    pub landmarks_indices: HashMap<String, usize>,

    /// Número de vértices
    pub n_vertices: usize,
}

impl BaselFaceModel {
    /// Cria um novo modelo BFM
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        shape_mu: na::DVector<f32>,
        shape_ev: na::DVector<f32>,
        shape_pc: na::DMatrix<f32>,
        color_mu: na::DVector<f32>,
        color_ev: na::DVector<f32>,
        color_pc: na::DMatrix<f32>,
        expression_mu: na::DVector<f32>,
        expression_ev: na::DVector<f32>,
        expression_pc: na::DMatrix<f32>,
        faces: Vec<[usize; 3]>,
        landmarks_indices: HashMap<String, usize>,
    ) -> Self {
        let n_vertices = shape_mu.len() / 3;
        Self {
            shape_mu,
            shape_ev,
            shape_pc,
            color_mu,
            color_ev,
            color_pc,
            expression_mu,
            expression_ev,
            expression_pc,
            faces,
            landmarks_indices,
            n_vertices,
        }
    }

    /// Retorna o número de componentes de shape
    pub fn n_shape_components(&self) -> usize {
        self.shape_pc.ncols()
    }

    /// Retorna o número de componentes de color
    pub fn n_color_components(&self) -> usize {
        self.color_pc.ncols()
    }

    /// Retorna o número de componentes de expression
    pub fn n_expression_components(&self) -> usize {
        self.expression_pc.ncols()
    }

    /// Gera uma instância do modelo BFM
    ///
    /// # Argumentos
    /// * `shape_coeffs` - Coeficientes para shape (identidade)
    /// * `color_coeffs` - Coeficientes para cor/textura
    /// * `expression_coeffs` - Coeficientes para expressão
    ///
    /// # Retorna
    /// Tupla (vertices, colors) com geometria e cores geradas
    ///
    /// # Fórmula PCA
    /// X = μ + Σ(α_i × √λ_i × PC_i)
    pub fn generate(
        &self,
        shape_coeffs: &[f32],
        color_coeffs: &[f32],
        expression_coeffs: &[f32],
    ) -> Result<BfmOutput> {
        // Validar dimensões
        if shape_coeffs.len() > self.n_shape_components() {
            return Err(Face3dError::InvalidShapeParams {
                expected: self.n_shape_components(),
                got: shape_coeffs.len(),
            });
        }

        if color_coeffs.len() > self.n_color_components() {
            return Err(Face3dError::InvalidShapeParams {
                expected: self.n_color_components(),
                got: color_coeffs.len(),
            });
        }

        if expression_coeffs.len() > self.n_expression_components() {
            return Err(Face3dError::InvalidExpressionParams {
                expected: self.n_expression_components(),
                got: expression_coeffs.len(),
            });
        }

        // Shape: S = μ_s + Σ(α_i × √λ_i × PC_i)
        let mut shape = self.shape_mu.clone();
        for (i, &coeff) in shape_coeffs.iter().enumerate() {
            let pc = self.shape_pc.column(i);
            let ev_sqrt = self.shape_ev[i].sqrt();
            shape += pc * (coeff * ev_sqrt);
        }

        // Expression: adicionar variação de expressão
        for (i, &coeff) in expression_coeffs.iter().enumerate() {
            let pc = self.expression_pc.column(i);
            let ev_sqrt = self.expression_ev[i].sqrt();
            shape += pc * (coeff * ev_sqrt);
        }

        // Color: C = μ_c + Σ(β_i × √λ_i × PC_i)
        let mut color = self.color_mu.clone();
        for (i, &coeff) in color_coeffs.iter().enumerate() {
            let pc = self.color_pc.column(i);
            let ev_sqrt = self.color_ev[i].sqrt();
            color += pc * (coeff * ev_sqrt);
        }

        // Converter para Points e Colors
        let vertices: Vec<na::Point3<f32>> = (0..self.n_vertices)
            .map(|i| {
                let idx = i * 3;
                na::Point3::new(shape[idx], shape[idx + 1], shape[idx + 2])
            })
            .collect();

        let colors: Vec<na::Vector3<f32>> = (0..self.n_vertices)
            .map(|i| {
                let idx = i * 3;
                // Normalizar cor para [0, 1] se necessário
                let r = color[idx].clamp(0.0, 1.0);
                let g = color[idx + 1].clamp(0.0, 1.0);
                let b = color[idx + 2].clamp(0.0, 1.0);
                na::Vector3::new(r, g, b)
            })
            .collect();

        Ok((vertices, colors))
    }

    /// Retorna índice de um landmark específico
    pub fn get_landmark_index(&self, name: &str) -> Option<usize> {
        self.landmarks_indices.get(name).copied()
    }

    /// Retorna posição 3D de um landmark
    pub fn get_landmark_position(
        &self,
        vertices: &[na::Point3<f32>],
        name: &str,
    ) -> Option<na::Point3<f32>> {
        self.get_landmark_index(name)
            .and_then(|idx| vertices.get(idx).copied())
    }

    /// Adiciona um landmark
    pub fn add_landmark(&mut self, name: String, index: usize) -> Result<()> {
        if index >= self.n_vertices {
            return Err(Face3dError::InvalidVertexIndex(index));
        }
        self.landmarks_indices.insert(name, index);
        Ok(())
    }

    /// Exporta mesh para formato OBJ com cores (requer feature "mesh-export")
    #[cfg(feature = "mesh-export")]
    pub fn export_obj_with_colors(
        &self,
        vertices: &[na::Point3<f32>],
        colors: &[na::Vector3<f32>],
        path: &str,
    ) -> Result<()> {
        use std::fs::File;
        use std::io::Write;

        let mut file = File::create(path)?;

        // Escrever vértices com cores
        for (v, c) in vertices.iter().zip(colors.iter()) {
            writeln!(file, "v {} {} {} {} {} {}",
                v.x, v.y, v.z, c.x, c.y, c.z
            )?;
        }

        // Escrever faces (OBJ usa índices 1-based)
        for face in &self.faces {
            writeln!(file, "f {} {} {}", face[0] + 1, face[1] + 1, face[2] + 1)?;
        }

        Ok(())
    }

    /// Carrega BFM de arquivo HDF5 (requer feature "scientific-io")
    #[cfg(feature = "scientific-io")]
    pub fn load_from_hdf5(path: &str) -> Result<Self> {
        // TODO: Implementar carregamento real de arquivo BFM .h5
        // Estrutura típica:
        // /shape/model/mean
        // /shape/model/pcaBasis
        // /shape/model/pcaVariance
        // /color/model/mean
        // /color/model/pcaBasis
        // /color/model/pcaVariance
        // /expression/model/mean
        // /expression/model/pcaBasis
        // /expression/model/pcaVariance
        // /shape/representer/cells

        Err(Face3dError::Other(anyhow::anyhow!(
            "HDF5 loading not yet implemented. Path: {}", path
        )))
    }
}

/// Builder para facilitar criação de modelos BFM
pub struct BfmBuilder {
    n_vertices: usize,
    n_shape_components: usize,
    n_color_components: usize,
    n_expression_components: usize,
}

impl BfmBuilder {
    pub fn new() -> Self {
        Self {
            n_vertices: 53149,  // BFM 2017 default
            n_shape_components: 199,
            n_color_components: 199,
            n_expression_components: 100,
        }
    }

    pub fn n_vertices(mut self, n: usize) -> Self {
        self.n_vertices = n;
        self
    }

    pub fn n_shape_components(mut self, n: usize) -> Self {
        self.n_shape_components = n;
        self
    }

    pub fn n_color_components(mut self, n: usize) -> Self {
        self.n_color_components = n;
        self
    }

    pub fn n_expression_components(mut self, n: usize) -> Self {
        self.n_expression_components = n;
        self
    }

    /// Cria um modelo BFM vazio (para testes)
    pub fn build_empty(self) -> BaselFaceModel {
        let shape_dim = self.n_vertices * 3;

        BaselFaceModel::new(
            na::DVector::zeros(shape_dim),
            na::DVector::from_element(self.n_shape_components, 1.0),
            na::DMatrix::zeros(shape_dim, self.n_shape_components),
            na::DVector::from_element(shape_dim, 0.5), // gray default
            na::DVector::from_element(self.n_color_components, 1.0),
            na::DMatrix::zeros(shape_dim, self.n_color_components),
            na::DVector::zeros(shape_dim),
            na::DVector::from_element(self.n_expression_components, 1.0),
            na::DMatrix::zeros(shape_dim, self.n_expression_components),
            vec![],
            HashMap::new(),
        )
    }
}

impl Default for BfmBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_bfm_model() {
        let model = BfmBuilder::new()
            .n_vertices(100)
            .n_shape_components(10)
            .n_color_components(10)
            .n_expression_components(5)
            .build_empty();

        assert_eq!(model.n_vertices, 100);
        assert_eq!(model.n_shape_components(), 10);
        assert_eq!(model.n_color_components(), 10);
        assert_eq!(model.n_expression_components(), 5);
    }

    #[test]
    fn test_bfm_generate() {
        let model = BfmBuilder::new()
            .n_vertices(10)
            .n_shape_components(5)
            .n_color_components(5)
            .n_expression_components(3)
            .build_empty();

        let shape_coeffs = vec![0.1, 0.2, 0.0, -0.1, 0.3];
        let color_coeffs = vec![0.0, 0.1, 0.2, 0.0, -0.1];
        let expr_coeffs = vec![0.5, -0.2, 0.1];

        let result = model.generate(&shape_coeffs, &color_coeffs, &expr_coeffs);
        assert!(result.is_ok());

        let (vertices, colors) = result.unwrap();
        assert_eq!(vertices.len(), 10);
        assert_eq!(colors.len(), 10);
    }

    #[test]
    fn test_landmarks() {
        let mut model = BfmBuilder::new()
            .n_vertices(100)
            .build_empty();

        assert!(model.add_landmark("nose_tip".to_string(), 50).is_ok());
        assert!(model.add_landmark("left_eye".to_string(), 25).is_ok());

        assert_eq!(model.get_landmark_index("nose_tip"), Some(50));
        assert_eq!(model.get_landmark_index("left_eye"), Some(25));
        assert_eq!(model.get_landmark_index("unknown"), None);

        // Testar índice inválido
        assert!(model.add_landmark("invalid".to_string(), 1000).is_err());
    }
}
