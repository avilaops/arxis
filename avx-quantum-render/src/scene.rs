//! Cena 3D para renderização quântica

use crate::photon::Vertex;

/// Material de superfície
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Material {
    /// Lambertiano (difuso ideal)
    Lambertian {
        /// Albedo (refletividade difusa) [0, 1]
        albedo: f64,
    },

    /// Especular (espelho)
    Specular {
        /// Refletância especular [0, 1]
        reflectance: f64,
    },

    /// Dielétrico (vidro, água)
    Dielectric {
        /// Índice de refração (n)
        refractive_index: f64,
        /// Transmitância [0, 1]
        transmittance: f64,
    },

    /// Metal (condutor)
    Metal {
        /// Refletância [0, 1]
        reflectance: f64,
        /// Rugosidade [0, 1]
        roughness: f64,
    },

    /// Absorvente (corpo negro)
    Absorbing {
        /// Coeficiente de absorção (m⁻¹)
        absorption_coeff: f64,
    },
}

impl Material {
    /// Cria material Lambertiano
    pub fn lambertian(albedo: f64) -> Self {
        Material::Lambertian {
            albedo: albedo.clamp(0.0, 1.0),
        }
    }

    /// Cria espelho perfeito
    pub fn mirror() -> Self {
        Material::Specular { reflectance: 1.0 }
    }

    /// Cria vidro (n ≈ 1.5)
    pub fn glass() -> Self {
        Material::Dielectric {
            refractive_index: 1.5,
            transmittance: 0.95,
        }
    }

    /// Cria metal
    pub fn metal(reflectance: f64) -> Self {
        Material::Metal {
            reflectance: reflectance.clamp(0.0, 1.0),
            roughness: 0.1,
        }
    }

    /// Retorna índice de refração (se aplicável)
    pub fn refractive_index(&self) -> f64 {
        match self {
            Material::Dielectric {
                refractive_index, ..
            } => *refractive_index,
            _ => 1.0,
        }
    }

    /// Retorna albedo/reflectância
    pub fn reflectance(&self) -> f64 {
        match self {
            Material::Lambertian { albedo } => *albedo,
            Material::Specular { reflectance } => *reflectance,
            Material::Metal { reflectance, .. } => *reflectance,
            Material::Dielectric { transmittance, .. } => 1.0 - transmittance,
            Material::Absorbing { .. } => 0.0,
        }
    }
}

/// Superfície na cena
#[derive(Debug, Clone)]
pub struct Surface {
    /// Posição da superfície
    pub position: [f64; 3],

    /// Normal da superfície
    pub normal: [f64; 3],

    /// Material
    pub material: Material,

    /// Área da superfície (para importance sampling)
    pub area: f64,
}

impl Surface {
    /// Cria nova superfície
    pub fn new(position: [f64; 3], normal: [f64; 3], material: Material) -> Self {
        Self {
            position,
            normal: Self::normalize(normal),
            material,
            area: 1.0,
        }
    }

    /// Cria superfície Lambertiana
    pub fn lambertian(position: [f64; 3], normal: [f64; 3], albedo: f64) -> Self {
        Self::new(position, normal, Material::lambertian(albedo))
    }

    /// Normaliza vetor
    fn normalize(v: [f64; 3]) -> [f64; 3] {
        let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
        if len > 1e-10 {
            [v[0] / len, v[1] / len, v[2] / len]
        } else {
            [0.0, 1.0, 0.0]
        }
    }

    /// Produto escalar com direção
    pub fn dot(&self, direction: [f64; 3]) -> f64 {
        self.normal[0] * direction[0]
            + self.normal[1] * direction[1]
            + self.normal[2] * direction[2]
    }

    /// Verifica se ponto está próximo da superfície
    pub fn is_near(&self, point: [f64; 3], tolerance: f64) -> bool {
        let dx = point[0] - self.position[0];
        let dy = point[1] - self.position[1];
        let dz = point[2] - self.position[2];
        let dist_sq = dx * dx + dy * dy + dz * dz;
        dist_sq < tolerance * tolerance
    }
}

/// Fonte de luz
#[derive(Debug, Clone)]
pub struct Light {
    /// Posição da luz
    pub position: [f64; 3],

    /// Intensidade (potência em Watts)
    pub intensity: f64,

    /// Cor (comprimento de onda em metros)
    pub wavelength: f64,

    /// Tipo de luz
    pub light_type: LightType,
}

/// Tipo de fonte de luz
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LightType {
    /// Luz pontual (onidirecional)
    Point,

    /// Luz direcional (sol)
    Directional,

    /// Luz spot (cone)
    Spot,

    /// Luz de área
    Area,
}

impl Light {
    /// Cria luz pontual
    pub fn point(position: [f64; 3], intensity: f64) -> Self {
        Self {
            position,
            intensity,
            wavelength: 550e-9, // Verde (550nm)
            light_type: LightType::Point,
        }
    }

    /// Cria luz direcional
    pub fn directional(direction: [f64; 3], intensity: f64) -> Self {
        Self {
            position: direction,
            intensity,
            wavelength: 550e-9,
            light_type: LightType::Directional,
        }
    }

