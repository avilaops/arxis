//! Game Leaderboard com AvilaDB
//!
//! Sistema de ranking em tempo real com:
//! - Inserção de alta performance
//! - Queries ordenadas otimizadas
//! - Cache automático
//! - Telemetria

use aviladb::{AvilaClient, Document};
use rand::Rng;
use std::time::Instant;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎮 Game Leaderboard - AvilaDB Demo\n");

    let client = AvilaClient::connect("http://localhost:8000").await?;
    let db = client.database("gamedb");
    let leaderboard = db.collection("leaderboard");

    println!("📊 Inicializando leaderboard...");

    // Simular 10,000 players
    let start = Instant::now();
    let mut rng = rand::thread_rng();

    for i in 0..10_000 {
        let score = rng.gen_range(0..1_000_000);
        let player = Document::new()
            .set("playerId", format!("player_{}", i))
            .set("username", format!("Player{}", i))
            .set("score", score)
            .set("level", rng.gen_range(1..100))
            .set("country", match i % 5 {
                0 => "BR",
                1 => "US",
                2 => "UK",
                3 => "JP",
                _ => "DE",
            })
            .set("lastActive", chrono::Utc::now().to_rfc3339());

        leaderboard.insert(player).await?;

        if i % 1000 == 0 {
            print!(".");
            std::io::Write::flush(&mut std::io::stdout())?;
        }
    }

    let elapsed = start.elapsed();
    println!("\n✅ 10,000 players inseridos em {:?}", elapsed);
    println!("📈 Throughput: {:.0} docs/s\n", 10_000.0 / elapsed.as_secs_f64());

    // Top 10 Global
    println!("🏆 TOP 10 GLOBAL");
    let start = Instant::now();
    let top10 = leaderboard
        .query("SELECT * FROM leaderboard ORDER BY score DESC LIMIT 10")
        .execute()
        .await?;
    let query_time = start.elapsed();

    for (i, player) in top10.iter().enumerate() {
        println!("  {}. {} - {:>9} pts (Level {})",
            i + 1,
            player["username"],
            player["score"],
            player["level"]
        );
    }
    println!("⏱️  Query time: {:?}\n", query_time);

    // Top 10 por país
    println!("🇧🇷 TOP 10 BRASIL");
    let start = Instant::now();
    let top10_br = leaderboard
        .query("SELECT * FROM leaderboard WHERE country = @country ORDER BY score DESC LIMIT 10")
        .param("country", "BR")
        .execute()
        .await?;
    let query_time = start.elapsed();

    for (i, player) in top10_br.iter().enumerate() {
        println!("  {}. {} - {:>9} pts",
            i + 1,
            player["username"],
            player["score"]
        );
    }
    println!("⏱️  Query time: {:?}\n", query_time);

    // Estatísticas
    println!("📊 ESTATÍSTICAS");
    let stats = leaderboard
        .query("SELECT country, COUNT(*) as players, AVG(score) as avg_score, MAX(score) as max_score FROM leaderboard GROUP BY country")
        .execute()
        .await?;

    for stat in stats {
        println!("  {} - {} players | Avg: {:.0} | Max: {}",
            stat["country"],
            stat["players"],
            stat["avg_score"],
            stat["max_score"]
        );
    }
    println!();

    // Simular updates em tempo real
    println!("⚡ Simulando updates em tempo real (5 segundos)...");
    let start_sim = Instant::now();
    let mut updates = 0;

    while start_sim.elapsed() < Duration::from_secs(5) {
        let player_id = format!("player_{}", rng.gen_range(0..10_000));
        let new_score = rng.gen_range(0..1_000_000);

        leaderboard
            .update(
                &player_id,
                Document::new()
                    .set("score", new_score)
                    .set("lastActive", chrono::Utc::now().to_rfc3339())
            )
            .await?;

        updates += 1;
        sleep(Duration::from_millis(10)).await;
    }

    println!("✅ {} updates executados", updates);
    println!("📈 Update rate: {:.0} updates/s\n", updates as f64 / 5.0);

    // Top 10 atualizado
    println!("🏆 TOP 10 ATUALIZADO");
    let start = Instant::now();
    let top10_updated = leaderboard
        .query("SELECT * FROM leaderboard ORDER BY score DESC LIMIT 10")
        .execute()
        .await?;
    let query_time = start.elapsed();

    for (i, player) in top10_updated.iter().enumerate() {
        println!("  {}. {} - {:>9} pts",
            i + 1,
            player["username"],
            player["score"]
        );
    }
    println!("⏱️  Query time: {:?}\n", query_time);

    // Telemetria final
    let diagnostics = client.diagnostics().await;
    println!("📈 MÉTRICAS FINAIS");
    println!("  Total requests: {}", diagnostics.total_requests);
    println!("  Avg latency: {:?}", diagnostics.avg_latency);
    println!("  Cache hit rate: {:.1}%", diagnostics.cache_hit_rate * 100.0);
    println!("  Throughput: {:.0} req/s", diagnostics.requests_per_second);

    println!("\n🎉 Demo concluída!");

    Ok(())
}
