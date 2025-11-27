//! Image operations (resize, crop, rotate, etc.)

use crate::Image;

impl Image {
    /// Resize image to new dimensions
    pub fn resize(&self, new_width: u32, new_height: u32) -> Image {
        // Stub - nearest neighbor resize
        let mut result = Image::new(new_width, new_height);

        for y in 0..new_height {
            for x in 0..new_width {
                let src_x = (x * self.width() / new_width).min(self.width() - 1);
                let src_y = (y * self.height() / new_height).min(self.height() - 1);
                let pixel = self.get_pixel(src_x, src_y);
                result.set_pixel(x, y, pixel);
            }
        }

        result
    }

    /// Crop image to specified region
    pub fn crop(&self, x: u32, y: u32, width: u32, height: u32) -> Image {
        let mut result = Image::new(width, height);

        for dy in 0..height {
            for dx in 0..width {
                let src_x = (x + dx).min(self.width() - 1);
                let src_y = (y + dy).min(self.height() - 1);
                let pixel = self.get_pixel(src_x, src_y);
                result.set_pixel(dx, dy, pixel);
            }
        }

        result
    }

    /// Rotate image 90 degrees clockwise
    pub fn rotate_90(&self) -> Image {
        let mut result = Image::new(self.height(), self.width());

        for y in 0..self.height() {
            for x in 0..self.width() {
                let pixel = self.get_pixel(x, y);
                result.set_pixel(self.height() - 1 - y, x, pixel);
            }
        }

        result
    }

    /// Flip image horizontally
    pub fn flip_horizontal(&self) -> Image {
        let mut result = Image::new(self.width(), self.height());

        for y in 0..self.height() {
            for x in 0..self.width() {
                let pixel = self.get_pixel(x, y);
                result.set_pixel(self.width() - 1 - x, y, pixel);
            }
        }

        result
    }

    /// Flip image vertically
    pub fn flip_vertical(&self) -> Image {
        let mut result = Image::new(self.width(), self.height());

        for y in 0..self.height() {
            for x in 0..self.width() {
                let pixel = self.get_pixel(x, y);
                result.set_pixel(x, self.height() - 1 - y, pixel);
            }
        }

        result
    }

    /// Convert to grayscale
    pub fn to_grayscale(&self) -> Image {
        let mut result = self.clone();

        for y in 0..self.height() {
            for x in 0..self.width() {
                let pixel = self.get_pixel(x, y);
                let gray = (0.299 * pixel.r() as f32
                          + 0.587 * pixel.g() as f32
                          + 0.114 * pixel.b() as f32) as u8;
                result.set_pixel(x, y, crate::Rgb([gray, gray, gray]));
            }
        }

        result
    }

    /// Adjust brightness
    pub fn adjust_brightness(&mut self, factor: f32) {
        for y in 0..self.height() {
            for x in 0..self.width() {
                let pixel = self.get_pixel(x, y);
                let r = ((pixel.r() as f32 * factor).min(255.0)) as u8;
                let g = ((pixel.g() as f32 * factor).min(255.0)) as u8;
                let b = ((pixel.b() as f32 * factor).min(255.0)) as u8;
                self.set_pixel(x, y, crate::Rgb([r, g, b]));
            }
        }
    }
}
