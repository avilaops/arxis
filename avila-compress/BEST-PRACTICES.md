# 📘 avila-compress - Production Best Practices

Este guia contém recomendações para uso de `avila-compress` em ambientes de produção.

---

## 🎯 Escolhendo o Nível de Compressão

### Level::Fast
**Quando usar:**
- ✅ Logs em tempo real (alta frequência)
- ✅ Telemetria de streaming
- ✅ Dados efêmeros (TTL curto)
- ✅ CPU limitada
- ✅ Latência crítica (< 1ms)

**Características:**
- Velocidade: ~2.5 GB/s
- Ratio: ~2.0x
- Uso de CPU: Baixo
- Latência: Mínima

**Exemplo:**
```rust
use avila_compress::{lz4, Level};

// Logs de alta frequência
let log_entry = format!("{}: {}", timestamp, message);
let compressed = lz4::compress_with_level(
    log_entry.as_bytes(),
    Level::Fast
)?;
```

---

### Level::Balanced (Default)
**Quando usar:**
- ✅ Uso geral
- ✅ Dados quentes no AvilaDB
- ✅ Documentos de tamanho médio
- ✅ Balanceamento entre velocidade e tamanho
- ✅ Cache comprimido

**Características:**
- Velocidade: ~1.3 GB/s
- Ratio: ~2.5x
- Uso de CPU: Moderado
- Latência: Aceitável

**Exemplo:**
```rust
// Documentos do AvilaDB
let document = serialize_to_json(&game_state);
let compressed = lz4::compress(document.as_bytes())?; // usa Balanced
```

---

### Level::Best
**Quando usar:**
- ✅ Arquivamento de longo prazo
- ✅ Backups
- ✅ Dados frios (acesso raro)
- ✅ Armazenamento caro
- ✅ Transferências de rede lentas

**Características:**
- Velocidade: ~600 MB/s
- Ratio: ~3.0x
- Uso de CPU: Alto
- Latência: Maior

**Exemplo:**
```rust
use avila_compress::{lz4, Level, checksum};

// Backup com compressão máxima
let compressed = lz4::compress_with_level(&backup_data, Level::Best)?;
let hash = checksum::xxhash64(&backup_data, 0);

// Armazenar compressed + hash para verificação
store_backup(&compressed, hash)?;
```

---

## 🌊 Streaming vs. Batch

### Use Streaming quando:
- ✅ Arquivos > 100 MB
- ✅ Dados chegam incrementalmente
- ✅ Memória limitada
- ✅ Upload/download de rede
- ✅ Processamento contínuo

**Exemplo:**
```rust
use avila_compress::stream::Lz4Encoder;
use std::fs::File;
use std::io::Read;

let mut encoder = Lz4Encoder::new();
let mut file = File::open("large_file.bin")?;
let mut buffer = vec![0u8; 64 * 1024]; // 64 KB chunks

loop {
    let bytes_read = file.read(&mut buffer)?;
    if bytes_read == 0 {
        break;
    }
    encoder.write(&buffer[..bytes_read])?;
}

let compressed = encoder.finish()?;
```

### Use Batch quando:
- ✅ Dados completos na memória
- ✅ Arquivos pequenos (< 10 MB)
- ✅ Processamento único
- ✅ APIs simples preferidas

**Exemplo:**
```rust
let data = std::fs::read("small_file.txt")?;
let compressed = lz4::compress(&data)?;
```

---

## 🚀 Compressão Paralela

### Quando usar parallel:
- ✅ Arquivos > 1 MB
- ✅ CPU multi-core disponível
- ✅ Processamento batch
- ✅ Throughput crítico
- ✅ Não há restrição de memória

**Nota:** Requer feature `parallel`

**Exemplo:**
```rust
use avila_compress::parallel;

// Determinar número de threads
let num_threads = num_cpus::get(); // ou usar 0 para auto

// Comprimir em paralelo
let compressed = parallel::compress_parallel(&data, num_threads)?;

// Descomprimir em paralelo
let decompressed = parallel::decompress_parallel(&compressed, num_threads)?;
```

