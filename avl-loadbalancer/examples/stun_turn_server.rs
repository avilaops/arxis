//! Exemplo: STUN/TURN Server para Desktop Remoto
//!
//! Demonstra configuração completa de servidor STUN+TURN
//! para NAT traversal em conexões WebRTC de desktop remoto.

use anyhow::Result;
use avl_loadbalancer::stun::{StunServer, TurnServer};
use std::time::Duration;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<()> {
    // Setup logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    println!("🌐 AVL STUN/TURN Server - Remote Desktop NAT Traversal\n");

    // Spawn STUN server for public IP discovery
    println!("1️⃣  Starting STUN server on 0.0.0.0:3478");
    let stun = StunServer::builder()
        .bind("0.0.0.0:3478")
        .build()
        .await?;

    tokio::spawn(async move {
        if let Err(e) = stun.listen().await {
            eprintln!("STUN server error: {}", e);
        }
    });

    println!("   ✅ STUN server running");
    println!("      Clients can discover public IP:port\n");

    // Start TURN server for traffic relay (when P2P fails)
    println!("2️⃣  Starting TURN server on 0.0.0.0:3479");
    let turn = TurnServer::builder()
        .bind("0.0.0.0:3479")
        .realm("avila.cloud")
        .auth_secret("demo-secret-key-change-in-production")
        .allocation_lifetime(Duration::from_secs(600)) // 10 minutes
        .max_allocations(1000)
        .bandwidth_limit(10_000_000) // 10 Mbps per allocation
        .build()
        .await?;

    println!("   ✅ TURN server running");
    println!("      Realm: avila.cloud");
    println!("      Max allocations: 1000");
    println!("      Allocation lifetime: 10 minutes");
    println!("      Bandwidth limit: 10 Mbps/allocation\n");

    println!("🎉 NAT Traversal infrastructure ready!\n");
    println!("💡 Usage:");
    println!("   • Clients connect to STUN (UDP 3478) for IP discovery");
    println!("   • If P2P fails, TURN (UDP 3479) relays traffic");
    println!("   • WebRTC uses ICE to coordinate connection\n");

    println!("📋 ICE Server Configuration (for clients):");
    println!(r#"
    {{
      "iceServers": [
        {{
          "urls": ["stun:your-server-ip:3478"]
        }},
        {{
          "urls": ["turn:your-server-ip:3479"],
          "username": "user",
          "credential": "demo-secret-key-change-in-production"
        }}
      ]
    }}
    "#);

    println!("\n🔐 Security Notes:");
    println!("   • Change auth_secret in production");
    println!("   • Use TLS (STUNS/TURNS) for encrypted signaling");
    println!("   • Implement proper authentication (HMAC-SHA1)");
    println!("   • Monitor bandwidth usage per allocation\n");

    println!("🚀 Server listening... Press Ctrl+C to stop\n");

    // Run TURN server on main thread
    turn.listen().await?;

    Ok(())
}
