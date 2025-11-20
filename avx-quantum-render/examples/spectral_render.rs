//! Exemplo de renderização espectral com múltiplos comprimentos de onda
//!
//! Demonstra:
//! - Amostragem espectral discreta (RGB)
//! - Controle de coerência
//! - Diagnóstico de caminhos
//! - Reconstrução de cor RGB a partir de espectro

use avx_quantum_render::prelude::*;
use std::time::Instant;

fn main() {
    println!("🌈 AVX Quantum Render - Spectral Rendering Demo");
    println!("================================================\n");

    // Demo 1: Renderização monocromática vs espectral
    demo_spectral_comparison();

    // Demo 2: Efeitos de coerência
    demo_coherence_effects();

    // Demo 3: Diagnóstico de caminhos
    demo_path_diagnostics();
}

/// Demo 1: Compara renderização monocromática com espectral RGB
fn demo_spectral_comparison() {
    println!("📊 Demo 1: Renderização Monocromática vs Espectral RGB\n");

    // Cena simples: Cornell Box
    let scene = create_cornell_box();

    // Configuração monocromática (550nm - verde)
    let config_mono = RenderConfig::preview();
    let renderer_mono = QEDRenderer::new(config_mono);

    println!("Renderizando monocromático (550nm)...");
    let start = Instant::now();
    let _image_mono = renderer_mono.render(&scene);
    let duration_mono = start.elapsed();
    println!("✓ Concluído em {:?}", duration_mono);

    // Configuração espectral RGB
    let config_rgb = RenderConfig::preview().with_wavelength_bands(vec![
        650.0e-9, // Vermelho
        510.0e-9, // Verde
        380.0e-9, // Azul/Violeta
    ]);
    let renderer_rgb = QEDRenderer::new(config_rgb);

    println!("\nRenderizando espectral RGB (650nm, 510nm, 380nm)...");
    let start = Instant::now();
    let _image_rgb = renderer_rgb.render(&scene);
    let duration_rgb = start.elapsed();
    println!("✓ Concluído em {:?}", duration_rgb);

    println!(
        "\n⚡ Overhead espectral: {:.1}x mais lento\n",
        duration_rgb.as_secs_f64() / duration_mono.as_secs_f64()
    );
}

/// Demo 2: Demonstra efeitos de comprimento de coerência
fn demo_coherence_effects() {
    println!("🌀 Demo 2: Efeitos de Coerência\n");

    // Caso 1: Coerência infinita (laser ideal)
    let mut phase_laser = PhaseAccumulator::new();
    phase_laser.add_propagation(1e-3, 632.8e-9, 1.0); // 1mm, HeNe laser

    let amp_laser = phase_laser.to_amplitude();
    println!(
        "Laser HeNe (coerência infinita):\n  \
         Caminho: 1 mm\n  \
         Amplitude: {:.6}\n  \
         Fase: {:.2} rad\n  \
         Probabilidade: {:.6}\n",
        amp_laser.magnitude(),
        amp_laser.phase(),
        amp_laser.probability()
    );

    // Caso 2: LED (coerência limitada ~10μm)
    let mut phase_led = PhaseAccumulator::with_coherence_length(10e-6);
    phase_led.add_propagation(1e-3, 520e-9, 1.0); // 1mm, LED verde

    let amp_led = phase_led.to_amplitude();
    println!(
        "LED verde (L_c = 10 μm):\n  \
         Caminho: 1 mm\n  \
         Amplitude: {:.6} (atenuada!)\n  \
         Fase: {:.2} rad\n  \
         Probabilidade: {:.6}\n",
        amp_led.magnitude(),
        amp_led.phase(),
        amp_led.probability()
    );

    // Caso 3: Luz solar (coerência muito curta ~1μm)
    let mut phase_sun = PhaseAccumulator::with_coherence_length(1e-6);
    phase_sun.add_propagation(1e-3, 550e-9, 1.0); // 1mm, luz amarela

    let amp_sun = phase_sun.to_amplitude();
    println!(
        "Luz solar (L_c = 1 μm):\n  \
         Caminho: 1 mm\n  \
         Amplitude: {:.6} (fortemente atenuada!)\n  \
         Fase: {:.2} rad\n  \
         Probabilidade: {:.8}\n",
        amp_sun.magnitude(),
        amp_sun.phase(),
        amp_sun.probability()
    );

    // Comparação visual
    println!("📉 Atenuação relativa:");
    println!(
        "  LED / Laser:  {:.2}%",
        (amp_led.probability() / amp_laser.probability()) * 100.0
    );
    println!(
        "  Sol / Laser:  {:.2}%\n",
        (amp_sun.probability() / amp_laser.probability()) * 100.0
    );
}

