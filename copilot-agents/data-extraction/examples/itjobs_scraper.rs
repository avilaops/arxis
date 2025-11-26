//! Example: Scrape ITJobs.pt for tech jobs in Portugal
//!
//! Usage: cargo run --example itjobs_scraper

use scraper_core::prelude::*;
use scraper_core::extractors::itjobs::ITJobsExtractor;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();

    info!("🕷️  ITJobs.pt Scraper Example - Portugal Tech Jobs");

    let engine = ScraperEngine::builder()
        .with_rate_limit(5)
        .with_max_retries(3)
        .with_anti_detection(AntiDetectionStrategy::default())
        .build()?;

    // Search for Rust jobs in Portugal
    let keywords = vec!["Rust".to_string()];
    let locations = vec!["Lisboa", "Porto", "Remote"];

    for location in locations {
        info!("📍 Searching for Rust jobs in {}", location);

        let url = format!(
            "https://www.itjobs.pt/ofertas-emprego?q=rust&location={}",
            location
        );

        match engine.scrape_url(&url).await {
            Ok(html) => {
                let extractor = ITJobsExtractor::new(keywords.clone());

                match extractor.extract(&html) {
                    Ok(jobs) => {
                        info!("✅ Found {} jobs in {}", jobs.len(), location);

                        for job in jobs.iter().take(3) {
                            info!("\n📋 {}", job.title);
                            info!("   Company: {}", job.company);
                            info!("   Location: {}", job.location);
                            info!("   Remote: {}", if job.remote { "Yes" } else { "No" });

                            if !job.required_skills.is_empty() {
                                info!("   Skills: {}", job.required_skills.join(", "));
                            }

                            // Store in AvilaDB
                            #[cfg(feature = "storage")]
                            {
                                let manager = scraper_core::storage::ScrapedDataManager::new(
                                    "http://localhost:8000",
                                    "market_intelligence",
                                )
                                .await?;

                                manager.store_job(job.clone()).await?;
                            }
                        }

                        if jobs.len() > 3 {
                            info!("\n... and {} more jobs", jobs.len() - 3);
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

        // Delay between locations
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    }

    info!("\n✅ Scraping complete!");

    // Show statistics
    #[cfg(feature = "storage")]
    {
        let manager = scraper_core::storage::ScrapedDataManager::new(
            "http://localhost:8000",
            "market_intelligence",
        )
        .await?;

        let stats = manager.get_stats().await?;
        info!("\n📊 Database Stats:");
        info!("   Total Jobs: {}", stats.total_jobs);
        info!("   Total Companies: {}", stats.total_companies);
    }

    Ok(())
}
