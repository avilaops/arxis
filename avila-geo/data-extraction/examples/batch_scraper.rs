//! Example: Batch scraping with monitoring
//!
//! Usage: cargo run --example batch_scraper

use scraper_core::prelude::*;
use scraper_core::monitoring::ScraperMonitor;
use tracing::info;
use std::time::Instant;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();

    info!("🕷️  Batch Scraper with Monitoring");

    let engine = ScraperEngine::builder()
        .with_rate_limit(10)
        .with_max_retries(2)
        .with_anti_detection(AntiDetectionStrategy::default())
        .build()?;

    let mut monitor = ScraperMonitor::new();

    // List of companies to scrape
    let company_urls = vec![
        "https://www.linkedin.com/company/microsoft",
        "https://www.linkedin.com/company/google",
        "https://www.linkedin.com/company/apple",
        "https://www.linkedin.com/company/amazon",
        "https://www.linkedin.com/company/meta",
    ];

    info!("📦 Scraping {} companies in batch...", company_urls.len());

    let extractor = LinkedInCompanyExtractor;

    for url in company_urls {
        let start = Instant::now();

        match engine.scrape_url(&url).await {
            Ok(html) => {
                let duration = start.elapsed();
                monitor.record_success(duration);

                match extractor.extract(&html) {
                    Ok(company) => {
                        info!("✅ {} - {}ms", company.name, duration.as_millis());

                        // Store in database
                        #[cfg(feature = "storage")]
                        {
                            let manager = scraper_core::storage::ScrapedDataManager::new(
                                "http://localhost:8000",
                                "market_intelligence",
                            )
                            .await?;

                            manager.store_company(company).await?;
                        }
                    }
                    Err(e) => {
                        monitor.record_error("extraction_error");
                        info!("❌ Extraction error: {}", e);
                    }
                }
            }
            Err(e) => {
                monitor.record_error(&e.to_string());
                info!("❌ Scraping error: {}", e);
            }
        }
    }

    // Show statistics
    info!("\n{}", "=".repeat(60));
    info!("{}", monitor.summary());
    info!("{}", "=".repeat(60));

    // Check for anomalies
    let alerts = monitor.check_anomalies();
    if !alerts.is_empty() {
        info!("\n⚠️  Anomalies detected:");
        for alert in alerts {
            info!("   {}", alert);
        }
    } else {
        info!("\n✅ No anomalies detected");
    }

    Ok(())
}
