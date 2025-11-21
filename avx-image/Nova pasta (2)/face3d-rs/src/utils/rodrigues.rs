use nalgebra as na;

/// Converte axis-angle para matriz de rotação (Fórmula de Rodrigues)
///
/// # Argumentos
/// * `axis_angle` - Vetor 3D representando eixo × ângulo (magnitude = ângulo em radianos)
///
/// # Retorna
/// Matriz de rotação 3×3
///
/// # Fórmula
/// R = I + sin(θ)K + (1 - cos(θ))K²
/// onde K é a matriz skew-symmetric do eixo normalizado
pub fn axis_angle_to_matrix(axis_angle: &na::DVectorView<f32>) -> na::Matrix3<f32> {
    let angle = axis_angle.norm();

    // Se ângulo muito pequeno, retorna identidade
    if angle < 1e-8 {
        return na::Matrix3::identity();
    }

    // Normalizar eixo
    let axis = axis_angle / angle;

    // Matriz skew-symmetric (cross-product matrix)
    let k = skew_symmetric(axis[0], axis[1], axis[2]);

    // Rodrigues: R = I + sin(θ)K + (1-cos(θ))K²
    na::Matrix3::identity()
        + k * angle.sin()
        + k * k * (1.0 - angle.cos())
}

/// Versão que aceita DVector diretamente
pub fn axis_angle_to_matrix_from_dvector(axis_angle: &na::DVector<f32>) -> na::Matrix3<f32> {
    axis_angle_to_matrix(&axis_angle.as_view())
}

/// Cria matriz skew-symmetric (anti-simétrica) de um vetor 3D
///
/// Para v = [x, y, z], retorna:
/// ```text
/// [ 0  -z   y]
/// [ z   0  -x]
/// [-y   x   0]
/// ```
pub fn skew_symmetric(x: f32, y: f32, z: f32) -> na::Matrix3<f32> {
    na::Matrix3::new(
        0.0, -z,   y,
        z,   0.0, -x,
        -y,  x,   0.0,
    )
}

/// Converte matriz de rotação para axis-angle
pub fn matrix_to_axis_angle(rot: &na::Matrix3<f32>) -> na::Vector3<f32> {
    // Extrair ângulo do traço da matriz
    let trace = rot.trace();
    let angle = ((trace - 1.0) / 2.0).clamp(-1.0, 1.0).acos();

    if angle.abs() < 1e-6 {
        // Rotação muito pequena
        return na::Vector3::zeros();
    }

    if (angle - std::f32::consts::PI).abs() < 1e-6 {
        // Rotação de 180 graus - caso especial
        // Extrair eixo da diagonal
        let i = rot.diagonal().imax();
        let mut axis = na::Vector3::zeros();
        axis[i] = ((rot[(i, i)] + 1.0) / 2.0).sqrt();

        for j in 0..3 {
            if j != i {
                axis[j] = rot[(i, j)] / (2.0 * axis[i]);
            }
        }

        return axis * angle;
    }

    // Caso geral
    let axis = na::Vector3::new(
        rot[(2, 1)] - rot[(1, 2)],
        rot[(0, 2)] - rot[(2, 0)],
        rot[(1, 0)] - rot[(0, 1)],
    );

    let axis = axis.normalize();
    axis * angle
}

/// Converte quaternion para matriz de rotação
pub fn quaternion_to_matrix(q: &na::UnitQuaternion<f32>) -> na::Matrix3<f32> {
    q.to_rotation_matrix().into_inner()
}

/// Converte axis-angle para quaternion
pub fn axis_angle_to_quaternion(axis_angle: &na::Vector3<f32>) -> na::UnitQuaternion<f32> {
    let angle = axis_angle.norm();

    if angle < 1e-8 {
        return na::UnitQuaternion::identity();
    }

    let axis = na::Unit::new_normalize(axis_angle / angle);
    na::UnitQuaternion::from_axis_angle(&axis, angle)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_identity() {
        let zero = na::DVector::from_vec(vec![0.0, 0.0, 0.0]);
        let rot = axis_angle_to_matrix_from_dvector(&zero);

        assert_relative_eq!(rot, na::Matrix3::identity(), epsilon = 1e-6);
    }

    #[test]
    fn test_rotation_x() {
        let angle = std::f32::consts::FRAC_PI_2; // 90 graus
        let axis_angle = na::DVector::from_vec(vec![angle, 0.0, 0.0]);
        let rot = axis_angle_to_matrix_from_dvector(&axis_angle);

        // Rotação de 90° em X deve transformar (0,1,0) em (0,0,1)
        let v = na::Vector3::new(0.0, 1.0, 0.0);
        let v_rot = rot * v;

        assert_relative_eq!(v_rot.x, 0.0, epsilon = 1e-6);
        assert_relative_eq!(v_rot.y, 0.0, epsilon = 1e-6);
        assert_relative_eq!(v_rot.z, 1.0, epsilon = 1e-6);
    }

    #[test]
    fn test_rotation_y() {
        let angle = std::f32::consts::FRAC_PI_2;
        let axis_angle = na::DVector::from_vec(vec![0.0, angle, 0.0]);
        let rot = axis_angle_to_matrix_from_dvector(&axis_angle);

        // Rotação de 90° em Y deve transformar (1,0,0) em (0,0,-1)
        let v = na::Vector3::new(1.0, 0.0, 0.0);
        let v_rot = rot * v;

        assert_relative_eq!(v_rot.x, 0.0, epsilon = 1e-6);
        assert_relative_eq!(v_rot.y, 0.0, epsilon = 1e-6);
        assert_relative_eq!(v_rot.z, -1.0, epsilon = 1e-6);
    }

    #[test]
    fn test_skew_symmetric() {
        let k = skew_symmetric(1.0, 2.0, 3.0);

        // Deve ser anti-simétrica: K^T = -K
        let kt = k.transpose();
        assert_relative_eq!(kt, -k, epsilon = 1e-6);
    }

    #[test]
    fn test_roundtrip() {
        let axis_angle = na::Vector3::new(0.5, 0.3, -0.2);
        let rot = axis_angle_to_matrix_from_dvector(&na::DVector::from_vec(axis_angle.as_slice().to_vec()));
        let recovered = matrix_to_axis_angle(&rot);

        assert_relative_eq!(recovered, axis_angle, epsilon = 1e-5);
    }
}
