//! Illumination estimation and white balance

use crate::core::ImageBuffer;
use crate::photometry::Illuminant;
use crate::Result;

/// Estimate illuminant using gray world assumption
pub fn estimate_illuminant(img: &ImageBuffer) -> Result<Illuminant> {
    if img.channels != 3 {
        return Ok(Illuminant {
            color_temperature: 6500.0,
            tint: 0.0,
            rgb_multipliers: (1.0, 1.0, 1.0),
        });
    }

    // Gray world: average of each channel
    let mut r_sum = 0.0;
    let mut g_sum = 0.0;
    let mut b_sum = 0.0;
    let n_pixels = (img.width * img.height) as f32;

    for i in (0..img.data.len()).step_by(3) {
        r_sum += img.data[i];
        g_sum += img.data[i + 1];
        b_sum += img.data[i + 2];
    }

    let r_avg = r_sum / n_pixels;
    let g_avg = g_sum / n_pixels;
    let b_avg = b_sum / n_pixels;

    // Normalize by green channel
    let r_mult = g_avg / r_avg.max(1e-6);
    let g_mult = 1.0;
    let b_mult = g_avg / b_avg.max(1e-6);

    Ok(Illuminant {
        color_temperature: estimate_temperature(r_avg, g_avg, b_avg),
        tint: 0.0,
        rgb_multipliers: (r_mult, g_mult, b_mult),
    })
}

/// Apply white balance correction
pub fn apply_white_balance(img: &ImageBuffer, illuminant: &Illuminant) -> Result<ImageBuffer> {
    let mut corrected = img.clone();

    for i in (0..img.data.len()).step_by(3) {
        corrected.data[i] = (img.data[i] * illuminant.rgb_multipliers.0).clamp(0.0, 1.0);
        corrected.data[i + 1] = (img.data[i + 1] * illuminant.rgb_multipliers.1).clamp(0.0, 1.0);
        corrected.data[i + 2] = (img.data[i + 2] * illuminant.rgb_multipliers.2).clamp(0.0, 1.0);
    }

    Ok(corrected)
}

/// Estimate color temperature from RGB averages
fn estimate_temperature(r: f32, g: f32, b: f32) -> f32 {
    // Simplified McCamy's formula
    let x = (-0.14282 * r + 1.54924 * g - 0.95641 * b) / (0.32466 * r + 0.57837 * g + 0.09567 * b);
    let n = (x - 0.3320) / (0.1858 - 0.2008 * x);
    437.0 * n.powi(3) + 3601.0 * n.powi(2) + 6861.0 * n + 5517.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_illuminant_estimation() {
        let mut img = ImageBuffer::new(100, 100, 3);

        // Fill with non-zero values to get valid estimation
        for i in 0..img.data.len() {
            img.data[i] = 0.5; // Mid-gray
        }

        let illuminant = estimate_illuminant(&img).unwrap();
        assert!(illuminant.color_temperature > 0.0);
        assert!(illuminant.rgb_multipliers.0 > 0.0);
        assert!(illuminant.rgb_multipliers.1 > 0.0);
        assert!(illuminant.rgb_multipliers.2 > 0.0);
    }
}
