//! Exemplo demonstrativo de avila-convexa4d
//!
//! Biblioteca para processamento 4D:
//! - Tensores 4D (tempo × profundidade × altura × largura)
//! - Sequências de volumes ao longo do tempo
//! - Filtros espaço-temporais
//! - Análise de movimento volumétrico

use avila_convexa4d::common::{Point4D, Size4D};
use avila_convexa4d::filters::{ConvolutionKernel4D, Filter4D};
use avila_convexa4d::processor::{MotionAnalyzer, SpatioTemporalProcessor};
use avila_convexa4d::sequence::VolumeSequenceProcessor;
use avila_convexa4d::tensor::TensorOps;

fn main() {
    println!("=== avila-convexa4d: Processamento 4D ===\n");

    // ===== Parte 1: Tensores 4D =====
    println!("🎯 Parte 1: Tensores 4D (Espaço-Temporais)");
    println!("------------------------------------------");

    // Criar tensor de teste
    let tensor = TensorOps::create_test_tensor(10, 15, 20, 25);
    let size = tensor.size();
    println!(
        "✓ Tensor criado: {}t × {}d × {}h × {}w",
        size.time, size.depth, size.height, size.width
    );
    println!("  Hipervolume: {} elementos", size.hypervolume());

    // Estatísticas
    let mean = TensorOps::mean(&tensor, 0);
    let std = TensorOps::std(&tensor, 0, mean);
    println!("✓ Média: {:.4}", mean);
    println!("✓ Desvio padrão: {:.4}", std);

    // Médias temporal e espacial
    let _temporal_mean = TensorOps::temporal_mean(&tensor);
    let _spatial_mean = TensorOps::spatial_mean(&tensor);
    println!("✓ Média temporal calculada (colapsa tempo)");
    println!("✓ Média espacial calculada (colapsa espaço)");

    // Threshold
    let binary = TensorOps::threshold(&tensor, 0.5);
    let ones = binary
        .data
        .iter()
        .filter(|&&x| x == 1.0)
        .count() as f32;
    let total = binary.data.len() as f32;
    println!(
        "✓ Threshold 0.5: {:.1}% elementos = 1",
        (ones / total) * 100.0
    );

    // Downsampling
    let downsampled = TensorOps::downsample(&tensor, 2);
    let new_size = downsampled.size();
    println!(
        "✓ Downsampling 2x: {}×{}×{}×{} → {}×{}×{}×{}",
        size.time,
        size.depth,
        size.height,
        size.width,
        new_size.time,
        new_size.depth,
        new_size.height,
        new_size.width
    );

    println!();

    // ===== Parte 2: Sequências de Volumes =====
    println!("📹 Parte 2: Sequências de Volumes");
    println!("----------------------------------");

    // Criar sequência volumétrica
    let sequence = VolumeSequenceProcessor::create_test_sequence(8, 10, 10, 10);
    println!(
        "✓ Sequência criada: {} frames volumétricos",
        sequence.num_frames()
    );
    println!(
        "  Dimensões: {}d × {}h × {}w",
        sequence.depth, sequence.height, sequence.width
    );
    println!("  Duração: {:.2} segundos", sequence.duration());

    // Média temporal
    let seq_mean = VolumeSequenceProcessor::temporal_mean(&sequence);
    println!("✓ Média temporal calculada: {:?}", seq_mean.shape());

    // Energia espaço-temporal
    let energy = VolumeSequenceProcessor::spatiotemporal_energy(&sequence);
    println!("✓ Energia espaço-temporal: {:.2}", energy);

    // Converter para tensor 4D
    let seq_tensor = sequence.to_tensor();
    println!(
        "✓ Convertido para tensor 4D: {:?}",
        seq_tensor.shape()
    );

    println!();

    // ===== Parte 3: Filtros Espaço-Temporais =====
    println!("🔧 Parte 3: Filtros Espaço-Temporais 4D");
    println!("---------------------------------------");

    let test_tensor = TensorOps::create_test_tensor(7, 7, 7, 7);

    // Filtro de média 4D
    let _mean_filtered = Filter4D::mean_filter(&test_tensor, 0);
    println!("✓ Filtro de média 3×3×3×3 aplicado");

    // Gaussian blur 4D
    let blurred = Filter4D::gaussian_blur(&test_tensor, 0);
    let blur_mean = TensorOps::mean(&blurred, 0);
    println!("✓ Gaussian blur 4D aplicado, média: {:.4}", blur_mean);

    // Laplaciano 4D (bordas espaço-temporais)
    let edges = Filter4D::laplacian(&test_tensor, 0);
    let edge_std = TensorOps::std(&edges, 0, 0.0);
    println!("✓ Laplaciano 4D aplicado, std: {:.4}", edge_std);

    // Kernels
    let kernel_mean = ConvolutionKernel4D::mean_3x3x3x3();
    let kernel_gauss = ConvolutionKernel4D::gaussian_3x3x3x3();
    println!("✓ Kernel média: tamanho {:?}", kernel_mean.size());
    println!("✓ Kernel Gaussiano: tamanho {:?}", kernel_gauss.size());

    println!();

    // ===== Parte 4: Processamento Espaço-Temporal =====
    println!("⚡ Parte 4: Análise Espaço-Temporal");
    println!("----------------------------------");

    // Correlação temporal
    let corr_lag1 = SpatioTemporalProcessor::temporal_correlation(&tensor, 1, 0);
    let corr_lag2 = SpatioTemporalProcessor::temporal_correlation(&tensor, 2, 0);
    println!("✓ Correlação temporal (lag=1): {:.4}", corr_lag1);
    println!("✓ Correlação temporal (lag=2): {:.4}", corr_lag2);

    // Derivadas
    let _dt = SpatioTemporalProcessor::temporal_derivative(&tensor);
    let _dx = SpatioTemporalProcessor::spatial_derivative(&tensor, 2);
    println!("✓ Derivada temporal calculada");
    println!("✓ Derivada espacial (X) calculada");

    // Magnitude do gradiente 4D
    let grad_mag = SpatioTemporalProcessor::gradient_magnitude_4d(&tensor);
    let grad_mean = TensorOps::mean(&grad_mag, 0);
    println!("✓ Magnitude do gradiente 4D: {:.4}", grad_mean);

    println!();

    // ===== Parte 5: Análise de Movimento Volumétrico =====
    println!("🎬 Parte 5: Análise de Movimento Volumétrico");
    println!("--------------------------------------------");

    // Detectar movimento
    let motion_frames = MotionAnalyzer::detect_volumetric_motion(&sequence, 10.0);
    let motion_rate = MotionAnalyzer::motion_rate(&motion_frames);
    println!(
        "✓ Frames com movimento: {} de {}",
        motion_frames.iter().filter(|&&x| x).count(),
        motion_frames.len()
    );
    println!("✓ Taxa de movimento: {:.1}%", motion_rate * 100.0);

    // Detectar transições
    let transitions = MotionAnalyzer::detect_transitions(&sequence, 100.0);
    println!("✓ Transições detectadas: {} momentos", transitions.len());
    if !transitions.is_empty() {
        println!("  Frames: {:?}", transitions);
    }

    println!();

    // ===== Parte 6: Geometria 4D =====
    println!("📐 Parte 6: Geometria 4D");
    println!("------------------------");

    let p1 = Point4D::new(0, 0, 0, 0);
    let p2 = Point4D::new(1, 1, 1, 1);
    println!("✓ Ponto 1: {:?}", p1);
    println!("✓ Ponto 2: {:?}", p2);
    println!("  Distância euclidiana 4D: {:.2}", p1.distance(&p2));
    println!("  Distância Manhattan 4D: {}", p1.manhattan_distance(&p2));

    let size4d = Size4D::new(10, 20, 30, 40);
    println!("✓ Tamanho 4D: {:?}", size4d);
    println!("  Hipervolume: {}", size4d.hypervolume());
    println!("  Volume espacial: {}", size4d.spatial_volume());

    println!();

    // ===== Resumo Final =====
    println!("✅ Demonstração completa!");
    println!("   - Tensores 4D espaço-temporais");
    println!("   - Sequências de volumes");
    println!("   - Filtros 4D (média, Gaussiano, Laplaciano)");
    println!("   - Derivadas temporais e espaciais");
    println!("   - Análise de movimento volumétrico");
    println!("   - Geometria 4D");
}
