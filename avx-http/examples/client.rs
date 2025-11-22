//! Basic HTTP client example

use avx_http::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌐 avx-http - Client Example\n");

    // Create client
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()?;

    // Make GET request to example.com
    println!("📤 Making GET request to http://example.com...");

    match client.get("http://example.com").send().await {
        Ok(response) => {
            println!("✅ Response Status: {}", response.status());
            println!("📊 Response Headers: {} headers", response.headers().len());
            println!("📦 Response Body: {} bytes\n", response.bytes().len());

            if let Ok(text) = response.text().await {
                let preview = if text.len() > 200 {
                    format!("{}...", &text[..200])
                } else {
                    text
                };
                println!("📄 Body Preview:\n{}\n", preview);
            }
        }
        Err(e) => {
            eprintln!("❌ Request failed: {}", e);
        }
    }

    println!("✅ Client example completed!");
    Ok(())
}
