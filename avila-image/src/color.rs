//! Color types and pixel traits

/// RGB color type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgb(pub [u8; 3]);

impl Rgb {
    /// Create a new RGB color
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Rgb([r, g, b])
    }

    /// Get the red channel
    pub fn r(&self) -> u8 {
        self.0[0]
    }

    /// Get the green channel
    pub fn g(&self) -> u8 {
        self.0[1]
    }

    /// Get the blue channel
    pub fn b(&self) -> u8 {
        self.0[2]
    }

    /// Get channels as slice
    pub fn channels(&self) -> &[u8] {
        &self.0
    }
}

/// RGBA color type with alpha channel
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgba(pub [u8; 4]);

impl Rgba {
    /// Create a new RGBA color
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Rgba([r, g, b, a])
    }

    /// Get the red channel
    pub fn r(&self) -> u8 {
        self.0[0]
    }

    /// Get the green channel
    pub fn g(&self) -> u8 {
        self.0[1]
    }

    /// Get the blue channel
    pub fn b(&self) -> u8 {
        self.0[2]
    }

    /// Get the alpha channel
    pub fn a(&self) -> u8 {
        self.0[3]
    }

    /// Get channels as slice
    pub fn channels(&self) -> &[u8] {
        &self.0
    }

    /// Convert to RGB by discarding alpha
    pub fn to_rgb(&self) -> Rgb {
        Rgb([self.0[0], self.0[1], self.0[2]])
    }
}

/// Grayscale color type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Gray(pub [u8; 1]);

impl Gray {
    /// Create a new grayscale value
    pub fn new(value: u8) -> Self {
        Gray([value])
    }

    /// Get the value
    pub fn value(&self) -> u8 {
        self.0[0]
    }

    /// Convert to RGB
    pub fn to_rgb(&self) -> Rgb {
        let v = self.0[0];
        Rgb([v, v, v])
    }
}

/// Trait for pixel types
pub trait Pixel: Copy + Clone {
    /// Number of channels
    const CHANNEL_COUNT: usize;

    /// Get channels as slice
    fn channels(&self) -> &[u8];

    /// Create from channels
    fn from_channels(channels: &[u8]) -> Self;
}

impl Pixel for Rgb {
    const CHANNEL_COUNT: usize = 3;

    fn channels(&self) -> &[u8] {
        &self.0
    }

    fn from_channels(channels: &[u8]) -> Self {
        Rgb([channels[0], channels[1], channels[2]])
    }
}

impl Pixel for Rgba {
    const CHANNEL_COUNT: usize = 4;

    fn channels(&self) -> &[u8] {
        &self.0
    }

    fn from_channels(channels: &[u8]) -> Self {
        Rgba([channels[0], channels[1], channels[2], channels[3]])
    }
}

impl Pixel for Gray {
    const CHANNEL_COUNT: usize = 1;

    fn channels(&self) -> &[u8] {
        &self.0
    }

    fn from_channels(channels: &[u8]) -> Self {
        Gray([channels[0]])
    }
}

/// Common color constants
impl Rgb {
    /// Black color
    pub const BLACK: Rgb = Rgb([0, 0, 0]);
    /// White color
    pub const WHITE: Rgb = Rgb([255, 255, 255]);
    /// Red color
    pub const RED: Rgb = Rgb([255, 0, 0]);
    /// Green color
    pub const GREEN: Rgb = Rgb([0, 255, 0]);
    /// Blue color
    pub const BLUE: Rgb = Rgb([0, 0, 255]);
}
