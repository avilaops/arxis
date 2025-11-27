//! Exemplo: Basel Face Model (BFM)
//!
//! Demonstra como usar o BFM com shape, color e expression.

use face3d_rs::models::bfm::BfmBuilder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧬 Face3D-RS - Exemplo Basel Face Model\n");

    // 1. Criar modelo BFM simplificado
    let model = BfmBuilder::new()
        .n_vertices(50)
        .n_shape_components(10)
        .n_color_components(10)
        .n_expression_components(5)
        .build_empty();

    println!("✅ Modelo BFM criado:");
    println!("   - Vértices: {}", model.n_vertices);
    println!("   - Shape components: {}", model.n_shape_components());
    println!("   - Color components: {}", model.n_color_components());
    println!("   - Expression components: {}", model.n_expression_components());
    println!();

    // 2. Face média (todos coeficientes = 0)
    println!("😐 Face média (coeficientes neutros):\n");

    let shape_coeffs = vec![0.0; 10];
    let color_coeffs = vec![0.0; 10];
    let expr_coeffs = vec![0.0; 5];

    let (vertices, colors) = model.generate(&shape_coeffs, &color_coeffs, &expr_coeffs)?;

    println!("  Vértices gerados: {}", vertices.len());
    println!("  Cores geradas: {}", colors.len());
    println!("  Primeiro vértice: ({:.3}, {:.3}, {:.3})",
        vertices[0].x, vertices[0].y, vertices[0].z
    );
    println!("  Primeira cor: RGB({:.3}, {:.3}, {:.3})",
        colors[0].x, colors[0].y, colors[0].z
    );
    println!();

    // 3. Variações de identidade (shape)
    println!("👤 Variações de identidade:\n");

    let identities = vec![
        ("Pessoa A", vec![2.0, 1.0, -0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]),
        ("Pessoa B", vec![-1.5, 0.0, 2.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]),
        ("Pessoa C", vec![0.0, -2.0, 0.0, -1.0, 1.5, 0.0, 0.0, 0.0, 0.0, 0.0]),
    ];

    for (name, coeffs) in identities {
        let (vertices, _) = model.generate(&coeffs, &color_coeffs, &expr_coeffs)?;
        println!("  {}: v0 = ({:.3}, {:.3}, {:.3})",
            name, vertices[0].x, vertices[0].y, vertices[0].z
        );
    }
    println!();

    // 4. Variações de cor
    println!("🎨 Variações de cor/textura:\n");

    let skin_tones = vec![
        ("Tom claro", vec![1.5, 0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]),
        ("Tom médio", vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]),
        ("Tom escuro", vec![-1.5, -0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]),
    ];

    for (name, coeffs) in skin_tones {
        let (_, colors) = model.generate(&shape_coeffs, &coeffs, &expr_coeffs)?;
        println!("  {}: RGB({:.3}, {:.3}, {:.3})",
            name, colors[0].x, colors[0].y, colors[0].z
        );
    }
    println!();

    // 5. Landmarks
    println!("📍 Testando landmarks:\n");

    let mut model_with_landmarks = model;

    model_with_landmarks.add_landmark("nose_tip".to_string(), 25)?;
    model_with_landmarks.add_landmark("left_eye".to_string(), 10)?;
    model_with_landmarks.add_landmark("right_eye".to_string(), 15)?;
    model_with_landmarks.add_landmark("mouth_center".to_string(), 30)?;

    let (vertices, _) = model_with_landmarks.generate(&shape_coeffs, &color_coeffs, &expr_coeffs)?;

    let landmarks = vec!["nose_tip", "left_eye", "right_eye", "mouth_center"];
    for name in landmarks {
        if let Some(pos) = model_with_landmarks.get_landmark_position(&vertices, name) {
            println!("  {}: ({:.3}, {:.3}, {:.3})", name, pos.x, pos.y, pos.z);
        }
    }
    println!();

    // 6. Combinação: identidade + expressão + cor
    println!("🎭 Combinação completa:\n");

    let shape_coeffs = vec![1.0, -0.5, 0.8, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    let color_coeffs = vec![0.5, 0.3, -0.2, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    let expr_coeffs = vec![1.5, 0.8, 0.0, 0.0, 0.0]; // Sorriso

    let (vertices, colors) = model_with_landmarks.generate(
        &shape_coeffs,
        &color_coeffs,
        &expr_coeffs
    )?;

    println!("  Gerado: {} vértices, {} cores", vertices.len(), colors.len());
    println!("  Amostra de 5 vértices:");
    for i in 0..5 {
        println!("    v{}: ({:.3}, {:.3}, {:.3}) | RGB({:.3}, {:.3}, {:.3})",
            i,
            vertices[i].x, vertices[i].y, vertices[i].z,
            colors[i].x, colors[i].y, colors[i].z
        );
    }

    Ok(())
}
