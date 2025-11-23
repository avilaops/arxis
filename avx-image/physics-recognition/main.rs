//! # Sistema de Reconhecimento Facial - Física e Matemática
//!
//! Demonstra os fundamentos físicos e matemáticos do reconhecimento facial:
//!
//! ## Física:
//! - Formação de imagem (óptica geométrica)
//! - Modelo de reflectância (equação de renderização)
//! - Irradiância e captura de luz
//!
//! ## Matemática:
//! - PCA (Principal Component Analysis) - Eigenfaces
//! - LDA (Linear Discriminant Analysis)
//! - Transformadas de Fourier (Gabor wavelets)
//! - Geometria diferencial (normais de superfície)
//! - Álgebra linear (projeções, distâncias)

mod optics;
mod geometry;
mod features;
mod recognition;

use image::{DynamicImage, GrayImage};
use ndarray::Array2;

fn main() {
    println!("=== Sistema de Reconhecimento Facial ===\n");

    // 1. FÍSICA: Formação da Imagem
    println!("1. FÍSICA - Formação da Imagem");
    demo_image_formation();

    // 2. GEOMETRIA: Análise de superfície
    println!("\n2. GEOMETRIA - Análise de Superfície Facial");
    demo_surface_geometry();

    // 3. FEATURES: Extração de características
    println!("\n3. MATEMÁTICA - Extração de Features");
    demo_feature_extraction();

    // 4. RECONHECIMENTO: Matching e identificação
    println!("\n4. RECONHECIMENTO - Sistema Completo");
    demo_recognition_system();
}

fn demo_image_formation() {
    use optics::*;

    println!("  Simulando captura de imagem por câmera...");

    let camera = Camera::new(
        1920, 1080,  // Resolução
        50.0,        // Distância focal (mm)
        35.0,        // Sensor size (mm)
    );

    // Ponto facial no espaço 3D
    let face_point = FacePoint {
        position: nalgebra::Point3::new(0.0, 0.0, 500.0), // 50cm da câmera
        normal: nalgebra::Vector3::new(0.0, 0.0, -1.0),   // Normal apontando pra câmera
        albedo: 0.65,  // Albedo da pele
        color: nalgebra::Vector3::new(0.85, 0.65, 0.55),  // Tom de pele
    };

    // Luz ambiente
    let light = Light {
        direction: nalgebra::Vector3::new(0.3, -0.5, -0.8).normalize(),
        intensity: 1.0,
        color: nalgebra::Vector3::new(1.0, 1.0, 1.0),
    };

    // Projeta ponto 3D -> 2D
    let pixel = camera.project_point(&face_point.position);
    let irradiance = calculate_irradiance(&face_point, &light);
    let pixel_color = calculate_pixel_color(&face_point, &[light]);

    println!("  ✓ Posição 3D: ({:.1}, {:.1}, {:.1}) mm",
             face_point.position.x, face_point.position.y, face_point.position.z);
    println!("  ✓ Projeção 2D: pixel ({:.0}, {:.0})", pixel.x, pixel.y);
    println!("  ✓ Irradiância: {:.3} W/m²", irradiance);
    println!("  ✓ Cor RGB: ({:.2}, {:.2}, {:.2})",
             pixel_color.x, pixel_color.y, pixel_color.z);

    // Equação de reflectância
    println!("\n  Equação de Reflectância (Lambert):");
    println!("  I(x,y) = ρ × (n · l) × E");
    println!("  onde:");
    println!("    ρ = {:.2} (albedo)", face_point.albedo);
    println!("    n · l = {:.3} (produto escalar normal-luz)",
             face_point.normal.dot(&light.direction).max(0.0));
    println!("    E = {:.1} (irradiância)", light.intensity);
}

