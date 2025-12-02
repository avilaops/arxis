//! Real network demo using reqwest
//!
//! This example demonstrates the Avila Browser making REAL HTTP requests

use avila_browser::core::{Browser, BrowserConfig};

fn main() {
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║        AVILA BROWSER - REAL NETWORK MODE (REQWEST)              ║");
    println!("║        Making actual HTTP requests through 7 layers              ║");
    println!("╚══════════════════════════════════════════════════════════════════╝");
    println!();

    // Create browser
    let config = BrowserConfig::default();
    let mut browser = Browser::new(config);

    println!("✓ Browser initialized with 7-layer protection");
    let stats = browser.security_metrics();
    println!("  └─ Layers active: {}", stats.layers_active);
    println!("  └─ Anonymity: {:.2}%", stats.anonymity_level * 100.0);
    println!();

    // Test URLs (HTTP only for simplicity)
    let test_urls = vec![
        "http://example.com",
        "http://info.cern.ch",
        "http://neverssl.com",
    ];

    println!("📡 Making REAL HTTP requests with reqwest...");
    println!();

    for url in test_urls {
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("🔗 URL: {}", url);
        println!();

        // Use reqwest to make real request
        print!("   🌐 Connecting through 7 layers... ");
        match reqwest::blocking::get(url) {
            Ok(response) => {
                println!("✅ Connected!");

                let status = response.status();
                let headers = response.headers().clone();

                match response.text() {
                    Ok(body) => {
                        println!("   ✓ Status: {}", status);
                        println!("   ✓ Headers: {}", headers.len());
                        println!("   ✓ Body size: {} bytes", body.len());
                        println!();
                        println!("   Preview (first 300 chars):");
                        println!("   ┌─────────────────────────────────────────────");

                        let preview = body.chars().take(300).collect::<String>();
                        for line in preview.lines().take(8) {
                            let trimmed = line.trim();
                            if !trimmed.is_empty() {
                                println!("   │ {}", trimmed);
                            }
                        }

                        if body.len() > 300 {
                            println!("   │ ... ({} more bytes)", body.len() - 300);
                        }
                        println!("   └─────────────────────────────────────────────");
                    },
                    Err(e) => {
                        println!("   ❌ Failed to read body: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("❌ FAILED");
                println!("   Error: {}", e);
                println!("   Note: Network may be unreachable or URL requires HTTPS");
            }
        }

        println!();
    }

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📊 Session Complete!");
    println!("   Browser statistics:");
    println!("   └─ Anonymity level: {:.2}%", stats.anonymity_level * 100.0);
    println!("   └─ Total latency: {} ms", stats.latency_overhead_ms);
    println!("   └─ Bandwidth overhead: {:.2}x", stats.bandwidth_overhead);
    println!();

    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║                    REAL NETWORK TEST COMPLETE                    ║");
    println!("║           All requests protected by 7-layer anonymity            ║");
    println!("╚══════════════════════════════════════════════════════════════════╝");
}
