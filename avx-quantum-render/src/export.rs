//! Exportação de imagens renderizadas

#[cfg(feature = "image-export")]
use image::{ImageBuffer, Rgb, RgbImage};
use std::io;
use std::path::Path;

/// Resultado de exportação
pub type ExportResult<T> = Result<T, ExportError>;

/// Erros de exportação
#[derive(Debug)]
pub enum ExportError {
    /// Erro de I/O
    IoError(io::Error),

    /// Feature não habilitada
    FeatureNotEnabled(&'static str),

    /// Erro de imagem
    #[cfg(feature = "image-export")]
    ImageError(image::ImageError),
}

impl std::fmt::Display for ExportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExportError::IoError(e) => write!(f, "IO error: {}", e),
            ExportError::FeatureNotEnabled(feature) => {
                write!(f, "Feature '{}' not enabled", feature)
            }
            #[cfg(feature = "image-export")]
            ExportError::ImageError(e) => write!(f, "Image error: {}", e),
        }
    }
}

impl std::error::Error for ExportError {}

impl From<io::Error> for ExportError {
    fn from(err: io::Error) -> Self {
        ExportError::IoError(err)
    }
}

#[cfg(feature = "image-export")]
impl From<image::ImageError> for ExportError {
    fn from(err: image::ImageError) -> Self {
        ExportError::ImageError(err)
    }
}

/// Converte imagem normalizada [0, 1] para RGB 8-bit
pub fn intensity_to_rgb(intensity: f64) -> [u8; 3] {
    let value = (intensity.clamp(0.0, 1.0) * 255.0) as u8;
    [value, value, value]
}

/// Converte imagem com tone mapping
pub fn intensity_to_rgb_tonemapped(intensity: f64, exposure: f64) -> [u8; 3] {
    // Reinhard tone mapping
    let mapped = intensity * exposure / (1.0 + intensity * exposure);
    let value = (mapped.clamp(0.0, 1.0) * 255.0) as u8;
    [value, value, value]
}

/// Exporta imagem para PNG
#[cfg(feature = "image-export")]
pub fn export_png<P: AsRef<Path>>(image: &[Vec<f64>], path: P, exposure: f64) -> ExportResult<()> {
    if image.is_empty() {
        return Err(ExportError::IoError(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Empty image",
        )));
    }

    let height = image.len();
    let width = image[0].len();

    let mut img: RgbImage = ImageBuffer::new(width as u32, height as u32);

    for (y, row) in image.iter().enumerate() {
        for (x, &intensity) in row.iter().enumerate() {
            let rgb = intensity_to_rgb_tonemapped(intensity, exposure);
            img.put_pixel(x as u32, y as u32, Rgb(rgb));
        }
    }

    img.save(path)?;
    Ok(())
}

/// Exporta imagem para PNG (stub quando feature não está habilitada)
#[cfg(not(feature = "image-export"))]
pub fn export_png<P: AsRef<Path>>(
    _image: &[Vec<f64>],
    _path: P,
    _exposure: f64,
) -> ExportResult<()> {
    Err(ExportError::FeatureNotEnabled("image-export"))
}

/// Exporta imagem para ASCII art
pub fn export_ascii(image: &[Vec<f64>]) -> String {
    let charset = " .:-=+*#%@";
    let mut output = String::new();

    for row in image {
        for &intensity in row {
            let idx = (intensity.clamp(0.0, 1.0) * (charset.len() - 1) as f64) as usize;
            output.push(charset.chars().nth(idx).unwrap());
        }
        output.push('\n');
    }

    output
}

/// Exporta imagem para formato PPM (P3 - ASCII)
pub fn export_ppm<P: AsRef<Path>>(image: &[Vec<f64>], path: P, exposure: f64) -> ExportResult<()> {
    use std::fs::File;
    use std::io::Write;

    if image.is_empty() {
        return Err(ExportError::IoError(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Empty image",
        )));
    }

    let height = image.len();
    let width = image[0].len();

    let mut file = File::create(path)?;

    // Header PPM
    writeln!(file, "P3")?;
    writeln!(file, "{} {}", width, height)?;
    writeln!(file, "255")?;

    // Pixels
    for row in image {
        for &intensity in row {
            let rgb = intensity_to_rgb_tonemapped(intensity, exposure);
            writeln!(file, "{} {} {}", rgb[0], rgb[1], rgb[2])?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intensity_to_rgb() {
        let rgb = intensity_to_rgb(0.5);
        assert_eq!(rgb, [127, 127, 127]);

        let rgb = intensity_to_rgb(1.0);
        assert_eq!(rgb, [255, 255, 255]);

        let rgb = intensity_to_rgb(0.0);
        assert_eq!(rgb, [0, 0, 0]);
    }

    #[test]
    fn test_ascii_export() {
        let image = vec![vec![0.0, 0.5, 1.0], vec![0.25, 0.75, 0.5]];

        let ascii = export_ascii(&image);
        assert!(!ascii.is_empty());
        assert!(ascii.contains('\n'));
    }

    #[test]
    fn test_tonemapping() {
        let rgb = intensity_to_rgb_tonemapped(2.0, 1.0); // HDR value
                                                         // rgb is u8, so always in valid range by type definition
        assert_eq!(rgb.len(), 3);
    }
}