**Trade-offs:**
- ✅ **Pro**: 8-10x speedup com 8 cores
- ✅ **Pro**: Ideal para processamento batch
- ❌ **Con**: Overhead de threads para arquivos pequenos
- ❌ **Con**: Maior uso de memória

**Recomendação de tamanho:**
| Tamanho       | Recomendação           |
| ------------- | ---------------------- |
| < 100 KB      | Batch simples          |
| 100 KB - 1 MB | Streaming ou batch     |
| 1 MB - 100 MB | Parallel (4-8 threads) |
| > 100 MB      | Parallel + streaming   |

---

## ✓ Checksums e Integridade

### XXHash64 (Recomendado)
**Quando usar:**
- ✅ Performance crítica
- ✅ Detecção de corrupção
- ✅ Hash tables
- ✅ Cache keys

**Características:**
- Velocidade: ~20 GB/s
- Tamanho: 64 bits
- Colisão: Extremamente rara
- Uso: Não-criptográfico

**Exemplo:**
```rust
use avila_compress::checksum;

// Calcular hash
let data = b"important data";
let hash = checksum::xxhash64(data, 0);

// Armazenar junto com dados comprimidos
let compressed = lz4::compress(data)?;
store_with_checksum(&compressed, hash)?;

// Verificar após recuperação
let retrieved = retrieve_data()?;
if !checksum::verify_xxhash64(&retrieved, hash) {
    return Err("Data corruption detected!");
}
```

---

### CRC32 (Compatibilidade)
**Quando usar:**
- ✅ Compatibilidade com sistemas legados
- ✅ Protocolos que exigem CRC32
- ✅ Verificação de integridade padrão

**Características:**
- Velocidade: ~5 GB/s
- Tamanho: 32 bits
- Uso: Detecção de erros

**Exemplo:**
```rust
let crc = checksum::crc32(data);
// Compatível com ZIP, PNG, Ethernet, etc.
```

---

## 🎯 Patterns para AvilaDB

### Pattern 1: Transparent Compression
```rust
use avila_compress::{lz4, checksum, Level};

struct CompressedDocument {
    data: Vec<u8>,
    checksum: u64,
    compression: String,
}

impl CompressedDocument {
    fn new(data: &[u8], level: Level) -> Self {
        let compressed = lz4::compress_with_level(data, level).unwrap();
        let checksum = checksum::xxhash64(data, 0);

        Self {
            data: compressed,
            checksum,
            compression: "lz4".to_string(),
        }
    }

    fn decompress(&self) -> Result<Vec<u8>, String> {
        let decompressed = lz4::decompress(&self.data)
            .map_err(|e| format!("Decompression failed: {}", e))?;

        if !checksum::verify_xxhash64(&decompressed, self.checksum) {
            return Err("Checksum verification failed".to_string());
        }

        Ok(decompressed)
    }
}

// Uso com AvilaDB
let doc = CompressedDocument::new(json_data.as_bytes(), Level::Balanced);
aviladb.insert("users", &doc)?;
```

---

### Pattern 2: Columnar Compression
```rust
// Comprimir colunas separadamente para melhor ratio
fn compress_columnar(
    column_a: &[f64],
    column_b: &[i32],
    column_c: &[String],
) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    // Serializar colunas
    let bytes_a: Vec<u8> = column_a
        .iter()
        .flat_map(|v| v.to_le_bytes())
        .collect();

    let bytes_b: Vec<u8> = column_b
        .iter()
        .flat_map(|v| v.to_le_bytes())
        .collect();

    let bytes_c = serde_json::to_vec(column_c).unwrap();

    // Comprimir separadamente
    let compressed_a = lz4::compress(&bytes_a).unwrap();
    let compressed_b = lz4::compress(&bytes_b).unwrap();
    let compressed_c = lz4::compress(&bytes_c).unwrap();

    (compressed_a, compressed_b, compressed_c)
}
```

---

### Pattern 3: Adaptive Compression
```rust
fn adaptive_compress(data: &[u8]) -> Vec<u8> {
    // Escolher nível baseado no tamanho
    let level = match data.len() {
        0..=1024 => Level::Fast,           // < 1 KB: Fast
        1025..=102400 => Level::Balanced,  // 1-100 KB: Balanced
        _ => Level::Best,                  // > 100 KB: Best
    };

    lz4::compress_with_level(data, level).unwrap()
}
```

