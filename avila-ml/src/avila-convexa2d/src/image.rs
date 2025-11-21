//! Processamento de imagens 2D
//!
//! Suporta múltiplos formatos de cor e operações básicas de imagem

use crate::common::{Size2D, Rect};
use ndarray::{Array2, Array3, ArrayView2};
use serde::{Deserialize, Serialize};

/// Espaço de cor da imagem
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ColorSpace {
    /// Escala de cinza (1 canal)
    Grayscale,
    /// RGB (3 canais)
    RGB,
    /// RGBA (4 canais)
    RGBA,
    /// HSV (3 canais)
    HSV,
}

/// Pixel individual (RGB)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Pixel {
    /// Cria novo pixel RGB
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    /// Cria novo pixel RGBA
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Cria pixel em escala de cinza
    pub fn gray(value: u8) -> Self {
        Self::rgb(value, value, value)
    }

    /// Converte para escala de cinza (luminance)
    pub fn to_gray(&self) -> u8 {
        (0.299 * self.r as f32 + 0.587 * self.g as f32 + 0.114 * self.b as f32) as u8
    }

    /// Preto
    pub fn black() -> Self {
        Self::rgb(0, 0, 0)
    }

    /// Branco
    pub fn white() -> Self {
        Self::rgb(255, 255, 255)
    }
}

/// Imagem 2D
#[derive(Debug, Clone)]
pub struct Image {
    /// Dados da imagem (altura x largura x canais)
    data: Array3<u8>,
    /// Espaço de cor
    color_space: ColorSpace,
}

impl Image {
    /// Cria nova imagem vazia
    pub fn new(width: u32, height: u32, color_space: ColorSpace) -> Self {
        let channels = match color_space {
            ColorSpace::Grayscale => 1,
            ColorSpace::RGB | ColorSpace::HSV => 3,
            ColorSpace::RGBA => 4,
        };

        Self {
            data: Array3::zeros((height as usize, width as usize, channels)),
            color_space,
        }
    }

    /// Cria imagem preenchida com cor
    pub fn filled(width: u32, height: u32, pixel: Pixel) -> Self {
        let mut img = Self::new(width, height, ColorSpace::RGB);
        for y in 0..height {
            for x in 0..width {
                img.set_pixel(x, y, pixel);
            }
        }
        img
    }

    /// Cria imagem em escala de cinza a partir de dados
    pub fn from_gray(data: Array2<u8>) -> Self {
        let (height, width) = data.dim();
        let data_3d = data.into_shape((height, width, 1)).unwrap();
        Self {
            data: data_3d,
            color_space: ColorSpace::Grayscale,
        }
    }

    /// Largura da imagem
    pub fn width(&self) -> u32 {
        self.data.shape()[1] as u32
    }

    /// Altura da imagem
    pub fn height(&self) -> u32 {
        self.data.shape()[0] as u32
    }

    /// Tamanho da imagem
    pub fn size(&self) -> Size2D {
        Size2D::new(self.width(), self.height())
    }

    /// Número de canais
    pub fn channels(&self) -> usize {
        self.data.shape()[2]
    }

    /// Espaço de cor
    pub fn color_space(&self) -> ColorSpace {
        self.color_space
    }

    /// Obtém pixel em coordenada
    pub fn get_pixel(&self, x: u32, y: u32) -> Pixel {
        if x >= self.width() || y >= self.height() {
            return Pixel::black();
        }

        let idx = (y as usize, x as usize);
        match self.channels() {
            1 => {
                let gray = self.data[[idx.0, idx.1, 0]];
                Pixel::gray(gray)
            }
            3 => Pixel::rgb(
                self.data[[idx.0, idx.1, 0]],
                self.data[[idx.0, idx.1, 1]],
                self.data[[idx.0, idx.1, 2]],
            ),
            4 => Pixel::rgba(
                self.data[[idx.0, idx.1, 0]],
                self.data[[idx.0, idx.1, 1]],
                self.data[[idx.0, idx.1, 2]],
                self.data[[idx.0, idx.1, 3]],
            ),
            _ => Pixel::black(),
        }
    }

    /// Define pixel em coordenada
    pub fn set_pixel(&mut self, x: u32, y: u32, pixel: Pixel) {
        if x >= self.width() || y >= self.height() {
            return;
        }

        let idx = (y as usize, x as usize);
        match self.channels() {
            1 => {
                self.data[[idx.0, idx.1, 0]] = pixel.to_gray();
            }
            3 => {
                self.data[[idx.0, idx.1, 0]] = pixel.r;
                self.data[[idx.0, idx.1, 1]] = pixel.g;
                self.data[[idx.0, idx.1, 2]] = pixel.b;
            }
            4 => {
                self.data[[idx.0, idx.1, 0]] = pixel.r;
                self.data[[idx.0, idx.1, 1]] = pixel.g;
                self.data[[idx.0, idx.1, 2]] = pixel.b;
                self.data[[idx.0, idx.1, 3]] = pixel.a;
            }
            _ => {}
        }
    }

