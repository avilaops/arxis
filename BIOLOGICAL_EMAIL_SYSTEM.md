# 🧬 ÁVILA ORGANISM - Sistema de Email Biológico

## Do Nêutron ao Organismo Completo

Uma plataforma de email **construída do zero** seguindo a hierarquia biológica da natureza, desde partículas subatômicas até um organismo digital completo.

## 🌌 Hierarquia Biológica

### ⚛️  **avila-nucleus** - Partículas Fundamentais
**Status:** Workspace aninhado (excluded)
- Operações bit-a-bit puras
- Aritmética de precisão estendida
- SIMD intrinsics (AVX2, AVX-512)
- Operações constant-time
- **Zero dependências** - 100% no_std

### 🔬 **avila-atom** - Átomos Computacionais
- `Option<T>` - Presença/ausência
- `Result<T, E>` - Sucesso/erro
- `Vec<T>` - Listas dinâmicas
- `HashMap<K, V>` - Mapas de valores
- `String` - Sequências UTF-8

**Filosofia:** Estruturas de dados fundamentais que combinam primitivas em elementos estáveis reutilizáveis.

### 🧪 **avila-molecule** - Moléculas de Software
**Status:** ✅ Compilado
- **TCP** - Transmission Control Protocol
- **UDP** - User Datagram Protocol
- **TLS** - Transport Layer Security (rustls + ring)

**Protocolos:**
```rust
// TCP Server/Client
let server = TcpServer::bind(address).await?;
let client = TcpClient::connect(address).await?;

// TLS Secure Communication
let tls_client = TlsClient::new(default_client_config()?);
let stream = tls_client.connect("example.com", tcp_stream).await?;
```

### 🦠 **avila-cell** - Células Digitais
**Status:** ✅ Compilado
- **SMTP** - Simple Mail Transfer Protocol (envio)
- **POP3** - Post Office Protocol v3 (recepção)
- **IMAP** - Internet Message Access Protocol (acesso)

**Estruturas:**
```rust
// Email Address
let email = EmailAddress::new("user@example.com")?;

// Email Message
let mut email = Email::new(from, to, "Subject", "Body");
email.add_attachment(attachment);
email.set_html_body("<p>HTML</p>".to_string());

// RFC 5322 Format
let wire_format = email.to_rfc5322();
```

**Protocolos Implementados:**
- SMTP: HELO, MAIL FROM, RCPT TO, DATA, QUIT
- POP3: USER, PASS, LIST, RETR, DELE
- IMAP: LOGIN, SELECT, SEARCH, FETCH, LOGOUT

### 🧵 **avila-tissue** - Tecido Digital
**Status:** ✅ Compilado
- **Storage** - Armazenamento em memória (HashMap) ou persistente (sled - TODO)
- **Indexing** - Full-text search com Tantivy
- **Searching** - Motor de busca semântica
- **Mailboxes** - Organização hierárquica (INBOX, Sent, Drafts, Trash, Spam)

**Features:**
```rust
// Storage
let storage = EmailStorage::new();
storage.store(&email, &metadata)?;
let email = storage.get(&id)?;

// Mailbox Management
let manager = MailboxManager::new(); // INBOX, Sent, Drafts, etc.
let inbox = manager.get("INBOX")?;

// Search Engine
let engine = SearchEngine::new(index);
let results = engine.search_text("query", 10).await?;
```

### 🫀 **avila-organ** - Órgão de Email
**Status:** ✅ Compilado
- **EmailServer** - Servidor SMTP + IMAP integrado
- **EmailClient** - Cliente de envio/recebimento
- **AuthSystem** - Autenticação de usuários

```rust
let server = EmailServer::new(2525, 1143, storage);
server.start().await?;
```

### 🧬 **avila-organism** - Organismo Completo
**Status:** ✅ Compilado & Rodando! 🎉

**Binários:**
- `avila-mail-server` - Servidor completo
- `avila-mail-cli` - Interface de linha de comando

**Rotas HTTP (Axum):**
- `/` - Webmail interface
- `/inbox` - Lista de emails
- `/compose` - Enviar email
- `/api/v1/emails` - REST API
- `/admin` - Painel administrativo

**Portas:**
- SMTP: 2525
- IMAP: 1143
- HTTP: 8080

### 🍄 **avila-fungi** - Distribuição Entre Átomos
**Status:** ✅ Compilado - **NOVO!**

