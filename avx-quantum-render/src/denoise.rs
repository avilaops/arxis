//! # Denoising Filters for Quantum Rendered Images
//!
//! Implementa filtros de redução de ruído para imagens renderizadas com path tracing.
//!
//! ## Filtros Disponíveis
//!
//! - **Gaussian Blur**: Filtro gaussiano clássico para suavização
//! - **Bilateral Filter**: Preserva bordas enquanto remove ruído

/// Imagem representada como matriz de intensidades
pub type Image = Vec<Vec<f64>>;

/// Aplica filtro gaussiano para redução de ruído
///
/// # Argumentos
///
/// * `image` - Imagem de entrada (matriz de intensidades)
/// * `kernel_size` - Tamanho do kernel (deve ser ímpar)
///
/// # Retorna
///
/// Imagem com ruído reduzido
///
/// # Exemplo
///
/// ```ignore
/// use avx_quantum_render::denoise::gaussian_blur;
///
/// let noisy_image = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
/// let denoised = gaussian_blur(&noisy_image, 3);
/// ```
pub fn gaussian_blur(image: &Image, kernel_size: usize) -> Image {
    if kernel_size % 2 == 0 {
        panic!("Kernel size must be odd");
    }

    let height = image.len();
    if height == 0 {
        return vec![];
    }
    let width = image[0].len();
    if width == 0 {
        return vec![];
    }

    // Gerar kernel gaussiano
    let sigma = (kernel_size as f64) / 6.0; // Regra prática: 3σ ≈ kernel_size/2
    let kernel = generate_gaussian_kernel(kernel_size, sigma);

    // Aplicar convolução
    let mut output = vec![vec![0.0; width]; height];
    let radius = kernel_size / 2;

    for y in 0..height {
        for x in 0..width {
            let mut sum = 0.0;
            let mut weight_sum = 0.0;

            for ky in 0..kernel_size {
                for kx in 0..kernel_size {
                    let iy = (y as i32) + (ky as i32) - (radius as i32);
                    let ix = (x as i32) + (kx as i32) - (radius as i32);

                    if iy >= 0 && iy < (height as i32) && ix >= 0 && ix < (width as i32) {
                        let weight = kernel[ky][kx];
                        sum += image[iy as usize][ix as usize] * weight;
                        weight_sum += weight;
                    }
                }
            }

            output[y][x] = if weight_sum > 0.0 {
                sum / weight_sum
            } else {
                image[y][x]
            };
        }
    }

    output
}

/// Aplica filtro bilateral que preserva bordas
///
/// O filtro bilateral usa pesos baseados em:
/// 1. Distância espacial (como gaussiano)
/// 2. Diferença de intensidade (preserva bordas)
///
/// # Argumentos
///
/// * `image` - Imagem de entrada
/// * `sigma_spatial` - Desvio padrão espacial (controla suavização)
/// * `sigma_range` - Desvio padrão de intensidade (controla preservação de bordas)
/// * `kernel_size` - Tamanho do kernel (deve ser ímpar)
///
/// # Retorna
///
/// Imagem com ruído reduzido e bordas preservadas
///
/// # Exemplo
///
/// ```ignore
/// use avx_quantum_render::denoise::bilateral_filter;
///
/// let noisy_image = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
/// let denoised = bilateral_filter(&noisy_image, 2.0, 0.1, 5);
/// ```
pub fn bilateral_filter(
    image: &Image,
    sigma_spatial: f64,
    sigma_range: f64,
    kernel_size: usize,
) -> Image {
    if kernel_size % 2 == 0 {
        panic!("Kernel size must be odd");
    }

    let height = image.len();
    if height == 0 {
        return vec![];
    }
    let width = image[0].len();
    if width == 0 {
        return vec![];
    }

    let mut output = vec![vec![0.0; width]; height];
    let radius = kernel_size / 2;

    for y in 0..height {
        for x in 0..width {
            let center_intensity = image[y][x];
            let mut sum = 0.0;
            let mut weight_sum = 0.0;

            for ky in 0..kernel_size {
                for kx in 0..kernel_size {
                    let iy = (y as i32) + (ky as i32) - (radius as i32);
                    let ix = (x as i32) + (kx as i32) - (radius as i32);

                    if iy >= 0 && iy < (height as i32) && ix >= 0 && ix < (width as i32) {
                        let neighbor_intensity = image[iy as usize][ix as usize];

                        // Peso espacial (distância)
                        let dy = (ky as f64) - (radius as f64);
                        let dx = (kx as f64) - (radius as f64);
                        let spatial_dist = (dx * dx + dy * dy).sqrt();
                        let spatial_weight = (-spatial_dist * spatial_dist
                            / (2.0 * sigma_spatial * sigma_spatial))
                            .exp();

                        // Peso de intensidade (preserva bordas)
                        let intensity_diff = neighbor_intensity - center_intensity;
                        let range_weight = (-intensity_diff * intensity_diff
                            / (2.0 * sigma_range * sigma_range))
                            .exp();

                        let weight = spatial_weight * range_weight;
                        sum += neighbor_intensity * weight;
                        weight_sum += weight;
                    }
                }
            }

            output[y][x] = if weight_sum > 0.0 {
                sum / weight_sum
            } else {
                image[y][x]
            };
        }
    }

    output
}

