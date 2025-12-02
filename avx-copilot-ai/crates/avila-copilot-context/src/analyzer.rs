// Semantic code analyzer

use crate::Result;

/// Semantic code analyzer
pub struct SemanticAnalyzer {
    // Analyzer state
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self {}
    }

    /// Analyze code semantics
    pub fn analyze(&self, code: &str) -> Result<SemanticInfo> {
        Ok(SemanticInfo {
            complexity: self.calculate_complexity(code),
            depth: self.calculate_depth(code),
            symbols: self.extract_symbols(code),
        })
    }

    fn calculate_complexity(&self, code: &str) -> usize {
        // Simple complexity metric - count decision points
        let mut complexity = 1;

        for line in code.lines() {
            if line.contains("if ") || line.contains("for ") || line.contains("while ") {
                complexity += 1;
            }
        }

        complexity
    }

    fn calculate_depth(&self, code: &str) -> usize {
        let mut max_depth = 0_usize;
        let mut current_depth = 0_usize;

        for ch in code.chars() {
            match ch {
                '{' => {
                    current_depth += 1;
                    max_depth = max_depth.max(current_depth);
                }
                '}' => {
                    current_depth = current_depth.saturating_sub(1);
                }
                _ => {}
            }
        }

        max_depth
    }

    fn extract_symbols(&self, code: &str) -> Vec<String> {
        let mut symbols = Vec::new();

        for line in code.lines() {
            if line.contains("fn ") {
                if let Some(name) = self.extract_function_name(line) {
                    symbols.push(name);
                }
            }
        }

        symbols
    }

    fn extract_function_name(&self, line: &str) -> Option<String> {
        line.split_whitespace()
            .skip_while(|&s| s != "fn")
            .nth(1)
            .map(|s| s.trim_end_matches('(').to_string())
    }
}

impl Default for SemanticAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// Semantic information about code
#[derive(Debug, Clone)]
pub struct SemanticInfo {
    pub complexity: usize,
    pub depth: usize,
    pub symbols: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze() {
        let analyzer = SemanticAnalyzer::new();
        let code = "fn main() {\n    if true {\n        println!(\"test\");\n    }\n}";

        let info = analyzer.analyze(code).unwrap();
        assert!(info.complexity > 1);
        assert!(info.depth > 0);
    }

    #[test]
    fn test_extract_symbols() {
        let analyzer = SemanticAnalyzer::new();
        let code = "fn main() {}\nfn test() {}";

        let symbols = analyzer.extract_symbols(code);
        assert_eq!(symbols.len(), 2);
        assert!(symbols.contains(&"main".to_string()));
        assert!(symbols.contains(&"test".to_string()));
    }
}
