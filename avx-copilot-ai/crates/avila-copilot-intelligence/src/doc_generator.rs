// Documentation generator

use crate::Result;
use std::sync::Arc;

/// Automatic documentation generator
pub struct DocGenerator {
}

impl DocGenerator {
    pub fn new() -> Self {
        Self { }
    }

    /// Generate documentation for code
    pub async fn generate(&self, code: &str) -> Result<String> {
        let mut docs = String::new();

        // Parse code to identify functions, structs, etc.
        for line in code.lines() {
            if line.contains("fn ") {
                let doc = self.generate_function_doc(line);
                docs.push_str(&doc);
                docs.push('\n');
            } else if line.contains("struct ") {
                let doc = self.generate_struct_doc(line);
                docs.push_str(&doc);
                docs.push('\n');
            }
        }

        Ok(docs)
    }

    fn generate_function_doc(&self, function_def: &str) -> String {
        let name = self.extract_function_name(function_def);

        format!(
            "/// {}\n/// \n/// # Arguments\n/// \n/// # Returns\n/// \n/// # Examples\n/// \n/// ```\n/// // Example usage\n/// ```",
            name
        )
    }

    fn generate_struct_doc(&self, struct_def: &str) -> String {
        let name = self.extract_struct_name(struct_def);

        format!(
            "/// {}\n/// \n/// # Fields\n/// \n/// # Examples\n/// \n/// ```\n/// // Example usage\n/// ```",
            name
        )
    }

    fn extract_function_name(&self, line: &str) -> String {
        line.split_whitespace()
            .skip_while(|&s| s != "fn")
            .nth(1)
            .unwrap_or("function")
            .trim_end_matches('(')
            .to_string()
    }

    fn extract_struct_name(&self, line: &str) -> String {
        line.split_whitespace()
            .skip_while(|&s| s != "struct")
            .nth(1)
            .unwrap_or("struct")
            .trim_end_matches('{')
            .trim()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_function_name() {
        let parser = Arc::new(Parser::with_config(Default::default()).unwrap());
        let generator = DocGenerator::new(parser);

        let name = generator.extract_function_name("pub fn test_function() {");
        assert_eq!(name, "test_function");
    }
}