---

## 🔥 Performance Tips

### 1. Reuse Buffers
```rust
// ❌ Ruim: aloca toda vez
for chunk in chunks {
    let compressed = lz4::compress(chunk)?;
    process(compressed);
}

// ✅ Bom: reusa buffer quando possível
let mut encoder = Lz4Encoder::new();
for chunk in chunks {
    encoder.write(chunk)?;
}
let compressed = encoder.finish()?;
```

---

### 2. Escolha o tamanho de chunk correto
```rust
// Para streaming
const CHUNK_SIZE: usize = 64 * 1024; // 64 KB (recomendado)

// Muito pequeno: overhead de função
// const CHUNK_SIZE: usize = 1024; // ❌ 1 KB

// Muito grande: picos de memória
// const CHUNK_SIZE: usize = 10 * 1024 * 1024; // ❌ 10 MB
```

---

### 3. Paralelização inteligente
```rust
#[cfg(feature = "parallel")]
fn compress_smart(data: &[u8]) -> Vec<u8> {
    // Usar parallel apenas se valer a pena
    if data.len() > 1024 * 1024 { // > 1 MB
        parallel::compress_parallel(data, 0).unwrap()
    } else {
        lz4::compress(data).unwrap()
    }
}
```

---

### 4. Evite compressão desnecessária
```rust
// ✅ Verificar se vale a pena comprimir
fn should_compress(data: &[u8]) -> bool {
    // Não comprimir dados muito pequenos
    if data.len() < 128 {
        return false;
    }

    // Não comprimir dados já comprimidos (imagens, vídeos)
    let magic = &data[..4];
    if magic == b"\x89PNG" || magic == b"\xFF\xD8\xFF" {
        return false;
    }

    true
}

if should_compress(&data) {
    let compressed = lz4::compress(&data)?;
    // Usar compressed
} else {
    // Usar data original
}
```

---

## ⚠️ Error Handling

### Pattern: Graceful Degradation
```rust
fn compress_safe(data: &[u8]) -> Vec<u8> {
    match lz4::compress(data) {
        Ok(compressed) => {
            // Verificar se realmente comprimiu
            if compressed.len() < data.len() {
                compressed
            } else {
                // Não comprimiu bem, retornar original
                data.to_vec()
            }
        }
        Err(e) => {
            eprintln!("Compression failed: {}, using original", e);
            data.to_vec()
        }
    }
}
```

---

### Pattern: Retry Logic
```rust
fn decompress_with_retry(data: &[u8], max_retries: u32) -> Result<Vec<u8>, String> {
    for attempt in 0..max_retries {
        match lz4::decompress(data) {
            Ok(decompressed) => return Ok(decompressed),
            Err(e) => {
                if attempt == max_retries - 1 {
                    return Err(format!("Failed after {} attempts: {}", max_retries, e));
                }
                eprintln!("Attempt {} failed: {}, retrying...", attempt + 1, e);
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        }
    }
    unreachable!()
}
```

---

## 📊 Monitoring & Metrics

### Metrics para coletar:
```rust
struct CompressionMetrics {
    original_size: usize,
    compressed_size: usize,
    compression_time: Duration,
    decompression_time: Duration,
    level: Level,
}

impl CompressionMetrics {
    fn ratio(&self) -> f64 {
        (self.compressed_size as f64 / self.original_size as f64) * 100.0
    }

    fn compression_throughput_mbps(&self) -> f64 {
        (self.original_size as f64 / self.compression_time.as_secs_f64())
            / (1024.0 * 1024.0)
    }
}
```

---

## 🎓 Summary

| Cenário             | Recomendação             |
| ------------------- | ------------------------ |
| Logs tempo real     | `Level::Fast`            |
| Documentos AvilaDB  | `Level::Balanced`        |
| Backups             | `Level::Best` + checksum |
| Arquivos grandes    | Streaming API            |
| Batch processing    | Parallel compression     |
| Integridade crítica | XXHash64 checksum        |
| Compatibilidade     | CRC32                    |

---

**Dúvidas?** Consulte os exemplos em `/examples/` ou abra uma issue no GitHub!
