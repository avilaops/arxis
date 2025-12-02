// Layer 5: Code Intelligence
// Advanced AST parsing, bug detection, and refactoring

use avila_copilot_context::ContextManager;
use std::sync::Arc;

pub mod bug_detector;
pub mod doc_generator;
pub mod error;
pub mod refactoring;
pub mod test_generator;

pub use bug_detector::BugDetector;
pub use doc_generator::DocGenerator;
pub use error::{IntelligenceError, Result};
pub use refactoring::RefactoringEngine;
pub use test_generator::TestGenerator;

/// Code intelligence system
pub struct CodeIntelligence {
    context_manager: Arc<ContextManager>,
    bug_detector: Arc<BugDetector>,
    doc_generator: Arc<DocGenerator>,
    test_generator: Arc<TestGenerator>,
    refactoring_engine: Arc<RefactoringEngine>,
}

impl CodeIntelligence {
    /// Create new code intelligence system
    pub async fn new(context_manager: Arc<ContextManager>) -> Result<Self> {
        let bug_detector = Arc::new(BugDetector::new());
        let doc_generator = Arc::new(DocGenerator::new());
        let test_generator = Arc::new(TestGenerator::new());
        let refactoring_engine = Arc::new(RefactoringEngine::new());

        Ok(Self {
            context_manager,
            bug_detector,
            doc_generator,
            test_generator,
            refactoring_engine,
        })
    }

    /// Detect bugs in code
    pub async fn detect_bugs(&self, code: &str) -> Result<Vec<Bug>> {
        self.bug_detector.detect(code).await
    }

    /// Generate documentation for code
    pub async fn generate_documentation(&self, code: &str) -> Result<String> {
        self.doc_generator.generate(code).await
    }

    /// Generate tests for code
    pub async fn generate_tests(&self, code: &str) -> Result<String> {
        self.test_generator.generate(code).await
    }

    /// Suggest refactorings
    pub async fn suggest_refactorings(&self, code: &str) -> Result<Vec<Refactoring>> {
        self.refactoring_engine.suggest(code).await
    }

    /// Apply refactoring
    pub async fn apply_refactoring(&self, code: &str, refactoring: &Refactoring) -> Result<String> {
        self.refactoring_engine.apply(code, refactoring).await
    }

    /// Get code complexity metrics
    pub fn analyze_complexity(&self, code: &str) -> Result<ComplexityMetrics> {
        Ok(ComplexityMetrics {
            cyclomatic_complexity: self.calculate_cyclomatic_complexity(code),
            cognitive_complexity: self.calculate_cognitive_complexity(code),
            lines_of_code: code.lines().count(),
            num_functions: self.count_functions(code),
        })
    }

    fn calculate_cyclomatic_complexity(&self, _code: &str) -> usize {
        // Simple implementation - count decision points
        1 // Base complexity
    }

    fn calculate_cognitive_complexity(&self, _code: &str) -> usize {
        // Cognitive complexity considers nesting
        0
    }

    fn count_functions(&self, _code: &str) -> usize {
        // Count function definitions
        0
    }
}

/// Bug detection result
#[derive(Debug, Clone)]
pub struct Bug {
    pub line: usize,
    pub column: usize,
    pub severity: BugSeverity,
    pub message: String,
    pub suggestion: Option<String>,
    pub rule: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BugSeverity {
    Error,
    Warning,
    Info,
}

/// Refactoring suggestion
#[derive(Debug, Clone)]
pub struct Refactoring {
    pub kind: RefactoringKind,
    pub description: String,
    pub location: CodeLocation,
    pub replacement: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RefactoringKind {
    ExtractFunction,
    ExtractVariable,
    InlineVariable,
    InlineFunction,
    RenameSymbol,
    SimplifyExpression,
    RemoveDeadCode,
    ExtractInterface,
    MoveFunction,
}

#[derive(Debug, Clone)]
pub struct CodeLocation {
    pub start_line: usize,
    pub start_column: usize,
    pub end_line: usize,
    pub end_column: usize,
}

/// Code complexity metrics
#[derive(Debug, Clone)]
pub struct ComplexityMetrics {
    pub cyclomatic_complexity: usize,
    pub cognitive_complexity: usize,
    pub lines_of_code: usize,
    pub num_functions: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complexity_metrics() {
        // Tests will be added
    }
}
