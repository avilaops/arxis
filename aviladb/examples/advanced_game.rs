//! Advanced game backend showcasing all AvilaDB features
//!
//! Run with: cargo run --example advanced_game

use aviladb::{
    AvilaClient, Config, Document,
    HierarchicalPartitionKey, PartitionStrategy,
    DistanceMetric, HnswIndex,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
struct PlayerProfile {
    user_id: String,
    username: String,
    level: u32,
    xp: u64,
    inventory: Vec<String>,
    stats: PlayerStats,
}

#[derive(Debug, Serialize, Deserialize)]
struct PlayerStats {
    kills: u32,
    deaths: u32,
    wins: u32,
    playtime_hours: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 AvilaDB Advanced Game Backend Demo\n");

    // Configure with production settings
    let config = Config::default()
        .with_enable_compression(true)
        .with_compression_level(6)
        .with_max_connections(100)
        .with_timeout_secs(30);

    let client = AvilaClient::with_config("http://localhost:8000", config).await?;
    let db = client.database("advanced_gamedb").await?;
    let players = db.collection("players").await?;

    // Example 1: Large document with compression
    println!("1️⃣ Testing large document with compression...");

    let large_inventory: Vec<String> = (0..1000)
        .map(|i| format!("item_{:04}", i))
        .collect();

    let player_doc = Document::new()
        .set("userId", "player_vip_001")
        .set("username", "MegaGamer_BR")
        .set("level", 99)
        .set("xp", 9_999_999_u64)
        .set("inventory", &large_inventory)
        .set("achievements", (0..500).map(|i| format!("achievement_{}", i)).collect::<Vec<_>>());

    let result = players.insert(player_doc).await?;
    println!("   ✓ Document ID: {}", result.id);
    println!("   ✓ Original size: {} bytes ({:.2} KB)", result.size_bytes, result.size_bytes as f64 / 1024.0);
    println!("   ✓ Compression ratio: {:.2}x", result.compression_ratio);
    println!("   ✓ Latency: {}ms\n", result.latency_ms);

    // Example 2: Batch insert
    println!("2️⃣ Batch inserting 100 players...");

    let mut batch = Vec::new();
    for i in 0..100 {
        batch.push(
            Document::new()
                .set("userId", format!("player_{:04}", i))
                .set("username", format!("Player_{}", i))
                .set("level", (i % 50) + 1)
                .set("xp", i as u64 * 1000)
                .set("inventory", vec!["sword", "shield"])
        );
    }

    let batch_results = players.insert_batch(batch).await?;
    let avg_compression: f64 = batch_results.iter()
        .map(|r| r.compression_ratio)
        .sum::<f64>() / batch_results.len() as f64;

    println!("   ✓ Inserted {} players", batch_results.len());
    println!("   ✓ Avg compression: {:.2}x", avg_compression);
    println!("   ✓ Avg latency: {}ms\n",
        batch_results.iter().map(|r| r.latency_ms).sum::<u64>() / batch_results.len() as u64);

    // Example 3: Query
    println!("3️⃣ Querying high-level players...");

    let high_level = players
        .query("SELECT * FROM players WHERE level > @min")
        .param("min", 80)
        .execute()
        .await?;

    println!("   ✓ Found {} players", high_level.documents.len());
    println!("   ✓ Query latency: {}ms\n", high_level.latency_ms);

    // Example 4: Vector search for matchmaking
    println!("4️⃣ Building HNSW index for matchmaking...");

    let mut matchmaking = HnswIndex::new(4, DistanceMetric::Cosine)
        .with_m(16)
        .with_ef_construction(200);

    // Add player skill embeddings
    for i in 0..100 {
        let skill_vector = vec![
            (i as f32 / 100.0),  // skill
            0.5 + (i % 30) as f32 / 60.0,  // aggression
            0.4 + (i % 20) as f32 / 50.0,  // teamwork
            0.6 + (i % 25) as f32 / 50.0,  // objective
        ];
        matchmaking.insert(i, skill_vector)?;
    }

    println!("   ✓ Indexed {} players", matchmaking.len());

    // Find similar players
    let query_player = vec![0.75, 0.65, 0.55, 0.70];
    let similar = matchmaking.search(&query_player, 5, None)?;

    println!("   ✓ Found {} similar players:", similar.len());
    for (idx, result) in similar.iter().enumerate() {
        println!("      {}. Player {} (similarity: {:.3})",
            idx + 1, result.id, 1.0 - result.distance);
    }
    println!();

    // Example 5: Hierarchical Partition Keys
    println!("5️⃣ Demonstrating Hierarchical Partition Keys...");

    let hpk = HierarchicalPartitionKey::triple(
        "game_fps_001",
        "season_2023_q4",
        "player_vip_001"
    );

    println!("   ✓ HPK string: {}", hpk.to_string());
    println!("   ✓ HPK hash: {}", hpk.hash());

    let prefix = HierarchicalPartitionKey::double("game_fps_001", "season_2023_q4");
    println!("   ✓ Prefix matching: {}", prefix.is_prefix_of(&hpk));
    println!("   ✓ Benefit: Efficient queries across tenant hierarchy\n");

    // Example 6: Partition strategy
    println!("6️⃣ Testing partition strategies...");

    let strategy = PartitionStrategy::Hierarchical {
        fields: vec!["gameId".to_string(), "userId".to_string()],
    };
    strategy.validate()?;

    let test_doc = json!({
        "gameId": "game_001",
        "userId": "player_123",
        "data": "test"
    });

    let pk = strategy.extract(&test_doc)?;
    println!("   ✓ Extracted partition key: {}", pk.to_string());
    println!("   ✓ Validation: passed\n");

    println!("✅ All advanced features demonstrated!");
    println!("\n📊 Feature Summary:");
    println!("   • Large documents (4 MB limit) with compression");
    println!("   • Batch operations for high throughput");
    println!("   • SQL-like queries with parameters");
    println!("   • HNSW vector search (matchmaking, recommendations)");
    println!("   • Hierarchical Partition Keys (HPK)");
    println!("   • Flexible partition strategies");
    println!("   • 5-10ms latency in Brazil 🇧🇷");

    Ok(())
}
