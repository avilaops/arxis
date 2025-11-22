//! Exemplo básico de renderização quântica (QED)
//!
//! Demonstra renderização usando path integral formulation

use avx_quantum_render::scene::{Camera, Light, Material, Surface};
use avx_quantum_render::{QEDRenderer, RenderConfig, Scene};

fn main() {
    println!("=== AVX Quantum Renderer - QED Path Integral Demo ===\n");

    // 1. Criar cena simples
    let scene = create_cornell_box();
    println!("✓ Cena criada: {} objetos", scene.num_objects());

    // 2. Configurar renderizador
    let config = RenderConfig::preview(); // Usa preview para ser rápido
    println!("✓ Renderizador QED configurado");
    println!("  - Samples por pixel: {}", config.samples_per_pixel);
    println!("  - Profundidade máxima: {}", config.max_path_depth);
    println!("  - Caminhos por sample: {}", config.num_paths);

    let renderer = QEDRenderer::new(config).with_seed(12345);

    // 3. Renderizar
    println!("\n🎨 Renderizando com Path Integral Monte Carlo...");
    let start = std::time::Instant::now();
    let image = renderer.render(&scene);
    let elapsed = start.elapsed();
    println!("✓ Renderização concluída em {:.2}s", elapsed.as_secs_f64());

    // 4. Estatísticas
    display_image_stats(&image);

    // 5. Salvar imagem ASCII
    save_ascii_image(&image, "quantum_render.txt");
    println!("\n✓ Imagem salva em: quantum_render.txt");

    // 6. Demonstrar cálculos quânticos
    demonstrate_quantum_calculations();
}