fn demo_surface_geometry() {
    use geometry::*;

    println!("  Analisando geometria 3D da face...");

    // Pontos característicos (landmarks)
    let nose_tip = nalgebra::Point3::new(0.0, 20.0, 450.0);
    let left_eye = nalgebra::Point3::new(-30.0, 40.0, 440.0);
    let right_eye = nalgebra::Point3::new(30.0, 40.0, 440.0);
    let mouth_center = nalgebra::Point3::new(0.0, -10.0, 455.0);

    // Distâncias euclidianas
    let eye_distance = euclidean_distance(&left_eye, &right_eye);
    let nose_to_mouth = euclidean_distance(&nose_tip, &mouth_center);

    println!("  ✓ Distância entre olhos: {:.1} mm", eye_distance);
    println!("  ✓ Distância nariz-boca: {:.1} mm", nose_to_mouth);

    // Calcula normal da superfície (nariz)
    let neighbor1 = nalgebra::Point3::new(-5.0, 20.0, 448.0);
    let neighbor2 = nalgebra::Point3::new(5.0, 20.0, 448.0);
    let normal = compute_surface_normal(&nose_tip, &neighbor1, &neighbor2);

    println!("  ✓ Normal do nariz: ({:.3}, {:.3}, {:.3})",
             normal.x, normal.y, normal.z);

    // Curvatura (simplificada)
    let curvature = estimate_curvature(&nose_tip, &neighbor1, &neighbor2);
    println!("  ✓ Curvatura gaussiana: {:.6}", curvature);
}

fn demo_feature_extraction() {
    use features::*;

    println!("  Extraindo features matemáticas...");

    // Cria uma imagem sintética 64x64
    let face_image = create_synthetic_face(64, 64);

    // 1. Histograma de gradientes orientados (HOG)
    let hog_features = compute_hog_features(&face_image, 8);
    println!("  ✓ HOG features: {} dimensões", hog_features.len());

    // 2. Local Binary Patterns (LBP)
    let lbp_histogram = compute_lbp_histogram(&face_image);
    println!("  ✓ LBP histogram: {} bins", lbp_histogram.len());

    // 3. Gabor wavelets (análise de frequência)
    let gabor_response = compute_gabor_response(&face_image, 0.1, 0.0);
    println!("  ✓ Gabor wavelets: {} coeficientes", gabor_response.len());

    println!("\n  Matemática das Features:");
    println!("  • HOG: ∇I(x,y) = [∂I/∂x, ∂I/∂y]ᵀ");
    println!("  • LBP: LBP(x,y) = Σ s(gₚ - gᶜ)2ᵖ");
    println!("  • Gabor: G(x,y) = exp(-x'²+γ²y'²/2σ²) cos(2πx'/λ)");
}

fn demo_recognition_system() {
    use recognition::*;

    println!("  Construindo sistema de reconhecimento...");

    // Cria dataset sintético
    let mut recognizer = FaceRecognizer::new();

    // Adiciona rostos ao banco
    for person_id in 0..5 {
        for sample in 0..3 {
            let face = features::create_synthetic_face_for_person(64, 64, person_id);
            recognizer.add_face(person_id, face);
        }
    }

    println!("  ✓ Database: {} pessoas, {} amostras", 5, 15);

    // Treina modelo (PCA - Eigenfaces)
    recognizer.train_pca(20); // 20 componentes principais
    println!("  ✓ PCA treinado: 20 eigenfaces");

    // Testa reconhecimento
    let test_face = features::create_synthetic_face_for_person(64, 64, 2);
    let (predicted_id, confidence) = recognizer.recognize(&test_face);

    println!("  ✓ Pessoa identificada: {}", predicted_id);
    println!("  ✓ Confiança: {:.1}%", confidence * 100.0);

    println!("\n  Matemática do Reconhecimento:");
    println!("  • PCA: X' = UᵀX (projeção em eigenfaces)");
    println!("  • Distância: d = ||X'₁ - X'₂||₂");
    println!("  • Decisão: argmin d(X', database)");
}
