//! Filtros e operações de convolução 2D

use crate::{Image, Matrix2D, MatrixOps};
use ndarray::Array2;

/// Kernel de convolução 2D
#[derive(Debug, Clone)]
pub struct ConvolutionKernel2D {
    /// Matriz de pesos
    pub weights: Matrix2D<f32>,
    /// Divisor para normalização
    pub divisor: f32,
    /// Offset para o resultado
    pub offset: f32,
}

impl ConvolutionKernel2D {
    /// Cria novo kernel
    pub fn new(weights: Matrix2D<f32>) -> Self {
        Self {
            weights,
            divisor: 1.0,
            offset: 0.0,
        }
    }

    /// Cria kernel com normalização
    pub fn normalized(weights: Matrix2D<f32>) -> Self {
        let sum = weights.sum();
        let divisor = if sum != 0.0 { sum } else { 1.0 };
        Self {
            weights,
            divisor,
            offset: 0.0,
        }
    }

    /// Filtro de desfoque (blur) 3x3
    pub fn blur_3x3() -> Self {
        let weights = Matrix2D::from_array(
            Array2::from_shape_vec(
                (3, 3),
                vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
            )
            .unwrap(),
        );
        Self::normalized(weights)
    }

    /// Filtro de desfoque gaussiano 3x3
    pub fn gaussian_blur_3x3() -> Self {
        let weights = Matrix2D::from_array(
            Array2::from_shape_vec(
                (3, 3),
                vec![1.0, 2.0, 1.0, 2.0, 4.0, 2.0, 1.0, 2.0, 1.0],
            )
            .unwrap(),
        );
        Self::normalized(weights)
    }

    /// Filtro de desfoque gaussiano 5x5
    pub fn gaussian_blur_5x5() -> Self {
        let weights = Matrix2D::from_array(
            Array2::from_shape_vec(
                (5, 5),
                vec![
                    1.0, 4.0, 7.0, 4.0, 1.0,
                    4.0, 16.0, 26.0, 16.0, 4.0,
                    7.0, 26.0, 41.0, 26.0, 7.0,
                    4.0, 16.0, 26.0, 16.0, 4.0,
                    1.0, 4.0, 7.0, 4.0, 1.0,
                ],
            )
            .unwrap(),
        );
        Self::normalized(weights)
    }

    /// Filtro de nitidez (sharpen)
    pub fn sharpen() -> Self {
        let weights = Matrix2D::from_array(
            Array2::from_shape_vec(
                (3, 3),
                vec![0.0, -1.0, 0.0, -1.0, 5.0, -1.0, 0.0, -1.0, 0.0],
            )
            .unwrap(),
        );
        Self::new(weights)
    }

    /// Filtro de realce de bordas
    pub fn edge_enhance() -> Self {
        let weights = Matrix2D::from_array(
            Array2::from_shape_vec(
                (3, 3),
                vec![-1.0, -1.0, -1.0, -1.0, 8.0, -1.0, -1.0, -1.0, -1.0],
            )
            .unwrap(),
        );
        Self::new(weights)
    }

    /// Aplica convolução em imagem em escala de cinza
    pub fn apply_gray(&self, image: &Image) -> Image {
        let gray = image.to_grayscale();
        let channel = gray.channel(0).unwrap();

        // Converte u8 para f32
        let matrix_f32 = channel.mapv(|x| x as f32);
        let matrix = Matrix2D::from_array(matrix_f32);

        let result = matrix.convolve(&self.weights);
        let normalized = result.scale(1.0 / self.divisor);

        // Converte de volta para imagem
        let mut output = Image::new(
            (normalized.cols()) as u32,
            (normalized.rows()) as u32,
            crate::ColorSpace::Grayscale,
        );

        for y in 0..normalized.rows() {
            for x in 0..normalized.cols() {
                let val = normalized.get(y, x).unwrap_or(0.0) + self.offset;
                let clamped = val.max(0.0).min(255.0) as u8;
                output.set_pixel(x as u32, y as u32, crate::Pixel::gray(clamped));
            }
        }

        output
    }
}

/// Detecção de bordas
pub struct EdgeDetection;

impl EdgeDetection {
    /// Operador Sobel X (detecção de bordas verticais)
    pub fn sobel_x() -> ConvolutionKernel2D {
        let weights = Matrix2D::from_array(
            Array2::from_shape_vec(
                (3, 3),
                vec![-1.0, 0.0, 1.0, -2.0, 0.0, 2.0, -1.0, 0.0, 1.0],
            )
            .unwrap(),
        );
        ConvolutionKernel2D::new(weights)
    }

