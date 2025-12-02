// Layer 3: Context Manager
// Unlimited workspace context with semantic analysis

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

pub mod analyzer;
pub mod error;
pub mod index;
pub mod workspace;
mod parser;

pub use error::{ContextError, Result};
use parser::SimpleParser;

/// Context manager with unlimited workspace awareness
pub struct ContextManager {
    max_context_tokens: usize,
    workspace_index: Arc<index::WorkspaceIndex>,
    semantic_cache: Arc<Mutex<HashMap<String, ContextData>>>,
    file_cache: Arc<Mutex<HashMap<PathBuf, FileContext>>>,
    parser: Arc<SimpleParser>,
}

impl ContextManager {
    /// Create new context manager
    pub async fn new(max_context_tokens: usize) -> Result<Self> {
        let semantic_cache = Arc::new(Mutex::new(HashMap::new()));
        let file_cache = Arc::new(Mutex::new(HashMap::new()));

        let parser = Arc::new(SimpleParser::new());

        let workspace_index = Arc::new(index::WorkspaceIndex::new().await?);

        Ok(Self {
            max_context_tokens,
            workspace_index,
            semantic_cache,
            file_cache,
            parser,
        })
    }

    /// Get context for code completion
    pub async fn get_context(&self, code: &str, cursor_position: usize) -> String {
        // Get local context (current file)
        let local_context = self.get_local_context(code, cursor_position);

        // Get workspace context (related files)
        let workspace_context = self.get_workspace_context(code).await;

        // Combine contexts
        self.combine_contexts(&local_context, &workspace_context)
    }

    /// Get local context from current file
    fn get_local_context(&self, code: &str, cursor_position: usize) -> LocalContext {
        // Get surrounding code
        let before = &code[..cursor_position.min(code.len())];
        let after = &code[cursor_position.min(code.len())..];

        // Parse to get AST
        let ast = self.parser.parse(code);

        // Find current scope
        let current_scope = self.find_current_scope(code, cursor_position);

        LocalContext {
            before: before.to_string(),
            after: after.to_string(),
            current_scope,
            ast: Some(ast),
        }
    }

    /// Get workspace context (related files and symbols)
    async fn get_workspace_context(&self, code: &str) -> WorkspaceContext {
        // Extract imports and references
        let references = self.extract_references(code);

        // Find related files
        let related_files = self.workspace_index.find_related_files(&references).await;

        WorkspaceContext { related_files }
    }

    /// Combine local and workspace contexts
    fn combine_contexts(&self, local: &LocalContext, workspace: &WorkspaceContext) -> String {
        let mut combined = String::new();

        // Add local context
        combined.push_str(&local.before);

        // Add workspace context (related symbols)
        for file in &workspace.related_files {
            combined.push_str("\n// Related: ");
            combined.push_str(&file.display().to_string());
            combined.push('\n');
        }

        combined
    }

    /// Find current scope at cursor position
    fn find_current_scope(&self, code: &str, cursor_position: usize) -> Option<String> {
        // Simple scope detection
        let before = &code[..cursor_position.min(code.len())];

        // Find last function/class definition
        for line in before.lines().rev() {
            if line.contains("fn ") || line.contains("function ") || line.contains("class ") {
                return Some(line.trim().to_string());
            }
        }

        None
    }

    /// Extract references from code
    fn extract_references(&self, code: &str) -> Vec<String> {
        let mut references = Vec::new();

        // Extract import statements
        for line in code.lines() {
            if line.contains("use ") || line.contains("import ") {
                references.push(line.trim().to_string());
            }
        }

        references
    }

    /// Index workspace directory
    pub async fn index_workspace(&self, workspace_path: &Path) -> Result<()> {
        self.workspace_index.index_directory(workspace_path).await
    }

    /// Get file context
    pub async fn get_file_context(&self, path: &Path) -> Result<FileContext> {
        // Check cache
        {
            let cache = self.file_cache.lock().unwrap();
            if let Some(cached) = cache.get(path) {
                return Ok(cached.clone());
            }
        }

        // Read and analyze file
        let content = tokio::fs::read_to_string(path).await?;
        let ast = self.parser.parse(&content);

        let context = FileContext {
            path: path.to_path_buf(),
            content,
            ast: Some(ast),
        };

        // Cache for future use
        {
            let mut cache = self.file_cache.lock().unwrap();
            cache.insert(path.to_path_buf(), context.clone());
        }

        Ok(context)
    }

    /// Clear caches
    pub fn clear_caches(&self) {
        let mut semantic_cache = self.semantic_cache.lock().unwrap();
        semantic_cache.clear();
        let mut file_cache = self.file_cache.lock().unwrap();
        file_cache.clear();
    }
}

/// Local context from current file
#[derive(Debug, Clone)]
struct LocalContext {
    before: String,
    after: String,
    current_scope: Option<String>,
    ast: Option<Vec<parser::AstNode>>,
}

/// Workspace context with related files
#[derive(Debug, Clone)]
struct WorkspaceContext {
    related_files: Vec<PathBuf>,
}

/// Context data for caching
#[derive(Debug, Clone)]
pub struct ContextData {
    pub text: String,
    pub tokens: Vec<u32>,
    pub timestamp: u64,
}

/// File context
#[derive(Debug, Clone)]
pub struct FileContext {
    pub path: PathBuf,
    pub content: String,
    pub ast: Option<Vec<parser::AstNode>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_context_manager_creation() {
        let manager = ContextManager::new(100_000).await.unwrap();
        assert_eq!(manager.max_context_tokens, 100_000);
    }

    #[tokio::test]
    async fn test_get_local_context() {
        let manager = ContextManager::new(100_000).await.unwrap();
        let code = "fn main() {\n    println!(\"test\");\n}";
        let context = manager.get_local_context(code, 10);

        assert!(!context.before.is_empty());
    }

    #[tokio::test]
    async fn test_extract_references() {
        let manager = ContextManager::new(100_000).await.unwrap();
        let code = "use std::collections::HashMap;\nfn main() {}";
        let refs = manager.extract_references(code);

        assert!(!refs.is_empty());
    }
}
