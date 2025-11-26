//! PNG and JPEG export for raster map rendering
//!
//! Converts Framebuffer to standard image formats using the `image` crate.

#[cfg(feature = "export-png")]
use image::{ImageBuffer, Rgb, RgbImage};

use crate::map::Map;
use crate::projection::Projection;
use crate::render::{Color, Framebuffer};
use std::io::Write;
use std::path::Path;

/// PNG/JPEG exporter for maps
pub struct ImageExporter;

impl ImageExporter {
    /// Export map to PNG file
    #[cfg(feature = "export-png")]
    pub fn save_png(
        map: &Map,
        projection: &dyn Projection,
        path: impl AsRef<Path>,
    ) -> Result<(), ImageExportError> {
        let fb = map.render(projection);
        Self::framebuffer_to_png(&fb, path)
    }

    /// Export map to JPEG file
    #[cfg(feature = "export-png")]
    pub fn save_jpeg(
        map: &Map,
        projection: &dyn Projection,
        path: impl AsRef<Path>,
        quality: u8,
    ) -> Result<(), ImageExportError> {
        let fb = map.render(projection);
        Self::framebuffer_to_jpeg(&fb, path, quality)
    }

    /// Convert Framebuffer to PNG
    #[cfg(feature = "export-png")]
    pub fn framebuffer_to_png(
        fb: &Framebuffer,
        path: impl AsRef<Path>,
    ) -> Result<(), ImageExportError> {
        let img = Self::framebuffer_to_image(fb);
        img.save_with_format(path, image::ImageFormat::Png)
            .map_err(|e| ImageExportError::IoError(e.to_string()))
    }

    /// Convert Framebuffer to JPEG
    #[cfg(feature = "export-png")]
    pub fn framebuffer_to_jpeg(
        fb: &Framebuffer,
        path: impl AsRef<Path>,
        quality: u8,
    ) -> Result<(), ImageExportError> {
        let img = Self::framebuffer_to_image(fb);

        let file = std::fs::File::create(path)
            .map_err(|e| ImageExportError::IoError(e.to_string()))?;
        let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(file, quality);

        encoder
            .encode(
                img.as_raw(),
                img.width(),
                img.height(),
                image::ColorType::Rgb8,
            )
            .map_err(|e| ImageExportError::IoError(e.to_string()))
    }

    /// Convert Framebuffer to ImageBuffer
    #[cfg(feature = "export-png")]
    fn framebuffer_to_image(fb: &Framebuffer) -> RgbImage {
        let mut img = ImageBuffer::new(fb.width, fb.height);

        for y in 0..fb.height {
            for x in 0..fb.width {
                let idx = (y * fb.width + x) as usize;
                let color = fb.pixels[idx];

                img.put_pixel(x, y, Rgb([color.r, color.g, color.b]));
            }
        }

        img
    }

    /// Get image as raw bytes (PNG format)
    #[cfg(feature = "export-png")]
    pub fn to_png_bytes(fb: &Framebuffer) -> Result<Vec<u8>, ImageExportError> {
        let img = Self::framebuffer_to_image(fb);
        let mut bytes = Vec::new();

        img.write_to(&mut std::io::Cursor::new(&mut bytes), image::ImageFormat::Png)
            .map_err(|e| ImageExportError::IoError(e.to_string()))?;

        Ok(bytes)
    }

    /// Get image as raw bytes (JPEG format)
    #[cfg(feature = "export-png")]
    pub fn to_jpeg_bytes(fb: &Framebuffer, quality: u8) -> Result<Vec<u8>, ImageExportError> {
        let img = Self::framebuffer_to_image(fb);
        let mut bytes = Vec::new();

        let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut bytes, quality);
        encoder
            .encode(
                img.as_raw(),
                img.width(),
                img.height(),
                image::ColorType::Rgb8,
            )
            .map_err(|e| ImageExportError::IoError(e.to_string()))?;

        Ok(bytes)
    }
}

/// Extension trait for Map to easily export to image formats
pub trait MapImageExt {
    #[cfg(feature = "export-png")]
    fn save_png(&self, projection: &dyn Projection, path: impl AsRef<Path>) -> Result<(), ImageExportError>;

    #[cfg(feature = "export-png")]
    fn save_jpeg(&self, projection: &dyn Projection, path: impl AsRef<Path>, quality: u8) -> Result<(), ImageExportError>;

    #[cfg(feature = "export-png")]
    fn to_png_bytes(&self, projection: &dyn Projection) -> Result<Vec<u8>, ImageExportError>;

