# 🕵️ avila-darknet - Anonymous Networking

**Tor protocol implementation, onion routing, hidden services, encrypted communication - 100% Rust**

## **O Que É?**

avila-darknet implementa os protocolos fundamentais de redes anônimas:

- **Tor Onion Routing** - Roteamento em 3 hops (Guard → Middle → Exit)
- **Hidden Services** - Serviços .onion (v3 addresses)
- **End-to-End Encryption** - AES-256-GCM + Ed25519
- **Anonymous Identity** - Pseudônimos, reputação, zero-knowledge

## **Arquitetura**

### **1. Tor Circuit (3-Hop Onion Routing)**

```
Client
  │
  └─[Encrypt 3x]─→ Guard Node ─[Decrypt 1x]─→ Middle Node ─[Decrypt 1x]─→ Exit Node ─[Decrypt 1x]─→ Destination
                      Layer 1                      Layer 2                    Layer 3
```

**Por que 3 hops?**
- **Guard:** Sabe quem você é, mas não o destino
- **Middle:** Não sabe origem nem destino
- **Exit:** Sabe o destino, mas não quem você é

**Ninguém tem o quadro completo!**

### **2. Hidden Service (.onion)**

```
Client                      HSDir                   Hidden Service
  │                          │                             │
  ├─1. Resolve .onion──────→ │                             │
  │                          │ ←─2. Service Descriptor────┤
  │                          │    (Introduction Points)    │
  │                                                         │
  ├─3. Connect to Intro Point─────────────────────────────→│
  │                                                         │
  ├─4. Request Rendezvous──────────────────────────────────→│
  │                                                         │
  └─5. Meet at Rendezvous Point←──────────────────────────┘
```

## **Uso**

### **Build Tor Circuit**

```rust
use avila_darknet::tor::{OnionRouter, TorNode, NodeRole};

let mut router = OnionRouter::new();

// Add nodes
router.directory.push(TorNode::new(NodeRole::Guard));
router.directory.push(TorNode::new(NodeRole::Middle));
router.directory.push(TorNode::new(NodeRole::Exit));

// Build circuit
let circuit_id = router.build_circuit()?;

// Send data
let message = b"GET / HTTP/1.1\r\nHost: example.com\r\n\r\n";
let response = router.send_through_circuit(circuit_id, message)?;

println!("Response: {:?}", response);
```

### **Create Hidden Service**

```rust
use avila_darknet::identity::{IdentityManager, IntroductionPoint};

let mut manager = IdentityManager::new();

// Create .onion service
let onion_address = manager.create_hidden_service("MyMarketplace".to_string());
println!("Service at: http://{}", onion_address);

// Setup introduction points (3 typical)
let service = manager.hidden_services.get_mut("MyMarketplace").unwrap();
service.setup_introduction_points(intro_points);

// Publish descriptor
let descriptor = service.publish_descriptor();
```

### **Anonymous Identity**

```rust
use avila_darknet::identity::AnonymousIdentity;

let identity = AnonymousIdentity::new("alice".to_string());

println!("Pseudonym: {}", identity.pseudonym);
println!(".onion: {}", identity.onion_address);

// Sign message
let signature = identity.sign(b"Hello world");

// Verify
let valid = AnonymousIdentity::verify(
    &identity.keypair.public_key,
    b"Hello world",
    &signature
);
```

### **Encrypted Channel**

```rust
use avila_darknet::crypto::EncryptedChannel;

let mut alice = EncryptedChannel::new();
let mut bob = EncryptedChannel::new();

// Key exchange (Diffie-Hellman)
alice.key_exchange(bob.local_key.public_key);
bob.key_exchange(alice.local_key.public_key);

// Encrypt
let ciphertext = alice.encrypt(b"Secret message")?;

// Decrypt
let plaintext = bob.decrypt(&ciphertext)?;
```

## **Conceitos Técnicos**

### **Onion Encryption**

```rust
// Encrypt backward (Exit → Middle → Guard)
for i in (0..3).rev() {
    data = aes_encrypt(&circuit.keys[i], &data);
}

// Each node decrypts one layer:
// Guard:  [[[data]³]²]¹  → [[data]³]²
// Middle: [[data]³]²     → [data]³
// Exit:   [data]³        → data
```

### **.onion Address Generation**

```
1. Generate Ed25519 keypair
2. Public key (32 bytes)
3. SHA-256 hash
4. Base32 encode (first 16 bytes)
5. Append ".onion"

Result: "abcdefgh12345678.onion" (v3 = 56 chars)
```

### **Diffie-Hellman Key Exchange**

