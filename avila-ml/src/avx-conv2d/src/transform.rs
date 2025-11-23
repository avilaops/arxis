//! Transformações geométricas 2D

use crate::{Image, Pixel};

/// Tipo de interpolação
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Interpolation {
    /// Vizinho mais próximo (rápido, baixa qualidade)
    NearestNeighbor,
    /// Bilinear (médio, boa qualidade)
    Bilinear,
    /// Bicúbica (lento, alta qualidade)
    Bicubic,
}

/// Transformações 2D
pub struct Transform2D;

impl Transform2D {
    /// Redimensiona imagem
    pub fn resize(
        image: &Image,
        new_width: u32,
        new_height: u32,
        interpolation: Interpolation,
    ) -> Image {
        match interpolation {
            Interpolation::NearestNeighbor => {
                Self::resize_nearest_neighbor(image, new_width, new_height)
            }
            Interpolation::Bilinear => Self::resize_bilinear(image, new_width, new_height),
            Interpolation::Bicubic => {
                // Fallback para bilinear por enquanto
                Self::resize_bilinear(image, new_width, new_height)
            }
        }
    }

    /// Redimensiona usando vizinho mais próximo
    fn resize_nearest_neighbor(image: &Image, new_width: u32, new_height: u32) -> Image {
        let mut result = Image::new(new_width, new_height, image.color_space());

        let x_ratio = image.width() as f32 / new_width as f32;
        let y_ratio = image.height() as f32 / new_height as f32;

        for y in 0..new_height {
            for x in 0..new_width {
                let src_x = (x as f32 * x_ratio) as u32;
                let src_y = (y as f32 * y_ratio) as u32;

                let pixel = image.get_pixel(src_x, src_y);
                result.set_pixel(x, y, pixel);
            }
        }

        result
    }

    /// Redimensiona usando interpolação bilinear
    fn resize_bilinear(image: &Image, new_width: u32, new_height: u32) -> Image {
        let mut result = Image::new(new_width, new_height, image.color_space());

        let x_ratio = (image.width() - 1) as f32 / (new_width - 1) as f32;
        let y_ratio = (image.height() - 1) as f32 / (new_height - 1) as f32;

        for y in 0..new_height {
            for x in 0..new_width {
                let src_x = x as f32 * x_ratio;
                let src_y = y as f32 * y_ratio;

                let x0 = src_x.floor() as u32;
                let y0 = src_y.floor() as u32;
                let x1 = (x0 + 1).min(image.width() - 1);
                let y1 = (y0 + 1).min(image.height() - 1);

                let dx = src_x - x0 as f32;
                let dy = src_y - y0 as f32;

                let p00 = image.get_pixel(x0, y0);
                let p10 = image.get_pixel(x1, y0);
                let p01 = image.get_pixel(x0, y1);
                let p11 = image.get_pixel(x1, y1);

                let interpolated = Self::bilinear_interpolate(p00, p10, p01, p11, dx, dy);
                result.set_pixel(x, y, interpolated);
            }
        }

        result
    }

    /// Interpolação bilinear entre 4 pixels
    fn bilinear_interpolate(
        p00: Pixel,
        p10: Pixel,
        p01: Pixel,
        p11: Pixel,
        dx: f32,
        dy: f32,
    ) -> Pixel {
        let lerp = |a: u8, b: u8, t: f32| -> u8 {
            (a as f32 * (1.0 - t) + b as f32 * t) as u8
        };

        let r0 = lerp(p00.r, p10.r, dx);
        let r1 = lerp(p01.r, p11.r, dx);
        let r = lerp(r0, r1, dy);

        let g0 = lerp(p00.g, p10.g, dx);
        let g1 = lerp(p01.g, p11.g, dx);
        let g = lerp(g0, g1, dy);

        let b0 = lerp(p00.b, p10.b, dx);
        let b1 = lerp(p01.b, p11.b, dx);
        let b = lerp(b0, b1, dy);

        Pixel::rgb(r, g, b)
    }

