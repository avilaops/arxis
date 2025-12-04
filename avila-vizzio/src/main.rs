//! Vizzio - Ultra-Secure Web Browser
//! Replacing Augin with Avila's 7-layer anonymity protection

use avila_browser::core::{Browser, BrowserConfig};

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║                     VIZZIO BROWSER                          ║");
    println!("║         Ultra-Secure Web Browser - Avila Ecosystem         ║");
    println!("║         7-Layer Anonymity Protection                       ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    // Initialize with maximum security
    println!("Initializing Vizzio with Avila's 7-layer protection...\n");

    let config = BrowserConfig {
        num_layers: 7,
        tor_enabled: true,
        vpn_enabled: true,
        i2p_enabled: true,
        obfuscation_enabled: true,
        enable_javascript: false,  // Disabled for security
        block_trackers: true,
        block_advertisements: true,
        ..Default::default()
    };

    let mut browser = Browser::new(config);

    // Display protection layers
    println!("Protection Layers Active:");
    println!("┌─────────────────────────────────────────────────────┐");

    for (i, layer) in browser.layer_stack.layers.iter().enumerate() {
        let status = if layer.enabled { "✓" } else { "✗" };
        println!("│ {} Layer {}: {:?} ({} ms latency)",
            status, i + 1, layer.layer_type, layer.latency_ms);
    }

    println!("└─────────────────────────────────────────────────────┘\n");

    // Security metrics
    let metrics = browser.security_metrics();

    println!("Security Metrics:");
    println!("Active Layers:        {}", metrics.layers_active);
    println!("Anonymity Level:      {:.2}%", metrics.anonymity_level * 100.0);
    println!("Latency Overhead:     {} ms", metrics.latency_overhead_ms);
    println!("Bandwidth Overhead:   {:.2}x", metrics.bandwidth_overhead);
    println!();

    println!("Vizzio ready for secure browsing!");
    println!("Navigate to URLs with maximum privacy and anonymity.");
}
