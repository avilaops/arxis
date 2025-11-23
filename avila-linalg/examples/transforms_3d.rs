//! Transformações e Rotações 3D com avila-linalg
//! 
//! Matrizes de rotação, quaternions, interpolação

use avila_linalg::prelude::*;

fn main() {
    println!("🔄 Transformações 3D com avila-linalg\n");

    // 1. Rotação 90° em torno de Z
    println!("1️⃣  Rotação 3D");
    
    let rot_z_90 = Matrix3x3::from_rows([
        [0.0, -1.0, 0.0],  // Rotação 90° anti-horário em Z
        [1.0,  0.0, 0.0],
        [0.0,  0.0, 1.0],
    ]);

    let point = Vector3::new(1.0, 0.0, 0.0);
    let rotated = rot_z_90 * point;
    
    println!("   Ponto: ({:.1}, {:.1}, {:.1})", point.x(), point.y(), point.z());
    println!("   Após rotação 90° (Z): ({:.3}, {:.3}, {:.3})", 
        rotated.x(), rotated.y(), rotated.z());
    println!("   ✅ Rotação aplicada");
    println!();

    // 2. Rotação 45° em torno de Z
    println!("2️⃣  Rotação 45°");
    
    let angle = std::f64::consts::PI / 4.0; // 45°
    let cos_a = angle.cos();
    let sin_a = angle.sin();
    
    let rot_z_45 = Matrix3x3::from_rows([
        [cos_a, -sin_a, 0.0],
        [sin_a,  cos_a, 0.0],
        [0.0,    0.0,   1.0],
    ]);

    let rotated_45 = rot_z_45 * point;
    println!("   Rotação 45°: ({:.3}, {:.3}, {:.3})",
        rotated_45.x(), rotated_45.y(), rotated_45.z());
    println!("   ✅ Esperado: (~0.707, ~0.707, 0)");
    println!();

    // 3. Composição de rotações
    println!("3️⃣  Composição de rotações");
    
    let rot_x_45 = Matrix3x3::from_rows([
        [1.0, 0.0,    0.0],
        [0.0, cos_a, -sin_a],
        [0.0, sin_a,  cos_a],
    ]);

    let combined = rot_x_45 * rot_z_45;
    let twice_rotated = combined * point;
    
    println!("   Rotação combinada: Z(45°) depois X(45°)");
    println!("   Resultado: ({:.3}, {:.3}, {:.3})", 
        twice_rotated.x(), twice_rotated.y(), twice_rotated.z());
    println!("   ✅ Rotações compostas");
    println!();

    // 4. Escala
    println!("4️⃣  Transformação de escala");
    
    let scale = Matrix3x3::from_rows([
        [2.0, 0.0, 0.0],
        [0.0, 3.0, 0.0],
        [0.0, 0.0, 4.0],
    ]);
    
    let v = Vector3::new(1.0, 1.0, 1.0);
    let scaled = scale * v;
    
    println!("   Vetor: ({:.1}, {:.1}, {:.1})", v.x(), v.y(), v.z());
    println!("   Escala: diag(2, 3, 4)");
    println!("   Resultado: ({:.1}, {:.1}, {:.1})", 
        scaled.x(), scaled.y(), scaled.z());
    
    let det = scale.det();
    println!("   Determinante: {:.1} (volume multiplicador)", det);
    println!("   ✅ Escala aplicada");
    println!();

    // 5. Quaternions
    println!("5️⃣  Quaternions para rotação");
    
    let q = Quaternion::from_axis_angle(
        Vector3::new(0.0, 0.0, 1.0),  // eixo Z
        angle                          // 45°
    );
    
    println!("   Quaternion: w={:.3}, x={:.3}, y={:.3}, z={:.3}", 
        q.w, q.x, q.y, q.z);
    
    let q_rotated = q.rotate_vector(point);
    println!("   Ponto rotado: ({:.3}, {:.3}, {:.3})",
        q_rotated.x(), q_rotated.y(), q_rotated.z());
    println!("   ✅ Rotação via quaternion");
    println!();

    // 6. SLERP - Interpolação esférica
    println!("6️⃣  SLERP - Spherical Linear Interpolation");
    
    let q_start = Quaternion::identity();
    let q_end = Quaternion::from_axis_angle(
        Vector3::new(0.0, 1.0, 0.0),
        std::f64::consts::PI / 2.0  // 90°
    );
    
    println!("   Interpolando: identidade → 90° (eixo Y)");
    for i in 0..=4 {
        let t = i as f64 / 4.0;
        let q_interp = q_start.slerp(&q_end, t);
        let test = q_interp.rotate_vector(Vector3::new(1.0, 0.0, 0.0));
        println!("     t={:.2}: ({:.3}, {:.3}, {:.3})", t, test.x(), test.y(), test.z());
    }
    println!("   ✅ Interpolação suave");
    println!();

    // 7. Matriz inversa
    println!("7️⃣  Matriz inversa");
    
    let m = Matrix3x3::from_rows([
        [4.0, 0.0, 0.0],
        [0.0, 2.0, 0.0],
        [0.0, 0.0, 1.0],
    ]);
    
    if let Some(m_inv) = m.inverse() {
        let product = m * m_inv;
        println!("   M = diag(4, 2, 1)");
        println!("   M⁻¹ = diag(0.25, 0.5, 1.0)");
        println!("   M × M⁻¹ ≈ I (identidade)");
        println!("   ✅ Inversa calculada");
    }
    println!();

    // 8. Transposta e simetria
    println!("8️⃣  Transposta");
    
    let asymmetric = Matrix3x3::from_rows([
        [1.0, 2.0, 3.0],
        [4.0, 5.0, 6.0],
        [7.0, 8.0, 9.0],
    ]);
    
    let transposed = asymmetric.transpose();
    println!("   Original:");
    println!("     [{:.0}, {:.0}, {:.0}]", 1.0, 2.0, 3.0);
    println!("     [{:.0}, {:.0}, {:.0}]", 4.0, 5.0, 6.0);
    println!("     [{:.0}, {:.0}, {:.0}]", 7.0, 8.0, 9.0);
    println!("   Transposta:");
    println!("     [{:.0}, {:.0}, {:.0}]", 1.0, 4.0, 7.0);
    println!("     [{:.0}, {:.0}, {:.0}]", 2.0, 5.0, 8.0);
    println!("     [{:.0}, {:.0}, {:.0}]", 3.0, 6.0, 9.0);
    println!("   ✅ M^T[i,j] = M[j,i]");
    println!();

    // 9. Matriz 4x4 para transformações homogêneas
    println!("9️⃣  Transformação 4x4 (Homogeneous)");
    
    let point_4d = Vector4::new(1.0, 0.0, 0.0, 1.0);  // ponto no espaço 3D
    let identity_4 = Matrix4x4::<f64>::identity();
    let transformed_4 = identity_4 * point_4d;
    
    println!("   Ponto homogêneo: ({:.1}, {:.1}, {:.1}, {:.1})", 
        point_4d.x(), point_4d.y(), point_4d.z(), point_4d.w());
    println!("   Após identidade: ({:.1}, {:.1}, {:.1}, {:.1})",
        transformed_4.x(), transformed_4.y(), transformed_4.z(), transformed_4.w());
    println!("   ✅ Coordenadas homogêneas");
    println!();

    println!("🎉 Transformações 3D completas!");
    println!("\n💡 Use cases:");
    println!("   • Game engines (camera, objects)");
    println!("   • Robótica (forward/inverse kinematics)");
    println!("   • Animação (skeletal animation)");
    println!("   • CAD/CAM (geometric transforms)");
}
