//! Operações de convolução e filtros 4D espaço-temporais

use ndarray::Array5;

use crate::tensor::Tensor4D;

/// Kernel de convolução 4D
#[derive(Debug, Clone)]
pub struct ConvolutionKernel4D {
    /// Dados do kernel (time × depth × height × width × channels)
    pub data: Array5<f32>,
}

impl ConvolutionKernel4D {
    /// Cria novo kernel 4D
    pub fn new(data: Array5<f32>) -> Self {
        Self { data }
    }

    /// Retorna tamanho do kernel
    pub fn size(&self) -> (usize, usize, usize, usize) {
        (
            self.data.shape()[0],
            self.data.shape()[1],
            self.data.shape()[2],
            self.data.shape()[3],
        )
    }

    /// Kernel de média 3×3×3×3
    pub fn mean_3x3x3x3() -> Self {
        let value = 1.0 / 81.0; // 3^4 = 81
        Self {
            data: Array5::from_elem((3, 3, 3, 3, 1), value),
        }
    }

    /// Kernel Gaussiano 3×3×3×3 simplificado
    pub fn gaussian_3x3x3x3() -> Self {
        let mut data = Array5::zeros((3, 3, 3, 3, 1));

        // Centro tem maior peso
        data[[1, 1, 1, 1, 0]] = 0.125;

        // Vizinhos diretos (faces)
        for t in 0..3 {
            for z in 0..3 {
                for y in 0..3 {
                    for x in 0..3 {
                        if (t == 1) as i32 + (z == 1) as i32 + (y == 1) as i32 + (x == 1) as i32
                            == 3
                        {
                            data[[t, z, y, x, 0]] = 0.0625;
                        }
                    }
                }
            }
        }

        // Normalizar
        let sum: f32 = data.iter().sum();
        if sum > 0.0 {
            data.mapv_inplace(|v| v / sum);
        }

        Self { data }
    }

    /// Kernel Laplaciano 4D (detecção de bordas espaço-temporais)
    pub fn laplacian_4d() -> Self {
        let mut data = Array5::zeros((3, 3, 3, 3, 1));

        // Centro negativo
        data[[1, 1, 1, 1, 0]] = -80.0;

        // 8 vizinhos diretos (faces do hipercubo)
        data[[0, 1, 1, 1, 0]] = 1.0;
        data[[2, 1, 1, 1, 0]] = 1.0;
        data[[1, 0, 1, 1, 0]] = 1.0;
        data[[1, 2, 1, 1, 0]] = 1.0;
        data[[1, 1, 0, 1, 0]] = 1.0;
        data[[1, 1, 2, 1, 0]] = 1.0;
        data[[1, 1, 1, 0, 0]] = 1.0;
        data[[1, 1, 1, 2, 0]] = 1.0;

        // Demais vizinhos com peso menor
        for t in 0..3 {
            for z in 0..3 {
                for y in 0..3 {
                    for x in 0..3 {
                        if data[[t, z, y, x, 0]] == 0.0 && !(t == 1 && z == 1 && y == 1 && x == 1)
                        {
                            data[[t, z, y, x, 0]] = 0.5;
                        }
                    }
                }
            }
        }

        Self { data }
    }
}

/// Filtro 4D espaço-temporal
pub struct Filter4D;

impl Filter4D {
    /// Aplica convolução 4D
    pub fn convolve(tensor: &Tensor4D, kernel: &ConvolutionKernel4D, channel: usize) -> Tensor4D {
        let (vt, vd, vh, vw, vc) = tensor.shape();
        let (kt, kd, kh, kw) = kernel.size();

        let pad_t = kt / 2;
        let pad_d = kd / 2;
        let pad_h = kh / 2;
        let pad_w = kw / 2;

        let mut result = Tensor4D::zeros(vt, vd, vh, vw, vc);

        for t in pad_t..(vt - pad_t) {
            for z in pad_d..(vd - pad_d) {
                for y in pad_h..(vh - pad_h) {
                    for x in pad_w..(vw - pad_w) {
                        let mut sum = 0.0;

                        for kt_i in 0..kt {
                            for kz in 0..kd {
                                for ky in 0..kh {
                                    for kx in 0..kw {
                                        let vol_t = t + kt_i - pad_t;
                                        let vol_z = z + kz - pad_d;
                                        let vol_y = y + ky - pad_h;
                                        let vol_x = x + kx - pad_w;

                                        sum += tensor.data[[vol_t, vol_z, vol_y, vol_x, channel]]
                                            * kernel.data[[kt_i, kz, ky, kx, 0]];
                                    }
                                }
                            }
                        }

                        result.data[[t, z, y, x, channel]] = sum;
                    }
                }
            }
        }

        result
    }

    /// Aplica filtro de média 4D
    pub fn mean_filter(tensor: &Tensor4D, channel: usize) -> Tensor4D {
        let kernel = ConvolutionKernel4D::mean_3x3x3x3();
        Self::convolve(tensor, &kernel, channel)
    }

    /// Aplica Gaussian blur 4D
    pub fn gaussian_blur(tensor: &Tensor4D, channel: usize) -> Tensor4D {
        let kernel = ConvolutionKernel4D::gaussian_3x3x3x3();
        Self::convolve(tensor, &kernel, channel)
    }

    /// Detecção de bordas espaço-temporais (Laplaciano 4D)
    pub fn laplacian(tensor: &Tensor4D, channel: usize) -> Tensor4D {
        let kernel = ConvolutionKernel4D::laplacian_4d();
        Self::convolve(tensor, &kernel, channel)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tensor::TensorOps;

    #[test]
    fn test_kernel_mean() {
        let kernel = ConvolutionKernel4D::mean_3x3x3x3();
        assert_eq!(kernel.size(), (3, 3, 3, 3));
        let sum: f32 = kernel.data.iter().sum();
        assert!((sum - 1.0).abs() < 1e-5);
    }

    #[test]
    fn test_kernel_gaussian() {
        let kernel = ConvolutionKernel4D::gaussian_3x3x3x3();
        assert_eq!(kernel.size(), (3, 3, 3, 3));
        let sum: f32 = kernel.data.iter().sum();
        assert!((sum - 1.0).abs() < 1e-4);
    }

    #[test]
    fn test_mean_filter() {
        let tensor = TensorOps::create_test_tensor(7, 7, 7, 7);
        let filtered = Filter4D::mean_filter(&tensor, 0);
        assert_eq!(filtered.shape(), tensor.shape());
    }

    #[test]
    fn test_gaussian_blur() {
        let tensor = TensorOps::create_test_tensor(7, 7, 7, 7);
        let blurred = Filter4D::gaussian_blur(&tensor, 0);
        assert_eq!(blurred.shape(), tensor.shape());
    }
}
