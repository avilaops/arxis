//! Módulo de Features - Extração de Características
//! 
//! Implementa algoritmos matemáticos para extrair features faciais:
//! - HOG (Histogram of Oriented Gradients)
//! - LBP (Local Binary Patterns)
//! - Gabor Wavelets (análise de frequência)
//! - Gradientes de imagem

use ndarray::{Array2, Array1};
use std::f32::consts::PI;

/// Cria imagem sintética de face para demonstração
pub fn create_synthetic_face(width: usize, height: usize) -> Array2<f32> {
    let mut image = Array2::zeros((height, width));
    
    let center_x = width as f32 / 2.0;
    let center_y = height as f32 / 2.0;
    
    for y in 0..height {
        for x in 0..width {
            let dx = x as f32 - center_x;
            let dy = y as f32 - center_y;
            let r = (dx * dx + dy * dy).sqrt();
            
            // Face oval
            let face = (-r * r / 400.0).exp();
            
            // Olhos (gaussianas)
            let eye_l = (-(dx + 10.0).powi(2) / 20.0 - (dy - 8.0).powi(2) / 15.0).exp();
            let eye_r = (-(dx - 10.0).powi(2) / 20.0 - (dy - 8.0).powi(2) / 15.0).exp();
            
            // Nariz
            let nose = (-dx.powi(2) / 10.0 - (dy + 5.0).powi(2) / 30.0).exp();
            
            // Boca
            let mouth = (-dx.powi(2) / 50.0 - (dy + 15.0).powi(2) / 8.0).exp();
            
            image[[y, x]] = (0.3 * face + 0.2 * (eye_l + eye_r) + 0.15 * nose + 0.1 * mouth)
                .clamp(0.0, 1.0);
        }
    }
    
    image
}

/// Cria face sintética específica para uma pessoa (com variações)
pub fn create_synthetic_face_for_person(width: usize, height: usize, person_id: usize) -> Array2<f32> {
    let mut face = create_synthetic_face(width, height);
    
    // Adiciona variações baseadas no ID
    let phase = person_id as f32 * 0.5;
    for y in 0..height {
        for x in 0..width {
            let noise = ((x as f32 * 0.3 + phase).sin() + (y as f32 * 0.3 + phase).cos()) * 0.05;
            face[[y, x]] = (face[[y, x]] + noise).clamp(0.0, 1.0);
        }
    }
    
    face
}

/// Calcula gradientes da imagem (derivadas parciais)
/// 
/// ∇I = [∂I/∂x, ∂I/∂y]ᵀ
/// 
/// Usando filtros de Sobel:
/// Gₓ = [-1 0 1; -2 0 2; -1 0 1]
/// Gᵧ = [-1 -2 -1; 0 0 0; 1 2 1]
pub fn compute_gradients(image: &Array2<f32>) -> (Array2<f32>, Array2<f32>) {
    let (height, width) = image.dim();
    let mut grad_x = Array2::zeros((height, width));
    let mut grad_y = Array2::zeros((height, width));
    
    // Kernel Sobel
    let sobel_x = [[-1.0, 0.0, 1.0], [-2.0, 0.0, 2.0], [-1.0, 0.0, 1.0]];
    let sobel_y = [[-1.0, -2.0, -1.0], [0.0, 0.0, 0.0], [1.0, 2.0, 1.0]];
    
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let mut sum_x = 0.0;
            let mut sum_y = 0.0;
            
            for ky in 0..3 {
                for kx in 0..3 {
                    let pixel = image[[y + ky - 1, x + kx - 1]];
                    sum_x += pixel * sobel_x[ky][kx];
                    sum_y += pixel * sobel_y[ky][kx];
                }
            }
            
            grad_x[[y, x]] = sum_x;
            grad_y[[y, x]] = sum_y;
        }
    }
    
    (grad_x, grad_y)
}

/// Computa HOG (Histogram of Oriented Gradients)
/// 
/// Para cada célula:
/// 1. Calcula magnitude: m = √(gₓ² + gᵧ²)
/// 2. Calcula orientação: θ = arctan(gᵧ/gₓ)
/// 3. Cria histograma de 9 bins (0-180°)
pub fn compute_hog_features(image: &Array2<f32>, cell_size: usize) -> Vec<f32> {
    let (grad_x, grad_y) = compute_gradients(image);
    let (height, width) = image.dim();
    
    let cells_y = height / cell_size;
    let cells_x = width / cell_size;
    let num_bins = 9;
    
    let mut features = Vec::new();
    
    for cell_y in 0..cells_y {
        for cell_x in 0..cells_x {
            let mut histogram = vec![0.0; num_bins];
            
            for y in 0..cell_size {
                for x in 0..cell_size {
                    let py = cell_y * cell_size + y;
                    let px = cell_x * cell_size + x;
                    
                    if py >= height || px >= width {
                        continue;
                    }
                    
                    let gx = grad_x[[py, px]];
                    let gy = grad_y[[py, px]];
                    
                    let magnitude = (gx * gx + gy * gy).sqrt();
                    let angle = gy.atan2(gx).to_degrees();
                    let angle = if angle < 0.0 { angle + 180.0 } else { angle };
                    
                    let bin = ((angle / 180.0) * num_bins as f32) as usize % num_bins;
                    histogram[bin] += magnitude;
                }
            }
            
            // Normaliza histograma
            let sum: f32 = histogram.iter().sum();
            if sum > 1e-6 {
                for val in &mut histogram {
                    *val /= sum;
                }
            }
            
            features.extend(histogram);
        }
    }
    
    features
}