    /// Energia do fóton (E = hc/λ)
    pub fn photon_energy(&self) -> f64 {
        use crate::{HBAR, SPEED_OF_LIGHT};
        2.0 * std::f64::consts::PI * HBAR * SPEED_OF_LIGHT / self.wavelength
    }

    /// Cria vértice de emissão a partir desta luz
    pub fn emit_vertex(&self, direction: [f64; 3]) -> Vertex {
        Vertex::emission(self.position, direction, self.photon_energy())
    }
}

/// Câmera/Sensor
#[derive(Debug, Clone)]
pub struct Camera {
    /// Posição da câmera
    pub position: [f64; 3],

    /// Direção de visão
    pub look_at: [f64; 3],

    /// Vetor "up"
    pub up: [f64; 3],

    /// Campo de visão (FOV em radianos)
    pub fov: f64,

    /// Resolução (largura, altura)
    pub resolution: (usize, usize),
}

impl Camera {
    /// Cria nova câmera
    pub fn new(position: [f64; 3], look_at: [f64; 3], fov: f64) -> Self {
        Self {
            position,
            look_at,
            up: [0.0, 1.0, 0.0],
            fov,
            resolution: (800, 600),
        }
    }

    /// Define resolução
    pub fn with_resolution(mut self, width: usize, height: usize) -> Self {
        self.resolution = (width, height);
        self
    }

    /// Gera raio para pixel (u, v) normalizado [0,1]
    pub fn generate_ray(&self, u: f64, v: f64) -> [f64; 3] {
        // Calcula direção do raio baseado em FOV
        let aspect = self.resolution.0 as f64 / self.resolution.1 as f64;
        let theta = self.fov * 0.5;
        let h = theta.tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect * viewport_height;

        // Coordenadas no plano de imagem
        let x = (2.0 * u - 1.0) * viewport_width * 0.5;
        let y = (2.0 * v - 1.0) * viewport_height * 0.5;

        // Direção normalizada
        let dir = [x, y, -1.0];
        Self::normalize(dir)
    }

    /// Cria vértice de detecção para pixel
    pub fn detect_vertex(&self, u: f64, v: f64) -> Vertex {
        let _direction = self.generate_ray(u, v);
        Vertex::detection(self.position, 0.0)
    }

    /// Normaliza vetor
    fn normalize(v: [f64; 3]) -> [f64; 3] {
        let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
        if len > 1e-10 {
            [v[0] / len, v[1] / len, v[2] / len]
        } else {
            [0.0, 0.0, -1.0]
        }
    }
}

/// Cena completa
#[derive(Debug, Clone)]
pub struct Scene {
    /// Luzes na cena
    pub lights: Vec<Light>,

    /// Superfícies na cena
    pub surfaces: Vec<Surface>,

    /// Câmera
    pub camera: Camera,

    /// Meio ambiente (índice de refração)
    pub ambient_refractive_index: f64,
}

impl Scene {
    /// Cria cena vazia
    pub fn new() -> Self {
        Self {
            lights: Vec::new(),
            surfaces: Vec::new(),
            camera: Camera::new(
                [0.0, 0.0, 5.0],
                [0.0, 0.0, 0.0],
                std::f64::consts::FRAC_PI_3,
            ),
            ambient_refractive_index: 1.0, // Vácuo/ar
        }
    }

    /// Adiciona luz à cena
    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    /// Adiciona superfície à cena
    pub fn add_surface(&mut self, surface: Surface) {
        self.surfaces.push(surface);
    }

    /// Define câmera
    pub fn set_camera(&mut self, camera: Camera) {
        self.camera = camera;
    }

    /// Número total de objetos
    pub fn num_objects(&self) -> usize {
        self.lights.len() + self.surfaces.len()
    }

    /// Valida cena
    pub fn is_valid(&self) -> bool {
        !self.lights.is_empty() && !self.surfaces.is_empty()
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_material_creation() {
        let mat = Material::lambertian(0.8);
        assert_eq!(mat.reflectance(), 0.8);

        let glass = Material::glass();
        assert_eq!(glass.refractive_index(), 1.5);
    }

    #[test]
    fn test_surface_creation() {
        let surf = Surface::lambertian([0.0, 0.0, 0.0], [0.0, 1.0, 0.0], 0.8);
        assert_eq!(surf.material.reflectance(), 0.8);
    }

    #[test]
    fn test_light_creation() {
        let light = Light::point([0.0, 5.0, 0.0], 100.0);
        assert!(light.photon_energy() > 0.0);
    }

    #[test]
    fn test_camera_ray_generation() {
        let camera = Camera::new(
            [0.0, 0.0, 5.0],
            [0.0, 0.0, 0.0],
            std::f64::consts::FRAC_PI_4,
        );
        let ray = camera.generate_ray(0.5, 0.5);

        // Raio central deve apontar para frente
        assert!(ray[2] < 0.0);
    }

    #[test]
    fn test_scene_creation() {
        let mut scene = Scene::new();
        scene.add_light(Light::point([0.0, 5.0, 0.0], 100.0));
        scene.add_surface(Surface::lambertian([0.0, 0.0, 0.0], [0.0, 1.0, 0.0], 0.8));

        assert!(scene.is_valid());
        assert_eq!(scene.num_objects(), 2);
    }
}
