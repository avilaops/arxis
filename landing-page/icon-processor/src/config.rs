/// Platform-specific icon configurations
/// Updated for 2025 standards

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IconSize {
    pub width: u32,
    pub height: u32,
    pub platform: String,
}

impl IconSize {
    pub fn new(width: u32, height: u32, platform: &str) -> Self {
        Self {
            width,
            height,
            platform: platform.to_string(),
        }
    }

    pub fn is_square(&self) -> bool {
        self.width == self.height
    }
}

#[derive(Debug, Clone)]
pub struct PlatformConfig {
    pub name: String,
    pub sizes: Vec<(u32, u32)>,
    pub padding_percent: u8,
}

impl PlatformConfig {
    pub fn favicon() -> Self {
        Self {
            name: "favicon".to_string(),
            sizes: vec![(16, 16), (32, 32), (48, 48), (64, 64)],
            padding_percent: 5,
        }
    }

    pub fn apple() -> Self {
        Self {
            name: "apple".to_string(),
            sizes: vec![
                (57, 57),
                (60, 60),
                (72, 72),
                (76, 76),
                (120, 120),
                (144, 144),
                (152, 152),
                (167, 167),
                (180, 180),
                (1024, 1024),
            ],
            padding_percent: 5,
        }
    }

    pub fn android() -> Self {
        Self {
            name: "android".to_string(),
            sizes: vec![
                (36, 36),
                (48, 48),
                (72, 72),
                (96, 96),
                (144, 144),
                (192, 192),
                (512, 512),
            ],
            padding_percent: 5,
        }
    }

    pub fn windows() -> Self {
        Self {
            name: "windows".to_string(),
            sizes: vec![
                (44, 44),
                (70, 70),
                (150, 150),
                (310, 150),
                (310, 310),
            ],
            padding_percent: 5,
        }
    }

    pub fn macos() -> Self {
        Self {
            name: "mac".to_string(),
            sizes: vec![
                (16, 16),
                (32, 32),
                (64, 64),
                (128, 128),
                (256, 256),
                (512, 512),
                (1024, 1024),
            ],
            padding_percent: 5,
        }
    }

    pub fn social() -> Self {
        Self {
            name: "social".to_string(),
            sizes: vec![
                (300, 300),   // LinkedIn
                (400, 400),   // Twitter/X
                (720, 720),   // Facebook
                (800, 800),   // YouTube
                (1080, 1080), // Instagram
            ],
            padding_percent: 2,
        }
    }

    pub fn all_platforms() -> Vec<Self> {
        vec![
            Self::favicon(),
            Self::apple(),
            Self::android(),
            Self::windows(),
            Self::macos(),
            Self::social(),
        ]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessorConfig {
    pub input_dir: String,
    pub output_dir: String,
    pub remove_background: bool,
    pub parallel: bool,
    pub quality: u8,
    pub platforms: Vec<String>,
}

impl Default for ProcessorConfig {
    fn default() -> Self {
        Self {
            input_dir: "./input".to_string(),
            output_dir: "./output".to_string(),
            remove_background: true,
            parallel: true,
            quality: 95,
            platforms: vec![
                "favicon".to_string(),
                "apple".to_string(),
                "android".to_string(),
                "windows".to_string(),
                "mac".to_string(),
                "social".to_string(),
            ],
        }
    }
}
