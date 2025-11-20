//! Renderizador QED usando Path Integral Monte Carlo

use crate::amplitude::ComplexAmplitude;
use crate::photon::{InteractionType, PhotonPath, Vertex};
use crate::scene::{Scene, Surface};
use rand::Rng;
use rayon::prelude::*;

#[cfg(feature = "diagnostics")]
use crate::diagnostics::{RenderMetrics, RenderTimer, SpectralMode};

/// Configuração do renderizador
#[derive(Debug, Clone)]
pub struct RenderConfig {
    /// Samples por pixel (SPP)
    pub samples_per_pixel: usize,

    /// Profundidade máxima de caminhos
    pub max_path_depth: usize,

    /// Número de caminhos a amostrar por pixel
    pub num_paths: usize,

    /// Uso de paralelismo
    pub parallel: bool,

    /// Threshold de probabilidade para Russian Roulette
    pub rr_threshold: f64,

    /// Comprimentos de onda para amostragem espectral (nm)
    /// Se None, usa apenas monocromático em 550nm (verde)
    /// Exemplo: [380.0, 510.0, 650.0] para RGB
    pub wavelength_bands: Option<Vec<f64>>,

    /// Habilita diagnóstico de performance
    pub enable_diagnostics: bool,
}

impl RenderConfig {
    /// Configuração padrão
    pub fn default() -> Self {
        Self {
            samples_per_pixel: 100,
            max_path_depth: 5,
            num_paths: 1000,
            parallel: true,
            rr_threshold: 0.1,
            wavelength_bands: None,
            enable_diagnostics: false,
        }
    }

    /// Configuração rápida (preview)
    pub fn preview() -> Self {
        Self {
            samples_per_pixel: 10,
            max_path_depth: 3,
            num_paths: 100,
            parallel: true,
            rr_threshold: 0.2,
            wavelength_bands: None,
            enable_diagnostics: false,
        }
    }

    /// Configuração alta qualidade
    pub fn high_quality() -> Self {
        Self {
            samples_per_pixel: 1000,
            max_path_depth: 10,
            num_paths: 10000,
            parallel: true,
            rr_threshold: 0.05,
            wavelength_bands: None,
            enable_diagnostics: false,
        }
    }

    /// Configuração com amostragem espectral RGB
    pub fn with_wavelength_bands(mut self, wavelengths: Vec<f64>) -> Self {
        self.wavelength_bands = Some(wavelengths);
        self
    }

    /// Habilita diagnóstico de performance
    pub fn with_diagnostics(mut self, enabled: bool) -> Self {
        self.enable_diagnostics = enabled;
        self
    }
}

/// Renderizador QED
pub struct QEDRenderer {
    /// Configuração
    config: RenderConfig,

    /// Gerador de números aleatórios (para reprodutibilidade)
    seed: u64,
}

impl QEDRenderer {
    /// Cria novo renderizador
    pub fn new(config: RenderConfig) -> Self {
        Self { config, seed: 42 }
    }

    /// Cria renderizador com configuração padrão
    pub fn default() -> Self {
        Self::new(RenderConfig::default())
    }