/// Gera kernel gaussiano 2D
fn generate_gaussian_kernel(size: usize, sigma: f64) -> Vec<Vec<f64>> {
    let mut kernel = vec![vec![0.0; size]; size];
    let center = size / 2;
    let mut sum = 0.0;

    for y in 0..size {
        for x in 0..size {
            let dy = (y as f64) - (center as f64);
            let dx = (x as f64) - (center as f64);
            let dist_sq = dx * dx + dy * dy;
            let value = (-dist_sq / (2.0 * sigma * sigma)).exp();
            kernel[y][x] = value;
            sum += value;
        }
    }

    // Normalizar kernel
    for y in 0..size {
        for x in 0..size {
            kernel[y][x] /= sum;
        }
    }

    kernel
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gaussian_kernel_generation() {
        let kernel = generate_gaussian_kernel(3, 1.0);
        assert_eq!(kernel.len(), 3);
        assert_eq!(kernel[0].len(), 3);

        // Verificar que a soma é aproximadamente 1.0 (normalizado)
        let sum: f64 = kernel.iter().flat_map(|row| row.iter()).sum();
        assert!((sum - 1.0).abs() < 1e-10);

        // Centro deve ter maior valor
        assert!(kernel[1][1] > kernel[0][0]);
        assert!(kernel[1][1] > kernel[2][2]);
    }

    #[test]
    fn test_gaussian_blur_identity() {
        let image = vec![
            vec![1.0, 1.0, 1.0],
            vec![1.0, 1.0, 1.0],
            vec![1.0, 1.0, 1.0],
        ];

        let blurred = gaussian_blur(&image, 3);

        // Imagem uniforme deve permanecer uniforme
        for y in 0..3 {
            for x in 0..3 {
                assert!((blurred[y][x] - 1.0).abs() < 1e-6);
            }
        }
    }

    #[test]
    fn test_gaussian_blur_smoothing() {
        let image = vec![
            vec![0.0, 0.0, 0.0],
            vec![0.0, 1.0, 0.0],
            vec![0.0, 0.0, 0.0],
        ];

        let blurred = gaussian_blur(&image, 3);

        // O pico deve ser suavizado
        assert!(blurred[1][1] < 1.0);
        assert!(blurred[1][1] > 0.0);

        // Vizinhos devem ter valores não-zero
        assert!(blurred[0][1] > 0.0);
        assert!(blurred[1][0] > 0.0);
    }

    #[test]
    fn test_bilateral_filter_preserves_edges() {
        // Criar imagem com borda forte (0.0 | 1.0)
        let image = vec![
            vec![0.0, 0.0, 1.0, 1.0],
            vec![0.0, 0.0, 1.0, 1.0],
            vec![0.0, 0.0, 1.0, 1.0],
        ];

        let filtered = bilateral_filter(&image, 2.0, 0.1, 3);

        // Borda deve ser preservada
        // Pixels escuros devem permanecer próximos de 0.0
        assert!(filtered[1][0] < 0.3);
        assert!(filtered[1][1] < 0.3);

        // Pixels claros devem permanecer próximos de 1.0
        assert!(filtered[1][2] > 0.7);
        assert!(filtered[1][3] > 0.7);
    }

    #[test]
    #[should_panic(expected = "Kernel size must be odd")]
    fn test_gaussian_blur_even_kernel_panics() {
        let image = vec![vec![1.0]];
        let _ = gaussian_blur(&image, 4); // Deve falhar
    }

    #[test]
    fn test_empty_image() {
        let image: Image = vec![];
        let blurred = gaussian_blur(&image, 3);
        assert_eq!(blurred.len(), 0);
    }
}
