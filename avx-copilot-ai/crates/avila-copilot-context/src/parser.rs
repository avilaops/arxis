// Simple AST parser stub for code analysis
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct AstNode {
    pub kind: NodeKind,
    pub name: String,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NodeKind {
    Function,
    Class,
    Variable,
    Import,
    Comment,
}

pub struct SimpleParser;

impl SimpleParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(&self, code: &str) -> Vec<AstNode> {
        let mut nodes = Vec::new();

        for (line_num, line) in code.lines().enumerate() {
            let trimmed = line.trim();

            // Detect functions
            if trimmed.contains("fn ") || trimmed.contains("function ") || trimmed.contains("def ") {
                if let Some(name) = self.extract_name(trimmed, &["fn ", "function ", "def "]) {
                    nodes.push(AstNode {
                        kind: NodeKind::Function,
                        name,
                        line: line_num + 1,
                        column: 0,
                    });
                }
            }

            // Detect classes
            if trimmed.contains("class ") || trimmed.contains("struct ") {
                if let Some(name) = self.extract_name(trimmed, &["class ", "struct "]) {
                    nodes.push(AstNode {
                        kind: NodeKind::Class,
                        name,
                        line: line_num + 1,
                        column: 0,
                    });
                }
            }

            // Detect imports
            if trimmed.starts_with("use ") || trimmed.starts_with("import ") {
                nodes.push(AstNode {
                    kind: NodeKind::Import,
                    name: trimmed.to_string(),
                    line: line_num + 1,
                    column: 0,
                });
            }
        }

        nodes
    }

    fn extract_name(&self, line: &str, keywords: &[&str]) -> Option<String> {
        for keyword in keywords {
            if let Some(pos) = line.find(keyword) {
                let after = &line[pos + keyword.len()..];
                let name = after
                    .split(|c: char| c.is_whitespace() || c == '(' || c == '{' || c == '<')
                    .next()?
                    .trim()
                    .to_string();
                if !name.is_empty() {
                    return Some(name);
                }
            }
        }
        None
    }

    pub fn find_symbols(&self, code: &str) -> HashMap<String, Vec<AstNode>> {
        let nodes = self.parse(code);
        let mut symbols = HashMap::new();

        for node in nodes {
            symbols
                .entry(node.name.clone())
                .or_insert_with(Vec::new)
                .push(node);
        }

        symbols
    }
}
