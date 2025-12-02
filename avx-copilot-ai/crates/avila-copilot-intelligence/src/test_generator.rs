// Test generator

use crate::Result;
use std::sync::Arc;

/// Test generator
pub struct TestGenerator {
}

impl TestGenerator {
    pub fn new() -> Self {
        Self { }
    }

    /// Generate tests for code
    pub async fn generate(&self, code: &str) -> Result<String> {
        let mut tests = String::from("#[cfg(test)]\nmod tests {\n    use super::*;\n\n");

        // Find all functions to test
        for line in code.lines() {
            if line.contains("fn ") && line.contains("pub") {
                let test = self.generate_function_test(line);
                tests.push_str(&test);
                tests.push_str("\n\n");
            }
        }

        tests.push_str("}\n");

        Ok(tests)
    }

    fn generate_function_test(&self, function_def: &str) -> String {
        let name = self.extract_function_name(function_def);

        format!(
            "    #[test]\n    fn test_{}() {{\n        // TODO: Implement test\n        assert!(true);\n    }}",
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_generate_tests() {
        let parser = Arc::new(Parser::with_config(Default::default()).unwrap());
        let generator = TestGenerator::new(parser);

        let code = "pub fn add(a: i32, b: i32) -> i32 { a + b }";
        let tests = generator.generate(code).await.unwrap();

        assert!(tests.contains("#[test]"));
        assert!(tests.contains("test_add"));
    }
}
