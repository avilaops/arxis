/// Core image processing module with background removal and smart resizing
use anyhow::{Context, Result};
use image::{imageops::FilterType, DynamicImage, ImageBuffer, ImageEncoder, Rgba, RgbaImage};
use std::path::Path;

use crate::config::PlatformConfig;

/// Detects the content bounds of an image (non-transparent pixels)
pub fn get_content_bounds(img: &RgbaImage) -> (u32, u32, u32, u32) {
    let (width, height) = img.dimensions();

    let mut min_x = width;
    let mut min_y = height;
    let mut max_x = 0u32;
    let mut max_y = 0u32;

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            if pixel[3] > 0 {
                // Alpha > 0 (not transparent)
                min_x = min_x.min(x);
                min_y = min_y.min(y);
                max_x = max_x.max(x);
                max_y = max_y.max(y);
            }
        }
    }

    // If image is fully transparent, return full bounds
    if min_x > max_x || min_y > max_y {
        return (0, 0, width, height);
    }

    (min_x, min_y, max_x + 1, max_y + 1)
}

/// Removes background from an image
/// Uses a simple threshold-based approach (can be upgraded to ML model)
pub fn remove_background(img: &DynamicImage) -> Result<RgbaImage> {
    let rgba = img.to_rgba8();
    let (width, height) = rgba.dimensions();
    let mut output = RgbaImage::new(width, height);

    // Simple background removal: detect edge color and remove similar colors
    // This is a placeholder - in production, you'd use rembg or similar
    let edge_color = rgba.get_pixel(0, 0);
    let threshold = 30u16; // Color similarity threshold

    for y in 0..height {
        for x in 0..width {
            let pixel = rgba.get_pixel(x, y);

            // Calculate color distance from edge color
            let distance = color_distance(pixel, edge_color);

            if distance > threshold {
                // Keep pixel
                output.put_pixel(x, y, *pixel);
            } else {
                // Make transparent
                output.put_pixel(x, y, Rgba([0, 0, 0, 0]));
            }
        }
    }

    Ok(output)
}

/// Calculate color distance between two pixels
fn color_distance(a: &Rgba<u8>, b: &Rgba<u8>) -> u16 {
    let r_diff = (a[0] as i16 - b[0] as i16).abs() as u16;
    let g_diff = (a[1] as i16 - b[1] as i16).abs() as u16;
    let b_diff = (a[2] as i16 - b[2] as i16).abs() as u16;
    r_diff + g_diff + b_diff
}

/// Smart resize with padding and centering
pub fn resize_with_padding(
    img: &RgbaImage,
    target_width: u32,
    target_height: u32,
    padding_percent: u8,
) -> Result<RgbaImage> {
    // Get content bounds
    let (left, top, right, bottom) = get_content_bounds(img);
    let content_width = right - left;
    let content_height = bottom - top;

    if content_width == 0 || content_height == 0 {
        anyhow::bail!("Image has no visible content");
    }

    // Calculate available space after padding
    let padding_x = (target_width * padding_percent as u32) / 100;
    let padding_y = (target_height * padding_percent as u32) / 100;
    let available_width = target_width.saturating_sub(2 * padding_x);
    let available_height = target_height.saturating_sub(2 * padding_y);

    // Calculate scale to fit content in available space
    let scale_x = available_width as f32 / img.width() as f32;
    let scale_y = available_height as f32 / img.height() as f32;
    let scale = scale_x.min(scale_y);

    // Calculate new dimensions
    let new_width = (img.width() as f32 * scale) as u32;
    let new_height = (img.height() as f32 * scale) as u32;

    // Resize the entire image
    let resized = image::imageops::resize(img, new_width, new_height, FilterType::Lanczos3);

    // Create output canvas with transparency
    let mut output = RgbaImage::from_pixel(target_width, target_height, Rgba([0, 0, 0, 0]));

    // Center the resized image
    let x_offset = (target_width - new_width) / 2;
    let y_offset = (target_height - new_height) / 2;

    // Copy resized image to output
    image::imageops::overlay(&mut output, &resized, x_offset as i64, y_offset as i64);

    Ok(output)
}

/// Load an image from a file path
pub fn load_image<P: AsRef<Path>>(path: P) -> Result<DynamicImage> {
    image::open(path.as_ref())
        .with_context(|| format!("Failed to load image: {}", path.as_ref().display()))
}

/// Save an image to a file path
pub fn save_image<P: AsRef<Path>>(img: &RgbaImage, path: P, quality: u8) -> Result<()> {
    // Create parent directories if they don't exist
    if let Some(parent) = path.as_ref().parent() {
        std::fs::create_dir_all(parent)?;
    }

    let encoder = image::codecs::png::PngEncoder::new(
        std::fs::File::create(path.as_ref())
            .with_context(|| format!("Failed to create file: {}", path.as_ref().display()))?,
    );

    encoder
        .write_image(
            img.as_raw(),
            img.width(),
            img.height(),
            image::ExtendedColorType::Rgba8,
        )
        .with_context(|| format!("Failed to write image: {}", path.as_ref().display()))?;

    Ok(())
}

/// Process a single image for a specific platform and size
pub fn process_icon(
    img: &RgbaImage,
    width: u32,
    height: u32,
    padding_percent: u8,
) -> Result<RgbaImage> {
    resize_with_padding(img, width, height, padding_percent)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_content_bounds_empty_image() {
        let img = RgbaImage::new(100, 100);
        let (left, top, right, bottom) = get_content_bounds(&img);
        assert_eq!((left, top, right, bottom), (0, 0, 100, 100));
    }

    #[test]
    fn test_content_bounds_centered_pixel() {
        let mut img = RgbaImage::new(100, 100);
        img.put_pixel(50, 50, Rgba([255, 0, 0, 255]));
        let (left, top, right, bottom) = get_content_bounds(&img);
        assert_eq!((left, top, right, bottom), (50, 50, 51, 51));
    }

    #[test]
    fn test_color_distance_same_color() {
        let a = Rgba([100, 150, 200, 255]);
        let b = Rgba([100, 150, 200, 255]);
        assert_eq!(color_distance(&a, &b), 0);
    }

    #[test]
    fn test_color_distance_different_colors() {
        let a = Rgba([0, 0, 0, 255]);
        let b = Rgba([255, 255, 255, 255]);
        assert_eq!(color_distance(&a, &b), 765); // 255*3
    }
}
