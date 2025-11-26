use crate::types::{CompanyInfo, JobPosting, Place, ScraperError, ScraperResult};
use serde_json::Value;

#[cfg(feature = "storage")]
pub struct ScrapedDataManager {
    client: aviladb_sdk::AvilaClient,
    database_name: String,
}

#[cfg(feature = "storage")]
impl ScrapedDataManager {
    pub async fn new(connection_string: &str, database: &str) -> ScraperResult<Self> {
        let client = aviladb_sdk::AvilaClient::connect(connection_string)
            .await
            .map_err(|e| ScraperError::StorageError(format!("Failed to connect to AvilaDB: {}", e)))?;

        Ok(Self {
            client,
            database_name: database.to_string(),
        })
    }

    /// Store company with automatic deduplication
    pub async fn store_company(&self, company: CompanyInfo) -> ScraperResult<()> {
        let db = self.client.database(&self.database_name)
            .await
            .map_err(|e| ScraperError::StorageError(e.to_string()))?;

        let collection = db.collection("companies")
            .await
            .map_err(|e| ScraperError::StorageError(e.to_string()))?;

        let id = self.generate_company_id(&company);

        // Check if exists
        let query = format!(
            "SELECT * FROM companies WHERE id = '{}'",
            id
        );

        let existing = collection.query(&query)
            .execute()
            .await
            .map_err(|e| ScraperError::StorageError(e.to_string()))?;

        let doc = serde_json::to_value(&company)
            .map_err(|e| ScraperError::StorageError(e.to_string()))?;

        if existing.is_empty() {
            // Insert new
            collection.insert(doc)
                .await
                .map_err(|e| ScraperError::StorageError(e.to_string()))?;
        } else {
            // Update existing
            collection.update(&id, doc)
                .await
                .map_err(|e| ScraperError::StorageError(e.to_string()))?;
        }

        Ok(())
    }

    /// Store job posting
    pub async fn store_job(&self, job: JobPosting) -> ScraperResult<()> {
        let db = self.client.database(&self.database_name)
            .await
            .map_err(|e| ScraperError::StorageError(e.to_string()))?;

        let collection = db.collection("jobs")
            .await
            .map_err(|e| ScraperError::StorageError(e.to_string()))?;

        let doc = serde_json::to_value(&job)
            .map_err(|e| ScraperError::StorageError(e.to_string()))?;

        collection.insert(doc)
            .await
            .map_err(|e| ScraperError::StorageError(e.to_string()))?;

        Ok(())
    }

    /// Store place/business listing
    pub async fn store_place(&self, place: Place) -> ScraperResult<()> {
        let db = self.client.database(&self.database_name)
            .await
            .map_err(|e| ScraperError::StorageError(e.to_string()))?;

        let collection = db.collection("places")
            .await
            .map_err(|e| ScraperError::StorageError(e.to_string()))?;

        let doc = serde_json::to_value(&place)
            .map_err(|e| ScraperError::StorageError(e.to_string()))?;

        collection.insert(doc)
            .await
            .map_err(|e| ScraperError::StorageError(e.to_string()))?;

        Ok(())
    }

    /// Batch store companies
    pub async fn store_companies(&self, companies: Vec<CompanyInfo>) -> ScraperResult<usize> {
        let mut stored = 0;

        for company in companies {
            if self.store_company(company).await.is_ok() {
                stored += 1;
            }
        }

        Ok(stored)
    }

    /// Generate unique ID for company (for deduplication)
    fn generate_company_id(&self, company: &CompanyInfo) -> String {
        use sha2::{Sha256, Digest};

        let mut hasher = Sha256::new();
        hasher.update(company.name.to_lowercase().as_bytes());
        hasher.update(company.location.as_bytes());

        format!("{:x}", hasher.finalize())[..16].to_string()
    }

    /// Query companies by criteria
    pub async fn query_companies(&self, query: &str) -> ScraperResult<Vec<CompanyInfo>> {
        let db = self.client.database(&self.database_name)
            .await
            .map_err(|e| ScraperError::StorageError(e.to_string()))?;

        let collection = db.collection("companies")
            .await
            .map_err(|e| ScraperError::StorageError(e.to_string()))?;

        let results = collection.query(query)
            .execute()
            .await
            .map_err(|e| ScraperError::StorageError(e.to_string()))?;

        let companies: Vec<CompanyInfo> = results
            .into_iter()
            .filter_map(|doc| serde_json::from_value(doc).ok())
            .collect();

        Ok(companies)
    }

