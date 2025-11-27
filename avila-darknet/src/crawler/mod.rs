//! Deep web crawler
//!
//! Discover hidden services, crawl .onion sites, extract links

use std::collections::{BTreeMap, BTreeSet, VecDeque};

/// Hidden service discovery
#[derive(Debug, Clone)]
pub struct HiddenService {
    pub onion_address: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub categories: Vec<String>,
    pub last_seen: u64,
    pub status: ServiceStatus,
    pub links: Vec<String>,        // Outbound .onion links
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ServiceStatus {
    Online,
    Offline,
    Unknown,
}

impl HiddenService {
    pub fn new(onion_address: String) -> Self {
        Self {
            onion_address,
            title: None,
            description: None,
            categories: Vec::new(),
            last_seen: current_timestamp(),
            status: ServiceStatus::Unknown,
            links: Vec::new(),
        }
    }
}

/// Web crawler for .onion sites
#[derive(Debug)]
pub struct DeepWebCrawler {
    pub discovered: BTreeMap<String, HiddenService>,
    pub queue: VecDeque<String>,           // URLs to crawl
    pub visited: BTreeSet<String>,
    pub max_depth: usize,
    pub max_pages: usize,
}

impl DeepWebCrawler {
    pub fn new() -> Self {
        Self {
            discovered: BTreeMap::new(),
            queue: VecDeque::new(),
            visited: BTreeSet::new(),
            max_depth: 3,
            max_pages: 10_000,
        }
    }

    /// Seed with known .onion addresses
    pub fn seed(&mut self, seeds: Vec<String>) {
        for seed in seeds {
            if is_valid_onion(&seed) {
                self.queue.push_back(seed);
            }
        }
    }

    /// Crawl next page in queue
    pub fn crawl_next(&mut self) -> Result<HiddenService, CrawlError> {
        let url = self.queue.pop_front()
            .ok_or(CrawlError::EmptyQueue)?;

        if self.visited.contains(&url) {
            return Err(CrawlError::AlreadyVisited);
        }

        if self.visited.len() >= self.max_pages {
            return Err(CrawlError::MaxPagesReached);
        }

        // Mark as visited
        self.visited.insert(url.clone());

        // Fetch page (simulated)
        let page = self.fetch_page(&url)?;

        // Extract metadata
        let mut service = HiddenService::new(url.clone());
        service.title = extract_title(&page);
        service.description = extract_description(&page);
        service.categories = categorize(&page);
        service.status = ServiceStatus::Online;
        service.links = extract_onion_links(&page);

        // Add discovered links to queue
        for link in &service.links {
            if !self.visited.contains(link) && !self.queue.contains(link) {
                self.queue.push_back(link.clone());
            }
        }

        // Store
        self.discovered.insert(url, service.clone());

        Ok(service)
    }

    /// Crawl all pages (BFS)
    pub fn crawl_all(&mut self) -> usize {
        let mut crawled = 0;

        while !self.queue.is_empty() && crawled < self.max_pages {
            match self.crawl_next() {
                Ok(_) => crawled += 1,
                Err(CrawlError::AlreadyVisited) => continue,
                Err(_) => break,
            }
        }

        crawled
    }

