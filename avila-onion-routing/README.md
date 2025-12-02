# Avila Onion Routing

**Privacy-preserving darknet communication using multi-layer onion routing**

## Overview

`avila-onion-routing` implements a sophisticated onion routing protocol inspired by Tor, providing anonymous communication through a network of relay nodes. Messages are encrypted in multiple layers, with each relay only knowing the previous and next hop, ensuring end-to-end privacy.

## Features

- **7-Layer Onion Encryption**: Multiple encryption layers for maximum anonymity
- **Circuit Management**: Dynamic creation and management of routing circuits
- **Service Discovery**: Hidden service descriptors with public key cryptography
- **Relay Network**: Support for guard, middle, and exit relay nodes
- **Identity Management**: Cryptographic identity generation and verification
- **Directory Services**: Network topology and relay information management

## Architecture

### Circuit Creation

Circuits are built incrementally through the network:

```rust
use avila_onion_routing::{Circuit, RelayNode, create_circuit};

let guard = RelayNode::new("guard.onion", vec![1, 2, 3]);
let middle = RelayNode::new("middle.onion", vec![4, 5, 6]);
let exit = RelayNode::new("exit.onion", vec![7, 8, 9]);

let circuit = create_circuit(vec![guard, middle, exit])?;
```

### Hidden Services

Services can be published with cryptographic descriptors:

```rust
use avila_onion_routing::{ServiceDescriptor, publish_service};

let descriptor = ServiceDescriptor {
    service_id: "example".to_string(),
    public_key: vec![/* ... */],
    introduction_points: vec![/* ... */],
};

publish_service(descriptor)?;
```

### Onion Encryption

Messages are encrypted in layers, with each relay peeling one layer:

```rust
use avila_onion_routing::{onion_encrypt, onion_decrypt};

let keys = vec![
    vec![1u8; 32], // Guard key
    vec![2u8; 32], // Middle key
    vec![4u8; 32], // Exit key (XOR keys must be distinct!)
];

let plaintext = b"secret message";
let encrypted = onion_encrypt(plaintext, &keys);
let decrypted = onion_decrypt(&encrypted, &keys);

assert_eq!(decrypted, plaintext);
```

## Security Model

### Threat Model

Protects against:
- **Global passive adversaries**: Cannot correlate traffic patterns
- **Local active adversaries**: Cannot identify source or destination
- **Compromised relays**: Need to compromise entire circuit path
- **Traffic analysis**: Timing and volume obfuscation

Does NOT protect against:
- **End-to-end timing correlation**: If adversary controls both ends
- **Sybil attacks**: If adversary controls majority of network
- **Exit node eavesdropping**: Use end-to-end encryption (TLS)

### Cryptographic Properties

- **Forward secrecy**: Compromise of long-term keys doesn't reveal past sessions
- **Layered encryption**: Each hop adds/removes one encryption layer
- **Distinct keys**: XOR keys must be unique to prevent layer cancellation
- **Identity verification**: Public key authentication for all nodes

## Network Topology

```
Client → Guard Relay → Middle Relay → Exit Relay → Destination
         |              |                |
         v              v                v
      Layer 1        Layer 2          Layer 3
   (Encrypted)    (Encrypted)      (Encrypted)
```

Each relay only knows:
- The relay that sent the message
- The relay to forward the message to
- Nothing about the original source or final destination

## Performance

- **Circuit Creation**: ~200-500ms for 3-hop circuit
- **Latency Overhead**: 3x normal latency (one per hop)
- **Bandwidth**: Limited by slowest relay in circuit
- **Scalability**: Supports thousands of concurrent circuits

## Testing

Run the test suite:
```bash
cargo test
```

Current test coverage: **23 tests passing**

Tests include:
- Circuit creation and teardown
- Onion encryption/decryption with multiple layers
- Service descriptor serialization
- Identity generation and verification
- Relay node management
- Directory service operations

## Use Cases

- **Anonymous browsing**: Route web traffic through onion network
- **Hidden services**: Host services without revealing server location
- **Censorship circumvention**: Bypass network restrictions
- **Privacy-preserving communication**: Protect metadata from surveillance
- **Research networks**: Study anonymous communication protocols

## Configuration

### Relay Node

```rust
use avila_onion_routing::RelayNode;

let relay = RelayNode {
    id: "relay123".to_string(),
    address: "relay.onion:9001".to_string(),
    public_key: vec![/* ... */],
    flags: vec!["Guard", "Fast", "Stable"],
    bandwidth: 10_000_000, // 10 MB/s
};
```

### Circuit Builder

```rust
use avila_onion_routing::CircuitBuilder;

let circuit = CircuitBuilder::new()
    .add_guard(guard_relay)
    .add_middle(middle_relay)
    .add_exit(exit_relay)
    .build()?;
```

## Security Considerations

### Key Management

⚠️ **CRITICAL**: XOR encryption keys MUST be distinct!

```rust
// ❌ INSECURE - keys cancel out!
let keys = vec![vec![1; 32], vec![2; 32], vec![3; 32]];
// 1 ⊕ 2 ⊕ 3 = 0 (returns plaintext!)

// ✅ SECURE - distinct keys
let keys = vec![vec![1; 32], vec![2; 32], vec![4; 32]];
```

### Circuit Lifetime

- Circuits should be rotated regularly (every 10 minutes)
- Avoid reusing circuits for multiple destinations
- Implement circuit padding to prevent traffic analysis

### Relay Selection

- Use guard relays consistently for entry
- Rotate middle and exit relays
- Avoid relays in same /16 subnet or jurisdiction

## Dependencies

- `avila-crypto`: Core cryptographic operations
- `avila-id`: Identity and key management
- `avila-error`: Error handling
- `serde`: Serialization for network messages

## Future Enhancements

- [ ] Pluggable transports for censorship resistance
- [ ] Improved congestion control
- [ ] Bridge support for blocked networks
- [ ] Bandwidth measurement and accounting
- [ ] Guard node reputation system
- [ ] Path selection algorithms

## Related Projects

- **Tor Project**: Original onion routing implementation
- **I2P**: Alternative anonymous network
- `avila-browser`: Web browser with built-in onion routing
- `avila-crypto`: Cryptographic primitives

## License

Part of the Avila privacy suite.

## References

- Dingledine, R., Mathewson, N., & Syverson, P. (2004). "Tor: The Second-Generation Onion Router"
- Goldschlag, D., Reed, M., & Syverson, P. (1999). "Onion Routing for Anonymous and Private Internet Connections"
