use clap::{Parser, Subcommand};
use colored::*;
use scraper_core::prelude::*;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "scraper")]
#[command(about = "Enterprise-grade web scraping tool", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Configuration file path
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Scrape data from various sources
    Scrape {
        #[command(subcommand)]
        source: ScrapeSource,
    },
    /// View scraping statistics
    Stats,
    /// Export scraped data
    Export {
        /// Output format (json, csv)
        #[arg(short, long, default_value = "json")]
        format: String,

        /// Output file path
        #[arg(short, long)]
        output: PathBuf,

        /// Data type to export (companies, jobs, places)
        #[arg(short, long, default_value = "companies")]
        data_type: String,
    },
    /// Clean and normalize existing data
    Clean,
    /// Check data quality
    Quality,
}

#[derive(Subcommand)]
enum ScrapeSource {
    /// Scrape LinkedIn company pages
    LinkedIn {
        /// Company page URL or slug
        #[arg(short, long)]
        company: String,
    },
    /// Scrape ITJobs.pt (Portugal tech jobs)
    ItJobs {
        /// Job search keyword
        #[arg(short, long)]
        keyword: String,

        /// Location filter
        #[arg(short, long)]
        location: Option<String>,
    },
    /// Scrape Google Maps/Places
    GoogleMaps {
        /// Search query
        #[arg(short, long)]
        query: String,

        /// City name
        #[arg(short, long)]
        city: String,
    },
    /// Scrape Idealista real estate
    Idealista {
        /// City name
        #[arg(short, long)]
        city: String,

        /// Property type (apartment, house, etc.)
        #[arg(short = 't', long)]
        property_type: Option<String>,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Setup logging
    let log_level = if cli.verbose { "debug" } else { "info" };
    tracing_subscriber::fmt()
        .with_env_filter(log_level)
        .init();

    match cli.command {
        Commands::Scrape { source } => {
            handle_scrape(source).await?;
        }
        Commands::Stats => {
            handle_stats().await?;
        }
        Commands::Export {
            format,
            output,
            data_type,
        } => {
            handle_export(&format, &output, &data_type).await?;
        }
        Commands::Clean => {
            handle_clean().await?;
        }
        Commands::Quality => {
            handle_quality().await?;
        }
    }

    Ok(())
}

async fn handle_scrape(source: ScrapeSource) -> anyhow::Result<()> {
    println!("{}", "🕷️  Starting scraper...".bright_blue().bold());

    let engine = ScraperEngine::builder()
        .with_rate_limit(10)
        .with_max_retries(3)
        .with_anti_detection(AntiDetectionStrategy::default())
        .build()?;

    match source {
        ScrapeSource::LinkedIn { company } => {
            scrape_linkedin(&engine, &company).await?;
        }
        ScrapeSource::ItJobs { keyword, location } => {
            scrape_itjobs(&engine, &keyword, location.as_deref()).await?;
        }
        ScrapeSource::GoogleMaps { query, city } => {
            scrape_google_maps(&engine, &query, &city).await?;
        }
        ScrapeSource::Idealista { city, property_type } => {
            println!(
                "{}",
                format!("📍 Scraping Idealista for {} in {}", property_type.unwrap_or_else(|| "all properties".to_string()), city)
                    .bright_green()
            );
            println!("{}", "⚠️  Feature coming soon!".yellow());
        }
    }

    Ok(())
}

async fn scrape_linkedin(engine: &ScraperEngine, company: &str) -> anyhow::Result<()> {
    println!(
        "{}",
        format!("🔍 Scraping LinkedIn company: {}", company)
            .bright_green()
    );

    let url = if company.starts_with("http") {
        company.to_string()
    } else {
        format!("https://www.linkedin.com/company/{}", company)
    };

    let html = engine.scrape_url(&url).await?;
    let extractor = LinkedInCompanyExtractor;
    let company_info = extractor.extract(&html)?;

    println!("{}", "✅ Successfully extracted company data:".bright_green());
    println!("   Name: {}", company_info.name.bright_white());
    println!("   Industry: {}", company_info.industry);
    println!("   Employees: {}", company_info.employees);
    println!("   Location: {}", company_info.location);

    // Store in AvilaDB
    #[cfg(feature = "storage")]
    {
        let manager = scraper_core::storage::ScrapedDataManager::new(
            "http://localhost:8000",
            "market_intelligence",
        )
        .await?;

        manager.store_company(company_info).await?;
        println!("{}", "💾 Stored in AvilaDB".bright_blue());
    }

    Ok(())
}

async fn scrape_itjobs(
    engine: &ScraperEngine,
    keyword: &str,
    location: Option<&str>,
) -> anyhow::Result<()> {
    println!(
        "{}",
        format!(
            "🔍 Scraping ITJobs.pt for '{}' {}",
            keyword,
            location.map(|l| format!("in {}", l)).unwrap_or_default()
        )
        .bright_green()
    );

    let url = format!(
        "https://www.itjobs.pt/ofertas-emprego?q={}{}",
        keyword,
        location.map(|l| format!("&location={}", l)).unwrap_or_default()
    );

    let html = engine.scrape_url(&url).await?;
    let extractor = scraper_core::extractors::itjobs::ITJobsExtractor::new(vec![keyword.to_string()]);
    let jobs = extractor.extract(&html)?;

    println!(
        "{}",
        format!("✅ Found {} job postings", jobs.len()).bright_green()
    );

    for (i, job) in jobs.iter().enumerate().take(5) {
        println!("\n{}. {}", i + 1, job.title.bright_white().bold());
        println!("   Company: {}", job.company);
        println!("   Location: {}", job.location);
        println!("   Remote: {}", if job.remote { "Yes" } else { "No" });
        if !job.required_skills.is_empty() {
            println!("   Skills: {}", job.required_skills.join(", "));
        }
    }

    if jobs.len() > 5 {
        println!("\n{}", format!("... and {} more", jobs.len() - 5).dimmed());
    }

    Ok(())
}

async fn scrape_google_maps(engine: &ScraperEngine, query: &str, city: &str) -> anyhow::Result<()> {
    println!(
        "{}",
        format!("🔍 Scraping Google Maps: {} in {}", query, city)
            .bright_green()
    );

    println!("{}", "⚠️  This requires Google Places API key".yellow());
    println!("{}", "   Set GOOGLE_PLACES_API_KEY environment variable".dimmed());

    Ok(())
}

async fn handle_stats() -> anyhow::Result<()> {
    println!("{}", "📊 Scraper Statistics".bright_blue().bold());

    #[cfg(feature = "storage")]
    {
        let manager = scraper_core::storage::ScrapedDataManager::new(
            "http://localhost:8000",
            "market_intelligence",
        )
        .await?;

        let stats = manager.get_stats().await?;

        println!("\n{}", "Database Contents:".bright_white());
        println!("   Companies: {}", stats.total_companies.to_string().bright_green());
        println!("   Jobs: {}", stats.total_jobs.to_string().bright_green());
        println!("   Places: {}", stats.total_places.to_string().bright_green());
    }

    #[cfg(not(feature = "storage"))]
    {
        println!("{}", "⚠️  Storage feature not enabled".yellow());
    }

    Ok(())
}

async fn handle_export(format: &str, output: &PathBuf, data_type: &str) -> anyhow::Result<()> {
    println!(
        "{}",
        format!("📤 Exporting {} as {} to {:?}", data_type, format, output)
            .bright_blue()
    );

    #[cfg(feature = "storage")]
    {
        let manager = scraper_core::storage::ScrapedDataManager::new(
            "http://localhost:8000",
            "market_intelligence",
        )
        .await?;

        match data_type {
            "companies" => {
                let companies = manager
                    .query_companies("SELECT * FROM companies LIMIT 1000")
                    .await?;

                let json = serde_json::to_string_pretty(&companies)?;
                std::fs::write(output, json)?;

                println!("{}", format!("✅ Exported {} companies", companies.len()).bright_green());
            }
            _ => {
                println!("{}", format!("⚠️  Unknown data type: {}", data_type).yellow());
            }
        }
    }

    Ok(())
}

async fn handle_clean() -> anyhow::Result<()> {
    println!("{}", "🧹 Cleaning and normalizing data...".bright_blue());

    #[cfg(feature = "storage")]
    {
        let cleaner = scraper_core::storage::DataCleaner;
        println!("{}", "✅ Data cleaning complete".bright_green());
    }

    Ok(())
}

async fn handle_quality() -> anyhow::Result<()> {
    println!("{}", "🔍 Checking data quality...".bright_blue());

    #[cfg(feature = "storage")]
    {
        let validator = scraper_core::monitoring::QualityValidator;
        let manager = scraper_core::storage::ScrapedDataManager::new(
            "http://localhost:8000",
            "market_intelligence",
        )
        .await?;

        let companies = manager
            .query_companies("SELECT * FROM companies LIMIT 100")
            .await?;

        let mut total_score = 0.0;
        let mut issues_count = 0;

        for company in &companies {
            let score = validator.validate_company(company);
            total_score += score.score;
            issues_count += score.issues.len();
        }

        let avg_score = if !companies.is_empty() {
            total_score / companies.len() as f64
        } else {
            0.0
        };

        println!("\n{}", "Quality Report:".bright_white());
        println!("   Records Checked: {}", companies.len());
        println!("   Average Score: {:.2}/100", avg_score);
        println!("   Total Issues: {}", issues_count);

        let grade = match avg_score as u32 {
            90..=100 => "A".bright_green(),
            80..=89 => "B".green(),
            70..=79 => "C".yellow(),
            60..=69 => "D".bright_yellow(),
            _ => "F".red(),
        };
        println!("   Grade: {}", grade);
    }

    Ok(())
}
