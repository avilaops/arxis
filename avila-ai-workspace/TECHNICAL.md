# 📚 Documentação Técnica - AvilaDB

## Índice
1. [Fundamentos Matemáticos](#fundamentos-matemáticos)
2. [Curvas Elípticas](#curvas-elípticas)
3. [Assinaturas Digitais](#assinaturas-digitais)
4. [Protocolo QUIC](#protocolo-quic)
5. [Storage Engine](#storage-engine)
6. [Performance Tuning](#performance-tuning)

---

## Fundamentos Matemáticos

### Aritmética Modular

#### **Por que Modular?**
Criptografia opera em grupos finitos. Números cresceriam infinitamente sem módulo.

```rust
// Adição modular: (a + b) mod p
// Mantém resultado dentro de [0, p-1]
fn add_mod(a: U256, b: U256, p: U256) -> U256 {
    let sum = a.wrapping_add(&b);
    if sum >= p { sum - p } else { sum }
}
```

#### **Montgomery Reduction**
Substituir divisão cara por multiplicação.

**Ideia:** Em vez de trabalhar com `x`, trabalha-se com `x̃ = x × R mod N`

```
Operação normal:   (a × b) mod N  → O(n²) divisão
Montgomery:        REDC(ã × b̃)   → O(n) multiplicação
```

**Algoritmo REDC:**
```
Input: T (produto em Montgomery space)
1. m = (T mod R) × N' mod R
2. t = (T + m × N) / R
3. Se t >= N, retorna t - N; senão retorna t
```

### SIMD (Single Instruction Multiple Data)

#### **AVX-512: Processa 8 u64s simultaneamente**
```rust
#[target_feature(enable = "avx512f")]
unsafe fn add_avx512(a: &[u64; 8], b: &[u64; 8]) -> [u64; 8] {
    let va = _mm512_loadu_epi64(a.as_ptr());
    let vb = _mm512_loadu_epi64(b.as_ptr());
    let sum = _mm512_add_epi64(va, vb);
    
    let mut result = [0u64; 8];
    _mm512_storeu_epi64(result.as_mut_ptr(), sum);
    result
}
```

**Speedup:** 8x em adição, ~4-6x em operações complexas (overhead de setup)

---

## Curvas Elípticas

### secp256k1 (Bitcoin)

#### **Equação:** `y² = x³ + 7 (mod p)`

**Parâmetros:**
```
p = 2²⁵⁶ - 2³² - 977  (field prime)
n = 0xFFFF...BD141     (order - número de pontos)
G = (Gx, Gy)           (generator point)
```

#### **Adição de Pontos**
```
P + Q = R onde:
λ = (Q.y - P.y) / (Q.x - P.x)  mod p
R.x = λ² - P.x - Q.x           mod p
R.y = λ(P.x - R.x) - P.y       mod p
```

#### **Dobramento de Ponto (2P)**
```
λ = (3P.x²) / (2P.y)  mod p  (a=0 em secp256k1 simplifica)
R.x = λ² - 2P.x       mod p
R.y = λ(P.x - R.x) - P.y  mod p
```

#### **Multiplicação Escalar (k × P)**
**Algoritmo:** Double-and-Add
```
resultado = ∞ (ponto no infinito)
base = P
para cada bit de k (LSB para MSB):
    se bit == 1:
        resultado = resultado + base
    base = 2 × base  (dobramento)
```

**Otimização GLV (Gallant-Lambert-Vanstone):**
secp256k1 tem endomorphism eficiente:
- Para P = (x, y), existe β tal que (βx, y) também está na curva
- Decompõe k = k1 + k2×λ onde |k1|, |k2| ≤ √n
- Calcula k1×P + k2×(β×P) em paralelo
- **Resultado: 2x mais rápido!**

### Curve25519 (Ed25519)

#### **Equação Montgomery:** `By² = x³ + Ax² + x`
#### **Equação Edwards:** `-x² + y² = 1 + dx²y²`

**Vantagem:** Prime modulus `p = 2²⁵⁵ - 19` permite aritmética ultra-rápida

#### **Montgomery Ladder (Constant-Time)**
```rust
fn scalar_mul(k: &[u8; 32], u: &U256) -> U256 {
    let mut x2 = U256::ONE;
    let mut z2 = U256::ZERO;
    let mut x3 = *u;
    let mut z3 = U256::ONE;
    
    // Itera bits de k (MSB para LSB)
    for bit in (0..255).rev() {
        let swap = (k[bit/8] >> (bit%8)) & 1;
        
        // Conditional swap (sem branches!)
        cswap(swap, &mut x2, &mut x3);
        cswap(swap, &mut z2, &mut z3);
        
        // Differential addition (fórmulas específicas)
        // ...
    }
    
    x2 * z2.mod_inverse(&P) % P
}
```

**Por que constant-time?** Evita timing attacks - tempo de execução não depende de bits secretos.

---

## Assinaturas Digitais

### Schnorr (Taproot Bitcoin)

#### **Por que Schnorr > ECDSA?**
1. **Linearidade:** Permite agregação de assinaturas
2. **Prova de segurança:** Mais simples matematicamente
3. **Determinístico:** Não precisa de RNG durante sign
4. **Menor tamanho:** Mais eficiente em batch

#### **Algoritmo de Assinatura**
```
Input: privkey (d), message (m)

1. k = H(d || m)           # nonce determinístico
2. R = k × G               # ponto na curva
3. r = R.x                 # coordenada x
4. e = H(r || P || m)      # challenge
5. s = k + e×d  mod n      # resposta

Output: (r, s)
```

#### **Verificação**
```
Input: pubkey (P), message (m), signature (r, s)

1. e = H(r || P || m)
2. R' = s×G - e×P          # ponto calculado
3. Verifica: R'.x == r
```

**Por que funciona?**
```
R' = s×G - e×P
   = (k + e×d)×G - e×(d×G)
   = k×G + e×d×G - e×d×G
   = k×G
   = R
```

#### **Agregação MuSig2**
```
Signers: Alice, Bob, Carol com keys (A, B, C)

1. Cada um gera nonce: (R₁, R₂, R₃)
2. R = R₁ + R₂ + R₃
3. P = A + B + C  (chave agregada)
4. e = H(R || P || m)
5. Cada um calcula: sᵢ = kᵢ + e×dᵢ
6. s = s₁ + s₂ + s₃

Resultado: Uma assinatura (R, s) para P
```

### ECDSA (Bitcoin Legacy)

#### **Algoritmo**
```
1. k = random nonce  ⚠️ DEVE ser único!
2. R = k × G
3. r = R.x mod n
4. s = k⁻¹ × (z + r×d) mod n

Output: (r, s)
```

**⚠️ CRITICAL:** Se `k` for reutilizado, chave privada vaza!
```
s₁ = k⁻¹(z₁ + r×d)
s₂ = k⁻¹(z₂ + r×d)

s₁ - s₂ = k⁻¹(z₁ - z₂)
k = (z₁ - z₂) / (s₁ - s₂)

d = (s×k - z) / r  ← CHAVE PRIVADA VAZOU!
```

**Caso real:** PlayStation 3 usou `k` constante → hack completo

---

## Protocolo QUIC

### Packet Format

#### **Long Header (Handshake)**
```
+--------+--------+--------+--------+
| Flags  | Version (4 bytes)        |
+--------+--------------------------+
| DCID Len | Dest Connection ID     |
+----------+------------------------+
| SCID Len | Source Connection ID   |
+----------+------------------------+
| Packet Number (1-4 bytes)         |
+-----------------------------------+
| Payload (frames)                  |
+-----------------------------------+
```

#### **Short Header (Application Data)**
```
+--------+--------+
| Flags  | DCID   |
+--------+--------+
| Packet Number  |
+----------------+
| Payload        |
+----------------+
```

### Congestion Control (Cubic)

#### **Slow Start**
```
cwnd += bytes_acked  # exponencial
se cwnd >= ssthresh:
    entra em congestion avoidance
```

#### **Congestion Avoidance (Cubic)**
```
W(t) = C(t - K)³ + W_max

onde:
- C = 0.4 (constante)
- K = ∛(W_max × β / C)
- β = 0.7 (fator de redução)
- t = tempo desde última perda
```

**Gráfico:**
```
cwnd
  ^
  |        ___----
  |    __--
  | __-
  |/_______________> tempo
    perda    K
```

### Loss Detection

#### **Threshold Detection**
```
se largest_acked_pn - sent_pn >= 3:
    marcar como perdido
```

#### **Time Threshold**
```
loss_delay = max(1ms, smoothed_rtt × 9/8)

se now - time_sent > loss_delay:
    marcar como perdido
```

#### **PTO (Probe Timeout)**
```
pto = smoothed_rtt 
    + max(4 × rtt_var, 1ms)
    + max_ack_delay

# Backoff exponencial
pto × 2^(min(pto_count, 5))
```

---

## Storage Engine

### LSM Tree (Log-Structured Merge Tree)

#### **Write Path**
```
1. Write → MemTable (in-memory sorted map)
2. Se MemTable > 4MB:
   a. Freeze MemTable (torna imutável)
   b. Cria novo MemTable
   c. Flush frozen MemTable → SSTable (disco)
3. Background compaction merge SSTables
```

#### **Read Path**
```
1. Busca MemTable ativa
2. Busca MemTables imutáveis
3. Busca SSTables (do mais recente ao mais antigo)
4. Retorna primeiro valor encontrado
```

#### **Compaction (Leveled)**
```
Level 0: 4 SSTables      (overlapping ranges)
Level 1: 40 SSTables     (10x, non-overlapping)
Level 2: 400 SSTables    (10x)
...

Quando Level N cheio:
1. Seleciona SSTables de Level N
2. Identifica overlapping em Level N+1
3. Merge → novas SSTables em Level N+1
4. Deleta SSTables antigas
```

**Trade-offs:**
- **Writes:** O(1) - apenas append to MemTable
- **Reads:** O(log N) - busca em múltiplos níveis
- **Space:** 10-30% overhead (compaction lag)

### MVCC (Multi-Version Concurrency Control)

#### **Snapshot Isolation**
```
Transaction T1 (snapshot_id = 100):
  READ key1 → vê versão <= 100
  WRITE key2 → cria versão 100

Transaction T2 (snapshot_id = 101):
  READ key1 → vê versão <= 101
  READ key2 → vê versão 100 de T1
```

#### **Versioning**
```
Key: "user:1"
Versions:
  [200] → "Alice (updated)"
  [150] → DELETED
  [100] → "Alice"
  [50]  → "Bob"

T(snapshot=175) lê → "Alice" (versão 100)
T(snapshot=225) lê → "Alice (updated)"
```

#### **Garbage Collection**
```
Se todas transações ativas têm snapshot_id > 150:
  → Pode deletar versões <= 150
  → Libera espaço
```

---

## Performance Tuning

### CPU-Level Optimizations

#### **1. Cache Line Alignment**
```rust
#[repr(align(64))]  // alinha em 64 bytes (cache line)
pub struct U2048 {
    limbs: [u64; 32],  // exatamente 256 bytes = 4 cache lines
}
```

**Por que?** Evita false sharing e cache misses.

#### **2. Loop Unrolling**
```rust
// Compilador desenrola automaticamente quando tamanho é conhecido
for i in 0..32 {  // 32 é constante compile-time
    result.limbs[i] = self.limbs[i] + rhs.limbs[i];
}

// Vira:
result.limbs[0] = self.limbs[0] + rhs.limbs[0];
result.limbs[1] = self.limbs[1] + rhs.limbs[1];
// ... 32 instruções diretas
```

#### **3. Branch Prediction**
```rust
// ❌ Ruim (branch imprevisível)
if condition { x } else { y }

// ✅ Bom (constant-time)
let mask = (condition as u64).wrapping_neg();
(x & mask) | (y & !mask)
```

### Memory Optimization

#### **Stack vs Heap**
```
Stack:
✅ Rápido (L1 cache)
✅ Sem overhead de allocação
✅ Cache-friendly
❌ Tamanho limitado (~8MB)

Heap:
❌ Lento (RAM)
❌ Overhead de allocação (~48 bytes)
❌ Fragmentação
✅ Tamanho ilimitado
```

**AvilaDB:** Stack para crypto ops, heap apenas para buffers de I/O

### Networking

#### **QUIC vs TCP**
```
TCP + TLS:
  SYN          → 0.5 RTT
  SYN-ACK      → 1.0 RTT
  ACK          → 1.5 RTT
  ClientHello  → 2.0 RTT
  ServerHello  → 2.5 RTT
  Data         → 3.0 RTT

QUIC:
  Initial (ClientHello) → 0.5 RTT
  Handshake            → 1.0 RTT
  Data                 → 1.0 RTT
  
Reconnect (0-RTT):
  Data → 0.5 RTT  ⚡
```

### Benchmarking

#### **Cycle Counting**
```rust
#[cfg(target_arch = "x86_64")]
fn rdtsc() -> u64 {
    unsafe {
        let lo: u32;
        let hi: u32;
        asm!(
            "rdtsc",
            out("eax") lo,
            out("edx") hi,
        );
        ((hi as u64) << 32) | (lo as u64)
    }
}

let start = rdtsc();
// operação
let end = rdtsc();
println!("Cycles: {}", end - start);
```

#### **Criterion Benchmarks**
```rust
use criterion::{black_box, Criterion};

fn bench_u256_add(c: &mut Criterion) {
    let a = U256::from_u64(12345);
    let b = U256::from_u64(67890);
    
    c.bench_function("u256_add", |bencher| {
        bencher.iter(|| {
            black_box(a + b)
        });
    });
}
```

---

## Segurança

### Constant-Time Programming

#### **Por que?**
```c
// ❌ Vulnerável a timing attack
if (password == correct_password) {
    return true;
}

// Comparação para no primeiro byte diferente
// Tempo de execução revela informação!
```

#### **Como fazer?**
```rust
// ✅ Constant-time comparison
fn secure_compare(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    
    let mut diff = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    
    diff == 0  // tempo constante
}
```

### Side-Channel Resistance

#### **Cache-Timing Attacks**
```rust
// ❌ Acesso dependente de secret
let value = table[secret_index];  // vaza índice via cache

// ✅ Acessa todos os índices
let mut value = 0;
for i in 0..table.len() {
    let mask = ((i == secret_index) as u8).wrapping_neg();
    value |= table[i] & mask;
}
```

### Memory Zeroization

```rust
impl Drop for PrivateKey {
    fn drop(&mut self) {
        // Garante que chave privada é zerada
        for byte in &mut self.bytes {
            unsafe {
                core::ptr::write_volatile(byte, 0);
            }
        }
    }
}
```

---

**Esta documentação é viva. Atualize conforme implementação evolui!**

```
Built with 🇧🇷 by Ávila Engineering
Matemática > Política
```
