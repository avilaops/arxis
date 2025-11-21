//! Exemplo demonstrativo de avila-convexa3d
//!
//! Biblioteca para processamento 3D:
//! - Vídeo (temporal): sequências de frames
//! - Volume (espacial): dados 3D como medical imaging
//! - Filtros: convolução 3D, blur, edge detection
//! - Motion: optical flow, motion detection

use avila_convexa3d::common::{Axis3D, Point3D, Size3D};
use avila_convexa3d::filters::Filter3D;
use avila_convexa3d::motion::{MotionDetector, OpticalFlow};
use avila_convexa3d::video::VideoProcessor;
use avila_convexa3d::volume::VolumeProcessor;

fn main() {
    println!("=== avila-convexa3d: Processamento 3D ===\n");

    // ===== Parte 1: Processamento de Vídeo (Temporal) =====
    println!("📹 Parte 1: Processamento de Vídeo");
    println!("-----------------------------------");

    // Criar vídeo de teste
    let video = VideoProcessor::create_test_video(100, 100, 30, 30.0);
    println!("✓ Vídeo criado: {}x{} pixels, {} frames, {:.1} fps",
        video.width, video.height, video.num_frames(), video.fps);
    println!("  Duração: {:.2} segundos", video.duration());

    // Estatísticas temporais
    let mean = VideoProcessor::temporal_mean(&video);
    let energy = VideoProcessor::temporal_energy(&video);
    println!("✓ Média temporal (primeiro pixel): {:.2}", mean[[0, 0, 0]]);
    println!("✓ Energia temporal: {:.2}", energy);

    // Converter para escala de cinza
    let gray_video = VideoProcessor::to_grayscale(&video);
    println!("✓ Vídeo convertido para grayscale: {} canal(is)", gray_video.channels);

    // Sub-vídeo
    let subvid = video.subvideo(10, 20);
    println!("✓ Sub-vídeo extraído: frames 10-20 ({} frames)", subvid.num_frames());

    println!();

    // ===== Parte 2: Optical Flow e Motion Detection =====
    println!("🎬 Parte 2: Optical Flow e Detecção de Movimento");
    println!("------------------------------------------------");

    if let (Some(f1), Some(f2)) = (video.get_frame(5), video.get_frame(6)) {
        // Block matching
        let flow_bm = OpticalFlow::block_matching(f1, f2, 10, 5);
        println!("✓ Optical flow (block matching): {}x{} blocos",
            flow_bm.width, flow_bm.height);
        println!("  Magnitude média: {:.4}", flow_bm.average_magnitude());

        // Gradient-based
        let flow_grad = OpticalFlow::gradient_based(f1, f2);
        println!("✓ Optical flow (gradient-based): {}x{} pixels",
            flow_grad.width, flow_grad.height);
        println!("  Magnitude média: {:.4}", flow_grad.average_magnitude());

        // Motion detection
        let motion_mask = MotionDetector::frame_difference_threshold(f1, f2, 30.0);
        let motion_pixels: usize = motion_mask.iter().filter(|&&x| x).count();
        println!("✓ Máscara de movimento: {} pixels em movimento", motion_pixels);
    }

    // Temporal motion energy
    let motion_energy = MotionDetector::temporal_motion_energy(&video, 0, video.num_frames());
    println!("✓ Energia de movimento temporal: {:.2}", motion_energy);

    // Detect motion frames
    let motion_frames = MotionDetector::detect_motion_frames(&video, 10.0);
    println!("✓ Frames com movimento significativo: {} frames", motion_frames.len());

    println!();

    // ===== Parte 3: Volumes 3D (Espacial) =====
    println!("🧊 Parte 3: Volumes 3D");
    println!("----------------------");

    // Criar volume de teste
    let volume = VolumeProcessor::create_test_volume(50, 50, 50);
    let size = volume.size();
    println!("✓ Volume criado: {}x{}x{}", size.width, size.height, size.depth);
    println!("  Volume total: {} voxels", size.volume());

    // Estatísticas espaciais
    let mean = VolumeProcessor::spatial_mean(&volume, 0);
    let std = VolumeProcessor::spatial_std(&volume, 0, mean);
    println!("✓ Média espacial: {:.4}", mean);
    println!("✓ Desvio padrão: {:.4}", std);

    // Criar esfera
    let sphere = VolumeProcessor::create_sphere(10);
    println!("✓ Esfera criada: raio 10 ({}x{}x{})",
        sphere.size().width, sphere.size().height, sphere.size().depth);
    println!("  Centro: {:.1}, Canto: {:.1}",
        sphere.get(10, 10, 10, 0), sphere.get(0, 0, 0, 0));

    // Slicing
    let slice_xy = volume.slice_at(Axis3D::Z, 25, 0);
    println!("✓ Slice XY no plano z=25: {}x{}",
        slice_xy.shape()[1], slice_xy.shape()[0]);

    // Threshold
    let binary = VolumeProcessor::threshold(&volume, 0.5);
    let ones: f32 = binary.data.iter().filter(|&&x| x == 1.0).count() as f32;
    let total = binary.data.len() as f32;
    println!("✓ Threshold aplicado: {:.1}% voxels = 1", (ones / total) * 100.0);

    // Downsampling
    let downsampled = VolumeProcessor::downsample(&volume, 2);
    println!("✓ Downsampling 2x: {}x{}x{} → {}x{}x{}",
        size.width, size.height, size.depth,
        downsampled.size().width, downsampled.size().height, downsampled.size().depth);

    // Gradient magnitude
    let gradient = VolumeProcessor::gradient_magnitude(&volume, 0);
    let grad_mean = VolumeProcessor::spatial_mean(&gradient, 0);
    println!("✓ Gradiente calculado, magnitude média: {:.4}", grad_mean);

    println!();

    // ===== Parte 4: Filtros e Convolução 3D =====
    println!("🔧 Parte 4: Filtros 3D");
    println!("----------------------");

    // Mean filter
    let _mean_filtered = Filter3D::mean_filter(&sphere, 0);
    println!("✓ Filtro de média 3x3x3 aplicado");

    // Gaussian blur
    let blurred = Filter3D::gaussian_blur(&sphere, 0);
    let blur_mean = VolumeProcessor::spatial_mean(&blurred, 0);
    println!("✓ Gaussian blur aplicado, média: {:.4}", blur_mean);

    // Laplacian (edge detection)
    let edges = Filter3D::laplacian(&sphere, 0);
    let edge_std = VolumeProcessor::spatial_std(&edges, 0, 0.0);
    println!("✓ Laplaciano (bordas) aplicado, std: {:.4}", edge_std);

    // Sobel X
    let sobel = Filter3D::sobel_x(&sphere, 0);
    let sobel_mean = VolumeProcessor::spatial_mean(&sobel, 0);
    println!("✓ Sobel X aplicado, média: {:.4}", sobel_mean);

    println!();

    // ===== Parte 5: Geometria 3D =====
    println!("📐 Parte 5: Geometria 3D");
    println!("------------------------");

    let p1 = Point3D::new(0, 0, 0);
    let p2 = Point3D::new(3, 4, 0);
    println!("✓ Ponto 1: {:?}", p1);
    println!("✓ Ponto 2: {:?}", p2);
    println!("  Distância euclidiana: {:.2}", p1.distance(&p2));
    println!("  Distância Manhattan: {}", p1.manhattan_distance(&p2));

    let size = Size3D::new(100, 200, 300);
    println!("✓ Tamanho: {:?}", size);
    println!("  Volume: {} voxels", size.volume());
    println!("  Área XY: {} pixels", size.area_xy());

    println!();

    // ===== Resumo Final =====
    println!("✅ Demonstração completa!");
    println!("   - Processamento de vídeo (temporal)");
    println!("   - Optical flow e detecção de movimento");
    println!("   - Volumes 3D (espacial)");
    println!("   - Filtros e convolução 3D");
    println!("   - Geometria 3D");
}