    /// Operador Sobel Y (detecção de bordas horizontais)
    pub fn sobel_y() -> ConvolutionKernel2D {
        let weights = Matrix2D::from_array(
            Array2::from_shape_vec(
                (3, 3),
                vec![-1.0, -2.0, -1.0, 0.0, 0.0, 0.0, 1.0, 2.0, 1.0],
            )
            .unwrap(),
        );
        ConvolutionKernel2D::new(weights)
    }

    /// Operador Sobel combinado (magnitude do gradiente)
    pub fn sobel(image: &Image) -> Image {
        let sobel_x = Self::sobel_x();
        let sobel_y = Self::sobel_y();

        let gx = sobel_x.apply_gray(image);
        let gy = sobel_y.apply_gray(image);

        // Combina gradientes: sqrt(gx² + gy²)
        let mut result = Image::new(gx.width(), gx.height(), crate::ColorSpace::Grayscale);

        for y in 0..gx.height() {
            for x in 0..gx.width() {
                let px = gx.get_pixel(x, y).to_gray() as f32;
                let py = gy.get_pixel(x, y).to_gray() as f32;
                let magnitude = (px * px + py * py).sqrt().min(255.0);
                result.set_pixel(x, y, crate::Pixel::gray(magnitude as u8));
            }
        }

        result
    }

    /// Operador Prewitt X
    pub fn prewitt_x() -> ConvolutionKernel2D {
        let weights = Matrix2D::from_array(
            Array2::from_shape_vec(
                (3, 3),
                vec![-1.0, 0.0, 1.0, -1.0, 0.0, 1.0, -1.0, 0.0, 1.0],
            )
            .unwrap(),
        );
        ConvolutionKernel2D::new(weights)
    }

    /// Operador Prewitt Y
    pub fn prewitt_y() -> ConvolutionKernel2D {
        let weights = Matrix2D::from_array(
            Array2::from_shape_vec(
                (3, 3),
                vec![-1.0, -1.0, -1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
            )
            .unwrap(),
        );
        ConvolutionKernel2D::new(weights)
    }

    /// Operador Laplaciano
    pub fn laplacian() -> ConvolutionKernel2D {
        let weights = Matrix2D::from_array(
            Array2::from_shape_vec(
                (3, 3),
                vec![0.0, 1.0, 0.0, 1.0, -4.0, 1.0, 0.0, 1.0, 0.0],
            )
            .unwrap(),
        );
        ConvolutionKernel2D::new(weights)
    }
}

/// Filtros pré-definidos
pub struct Filter;

impl Filter {
    /// Desfoque simples
    pub fn blur(image: &Image) -> Image {
        ConvolutionKernel2D::blur_3x3().apply_gray(image)
    }

    /// Desfoque gaussiano
    pub fn gaussian_blur(image: &Image, size: usize) -> Image {
        let kernel = match size {
            3 => ConvolutionKernel2D::gaussian_blur_3x3(),
            5 => ConvolutionKernel2D::gaussian_blur_5x5(),
            _ => ConvolutionKernel2D::gaussian_blur_3x3(),
        };
        kernel.apply_gray(image)
    }

    /// Nitidez
    pub fn sharpen(image: &Image) -> Image {
        ConvolutionKernel2D::sharpen().apply_gray(image)
    }

    /// Realce de bordas
    pub fn edge_enhance(image: &Image) -> Image {
        ConvolutionKernel2D::edge_enhance().apply_gray(image)
    }

    /// Detecção de bordas Sobel
    pub fn detect_edges_sobel(image: &Image) -> Image {
        EdgeDetection::sobel(image)
    }

    /// Detecção de bordas Laplaciano
    pub fn detect_edges_laplacian(image: &Image) -> Image {
        EdgeDetection::laplacian().apply_gray(image)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Pixel;

    #[test]
    fn test_blur_kernel() {
        let kernel = ConvolutionKernel2D::blur_3x3();
        assert_eq!(kernel.weights.shape(), (3, 3));
    }

    #[test]
    fn test_sobel_kernels() {
        let sobel_x = EdgeDetection::sobel_x();
        let sobel_y = EdgeDetection::sobel_y();

        assert_eq!(sobel_x.weights.shape(), (3, 3));
        assert_eq!(sobel_y.weights.shape(), (3, 3));
    }

    #[test]
    fn test_apply_blur() {
        let mut img = Image::new(10, 10, crate::ColorSpace::Grayscale);
        for x in 0..10 {
            for y in 0..10 {
                img.set_pixel(x, y, Pixel::gray(128));
            }
        }

        let blurred = Filter::blur(&img);
        assert_eq!(blurred.width(), 8); // 10 - 3 + 1
        assert_eq!(blurred.height(), 8);
    }

    #[test]
    fn test_edge_detection() {
        let img = Image::filled(20, 20, Pixel::white());
        let edges = Filter::detect_edges_sobel(&img);

        assert!(edges.width() > 0);
        assert!(edges.height() > 0);
    }
}