    /// Rotaciona imagem (ângulo em graus)
    pub fn rotate(image: &Image, angle_deg: f32, fill: Pixel) -> Image {
        let angle_rad = angle_deg.to_radians();
        let cos_a = angle_rad.cos();
        let sin_a = angle_rad.sin();

        let (w, h) = (image.width() as i32, image.height() as i32);
        let cx = w as f32 / 2.0;
        let cy = h as f32 / 2.0;

        // Calcula novo tamanho
        let corners = [
            (0.0, 0.0),
            (w as f32, 0.0),
            (0.0, h as f32),
            (w as f32, h as f32),
        ];

        let rotated_corners: Vec<_> = corners
            .iter()
            .map(|(x, y)| {
                let rx = x - cx;
                let ry = y - cy;
                (
                    rx * cos_a - ry * sin_a + cx,
                    rx * sin_a + ry * cos_a + cy,
                )
            })
            .collect();

        let min_x = rotated_corners.iter().map(|(x, _)| *x).fold(f32::INFINITY, f32::min);
        let max_x = rotated_corners.iter().map(|(x, _)| *x).fold(f32::NEG_INFINITY, f32::max);
        let min_y = rotated_corners.iter().map(|(_, y)| *y).fold(f32::INFINITY, f32::min);
        let max_y = rotated_corners.iter().map(|(_, y)| *y).fold(f32::NEG_INFINITY, f32::max);

        let new_w = (max_x - min_x).ceil() as u32;
        let new_h = (max_y - min_y).ceil() as u32;

        let mut result = Image::filled(new_w, new_h, fill);

        let new_cx = new_w as f32 / 2.0;
        let new_cy = new_h as f32 / 2.0;

        for y in 0..new_h {
            for x in 0..new_w {
                let rx = x as f32 - new_cx;
                let ry = y as f32 - new_cy;

                let src_x = (rx * cos_a + ry * sin_a + cx) as i32;
                let src_y = (-rx * sin_a + ry * cos_a + cy) as i32;

                if src_x >= 0 && src_x < w && src_y >= 0 && src_y < h {
                    let pixel = image.get_pixel(src_x as u32, src_y as u32);
                    result.set_pixel(x, y, pixel);
                }
            }
        }

        result
    }

    /// Flip horizontal
    pub fn flip_horizontal(image: &Image) -> Image {
        let mut result = Image::new(image.width(), image.height(), image.color_space());

        for y in 0..image.height() {
            for x in 0..image.width() {
                let pixel = image.get_pixel(x, y);
                result.set_pixel(image.width() - 1 - x, y, pixel);
            }
        }

        result
    }

    /// Flip vertical
    pub fn flip_vertical(image: &Image) -> Image {
        let mut result = Image::new(image.width(), image.height(), image.color_space());

        for y in 0..image.height() {
            for x in 0..image.width() {
                let pixel = image.get_pixel(x, y);
                result.set_pixel(x, image.height() - 1 - y, pixel);
            }
        }

        result
    }

    /// Translada imagem
    pub fn translate(image: &Image, dx: i32, dy: i32, fill: Pixel) -> Image {
        let mut result = Image::filled(image.width(), image.height(), fill);

        for y in 0..image.height() {
            for x in 0..image.width() {
                let new_x = x as i32 + dx;
                let new_y = y as i32 + dy;

                if new_x >= 0
                    && new_x < image.width() as i32
                    && new_y >= 0
                    && new_y < image.height() as i32
                {
                    let pixel = image.get_pixel(x, y);
                    result.set_pixel(new_x as u32, new_y as u32, pixel);
                }
            }
        }

        result
    }

    /// Escala mantendo proporção
    pub fn scale_proportional(image: &Image, max_size: u32) -> Image {
        let (w, h) = (image.width(), image.height());
        let ratio = w as f32 / h as f32;

        let (new_w, new_h) = if w > h {
            (max_size, (max_size as f32 / ratio) as u32)
        } else {
            ((max_size as f32 * ratio) as u32, max_size)
        };

        Self::resize(image, new_w, new_h, Interpolation::Bilinear)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resize_nearest_neighbor() {
        let img = Image::filled(100, 100, Pixel::white());
        let resized = Transform2D::resize(&img, 50, 50, Interpolation::NearestNeighbor);

        assert_eq!(resized.width(), 50);
        assert_eq!(resized.height(), 50);
    }

    #[test]
    fn test_resize_bilinear() {
        let img = Image::filled(100, 100, Pixel::white());
        let resized = Transform2D::resize(&img, 150, 150, Interpolation::Bilinear);

        assert_eq!(resized.width(), 150);
        assert_eq!(resized.height(), 150);
    }

    #[test]
    fn test_flip_horizontal() {
        let mut img = Image::new(10, 10, crate::ColorSpace::RGB);
        img.set_pixel(0, 0, Pixel::rgb(255, 0, 0));

        let flipped = Transform2D::flip_horizontal(&img);
        let pixel = flipped.get_pixel(9, 0);

        assert_eq!(pixel.r, 255);
    }

    #[test]
    fn test_flip_vertical() {
        let mut img = Image::new(10, 10, crate::ColorSpace::RGB);
        img.set_pixel(0, 0, Pixel::rgb(255, 0, 0));

        let flipped = Transform2D::flip_vertical(&img);
        let pixel = flipped.get_pixel(0, 9);

        assert_eq!(pixel.r, 255);
    }

    #[test]
    fn test_translate() {
        let mut img = Image::new(10, 10, crate::ColorSpace::RGB);
        img.set_pixel(5, 5, Pixel::rgb(255, 0, 0));

        let translated = Transform2D::translate(&img, 2, 2, Pixel::black());
        let pixel = translated.get_pixel(7, 7);

        assert_eq!(pixel.r, 255);
    }

    #[test]
    fn test_scale_proportional() {
        let img = Image::new(200, 100, crate::ColorSpace::RGB);
        let scaled = Transform2D::scale_proportional(&img, 100);

        assert!(scaled.width() <= 100 || scaled.height() <= 100);
    }
}