    /// Get statistics
    pub async fn get_stats(&self) -> ScraperResult<StorageStats> {
        let db = self.client.database(&self.database_name)
            .await
            .map_err(|e| ScraperError::StorageError(e.to_string()))?;

        // Count companies
        let companies_coll = db.collection("companies")
            .await
            .map_err(|e| ScraperError::StorageError(e.to_string()))?;

        let companies_count = companies_coll
            .query("SELECT COUNT(*) as count FROM companies")
            .execute()
            .await
            .map_err(|e| ScraperError::StorageError(e.to_string()))?
            .first()
            .and_then(|doc| doc["count"].as_u64())
            .unwrap_or(0) as usize;

        // Count jobs
        let jobs_coll = db.collection("jobs")
            .await
            .map_err(|e| ScraperError::StorageError(e.to_string()))?;

        let jobs_count = jobs_coll
            .query("SELECT COUNT(*) as count FROM jobs")
            .execute()
            .await
            .map_err(|e| ScraperError::StorageError(e.to_string()))?
            .first()
            .and_then(|doc| doc["count"].as_u64())
            .unwrap_or(0) as usize;

        Ok(StorageStats {
            total_companies: companies_count,
            total_jobs: jobs_count,
            total_places: 0,
        })
    }
}

#[derive(Debug)]
pub struct StorageStats {
    pub total_companies: usize,
    pub total_jobs: usize,
    pub total_places: usize,
}

/// Data cleaner and normalizer
pub struct DataCleaner;

impl DataCleaner {
    /// Normalize company name
    pub fn normalize_company_name(&self, name: &str) -> String {
        name.trim()
            .to_lowercase()
            // Remove legal entity suffixes
            .replace(", lda", "")
            .replace(", s.a.", "")
            .replace(", sa", "")
            .replace(" llc", "")
            .replace(" inc", "")
            .replace(" ltd", "")
            .replace(" limited", "")
            // Remove extra whitespace
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// Normalize phone number (Portugal format)
    pub fn normalize_phone_pt(&self, phone: &str) -> Option<String> {
        // Remove non-digits
        let digits: String = phone.chars()
            .filter(|c| c.is_ascii_digit())
            .collect();

        // Portugal: country code +351, 9 digits
        if digits.len() == 9 && (digits.starts_with('2') || digits.starts_with('9')) {
            Some(format!("+351{}", digits))
        } else if digits.len() == 12 && digits.starts_with("351") {
            Some(format!("+{}", digits))
        } else {
            None
        }
    }

    /// Clean and categorize industry
    pub fn normalize_industry(&self, industry: &str) -> crate::types::Industry {
        use crate::types::Industry;

        let normalized = industry.to_lowercase();

        if normalized.contains("software")
            || normalized.contains("tech")
            || normalized.contains("tecnologia") {
            Industry::Technology
        } else if normalized.contains("finance")
            || normalized.contains("bank")
            || normalized.contains("finan") {
            Industry::Finance
        } else if normalized.contains("health")
            || normalized.contains("saúde")
            || normalized.contains("medical") {
            Industry::Healthcare
        } else if normalized.contains("retail")
            || normalized.contains("varejo")
            || normalized.contains("comércio") {
            Industry::Retail
        } else if normalized.contains("manufact")
            || normalized.contains("indústria") {
            Industry::Manufacturing
        } else if normalized.contains("educa")
            || normalized.contains("ensino") {
            Industry::Education
        } else if normalized.contains("real estate")
            || normalized.contains("imobiliária") {
            Industry::RealEstate
        } else {
            Industry::Other(industry.to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_company_name() {
        let cleaner = DataCleaner;

        let result = cleaner.normalize_company_name("Example Company, Lda");
        assert_eq!(result, "example company");

        let result2 = cleaner.normalize_company_name("Tech   Inc");
        assert_eq!(result2, "tech");
    }

    #[test]
    fn test_normalize_phone() {
        let cleaner = DataCleaner;

        let result = cleaner.normalize_phone_pt("912345678");
        assert_eq!(result, Some("+351912345678".to_string()));

        let result2 = cleaner.normalize_phone_pt("+351 91 234 5678");
        assert_eq!(result2, Some("+351912345678".to_string()));
    }

    #[test]
    fn test_normalize_industry() {
        let cleaner = DataCleaner;

        assert!(matches!(
            cleaner.normalize_industry("Software Development"),
            crate::types::Industry::Technology
        ));

        assert!(matches!(
            cleaner.normalize_industry("Banking & Finance"),
            crate::types::Industry::Finance
        ));
    }
}
