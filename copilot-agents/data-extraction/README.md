# 🕷️ Data Extraction & Web Scraping Agent

**Enterprise-grade web scraping toolkit** for automated data collection, specialized in Portugal & LATAM market intelligence.

## 🎯 Features

- ✅ **Robust Scraping**: Rate limiting, retries, proxy rotation
- ✅ **Multi-Source**: LinkedIn, Google Maps, ITJobs, Idealista, and more
- ✅ **AvilaDB Integration**: Automatic storage with deduplication
- ✅ **Anti-Detection**: User-agent rotation, request delays, robots.txt compliance
- ✅ **JavaScript Support**: Headless Chrome for SPA scraping
- ✅ **Data Quality**: Validation, normalization, cleaning
- ✅ **Monitoring**: Real-time metrics and alerting
- ✅ **Ethical**: GDPR-compliant, respects ToS

## 🚀 Quick Start

### Installation

```bash
# Install Rust (if not already)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone repository
git clone https://github.com/avelan/data-extraction-agent
cd data-extraction-agent

# Build project
cargo build --release
```

### Basic Usage

```rust
use scraper_core::{ScraperEngine, AntiDetectionStrategy, extractors::LinkedInCompanyExtractor};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize scraper
    let scraper = ScraperEngine::builder()
        .with_rate_limit(10) // 10 requests per second
        .with_anti_detection(AntiDetectionStrategy::default())
        .build()?;

    // Scrape LinkedIn company
    let html = scraper.scrape_url("https://www.linkedin.com/company/example").await?;

    // Extract data
    let extractor = LinkedInCompanyExtractor::new();
    let company = extractor.extract(&html)?;

    println!("Company: {:?}", company);

    Ok(())
}
```

## 📦 Project Structure

```
data-extraction/
├── scraper-core/          # Core scraping engine
│   ├── src/
│   │   ├── engine.rs      # Main scraper engine
│   │   ├── extractors/    # Data extractors
│   │   ├── storage/       # AvilaDB integration
│   │   ├── anti_detect/   # Anti-detection strategies
│   │   └── monitoring/    # Metrics & quality control
├── scraper-cli/           # CLI tool
│   └── src/
│       └── main.rs
├── examples/              # Usage examples
│   ├── linkedin_scraper.rs
│   ├── itjobs_scraper.rs
│   └── idealista_scraper.rs
└── config/
    └── default.toml       # Configuration
```

## 🔧 CLI Usage

```bash
# Scrape company from LinkedIn
cargo run --bin scraper-cli -- scrape linkedin --company "example-company"

# Scrape jobs from ITJobs Portugal
cargo run --bin scraper-cli -- scrape itjobs --keyword "rust developer" --location "Lisboa"

# Scrape real estate from Idealista
cargo run --bin scraper-cli -- scrape idealista --city "Porto" --type "apartment"

# View scraping statistics
cargo run --bin scraper-cli -- stats

# Export scraped data
cargo run --bin scraper-cli -- export --format json --output data.json
```

## 🌍 Supported Sources

### Portugal-Specific
- **ITJobs.pt** - Tech job listings
- **Idealista** - Real estate listings
- **Racius** - Company information
- **INE Portugal** - Statistics
- **Pordata** - Comprehensive data

### International
- **LinkedIn** - Company profiles, job postings
- **Google Maps** - Business listings, reviews
- **Crunchbase** - Funding data
- **GitHub** - Open source activity

## 📊 AvilaDB Integration

Store scraped data efficiently with automatic deduplication:

```rust
use scraper_core::storage::ScrapedDataManager;
use aviladb::AvilaClient;

let client = AvilaClient::connect("http://localhost:8000").await?;
let db = client.database("market_intelligence").await?;

let manager = ScrapedDataManager::new(db);

// Store with automatic deduplication
manager.store_company(company_data).await?;
```

## ⚖️ Ethical Guidelines

This toolkit enforces ethical scraping:

1. ✅ **Respects robots.txt** automatically
2. ✅ **Rate limiting** prevents server overload
3. ✅ **User-agent identification** for transparency
4. ✅ **GDPR compliance** for personal data
5. ✅ **ToS compliance** checking

## 🔒 Configuration

Create `config/local.toml`:

```toml
[scraper]
rate_limit_per_second = 10
max_concurrent_requests = 5
request_timeout_seconds = 30
max_retries = 3

[proxy]
enabled = true
rotation = "round_robin"
proxies = [
    "http://proxy1.example.com:8080",
    "http://proxy2.example.com:8080"
]

[aviladb]
connection_string = "http://localhost:8000"
database = "market_intelligence"
collection = "companies"

[anti_detection]
randomize_delays = true
min_delay_ms = 500
max_delay_ms = 2000
rotate_user_agents = true
```

## 📈 Monitoring

Real-time metrics dashboard:

```rust
let monitor = scraper.get_monitor();

println!("URLs scraped: {}", monitor.urls_scraped());
println!("Success rate: {:.2}%", monitor.success_rate() * 100.0);
println!("Avg response time: {:.2}ms", monitor.avg_response_time_ms());
println!("Data quality score: {:.2}", monitor.data_quality_score());
```

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_linkedin_extractor

# Run with logging
RUST_LOG=debug cargo test
```

## 📝 License

MIT License - See LICENSE file for details

## 🤝 Contributing

Contributions welcome! Please read CONTRIBUTING.md first.

---

Built with ❤️ by the Avelan Team for Portugal's digital transformation 🇵🇹
