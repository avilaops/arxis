//! Exemplo: Usando 3DMM (3D Morphable Model)
//!
//! Este exemplo mostra como criar e usar um modelo 3DMM básico
//! para gerar faces 3D paramétricas.

use face3d_rs::models::MorphableModel;
use nalgebra as na;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎭 Face3D-RS - Exemplo 3DMM\n");

    // 1. Criar modelo 3DMM simples (10 vértices, 5 componentes)
    let n_vertices = 10;
    let n_components = 5;
    let shape_dim = n_vertices * 3;

    // Mean shape: esfera centrada na origem
    let mean_shape = na::DVector::from_fn(shape_dim, |i, _| {
        let vertex_idx = i / 3;
        let coord_idx = i % 3;

        // Distribuir vértices em esfera
        let theta = 2.0 * std::f32::consts::PI * vertex_idx as f32 / n_vertices as f32;
        match coord_idx {
            0 => theta.cos(), // x
            1 => theta.sin(), // y
            _ => 0.0,         // z
        }
    });

    // Basis aleatórias (simplificado)
    let shape_basis = na::DMatrix::from_fn(shape_dim, n_components, |_, _| {
        (rand::random::<f32>() - 0.5) * 0.1
    });

    let texture_basis = na::DMatrix::from_fn(shape_dim, n_components, |_, _| {
        (rand::random::<f32>() - 0.5) * 0.05
    });

    // Mean texture: cinza
    let mean_texture = na::DVector::from_element(shape_dim, 0.5);

    // Topologia simples (sem faces neste exemplo)
    let faces = vec![];

    let model = MorphableModel::new(
        mean_shape,
        shape_basis,
        texture_basis,
        mean_texture,
        faces,
    );

    println!("✅ Modelo 3DMM criado:");
    println!("   - Vértices: {}", model.n_vertices);
    println!("   - Componentes shape: {}", model.n_shape_components());
    println!("   - Componentes texture: {}", model.n_texture_components());
    println!();

    // 2. Gerar faces variando parâmetros
    println!("🎨 Gerando faces com diferentes parâmetros:\n");

    for i in 0..3 {
        let alpha = i as f32 * 0.5;

        let shape_params = na::DVector::from_element(n_components, alpha);
        let texture_params = na::DVector::from_element(n_components, -alpha);

        let (shape, texture) = model.generate_face(&shape_params, &texture_params)?;

        println!("Face #{} (α={:.1}):", i + 1, alpha);

        // Mostrar primeiro vértice
        let v0 = model.get_vertex(&shape, 0)?;
        let c0_idx = 0;
        println!("  Vértice 0: ({:.3}, {:.3}, {:.3})", v0.x, v0.y, v0.z);
        println!("  Cor 0: ({:.3}, {:.3}, {:.3})",
            texture[c0_idx],
            texture[c0_idx + 1],
            texture[c0_idx + 2]
        );
        println!();
    }

    // 3. Converter para pontos 3D
    let shape_params = na::DVector::zeros(n_components);
    let texture_params = na::DVector::zeros(n_components);
    let (shape, _) = model.generate_face(&shape_params, &texture_params)?;

    let vertices = model.shape_to_vertices(&shape);

    println!("📊 Vértices extraídos: {} pontos", vertices.len());
    for (i, v) in vertices.iter().take(5).enumerate() {
        println!("  v{}: ({:.3}, {:.3}, {:.3})", i, v.x, v.y, v.z);
    }

    Ok(())
}
