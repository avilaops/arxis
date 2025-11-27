//! Native color space conversions
//!
//! Pure Rust implementations of all color space transforms

use crate::native::math::clamp;

/// RGB to Grayscale (ITU-R BT.709)
#[inline]
pub fn rgb_to_gray(r: f32, g: f32, b: f32) -> f32 {
    0.2126 * r + 0.7152 * g + 0.0722 * b
}

/// RGB to HSV
pub fn rgb_to_hsv(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;

    // Hue
    let h = if delta < 1e-10 {
        0.0
    } else if (max - r).abs() < 1e-10 {
        60.0 * (((g - b) / delta) % 6.0)
    } else if (max - g).abs() < 1e-10 {
        60.0 * (((b - r) / delta) + 2.0)
    } else {
        60.0 * (((r - g) / delta) + 4.0)
    };

    let h = if h < 0.0 { h + 360.0 } else { h };

    // Saturation
    let s = if max < 1e-10 { 0.0 } else { delta / max };

    // Value
    let v = max;

    (h, s, v)
}

/// HSV to RGB
pub fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (f32, f32, f32) {
    let c = v * s;
    let h_prime = h / 60.0;
    let x = c * (1.0 - ((h_prime % 2.0) - 1.0).abs());
    let m = v - c;

    let (r_prime, g_prime, b_prime) = match h_prime as i32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        5 => (c, 0.0, x),
        _ => (c, x, 0.0),
    };

    (r_prime + m, g_prime + m, b_prime + m)
}

/// RGB to HSL
pub fn rgb_to_hsl(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;

    // Lightness
    let l = (max + min) / 2.0;

    // Saturation
    let s = if delta < 1e-10 {
        0.0
    } else if l < 0.5 {
        delta / (max + min)
    } else {
        delta / (2.0 - max - min)
    };

    // Hue (same as HSV)
    let h = if delta < 1e-10 {
        0.0
    } else if (max - r).abs() < 1e-10 {
        60.0 * (((g - b) / delta) % 6.0)
    } else if (max - g).abs() < 1e-10 {
        60.0 * (((b - r) / delta) + 2.0)
    } else {
        60.0 * (((r - g) / delta) + 4.0)
    };

    let h = if h < 0.0 { h + 360.0 } else { h };

    (h, s, l)
}

/// HSL to RGB
pub fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (f32, f32, f32) {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let h_prime = h / 60.0;
    let x = c * (1.0 - ((h_prime % 2.0) - 1.0).abs());
    let m = l - c / 2.0;

    let (r_prime, g_prime, b_prime) = match h_prime as i32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        5 => (c, 0.0, x),
        _ => (c, x, 0.0),
    };

    (r_prime + m, g_prime + m, b_prime + m)
}

/// RGB to YCbCr (JPEG color space)
pub fn rgb_to_ycbcr(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let y = 0.299 * r + 0.587 * g + 0.114 * b;
    let cb = -0.168736 * r - 0.331264 * g + 0.5 * b + 0.5;
    let cr = 0.5 * r - 0.418688 * g - 0.081312 * b + 0.5;
    (y, cb, cr)
}

/// YCbCr to RGB
pub fn ycbcr_to_rgb(y: f32, cb: f32, cr: f32) -> (f32, f32, f32) {
    let cb = cb - 0.5;
    let cr = cr - 0.5;

    let r = y + 1.402 * cr;
    let g = y - 0.344136 * cb - 0.714136 * cr;
    let b = y + 1.772 * cb;

    (
        clamp(r, 0.0, 1.0),
        clamp(g, 0.0, 1.0),
        clamp(b, 0.0, 1.0),
    )
}

/// RGB to LAB (CIE L*a*b*)
pub fn rgb_to_lab(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    // First convert to XYZ
    let (x, y, z) = rgb_to_xyz(r, g, b);

    // Then XYZ to LAB
    xyz_to_lab(x, y, z)
}

/// RGB to XYZ (D65 illuminant)
pub fn rgb_to_xyz(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    // Gamma correction
    let r = if r <= 0.04045 {
        r / 12.92
    } else {
        ((r + 0.055) / 1.055).powf(2.4)
    };

    let g = if g <= 0.04045 {
        g / 12.92
    } else {
        ((g + 0.055) / 1.055).powf(2.4)
    };

    let b = if b <= 0.04045 {
        b / 12.92
    } else {
        ((b + 0.055) / 1.055).powf(2.4)
    };

    // sRGB to XYZ (D65)
    let x = 0.4124564 * r + 0.3575761 * g + 0.1804375 * b;
    let y = 0.2126729 * r + 0.7151522 * g + 0.0721750 * b;
    let z = 0.0193339 * r + 0.1191920 * g + 0.9503041 * b;

    (x, y, z)
}

