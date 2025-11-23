//! Módulo de Óptica - Formação de Imagem
//!
//! Implementa os princípios físicos de captura de imagem:
//! - Projeção perspectiva (pinhole camera model)
//! - Equação de reflectância (Lambertian + Phong)
//! - Modelo de iluminação

use nalgebra::{Point2, Point3, Vector3};

/// Modelo de câmera pinhole
pub struct Camera {
    /// Largura da imagem em pixels
    width: u32,
    /// Altura da imagem em pixels
    height: u32,
    /// Distância focal em mm
    focal_length: f32,
    /// Tamanho do sensor em mm
    sensor_size: f32,
}

impl Camera {
    pub fn new(width: u32, height: u32, focal_length: f32, sensor_size: f32) -> Self {
        Self { width, height, focal_length, sensor_size }
    }

    /// Projeta ponto 3D no plano da imagem (projeção perspectiva)
    ///
    /// Matemática:
    /// x' = f × (x / z)
    /// y' = f × (y / z)
    ///
    /// onde f = distância focal, z = profundidade
    pub fn project_point(&self, point: &Point3<f32>) -> Point2<f32> {
        // Evita divisão por zero
        let z = point.z.max(1.0);

        // Projeção perspectiva
        let x_ndc = (self.focal_length * point.x) / z;
        let y_ndc = (self.focal_length * point.y) / z;

        // Converte NDC para coordenadas de pixel
        let pixel_x = (x_ndc / self.sensor_size + 0.5) * self.width as f32;
        let pixel_y = (y_ndc / self.sensor_size + 0.5) * self.height as f32;

        Point2::new(pixel_x, pixel_y)
    }

    /// Calcula profundidade de campo (depth of field)
    pub fn depth_of_field(&self, aperture: f32, focus_distance: f32) -> (f32, f32) {
        let coc = 0.03; // Circle of confusion (mm)

        // Hiperfocal distance
        let h = (self.focal_length.powi(2)) / (aperture * coc);

        // Near and far limits
        let near = (focus_distance * h) / (h + focus_distance);
        let far = (focus_distance * h) / (h - focus_distance);

        (near, far)
    }
}

/// Fonte de luz
#[derive(Debug, Clone, Copy)]
pub struct Light {
    /// Direção da luz (vetor unitário)
    pub direction: Vector3<f32>,
    /// Intensidade (W/m²)
    pub intensity: f32,
    /// Cor da luz (RGB)
    pub color: Vector3<f32>,
}

/// Ponto na superfície facial
pub struct FacePoint {
    /// Posição 3D no espaço
    pub position: Point3<f32>,
    /// Normal da superfície
    pub normal: Vector3<f32>,
    /// Albedo (coeficiente de reflexão difusa)
    pub albedo: f32,
    /// Cor base da pele
    pub color: Vector3<f32>,
}

/// Calcula irradiância em um ponto (Lei de Lambert)
///
/// E = I × cos(θ) = I × (n · l)
///
/// onde:
/// - E = irradiância
/// - I = intensidade da luz
/// - θ = ângulo entre normal e luz
/// - n = vetor normal
/// - l = vetor de luz
pub fn calculate_irradiance(point: &FacePoint, light: &Light) -> f32 {
    let cos_theta = point.normal.dot(&light.direction).max(0.0);
    light.intensity * cos_theta
}

/// Calcula cor do pixel usando modelo de reflectância Lambertiana
///
/// I(x,y) = ρ × (n · l) × Eₗᵢgₕₜ × Cₛᵤᵣfₐcₑ
pub fn calculate_pixel_color(point: &FacePoint, lights: &[Light]) -> Vector3<f32> {
    let mut color = Vector3::zeros();

    for light in lights {
        let irradiance = calculate_irradiance(point, light);

        // Componente difusa (Lambertian)
        let diffuse = point.albedo * irradiance;

        // Multiplica pela cor da superfície e da luz
        color += diffuse * point.color.component_mul(&light.color);
    }

    // Clamp valores para [0, 1]
    color.map(|v| v.clamp(0.0, 1.0))
}

/// Modelo de reflectância Phong (inclui componente especular)
///
/// I = Iₐ × kₐ + Iₗ × (kd × (n·l) + kₛ × (r·v)ⁿ)
///
/// onde:
/// - Iₐ = luz ambiente
/// - kₐ = coeficiente ambiente
/// - kd = coeficiente difuso
/// - kₛ = coeficiente especular
/// - n = expoente de Phong
/// - r = vetor refletido
/// - v = vetor de visão
pub fn calculate_phong_reflection(
    point: &FacePoint,
    light: &Light,
    view_dir: &Vector3<f32>,
    specular_coef: f32,
    shininess: f32,
) -> Vector3<f32> {
    // Componente difusa
    let diffuse = calculate_pixel_color(point, &[light.clone()]);

    // Componente especular
    let light_dir = -light.direction; // Direção para a luz
    let reflect_dir = reflect(&light_dir, &point.normal);
    let spec = reflect_dir.dot(view_dir).max(0.0).powf(shininess);
    let specular = specular_coef * spec * light.color;

    diffuse + specular
}

/// Calcula vetor refletido (lei da reflexão)
///
/// r = d - 2(d·n)n
///
/// onde d = vetor incidente, n = normal
fn reflect(incident: &Vector3<f32>, normal: &Vector3<f32>) -> Vector3<f32> {
    incident - 2.0 * incident.dot(normal) * normal
}

/// Simula difração por abertura circular (padrão de Airy)
///
/// I(θ) = I₀ × [2J₁(x)/x]²
///
/// onde J₁ = função de Bessel de primeira ordem
/// x = (2πa/λ) × sin(θ)
pub fn airy_pattern(radius: f32, wavelength: f32, angle: f32) -> f32 {
    let x = (2.0 * std::f32::consts::PI * radius / wavelength) * angle.sin();

    if x.abs() < 1e-6 {
        return 1.0;
    }

    // Aproximação de J₁(x)/x
    let bessel = bessel_j1(x) / x;
    bessel.powi(2)
}

/// Função de Bessel de primeira ordem (aproximação)
fn bessel_j1(x: f32) -> f32 {
    if x.abs() < 3.0 {
        // Série de Taylor
        let x2 = x * x;
        x * (0.5 - x2 / 16.0 * (1.0 - x2 / 24.0))
    } else {
        // Aproximação assintótica
        let phase = x - 3.0 * std::f32::consts::PI / 4.0;
        (2.0 / (std::f32::consts::PI * x)).sqrt() * phase.cos()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camera_projection() {
        let camera = Camera::new(1920, 1080, 50.0, 35.0);
        let point = Point3::new(0.0, 0.0, 500.0);
        let pixel = camera.project_point(&point);

        // Centro da imagem
        assert!((pixel.x - 960.0).abs() < 10.0);
        assert!((pixel.y - 540.0).abs() < 10.0);
    }

    #[test]
    fn test_lambertian_reflection() {
        let point = FacePoint {
            position: Point3::origin(),
            normal: Vector3::new(0.0, 0.0, 1.0),
            albedo: 0.7,
            color: Vector3::new(1.0, 1.0, 1.0),
        };

        let light = Light {
            direction: Vector3::new(0.0, 0.0, 1.0),
            intensity: 1.0,
            color: Vector3::new(1.0, 1.0, 1.0),
        };

        let irradiance = calculate_irradiance(&point, &light);
        assert!((irradiance - 1.0).abs() < 1e-5);
    }
}
