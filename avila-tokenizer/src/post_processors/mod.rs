pub mod bert;
pub mod roberta;
pub mod template;

pub use bert::BertProcessing;
pub use roberta::RobertaProcessing;
pub use template::TemplateProcessing;

use crate::error::Result;

/// Trait for post-processing token IDs (adding special tokens, formatting, etc.)
pub trait PostProcessor: Send + Sync {
    /// Process token IDs, optionally with a second sequence
    fn process(&self, ids: Vec<u32>, pair_ids: Option<Vec<u32>>) -> Result<Vec<u32>>;
}
