//! Exemplo avançado de renderização com diversos materiais e técnicas

use avx_quantum_render::prelude::*;
use avx_quantum_render::scene::Camera;
use avx_quantum_render::{export_ascii, export_ppm};
use std::f64::consts::PI;

fn main() {
    println!("=== AVX Quantum Render - Demonstração Avançada ===\n");

    // Configuração de alta qualidade com diagnósticos
    let config = RenderConfig::default()
        .with_diagnostics(true)
        .with_wavelength_bands(vec![
            650e-9, // Vermelho
            510e-9, // Verde
            380e-9, // Azul/Violeta
        ]);

    let renderer = QEDRenderer::new(config);

    // === CENA 1: Materials Showcase ===
    println!("📐 Criando cena 1: Showcase de Materiais\n");
    let scene1 = create_materials_showcase();

    println!("🎨 Renderizando cena 1...");
    let image1 = renderer.render(&scene1);

    println!("\n📊 ASCII Preview:");
    let ascii = export_ascii(&image1);
    print!("{}", ascii);

    // Exportar sem denoising
    if let Err(e) = export_ppm(&image1, "output_raw.ppm", 1.0) {
        println!("⚠️  Erro ao exportar: {}", e);
    } else {
        println!("✅ Salvo: output_raw.ppm");
    }

    // === CENA 2: Complex Scene with Denoising ===
    println!("\n\n📐 Criando cena 2: Cena Complexa\n");
    let scene2 = create_complex_scene();

    println!("🎨 Renderizando cena 2...");
    let image2 = renderer.render(&scene2);

    // Aplicar denoising
    println!("🔧 Aplicando denoising Gaussiano...");
    let denoised_gaussian = gaussian_blur(&image2, 5);

    println!("🔧 Aplicando denoising Bilateral...");
    let denoised_bilateral = bilateral_filter(&image2, 2.0, 0.1, 5);

    // Exportar versões
    if let Err(e) = export_ppm(&denoised_gaussian, "output_gaussian.ppm", 1.5) {
        println!("⚠️  Erro ao exportar: {}", e);
    } else {
        println!("✅ Salvo: output_gaussian.ppm");
    }

    if let Err(e) = export_ppm(&denoised_bilateral, "output_bilateral.ppm", 1.5) {
        println!("⚠️  Erro ao exportar: {}", e);
    } else {
        println!("✅ Salvo: output_bilateral.ppm");
    }

    // === CENA 3: Glass & Caustics ===
    println!("\n\n📐 Criando cena 3: Vidro e Cáusticas\n");
    let scene3 = create_glass_scene();

    println!("🎨 Renderizando cena 3...");
    let image3 = renderer.render(&scene3);

    if let Err(e) = export_ppm(&image3, "output_glass.ppm", 2.0) {
        println!("⚠️  Erro ao exportar: {}", e);
    } else {
        println!("✅ Salvo: output_glass.ppm");
    }

    println!("\n✅ Renderização completa!");
    println!("\n📁 Arquivos gerados:");
    println!("   - output_raw.ppm (sem denoising)");
    println!("   - output_gaussian.ppm (denoising Gaussiano)");
    println!("   - output_bilateral.ppm (denoising Bilateral)");
    println!("   - output_glass.ppm (cena com vidro)");
}

/// Cria cena demonstrando diversos materiais
fn create_materials_showcase() -> Scene {
    let mut scene = Scene::new();

    // Câmera
    let camera = Camera::new([0.0, 3.0, 8.0], [0.0, 1.0, 0.0], PI / 3.0).with_resolution(120, 60);
    scene.set_camera(camera);

    // Iluminação
    scene.add_light(Light::point([0.0, 8.0, 3.0], 150.0));
    scene.add_light(Light::point([-5.0, 5.0, 5.0], 80.0));

    // Chão difuso
    scene.add_surface(Surface {
        position: [0.0, 0.0, 0.0],
        normal: [0.0, 1.0, 0.0],
        material: Material::Lambertian { albedo: 0.7 },
        area: 100.0,
    });

    // Esfera metálica (esquerda)
    scene.add_surface(Surface {
        position: [-3.0, 1.5, 0.0],
        normal: [0.0, 1.0, 0.0],
        material: Material::Metal {
            reflectance: 0.95,
            roughness: 0.05,
        },
        area: 28.27, // 4πr² com r=1.5
    });

    // Esfera de vidro (centro)
    scene.add_surface(Surface {
        position: [0.0, 1.5, 0.0],
        normal: [0.0, 1.0, 0.0],
        material: Material::Dielectric {
            refractive_index: 1.5,
            transmittance: 0.95,
        },
        area: 28.27,
    });

    // Esfera difusa colorida (direita)
    scene.add_surface(Surface {
        position: [3.0, 1.5, 0.0],
        normal: [0.0, 1.0, 0.0],
        material: Material::Lambertian { albedo: 0.8 },
        area: 28.27,
    });

    // Parede espelho (fundo)
    scene.add_surface(Surface {
        position: [0.0, 3.0, -3.0],
        normal: [0.0, 0.0, 1.0],
        material: Material::Specular { reflectance: 0.9 },
        area: 36.0, // 6m x 6m
    });

    scene
}

