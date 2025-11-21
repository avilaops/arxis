use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Theme {
    Light,
    Dark,
}

impl Theme {
    pub fn as_str(&self) -> &'static str {
        match self {
            Theme::Light => "light",
            Theme::Dark => "dark",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "light" => Some(Theme::Light),
            "dark" => Some(Theme::Dark),
            _ => None,
        }
    }

    pub fn toggle(&self) -> Self {
        match self {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Light,
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Theme::Light => "🌙",
            Theme::Dark => "☀️",
        }
    }
}
