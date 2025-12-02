//! Real network demo - Making actual HTTP requests through 7 layers
//!
//! This example demonstrates the Avila Browser making REAL network requests
//! through all 7 protection layers.

use avila_browser::core::{Browser, BrowserConfig};

fn main() {
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║        AVILA BROWSER - REAL NETWORK MODE                         ║");
    println!("║        Making actual HTTP requests through 7 layers              ║");
    println!("╚══════════════════════════════════════════════════════════════════╝");
    println!();

    // Create browser with maximum security
    let config = BrowserConfig::default();
    let mut browser = Browser::new(config);

    println!("✓ Browser initialized with 7-layer protection");
    println!();

    // List of test URLs
    let test_urls = vec![
        "http://example.com",
        "http://info.cern.ch",
        "http://neverssl.com",
    ];

    println!("📡 Testing real network requests...");
    println!();

    for url in test_urls {
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("🔗 URL: {}", url);
        println!();

        match browser.navigate(url) {
            Ok(response) => {
                println!("✅ SUCCESS!");
                println!("   Status: {}", response.status_code);
                println!("   Body size: {} bytes", response.body.len());
                
                let body_str = response.body_as_string();
                let preview_len = body_str.len().min(200);
                println!("   Preview:");
                println!("   ┌─────────────────────────────────────────────");
                for line in body_str[..preview_len].lines().take(5) {
                    println!("   │ {}", line);
                }
                if body_str.len() > preview_len {
                    println!("   │ ... ({} more bytes)", body_str.len() - preview_len);
                }
                println!("   └─────────────────────────────────────────────");
            }
            Err(e) => {
                println!("❌ FAILED: {:?}", e);
                println!("   Note: Some URLs may require HTTPS or be unreachable");
            }
        }

        println!();
    }

    // Show statistics
    let stats = browser.security_metrics();
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📊 Session Statistics:");
    println!("   Requests made: {}", browser.history.len());
    println!("   Cache size: {}", browser.cache.len());
    println!("   Active layers: {}", stats.layers_active);
    println!("   Anonymity level: {:.2}%", stats.anonymity_level * 100.0);
    println!("   Total latency: {} ms", stats.latency_overhead_ms);
    println!("   Bandwidth overhead: {:.2}x", stats.bandwidth_overhead);
    println!();

    // Clear data
    println!("🧹 Clearing browsing data...");
    browser.clear_data();
    println!("✓ All traces removed");
    println!();

    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║                    REAL NETWORK TEST COMPLETE                    ║");
    println!("║            All requests routed through 7 layers                  ║");
    println!("╚══════════════════════════════════════════════════════════════════╝");
}
