// Code vocabulary with programming language awareness

use std::collections::HashMap;

/// Code-specific vocabulary
pub struct CodeVocabulary {
    keywords: HashMap<String, u32>,
    operators: HashMap<String, u32>,
    next_id: u32,
}

impl CodeVocabulary {
    pub fn new() -> Self {
        let mut vocab = Self {
            keywords: HashMap::new(),
            operators: HashMap::new(),
            next_id: 1000, // Reserve 0-999 for base tokens
        };

        vocab.init_keywords();
        vocab.init_operators();
        vocab
    }

    fn init_keywords(&mut self) {
        let rust_keywords = vec![
            "fn", "let", "mut", "const", "static", "impl", "trait", "struct",
            "enum", "type", "pub", "use", "mod", "crate", "self", "super",
            "if", "else", "match", "loop", "while", "for", "in", "return",
            "break", "continue", "async", "await", "unsafe", "where",
        ];

        for keyword in rust_keywords {
            self.keywords.insert(keyword.to_string(), self.next_id);
            self.next_id += 1;
        }

        let common_keywords = vec![
            "function", "class", "interface", "extends", "implements",
            "import", "export", "default", "new", "this", "super",
            "var", "let", "const", "void", "null", "undefined",
        ];

        for keyword in common_keywords {
            if !self.keywords.contains_key(keyword) {
                self.keywords.insert(keyword.to_string(), self.next_id);
                self.next_id += 1;
            }
        }
    }

    fn init_operators(&mut self) {
        let operators = vec![
            "+", "-", "*", "/", "%", "=", "==", "!=", "<", ">", "<=", ">=",
            "&&", "||", "!", "&", "|", "^", "<<", ">>", "++", "--",
            "+=", "-=", "*=", "/=", "->", "=>", "::", ".", ",", ";",
        ];

        for op in operators {
            self.operators.insert(op.to_string(), self.next_id);
            self.next_id += 1;
        }
    }

    pub fn get_keyword_id(&self, keyword: &str) -> Option<u32> {
        self.keywords.get(keyword).copied()
    }

    pub fn get_operator_id(&self, operator: &str) -> Option<u32> {
        self.operators.get(operator).copied()
    }

    pub fn is_keyword(&self, word: &str) -> bool {
        self.keywords.contains_key(word)
    }

    pub fn is_operator(&self, op: &str) -> bool {
        self.operators.contains_key(op)
    }
}

impl Default for CodeVocabulary {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vocabulary_keywords() {
        let vocab = CodeVocabulary::new();
        assert!(vocab.is_keyword("fn"));
        assert!(vocab.is_keyword("function"));
        assert!(!vocab.is_keyword("notakeyword"));
    }

    #[test]
    fn test_vocabulary_operators() {
        let vocab = CodeVocabulary::new();
        assert!(vocab.is_operator("+"));
        assert!(vocab.is_operator("->"));
        assert!(!vocab.is_operator("@"));
    }
}
