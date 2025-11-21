//! Exemplo: Usando FLAME (articulado com expressões)
//!
//! Demonstra como usar o modelo FLAME com parâmetros de
//! shape, expression e pose.

use face3d_rs::models::flame::FlameBuilder;
use nalgebra as na;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔥 Face3D-RS - Exemplo FLAME\n");

    // 1. Criar modelo FLAME simplificado
    let model = FlameBuilder::new()
        .n_vertices(100)
        .n_shape_components(10)
        .n_expression_components(5)
        .n_joints(3)
        .build_empty();

    println!("✅ Modelo FLAME criado:");
    println!("   - Vértices: {}", model.n_vertices);
    println!("   - Shape components: {}", model.n_shape_components());
    println!("   - Expression components: {}", model.n_expression_components());
    println!("   - Juntas: {}", model.n_joints());
    println!();

    // 2. Parâmetros neutros
    println!("😐 Face neutra (todos parâmetros = 0):\n");

    let shape_params = na::DVector::zeros(model.n_shape_components());
    let expr_params = na::DVector::zeros(model.n_expression_components());
    let pose_params = na::DVector::zeros(model.n_joints() * 3);

    let vertices = model.forward(&shape_params, &expr_params, &pose_params)?;

    println!("  Shape: {} × {}", vertices.nrows(), vertices.ncols());
    println!("  Primeiro vértice: ({:.3}, {:.3}, {:.3})",
        vertices[(0, 0)], vertices[(0, 1)], vertices[(0, 2)]
    );
    println!();

    // 3. Variando expressões
    println!("😊 Testando diferentes expressões:\n");

    let expressions = vec![
        ("Neutra", vec![0.0, 0.0, 0.0, 0.0, 0.0]),
        ("Sorriso", vec![1.0, 0.0, 0.0, 0.0, 0.0]),
        ("Surpresa", vec![0.0, 1.5, 0.0, 0.0, 0.0]),
        ("Raiva", vec![0.0, 0.0, -1.0, 0.0, 0.0]),
    ];

    for (name, coeffs) in expressions {
        let expr_params = na::DVector::from_vec(coeffs);
        let vertices = model.forward(&shape_params, &expr_params, &pose_params)?;

        println!("  {}: v0 = ({:.3}, {:.3}, {:.3})",
            name,
            vertices[(0, 0)],
            vertices[(0, 1)],
            vertices[(0, 2)]
        );
    }
    println!();

    // 4. Variando pose (rotação de cabeça)
    println!("🔄 Testando rotação de cabeça:\n");

    let poses = vec![
        ("Frente", vec![0.0, 0.0, 0.0]),
        ("Direita", vec![0.0, 0.5, 0.0]),
        ("Esquerda", vec![0.0, -0.5, 0.0]),
        ("Cima", vec![0.3, 0.0, 0.0]),
    ];

    for (name, rotation) in poses {
        // Primeira junta = rotação global da cabeça
        let mut pose_params = na::DVector::zeros(model.n_joints() * 3);
        pose_params[0] = rotation[0];
        pose_params[1] = rotation[1];
        pose_params[2] = rotation[2];

        let vertices = model.forward(&shape_params, &expr_params, &pose_params)?;

        println!("  {}: v0 = ({:.3}, {:.3}, {:.3})",
            name,
            vertices[(0, 0)],
            vertices[(0, 1)],
            vertices[(0, 2)]
        );
    }
    println!();

    // 5. Combinando tudo
    println!("🎭 Combinação: Sorriso + Rotação:\n");

    let shape_params = na::DVector::from_element(model.n_shape_components(), 0.2);
    let expr_params = na::DVector::from_vec(vec![1.5, 0.5, 0.0, 0.0, 0.0]);
    let mut pose_params = na::DVector::zeros(model.n_joints() * 3);
    pose_params[1] = 0.3; // Rotação Y

    let vertices = model.forward(&shape_params, &expr_params, &pose_params)?;
    let points = model.vertices_to_points(&vertices);

    println!("  Pontos gerados: {}", points.len());
    println!("  Primeiros 3 vértices:");
    for (i, p) in points.iter().take(3).enumerate() {
        println!("    v{}: ({:.3}, {:.3}, {:.3})", i, p.x, p.y, p.z);
    }

    Ok(())
}
