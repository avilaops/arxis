//! Exemplo: Projeção 3D → 2D
//!
//! Demonstra como projetar pontos 3D em imagens 2D usando
//! diferentes modelos de câmera.

use face3d_rs::models::MorphableModel;
use face3d_rs::utils::projection::{
    PerspectiveCamera, WeakPerspectiveCamera,
    perspective_projection, weak_perspective_projection,
};
use nalgebra as na;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📷 Face3D-RS - Exemplo de Projeção 3D → 2D\n");

    // 1. Criar alguns pontos 3D de teste
    let points_3d = vec![
        na::Point3::new(0.0, 0.0, 1.0),    // Centro
        na::Point3::new(0.1, 0.0, 1.0),    // Direita
        na::Point3::new(-0.1, 0.0, 1.0),   // Esquerda
        na::Point3::new(0.0, 0.1, 1.0),    // Cima
        na::Point3::new(0.0, -0.1, 1.0),   // Baixo
    ];

    println!("📊 Pontos 3D originais:");
    for (i, p) in points_3d.iter().enumerate() {
        println!("  p{}: ({:.2}, {:.2}, {:.2})", i, p.x, p.y, p.z);
    }
    println!();

    // 2. Projeção perspectiva
    println!("🎯 Projeção Perspectiva (focal=1000px, 640×480):\n");

    let persp_camera = PerspectiveCamera::new(1000.0, 640, 480);
    let projected_2d = perspective_projection(&points_3d, &persp_camera);

    println!("  Matriz intrínseca K:");
    let k = persp_camera.intrinsic_matrix();
    println!("    [{:.1}  {:.1}  {:.1}]", k[(0, 0)], k[(0, 1)], k[(0, 2)]);
    println!("    [{:.1}  {:.1}  {:.1}]", k[(1, 0)], k[(1, 1)], k[(1, 2)]);
    println!("    [{:.1}  {:.1}  {:.1}]", k[(2, 0)], k[(2, 1)], k[(2, 2)]);
    println!();

    println!("  Pontos 2D projetados:");
    for (i, p) in projected_2d.iter().enumerate() {
        println!("    p{}: ({:.1}, {:.1}) px", i, p.x, p.y);
    }
    println!();

    // 3. Weak perspective
    println!("🔲 Projeção Weak Perspective (scale=100, center):\n");

    let weak_camera = WeakPerspectiveCamera::new(100.0, 320.0, 240.0, 640, 480);
    let projected_weak = weak_perspective_projection(&points_3d, &weak_camera);

    println!("  Pontos 2D projetados:");
    for (i, p) in projected_weak.iter().enumerate() {
        println!("    p{}: ({:.1}, {:.1}) px", i, p.x, p.y);
    }
    println!();

    // 4. Projeção de uma face 3DMM
    println!("😀 Projetando face 3DMM:\n");

    let n_vertices = 10;
    let model = create_simple_face_model(n_vertices);

    let shape_params = na::DVector::zeros(5);
    let texture_params = na::DVector::zeros(5);
    let (shape, _) = model.generate_face(&shape_params, &texture_params)?;

    // Converter para pontos 3D
    let face_vertices = model.shape_to_vertices(&shape);

    // Projetar todos os vértices
    let face_2d = perspective_projection(&face_vertices, &persp_camera);

    println!("  Face com {} vértices projetada", face_vertices.len());
    println!("  Primeiros 5 vértices 2D:");
    for (i, p) in face_2d.iter().take(5).enumerate() {
        println!("    v{}: ({:.1}, {:.1}) px", i, p.x, p.y);
    }
    println!();

    // 5. Diferentes distâncias (efeito perspectiva)
    println!("🔍 Efeito de distância na projeção perspectiva:\n");

    let point = na::Point3::new(0.1, 0.1, 1.0);

    for distance in &[0.5, 1.0, 2.0, 5.0] {
        let p = na::Point3::new(
            point.x,
            point.y,
            *distance,
        );

        let projected = perspective_projection(&[p], &persp_camera);
        println!("  Distância {:.1}m → ({:.1}, {:.1}) px",
            distance, projected[0].x, projected[0].y
        );
    }
    println!();

    // 6. Comparação perspectiva vs weak perspective
    println!("⚖️  Comparação Perspectiva vs Weak Perspective:\n");

    let test_points = vec![
        na::Point3::new(0.0, 0.0, 1.0),
        na::Point3::new(0.2, 0.2, 1.0),
        na::Point3::new(0.2, 0.2, 2.0), // Mesmo ponto, mais longe
    ];

    println!("  Perspectiva:");
    let proj_persp = perspective_projection(&test_points, &persp_camera);
    for (i, p) in proj_persp.iter().enumerate() {
        println!("    p{}: ({:.1}, {:.1})", i, p.x, p.y);
    }

    println!("\n  Weak Perspective:");
    let proj_weak = weak_perspective_projection(&test_points, &weak_camera);
    for (i, p) in proj_weak.iter().enumerate() {
        println!("    p{}: ({:.1}, {:.1})", i, p.x, p.y);
    }

    Ok(())
}

fn create_simple_face_model(n_vertices: usize) -> MorphableModel {
    let shape_dim = n_vertices * 3;
    let n_components = 5;

    let mean_shape = na::DVector::from_fn(shape_dim, |i, _| {
        let vertex_idx = i / 3;
        let coord_idx = i % 3;

        let theta = 2.0 * std::f32::consts::PI * vertex_idx as f32 / n_vertices as f32;
        match coord_idx {
            0 => theta.cos() * 0.1,
            1 => theta.sin() * 0.1,
            _ => 1.0, // z = 1m de distância
        }
    });

    MorphableModel::new(
        mean_shape,
        na::DMatrix::zeros(shape_dim, n_components),
        na::DMatrix::zeros(shape_dim, n_components),
        na::DVector::zeros(shape_dim),
        vec![],
    )
}
