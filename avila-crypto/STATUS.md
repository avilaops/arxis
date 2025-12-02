# avila-crypto v0.1.0

**Criptografia soberana com zero dependências externas**

##  Status: 15/15 testes passando

###  Implementações Completas

#### Hash Functions
- **BLAKE3** - Merkle tree, 7 rounds, 4x mais rápido que SHA-256
- **SHA-256** - Bitcoin standard, 64 rounds
- **Keccak-256** - Ethereum/SHA-3, sponge construction, 24 rounds

#### Message Authentication
- **Poly1305** - 130-bit prime field MAC por Daniel J. Bernstein

#### Encryption
- **ChaCha20** - Stream cipher, 20 rounds
- **ChaCha20-Poly1305** - AEAD (RFC 8439) com autenticação

#### Signatures
- **ECDSA** - secp256k1 (Bitcoin/Ethereum)
- **Schnorr** - Stub (requer refactor secp256k1)

#### BigInt
- **U256** - Aritmética modular 256-bit
- **U512** - Operações 512-bit

#### Curves
- **secp256k1** - y² = x³ + 7, curva do Bitcoin

---

##  Métricas

- **Arquivos:** 25 módulos Rust
- **Linhas:** ~2000 LOC
- **Testes:** 15 testes unitários
- **Dependencies:** 0 (zero absoluto)
- **unsafe:** Proibido (#![forbid(unsafe_code)])
- **no_std:** Compatível

##  Testes

\\\ash
cargo test --lib
\\\

**Resultado:**  15 passed, 0 failed

### Cobertura por Módulo

| Módulo | Testes | Status |
|--------|--------|--------|
| bigint::u256 | 2 |  |
| curves::secp256k1 | 2 |  |
| encryption::chacha20 | 1 |  |
| encryption::chacha20_poly1305 | 2 |  |
| hash::blake3 | 2 |  |
| hash::sha256 | 2 |  |
| hash::sha3 | 2 |  |
| mac::poly1305 | 2 |  |

##  Arquitetura

\\\
src/
 bigint/         # U256, U512 aritmética modular
 curves/         # secp256k1, curve25519 (stub)
 encryption/     # ChaCha20, ChaCha20-Poly1305, AES-GCM (stub)
 hash/           # BLAKE3, SHA-256, Keccak-256
 mac/            # Poly1305
 signatures/     # ECDSA, Schnorr (stub), EdDSA (stub)
\\\

##  Princípios

1. **Stack-only allocation** - Zero heap
2. **Fixed-size types** - Compile-time known
3. **Constant-time ops** - Side-channel resistant
4. **Zero dependencies** - Tudo from scratch
5. **Mathematics > Politics** - Transparência matemática

##  Próximos Passos (Opcional)

- [ ] EdDSA/Ed25519 completo (requer Curve25519)
- [ ] Schnorr BIP-340 completo (requer refactor trait)
- [ ] U384/U2048/U4096 para RSA
- [ ] Montgomery multiplication optimization
- [ ] Test vectors RFC compliance

---

**Filosofia:** Soberania criptográfica - código auditável, matemática verificável, zero confiança em padrões governamentais.
