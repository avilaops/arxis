//! Operações de convolução e filtros 3D

use ndarray::Array4;

use crate::volume::Volume3D;

/// Kernel de convolução 3D
#[derive(Debug, Clone)]
pub struct ConvolutionKernel3D {
    /// Dados do kernel (depth × height × width)
    pub data: Array4<f32>,
}

impl ConvolutionKernel3D {
    /// Cria novo kernel
    pub fn new(data: Array4<f32>) -> Self {
        Self { data }
    }

    /// Retorna tamanho do kernel
    pub fn size(&self) -> (usize, usize, usize) {
        (self.data.shape()[0], self.data.shape()[1], self.data.shape()[2])
    }

    /// Kernel de média 3×3×3
    pub fn mean_3x3x3() -> Self {
        let value = 1.0 / 27.0;
        Self {
            data: Array4::from_elem((3, 3, 3, 1), value),
        }
    }

    /// Kernel Gaussiano 3×3×3 (sigma=1.0)
    pub fn gaussian_3x3x3() -> Self {
        #[rustfmt::skip]
        let kernel_data = vec![
            // z=0
            0.011, 0.019, 0.011,
            0.019, 0.032, 0.019,
            0.011, 0.019, 0.011,
            // z=1
            0.019, 0.032, 0.019,
            0.032, 0.053, 0.032,
            0.019, 0.032, 0.019,
            // z=2
            0.011, 0.019, 0.011,
            0.019, 0.032, 0.019,
            0.011, 0.019, 0.011,
        ];

        Self {
            data: Array4::from_shape_vec((3, 3, 3, 1), kernel_data).unwrap(),
        }
    }

    /// Kernel Laplaciano 3D
    pub fn laplacian_3d() -> Self {
        let mut data = Array4::zeros((3, 3, 3, 1));

        // Centro
        data[[1, 1, 1, 0]] = -26.0;

        // Faces adjacentes
        data[[0, 1, 1, 0]] = 1.0;
        data[[2, 1, 1, 0]] = 1.0;
        data[[1, 0, 1, 0]] = 1.0;
        data[[1, 2, 1, 0]] = 1.0;
        data[[1, 1, 0, 0]] = 1.0;
        data[[1, 1, 2, 0]] = 1.0;

        // Arestas
        for z in 0..3 {
            for y in 0..3 {
                for x in 0..3 {
                    if (z == 1 && y == 1) || (z == 1 && x == 1) || (y == 1 && x == 1) {
                        continue;
                    }
                    if z != 1 || y != 1 || x != 1 {
                        data[[z, y, x, 0]] = 1.0;
                    }
                }
            }
        }

        Self { data }
    }

    /// Kernel Sobel X (3D)
    pub fn sobel_x_3d() -> Self {
        let mut data = Array4::zeros((3, 3, 3, 1));

        // Plano z=0
        data[[0, 0, 0, 0]] = -1.0;
        data[[0, 1, 0, 0]] = -2.0;
        data[[0, 2, 0, 0]] = -1.0;
        data[[0, 0, 2, 0]] = 1.0;
        data[[0, 1, 2, 0]] = 2.0;
        data[[0, 2, 2, 0]] = 1.0;

        // Plano z=1
        data[[1, 0, 0, 0]] = -2.0;
        data[[1, 1, 0, 0]] = -4.0;
        data[[1, 2, 0, 0]] = -2.0;
        data[[1, 0, 2, 0]] = 2.0;
        data[[1, 1, 2, 0]] = 4.0;
        data[[1, 2, 2, 0]] = 2.0;

        // Plano z=2
        data[[2, 0, 0, 0]] = -1.0;
        data[[2, 1, 0, 0]] = -2.0;
        data[[2, 2, 0, 0]] = -1.0;
        data[[2, 0, 2, 0]] = 1.0;
        data[[2, 1, 2, 0]] = 2.0;
        data[[2, 2, 2, 0]] = 1.0;

        Self { data }
    }
}

/// Filtro 3D
pub struct Filter3D;

impl Filter3D {
    /// Aplica convolução 3D
    pub fn convolve(volume: &Volume3D, kernel: &ConvolutionKernel3D, channel: usize) -> Volume3D {
        let (vd, vh, vw, vc) = volume.shape();
        let (kd, kh, kw) = kernel.size();

        let pad_d = kd / 2;
        let pad_h = kh / 2;
        let pad_w = kw / 2;

        let mut result = Volume3D::zeros(vd, vh, vw, vc);

        for z in pad_d..(vd - pad_d) {
            for y in pad_h..(vh - pad_h) {
                for x in pad_w..(vw - pad_w) {
                    let mut sum = 0.0;

                    for kz in 0..kd {
                        for ky in 0..kh {
                            for kx in 0..kw {
                                let vol_z = z + kz - pad_d;
                                let vol_y = y + ky - pad_h;
                                let vol_x = x + kx - pad_w;

                                sum += volume.data[[vol_z, vol_y, vol_x, channel]]
                                    * kernel.data[[kz, ky, kx, 0]];
                            }
                        }
                    }

                    result.data[[z, y, x, channel]] = sum;
                }
            }
        }

        result
    }

    /// Aplica blur Gaussiano 3D
    pub fn gaussian_blur(volume: &Volume3D, channel: usize) -> Volume3D {
        let kernel = ConvolutionKernel3D::gaussian_3x3x3();
        Self::convolve(volume, &kernel, channel)
    }

    /// Aplica filtro de média 3D
    pub fn mean_filter(volume: &Volume3D, channel: usize) -> Volume3D {
        let kernel = ConvolutionKernel3D::mean_3x3x3();
        Self::convolve(volume, &kernel, channel)
    }

    /// Detecção de bordas (Laplaciano)
    pub fn laplacian(volume: &Volume3D, channel: usize) -> Volume3D {
        let kernel = ConvolutionKernel3D::laplacian_3d();
        Self::convolve(volume, &kernel, channel)
    }

    /// Detecção de bordas (Sobel X)
    pub fn sobel_x(volume: &Volume3D, channel: usize) -> Volume3D {
        let kernel = ConvolutionKernel3D::sobel_x_3d();
        Self::convolve(volume, &kernel, channel)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::volume::VolumeProcessor;

    #[test]
    fn test_kernel_mean() {
        let kernel = ConvolutionKernel3D::mean_3x3x3();
        assert_eq!(kernel.size(), (3, 3, 3));
        let sum: f32 = kernel.data.iter().sum();
        assert!((sum - 1.0).abs() < 1e-5);
    }

    #[test]
    fn test_kernel_gaussian() {
        let kernel = ConvolutionKernel3D::gaussian_3x3x3();
        assert_eq!(kernel.size(), (3, 3, 3));
    }

    #[test]
    fn test_convolve_mean() {
        let volume = VolumeProcessor::create_test_volume(10, 10, 10);
        let filtered = Filter3D::mean_filter(&volume, 0);
        assert_eq!(filtered.shape(), volume.shape());
    }

    #[test]
    fn test_gaussian_blur() {
        let volume = VolumeProcessor::create_sphere(5);
        let blurred = Filter3D::gaussian_blur(&volume, 0);
        assert_eq!(blurred.shape(), volume.shape());
    }
}
