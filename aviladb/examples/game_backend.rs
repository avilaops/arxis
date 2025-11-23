//! Game backend example with AvilaDB
//!
//! Run with: cargo run --example game_backend

use aviladb::{AvilaClient, Document};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎮 AvilaDB Game Backend Example\n");

    let client = AvilaClient::connect("http://localhost:8000").await?;
    let db = client.database("gamedb").await?;

    // Player profiles
    let players = db.collection("players").await?;
    println!("✓ Collection: players");

    // Leaderboards
    let leaderboard = db.collection("leaderboard").await?;
    println!("✓ Collection: leaderboard");

    // Game sessions
    let sessions = db.collection("sessions").await?;
    println!("✓ Collection: sessions\n");

    // Create player profile
    println!("Creating player profile...");
    let player = Document::new()
        .set("userId", "player_br_001")
        .set("username", "BrazilianWarrior")
        .set("level", 42)
        .set("experience", 15750)
        .set("class", "Warrior")
        .set("region", "sa-east-1") // São Paulo
        .set("inventory", json!({
            "weapons": ["legendary_sword", "epic_shield"],
            "armor": ["mythic_helmet", "legendary_chest"],
            "consumables": {
                "health_potions": 15,
                "mana_potions": 10
            }
        }))
        .set("stats", json!({
            "hp": 250,
            "mp": 100,
            "attack": 75,
            "defense": 60,
            "speed": 45
        }))
        .set("achievements", vec!["dragon_slayer", "treasure_hunter"])
        .set("lastLogin", chrono::Utc::now().to_rfc3339());

    let result = players.insert(player).await?;
    println!("✓ Player created: {}", result.id);
    println!("  Latency: {} ms (5-10ms typical in Brazil!)\n", result.latency_ms);

    // Add to leaderboard
    println!("Adding to leaderboard...");
    let leaderboard_entry = Document::new()
        .set("userId", "player_br_001")
        .set("username", "BrazilianWarrior")
        .set("score", 15750)
        .set("rank", 42)
        .set("region", "Brazil")
        .set("updatedAt", chrono::Utc::now().to_rfc3339());

    leaderboard.insert(leaderboard_entry).await?;
    println!("✓ Leaderboard entry added\n");

    // Create game session
    println!("Creating game session...");
    let session = Document::new()
        .set("sessionId", uuid::Uuid::new_v4().to_string())
        .set("userId", "player_br_001")
        .set("serverId", "br-sp-01") // São Paulo server
        .set("gameMode", "ranked")
        .set("status", "active")
        .set("startTime", chrono::Utc::now().to_rfc3339())
        .set("matchData", json!({
            "map": "ancient_ruins",
            "difficulty": "hard",
            "team": ["player_br_001", "player_br_002", "player_br_003"]
        }));

    sessions.insert(session).await?;
    println!("✓ Game session created\n");

    // Query top players in Brazil
    println!("Querying top 10 Brazilian players...");
    let top_players = leaderboard
        .query("SELECT * FROM leaderboard WHERE region = @region ORDER BY score DESC LIMIT 10")
        .param("region", "Brazil")
        .execute()
        .await?;

    println!("✓ Found {} top players", top_players.total_count);
    println!("  Query latency: {} ms\n", top_players.latency_ms);

    // Update player stats after battle
    println!("Updating player stats after battle...");
    players
        .update()
        .await
        .set("experience", 16000)
        .set("level", 43)
        .set("stats", json!({
            "hp": 260,
            "mp": 105,
            "attack": 77,
            "defense": 62,
            "speed": 46
        }))
        .where_eq("userId", "player_br_001")
        .execute()
        .await?;

    println!("✓ Player stats updated\n");

    println!("🎉 Game backend example complete!\n");
    println!("💡 Key features demonstrated:");
    println!("   ✓ Player profiles with complex nested data");
    println!("   ✓ Leaderboards with rankings");
    println!("   ✓ Real-time game sessions");
    println!("   ✓ Fast queries (5-10ms in Brazil)");
    println!("   ✓ 4 MB documents (store entire match data!)");

    Ok(())
}
