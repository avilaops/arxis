// Workspace indexing for fast symbol lookup

use crate::{ContextError, Result};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Workspace index for fast symbol and file lookup
pub struct WorkspaceIndex {
    files: HashMap<PathBuf, FileInfo>,
    symbols: HashMap<String, Vec<SymbolLocation>>,
}

impl WorkspaceIndex {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            files: HashMap::new(),
            symbols: HashMap::new(),
        })
    }

    /// Index a directory recursively
    pub async fn index_directory(&self, path: &Path) -> Result<()> {
        // TODO: Implement recursive directory indexing
        Ok(())
    }

    /// Find files related to given references
    pub async fn find_related_files(&self, references: &[String]) -> Vec<PathBuf> {
        let mut related = Vec::new();

        for reference in references {
            // Extract symbol name from reference
            let symbol = self.extract_symbol_name(reference);

            // Find files containing this symbol
            if let Some(locations) = self.symbols.get(&symbol) {
                for location in locations {
                    if !related.contains(&location.file) {
                        related.push(location.file.clone());
                    }
                }
            }
        }

        related
    }

    /// Extract symbol name from import/use statement
    fn extract_symbol_name(&self, reference: &str) -> String {
        // Simple extraction - can be improved
        reference
            .split_whitespace()
            .last()
            .unwrap_or("")
            .trim_end_matches(';')
            .to_string()
    }

    /// Add file to index
    pub fn add_file(&mut self, path: PathBuf, info: FileInfo) {
        self.files.insert(path, info);
    }

    /// Add symbol to index
    pub fn add_symbol(&mut self, name: String, location: SymbolLocation) {
        self.symbols
            .entry(name)
            .or_insert_with(Vec::new)
            .push(location);
    }

    /// Get file info
    pub fn get_file_info(&self, path: &Path) -> Option<&FileInfo> {
        self.files.get(path)
    }

    /// Get symbol locations
    pub fn get_symbol_locations(&self, name: &str) -> Option<&[SymbolLocation]> {
        self.symbols.get(name).map(|v| v.as_slice())
    }
}

/// File information
#[derive(Debug, Clone)]
pub struct FileInfo {
    pub path: PathBuf,
    pub size: u64,
    pub modified: u64,
    pub language: String,
}

/// Symbol location
#[derive(Debug, Clone)]
pub struct SymbolLocation {
    pub file: PathBuf,
    pub line: usize,
    pub column: usize,
    pub kind: SymbolKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolKind {
    Function,
    Class,
    Variable,
    Constant,
    Type,
    Interface,
    Module,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_workspace_index_creation() {
        let index = WorkspaceIndex::new().await.unwrap();
        assert!(index.files.is_empty());
        assert!(index.symbols.is_empty());
    }

    #[test]
    fn test_extract_symbol_name() {
        let index = WorkspaceIndex {
            files: HashMap::new(),
            symbols: HashMap::new(),
        };

        let symbol = index.extract_symbol_name("use std::collections::HashMap;");
        assert_eq!(symbol, "HashMap");
    }
}