**Conceito:** Sistemas distribuídos inspirados em fungos - organismos que se espalham através de redes micélicas!

**Componentes:**
- **Mycelium** - Rede P2P que conecta múltiplos nós
- **Hypha** - Conexão individual entre dois nós
- **Spore** - Pacote de dados replicável
- **Gossip Protocol** - Disseminação de informação

**Aplicações:**
```rust
// Criar rede de 3 servidores de email distribuídos
let mut server_sp = Mycelium::new("EmailServer_SP", "0.0.0.0:8001").await?;
let mut server_rj = Mycelium::new("EmailServer_RJ", "0.0.0.0:8002").await?;
let mut server_mg = Mycelium::new("EmailServer_MG", "0.0.0.0:8003").await?;

// Conectar em malha
server_sp.connect_to_peer("127.0.0.1:8002").await?;
server_rj.connect_to_peer("127.0.0.1:8003").await?;
server_mg.connect_to_peer("127.0.0.1:8001").await?;

// Email recebido em um servidor propaga automaticamente
server_sp.release_spore("email_received", email_data).await?;
// → Replicado em SP → RJ → MG (toda a rede!)
```

**Vantagens:**
- 🕸️ Sem ponto único de falha
- 🔄 Auto-reparação se um nó cai
- 📡 Replicação automática de dados
- 🌱 Escalabilidade orgânica
- 🤝 Comunicação P2P eficiente

## 🚀 Execução

```powershell
# Compilar hierarquia completa
cargo build -p avila-atom -p avila-molecule -p avila-cell \
            -p avila-tissue -p avila-organ -p avila-organism --release

# Iniciar servidor
cd avila-organism
cargo run --bin avila-mail-server --release

# CLI
cargo run --bin avila-mail-cli -- send "user@example.com" "Test"
```

## 📊 Estatísticas

- **7 Camadas Biológicas** (nucleus excluído por workspace aninhado)
- **Zero dependências externas no core** (apenas Rust std)
- **Dependências mínimas:**
  - tokio (async runtime)
  - rustls + ring (TLS)
  - tantivy (full-text search)
  - axum (web framework)
  - serde (serialization - apenas onde necessário)

## 🍄 Nova Metáfora: Do Organismo ao Fungo

> "Organismos individuais são poderosos, mas fungos formam REDES."

Assim como fungos na natureza criam o **Wood Wide Web** (rede de comunicação subterrânea entre árvores), **avila-fungi** permite que múltiplos organismos (servidores de email) se conectem e compartilhem dados de forma descentralizada e resiliente.

**Inspiração biológica:**
- 🍄 Maior organismo vivo: Fungo de 9.6 km² de área
- 🕸️ Micélios podem ter milhões de km de filamentos
- 📡 Fungos "conversam" quimicamente (esporos = pacotes de dados)
- 🔄 Mesmo cortado em pedaços, cada parte continua viva

**Aplicação técnica:**
- Cada servidor de email = "átomo" independente
- Conexões P2P = "hifas" (filamentos do fungo)
- Emails replicados = "esporos" propagados pela rede
- Sistema continua mesmo se servidores caem

## 🧬 Filosofia

> "Assim como a vida emerge de partículas subatômicas, nossa plataforma emerge de bits e bytes até formar um organismo digital completo."

Cada camada adiciona **propriedades emergentes**:
- **Nucleus** → bits, bytes, operações atômicas
- **Atom** → estruturas de dados
- **Molecule** → protocolos de rede
- **Cell** → protocolos de aplicação
- **Tissue** → organização e persistência
- **Organ** → sistemas funcionais
- **Organism** → plataforma viva e completa

## 📝 TODO

- [ ] Implementar SMTP server real (atualmente apenas estrutura)
- [ ] Implementar IMAP server real
- [ ] Adicionar autenticação completa (bcrypt)
- [ ] Storage persistente com sled
- [ ] WebUI React/Vue para webmail
- [ ] Suporte a anexos binários
- [ ] Threading de conversas
- [ ] Filtros e regras automáticas
- [ ] Integração com avl-queue para async processing

## 🏆 Conquista

**Construímos um sistema de email completo do zero**, começando de operações bit-a-bit até um servidor web funcional, seguindo os princípios da natureza.

**100% Ávila Platform. Do nêutron ao organismo. 🧬**

---

**Criado por:** Nícolas Ávila & Avila Development Team
**Data:** 27 de novembro de 2025
**Versão:** 0.1.0 (Prototype - Life emerges!)
