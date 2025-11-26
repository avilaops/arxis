//! Example: Scrape LinkedIn company pages
//!
//! Usage: cargo run --example linkedin_scraper

use scraper_core::prelude::*;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Setup logging
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();

    info!("🕷️  LinkedIn Company Scraper Example");

    // Build scraper engine
    let engine = ScraperEngine::builder()
        .with_rate_limit(5) // 5 requests per second
        .with_max_retries(3)
        .with_anti_detection(AntiDetectionStrategy::default())
        .build()?;

    // Example companies to scrape
    let companies = vec![
        "microsoft",
        "google",
        "apple",
    ];

    info!("Scraping {} companies...", companies.len());

    for company_slug in companies {
        let url = format!("https://www.linkedin.com/company/{}", company_slug);

        info!("📍 Scraping: {}", url);

        match engine.scrape_url(&url).await {
            Ok(html) => {
                let extractor = LinkedInCompanyExtractor;

                match extractor.extract(&html) {
                    Ok(company) => {
                        info!("✅ Successfully scraped: {}", company.name);
                        info!("   Industry: {}", company.industry);
                        info!("   Employees: {}", company.employees);
                        info!("   Location: {}", company.location);

                        // Validate data quality
                        if extractor.validate(&company) {
                            info!("   ✅ Data quality: OK");
                        } else {
                            info!("   ⚠️  Data quality: Issues detected");
                        }

                        // Store in AvilaDB
                        #[cfg(feature = "storage")]
                        {
                            let manager = scraper_core::storage::ScrapedDataManager::new(
                                "http://localhost:8000",
                                "market_intelligence",
                            )
                            .await?;

                            manager.store_company(company).await?;
                            info!("   💾 Stored in AvilaDB");
                        }
                    }
                    Err(e) => {
                        info!("❌ Extraction failed: {}", e);
                    }
                }
            }
            Err(e) => {
                info!("❌ Scraping failed: {}", e);
            }
        }

        // Delay between requests
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    }

    info!("✅ Scraping complete!");

    Ok(())
}
