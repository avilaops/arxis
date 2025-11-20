//! Benchmarks de renderização QED
//!
//! Compara performance de:
//! - Diferentes configurações (preview, default, high-quality)
//! - Monocromático vs espectral
//! - Serial vs paralelo
//! - Diferentes números de caminhos
//!
//! Executar: `cargo bench --bench render_bench`

use avx_quantum_render::prelude::*;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

/// Cria Cornell Box padrão para benchmarks
fn create_test_scene() -> Scene {
    let mut scene = Scene::new();

    // Luz pontual
    scene.add_light(Light::point([0.0, 4.9, 0.0], 100.0));

    // Chão
    scene.add_surface(Surface::lambertian([0.0, 0.0, 0.0], [0.0, 1.0, 0.0], 0.8));

    // Paredes
    scene.add_surface(Surface::lambertian([-5.0, 0.0, 0.0], [1.0, 0.0, 0.0], 0.7));

    scene.add_surface(Surface::lambertian([5.0, 0.0, 0.0], [-1.0, 0.0, 0.0], 0.7));

    scene.add_surface(Surface::lambertian([0.0, 0.0, -5.0], [0.0, 0.0, 1.0], 0.8));

    // Teto
    scene.add_surface(Surface::lambertian([0.0, 5.0, 0.0], [0.0, -1.0, 0.0], 0.8));

    scene
}

/// Benchmark: Configurações de qualidade
fn bench_quality_presets(c: &mut Criterion) {
    let scene = create_test_scene();
    let mut group = c.benchmark_group("quality_presets");

    // Preview quality (10 SPP)
    group.bench_function("preview", |b| {
        let config = RenderConfig::preview();
        let renderer = QEDRenderer::new(config);
        b.iter(|| {
            let image = renderer.render(black_box(&scene));
            black_box(image);
        });
    });

    // Default quality (100 SPP)
    group.bench_function("default", |b| {
        let config = RenderConfig::default();
        let renderer = QEDRenderer::new(config);
        b.iter(|| {
            let image = renderer.render(black_box(&scene));
            black_box(image);
        });
    });

    group.finish();
}

/// Benchmark: Monocromático vs Espectral RGB
fn bench_spectral_modes(c: &mut Criterion) {
    let scene = create_test_scene();
    let mut group = c.benchmark_group("spectral_modes");

    // Monocromático
    group.bench_function("monochromatic", |b| {
        let config = RenderConfig::preview();
        let renderer = QEDRenderer::new(config);
        b.iter(|| {
            let image = renderer.render(black_box(&scene));
            black_box(image);
        });
    });

    // RGB espectral (3 bandas)
    group.bench_function("spectral_rgb", |b| {
        let config = RenderConfig::preview().with_wavelength_bands(vec![
            650.0e-9, // Vermelho
            510.0e-9, // Verde
            380.0e-9, // Azul
        ]);
        let renderer = QEDRenderer::new(config);
        b.iter(|| {
            let image = renderer.render(black_box(&scene));
            black_box(image);
        });
    });

    // Espectral completo (7 bandas)
    group.bench_function("spectral_full", |b| {
        let config = RenderConfig::preview().with_wavelength_bands(vec![
            700.0e-9, // Vermelho profundo
            650.0e-9, // Vermelho
            580.0e-9, // Amarelo
            550.0e-9, // Verde
            480.0e-9, // Ciano
            450.0e-9, // Azul
            400.0e-9, // Violeta
        ]);
        let renderer = QEDRenderer::new(config);
        b.iter(|| {
            let image = renderer.render(black_box(&scene));
            black_box(image);
        });
    });

    group.finish();
}

/// Benchmark: Serial vs Paralelo
fn bench_parallel_modes(c: &mut Criterion) {
    let scene = create_test_scene();
    let mut group = c.benchmark_group("parallel_modes");

    // Serial
    group.bench_function("serial", |b| {
        let mut config = RenderConfig::preview();
        config.parallel = false;
        let renderer = QEDRenderer::new(config);
        b.iter(|| {
            let image = renderer.render(black_box(&scene));
            black_box(image);
        });
    });

    // Paralelo (Rayon)
    group.bench_function("parallel_rayon", |b| {
        let mut config = RenderConfig::preview();
        config.parallel = true;
        let renderer = QEDRenderer::new(config);
        b.iter(|| {
            let image = renderer.render(black_box(&scene));
            black_box(image);
        });
    });

    group.finish();
}

