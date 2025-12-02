// Workspace management

use crate::Result;
use std::path::{Path, PathBuf};

/// Workspace manager
pub struct Workspace {
    root: PathBuf,
    ignored_patterns: Vec<String>,
}

impl Workspace {
    pub fn new(root: PathBuf) -> Self {
        let ignored_patterns = vec![
            "target".to_string(),
            "node_modules".to_string(),
            ".git".to_string(),
            "dist".to_string(),
            "build".to_string(),
        ];

        Self {
            root,
            ignored_patterns,
        }
    }

    /// Check if path should be ignored
    pub fn should_ignore(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy();

        for pattern in &self.ignored_patterns {
            if path_str.contains(pattern) {
                return true;
            }
        }

        false
    }

    /// Get all source files in workspace
    pub fn get_source_files(&self) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();
        self.walk_directory(&self.root, &mut files)?;
        Ok(files)
    }

    fn walk_directory(&self, dir: &Path, files: &mut Vec<PathBuf>) -> Result<()> {
        if !dir.is_dir() || self.should_ignore(dir) {
            return Ok(());
        }

        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                self.walk_directory(&path, files)?;
            } else if self.is_source_file(&path) {
                files.push(path);
            }
        }

        Ok(())
    }

    fn is_source_file(&self, path: &Path) -> bool {
        if let Some(ext) = path.extension() {
            matches!(
                ext.to_str(),
                Some("rs" | "js" | "ts" | "py" | "java" | "c" | "cpp" | "go" | "rb")
            )
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_ignore() {
        let workspace = Workspace::new(PathBuf::from("/test"));
        assert!(workspace.should_ignore(Path::new("/test/target/debug")));
        assert!(workspace.should_ignore(Path::new("/test/node_modules")));
        assert!(!workspace.should_ignore(Path::new("/test/src")));
    }

    #[test]
    fn test_is_source_file() {
        let workspace = Workspace::new(PathBuf::from("/test"));
        assert!(workspace.is_source_file(Path::new("test.rs")));
        assert!(workspace.is_source_file(Path::new("test.ts")));
        assert!(!workspace.is_source_file(Path::new("test.txt")));
    }
}
