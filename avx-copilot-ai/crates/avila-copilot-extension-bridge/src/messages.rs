// Message types for extension communication

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionRequest {
    pub text: String,
    pub position: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionResponse {
    pub completions: Vec<String>,
    pub latency_ms: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiagnosticRequest {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiagnosticResponse {
    pub diagnostics: Vec<DiagnosticItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiagnosticItem {
    pub line: usize,
    pub column: usize,
    pub severity: String,
    pub message: String,
}