    /// Fetch page content (simulated - production: use Tor circuit)
    fn fetch_page(&self, url: &str) -> Result<String, CrawlError> {
        // Production: Send HTTP request through Tor circuit
        // For now: Simulate page content

        if url.contains("marketplace") {
            Ok(r#"
                <html>
                <head><title>Dark Marketplace</title></head>
                <body>
                    <h1>Welcome to Marketplace</h1>
                    <p>Buy and sell anonymously</p>
                    <a href="vendor1.onion">Vendor 1</a>
                    <a href="vendor2.onion">Vendor 2</a>
                </body>
                </html>
            "#.to_string())
        } else if url.contains("forum") {
            Ok(r#"
                <html>
                <head><title>Anonymous Forum</title></head>
                <body>
                    <h1>Discussion Forum</h1>
                    <a href="tech-board.onion">Tech Board</a>
                </body>
                </html>
            "#.to_string())
        } else {
            Ok("<html><head><title>Hidden Service</title></head><body></body></html>".to_string())
        }
    }

    /// Get statistics
    pub fn stats(&self) -> CrawlStats {
        CrawlStats {
            discovered: self.discovered.len(),
            visited: self.visited.len(),
            queued: self.queue.len(),
            online: self.discovered.values()
                .filter(|s| s.status == ServiceStatus::Online)
                .count(),
        }
    }
}

#[derive(Debug)]
pub enum CrawlError {
    EmptyQueue,
    AlreadyVisited,
    MaxPagesReached,
    FetchFailed,
    InvalidUrl,
}

#[derive(Debug)]
pub struct CrawlStats {
    pub discovered: usize,
    pub visited: usize,
    pub queued: usize,
    pub online: usize,
}

// ============================================================================
// HTML Parsing (simplified)
// ============================================================================

fn extract_title(html: &str) -> Option<String> {
    // Regex would be better, but keeping zero deps
    if let Some(start) = html.find("<title>") {
        if let Some(end) = html[start..].find("</title>") {
            let title = &html[start + 7..start + end];
            return Some(title.to_string());
        }
    }
    None
}

fn extract_description(html: &str) -> Option<String> {
    // Look for meta description
    if let Some(pos) = html.find("name=\"description\"") {
        if let Some(content_start) = html[pos..].find("content=\"") {
            let start = pos + content_start + 9;
            if let Some(end) = html[start..].find("\"") {
                return Some(html[start..start + end].to_string());
            }
        }
    }
    None
}

fn extract_onion_links(html: &str) -> Vec<String> {
    let mut links = Vec::new();

    // Find all <a href="...">
    let mut pos = 0;
    while let Some(href_pos) = html[pos..].find("href=\"") {
        let start = pos + href_pos + 6;
        if let Some(end) = html[start..].find("\"") {
            let link = &html[start..start + end];

            // Only .onion links
            if link.ends_with(".onion") || link.contains(".onion/") {
                let onion = extract_onion_domain(link);
                if !links.contains(&onion) {
                    links.push(onion);
                }
            }

            pos = start + end;
        } else {
            break;
        }
    }

    links
}

fn extract_onion_domain(url: &str) -> String {
    // Extract just the .onion domain
    if let Some(start) = url.find("://") {
        let after_protocol = &url[start + 3..];
        if let Some(end) = after_protocol.find("/") {
            after_protocol[..end].to_string()
        } else {
            after_protocol.to_string()
        }
    } else {
        url.split("/").next().unwrap_or(url).to_string()
    }
}

fn categorize(html: &str) -> Vec<String> {
    let mut categories = Vec::new();

    let html_lower = html.to_lowercase();

    // Keyword-based categorization
    if html_lower.contains("marketplace") || html_lower.contains("shop") || html_lower.contains("buy") {
        categories.push("marketplace".to_string());
    }

    if html_lower.contains("forum") || html_lower.contains("discussion") || html_lower.contains("board") {
        categories.push("forum".to_string());
    }

    if html_lower.contains("wiki") || html_lower.contains("encyclopedia") {
        categories.push("wiki".to_string());
    }

    if html_lower.contains("blog") || html_lower.contains("news") {
        categories.push("blog".to_string());
    }

    if html_lower.contains("email") || html_lower.contains("mail") {
        categories.push("email".to_string());
    }

    if categories.is_empty() {
        categories.push("unknown".to_string());
    }

    categories
}

fn is_valid_onion(url: &str) -> bool {
    // v3 onion: 56 chars + .onion
    url.ends_with(".onion")
}

fn current_timestamp() -> u64 {
    // Production: actual timestamp
    1700000000
}

/// HSDir (Hidden Service Directory) lookup
#[derive(Debug)]
pub struct HSDir {
    pub descriptors: BTreeMap<String, ServiceDescriptor>,
}

#[derive(Debug, Clone)]
pub struct ServiceDescriptor {
    pub onion_address: String,
    pub public_key: [u8; 32],
    pub introduction_points: Vec<IntroPoint>,
    pub published_at: u64,
}

#[derive(Debug, Clone)]
pub struct IntroPoint {
    pub node_id: [u8; 32],
    pub ip: [u8; 4],
    pub port: u16,
}

impl HSDir {
    pub fn new() -> Self {
        Self {
            descriptors: BTreeMap::new(),
        }
    }

    /// Lookup service descriptor
    pub fn lookup(&self, onion_address: &str) -> Option<&ServiceDescriptor> {
        self.descriptors.get(onion_address)
    }

    /// Store descriptor (published by hidden service)
    pub fn store(&mut self, descriptor: ServiceDescriptor) {
        self.descriptors.insert(descriptor.onion_address.clone(), descriptor);
    }

    /// List all known .onion addresses
    pub fn list_all(&self) -> Vec<String> {
        self.descriptors.keys().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crawler_creation() {
        let crawler = DeepWebCrawler::new();
        assert_eq!(crawler.discovered.len(), 0);
    }

    #[test]
    fn test_seed() {
        let mut crawler = DeepWebCrawler::new();
        crawler.seed(vec![
            "marketplace.onion".to_string(),
            "forum.onion".to_string(),
        ]);

        assert_eq!(crawler.queue.len(), 2);
    }

    #[test]
    fn test_extract_title() {
        let html = "<html><title>Test Site</title></html>";
        let title = extract_title(html);
        assert_eq!(title, Some("Test Site".to_string()));
    }

    #[test]
    fn test_extract_onion_links() {
        let html = r#"<a href="site1.onion">Link 1</a><a href="http://site2.onion/page">Link 2</a>"#;
        let links = extract_onion_links(html);
        assert_eq!(links.len(), 2);
        assert!(links.contains(&"site1.onion".to_string()));
    }

    #[test]
    fn test_categorize() {
        let html = "<html>Welcome to our marketplace</html>";
        let categories = categorize(html);
        assert!(categories.contains(&"marketplace".to_string()));
    }
}
