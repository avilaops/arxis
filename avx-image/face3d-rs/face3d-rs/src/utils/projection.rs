use nalgebra as na;

/// Parâmetros de câmera para projeção perspectiva
#[derive(Clone, Debug)]
pub struct PerspectiveCamera {
    /// Distância focal em pixels (fx, fy)
    pub focal_length: na::Vector2<f32>,

    /// Centro principal em pixels (cx, cy)
    pub principal_point: na::Vector2<f32>,

    /// Largura da imagem
    pub image_width: u32,

    /// Altura da imagem
    pub image_height: u32,
}

impl PerspectiveCamera {
    /// Cria uma câmera perspectiva simples
    pub fn new(focal_length: f32, image_width: u32, image_height: u32) -> Self {
        Self {
            focal_length: na::Vector2::new(focal_length, focal_length),
            principal_point: na::Vector2::new(
                image_width as f32 / 2.0,
                image_height as f32 / 2.0,
            ),
            image_width,
            image_height,
        }
    }

    /// Cria câmera com parâmetros intrínsecos completos
    pub fn from_intrinsics(
        fx: f32,
        fy: f32,
        cx: f32,
        cy: f32,
        width: u32,
        height: u32,
    ) -> Self {
        Self {
            focal_length: na::Vector2::new(fx, fy),
            principal_point: na::Vector2::new(cx, cy),
            image_width: width,
            image_height: height,
        }
    }

    /// Retorna matriz intrínseca K (3×3)
    pub fn intrinsic_matrix(&self) -> na::Matrix3<f32> {
        na::Matrix3::new(
            self.focal_length.x, 0.0, self.principal_point.x,
            0.0, self.focal_length.y, self.principal_point.y,
            0.0, 0.0, 1.0,
        )
    }
}

/// Projeção perspectiva de pontos 3D para 2D
///
/// # Argumentos
/// * `points` - Pontos 3D no espaço da câmera
/// * `camera` - Parâmetros da câmera
///
/// # Retorna
/// Pontos 2D projetados na imagem (x, y em pixels)
///
/// # Fórmula
/// x' = fx * (X/Z) + cx
/// y' = fy * (Y/Z) + cy
pub fn perspective_projection(
    points: &[na::Point3<f32>],
    camera: &PerspectiveCamera,
) -> Vec<na::Point2<f32>> {
    points
        .iter()
        .map(|p| {
            let z = p.z.max(1e-6); // Evitar divisão por zero
            let x = camera.focal_length.x * (p.x / z) + camera.principal_point.x;
            let y = camera.focal_length.y * (p.y / z) + camera.principal_point.y;
            na::Point2::new(x, y)
        })
        .collect()
}

/// Parâmetros de câmera weak perspective (ortográfica com escala)
#[derive(Clone, Debug)]
pub struct WeakPerspectiveCamera {
    /// Escala (zoom)
    pub scale: f32,

    /// Translação 2D (tx, ty)
    pub translation: na::Vector2<f32>,

    /// Largura da imagem
    pub image_width: u32,

    /// Altura da imagem
    pub image_height: u32,
}

impl WeakPerspectiveCamera {
    pub fn new(scale: f32, tx: f32, ty: f32, width: u32, height: u32) -> Self {
        Self {
            scale,
            translation: na::Vector2::new(tx, ty),
            image_width: width,
            image_height: height,
        }
    }
}

/// Projeção weak perspective (simplificação da perspectiva)
///
/// Assume que a profundidade varia pouco comparada à distância média.
/// Útil para faces frontais distantes.
///
/// # Fórmula
/// x' = s * X + tx
/// y' = s * Y + ty
pub fn weak_perspective_projection(
    points: &[na::Point3<f32>],
    camera: &WeakPerspectiveCamera,
) -> Vec<na::Point2<f32>> {
    points
        .iter()
        .map(|p| {
            let x = camera.scale * p.x + camera.translation.x;
            let y = camera.scale * p.y + camera.translation.y;
            na::Point2::new(x, y)
        })
        .collect()
}

/// Projeção ortográfica simples (ignora Z)
pub fn orthographic_projection(points: &[na::Point3<f32>]) -> Vec<na::Point2<f32>> {
    points
        .iter()
        .map(|p| na::Point2::new(p.x, p.y))
        .collect()
}

/// Calcula matriz de projeção perspectiva OpenGL-style
pub fn perspective_projection_matrix(
    fov_y: f32,
    aspect_ratio: f32,
    near: f32,
    far: f32,
) -> na::Matrix4<f32> {
    na::Matrix4::new_perspective(aspect_ratio, fov_y, near, far)
}

/// Calcula matriz view (câmera lookAt)
pub fn look_at_matrix(
    eye: &na::Point3<f32>,
    target: &na::Point3<f32>,
    up: &na::Vector3<f32>,
) -> na::Matrix4<f32> {
    na::Matrix4::look_at_rh(eye, target, up)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_perspective_projection() {
        let camera = PerspectiveCamera::new(1000.0, 640, 480);

        // Ponto no centro, 1m de distância
        let points = vec![na::Point3::new(0.0, 0.0, 1.0)];
        let projected = perspective_projection(&points, &camera);

        // Deve projetar no centro da imagem
        assert_relative_eq!(projected[0].x, 320.0, epsilon = 1e-3);
        assert_relative_eq!(projected[0].y, 240.0, epsilon = 1e-3);
    }

    #[test]
    fn test_perspective_projection_offset() {
        let camera = PerspectiveCamera::new(1000.0, 640, 480);

        // Ponto deslocado 0.1m para direita, 1m de distância
        let points = vec![na::Point3::new(0.1, 0.0, 1.0)];
        let projected = perspective_projection(&points, &camera);

        // Deve projetar 100 pixels à direita do centro
        assert_relative_eq!(projected[0].x, 420.0, epsilon = 1e-3);
        assert_relative_eq!(projected[0].y, 240.0, epsilon = 1e-3);
    }

    #[test]
    fn test_weak_perspective() {
        let camera = WeakPerspectiveCamera::new(100.0, 320.0, 240.0, 640, 480);

        // Ponto na origem
        let points = vec![na::Point3::new(0.0, 0.0, 0.0)];
        let projected = weak_perspective_projection(&points, &camera);

        // Deve projetar na translação
        assert_relative_eq!(projected[0].x, 320.0, epsilon = 1e-6);
        assert_relative_eq!(projected[0].y, 240.0, epsilon = 1e-6);
    }

    #[test]
    fn test_weak_perspective_scaled() {
        let camera = WeakPerspectiveCamera::new(100.0, 0.0, 0.0, 640, 480);

        // Ponto em (1, 1)
        let points = vec![na::Point3::new(1.0, 1.0, 0.0)];
        let projected = weak_perspective_projection(&points, &camera);

        // Deve escalar por 100
        assert_relative_eq!(projected[0].x, 100.0, epsilon = 1e-6);
        assert_relative_eq!(projected[0].y, 100.0, epsilon = 1e-6);
    }

    #[test]
    fn test_intrinsic_matrix() {
        let camera = PerspectiveCamera::new(1000.0, 640, 480);
        let k = camera.intrinsic_matrix();

        assert_relative_eq!(k[(0, 0)], 1000.0);
        assert_relative_eq!(k[(1, 1)], 1000.0);
        assert_relative_eq!(k[(0, 2)], 320.0);
        assert_relative_eq!(k[(1, 2)], 240.0);
        assert_relative_eq!(k[(2, 2)], 1.0);
    }
}