    /// Define seed para reprodutibilidade
    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seed = seed;
        self
    }

    /// Renderiza cena completa
    ///
    /// Retorna imagem como vetor de intensidades [0.0, 1.0]
    pub fn render(&self, scene: &Scene) -> Vec<Vec<f64>> {
        let (width, height) = scene.camera.resolution;

        #[cfg(feature = "diagnostics")]
        let timer = RenderTimer::start();

        let image = if self.config.parallel {
            self.render_parallel(scene, width, height)
        } else {
            self.render_sequential(scene, width, height)
        };

        #[cfg(feature = "diagnostics")]
        if self.config.enable_diagnostics {
            let mut metrics = RenderMetrics::new(
                (width, height),
                self.config.samples_per_pixel,
                self.config.num_paths,
            );
            metrics.total_duration = timer.elapsed();
            metrics.max_depth = self.config.max_path_depth;
            metrics.parallel_execution = self.config.parallel;
            metrics.compute_image_stats(&image);
            metrics.compute_throughput();

            // Define modo espectral
            metrics.spectral_mode = if let Some(ref bands) = self.config.wavelength_bands {
                SpectralMode::Multispectral {
                    bands: bands.clone(),
                }
            } else {
                SpectralMode::Monochromatic { wavelength: 550e-9 }
            };

            println!("\n=== Métricas de Renderização ===");
            println!("Duração: {:.2}s", metrics.total_duration.as_secs_f64());
            println!(
                "Throughput: {:.1} pixels/s",
                metrics.throughput_pixels_per_sec
            );
            println!("Intensidade média: {:.4}", metrics.mean_intensity);

            if let Ok(json) = metrics.to_json() {
                println!("\n{}", json);
            }
        }

        image
    }

    /// Renderização sequencial
    fn render_sequential(&self, scene: &Scene, width: usize, height: usize) -> Vec<Vec<f64>> {
        let mut image = vec![vec![0.0; width]; height];
        let mut rng = rand::thread_rng();

        for y in 0..height {
            for x in 0..width {
                let pixel_value = self.render_pixel(scene, x, y, width, height, &mut rng);
                image[y][x] = pixel_value;
            }

            if y % 10 == 0 {
                println!("Progress: {:.1}%", 100.0 * y as f64 / height as f64);
            }
        }

        image
    }

    /// Renderização paralela (mais rápido)
    fn render_parallel(&self, scene: &Scene, width: usize, height: usize) -> Vec<Vec<f64>> {
        let pixels: Vec<(usize, usize)> = (0..height)
            .flat_map(|y| (0..width).map(move |x| (x, y)))
            .collect();

        let results: Vec<f64> = pixels
            .par_iter()
            .map(|&(x, y)| {
                let mut rng = rand::thread_rng();
                self.render_pixel(scene, x, y, width, height, &mut rng)
            })
            .collect();

        // Reorganiza em matriz
        let mut image = vec![vec![0.0; width]; height];
        for (i, &value) in results.iter().enumerate() {
            let y = i / width;
            let x = i % width;
            image[y][x] = value;
        }

        image
    }

    /// Renderiza um único pixel
    fn render_pixel<R: Rng>(
        &self,
        scene: &Scene,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        rng: &mut R,
    ) -> f64 {
        let mut total_amplitude = ComplexAmplitude::zero();

        // Coordenadas normalizadas [0, 1]
        let u = (x as f64 + 0.5) / width as f64;
        let v = (y as f64 + 0.5) / height as f64;

        // Amostra múltiplos caminhos
        for _ in 0..self.config.samples_per_pixel {
            // Adiciona jitter para anti-aliasing
            let u_jitter = u + (rng.gen::<f64>() - 0.5) / width as f64;
            let v_jitter = v + (rng.gen::<f64>() - 0.5) / height as f64;

            // Gera caminhos de fótons
            for _ in 0..self.config.num_paths {
                let path = self.sample_photon_path(scene, u_jitter, v_jitter, rng);
                if path.is_valid() {
                    total_amplitude = total_amplitude + path.total_amplitude;
                }
            }
        }

        // Normaliza por número de samples
        let normalization = (self.config.samples_per_pixel * self.config.num_paths) as f64;
        let normalized_amplitude = total_amplitude * (1.0 / normalization);

        // Intensidade = |A|²
        normalized_amplitude.probability().clamp(0.0, 1.0)
    }

    /// Amostra um caminho de fóton usando Monte Carlo
    ///
    /// Implementa path integral formulation:
    /// A = Σ_caminhos exp(i·S[caminho]/ℏ)
    fn sample_photon_path<R: Rng>(&self, scene: &Scene, u: f64, v: f64, rng: &mut R) -> PhotonPath {
        let mut path = PhotonPath::new();

        // 1. Seleciona luz aleatoriamente
        if scene.lights.is_empty() {
            return path;
        }
        let light_idx = rng.gen_range(0..scene.lights.len());
        let light = &scene.lights[light_idx];

        // 2. Gera direção de emissão aleatória
        let direction = self.sample_direction(rng);
        let emission_vertex = light.emit_vertex(direction);
        path.add_vertex(emission_vertex);

        // 3. Propaga fóton até atingir superfície ou sair da cena
        let mut current_pos = light.position;
        let mut current_dir = direction;
        let mut depth = 0;

        while depth < self.config.max_path_depth {
            // Tenta intersectar com superfícies
            if let Some((surface, t)) = self.ray_intersect(scene, current_pos, current_dir) {
                // Cria vértice de interação
                let hit_pos = [
                    current_pos[0] + t * current_dir[0],
                    current_pos[1] + t * current_dir[1],
                    current_pos[2] + t * current_dir[2],
                ];

                // Determina tipo de interação baseado no material
                let interaction = self.sample_interaction(&surface, current_dir, rng);

                let mut vertex = Vertex::new(
                    hit_pos,
                    t / crate::SPEED_OF_LIGHT,
                    interaction,
                    current_dir,
                    light.photon_energy(),
                );
                vertex.refractive_index = surface.material.refractive_index();

                path.add_vertex(vertex);

                // Russian Roulette para terminar caminhos
                if rng.gen::<f64>() < self.config.rr_threshold {
                    break;
                }

                // Atualiza posição e direção
                current_pos = hit_pos;
                current_dir = self.sample_new_direction(&surface, current_dir, interaction, rng);
                depth += 1;
            } else {
                // Raio escapou da cena
                break;
            }
        }

        // 4. Adiciona vértice de detecção
        let detection_vertex = scene.camera.detect_vertex(u, v);
        path.add_vertex(detection_vertex);

        // 5. Calcula amplitude total do caminho
        path.compute_total_amplitude();

        path
    }

    /// Amostra direção uniforme na esfera
    fn sample_direction<R: Rng>(&self, rng: &mut R) -> [f64; 3] {
        let theta = rng.gen::<f64>() * 2.0 * std::f64::consts::PI;
        let phi = (rng.gen::<f64>() * 2.0 - 1.0).acos();

        [phi.sin() * theta.cos(), phi.sin() * theta.sin(), phi.cos()]
    }

    /// Intersecta raio com superfícies da cena
    fn ray_intersect<'a>(
        &self,
        scene: &'a Scene,
        origin: [f64; 3],
        direction: [f64; 3],
    ) -> Option<(&'a Surface, f64)> {
        let mut closest_t = f64::INFINITY;
        let mut closest_surface = None;

        for surface in &scene.surfaces {
            // Intersecção raio-plano simplificada
            let denom = surface.dot(direction);
            if denom.abs() > 1e-6 {
                let to_surface = [
                    surface.position[0] - origin[0],
                    surface.position[1] - origin[1],
                    surface.position[2] - origin[2],
                ];
                let t = (surface.normal[0] * to_surface[0]
                    + surface.normal[1] * to_surface[1]
                    + surface.normal[2] * to_surface[2])
                    / denom;

                if t > 1e-6 && t < closest_t {
                    closest_t = t;
                    closest_surface = Some(surface);
                }
            }
        }

        closest_surface.map(|s| (s, closest_t))
    }

    /// Amostra tipo de interação baseado no material
    fn sample_interaction<R: Rng>(
        &self,
        surface: &Surface,
        _incident: [f64; 3],
        rng: &mut R,
    ) -> InteractionType {
        use crate::scene::Material;

        match surface.material {
            Material::Lambertian { .. } => InteractionType::Scattering,
            Material::Specular { .. } => InteractionType::Reflection,
            Material::Dielectric { .. } => {
                if rng.gen::<f64>() < 0.5 {
                    InteractionType::Reflection
                } else {
                    InteractionType::Refraction
                }
            }
            Material::Metal { .. } => InteractionType::Reflection,
            Material::Absorbing { .. } => InteractionType::Absorption,
        }
    }

    /// Amostra nova direção após interação
    fn sample_new_direction<R: Rng>(
        &self,
        surface: &Surface,
        incident: [f64; 3],
        interaction: InteractionType,
        rng: &mut R,
    ) -> [f64; 3] {
        match interaction {
            InteractionType::Reflection => self.reflect(incident, surface.normal),
            InteractionType::Scattering => self.sample_hemisphere(surface.normal, rng),
            InteractionType::Refraction => self.refract(incident, surface.normal, 1.5, rng),
            _ => incident,
        }
    }

    /// Reflexão especular
    fn reflect(&self, incident: [f64; 3], normal: [f64; 3]) -> [f64; 3] {
        let dot = incident[0] * normal[0] + incident[1] * normal[1] + incident[2] * normal[2];
        [
            incident[0] - 2.0 * dot * normal[0],
            incident[1] - 2.0 * dot * normal[1],
            incident[2] - 2.0 * dot * normal[2],
        ]
    }

    /// Refração (Lei de Snell)
    fn refract<R: Rng>(
        &self,
        incident: [f64; 3],
        normal: [f64; 3],
        ior: f64,
        _rng: &mut R,
    ) -> [f64; 3] {
        let dot = -(incident[0] * normal[0] + incident[1] * normal[1] + incident[2] * normal[2]);
        let eta = if dot > 0.0 { 1.0 / ior } else { ior };

        let k = 1.0 - eta * eta * (1.0 - dot * dot);
        if k < 0.0 {
            // Reflexão total interna
            self.reflect(incident, normal)
        } else {
            let scale = eta * dot - k.sqrt();
            [
                eta * incident[0] + scale * normal[0],
                eta * incident[1] + scale * normal[1],
                eta * incident[2] + scale * normal[2],
            ]
        }
    }

    /// Amostra direção no hemisfério
    fn sample_hemisphere<R: Rng>(&self, normal: [f64; 3], rng: &mut R) -> [f64; 3] {
        let dir = self.sample_direction(rng);
        let dot = dir[0] * normal[0] + dir[1] * normal[1] + dir[2] * normal[2];

        if dot > 0.0 {
            dir
        } else {
            [-dir[0], -dir[1], -dir[2]]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_config() {
        let config = RenderConfig::default();
        assert!(config.samples_per_pixel > 0);

        let preview = RenderConfig::preview();
        assert!(preview.samples_per_pixel < config.samples_per_pixel);
    }

    #[test]
    fn test_renderer_creation() {
        let config = RenderConfig::default();
        let renderer = QEDRenderer::new(config);
        assert_eq!(renderer.seed, 42);
    }

    #[test]
    fn test_direction_sampling() {
        let renderer = QEDRenderer::default();
        let mut rng = rand::thread_rng();

        let dir = renderer.sample_direction(&mut rng);
        let len = (dir[0] * dir[0] + dir[1] * dir[1] + dir[2] * dir[2]).sqrt();
        assert!((len - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_reflection() {
        let renderer = QEDRenderer::default();
        let incident = [0.0, -1.0, 0.0];
        let normal = [0.0, 1.0, 0.0];

        let reflected = renderer.reflect(incident, normal);
        assert_eq!(reflected[1], 1.0);
    }
}
