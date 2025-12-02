// Hover handler

use crate::Result;

pub struct HoverHandler {
    // Handler state
}

impl HoverHandler {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn handle(&self, text: &str, position: usize) -> Result<Option<HoverInfo>> {
        Ok(None)
    }
}

impl Default for HoverHandler {
    fn default() -> Self {
        Self::new()
    }
}

pub struct HoverInfo {
    pub content: String,
}
