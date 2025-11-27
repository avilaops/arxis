//! Example: Hidden service (.onion) setup

use avila_darknet::identity::{IdentityManager, IntroductionPoint};

fn main() {
    println!("=== Hidden Service (.onion) Example ===\n");

    let mut manager = IdentityManager::new();

    // 1. Create anonymous identity
    println!("Creating anonymous identity...");
    let identity = manager.create_identity("SecureMarketplace".to_string());

    println!("  Pseudonym: {}", identity.pseudonym);
    println!("  Onion address: {}", identity.onion_address);
    println!("  Public key: {:02x}{:02x}...{:02x}{:02x}",
        identity.keypair.public_key[0],
        identity.keypair.public_key[1],
        identity.keypair.public_key[30],
        identity.keypair.public_key[31]
    );

    // 2. Create hidden service
    println!("\nCreating hidden service...");
    let onion_address = manager.create_hidden_service("MyMarketplace".to_string());

    println!("  .onion address: {}", onion_address);

    // 3. Setup introduction points
    println!("\nSetting up introduction points...");

    let mut intro_points = Vec::new();
    for i in 0..3 {
        let intro = IntroductionPoint {
            node_id: [(i + 1) as u8; 32],
            public_key: [(i + 10) as u8; 32],
            ip: [127, 0, 0, i + 1],
            port: 9001 + i as u16,
        };

        println!("  Intro point {}: {}:{}",
            i + 1,
            intro.ip.iter().map(|b| b.to_string()).collect::<Vec<_>>().join("."),
            intro.port
        );

        intro_points.push(intro);
    }

    let service = manager.hidden_services.get_mut("MyMarketplace").unwrap();
    service.setup_introduction_points(intro_points);

    // 4. Publish descriptor
    println!("\nPublishing service descriptor...");
    let descriptor = service.publish_descriptor();

    println!("  Published at: {}", descriptor.published_at);
    println!("  Introduction points: {}", descriptor.introduction_points.len());
    println!("  Signature: {:02x}{:02x}...{:02x}{:02x}",
        descriptor.signature[0],
        descriptor.signature[1],
        descriptor.signature[62],
        descriptor.signature[63]
    );

    // 5. Simulate client connection
    println!("\nSimulating client connection...");
    let client_cookie = [42u8; 32];
    let rendezvous = service.handle_client(client_cookie);

    println!("  Rendezvous established");
    println!("  Cookie: {:02x}{:02x}...{:02x}{:02x}",
        rendezvous.cookie[0],
        rendezvous.cookie[1],
        rendezvous.cookie[30],
        rendezvous.cookie[31]
    );

    // 6. Lookup service
    println!("\nLooking up service by .onion address...");
    if let Some(found) = manager.lookup_service(&onion_address) {
        println!("  Found service: {}", found.identity.pseudonym);
        println!("  Reputation: {}", found.identity.reputation);
    }

    println!("\n=== Hidden Service Ready ===");
    println!("Access at: http://{}", onion_address);
}