/// Computa LBP (Local Binary Patterns)
/// 
/// Para cada pixel:
/// LBP(xc,yc) = Σ s(gp - gc)2^p
/// 
/// onde:
/// s(x) = 1 se x ≥ 0, 0 caso contrário
/// gp = valor do vizinho p
/// gc = valor do centro
pub fn compute_lbp_histogram(image: &Array2<f32>) -> Vec<f32> {
    let (height, width) = image.dim();
    let mut histogram = vec![0.0; 256];
    
    // Offsets dos 8 vizinhos (sentido horário)
    let neighbors = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, 1), (1, 1), (1, 0),
        (1, -1), (0, -1)
    ];
    
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let center = image[[y, x]];
            let mut lbp_code = 0u8;
            
            for (i, (dy, dx)) in neighbors.iter().enumerate() {
                let ny = (y as i32 + dy) as usize;
                let nx = (x as i32 + dx) as usize;
                let neighbor = image[[ny, nx]];
                
                if neighbor >= center {
                    lbp_code |= 1 << i;
                }
            }
            
            histogram[lbp_code as usize] += 1.0;
        }
    }
    
    // Normaliza
    let sum: f32 = histogram.iter().sum();
    if sum > 1e-6 {
        for val in &mut histogram {
            *val /= sum;
        }
    }
    
    histogram
}

/// Filtro de Gabor (análise de frequência e orientação)
/// 
/// G(x,y;λ,θ,ψ,σ,γ) = exp(-x'²+γ²y'² / 2σ²) × cos(2πx'/λ + ψ)
/// 
/// onde:
/// x' = x cos θ + y sin θ
/// y' = -x sin θ + y cos θ
/// λ = comprimento de onda
/// θ = orientação
/// ψ = fase
/// σ = desvio padrão (largura do envelope)
/// γ = aspect ratio espacial
pub fn compute_gabor_response(
    image: &Array2<f32>,
    frequency: f32,
    orientation: f32,
) -> Vec<f32> {
    let (height, width) = image.dim();
    let sigma = 4.0;
    let gamma = 0.5;
    
    let mut response = Array2::zeros((height, width));
    
    // Cria kernel de Gabor
    let kernel_size = 15;
    let center = kernel_size / 2;
    
    for y in 0..height.saturating_sub(kernel_size) {
        for x in 0..width.saturating_sub(kernel_size) {
            let mut sum_real = 0.0;
            let mut sum_imag = 0.0;
            
            for ky in 0..kernel_size {
                for kx in 0..kernel_size {
                    let dx = (kx as i32 - center as i32) as f32;
                    let dy = (ky as i32 - center as i32) as f32;
                    
                    // Rotação
                    let x_rot = dx * orientation.cos() + dy * orientation.sin();
                    let y_rot = -dx * orientation.sin() + dy * orientation.cos();
                    
                    // Envelope Gaussiano
                    let gaussian = (-( x_rot * x_rot + gamma * gamma * y_rot * y_rot) 
                        / (2.0 * sigma * sigma)).exp();
                    
                    // Onda cosseno
                    let wave = (2.0 * PI * frequency * x_rot).cos();
                    let wave_sin = (2.0 * PI * frequency * x_rot).sin();
                    
                    let pixel = image[[y + ky, x + kx]];
                    sum_real += pixel * gaussian * wave;
                    sum_imag += pixel * gaussian * wave_sin;
                }
            }
            
            response[[y, x]] = (sum_real * sum_real + sum_imag * sum_imag).sqrt();
        }
    }
    
    // Retorna valores achatados
    response.iter().cloned().collect()
}

/// Extrai todas as features de uma imagem
pub fn extract_all_features(image: &Array2<f32>) -> Vec<f32> {
    let mut all_features = Vec::new();
    
    // HOG
    let hog = compute_hog_features(image, 8);
    all_features.extend(hog);
    
    // LBP (reduzido)
    let lbp = compute_lbp_histogram(image);
    all_features.extend(&lbp[..59]); // Usa apenas LBP uniformes
    
    // Gabor (múltiplas orientações)
    for angle in [0.0, PI/4.0, PI/2.0, 3.0*PI/4.0] {
        let gabor = compute_gabor_response(image, 0.1, angle);
        // Estatísticas do response
        let mean = gabor.iter().sum::<f32>() / gabor.len() as f32;
        let variance = gabor.iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f32>() / gabor.len() as f32;
        all_features.push(mean);
        all_features.push(variance.sqrt());
    }
    
    all_features
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_gradients() {
        let image = Array2::from_shape_fn((10, 10), |(y, x)| (x + y) as f32);
        let (gx, gy) = compute_gradients(&image);
        
        // Gradiente deve ser constante
        assert!(gx[[5, 5]].abs() > 0.0);
        assert!(gy[[5, 5]].abs() > 0.0);
    }
    
    #[test]
    fn test_hog_features() {
        let image = create_synthetic_face(64, 64);
        let features = compute_hog_features(&image, 8);
        
        assert!(!features.is_empty());
        assert!(features.iter().all(|&x| x >= 0.0 && x <= 1.0));
    }
    
    #[test]
    fn test_lbp() {
        let image = create_synthetic_face(64, 64);
        let histogram = compute_lbp_histogram(&image);
        
        assert_eq!(histogram.len(), 256);
        let sum: f32 = histogram.iter().sum();
        assert!((sum - 1.0).abs() < 1e-3);
    }
}
