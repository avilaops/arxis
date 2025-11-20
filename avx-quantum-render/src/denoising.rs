//! Sistema de denoising para imagens renderizadas

/// Filtro Gaussiano simples para denoising
pub fn gaussian_blur(image: &[Vec<f64>], kernel_size: usize) -> Vec<Vec<f64>> {
    if image.is_empty() || kernel_size == 0 {
        return image.to_vec();
    }

    let height = image.len();
    let width = image[0].len();
    let mut result = vec![vec![0.0; width]; height];

    let radius = kernel_size / 2;
    let sigma = (kernel_size as f64) / 6.0;
    let kernel = gaussian_kernel(kernel_size, sigma);

    for y in 0..height {
        for x in 0..width {
            let mut sum = 0.0;
            let mut weight_sum = 0.0;

            for ky in 0..kernel_size {
                for kx in 0..kernel_size {
                    let py = (y as isize + ky as isize - radius as isize).clamp(0, (height - 1) as isize) as usize;
                    let px = (x as isize + kx as isize - radius as isize).clamp(0, (width - 1) as isize) as usize;

                    let weight = kernel[ky][kx];
                    sum += image[py][px] * weight;
                    weight_sum += weight;
                }
            }

            result[y][x] = if weight_sum > 0.0 { sum / weight_sum } else { 0.0 };
        }
    }

    result
}

/// Gera kernel Gaussiano
fn gaussian_kernel(size: usize, sigma: f64) -> Vec<Vec<f64>> {
    let mut kernel = vec![vec![0.0; size]; size];
    let center = size / 2;
    let two_sigma_sq = 2.0 * sigma * sigma;

    for i in 0..size {
        for j in 0..size {
            let x = (i as isize - center as isize) as f64;
            let y = (j as isize - center as isize) as f64;
            kernel[i][j] = (-(x * x + y * y) / two_sigma_sq).exp();
        }
    }

    kernel
}

/// Filtro bilateral (preserva bordas)
pub fn bilateral_filter(
    image: &[Vec<f64>],
    spatial_sigma: f64,
    range_sigma: f64,
    kernel_size: usize,
) -> Vec<Vec<f64>> {
    if image.is_empty() {
        return image.to_vec();
    }

    let height = image.len();
    let width = image[0].len();
    let mut result = vec![vec![0.0; width]; height];
    let radius = kernel_size / 2;

    for y in 0..height {
        for x in 0..width {
            let center_value = image[y][x];
            let mut sum = 0.0;
            let mut weight_sum = 0.0;

            for ky in 0..kernel_size {
                for kx in 0..kernel_size {
                    let py = (y as isize + ky as isize - radius as isize).clamp(0, (height - 1) as isize) as usize;
                    let px = (x as isize + kx as isize - radius as isize).clamp(0, (width - 1) as isize) as usize;

                    let pixel_value = image[py][px];
                    
                    // Peso espacial (distância)
                    let dx = (x as isize - px as isize) as f64;
                    let dy = (y as isize - py as isize) as f64;
                    let spatial_weight = (-(dx * dx + dy * dy) / (2.0 * spatial_sigma * spatial_sigma)).exp();

                    // Peso de range (diferença de intensidade)
                    let value_diff = center_value - pixel_value;
                    let range_weight = (-(value_diff * value_diff) / (2.0 * range_sigma * range_sigma)).exp();

                    let weight = spatial_weight * range_weight;
                    sum += pixel_value * weight;
                    weight_sum += weight;
                }
            }

            result[y][x] = if weight_sum > 0.0 { sum / weight_sum } else { center_value };
        }
    }

    result
}

/// Filtro de mediana (remove salt & pepper noise)
pub fn median_filter(image: &[Vec<f64>], kernel_size: usize) -> Vec<Vec<f64>> {
    if image.is_empty() {
        return image.to_vec();
    }

    let height = image.len();
    let width = image[0].len();
    let mut result = vec![vec![0.0; width]; height];
    let radius = kernel_size / 2;

    for y in 0..height {
        for x in 0..width {
            let mut neighborhood = Vec::new();

            for ky in 0..kernel_size {
                for kx in 0..kernel_size {
                    let py = (y as isize + ky as isize - radius as isize).clamp(0, (height - 1) as isize) as usize;
                    let px = (x as isize + kx as isize - radius as isize).clamp(0, (width - 1) as isize) as usize;
                    neighborhood.push(image[py][px]);
                }
            }

            neighborhood.sort_by(|a, b| a.partial_cmp(b).unwrap());
            result[y][x] = neighborhood[neighborhood.len() / 2];
        }
    }

    result
}

/// Filtro de média simples
pub fn mean_filter(image: &[Vec<f64>], kernel_size: usize) -> Vec<Vec<f64>> {
    if image.is_empty() {
        return image.to_vec();
    }

    let height = image.len();
    let width = image[0].len();
    let mut result = vec![vec![0.0; width]; height];
    let radius = kernel_size / 2;

    for y in 0..height {
        for x in 0..width {
            let mut sum = 0.0;
            let mut count = 0;

            for ky in 0..kernel_size {
                for kx in 0..kernel_size {
                    let py = (y as isize + ky as isize - radius as isize).clamp(0, (height - 1) as isize) as usize;
                    let px = (x as isize + kx as isize - radius as isize).clamp(0, (width - 1) as isize) as usize;
                    sum += image[py][px];
                    count += 1;
                }
            }

            result[y][x] = if count > 0 { sum / count as f64 } else { 0.0 };
        }
    }

    result
}

/// Aplica múltiplas passadas de denoising
pub fn denoise_multipass(
    image: &[Vec<f64>],
    passes: usize,
    kernel_size: usize,
) -> Vec<Vec<f64>> {
    let mut result = image.to_vec();

    for _ in 0..passes {
        result = gaussian_blur(&result, kernel_size);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gaussian_blur() {
        let image = vec![vec![1.0, 0.0, 1.0], vec![0.0, 1.0, 0.0], vec![1.0, 0.0, 1.0]];

        let blurred = gaussian_blur(&image, 3);

        assert_eq!(blurred.len(), 3);
        assert_eq!(blurred[0].len(), 3);

        // Centro deve ser suavizado
        assert!(blurred[1][1] < 1.0);
        assert!(blurred[1][1] > 0.0);
    }

    #[test]
    fn test_median_filter() {
        let image = vec![vec![1.0, 1.0, 1.0], vec![1.0, 0.0, 1.0], vec![1.0, 1.0, 1.0]];

        let filtered = median_filter(&image, 3);

        // Centro deve ser corrigido para valor da mediana
        assert!(filtered[1][1] > 0.5);
    }

    #[test]
    fn test_mean_filter() {
        let image = vec![vec![1.0, 0.0], vec![0.0, 1.0]];

        let filtered = mean_filter(&image, 2);

        // Todos os valores devem estar próximos da média
        for row in filtered {
            for value in row {
                assert!((value - 0.5).abs() < 0.1);
            }
        }
    }

    #[test]
    fn test_bilateral_filter() {
        let image = vec![vec![1.0, 1.0, 0.0], vec![1.0, 1.0, 0.0], vec![0.0, 0.0, 0.0]];

        let filtered = bilateral_filter(&image, 1.0, 0.1, 3);

        // Deve preservar bordas melhor que Gaussian
        assert_eq!(filtered.len(), 3);
        assert_eq!(filtered[0].len(), 3);
    }
}