    /// Converte para escala de cinza
    pub fn to_grayscale(&self) -> Image {
        if self.color_space == ColorSpace::Grayscale {
            return self.clone();
        }

        let mut gray = Image::new(self.width(), self.height(), ColorSpace::Grayscale);
        for y in 0..self.height() {
            for x in 0..self.width() {
                let pixel = self.get_pixel(x, y);
                gray.set_pixel(x, y, Pixel::gray(pixel.to_gray()));
            }
        }
        gray
    }

    /// Recorta região da imagem
    pub fn crop(&self, rect: Rect) -> Option<Image> {
        if rect.x < 0 || rect.y < 0 {
            return None;
        }

        let x = rect.x as u32;
        let y = rect.y as u32;

        if x + rect.width > self.width() || y + rect.height > self.height() {
            return None;
        }

        let mut cropped = Image::new(rect.width, rect.height, self.color_space);
        for dy in 0..rect.height {
            for dx in 0..rect.width {
                let pixel = self.get_pixel(x + dx, y + dy);
                cropped.set_pixel(dx, dy, pixel);
            }
        }
        Some(cropped)
    }

    /// Retorna canal específico como matriz 2D
    pub fn channel(&self, index: usize) -> Option<ArrayView2<u8>> {
        if index >= self.channels() {
            return None;
        }
        Some(self.data.slice(ndarray::s![.., .., index]))
    }

    /// Acesso aos dados brutos
    pub fn data(&self) -> &Array3<u8> {
        &self.data
    }
}

/// Processador de imagens
pub struct ImageProcessor;

impl ImageProcessor {
    /// Inverte cores da imagem
    pub fn invert(image: &Image) -> Image {
        let mut inverted = image.clone();
        for y in 0..image.height() {
            for x in 0..image.width() {
                let pixel = image.get_pixel(x, y);
                inverted.set_pixel(
                    x,
                    y,
                    Pixel::rgba(255 - pixel.r, 255 - pixel.g, 255 - pixel.b, pixel.a),
                );
            }
        }
        inverted
    }

    /// Ajusta brilho (-255 a +255)
    pub fn brightness(image: &Image, value: i32) -> Image {
        let mut result = image.clone();
        for y in 0..image.height() {
            for x in 0..image.width() {
                let pixel = image.get_pixel(x, y);
                result.set_pixel(
                    x,
                    y,
                    Pixel::rgb(
                        ((pixel.r as i32 + value).max(0).min(255)) as u8,
                        ((pixel.g as i32 + value).max(0).min(255)) as u8,
                        ((pixel.b as i32 + value).max(0).min(255)) as u8,
                    ),
                );
            }
        }
        result
    }

    /// Ajusta contraste (0.0 = sem contraste, 1.0 = normal, 2.0 = 2x contraste)
    pub fn contrast(image: &Image, factor: f32) -> Image {
        let mut result = image.clone();
        for y in 0..image.height() {
            for x in 0..image.width() {
                let pixel = image.get_pixel(x, y);
                let adjust = |val: u8| -> u8 {
                    let v = ((val as f32 - 128.0) * factor + 128.0)
                        .max(0.0)
                        .min(255.0);
                    v as u8
                };
                result.set_pixel(
                    x,
                    y,
                    Pixel::rgb(adjust(pixel.r), adjust(pixel.g), adjust(pixel.b)),
                );
            }
        }
        result
    }

    /// Binarização (threshold)
    pub fn threshold(image: &Image, threshold: u8) -> Image {
        let gray = image.to_grayscale();
        let mut binary = Image::new(gray.width(), gray.height(), ColorSpace::Grayscale);

        for y in 0..gray.height() {
            for x in 0..gray.width() {
                let value = gray.get_pixel(x, y).to_gray();
                let binary_val = if value >= threshold { 255 } else { 0 };
                binary.set_pixel(x, y, Pixel::gray(binary_val));
            }
        }
        binary
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_creation() {
        let img = Image::new(100, 100, ColorSpace::RGB);
        assert_eq!(img.width(), 100);
        assert_eq!(img.height(), 100);
        assert_eq!(img.channels(), 3);
    }

    #[test]
    fn test_pixel_operations() {
        let mut img = Image::new(10, 10, ColorSpace::RGB);
        let red = Pixel::rgb(255, 0, 0);
        img.set_pixel(5, 5, red);

        let pixel = img.get_pixel(5, 5);
        assert_eq!(pixel.r, 255);
        assert_eq!(pixel.g, 0);
        assert_eq!(pixel.b, 0);
    }

    #[test]
    fn test_grayscale_conversion() {
        let pixel = Pixel::rgb(100, 150, 200);
        let gray = pixel.to_gray();
        assert!(gray > 0);
    }

    #[test]
    fn test_crop() {
        let img = Image::filled(100, 100, Pixel::white());
        let rect = Rect::new(10, 10, 20, 20);
        let cropped = img.crop(rect).unwrap();

        assert_eq!(cropped.width(), 20);
        assert_eq!(cropped.height(), 20);
    }

    #[test]
    fn test_invert() {
        let mut img = Image::new(10, 10, ColorSpace::RGB);
        img.set_pixel(5, 5, Pixel::rgb(100, 150, 200));

        let inverted = ImageProcessor::invert(&img);
        let pixel = inverted.get_pixel(5, 5);

        assert_eq!(pixel.r, 155);
        assert_eq!(pixel.g, 105);
        assert_eq!(pixel.b, 55);
    }
}
