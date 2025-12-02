// Bridge between VS Code extension and Rust backend

use serde::{Deserialize, Serialize};

pub mod messages;

/// Request from VS Code extension
#[derive(Debug, Serialize, Deserialize)]
pub struct ExtensionRequest {
    pub id: String,
    pub method: RequestMethod,
    pub params: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RequestMethod {
    Complete,
    Diagnose,
    Refactor,
    GenerateDocs,
    GenerateTests,
}

/// Response to VS Code extension
#[derive(Debug, Serialize, Deserialize)]
pub struct ExtensionResponse {
    pub id: String,
    pub result: serde_json::Value,
    pub error: Option<String>,
}
