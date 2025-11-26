//! Rendering algorithms for rasterization
//!
//! This module implements fundamental computer graphics algorithms:
//! - Bresenham's line algorithm
//! - Scanline polygon fill
//! - Anti-aliased line drawing (Xiaolin Wu)
//! - Circle drawing (Midpoint)

use crate::coords::CartesianCoord;

/// RGB color
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub const fn from_hex(hex: u32) -> Self {
        Self {
            r: ((hex >> 16) & 0xFF) as u8,
            g: ((hex >> 8) & 0xFF) as u8,
            b: (hex & 0xFF) as u8,
        }
    }

    pub const fn to_hex(&self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }

    /// Blend with another color
    pub fn blend(&self, other: &Color, alpha: f32) -> Color {
        Color {
            r: (self.r as f32 * (1.0 - alpha) + other.r as f32 * alpha) as u8,
            g: (self.g as f32 * (1.0 - alpha) + other.g as f32 * alpha) as u8,
            b: (self.b as f32 * (1.0 - alpha) + other.b as f32 * alpha) as u8,
        }
    }

    // Common colors
    pub const BLACK: Color = Color::new(0, 0, 0);
    pub const WHITE: Color = Color::new(255, 255, 255);
    pub const RED: Color = Color::new(255, 0, 0);
    pub const GREEN: Color = Color::new(0, 255, 0);
    pub const BLUE: Color = Color::new(0, 0, 255);
    pub const YELLOW: Color = Color::new(255, 255, 0);
    pub const CYAN: Color = Color::new(0, 255, 255);
    pub const MAGENTA: Color = Color::new(255, 0, 255);
    pub const GRAY: Color = Color::new(128, 128, 128);
}

/// Framebuffer for rendering
pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>, // RGB triplets
}

impl Framebuffer {
    /// Create a new framebuffer with background color
    pub fn new(width: u32, height: u32, background: Color) -> Self {
        let size = (width * height * 3) as usize;
        let mut data = vec![0u8; size];

        // Fill with background
        for i in 0..(width * height) as usize {
            let offset = i * 3;
            data[offset] = background.r;
            data[offset + 1] = background.g;
            data[offset + 2] = background.b;
        }

        Self { width, height, data }
    }

    /// Set pixel color
    #[inline]
    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        if x < self.width && y < self.height {
            let offset = ((y * self.width + x) * 3) as usize;
            self.data[offset] = color.r;
            self.data[offset + 1] = color.g;
            self.data[offset + 2] = color.b;
        }
    }

    /// Get pixel color
    #[inline]
    pub fn get_pixel(&self, x: u32, y: u32) -> Option<Color> {
        if x < self.width && y < self.height {
            let offset = ((y * self.width + x) * 3) as usize;
            Some(Color::new(
                self.data[offset],
                self.data[offset + 1],
                self.data[offset + 2],
            ))
        } else {
            None
        }
    }

    /// Blend pixel with existing color
    #[inline]
    pub fn blend_pixel(&mut self, x: u32, y: u32, color: Color, alpha: f32) {
        if let Some(existing) = self.get_pixel(x, y) {
            let blended = existing.blend(&color, alpha);
            self.set_pixel(x, y, blended);
        }
    }

    /// Clear with color
    pub fn clear(&mut self, color: Color) {
        for i in 0..(self.width * self.height) as usize {
            let offset = i * 3;
            self.data[offset] = color.r;
            self.data[offset + 1] = color.g;
            self.data[offset + 2] = color.b;
        }
    }

    /// Export as PPM (Portable Pixmap)
    pub fn to_ppm(&self) -> String {
        let mut ppm = format!("P3\n{} {}\n255\n", self.width, self.height);

        for chunk in self.data.chunks(3) {
            ppm.push_str(&format!("{} {} {} ", chunk[0], chunk[1], chunk[2]));
        }

        ppm
    }

    /// Export as raw RGB data
    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }
}

