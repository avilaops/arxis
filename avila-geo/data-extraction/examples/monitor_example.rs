//! Example: Data quality monitoring
//!
//! Usage: cargo run --example monitor_example

use scraper_core::prelude::*;
use scraper_core::monitoring::{ScraperMonitor, QualityValidator};
use scraper_core::storage::DataCleaner;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();

    info!("🔍 Data Quality Monitoring Example");

    // Create sample company data
    let company = CompanyInfo {
        id: "test-123".to_string(),
        name: "Example Tech Company, Lda".to_string(),
        industry: "Software Development".to_string(),
        size: CompanySize::Medium,
        employees: 150,
        location: "Lisboa, Portugal".to_string(),
        website: Some("https://example.com".to_string()),
        description: Some("A leading tech company focused on innovative solutions for the enterprise market".to_string()),
        founded_year: Some(2015),
        source: "LinkedIn".to_string(),
        scraped_at: chrono::Utc::now(),
    };

    // Validate data quality
    let validator = QualityValidator;
    let score = validator.validate_company(&company);

    info!("\n📊 Quality Score: {:.2}/100 (Grade: {})", score.score, score.grade());

    if !score.issues.is_empty() {
        info!("\n⚠️  Issues found:");
        for issue in &score.issues {
            info!("   - {}", issue);
        }
    } else {
        info!("✅ No issues found");
    }

    // Clean and normalize data
    info!("\n🧹 Data Cleaning:");
    let cleaner = DataCleaner;

    let clean_name = cleaner.normalize_company_name(&company.name);
    info!("   Original name: {}", company.name);
    info!("   Cleaned name: {}", clean_name);

    let industry = cleaner.normalize_industry(&company.industry);
    info!("   Normalized industry: {:?}", industry);

    // Test phone number cleaning
    let test_phones = vec![
        "912 345 678",
        "+351 91 234 5678",
        "00351912345678",
    ];

    info!("\n📞 Phone Number Cleaning:");
    for phone in test_phones {
        if let Some(clean) = cleaner.normalize_phone_pt(phone) {
            info!("   {} → {}", phone, clean);
        }
    }

    // Create incomplete company for comparison
    let incomplete_company = CompanyInfo {
        id: "test-456".to_string(),
        name: "Test Company".to_string(),
        industry: String::new(),
        size: CompanySize::Small,
        employees: 0,
        location: "Unknown".to_string(),
        website: None,
        description: None,
        founded_year: None,
        source: "Test".to_string(),
        scraped_at: chrono::Utc::now(),
    };

    let incomplete_score = validator.validate_company(&incomplete_company);
    info!("\n📊 Incomplete Company Score: {:.2}/100 (Grade: {})",
        incomplete_score.score,
        incomplete_score.grade()
    );

    info!("\n⚠️  Issues with incomplete company:");
    for issue in &incomplete_score.issues {
        info!("   - {}", issue);
    }

    // Monitoring example
    info!("\n📈 Scraper Monitoring:");
    let mut monitor = ScraperMonitor::new();

    // Simulate some scraping
    use std::time::Duration;
    monitor.record_success(Duration::from_millis(150));
    monitor.record_success(Duration::from_millis(200));
    monitor.record_success(Duration::from_millis(180));
    monitor.record_error("HTTP 404");
    monitor.record_success(Duration::from_millis(220));

    info!("\n{}", monitor.summary());

    let alerts = monitor.check_anomalies();
    if alerts.is_empty() {
        info!("✅ System operating normally");
    }

    Ok(())
}