    #[cfg(feature = "export-png")]
    fn to_jpeg_bytes(&self, projection: &dyn Projection, quality: u8) -> Result<Vec<u8>, ImageExportError>;
}

impl MapImageExt for Map {
    #[cfg(feature = "export-png")]
    fn save_png(&self, projection: &dyn Projection, path: impl AsRef<Path>) -> Result<(), ImageExportError> {
        ImageExporter::save_png(self, projection, path)
    }

    #[cfg(feature = "export-png")]
    fn save_jpeg(&self, projection: &dyn Projection, path: impl AsRef<Path>, quality: u8) -> Result<(), ImageExportError> {
        ImageExporter::save_jpeg(self, projection, path, quality)
    }

    #[cfg(feature = "export-png")]
    fn to_png_bytes(&self, projection: &dyn Projection) -> Result<Vec<u8>, ImageExportError> {
        let fb = self.render(projection);
        ImageExporter::to_png_bytes(&fb)
    }

    #[cfg(feature = "export-png")]
    fn to_jpeg_bytes(&self, projection: &dyn Projection, quality: u8) -> Result<Vec<u8>, ImageExportError> {
        let fb = self.render(projection);
        ImageExporter::to_jpeg_bytes(&fb, quality)
    }
}

/// Extension trait for Framebuffer
pub trait FramebufferImageExt {
    #[cfg(feature = "export-png")]
    fn save_png(&self, path: impl AsRef<Path>) -> Result<(), ImageExportError>;

    #[cfg(feature = "export-png")]
    fn save_jpeg(&self, path: impl AsRef<Path>, quality: u8) -> Result<(), ImageExportError>;

    #[cfg(feature = "export-png")]
    fn to_png_bytes(&self) -> Result<Vec<u8>, ImageExportError>;

    #[cfg(feature = "export-png")]
    fn to_jpeg_bytes(&self, quality: u8) -> Result<Vec<u8>, ImageExportError>;
}

impl FramebufferImageExt for Framebuffer {
    #[cfg(feature = "export-png")]
    fn save_png(&self, path: impl AsRef<Path>) -> Result<(), ImageExportError> {
        ImageExporter::framebuffer_to_png(self, path)
    }

    #[cfg(feature = "export-png")]
    fn save_jpeg(&self, path: impl AsRef<Path>, quality: u8) -> Result<(), ImageExportError> {
        ImageExporter::framebuffer_to_jpeg(self, path, quality)
    }

    #[cfg(feature = "export-png")]
    fn to_png_bytes(&self) -> Result<Vec<u8>, ImageExportError> {
        ImageExporter::to_png_bytes(self)
    }

    #[cfg(feature = "export-png")]
    fn to_jpeg_bytes(&self, quality: u8) -> Result<Vec<u8>, ImageExportError> {
        ImageExporter::to_jpeg_bytes(self, quality)
    }
}

#[derive(Debug)]
pub enum ImageExportError {
    IoError(String),
    UnsupportedFormat,
}

impl std::fmt::Display for ImageExportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImageExportError::IoError(s) => write!(f, "IO Error: {}", s),
            ImageExportError::UnsupportedFormat => write!(f, "Unsupported image format"),
        }
    }
}

impl std::error::Error for ImageExportError {}

#[cfg(test)]
#[cfg(feature = "export-png")]
mod tests {
    use super::*;
    use crate::coords::GeoCoord;
    use crate::geometry::{GeoCollection, GeoPoint};
    use crate::map::{Layer, Style};
    use crate::projection::Mercator;

    #[test]
    fn test_framebuffer_to_image() {
        let fb = Framebuffer::new(100, 100);
        let img = ImageExporter::framebuffer_to_image(&fb);

        assert_eq!(img.width(), 100);
        assert_eq!(img.height(), 100);
    }

    #[test]
    fn test_png_bytes() {
        let fb = Framebuffer::new(10, 10);
        let bytes = ImageExporter::to_png_bytes(&fb).unwrap();

        assert!(!bytes.is_empty());
        // PNG magic number
        assert_eq!(&bytes[0..4], &[137, 80, 78, 71]);
    }

    #[test]
    fn test_jpeg_bytes() {
        let fb = Framebuffer::new(10, 10);
        let bytes = ImageExporter::to_jpeg_bytes(&fb, 85).unwrap();

        assert!(!bytes.is_empty());
        // JPEG magic number
        assert_eq!(&bytes[0..2], &[255, 216]);
    }
}