/// Cria cena complexa com múltiplas interações
fn create_complex_scene() -> Scene {
    let mut scene = Scene::new();

    // Câmera
    let camera = Camera::new([5.0, 5.0, 10.0], [0.0, 2.0, 0.0], PI / 3.5).with_resolution(100, 80);
    scene.set_camera(camera);

    // Múltiplas luzes
    scene.add_light(Light::point([0.0, 10.0, 0.0], 200.0));
    scene.add_light(Light::point([5.0, 3.0, 5.0], 50.0));
    scene.add_light(Light::point([-5.0, 3.0, 5.0], 50.0));

    // Chão
    scene.add_surface(Surface {
        position: [0.0, 0.0, 0.0],
        normal: [0.0, 1.0, 0.0],
        material: Material::Lambertian { albedo: 0.6 },
        area: 100.0,
    });

    // Grid de objetos com materiais variados
    for i in -2..=2 {
        for j in -2..=2 {
            let x = i as f64 * 2.0;
            let z = j as f64 * 2.0;

            let material = match (i + j) % 4 {
                0 => Material::Lambertian { albedo: 0.8 },
                1 => Material::Specular { reflectance: 0.9 },
                2 => Material::Metal {
                    reflectance: 0.85,
                    roughness: 0.1,
                },
                _ => Material::Dielectric {
                    refractive_index: 1.5,
                    transmittance: 0.9,
                },
            };

            scene.add_surface(Surface {
                position: [x, 1.0, z],
                normal: [0.0, 1.0, 0.0],
                material,
                area: 12.56, // 4πr² com r=1.0
            });
        }
    }

    scene
}

/// Cria cena focada em vidro e cáusticas
fn create_glass_scene() -> Scene {
    let mut scene = Scene::new();

    // Câmera
    let camera = Camera::new([0.0, 3.0, 7.0], [0.0, 1.5, 0.0], PI / 3.0).with_resolution(100, 100);
    scene.set_camera(camera);

    // Luz forte e direcionada
    scene.add_light(Light::point([0.0, 8.0, 2.0], 300.0));

    // Chão branco para mostrar cáusticas
    scene.add_surface(Surface {
        position: [0.0, 0.0, 0.0],
        normal: [0.0, 1.0, 0.0],
        material: Material::Lambertian { albedo: 0.95 },
        area: 100.0,
    });

    // Esfera de vidro grande (centro)
    scene.add_surface(Surface {
        position: [0.0, 2.0, 0.0],
        normal: [0.0, 1.0, 0.0],
        material: Material::Dielectric {
            refractive_index: 1.5,
            transmittance: 0.98,
        },
        area: 50.27, // 4πr² com r=2.0
    });

    // Esferas de vidro menores ao redor
    for i in 0..6 {
        let angle = (i as f64 / 6.0) * 2.0 * PI;
        let radius = 3.5;
        let x = angle.cos() * radius;
        let z = angle.sin() * radius;

        scene.add_surface(Surface {
            position: [x, 1.0, z],
            normal: [0.0, 1.0, 0.0],
            material: Material::Dielectric {
                refractive_index: 1.5,
                transmittance: 0.95,
            },
            area: 12.56, // 4πr² com r=1.0
        });
    }

    // Parede de fundo
    scene.add_surface(Surface {
        position: [0.0, 3.0, -5.0],
        normal: [0.0, 0.0, 1.0],
        material: Material::Lambertian { albedo: 0.5 },
        area: 36.0,
    });

    scene
}
