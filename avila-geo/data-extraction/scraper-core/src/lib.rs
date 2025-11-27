pub mod engine;
pub mod extractors;
pub mod anti_detect;
pub mod monitoring;
pub mod types;

#[cfg(feature = "storage")]
pub mod storage;

pub use engine::{ScraperEngine, ScraperEngineBuilder};
pub use anti_detect::AntiDetectionStrategy;
pub use types::*;

pub mod prelude {
    pub use crate::engine::{ScraperEngine, ScraperEngineBuilder};
    pub use crate::extractors::{DataExtractor, LinkedInCompanyExtractor, JobPostingExtractor};
    pub use crate::anti_detect::AntiDetectionStrategy;
    pub use crate::types::*;

    #[cfg(feature = "storage")]
    pub use crate::storage::ScrapedDataManager;
}