/// Cria Cornell Box clássica
fn create_cornell_box() -> Scene {
    let mut scene = Scene::new();

    // Luz pontual no topo
    scene.add_light(Light::point([0.0, 4.9, 0.0], 50.0));

    // Chão (cinza difuso)
    scene.add_surface(Surface::new(
        [0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        Material::lambertian(0.8),
    ));

    // Parede esquerda (vermelha)
    scene.add_surface(Surface::new(
        [-5.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        Material::lambertian(0.7),
    ));

    // Parede direita (verde)
    scene.add_surface(Surface::new(
        [5.0, 0.0, 0.0],
        [-1.0, 0.0, 0.0],
        Material::lambertian(0.7),
    ));

    // Parede fundo (branca)
    scene.add_surface(Surface::new(
        [0.0, 0.0, -5.0],
        [0.0, 0.0, 1.0],
        Material::lambertian(0.9),
    ));

    // Espelho (parede direita)
    scene.add_surface(Surface::new(
        [3.0, 2.0, 0.0],
        [-1.0, 0.0, 0.0],
        Material::mirror(),
    ));

    // Esfera de vidro
    scene.add_surface(Surface::new(
        [-2.0, 1.0, -2.0],
        [0.0, 1.0, 0.0],
        Material::glass(),
    ));

    // Configurar câmera
    let camera = Camera::new(
        [0.0, 2.5, 8.0],             // Posição
        [0.0, 2.5, 0.0],             // Olhando para
        std::f64::consts::FRAC_PI_3, // FOV
    )
    .with_resolution(80, 40); // Resolução baixa para ASCII

    scene.set_camera(camera);

    scene
}

/// Exibe estatísticas da imagem
fn display_image_stats(image: &[Vec<f64>]) {
    let height = image.len();
    let width = if height > 0 { image[0].len() } else { 0 };

    let mut sum = 0.0;
    let mut min = f64::MAX;
    let mut max = f64::MIN;

    for row in image {
        for &pixel in row {
            sum += pixel;
            min = min.min(pixel);
            max = max.max(pixel);
        }
    }

    let total_pixels = (width * height) as f64;
    let mean = sum / total_pixels;

    println!("\n📊 Estatísticas da Imagem:");
    println!("  Resolução: {}x{}", width, height);
    println!("  Intensidade média: {:.4}", mean);
    println!("  Intensidade mín: {:.4}", min);
    println!("  Intensidade máx: {:.4}", max);
}

/// Salva imagem como ASCII art
fn save_ascii_image(image: &[Vec<f64>], filename: &str) {
    let chars = " .:-=+*#%@";
    let mut output = String::new();

    output.push_str("=== Quantum Rendered Image (ASCII) ===\n\n");

    for row in image {
        for &pixel in row {
            let index = (pixel * (chars.len() - 1) as f64) as usize;
            let ch = chars.chars().nth(index.min(chars.len() - 1)).unwrap();
            output.push(ch);
        }
        output.push('\n');
    }

    std::fs::write(filename, output).expect("Falha ao salvar imagem");
}

/// Demonstra cálculos quânticos individuais
fn demonstrate_quantum_calculations() {
    use avx_quantum_render::amplitude::{ComplexAmplitude, PhaseAccumulator};
    use avx_quantum_render::feynman::{compton_scattering_amplitude, FeynmanVertex};
    use avx_quantum_render::photon::{InteractionType, PhotonPath, Vertex};
    use avx_quantum_render::FINE_STRUCTURE;

    println!("\n\n🔬 === Demonstração de Cálculos Quânticos ===\n");

    // 1. Amplitude Complexa
    println!("1. Amplitude Complexa:");
    let amp1 = ComplexAmplitude::from_polar(2.0, std::f64::consts::PI / 4.0);
    let amp2 = ComplexAmplitude::from_polar(1.5, std::f64::consts::PI / 3.0);
    let sum = amp1 + amp2;
    println!(
        "   |A₁| = {:.3}, φ₁ = {:.3}°",
        amp1.magnitude(),
        amp1.phase().to_degrees()
    );
    println!(
        "   |A₂| = {:.3}, φ₂ = {:.3}°",
        amp2.magnitude(),
        amp2.phase().to_degrees()
    );
    println!("   |A₁+A₂| = {:.3}", sum.magnitude());
    println!("   P(A₁+A₂) = |A|² = {:.4}", sum.probability());

    // 2. Acumulação de Fase
    println!("\n2. Acumulação de Fase Quântica:");
    let mut phase = PhaseAccumulator::new();
    let wavelength = 550e-9; // Verde (550nm)
    let distance = 1e-6; // 1 micrômetro
    phase.add_propagation(distance, wavelength, 1.0);
    println!("   λ = {:.0} nm", wavelength * 1e9);
    println!("   d = {:.1} μm", distance * 1e6);
    println!(
        "   Fase acumulada: {:.2} rad = {:.1}°",
        phase.total_phase,
        phase.total_phase.to_degrees()
    );
    let amp = phase.to_amplitude();
    println!(
        "   Amplitude resultante: {:.3}∠{:.1}°",
        amp.magnitude(),
        amp.phase().to_degrees()
    );

    // 3. Vértice de Feynman
    println!("\n3. Vértice de Feynman (QED):");
    let mut vertex = FeynmanVertex::new([0.0, 0.0, 0.0], InteractionType::Emission);
    vertex.compute_qed_amplitude();
    println!(
        "   Constante de estrutura fina: α = {:.6} ≈ 1/{:.0}",
        FINE_STRUCTURE,
        1.0 / FINE_STRUCTURE
    );
    println!(
        "   Coupling constant: e = √(4πα) = {:.6}",
        (4.0 * std::f64::consts::PI * FINE_STRUCTURE).sqrt()
    );
    println!(
        "   Amplitude do vértice: |V| = {:.6}",
        vertex.amplitude.magnitude()
    );

    // 4. Espalhamento Compton
    println!("\n4. Espalhamento Compton (γ + e⁻ → γ + e⁻):");
    let photon_in = 1e-15; // 1 keV
    let photon_out = 0.9e-15;
    let angle = std::f64::consts::FRAC_PI_4; // 45°
    let compton_amp = compton_scattering_amplitude(photon_in, photon_out, angle);
    println!("   E_in = {:.1} keV", photon_in * 1e18 / 1.602176634);
    println!("   E_out = {:.1} keV", photon_out * 1e18 / 1.602176634);
    println!("   θ = {:.0}°", angle.to_degrees());
    println!("   Amplitude: {:.6}", compton_amp.magnitude());
    println!(
        "   Seção de choque ∝ |A|²: {:.3e}",
        compton_amp.probability()
    );

    // 5. Caminho de Fóton
    println!("\n5. Caminho Quântico de Fóton:");
    let mut path = PhotonPath::new();
    let v1 = Vertex::emission([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], 3e-19);
    let v2 = Vertex::new(
        [1.0, 0.0, 0.0],
        1e-9,
        InteractionType::Scattering,
        [0.0, 1.0, 0.0],
        3e-19,
    );
    let v3 = Vertex::detection([1.0, 1.0, 0.0], 2e-9);

    path.add_vertex(v1);
    path.add_vertex(v2);
    path.add_vertex(v3);
    path.compute_total_amplitude();

    println!("   Vértices: {}", path.vertices.len());
    println!("   Interações: {}", path.num_interactions());
    println!("   Comprimento óptico: {:.2} m", path.optical_length());
    println!("   Fase total: {:.2} rad", path.total_phase);
    println!("   |A_total| = {:.6}", path.total_amplitude.magnitude());
    println!("   Probabilidade: P = {:.3e}", path.probability());
    println!("   Caminho válido: {}", path.is_valid());

    println!("\n✓ Demonstração de física quântica concluída!");
}
