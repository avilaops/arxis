use nalgebra as na;
use crate::error::{Face3dError, Result};
use crate::utils::rodrigues::axis_angle_to_matrix;

/// FLAME: Faces Learned with an Articulated Model and Expressions
///
/// Modelo articulado de face 3D com skeleton e blend skinning.
/// Suporta:
/// - Shape parameters (identidade) - β (300d)
/// - Expression parameters (expressões faciais) - ψ (100d)
/// - Pose parameters (articulações) - θ (15 joints × 3 = 45d)
#[derive(Clone, Debug)]
pub struct FlameModel {
    /// Vértices do template base (5023 vertices × 3)
    pub template: na::DMatrix<f32>,

    /// PCA shape basis (identidade): 15069 × 300
    pub shape_basis: na::DMatrix<f32>,

    /// PCA expression basis (expressões faciais): 15069 × 100
    pub expression_basis: na::DMatrix<f32>,

    /// Pose blend shapes (correções para poses): 15069 × 36
    pub pose_basis: na::DMatrix<f32>,

    /// Skinning weights para articulações: 5023 × 5 (joints)
    pub lbs_weights: na::DMatrix<f32>,

    /// Hierarquia de juntas (skeleton)
    pub joint_hierarchy: Vec<Option<usize>>,

    /// Regressores para localização de juntas
    pub joint_regressor: na::DMatrix<f32>,

    /// Faces (topologia)
    pub faces: Vec<[usize; 3]>,

    /// Número de vértices
    pub n_vertices: usize,
}

impl FlameModel {
    /// Cria um novo modelo FLAME
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        template: na::DMatrix<f32>,
        shape_basis: na::DMatrix<f32>,
        expression_basis: na::DMatrix<f32>,
        pose_basis: na::DMatrix<f32>,
        lbs_weights: na::DMatrix<f32>,
        joint_hierarchy: Vec<Option<usize>>,
        joint_regressor: na::DMatrix<f32>,
        faces: Vec<[usize; 3]>,
    ) -> Self {
        let n_vertices = template.nrows();
        Self {
            template,
            shape_basis,
            expression_basis,
            pose_basis,
            lbs_weights,
            joint_hierarchy,
            joint_regressor,
            faces,
            n_vertices,
        }
    }

    /// Retorna o número de componentes de shape
    pub fn n_shape_components(&self) -> usize {
        self.shape_basis.ncols()
    }

    /// Retorna o número de componentes de expression
    pub fn n_expression_components(&self) -> usize {
        self.expression_basis.ncols()
    }

    /// Retorna o número de juntas
    pub fn n_joints(&self) -> usize {
        self.joint_hierarchy.len()
    }

    /// Gera mesh FLAME com todos os parâmetros
    ///
    /// # Argumentos
    /// * `shape_params` - β (300d) - parâmetros de identidade
    /// * `expression_params` - ψ (100d) - parâmetros de expressão
    /// * `pose_params` - θ (n_joints × 3) - parâmetros de pose em axis-angle
    ///
    /// # Retorna
    /// Matriz de vértices (n_vertices × 3) após aplicar todas as transformações
    pub fn forward(
        &self,
        shape_params: &na::DVector<f32>,
        expression_params: &na::DVector<f32>,
        pose_params: &na::DVector<f32>,
    ) -> Result<na::DMatrix<f32>> {
        // Validar dimensões
        if shape_params.len() != self.n_shape_components() {
            return Err(Face3dError::InvalidShapeParams {
                expected: self.n_shape_components(),
                got: shape_params.len(),
            });
        }

        if expression_params.len() != self.n_expression_components() {
            return Err(Face3dError::InvalidExpressionParams {
                expected: self.n_expression_components(),
                got: expression_params.len(),
            });
        }

        let expected_pose_params = self.n_joints() * 3;
        if pose_params.len() != expected_pose_params {
            return Err(Face3dError::InvalidPoseParams {
                expected: expected_pose_params,
                got: pose_params.len(),
            });
        }

        // 1. Template + shape + expression
        let shape_delta = self.compute_shape_delta(shape_params)?;
        let expr_delta = self.compute_expression_delta(expression_params)?;

        let mut vertices = self.template.clone();

        // Adicionar deltas ao template
        for i in 0..self.n_vertices {
            for j in 0..3 {
                let idx = i * 3 + j;
                vertices[(i, j)] += shape_delta[idx] + expr_delta[idx];
            }
        }

        // 2. Calcular posições das juntas
        let joints = &self.joint_regressor * &vertices;

        // 3. Linear Blend Skinning (LBS) com pose
        let posed_vertices = self.apply_lbs(&vertices, &joints, pose_params)?;

        Ok(posed_vertices)
    }

    /// Computa o delta de shape a partir dos parâmetros
    fn compute_shape_delta(&self, shape_params: &na::DVector<f32>) -> Result<na::DVector<f32>> {
        // Δ_shape = B_shape × β
        let delta = &self.shape_basis * shape_params;
        Ok(delta)
    }

    /// Computa o delta de expression a partir dos parâmetros
    fn compute_expression_delta(&self, expr_params: &na::DVector<f32>) -> Result<na::DVector<f32>> {
        // Δ_expr = B_expr × ψ
        let delta = &self.expression_basis * expr_params;
        Ok(delta)
    }

    /// Aplica Linear Blend Skinning (LBS)
    ///
    /// Transforma os vértices de acordo com as poses das juntas e os pesos de skinning
    fn apply_lbs(
        &self,
        vertices: &na::DMatrix<f32>,
        joints: &na::DMatrix<f32>,
        pose_params: &na::DVector<f32>,
    ) -> Result<na::DMatrix<f32>> {
        let n_joints = self.n_joints();

        // Criar transformações de rotação para cada junta (Rodrigues)
        let transforms: Vec<na::Matrix4<f32>> = (0..n_joints)
            .map(|j| {
                let axis_angle = pose_params.rows(j * 3, 3);
                let rot_mat = axis_angle_to_matrix(&axis_angle);

                // Matriz de transformação 4×4 (homogênea)
                let mut transform = na::Matrix4::identity();
                transform.fixed_view_mut::<3, 3>(0, 0).copy_from(&rot_mat);

                // Adicionar translação da junta
                if j < joints.nrows() {
                    transform[(0, 3)] = joints[(j, 0)];
                    transform[(1, 3)] = joints[(j, 1)];
                    transform[(2, 3)] = joints[(j, 2)];
                }

                transform
            })
            .collect();

        // Aplicar transformações com pesos de skinning
        let mut result = na::DMatrix::zeros(self.n_vertices, 3);

        for v_idx in 0..self.n_vertices {
            let v = vertices.row(v_idx);
            let v_homo = na::Vector4::new(v[0], v[1], v[2], 1.0);

            // Blend skinning: v' = Σ(w_j × T_j × v)
            let mut blended = na::Vector4::zeros();

            let max_joints = n_joints.min(self.lbs_weights.ncols());
            for (j, transform) in transforms.iter().enumerate().take(max_joints) {
                let weight = self.lbs_weights[(v_idx, j)];
                if weight > 1e-6 {
                    blended += weight * (transform * v_homo);
                }
            }

            result[(v_idx, 0)] = blended[0];
            result[(v_idx, 1)] = blended[1];
            result[(v_idx, 2)] = blended[2];
        }

        Ok(result)
    }

    /// Converte vértices para lista de pontos 3D
    pub fn vertices_to_points(&self, vertices: &na::DMatrix<f32>) -> Vec<na::Point3<f32>> {
        (0..vertices.nrows())
            .map(|i| {
                na::Point3::new(
                    vertices[(i, 0)],
                    vertices[(i, 1)],
                    vertices[(i, 2)],
                )
            })
            .collect()
    }

    /// Exporta mesh para formato OBJ (requer feature "mesh-export")
    #[cfg(feature = "mesh-export")]
    pub fn export_obj(&self, vertices: &na::DMatrix<f32>, path: &str) -> Result<()> {
        use std::fs::File;
        use std::io::Write;

        let mut file = File::create(path)?;

        // Escrever vértices
        for i in 0..vertices.nrows() {
            writeln!(file, "v {} {} {}",
                vertices[(i, 0)],
                vertices[(i, 1)],
                vertices[(i, 2)]
            )?;
        }

        // Escrever faces (OBJ usa índices 1-based)
        for face in &self.faces {
            writeln!(file, "f {} {} {}", face[0] + 1, face[1] + 1, face[2] + 1)?;
        }

        Ok(())
    }
}

