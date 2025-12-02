// Refactoring handler

use crate::Result;

pub struct RefactorHandler {
    // Handler state
}

impl RefactorHandler {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn handle(&self, text: &str) -> Result<Vec<RefactorAction>> {
        Ok(Vec::new())
    }
}

impl Default for RefactorHandler {
    fn default() -> Self {
        Self::new()
    }
}

pub struct RefactorAction {
    pub title: String,
    pub edit: String,
}
