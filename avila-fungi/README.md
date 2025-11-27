# 🍄 avila-fungi - Sistema Distribuído como Fungo

**"Espalhando-se entre átomos como micélio"**

## Conceito Biológico

Fungos são organismos fascinantes que se espalham através de **micélios** - vastas redes subterrâneas de filamentos que conectam múltiplos pontos. O maior organismo vivo da Terra é um fungo (Armillaria ostoyae) com 9.6 km² de área!

### Por que Fungos para Sistemas Distribuídos?

- **🕸️ Rede Descentralizada**: Não há "centro" - cada nó é igualmente importante
- **🔄 Auto-Reparação**: Se uma parte morre, o resto continua funcionando
- **📡 Comunicação Química**: Propagam informação via "esporos" (pacotes de dados)
- **🌱 Crescimento Orgânico**: Expandem naturalmente conforme necessário
- **🤝 Simbiose**: Podem formar conexões mutualmente benéficas

## Arquitetura

```
Hierarquia Biológica ÁVILA Platform:

⚛️  Nucleus  → Operações fundamentais (bits, bytes)
🔬 Atom     → Estruturas de dados (Option, Result, Vec)
🧪 Molecule → Protocolos de rede (TCP, UDP, TLS)
🦠 Cell     → Protocolos de aplicação (SMTP, IMAP)
🧵 Tissue   → Organização e armazenamento
🫀 Organ    → Sistemas completos (Email Server)
🧬 Organism → Plataformas (Email Platform)
🍄 Fungi    → DISTRIBUIÇÃO ENTRE ÁTOMOS! ← VOCÊ ESTÁ AQUI
```

### Componentes

#### 1. **Mycelium** (Micélio)
Rede P2P que conecta múltiplos nós (átomos):
```rust
let mut mycelium = Mycelium::new("Node_A", "0.0.0.0:7000").await?;
mycelium.start().await?;
mycelium.connect_to_peer("192.168.1.100:7000").await?;
```

#### 2. **Hypha** (Hifa)
Conexão individual entre dois nós:
```rust
let hypha = Hypha::new(tcp_connection, "peer_addr");
hypha.send(b"data").await?;
```

#### 3. **Spore** (Esporo)
Pacote de dados replicável que se propaga pela rede:
```rust
let spore = SporeData::new("email_received", email_bytes, 10 /* TTL */);
mycelium.release_spore("email", payload).await?;
```

#### 4. **Gossip Protocol**
Protocolo de disseminação de informação estilo "fofoca":
```rust
let gossip = GossipEngine::new(GossipConfig::default());
gossip.start().await?;
```

## Casos de Uso

### 📧 Email Distribuído
Múltiplos servidores de email replicando mensagens automaticamente:
```rust
// Server SP recebe email
server_sp.release_spore("email_received", email_data).await?;

// Propaga automaticamente:
// SP → RJ → MG → ... (toda a rede)
```

### 💾 Storage Distribuído
Armazenamento resiliente sem ponto único de falha:
```rust
mycelium.release_spore("file_chunk", chunk_data).await?;
// Replicado em múltiplos nós automaticamente
```

### 🔍 Search Distribuído
Índice de busca distribuído entre nós:
```rust
mycelium.release_spore("index_update", new_index).await?;
```

## Exemplo Completo

```rust
use avila_fungi::{Mycelium, SporeData};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Criar 3 nós
    let mut node_a = Mycelium::new("A", "127.0.0.1:7001").await?;
    let mut node_b = Mycelium::new("B", "127.0.0.1:7002").await?;
    let mut node_c = Mycelium::new("C", "127.0.0.1:7003").await?;

    node_a.start().await?;
    node_b.start().await?;
    node_c.start().await?;

    // Conectar em malha
    node_b.connect_to_peer("127.0.0.1:7001").await?;
    node_c.connect_to_peer("127.0.0.1:7002").await?;
    node_a.connect_to_peer("127.0.0.1:7003").await?;

    println!("🍄 Rede micélica formada!");
    println!("   Nós: A ({}), B ({}), C ({})",
        node_a.peer_count().await,
        node_b.peer_count().await,
        node_c.peer_count().await
    );

    // Liberar esporo do nó A
    let data = b"Hello from the mycelium!".to_vec();
    node_a.release_spore("greeting", data).await?;

    println!("🌊 Esporo propagado pela rede!");

    Ok(())
}
```

## Vantagens

✅ **Sem ponto único de falha** - Rede continua se um nó cai
✅ **Auto-organização** - Nós descobrem uns aos outros
✅ **Replicação automática** - Dados se espalham naturalmente
✅ **Escalabilidade** - Adicione nós conforme necessário
✅ **Baixa latência** - Dados próximos geograficamente
✅ **Resiliente a partições** - Rede se reconecta automaticamente

## Roadmap

- [x] Estrutura básica (Mycelium, Hypha, Spore)
- [x] Protocolo Gossip
- [ ] Descoberta automática de peers (mDNS/Rendezvous)
- [ ] Roteamento DHT (Distributed Hash Table)
- [ ] Consensus (Raft/Paxos) para dados críticos
- [ ] Sincronização de relógio (NTP/PTP)
- [ ] Criptografia end-to-end entre hifas
- [ ] Métricas e observabilidade distribuída
- [ ] Load balancing automático
- [ ] Geo-replicação inteligente

## Filosofia

> "Assim como fungos transformam a floresta em uma rede viva e interconectada,
> **avila-fungi** transforma seus átomos independentes em um organismo
> distribuído resiliente."

🍄 **Do átomo ao fungo. Da máquina à floresta digital.**

---

**Parte da Ávila Platform** - Construindo sistemas biológicos do zero
Licença: MIT ou Apache-2.0
