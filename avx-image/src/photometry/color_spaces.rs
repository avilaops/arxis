//! Color space conversions

use crate::core::ImageBuffer;
use crate::Result;

/// Convert RGB to HSV color space
pub fn rgb_to_hsv(img: &ImageBuffer) -> Result<ImageBuffer> {
    let mut hsv = img.clone();

    for i in (0..img.data.len()).step_by(3) {
        let r = img.data[i];
        let g = img.data[i + 1];
        let b = img.data[i + 2];

        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let delta = max - min;

        // Hue
        let h = if delta == 0.0 {
            0.0
        } else if max == r {
            60.0 * (((g - b) / delta) % 6.0)
        } else if max == g {
            60.0 * (((b - r) / delta) + 2.0)
        } else {
            60.0 * (((r - g) / delta) + 4.0)
        };

        // Saturation
        let s = if max == 0.0 { 0.0 } else { delta / max };

        // Value
        let v = max;

        hsv.data[i] = h / 360.0;
        hsv.data[i + 1] = s;
        hsv.data[i + 2] = v;
    }

    Ok(hsv)
}

/// Convert RGB to LAB color space
pub fn rgb_to_lab(img: &ImageBuffer) -> Result<ImageBuffer> {
    // TODO: Implement RGB → XYZ → LAB conversion
    Ok(img.clone())
}

/// Convert RGB to YCbCr
pub fn rgb_to_ycbcr(img: &ImageBuffer) -> Result<ImageBuffer> {
    let mut ycbcr = img.clone();

    for i in (0..img.data.len()).step_by(3) {
        let r = img.data[i] * 255.0;
        let g = img.data[i + 1] * 255.0;
        let b = img.data[i + 2] * 255.0;

        let y = 0.299 * r + 0.587 * g + 0.114 * b;
        let cb = 128.0 - 0.168736 * r - 0.331264 * g + 0.5 * b;
        let cr = 128.0 + 0.5 * r - 0.418688 * g - 0.081312 * b;

        ycbcr.data[i] = y / 255.0;
        ycbcr.data[i + 1] = cb / 255.0;
        ycbcr.data[i + 2] = cr / 255.0;
    }

    Ok(ycbcr)
}
