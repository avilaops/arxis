//! Exemplo de uso da avila-linalg
//!
//! Demonstra operações básicas com vetores e matrizes

use avila_linalg::prelude::*;

fn main() {
    println!("=== Avila Linear Algebra - Exemplo de Uso ===\n");

    // Vetores 3D
    println!("📐 Vetores 3D:");
    let v1 = Vector3::new(1.0, 2.0, 3.0);
    let v2 = Vector3::new(4.0, 5.0, 6.0);

    println!("v1 = {:?}", v1);
    println!("v2 = {:?}", v2);

    let dot = v1.dot(&v2);
    println!("v1 · v2 = {}", dot);

    let cross = v1.cross(&v2);
    println!("v1 × v2 = {:?}", cross);

    let norm = v1.norm();
    println!("|v1| = {}", norm);

    let unit = v1.normalize();
    println!("v1 normalizado = {:?}", unit);
    println!();

    // Matrizes 3x3
    println!("🔢 Matrizes 3x3:");
    let identity = Matrix3x3::<f64>::identity();
    println!("Matriz identidade:");
    println!("{:?}", identity);

    let m = Matrix3x3::from_rows([
        [1.0, 2.0, 3.0],
        [4.0, 5.0, 6.0],
        [7.0, 8.0, 9.0],
    ]);
    println!("\nMatriz M:");
    println!("{:?}", m);

    let det = m.det();
    println!("det(M) = {}", det);

    let tr = m.trace();
    println!("tr(M) = {}", tr);

    let mt = m.transpose();
    println!("M^T = {:?}", mt);
    println!();

    // Matriz × Vetor
    println!("🎯 Transformações (Matriz × Vetor):");
    let rotation = Matrix3x3::from_rows([
        [0.0, -1.0, 0.0],  // Rotação 90° em torno de Z
        [1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0],
    ]);

    let point = Vector3::new(1.0, 0.0, 0.0);
    let rotated = rotation * point;

    println!("Ponto original: {:?}", point);
    println!("Após rotação 90°: {:?}", rotated);
    println!();

    // Operações genéricas
    println!("⚙️  Operações Genéricas:");
    let a = 0.0;
    let b = 10.0;
    let t = 0.5;
    let interpolated = lerp(a, b, t);
    println!("lerp({}, {}, {}) = {}", a, b, t, interpolated);

    let value = 15.0;
    let clamped = clamp(value, 0.0, 10.0);
    println!("clamp({}, 0, 10) = {}", value, clamped);
    println!();

    // Matriz dinâmica
    println!("📊 Matriz Dinâmica (MxN):");
    let mat = MatrixMxN::from_vec(
        2, 3,
        vec![
            1.0, 2.0, 3.0,
            4.0, 5.0, 6.0,
        ]
    );
    println!("Matriz 2×3:");
    for i in 0..mat.rows() {
        print!("[ ");
        for j in 0..mat.cols() {
            print!("{} ", mat.get(i, j));
        }
        println!("]");
    }

    let mat_t = mat.transpose();
    println!("\nTransposta 3×2:");
    for i in 0..mat_t.rows() {
        print!("[ ");
        for j in 0..mat_t.cols() {
            print!("{} ", mat_t.get(i, j));
        }
        println!("]");
    }

    println!("\n✅ Avila Linear Algebra - 100% Genuíno, 0 Bloat!");
}
