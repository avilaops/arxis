// Layer 6: LSP Server
// Language Server Protocol implementation for Avila Copilot

use avila_copilot_core::CopilotEngine;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::sync::RwLock;

pub mod completion;
pub mod diagnostics;
pub mod error;
pub mod hover;
pub mod protocol;
pub mod refactor;

pub use error::{LspError, Result};

/// LSP server for Avila Copilot
pub struct LspServer {
    engine: Arc<CopilotEngine>,
    state: Arc<RwLock<ServerState>>,
}

impl LspServer {
    pub fn new(engine: CopilotEngine) -> Self {
        Self {
            engine: Arc::new(engine),
            state: Arc::new(RwLock::new(ServerState::default())),
        }
    }

    /// Run LSP server
    pub async fn run(&self) -> Result<()> {
        let stdin = tokio::io::stdin();
        let stdout = tokio::io::stdout();

        let mut reader = BufReader::new(stdin);
        let mut writer = stdout;

        loop {
            // Read LSP message
            let message = self.read_message(&mut reader).await?;

            // Process message
            let response = self.handle_message(&message).await?;

            // Send response
            self.write_message(&mut writer, &response).await?;
        }
    }

    async fn read_message<R: AsyncBufReadExt + Unpin>(&self, reader: &mut R) -> Result<LspMessage> {
        let mut headers = Vec::new();
        let mut line = String::new();

        // Read headers
        loop {
            line.clear();
            reader.read_line(&mut line).await?;

            if line == "\r\n" || line == "\n" {
                break;
            }

            headers.push(line.clone());
        }

        // Parse Content-Length
        let content_length = self.parse_content_length(&headers)?;

        // Read content
        let mut content = vec![0u8; content_length];
        reader.read_exact(&mut content).await?;

        let message: LspMessage = serde_json::from_slice(&content)?;
        Ok(message)
    }

    async fn write_message<W: AsyncWriteExt + Unpin>(&self, writer: &mut W, message: &LspMessage) -> Result<()> {
        let content = serde_json::to_string(message)?;
        let header = format!("Content-Length: {}\r\n\r\n", content.len());

        writer.write_all(header.as_bytes()).await?;
        writer.write_all(content.as_bytes()).await?;
        writer.flush().await?;

        Ok(())
    }

    fn parse_content_length(&self, headers: &[String]) -> Result<usize> {
        for header in headers {
            if header.starts_with("Content-Length:") {
                let length_str = header.trim_start_matches("Content-Length:").trim();
                return length_str.parse().map_err(|_| LspError::ParseError("Invalid Content-Length".to_string()));
            }
        }

        Err(LspError::ParseError("Missing Content-Length header".to_string()))
    }

    async fn handle_message(&self, message: &LspMessage) -> Result<LspMessage> {
        match &message.method[..] {
            "initialize" => self.handle_initialize(message).await,
            "textDocument/completion" => self.handle_completion(message).await,
            "textDocument/hover" => self.handle_hover(message).await,
            "textDocument/diagnostic" => self.handle_diagnostic(message).await,
            "textDocument/codeAction" => self.handle_code_action(message).await,
            _ => Ok(self.create_error_response(message, "Unknown method")),
        }
    }

    async fn handle_initialize(&self, message: &LspMessage) -> Result<LspMessage> {
        let capabilities = ServerCapabilities {
            completion_provider: Some(CompletionOptions {
                trigger_characters: vec![".".to_string(), ":".to_string()],
            }),
            hover_provider: Some(true),
            diagnostic_provider: Some(true),
            code_action_provider: Some(true),
        };

        Ok(LspMessage {
            jsonrpc: "2.0".to_string(),
            id: message.id.clone(),
            method: String::new(),
            params: serde_json::to_value(capabilities)?,
        })
    }

    async fn handle_completion(&self, message: &LspMessage) -> Result<LspMessage> {
        let params: CompletionParams = serde_json::from_value(message.params.clone())?;

        // Get completion from engine
        let completion = self.engine
            .complete(&params.text, params.position)
            .await
            .map_err(|e| LspError::EngineError(e.to_string()))?;

        let items = vec![CompletionItem {
            label: completion.text.clone(),
            kind: CompletionItemKind::Text,
            detail: Some(format!("Latency: {}ms", completion.latency_ms)),
            documentation: None,
        }];

        Ok(LspMessage {
            jsonrpc: "2.0".to_string(),
            id: message.id.clone(),
            method: String::new(),
            params: serde_json::to_value(items)?,
        })
    }

