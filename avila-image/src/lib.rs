//! # avila-image
//!
//! Image processing library for AVL Platform.
//!
//! ## Features
//!
//! - PNG, JPEG, BMP codec support
//! - Basic image operations (resize, crop, rotate)
//! - Color spaces (RGB, RGBA, Grayscale)
//! - Zero-copy where possible
//!
//! ## Quick Start
//!
//! ```rust
//! use avila_image::{Image, Rgb};
//!
//! let mut img = Image::new(800, 600);
//! img.set_pixel(100, 100, Rgb([255, 0, 0]));
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]

pub mod color;
pub mod image;
pub mod codecs;
pub mod ops;
pub mod error;

pub use color::{Rgb, Rgba, Gray, Pixel};
pub use image::{Image, ImageBuffer};
pub use error::{ImageError, Result};

/// Image format enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
    /// PNG format
    Png,
    /// JPEG format
    Jpeg,
    /// BMP format
    Bmp,
}

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_image() {
        let img = Image::new(100, 100);
        assert_eq!(img.width(), 100);
        assert_eq!(img.height(), 100);
    }

    #[test]
    fn test_pixel_operations() {
        let mut img = Image::new(10, 10);
        let red = Rgb([255, 0, 0]);
        img.set_pixel(5, 5, red);
        let pixel = img.get_pixel(5, 5);
        assert_eq!(pixel, red);
    }

    #[test]
    fn test_rgb_color() {
        let color = Rgb([100, 150, 200]);
        assert_eq!(color.channels(), &[100, 150, 200]);
    }
}
