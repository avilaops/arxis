// Diagnostics handler

use crate::Result;

pub struct DiagnosticsHandler {
    // Handler state
}

impl DiagnosticsHandler {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn handle(&self, text: &str) -> Result<Vec<Diagnostic>> {
        Ok(Vec::new())
    }
}

impl Default for DiagnosticsHandler {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Diagnostic {
    pub line: usize,
    pub message: String,
}