    async fn handle_hover(&self, message: &LspMessage) -> Result<LspMessage> {
        // TODO: Implement hover
        Ok(self.create_empty_response(message))
    }

    async fn handle_diagnostic(&self, message: &LspMessage) -> Result<LspMessage> {
        let params: DiagnosticParams = serde_json::from_value(message.params.clone())?;

        // Get bugs from engine
        let bugs = self.engine
            .detect_bugs(&params.text)
            .await
            .map_err(|e| LspError::EngineError(e.to_string()))?;

        let diagnostics: Vec<Diagnostic> = bugs
            .into_iter()
            .map(|bug| Diagnostic {
                range: Range {
                    start: Position { line: bug.line, character: bug.column },
                    end: Position { line: bug.line, character: bug.column + 1 },
                },
                severity: match bug.severity {
                    avila_copilot_core::BugSeverity::Error => DiagnosticSeverity::Error,
                    avila_copilot_core::BugSeverity::Warning => DiagnosticSeverity::Warning,
                    avila_copilot_core::BugSeverity::Info => DiagnosticSeverity::Information,
                },
                message: bug.message,
            })
            .collect();

        Ok(LspMessage {
            jsonrpc: "2.0".to_string(),
            id: message.id.clone(),
            method: String::new(),
            params: serde_json::to_value(diagnostics)?,
        })
    }

    async fn handle_code_action(&self, message: &LspMessage) -> Result<LspMessage> {
        let params: CodeActionParams = serde_json::from_value(message.params.clone())?;

        // Get refactorings from engine
        let refactorings = self.engine
            .suggest_refactorings(&params.text)
            .await
            .map_err(|e| LspError::EngineError(e.to_string()))?;

        let actions: Vec<CodeAction> = refactorings
            .into_iter()
            .map(|r| CodeAction {
                title: r.description,
                kind: CodeActionKind::Refactor,
            })
            .collect();

        Ok(LspMessage {
            jsonrpc: "2.0".to_string(),
            id: message.id.clone(),
            method: String::new(),
            params: serde_json::to_value(actions)?,
        })
    }

    fn create_error_response(&self, message: &LspMessage, error: &str) -> LspMessage {
        LspMessage {
            jsonrpc: "2.0".to_string(),
            id: message.id.clone(),
            method: String::new(),
            params: serde_json::json!({ "error": error }),
        }
    }

    fn create_empty_response(&self, message: &LspMessage) -> LspMessage {
        LspMessage {
            jsonrpc: "2.0".to_string(),
            id: message.id.clone(),
            method: String::new(),
            params: serde_json::json!(null),
        }
    }
}

/// Server state
#[derive(Default)]
struct ServerState {
    initialized: bool,
}

/// LSP message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LspMessage {
    pub jsonrpc: String,
    pub id: Option<i64>,
    pub method: String,
    pub params: serde_json::Value,
}

/// Server capabilities
#[derive(Debug, Serialize, Deserialize)]
struct ServerCapabilities {
    completion_provider: Option<CompletionOptions>,
    hover_provider: Option<bool>,
    diagnostic_provider: Option<bool>,
    code_action_provider: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CompletionOptions {
    trigger_characters: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CompletionParams {
    text: String,
    position: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct CompletionItem {
    label: String,
    kind: CompletionItemKind,
    detail: Option<String>,
    documentation: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
enum CompletionItemKind {
    Text = 1,
    Method = 2,
    Function = 3,
    Constructor = 4,
    Field = 5,
    Variable = 6,
}

#[derive(Debug, Serialize, Deserialize)]
struct DiagnosticParams {
    text: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Diagnostic {
    range: Range,
    severity: DiagnosticSeverity,
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
enum DiagnosticSeverity {
    Error = 1,
    Warning = 2,
    Information = 3,
    Hint = 4,
}

#[derive(Debug, Serialize, Deserialize)]
struct Range {
    start: Position,
    end: Position,
}

#[derive(Debug, Serialize, Deserialize)]
struct Position {
    line: usize,
    character: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct CodeActionParams {
    text: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CodeAction {
    title: String,
    kind: CodeActionKind,
}

#[derive(Debug, Serialize, Deserialize)]
enum CodeActionKind {
    QuickFix,
    Refactor,
}
