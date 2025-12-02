// Bug detection engine

use crate::{Bug, BugSeverity, Result};
use std::sync::Arc;

/// Bug detector with pattern matching
pub struct BugDetector {
    rules: Vec<BugRule>,
}

impl BugDetector {
    pub fn new() -> Self {
        let rules = Self::init_rules();
        Self { rules }
    }

    /// Detect bugs in code
    pub async fn detect(&self, code: &str) -> Result<Vec<Bug>> {
        let mut bugs = Vec::new();

        // Run all detection rules
        for rule in &self.rules {
            if let Some(mut detected) = (rule.detector)(code) {
                bugs.append(&mut detected);
            }
        }

        Ok(bugs)
    }

    fn init_rules() -> Vec<BugRule> {
        vec![
            BugRule {
                name: "unused-variable".to_string(),
                detector: Box::new(Self::detect_unused_variables),
            },
            BugRule {
                name: "null-dereference".to_string(),
                detector: Box::new(Self::detect_null_dereference),
            },
            BugRule {
                name: "division-by-zero".to_string(),
                detector: Box::new(Self::detect_division_by_zero),
            },
            BugRule {
                name: "unreachable-code".to_string(),
                detector: Box::new(Self::detect_unreachable_code),
            },
        ]
    }

    fn detect_unused_variables(code: &str) -> Option<Vec<Bug>> {
        let mut bugs = Vec::new();

        // Simple heuristic: find variables that are declared but never used
        for (line_num, line) in code.lines().enumerate() {
            if line.contains("let ") && !line.contains("= ") {
                bugs.push(Bug {
                    line: line_num + 1,
                    column: 0,
                    severity: BugSeverity::Warning,
                    message: "Variable declared but never used".to_string(),
                    suggestion: Some("Remove unused variable or use it".to_string()),
                    rule: "unused-variable".to_string(),
                });
            }
        }

        if bugs.is_empty() {
            None
        } else {
            Some(bugs)
        }
    }

    fn detect_null_dereference(code: &str) -> Option<Vec<Bug>> {
        let mut bugs = Vec::new();

        for (line_num, line) in code.lines().enumerate() {
            if line.contains(".unwrap()") {
                bugs.push(Bug {
                    line: line_num + 1,
                    column: line.find(".unwrap()").unwrap_or(0),
                    severity: BugSeverity::Warning,
                    message: "Potential panic: unwrap() on Option/Result".to_string(),
                    suggestion: Some("Use pattern matching or ? operator instead".to_string()),
                    rule: "null-dereference".to_string(),
                });
            }
        }

        if bugs.is_empty() {
            None
        } else {
            Some(bugs)
        }
    }

    fn detect_division_by_zero(code: &str) -> Option<Vec<Bug>> {
        let mut bugs = Vec::new();

        for (line_num, line) in code.lines().enumerate() {
            if line.contains("/ 0") || line.contains("/0") {
                bugs.push(Bug {
                    line: line_num + 1,
                    column: line.find("/ 0").or_else(|| line.find("/0")).unwrap_or(0),
                    severity: BugSeverity::Error,
                    message: "Division by zero".to_string(),
                    suggestion: Some("Check divisor before division".to_string()),
                    rule: "division-by-zero".to_string(),
                });
            }
        }

        if bugs.is_empty() {
            None
        } else {
            Some(bugs)
        }
    }

    fn detect_unreachable_code(code: &str) -> Option<Vec<Bug>> {
        let mut bugs = Vec::new();
        let mut after_return = false;

        for (line_num, line) in code.lines().enumerate() {
            let trimmed = line.trim();

            if trimmed.starts_with("return ") {
                after_return = true;
            } else if after_return && !trimmed.is_empty() && trimmed != "}" {
                bugs.push(Bug {
                    line: line_num + 1,
                    column: 0,
                    severity: BugSeverity::Warning,
                    message: "Unreachable code after return".to_string(),
                    suggestion: Some("Remove unreachable code".to_string()),
                    rule: "unreachable-code".to_string(),
                });
                after_return = false;
            } else if trimmed == "}" {
                after_return = false;
            }
        }

        if bugs.is_empty() {
            None
        } else {
            Some(bugs)
        }
    }
}

/// Bug detection rule
struct BugRule {
    name: String,
    detector: Box<dyn Fn(&str) -> Option<Vec<Bug>> + Send + Sync>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_division_by_zero() {
        let code = "let x = 10 / 0;";
        let bugs = BugDetector::detect_division_by_zero(code);
        assert!(bugs.is_some());
        assert_eq!(bugs.unwrap().len(), 1);
    }

    #[test]
    fn test_detect_unwrap() {
        let code = "let x = some_option.unwrap();";
        let bugs = BugDetector::detect_null_dereference(code);
        assert!(bugs.is_some());
    }
}