/// Builder para facilitar criação de modelos FLAME simplificados
pub struct FlameBuilder {
    n_vertices: usize,
    n_shape_components: usize,
    n_expression_components: usize,
    n_joints: usize,
}

impl FlameBuilder {
    pub fn new() -> Self {
        Self {
            n_vertices: 5023,
            n_shape_components: 300,
            n_expression_components: 100,
            n_joints: 5,
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

    pub fn n_expression_components(mut self, n: usize) -> Self {
        self.n_expression_components = n;
        self
    }

    pub fn n_joints(mut self, n: usize) -> Self {
        self.n_joints = n;
        self
    }

    /// Cria um modelo FLAME vazio (para testes)
    pub fn build_empty(self) -> FlameModel {
        FlameModel::new(
            na::DMatrix::zeros(self.n_vertices, 3),
            na::DMatrix::zeros(self.n_vertices * 3, self.n_shape_components),
            na::DMatrix::zeros(self.n_vertices * 3, self.n_expression_components),
            na::DMatrix::zeros(self.n_vertices * 3, 36),
            na::DMatrix::zeros(self.n_vertices, self.n_joints),
            vec![None; self.n_joints],
            na::DMatrix::zeros(self.n_joints, self.n_vertices),
            vec![],
        )
    }
}

impl Default for FlameBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_flame_model() {
        let model = FlameBuilder::new()
            .n_vertices(100)
            .n_shape_components(10)
            .n_expression_components(5)
            .n_joints(3)
            .build_empty();

        assert_eq!(model.n_vertices, 100);
        assert_eq!(model.n_shape_components(), 10);
        assert_eq!(model.n_expression_components(), 5);
        assert_eq!(model.n_joints(), 3);
    }

    #[test]
    fn test_flame_forward() {
        let model = FlameBuilder::new()
            .n_vertices(10)
            .n_shape_components(5)
            .n_expression_components(3)
            .n_joints(2)
            .build_empty();

        let shape_params = na::DVector::zeros(5);
        let expr_params = na::DVector::zeros(3);
        let pose_params = na::DVector::zeros(6); // 2 joints × 3

        let result = model.forward(&shape_params, &expr_params, &pose_params);
        assert!(result.is_ok());

        let vertices = result.unwrap();
        assert_eq!(vertices.nrows(), 10);
        assert_eq!(vertices.ncols(), 3);
    }

    #[test]
    fn test_invalid_params() {
        let model = FlameBuilder::new()
            .n_vertices(10)
            .n_shape_components(5)
            .n_expression_components(3)
            .n_joints(2)
            .build_empty();

        // Wrong shape params
        let wrong_shape = na::DVector::zeros(3);
        let correct_expr = na::DVector::zeros(3);
        let correct_pose = na::DVector::zeros(6);

        let result = model.forward(&wrong_shape, &correct_expr, &correct_pose);
        assert!(result.is_err());
    }
}
