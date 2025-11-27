//! Example: Create Tor circuit and send data

use avila_darknet::tor::{TorNode, NodeRole, OnionRouter};

fn main() {
    println!("=== Tor Circuit Example ===\n");

    // 1. Create router
    let mut router = OnionRouter::new();

    // 2. Populate directory with nodes
    println!("Adding nodes to directory...");

    for i in 0..5 {
        let role = match i % 3 {
            0 => NodeRole::Guard,
            1 => NodeRole::Middle,
            _ => NodeRole::Exit,
        };

        let mut node = TorNode::new(role);
        node.id = [i as u8; 32];
        node.bandwidth = (i + 1) as u64 * 500_000;

        router.directory.push(node);
        println!("  Added {:?} node {} (bandwidth: {} KB/s)",
            role, i, (i + 1) * 500);
    }

    // 3. Build circuit
    println!("\nBuilding 3-hop circuit...");
    let circuit_id = router.build_circuit().unwrap();
    println!("  Circuit ID: {}", circuit_id);

    let circuit = &router.circuits[&circuit_id];
    println!("  Guard:  Node {:?}", circuit.hops[0].id[0]);
    println!("  Middle: Node {:?}", circuit.hops[1].id[0]);
    println!("  Exit:   Node {:?}", circuit.hops[2].id[0]);

    // 4. Send data through circuit
    println!("\nSending data through circuit...");
    let message = b"GET / HTTP/1.1\r\nHost: example.com\r\n\r\n";

    match router.send_through_circuit(circuit_id, message) {
        Ok(response) => {
            println!("  Sent {} bytes", message.len());
            println!("  Received {} bytes", response.len());

            let circuit = &router.circuits[&circuit_id];
            println!("  Total sent: {} bytes", circuit.bytes_sent);
        }
        Err(e) => {
            println!("  Error: {:?}", e);
        }
    }

    // 5. Circuit stats
    println!("\nCircuit statistics:");
    for (id, circuit) in &router.circuits {
        println!("  Circuit {}: {} hops, {} bytes sent",
            id, circuit.hops.len(), circuit.bytes_sent);
    }

    // 6. Close circuit
    println!("\nClosing circuit...");
    router.close_circuit(circuit_id);
    println!("  Circuits remaining: {}", router.circuits.len());
}