/// Demo 3: Diagnóstico detalhado de caminhos
fn demo_path_diagnostics() {
    println!("🔍 Demo 3: Diagnóstico de Caminhos de Fótons\n");

    // Criar caminho manualmente
    let mut path = PhotonPath::new();

    // Vértice 1: Emissão (luz)
    let v1 = Vertex::emission([0.0, 5.0, 0.0], [0.0, -1.0, 0.0], 3.6e-19); // ~550nm
    path.add_vertex(v1);

    // Vértice 2: Reflexão (chão)
    let mut v2 = Vertex::new(
        [0.0, 0.0, 0.0],
        1.67e-8,
        InteractionType::Reflection,
        [0.7, 0.7, 0.0],
        3.6e-19,
    );
    v2.refractive_index = 1.0;
    path.add_vertex(v2);

    // Vértice 3: Espalhamento (parede)
    let mut v3 = Vertex::new(
        [3.5, 3.5, 0.0],
        3.34e-8,
        InteractionType::Scattering,
        [-0.5, 0.5, 0.7],
        3.6e-19,
    );
    v3.refractive_index = 1.0;
    path.add_vertex(v3);

    // Vértice 4: Detecção (câmera)
    let v4 = Vertex::detection([2.0, 4.0, 2.0], 4.5e-8);
    path.add_vertex(v4);

    // Calcular amplitude total
    path.compute_total_amplitude();

    // Emitir diagnósticos
    let diagnostics = path.emit_diagnostics();

    println!("📋 Resumo do Caminho:");
    println!("  Vértices: {}", diagnostics.num_vertices);
    println!("  Interações: {}", diagnostics.num_interactions);
    println!(
        "  Amplitude total: {:.6}",
        diagnostics.total_amplitude_magnitude
    );
    println!("  Fase total: {:.2} rad", diagnostics.total_phase);
    println!("  Probabilidade: {:.6}", diagnostics.total_probability);
    println!("  Comprimento óptico: {:.3} m", diagnostics.optical_length);
    println!("  Peso: {:.3}\n", diagnostics.weight);

    println!("🔸 Vértices:");
    for (i, v) in diagnostics.vertices.iter().enumerate() {
        println!(
            "  [{}] {} @ ({:.1}, {:.1}, {:.1})",
            i, v.interaction_type, v.position[0], v.position[1], v.position[2]
        );
        println!(
            "      λ = {:.1} nm, n = {:.2}",
            v.wavelength * 1e9,
            v.refractive_index
        );
    }

    println!("\n⚡ Interações:");
    for (i, int) in diagnostics.interactions.iter().enumerate() {
        println!(
            "  [{}→{}] |A| = {:.6}, φ = {:.2} rad, P = {:.6}, d = {:.3} m",
            i,
            i + 1,
            int.amplitude_magnitude,
            int.amplitude_phase,
            int.probability,
            int.distance
        );
    }

    println!(
        "\n✨ Este caminho contribui com {:.2}% para o pixel final\n",
        diagnostics.total_probability * 100.0
    );
}

/// Cria Cornell Box simples para testes
fn create_cornell_box() -> Scene {
    let mut scene = Scene::new();

    // Luz pontual no topo
    scene.add_light(Light::point([0.0, 4.9, 0.0], 100.0));

    // Chão (cinza difuso)
    scene.add_surface(Surface::lambertian([0.0, 0.0, 0.0], [0.0, 1.0, 0.0], 0.8));

    // Parede esquerda (vermelha)
    scene.add_surface(Surface::lambertian([-5.0, 0.0, 0.0], [1.0, 0.0, 0.0], 0.7));

    // Parede direita (verde)
    scene.add_surface(Surface::lambertian([5.0, 0.0, 0.0], [-1.0, 0.0, 0.0], 0.7));

    // Parede de fundo (branca)
    scene.add_surface(Surface::lambertian([0.0, 0.0, -5.0], [0.0, 0.0, 1.0], 0.8));

    // Teto (branco)
    scene.add_surface(Surface::lambertian([0.0, 5.0, 0.0], [0.0, -1.0, 0.0], 0.8));

    scene
}

/// Converte RGB espectral para cor 8-bit (simplificado)
#[allow(dead_code)]
fn spectral_to_rgb(intensity_red: f64, intensity_green: f64, intensity_blue: f64) -> (u8, u8, u8) {
    let r = (intensity_red * 255.0).clamp(0.0, 255.0) as u8;
    let g = (intensity_green * 255.0).clamp(0.0, 255.0) as u8;
    let b = (intensity_blue * 255.0).clamp(0.0, 255.0) as u8;
    (r, g, b)
}
