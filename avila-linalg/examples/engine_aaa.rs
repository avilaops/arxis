//! Exemplo: Transformações 3D/4D para Engines AAA
//!
//! Demonstra operações avançadas de álgebra linear:
//! - Quaternions (rotações sem gimbal lock)
//! - Matrizes 4×4 (transformações homogêneas)
//! - Projeção perspectiva
//! - Câmera look-at
//! - SLERP (interpolação esférica)

use avila_linalg::prelude::*;
use std::f64::consts::PI;

fn main() {
    println!("=== Avila Linear Algebra - Engine AAA Demo ===\n");

    // 1. QUATERNIONS - Rotações Avançadas
    println!("🔄 QUATERNIONS (Rotações sem Gimbal Lock):");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    // Rotação de 90° em Z usando quaternion
    let axis = Vector3::new(0.0, 0.0, 1.0);
    let angle = PI / 2.0;  // 90 graus
    let quat = Quaternion::from_axis_angle(axis, angle);

    println!("Quaternion (90° em Z): w={:.4}, x={:.4}, y={:.4}, z={:.4}",
             quat.w, quat.x, quat.y, quat.z);

    // Rotacionar vetor (1, 0, 0) → (0, 1, 0)
    let v = Vector3::new(1.0, 0.0, 0.0);
    let rotated = quat.rotate_vector(v);
    println!("Vetor (1,0,0) rotacionado: ({:.4}, {:.4}, {:.4})",
             rotated.x(), rotated.y(), rotated.z());

    // Converte para matriz de rotação
    let rot_matrix = quat.to_matrix3();
    println!("Matriz 3×3 equivalente:");
    for i in 0..3 {
        println!("  [{:.4}, {:.4}, {:.4}]",
                 rot_matrix.data()[i][0],
                 rot_matrix.data()[i][1],
                 rot_matrix.data()[i][2]);
    }
    println!();

    // 2. SLERP - Interpolação Esférica (animações suaves)
    println!("🎬 SLERP (Interpolação para Animações):");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    let q1 = Quaternion::<f64>::identity();  // Sem rotação
    let q2 = Quaternion::from_axis_angle(Vector3::new(0.0, 1.0, 0.0), PI / 2.0);  // 90° em Y

    println!("Interpolação de 0° → 90° em Y:");
    for t in [0.0, 0.25, 0.5, 0.75, 1.0] {
        let interpolated = q1.slerp(&q2, t);
        let test_vec = interpolated.rotate_vector(Vector3::new(1.0, 0.0, 0.0));
        println!("  t={:.2}: vetor = ({:.3}, {:.3}, {:.3})",
                 t, test_vec.x(), test_vec.y(), test_vec.z());
    }
    println!();

    // 3. TRANSFORMAÇÕES 4D - Coordenadas Homogêneas
    println!("📐 TRANSFORMAÇÕES 4D (Coordenadas Homogêneas):");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    // Translação
    let translation = Matrix4x4::translation(5.0, 3.0, 2.0);
    let point = Vector4::from_point(Vector3::new(0.0, 0.0, 0.0));
    let translated = translation * point;
    println!("Translação (5, 3, 2): {:?}", translated.to_vector3());

    // Escala
    let scale = Matrix4x4::scale_xyz(2.0, 3.0, 4.0);
    let scaled_point = Vector4::from_point(Vector3::new(1.0, 1.0, 1.0));
    let scaled = scale * scaled_point;
    println!("Escala (2x, 3y, 4z): {:?}", scaled.to_vector3());

    // Rotação em Y
    let rotation_y = Matrix4x4::rotation_y(PI / 4.0);  // 45°
    let rotated_4d = rotation_y * Vector4::from_point(Vector3::new(1.0, 0.0, 0.0));
    println!("Rotação 45° em Y: ({:.3}, {:.3}, {:.3})",
             rotated_4d.x(), rotated_4d.y(), rotated_4d.z());
    println!();

    // 4. COMPOSIÇÃO DE TRANSFORMAÇÕES (TRS)
    println!("🔗 COMPOSIÇÃO: Translate → Rotate → Scale:");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    let t = Matrix4x4::translation(0.0, 5.0, 0.0);
    let r = Matrix4x4::rotation_z(PI / 6.0);  // 30°
    let s = Matrix4x4::scale(2.0);

    // Ordem importa! T * R * S
    let trs = t * r * s;

    let local_point = Vector4::from_point(Vector3::new(1.0, 0.0, 0.0));
    let world_point = trs * local_point;
    println!("Ponto local (1,0,0) no mundo: ({:.3}, {:.3}, {:.3})",
             world_point.x(), world_point.y(), world_point.z());
    println!();

    // 5. CÂMERA - Look At Matrix
    println!("📹 CÂMERA (Look-At Matrix):");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    let eye = Vector3::new(0.0, 5.0, 10.0);      // Posição da câmera
    let target = Vector3::new(0.0, 0.0, 0.0);    // Olhando para origem
    let up = Vector3::new(0.0, 1.0, 0.0);        // Up vector

    let view_matrix = Matrix4x4::look_at(eye, target, up);

    println!("Câmera em (0, 5, 10) olhando para origem:");
    let world_pos = Vector4::from_point(Vector3::new(0.0, 0.0, 0.0));
    let view_space = view_matrix * world_pos;
    println!("  Origem no espaço da câmera: ({:.3}, {:.3}, {:.3})",
             view_space.x(), view_space.y(), view_space.z());
    println!();

    // 6. PROJEÇÃO PERSPECTIVA
    println!("🎥 PROJEÇÃO PERSPECTIVA:");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    let fovy = PI / 3.0;  // 60 graus FOV
    let aspect = 16.0 / 9.0;  // 16:9 aspect ratio
    let near = 0.1;
    let far = 100.0;

    let projection = Matrix4x4::perspective(fovy, aspect, near, far);

    println!("FOV: {:.1}°, Aspect: {:.2}, Near: {}, Far: {}",
             fovy * 180.0 / PI, aspect, near, far);

    // Projetar ponto 3D no plano da tela
    let point_3d = Vector4::from_point(Vector3::new(0.0, 0.0, -5.0));
    let view_space_point = view_matrix * point_3d;
    let clip_space = projection * view_space_point;

    // NDC (Normalized Device Coordinates) após perspective divide
    let ndc = Vector3::new(
        clip_space.x() / clip_space.w(),
        clip_space.y() / clip_space.w(),
        clip_space.z() / clip_space.w(),
    );

    println!("Ponto (0,0,-5) em NDC: ({:.3}, {:.3}, {:.3})", ndc.x(), ndc.y(), ndc.z());
    println!();

    // 7. PROJEÇÃO ORTOGRÁFICA (para UI/HUD)
    println!("📊 PROJEÇÃO ORTOGRÁFICA (UI/HUD):");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    let ortho = Matrix4x4::orthographic(
        -10.0, 10.0,  // left, right
        -10.0, 10.0,  // bottom, top
        -1.0, 1.0,    // near, far
    );

    let ui_point = Vector4::from_point(Vector3::new(5.0, 5.0, 0.0));
    let screen_point = ortho * ui_point;

    println!("Ponto UI (5, 5, 0):");
    println!("  Em NDC: ({:.3}, {:.3}, {:.3})",
             screen_point.x(), screen_point.y(), screen_point.z());
    println!();

    // 8. PIPELINE COMPLETO: World → View → Projection
    println!("🎮 PIPELINE COMPLETO (World → View → Projection):");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    // Objeto no mundo (cubo no chão)
    let model = Matrix4x4::translation(0.0, 0.0, -5.0) * Matrix4x4::scale(2.0);

    // Vértice local do cubo
    let local_vertex = Vector4::from_point(Vector3::new(1.0, 1.0, 1.0));

    // Transformações:
    let world_vertex = model * local_vertex;  // Model → World
    let view_vertex = view_matrix * world_vertex;  // World → View
    let clip_vertex = projection * view_vertex;  // View → Clip

    println!("Vértice (1,1,1) do cubo:");
    println!("  Local:     ({:.2}, {:.2}, {:.2})",
             local_vertex.x(), local_vertex.y(), local_vertex.z());
    println!("  World:     ({:.2}, {:.2}, {:.2})",
             world_vertex.x(), world_vertex.y(), world_vertex.z());
    println!("  View:      ({:.2}, {:.2}, {:.2})",
             view_vertex.x(), view_vertex.y(), view_vertex.z());
    println!("  Clip:      ({:.2}, {:.2}, {:.2}, {:.2})",
             clip_vertex.x(), clip_vertex.y(), clip_vertex.z(), clip_vertex.w());

    let ndc_vertex = Vector3::new(
        clip_vertex.x() / clip_vertex.w(),
        clip_vertex.y() / clip_vertex.w(),
        clip_vertex.z() / clip_vertex.w(),
    );
    println!("  NDC:       ({:.3}, {:.3}, {:.3})", ndc_vertex.x(), ndc_vertex.y(), ndc_vertex.z());
    println!();

    // 9. MÚLTIPLAS ROTAÇÕES (Evita Gimbal Lock com Quaternions)
    println!("⚙️  GIMBAL LOCK - Quaternions vs Euler:");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    // Euler angles: Roll, Pitch, Yaw
    let euler_quat = Quaternion::from_euler(PI / 6.0, PI / 4.0, PI / 3.0);
    println!("Euler (30°, 45°, 60°) → Quaternion:");
    println!("  w={:.4}, x={:.4}, y={:.4}, z={:.4}",
             euler_quat.w, euler_quat.x, euler_quat.y, euler_quat.z);

    // Composição de rotações
    let q_pitch = Quaternion::from_axis_angle(Vector3::new(1.0, 0.0, 0.0), PI / 4.0);
    let q_yaw = Quaternion::from_axis_angle(Vector3::new(0.0, 1.0, 0.0), PI / 3.0);
    let q_combined = q_yaw * q_pitch;  // Ordem importa!

    println!("Composição (Yaw × Pitch):");
    println!("  w={:.4}, x={:.4}, y={:.4}, z={:.4}",
             q_combined.w, q_combined.x, q_combined.y, q_combined.z);
    println!();

    println!("✅ Todas as operações de Engine AAA funcionando!");
    println!("📦 100% Avila, 0 Bloat, Performance Total!");
}