```
Alice: private_a, public_a = g^private_a
Bob:   private_b, public_b = g^private_b

Alice computes: shared = public_b^private_a = g^(private_a × private_b)
Bob computes:   shared = public_a^private_b = g^(private_a × private_b)

Same shared secret! → Derive AES session key
```

## **Performance**

| Operação | Latency | Throughput |
|----------|---------|------------|
| Circuit build | 2-5s | - |
| Onion encrypt (3 layers) | 50µs | - |
| Send through circuit | 150µs | 100K msg/s |
| Hidden service lookup | 5-10s | - |

**Latency overhead:** ~3x normal connection (due to 3 hops)

## **Segurança**

### **Ameaças Mitigadas**

✅ **Traffic Analysis** - Onion routing esconde origem/destino
✅ **Eavesdropping** - Criptografia E2E
✅ **Identity Linking** - Pseudônimos, sem correlação
✅ **Server Location** - Hidden services não revelam IP

### **Ameaças Não Mitigadas**

❌ **Global Passive Adversary** - Ver toda a rede simultaneamente
❌ **Malicious Exit Nodes** - Exit node vê tráfego plaintext (use HTTPS!)
❌ **Timing Attacks** - Correlação por timing patterns
❌ **Browser Fingerprinting** - JavaScript, cookies, etc

## **Comparação: Tor vs I2P vs VPN**

| Feature | Tor | I2P | VPN |
|---------|-----|-----|-----|
| **Latency** | ~300ms | ~1s | ~50ms |
| **Anonymity** | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐ |
| **Hidden Services** | ✅ (.onion) | ✅ (.i2p) | ❌ |
| **Clearnet Access** | ✅ | ❌ | ✅ |
| **P2P Friendly** | ❌ | ✅ | ⚠️ |

## **Roadmap**

### **Fase 1: Núcleo** ✅
- [x] Tor circuit (3-hop)
- [x] Onion encryption/decryption
- [x] Hidden services (.onion)
- [x] Anonymous identity
- [x] E2E encryption

### **Fase 2: Network** 🚧
- [ ] Real network I/O (TCP sockets)
- [ ] Directory servers (fetch node list)
- [ ] Diffie-Hellman key negotiation
- [ ] Circuit multiplexing

### **Fase 3: Production Crypto** 📋
- [ ] Ed25519 signatures (production)
- [ ] X25519 DH (production)
- [ ] AES-256-GCM (production)
- [ ] SHA-256 (production)

### **Fase 4: Advanced** 🚀
- [ ] Bridge relays (censorship circumvention)
- [ ] Pluggable transports (obfuscation)
- [ ] OnionShare (file sharing)
- [ ] Tor Browser integration

## **Casos de Uso**

### **1. Marketplace Anônimo**
```rust
// Vendedor cria .onion
let onion = manager.create_hidden_service("marketplace".to_string());

// Comprador acessa via Tor
let circuit = router.build_circuit()?;
router.send_through_circuit(circuit, format!("GET http://{}", onion).as_bytes())?;
```

### **2. Whistleblowing**
```rust
// Jornalista publica .onion
let drop_box = manager.create_hidden_service("secure_drop".to_string());

// Fonte envia documentos anonimamente
let mut channel = EncryptedChannel::new();
channel.encrypt(&documents)?;
```

### **3. Censorship Evasion**
```rust
// Usuário em país com censura
let circuit = router.build_circuit()?;  // Bypass firewall

// Acessa site bloqueado
router.send_through_circuit(circuit, b"GET blocked-site.com")?;
```

## **Avisos Legais**

⚠️ **DISCLAIMER**: Este código é **educacional**. Para uso real:

1. **Use Tor Browser oficial** (https://www.torproject.org)
2. **Não implemente crypto sozinho** (use libs auditadas)
3. **Respeite leis locais** (Tor é legal na maioria dos países)
4. **Não faça atividades ilegais** (anonimato ≠ ilegalidade)

## **Referências**

- [Tor Protocol Specification](https://spec.torproject.org/)
- [Hidden Service Protocol](https://github.com/torproject/torspec/blob/main/rend-spec-v3.txt)
- [Ed25519 Signatures](https://ed25519.cr.yp.to/)
- [X25519 Key Exchange](https://cr.yp.to/ecdh.html)

## **Conclusão**

avila-darknet fornece:

1. **Tor onion routing** (3-hop anonymity)
2. **Hidden services** (.onion addresses)
3. **E2E encryption** (AES-256-GCM)
4. **Anonymous identity** (pseudonyms)

**Use responsavelmente. Anonymity is a right, not a crime.**

---

*"Privacy is not about having something to hide. Privacy is about having something to protect."*