/// XYZ to LAB
pub fn xyz_to_lab(x: f32, y: f32, z: f32) -> (f32, f32, f32) {
    // D65 reference white
    const XN: f32 = 0.95047;
    const YN: f32 = 1.00000;
    const ZN: f32 = 1.08883;

    let x = x / XN;
    let y = y / YN;
    let z = z / ZN;

    let fx = lab_f(x);
    let fy = lab_f(y);
    let fz = lab_f(z);

    let l = 116.0 * fy - 16.0;
    let a = 500.0 * (fx - fy);
    let b = 200.0 * (fy - fz);

    (l, a, b)
}

#[inline]
fn lab_f(t: f32) -> f32 {
    const DELTA: f32 = 6.0 / 29.0;
    const DELTA_CUBED: f32 = DELTA * DELTA * DELTA;

    if t > DELTA_CUBED {
        t.cbrt()
    } else {
        t / (3.0 * DELTA * DELTA) + 4.0 / 29.0
    }
}

/// LAB to XYZ
pub fn lab_to_xyz(l: f32, a: f32, b: f32) -> (f32, f32, f32) {
    const XN: f32 = 0.95047;
    const YN: f32 = 1.00000;
    const ZN: f32 = 1.08883;

    let fy = (l + 16.0) / 116.0;
    let fx = a / 500.0 + fy;
    let fz = fy - b / 200.0;

    let x = XN * lab_f_inv(fx);
    let y = YN * lab_f_inv(fy);
    let z = ZN * lab_f_inv(fz);

    (x, y, z)
}

#[inline]
fn lab_f_inv(t: f32) -> f32 {
    const DELTA: f32 = 6.0 / 29.0;

    if t > DELTA {
        t * t * t
    } else {
        3.0 * DELTA * DELTA * (t - 4.0 / 29.0)
    }
}

/// XYZ to RGB
pub fn xyz_to_rgb(x: f32, y: f32, z: f32) -> (f32, f32, f32) {
    // XYZ to linear RGB
    let r = 3.2404542 * x - 1.5371385 * y - 0.4985314 * z;
    let g = -0.9692660 * x + 1.8760108 * y + 0.0415560 * z;
    let b = 0.0556434 * x - 0.2040259 * y + 1.0572252 * z;

    // Gamma correction
    let r = if r <= 0.0031308 {
        12.92 * r
    } else {
        1.055 * r.powf(1.0 / 2.4) - 0.055
    };

    let g = if g <= 0.0031308 {
        12.92 * g
    } else {
        1.055 * g.powf(1.0 / 2.4) - 0.055
    };

    let b = if b <= 0.0031308 {
        12.92 * b
    } else {
        1.055 * b.powf(1.0 / 2.4) - 0.055
    };

    (
        clamp(r, 0.0, 1.0),
        clamp(g, 0.0, 1.0),
        clamp(b, 0.0, 1.0),
    )
}

/// LAB to RGB (composite function)
pub fn lab_to_rgb(l: f32, a: f32, b: f32) -> (f32, f32, f32) {
    let (x, y, z) = lab_to_xyz(l, a, b);
    xyz_to_rgb(x, y, z)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: f32, b: f32, epsilon: f32) -> bool {
        (a - b).abs() < epsilon
    }

    #[test]
    fn test_rgb_to_gray() {
        let gray = rgb_to_gray(1.0, 0.0, 0.0);
        assert!(gray > 0.2 && gray < 0.22);
    }

    #[test]
    fn test_rgb_hsv_roundtrip() {
        let (r, g, b) = (0.5, 0.7, 0.3);
        let (h, s, v) = rgb_to_hsv(r, g, b);
        let (r2, g2, b2) = hsv_to_rgb(h, s, v);

        assert!(approx_eq(r, r2, 0.01));
        assert!(approx_eq(g, g2, 0.01));
        assert!(approx_eq(b, b2, 0.01));
    }

    #[test]
    fn test_rgb_hsl_roundtrip() {
        let (r, g, b) = (0.8, 0.2, 0.6);
        let (h, s, l) = rgb_to_hsl(r, g, b);
        let (r2, g2, b2) = hsl_to_rgb(h, s, l);

        assert!(approx_eq(r, r2, 0.01));
        assert!(approx_eq(g, g2, 0.01));
        assert!(approx_eq(b, b2, 0.01));
    }

    #[test]
    fn test_rgb_ycbcr_roundtrip() {
        let (r, g, b) = (0.5, 0.5, 0.5);
        let (y, cb, cr) = rgb_to_ycbcr(r, g, b);
        let (r2, g2, b2) = ycbcr_to_rgb(y, cb, cr);

        assert!(approx_eq(r, r2, 0.01));
        assert!(approx_eq(g, g2, 0.01));
        assert!(approx_eq(b, b2, 0.01));
    }

    #[test]
    fn test_rgb_lab_roundtrip() {
        let (r, g, b) = (0.7, 0.3, 0.9);
        let (l, a, b_val) = rgb_to_lab(r, g, b);
        let (r2, g2, b2) = lab_to_rgb(l, a, b_val);

        assert!(approx_eq(r, r2, 0.02));
        assert!(approx_eq(g, g2, 0.02));
        assert!(approx_eq(b, b2, 0.02));
    }
}