/// Bresenham's line drawing algorithm
///
/// Fast, integer-only algorithm for drawing straight lines
pub fn draw_line_bresenham(
    fb: &mut Framebuffer,
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    color: Color,
) {
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };

    let mut err = dx + dy;
    let mut x = x0;
    let mut y = y0;

    loop {
        if x >= 0 && y >= 0 {
            fb.set_pixel(x as u32, y as u32, color);
        }

        if x == x1 && y == y1 {
            break;
        }

        let e2 = 2 * err;

        if e2 >= dy {
            if x == x1 {
                break;
            }
            err += dy;
            x += sx;
        }

        if e2 <= dx {
            if y == y1 {
                break;
            }
            err += dx;
            y += sy;
        }
    }
}

/// Draw line from Cartesian coordinates
pub fn draw_line(fb: &mut Framebuffer, p1: &CartesianCoord, p2: &CartesianCoord, color: Color) {
    let (x0, y0) = p1.to_i32();
    let (x1, y1) = p2.to_i32();
    draw_line_bresenham(fb, x0, y0, x1, y1, color);
}

/// Xiaolin Wu's anti-aliased line algorithm
pub fn draw_line_aa(
    fb: &mut Framebuffer,
    x0: f64,
    y0: f64,
    x1: f64,
    y1: f64,
    color: Color,
) {
    let steep = (y1 - y0).abs() > (x1 - x0).abs();

    let (x0, y0, x1, y1) = if steep {
        (y0, x0, y1, x1)
    } else {
        (x0, y0, x1, y1)
    };

    let (x0, y0, x1, y1) = if x0 > x1 {
        (x1, y1, x0, y0)
    } else {
        (x0, y0, x1, y1)
    };

    let dx = x1 - x0;
    let dy = y1 - y0;
    let gradient = if dx.abs() < 1e-10 { 1.0 } else { dy / dx };

    // First endpoint
    let xend = x0.round();
    let yend = y0 + gradient * (xend - x0);
    let xgap = 1.0 - (x0 + 0.5).fract();
    let xpxl1 = xend as i32;
    let ypxl1 = yend.floor() as i32;

    if steep {
        fb.blend_pixel(ypxl1 as u32, xpxl1 as u32, color, (1.0 - yend.fract()) as f32 * xgap as f32);
        fb.blend_pixel((ypxl1 + 1) as u32, xpxl1 as u32, color, yend.fract() as f32 * xgap as f32);
    } else {
        fb.blend_pixel(xpxl1 as u32, ypxl1 as u32, color, (1.0 - yend.fract()) as f32 * xgap as f32);
        fb.blend_pixel(xpxl1 as u32, (ypxl1 + 1) as u32, color, yend.fract() as f32 * xgap as f32);
    }

    let mut intery = yend + gradient;

    // Second endpoint
    let xend = x1.round();
    let yend = y1 + gradient * (xend - x1);
    let xgap = (x1 + 0.5).fract();
    let xpxl2 = xend as i32;
    let ypxl2 = yend.floor() as i32;

    if steep {
        fb.blend_pixel(ypxl2 as u32, xpxl2 as u32, color, (1.0 - yend.fract()) as f32 * xgap as f32);
        fb.blend_pixel((ypxl2 + 1) as u32, xpxl2 as u32, color, yend.fract() as f32 * xgap as f32);
    } else {
        fb.blend_pixel(xpxl2 as u32, ypxl2 as u32, color, (1.0 - yend.fract()) as f32 * xgap as f32);
        fb.blend_pixel(xpxl2 as u32, (ypxl2 + 1) as u32, color, yend.fract() as f32 * xgap as f32);
    }

    // Main loop
    for x in (xpxl1 + 1)..xpxl2 {
        if steep {
            fb.blend_pixel(intery.floor() as u32, x as u32, color, 1.0 - intery.fract() as f32);
            fb.blend_pixel((intery.floor() + 1.0) as u32, x as u32, color, intery.fract() as f32);
        } else {
            fb.blend_pixel(x as u32, intery.floor() as u32, color, 1.0 - intery.fract() as f32);
            fb.blend_pixel(x as u32, (intery.floor() + 1.0) as u32, color, intery.fract() as f32);
        }
        intery += gradient;
    }
}