/// Benchmark: Escalabilidade com número de caminhos
fn bench_path_count(c: &mut Criterion) {
    let scene = create_test_scene();
    let mut group = c.benchmark_group("path_count");

    for num_paths in [10, 50, 100, 500, 1000].iter() {
        group.throughput(Throughput::Elements(*num_paths));
        group.bench_with_input(
            BenchmarkId::from_parameter(num_paths),
            num_paths,
            |b, &num_paths| {
                let mut config = RenderConfig::preview();
                config.num_paths = num_paths as usize;
                let renderer = QEDRenderer::new(config);
                b.iter(|| {
                    let image = renderer.render(black_box(&scene));
                    black_box(image);
                });
            },
        );
    }

    group.finish();
}

/// Benchmark: Componentes individuais
fn bench_components(c: &mut Criterion) {
    let mut group = c.benchmark_group("components");

    // Amplitude complexa - operações básicas
    group.bench_function("amplitude_operations", |b| {
        b.iter(|| {
            let a1 = ComplexAmplitude::from_polar(1.0, 0.5);
            let a2 = ComplexAmplitude::from_polar(0.8, 1.2);
            let sum = black_box(a1 + a2);
            let prod = black_box(a1 * a2);
            let prob = black_box(sum.probability());
            black_box((sum, prod, prob));
        });
    });

    // Acumulação de fase
    group.bench_function("phase_accumulation", |b| {
        b.iter(|| {
            let mut phase = PhaseAccumulator::new();
            phase.add_propagation(1e-3, 550e-9, 1.0);
            phase.add_propagation(2e-3, 550e-9, 1.5);
            phase.add_interface_phase(std::f64::consts::PI);
            let amp = black_box(phase.to_amplitude());
            black_box(amp);
        });
    });

    // Coerência com atenuação
    group.bench_function("coherence_attenuation", |b| {
        b.iter(|| {
            let mut phase = PhaseAccumulator::with_coherence_length(10e-6);
            for _ in 0..10 {
                phase.add_propagation(100e-6, 550e-9, 1.0);
            }
            let amp = black_box(phase.to_amplitude());
            black_box(amp);
        });
    });

    // Criação e validação de caminho
    group.bench_function("path_creation", |b| {
        b.iter(|| {
            let mut path = PhotonPath::new();
            path.add_vertex(Vertex::emission([0.0, 5.0, 0.0], [0.0, -1.0, 0.0], 3.6e-19));
            path.add_vertex(Vertex::new(
                [0.0, 0.0, 0.0],
                1e-8,
                InteractionType::Reflection,
                [1.0, 0.0, 0.0],
                3.6e-19,
            ));
            path.add_vertex(Vertex::detection([1.0, 0.0, 0.0], 2e-8));
            path.compute_total_amplitude();
            black_box(path.is_valid());
        });
    });

    // Diagnóstico completo
    group.bench_function("path_diagnostics", |b| {
        let mut path = PhotonPath::new();
        path.add_vertex(Vertex::emission([0.0, 5.0, 0.0], [0.0, -1.0, 0.0], 3.6e-19));
        for i in 0..5 {
            path.add_vertex(Vertex::new(
                [i as f64, 0.0, 0.0],
                (i as f64) * 1e-8,
                InteractionType::Scattering,
                [1.0, 0.0, 0.0],
                3.6e-19,
            ));
        }
        path.add_vertex(Vertex::detection([5.0, 0.0, 0.0], 6e-8));
        path.compute_total_amplitude();

        b.iter(|| {
            let diag = black_box(path.emit_diagnostics());
            black_box(diag);
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_quality_presets,
    bench_spectral_modes,
    bench_parallel_modes,
    bench_path_count,
    bench_components
);

criterion_main!(benches);
