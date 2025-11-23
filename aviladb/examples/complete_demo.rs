//! Complete showcase of all AvilaDB SDK features
//!
//! Run with: cargo run --example complete_demo

use aviladb::{
    AvilaClient, Config, Document,
    HierarchicalPartitionKey, PartitionStrategy,
    DistanceMetric, HnswIndex,
};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("═══════════════════════════════════════════════════════");
    println!("  🚀 AvilaDB Rust SDK - Complete Feature Showcase");
    println!("═══════════════════════════════════════════════════════\n");

    // ═══════════════════════════════════════════════════════════
    // CONFIGURATION
    // ═══════════════════════════════════════════════════════════
    println!("⚙️  CONFIGURATION");
    println!("─────────────────────────────────────────────────────");
    
    let config = Config::default();
    // Note: Config is already set up with good defaults:
    // - compression enabled (level 6)
    // - max_connections: 1000
    // - timeout: 60s
    // - cache enabled (TTL: 300s, max: 1000 entries)

    println!("✓ Compression: {} (level {})", 
        config.enable_compression, config.compression_level);
    println!("✓ Max connections: {}", config.max_connections);
    println!("✓ Timeout: {}s", config.request_timeout);
    println!("✓ Cache: {} (TTL: {}s, max: {} entries)\n", 
        if config.enable_cache { "enabled" } else { "disabled" },
        config.cache_ttl, config.max_cache_entries);

    // ═══════════════════════════════════════════════════════════
    // CLIENT & DATABASE SETUP
    // ═══════════════════════════════════════════════════════════
    println!("🔌 CLIENT & DATABASE");
    println!("─────────────────────────────────────────────────────");
    
    let client = AvilaClient::connect("http://localhost:8000").await?;
    println!("✓ Connected to AvilaDB");
    
    let db = client.database("demo_db").await?;
    println!("✓ Database: demo_db");
    
    let products = db.collection("products").await?;
    println!("✓ Collection: products\n");

    // ═══════════════════════════════════════════════════════════
    // DOCUMENT OPERATIONS
    // ═══════════════════════════════════════════════════════════
    println!("📄 DOCUMENT OPERATIONS");
    println!("─────────────────────────────────────────────────────");
    
    // Single insert
    let product = Document::new()
        .set("productId", "PROD-001")
        .set("name", "Gaming Laptop")
        .set("price", 7999.99)
        .set("category", "Electronics")
        .set("specs", json!({
            "cpu": "AMD Ryzen 9 7950X",
            "gpu": "NVIDIA RTX 4090",
            "ram": "64GB DDR5",
            "storage": "2TB NVMe SSD"
        }))
        .set("tags", vec!["gaming", "laptop", "high-end"])
        .set("inStock", true);

    let result = products.insert(product).await?;
    println!("✓ Inserted product: {}", result.id);
    println!("  Size: {} bytes ({:.2} KB)", 
        result.size_bytes, result.size_bytes as f64 / 1024.0);
    println!("  Compression: {:.2}x", result.compression_ratio);
    println!("  Latency: {}ms", result.latency_ms);

    // Batch insert
    println!("\n📦 Batch Operations:");
    let mut batch = Vec::new();
    for i in 0..50 {
        batch.push(
            Document::new()
                .set("productId", format!("PROD-{:03}", i + 2))
                .set("name", format!("Product {}", i + 2))
                .set("price", 100.0 + (i as f64 * 10.0))
                .set("category", if i % 3 == 0 { "Electronics" } else { "Accessories" })
                .set("inStock", i % 2 == 0)
        );
    }

    let batch_results = products.insert_batch(batch).await?;
    let avg_compression: f64 = batch_results.iter()
        .map(|r| r.compression_ratio)
        .sum::<f64>() / batch_results.len() as f64;
    
    println!("✓ Batch inserted {} products", batch_results.len());
    println!("  Avg compression: {:.2}x", avg_compression);
    println!("  Avg latency: {}ms", 
        batch_results.iter().map(|r| r.latency_ms).sum::<u128>() / batch_results.len() as u128);

    // ═══════════════════════════════════════════════════════════
    // QUERY OPERATIONS
    // ═══════════════════════════════════════════════════════════
    println!("\n🔍 QUERY OPERATIONS");
    println!("─────────────────────────────────────────────────────");
    
    let expensive_products = products
        .query("SELECT * FROM products WHERE price > @min_price AND category = @cat")
        .param("min_price", 500.0)
        .param("cat", "Electronics")
        .execute()
        .await?;

    println!("✓ Query: Expensive electronics");
    println!("  Found: {} products", expensive_products.documents.len());
    println!("  Latency: {}ms", expensive_products.latency_ms);
    println!("  Compression: {:.2}x", expensive_products.compression_ratio);

    // ═══════════════════════════════════════════════════════════
    // UPDATE & DELETE OPERATIONS
    // ═══════════════════════════════════════════════════════════
    println!("\n✏️  UPDATE & DELETE OPERATIONS");
    println!("─────────────────────────────────────────────────────");
    
    // Update with safety validation
    let updated = products
        .update()
        .await
        .set("price", 6999.99)
        .set("onSale", true)
        .where_eq("productId", "PROD-001")
        .execute()
        .await?;

    println!("✓ Updated {} product(s)", updated);

    // Delete with safety check
    let deleted = products
        .delete()
        .await
        .where_eq("productId", "PROD-002")
        .execute()
        .await?;

    println!("✓ Deleted {} product(s)", deleted);

    // ═══════════════════════════════════════════════════════════
    // VECTOR SEARCH (HNSW)
    // ═══════════════════════════════════════════════════════════
    println!("\n🎯 VECTOR SEARCH (HNSW Algorithm)");
    println!("─────────────────────────────────────────────────────");
    
    // Create product embeddings (simulated - in production use ML models)
    let mut product_index = HnswIndex::new(4, DistanceMetric::Cosine)
        .with_m(16)
        .with_ef_construction(200);

    println!("Building HNSW index for 100 products...");
    for i in 0..100 {
        // Simulate product features: [price_norm, quality, popularity, availability]
        let embedding = vec![
            (i as f32 % 100.0) / 100.0,  // normalized price
            0.5 + (i as f32 % 50.0) / 100.0,  // quality score
            0.6 + (i as f32 % 40.0) / 100.0,  // popularity
            if i % 2 == 0 { 1.0 } else { 0.5 },  // availability
        ];
        product_index.insert(i, embedding)?;
    }

    println!("✓ Indexed {} products", product_index.len());

    // Find similar products
    let search_embedding = vec![0.75, 0.85, 0.70, 1.0];  // High-end, available product
    let similar = product_index.search(&search_embedding, 5, None)?;

    println!("✓ Top 5 similar products:");
    for (idx, result) in similar.iter().enumerate() {
        println!("  {}. Product {} (similarity: {:.3})", 
            idx + 1, result.id, 1.0 - result.distance);
    }

    // ═══════════════════════════════════════════════════════════
    // HIERARCHICAL PARTITION KEYS
    // ═══════════════════════════════════════════════════════════
    println!("\n🔑 HIERARCHICAL PARTITION KEYS");
    println!("─────────────────────────────────────────────────────");
    
    // Create multi-level partition key
    let hpk = HierarchicalPartitionKey::triple(
        "store_br_sp",      // Store location
        "electronics",      // Category
        "gaming"            // Subcategory
    );

    println!("✓ HPK: {}", hpk.to_string());
    println!("  Hash: {}", hpk.hash());
    println!("  Components: {}", hpk.components().len());

    // Demonstrate prefix matching
    let store_prefix = HierarchicalPartitionKey::single("store_br_sp");
    let category_prefix = HierarchicalPartitionKey::double("store_br_sp", "electronics");

    println!("  Prefix matching:");
    println!("    - store_prefix matches: {}", store_prefix.is_prefix_of(&hpk));
    println!("    - category_prefix matches: {}", category_prefix.is_prefix_of(&hpk));

    // Test partition strategies
    println!("\n📊 Partition Strategies:");
    
    let strategies = vec![
        ("Single", PartitionStrategy::Single { 
            field: "productId".to_string() 
        }),
        ("Hierarchical", PartitionStrategy::Hierarchical { 
            fields: vec!["store".to_string(), "category".to_string()] 
        }),
        ("Synthetic", PartitionStrategy::Synthetic { 
            num_partitions: 100 
        }),
    ];

    for (name, strategy) in strategies {
        strategy.validate()?;
        println!("  ✓ {} strategy validated", name);
    }

    // Extract partition key from document
    let test_doc = json!({
        "productId": "PROD-001",
        "store": "store_br_sp",
        "category": "electronics"
    });

    let hierarchical_strategy = PartitionStrategy::Hierarchical {
        fields: vec!["store".to_string(), "category".to_string()],
    };

    let extracted_pk = hierarchical_strategy.extract(&test_doc)?;
    println!("  ✓ Extracted partition key: {}", extracted_pk.to_string());

    // ═══════════════════════════════════════════════════════════
    // TELEMETRY & STATISTICS
    // ═══════════════════════════════════════════════════════════
    println!("\n📈 TELEMETRY & STATISTICS");
    println!("─────────────────────────────────────────────────────");
    
    let stats = client.stats().await;
    println!("Client Statistics:");
    println!("  HTTP requests: {} (successes: {}, failures: {})", 
        stats.http_requests, stats.http_successes, stats.http_failures);
    println!("  Avg latency: {}ms", stats.avg_latency_ms);
    println!("  Cache hits: {} / misses: {} (hit rate: {:.1}%)", 
        stats.cache_hits, stats.cache_misses, stats.cache_hit_rate * 100.0);

    // ═══════════════════════════════════════════════════════════
    // SUMMARY
    // ═══════════════════════════════════════════════════════════
    println!("\n═══════════════════════════════════════════════════════");
    println!("  ✅ DEMO COMPLETE - All Features Demonstrated!");
    println!("═══════════════════════════════════════════════════════");
    
    println!("\n📋 Features Showcased:");
    println!("  ✓ Configuration with production settings");
    println!("  ✓ Single & batch document operations");
    println!("  ✓ Automatic compression (Brotli)");
    println!("  ✓ SQL-like queries with parameters");
    println!("  ✓ Update & delete with safety validations");
    println!("  ✓ HNSW vector search (O(log N) performance)");
    println!("  ✓ Hierarchical Partition Keys (HPK)");
    println!("  ✓ Flexible partition strategies");
    println!("  ✓ Telemetry & statistics tracking");
    println!("  ✓ Query cache with hit rate monitoring");

    println!("\n🎯 Next Steps:");
    println!("  1. Check examples/ for more use cases");
    println!("  2. Read PHASE3_COMPLETE.md for full documentation");
    println!("  3. Run benchmarks: cargo bench");
    println!("  4. Integrate with your application!");

    println!("\n🇧🇷 Built for Brazil & LATAM with ❤️");
    println!("   Database genuíno da AVL Cloud Platform!");

    Ok(())
}