/// Scanline polygon fill algorithm
pub fn fill_polygon(fb: &mut Framebuffer, points: &[CartesianCoord], color: Color) {
    if points.len() < 3 {
        return;
    }

    // Find bounding box
    let mut min_y = f64::INFINITY;
    let mut max_y = f64::NEG_INFINITY;

    for p in points {
        min_y = min_y.min(p.y);
        max_y = max_y.max(p.y);
    }

    let min_y = min_y.max(0.0) as u32;
    let max_y = max_y.min(fb.height as f64 - 1.0) as u32;

    // Scanline fill
    for y in min_y..=max_y {
        let mut intersections = Vec::new();

        // Find intersections with polygon edges
        for i in 0..points.len() {
            let j = (i + 1) % points.len();
            let p1 = &points[i];
            let p2 = &points[j];

            let y_f64 = y as f64;

            if (p1.y <= y_f64 && p2.y > y_f64) || (p2.y <= y_f64 && p1.y > y_f64) {
                let x = p1.x + (y_f64 - p1.y) * (p2.x - p1.x) / (p2.y - p1.y);
                intersections.push(x);
            }
        }

        intersections.sort_by(|a, b| a.partial_cmp(b).unwrap());

        // Fill between pairs of intersections
        for i in (0..intersections.len()).step_by(2) {
            if i + 1 < intersections.len() {
                let x_start = intersections[i].max(0.0) as u32;
                let x_end = intersections[i + 1].min(fb.width as f64 - 1.0) as u32;

                for x in x_start..=x_end {
                    fb.set_pixel(x, y, color);
                }
            }
        }
    }
}

/// Draw polyline (connected line segments)
pub fn draw_polyline(fb: &mut Framebuffer, points: &[CartesianCoord], color: Color) {
    for i in 1..points.len() {
        draw_line(fb, &points[i - 1], &points[i], color);
    }
}

/// Draw circle using Midpoint Circle Algorithm
pub fn draw_circle(fb: &mut Framebuffer, cx: i32, cy: i32, radius: i32, color: Color) {
    let mut x = radius;
    let mut y = 0;
    let mut err = 0;

    while x >= y {
        // Draw 8 octants
        let points = [
            (cx + x, cy + y),
            (cx + y, cy + x),
            (cx - y, cy + x),
            (cx - x, cy + y),
            (cx - x, cy - y),
            (cx - y, cy - x),
            (cx + y, cy - x),
            (cx + x, cy - y),
        ];

        for &(px, py) in &points {
            if px >= 0 && py >= 0 {
                fb.set_pixel(px as u32, py as u32, color);
            }
        }

        y += 1;
        if err <= 0 {
            err += 2 * y + 1;
        }
        if err > 0 {
            x -= 1;
            err -= 2 * x + 1;
        }
    }
}

/// Fill circle
pub fn fill_circle(fb: &mut Framebuffer, cx: i32, cy: i32, radius: i32, color: Color) {
    let r_sq = radius * radius;

    for y in -radius..=radius {
        for x in -radius..=radius {
            if x * x + y * y <= r_sq {
                let px = cx + x;
                let py = cy + y;
                if px >= 0 && py >= 0 {
                    fb.set_pixel(px as u32, py as u32, color);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_framebuffer_creation() {
        let fb = Framebuffer::new(100, 100, Color::WHITE);
        assert_eq!(fb.width, 100);
        assert_eq!(fb.height, 100);
        assert_eq!(fb.data.len(), 100 * 100 * 3);
    }

    #[test]
    fn test_set_pixel() {
        let mut fb = Framebuffer::new(10, 10, Color::BLACK);
        fb.set_pixel(5, 5, Color::RED);
        assert_eq!(fb.get_pixel(5, 5), Some(Color::RED));
    }

    #[test]
    fn test_draw_line() {
        let mut fb = Framebuffer::new(100, 100, Color::WHITE);
        draw_line_bresenham(&mut fb, 0, 0, 99, 99, Color::BLACK);

        // Check that diagonal was drawn
        assert_eq!(fb.get_pixel(0, 0), Some(Color::BLACK));
        assert_eq!(fb.get_pixel(50, 50), Some(Color::BLACK));
        assert_eq!(fb.get_pixel(99, 99), Some(Color::BLACK));
    }

    #[test]
    fn test_color_blend() {
        let c1 = Color::BLACK;
        let c2 = Color::WHITE;
        let blended = c1.blend(&c2, 0.5);
        assert_eq!(blended.r, 127);
        assert_eq!(blended.g, 127);
        assert_eq!(blended.b, 127);
    }
}
