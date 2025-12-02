// LSP Protocol types and utilities

use serde::{Deserialize, Serialize};

/// LSP protocol version
pub const LSP_VERSION: &str = "3.17";

/// Initialize request
#[derive(Debug, Serialize, Deserialize)]
pub struct InitializeRequest {
    pub process_id: Option<i64>,
    pub client_info: Option<ClientInfo>,
    pub capabilities: ClientCapabilities,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientInfo {
    pub name: String,
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientCapabilities {
    pub text_document: Option<TextDocumentClientCapabilities>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextDocumentClientCapabilities {
    pub completion: Option<CompletionClientCapabilities>,
    pub hover: Option<HoverClientCapabilities>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionClientCapabilities {
    pub dynamic_registration: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HoverClientCapabilities {
    pub dynamic_registration: Option<bool>,
}
