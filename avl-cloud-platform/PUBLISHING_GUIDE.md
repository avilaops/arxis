# 🌩️ Publicação de Dependências - Avila Cloud

## 📋 Índice

- [Status Atual](#status-atual)
- [Pré-requisitos](#pré-requisitos)
- [Guia Rápido](#guia-rápido)
- [Publicação Automatizada](#publicação-automatizada)
- [Publicação Manual](#publicação-manual)
- [Troubleshooting](#troubleshooting)
- [Melhores Práticas](#melhores-práticas)
- [Alterações Técnicas](#alterações-técnicas)
- [Referências](#referências)

---

## 📊 Status Atual

### ✅ Dependências Publicadas no crates.io

| Pacote | Versão | Descrição | Status |
|--------|--------|-----------|--------|
| **avx-telemetry** | v0.1.0 | Observability and distributed tracing | 🟢 Publicado |
| **avx-http** | v0.4.0 | Pure Rust HTTP/1.1 + HTTP/2 implementation | 🟢 Publicado |

### ⏳ Dependências Pendentes de Publicação

| # | Pacote | Versão | Dependências | Status | Prioridade |
|---|--------|--------|--------------|--------|------------|
| 1 | **avila-error-derive** | v0.1.0 | Nenhuma | 🔴 Pendente | Alta |
| 2 | **avila-error** | v0.2.0 | avila-error-derive | 🔴 Pendente | Alta |
| 3 | **avx-gateway** | v0.1.0 | Nenhuma | 🔴 Pendente | Média |
| 4 | **avl-loadbalancer** | v0.1.0 | Nenhuma | 🔴 Pendente | Média |

**Nota:** A ordem de publicação é importante devido às dependências entre pacotes.

---

## 🔧 Pré-requisitos

### 🛠️ Ferramentas Necessárias

1. **Rust Toolchain** (≥ 1.70.0)
   ```powershell
   # Verificar instalação
   rustc --version
   cargo --version

   # Instalar/atualizar se necessário
   # Windows: https://rustup.rs/
   # Linux/Mac: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Conta no crates.io**
   - Criar conta em: https://crates.io/
   - Gerar token de API: https://crates.io/settings/tokens

3. **Configurar Token de Autenticação**
   ```powershell
   # Fazer login no cargo
   cargo login <seu-token-aqui>

   # Token será salvo em: ~/.cargo/credentials
   ```

4. **Git** (recomendado)
   ```powershell
   git --version
   ```

### 📝 Validações Pré-Publicação

Antes de publicar, certifique-se de que cada pacote atende aos seguintes critérios:

- [ ] **Cargo.toml completo** com todos os metadados obrigatórios:
  - `name` - Nome do pacote
  - `version` - Versão semântica (SemVer)
  - `edition` - Edição do Rust (2021)
  - `description` - Descrição concisa (< 160 caracteres)
  - `license` - Licença (ex: "MIT OR Apache-2.0")
  - `repository` - URL do repositório (recomendado)
  - `keywords` - Palavras-chave (máx. 5)
  - `categories` - Categorias do crates.io

- [ ] **Código compila sem erros**
  ```powershell
  cargo build --release
  ```

- [ ] **Testes passam**
  ```powershell
  cargo test
  ```

- [ ] **Documentação está presente**
  ```powershell
  cargo doc --no-deps --open
  ```

- [ ] **Sem dependências locais não publicadas** (ou use `path` com `version`)

- [ ] **README.md presente** na raiz do pacote (recomendado)

- [ ] **LICENSE** arquivo presente (obrigatório)

---

## 🚀 Guia Rápido

### Primeira Vez

1. **Configure o ambiente:**
   ```powershell
   cargo login <seu-token>
   ```

2. **Execute o script automatizado:**
   ```powershell
   cd d:\arxis\avila-cloud
   .\publish-dependencies.ps1
   ```

### Execução Subsequente

```powershell
# Modo normal
.\publish-dependencies.ps1

# Modo dry-run (teste sem publicar)
.\publish-dependencies.ps1 -DryRun

# Personalizado
.\publish-dependencies.ps1 -WaitTime 15 -SkipTests
```

---

## 🤖 Publicação Automatizada

### Script PowerShell Avançado

O script `publish-dependencies.ps1` oferece:

#### ✨ Recursos

- ✅ **Validação Completa**: Verifica Cargo.toml, build, testes
- 🔍 **Detecção de Duplicatas**: Verifica se já está publicado
- 📊 **Relatórios Detalhados**: Logs completos e estatísticas
- ⚡ **Retry Logic**: Aguarda entre publicações
- 🎯 **Modo Dry-Run**: Testa sem publicar
- 📝 **Logging Avançado**: Arquivo de log com timestamp
- 🛡️ **Tratamento de Erros**: Captura e reporta erros específicos

#### 📖 Parâmetros Disponíveis

```powershell
# Modo dry-run (simula publicação)
.\publish-dependencies.ps1 -DryRun

# Pular testes (não recomendado)
.\publish-dependencies.ps1 -SkipTests

# Pular todas as validações (uso avançado)
.\publish-dependencies.ps1 -SkipValidation

# Tempo de espera customizado (segundos)
.\publish-dependencies.ps1 -WaitTime 15

# Arquivo de log customizado
.\publish-dependencies.ps1 -LogFile "meu-log.txt"

# Combinado
.\publish-dependencies.ps1 -DryRun -WaitTime 20 -LogFile "test-run.log"
```

#### 🎯 Exemplos de Uso

**Teste Completo (Recomendado antes da primeira publicação):**
```powershell
.\publish-dependencies.ps1 -DryRun
```

**Publicação Normal:**
```powershell
.\publish-dependencies.ps1
```

**Publicação Rápida (pula testes):**
```powershell
.\publish-dependencies.ps1 -SkipTests -WaitTime 5
```

#### 📊 Interpretando o Output

```
✅ - Sucesso
❌ - Erro
⚠️ - Aviso
ℹ️ - Informação
📦 - Pacote
🚀 - Publicação
⏳ - Aguardando
✓ - Check passou
✗ - Check falhou
➜ - Próxima ação
⭐ - Destaque
```

---

## 🔨 Publicação Manual

### Processo Passo a Passo

⚠️ **IMPORTANTE**: Publique na ordem exata especificada devido às dependências.

#### Etapa 1: Publicar avila-error-derive

```powershell
# Navegar para o diretório
cd d:\arxis\avila-error\avila-error-derive

# Validar o pacote
cargo publish --dry-run

# Publicar
cargo publish --allow-dirty

# Aguardar propagação (30-60 segundos)
Start-Sleep -Seconds 60
```

#### Etapa 2: Publicar avila-error

```powershell
# Navegar para o diretório
cd d:\arxis\avila-error

# Validar o pacote
cargo publish --dry-run

# Publicar
cargo publish --allow-dirty

# Aguardar propagação
Start-Sleep -Seconds 60
```

#### Etapa 3: Publicar avx-gateway

```powershell
cd d:\arxis\avx-gateway
cargo publish --dry-run
cargo publish --allow-dirty
Start-Sleep -Seconds 60
```

#### Etapa 4: Publicar avl-loadbalancer

```powershell
cd d:\arxis\avl-loadbalancer
cargo publish --dry-run
cargo publish --allow-dirty
```

### ✅ Verificar Publicação

Após cada publicação, verifique se foi bem-sucedida:

```powershell
# Buscar o pacote no crates.io
cargo search "^nome-do-pacote$" --limit 1

# Ver detalhes completos
cargo info nome-do-pacote

# Exemplos:
cargo search "^avila-error$" --limit 1
cargo info avila-error
```

---

## 🔧 Troubleshooting

### Problema: Rate Limit (429 Too Many Requests)

**Sintoma:**
```
Error 429 Too Many Requests: You have published too many new crates in a short
period of time. Please try again after [DATA/HORA] or email help@crates.io.
```

**Soluções:**

1. **Aguardar o período especificado:**
   ```powershell
   # Anotar o horário limite e aguardar
   # Exemplo: Mon, 02 Dec 2025 23:57:57 GMT
   ```

2. **Solicitar aumento de limite:**
   ```
   Para: help@crates.io
   Assunto: Request for increased publishing rate limit

   Corpo:
   Hello,

   I'm publishing multiple related crates for the Avila Cloud project.
   These are infrastructure components that need to be published together:
   - avila-error-derive (proc-macro)
   - avila-error (error handling)
   - avx-gateway (API gateway)
   - avl-loadbalancer (load balancer)

   Could you please increase my publishing rate limit temporarily?

   Repository: https://github.com/avilaops/arxis

   Thank you!
   ```

3. **Espaçar publicações:**
   ```powershell
   # Aguardar 10-15 minutos entre publicações
   .\publish-dependencies.ps1 -WaitTime 600
   ```

### Problema: Dependência Não Encontrada

**Sintoma:**
```
error: no matching package named `avila-error-derive` found
```

**Causas e Soluções:**

1. **Dependência não publicada ainda:**
   - Publique as dependências na ordem correta
   - Aguarde 30-60 segundos após publicar para propagação

2. **Versão incorreta no Cargo.toml:**
   ```toml
   # Certifique-se de que a versão corresponde
   [dependencies]
   avila-error-derive = { version = "0.1.0", optional = true }
   ```

3. **Cache do cargo desatualizado:**
   ```powershell
   # Atualizar índice do crates.io
   cargo update

   # Limpar cache (último recurso)
   rm -r -force $env:USERPROFILE\.cargo\registry\cache
   cargo update
   ```

### Problema: Build Falha Durante Publicação

**Sintoma:**
```
error: could not compile `nome-pacote`
```

**Soluções:**

1. **Compilar localmente primeiro:**
   ```powershell
   cargo clean
   cargo build --release
   cargo test
   ```

2. **Verificar features:**
   ```powershell
   # Compilar com todas as features
   cargo build --all-features

   # Compilar sem features padrão
   cargo build --no-default-features
   ```

3. **Verificar dependências de path:**
   ```toml
   # ❌ Incorreto (só path)
   avila-error = { path = "../avila-error" }

   # ✅ Correto (path + version)
   avila-error = { path = "../avila-error", version = "0.2.0" }
   ```

### Problema: Metadados Faltando

**Sintoma:**
```
warning: manifest has no documentation, homepage or repository
```

**Solução:**
```toml
[package]
name = "meu-pacote"
version = "0.1.0"
edition = "2021"
description = "Descrição concisa do pacote (< 160 caracteres)"
license = "MIT OR Apache-2.0"
repository = "https://github.com/usuario/repositorio"
homepage = "https://meusite.com"
documentation = "https://docs.rs/meu-pacote"
keywords = ["palavra1", "palavra2", "palavra3"]
categories = ["development-tools", "web-programming"]
readme = "README.md"
```

### Problema: Arquivo .gitignore Muito Restritivo

**Sintoma:**
```
warning: file `src/important.rs` is not included in the package
```

**Solução:**

Criar/editar `.cargo-ok` ou `Cargo.toml`:

```toml
[package.metadata]
# Incluir arquivos específicos
include = [
    "src/**/*",
    "Cargo.toml",
    "README.md",
    "LICENSE",
]

# Ou excluir arquivos específicos
exclude = [
    "target/",
    ".git/",
    "*.log",
]
```

### Problema: Versão Já Existe

**Sintoma:**
```
error: crate version `0.1.0` is already uploaded
```

**Solução:**

1. **Incrementar versão no Cargo.toml:**
   ```toml
   [package]
   version = "0.1.1"  # ou 0.2.0, 1.0.0, etc.
   ```

2. **Seguir Versionamento Semântico:**
   - **MAJOR** (1.0.0): Mudanças incompatíveis na API
   - **MINOR** (0.1.0): Nova funcionalidade compatível
   - **PATCH** (0.0.1): Correções de bugs compatíveis

---

## 📚 Melhores Práticas

### 🎯 Antes de Publicar

1. **Versione Semanticamente**
   ```
   MAJOR.MINOR.PATCH
   1.0.0 - API estável
   0.x.y - API em desenvolvimento
   ```

2. **Documente Adequadamente**
   ```rust
   //! Documentação do módulo

   /// Documentação da função
   ///
   /// # Examples
   ///
   /// ```
   /// use meu_pacote::minha_funcao;
   /// assert_eq!(minha_funcao(2), 4);
   /// ```
   pub fn minha_funcao(x: i32) -> i32 {
       x * 2
   }
   ```

3. **Teste Exaustivamente**
   ```powershell
   # Testes unitários
   cargo test

   # Testes de documentação
   cargo test --doc

   # Testes de integração
   cargo test --test '*'

   # Com todas as features
   cargo test --all-features
   ```

4. **Valide com Clippy**
   ```powershell
   cargo clippy --all-features -- -D warnings
   ```

5. **Formate o Código**
   ```powershell
   cargo fmt --all
   ```

### 📝 Documentação

1. **README.md Completo:**
   ```markdown
   # Nome do Pacote

   Descrição breve

   ## Instalação

   ```toml
   [dependencies]
   meu-pacote = "0.1.0"
   ```

   ## Uso

   ```rust
   use meu_pacote::exemplo;

   fn main() {
       exemplo();
   }
   ```

   ## Features

   - `feature1`: Descrição
   - `feature2`: Descrição

   ## Licença

   MIT OR Apache-2.0
   ```

2. **CHANGELOG.md:**
   ```markdown
   # Changelog

   ## [0.1.0] - 2025-12-02

   ### Added
   - Funcionalidade X
   - Funcionalidade Y

   ### Changed
   - Melhoria Z

   ### Fixed
   - Bug W
   ```

### 🔒 Segurança

1. **Nunca commite tokens:**
   ```powershell
   # Tokens ficam em:
   $env:USERPROFILE\.cargo\credentials

   # Adicione ao .gitignore:
   echo ".cargo/credentials" >> .gitignore
   ```

2. **Use features para funcionalidades opcionais:**
   ```toml
   [features]
   default = ["funcionalidade-segura"]
   funcionalidade-experimental = []
   ```

3. **Especifique versões mínimas de dependências:**
   ```toml
   [dependencies]
   serde = "1.0.100"  # Específico
   # Evite: serde = "1"  # Muito genérico
   ```

---

## 🔄 Atualizando Cargo.toml do Avila Cloud

Após publicar as dependências, atualize `d:\arxis\avila-cloud\Cargo.toml`:

### Antes:
```toml
[dependencies]
# Core dependencies (commented for now - enable as needed)
# avx-gateway = { path = "../avx-gateway", optional = true }
# avx-telemetry = { path = "../avx-telemetry", optional = true }
# avx-http = { path = "../avx-http", optional = true }
# avl-loadbalancer = { path = "../avl-loadbalancer", optional = true }
# avila-error = { path = "../avila-error", optional = true }
```

### Depois:
```toml
[dependencies]
# Core dependencies from crates.io
avx-gateway = { version = "0.1.0", optional = true }
avx-telemetry = { version = "0.1.0", optional = true }
avx-http = { version = "0.4.0", optional = true }
avl-loadbalancer = { version = "0.1.0", optional = true }
avila-error = { version = "0.2.0", features = ["derive"], optional = true }

# Para desenvolvimento local, use:
# avx-gateway = { path = "../avx-gateway", optional = true }
```

---

## 🔧 Alterações Técnicas

### Substituição de Termos Não Técnicos por Termos Técnicos

As seguintes melhorias técnicas foram implementadas:


| Arquivo | Antes | Depois | Justificativa |
|---------|-------|--------|---------------|
| **src/main.rs** | `"Your Cloud, Your Way"` | `"Enterprise Cloud Infrastructure Platform"` | Descrição técnica mais precisa do produto |
| **src/api.rs** | `// Simplified - use avx-http Router` | `// TODO: Integrate avx-http Router for production-grade HTTP routing` | Especifica o padrão técnico de integração |
| **src/auth.rs** | `// Simplified - use proper JWT` | `// TODO: Implement RFC 7519 compliant JWT token generation with HMAC-SHA256` | Referencia padrão RFC e algoritmo criptográfico |
| **src/auth.rs** | `// Simplified validation` | `// TODO: Implement JWT signature verification with HMAC-SHA256` | Especifica método de validação criptográfica |
| **Cargo.toml** | `"Complete cloud provider platform built in 100% Rust"` | `"Enterprise-grade cloud infrastructure platform with compute, storage, networking, and billing services"` | Descrição mais profissional e detalhada |

### Melhorias de Arquitetura

1. **Modularização Aprimorada:**
   - Separação clara de responsabilidades entre módulos
   - Interfaces bem definidas entre componentes
   - Uso de traits para abstrações

2. **Documentação Técnica:**
   - TODOs técnicos especificam padrões (RFC 7519, HMAC-SHA256)
   - Comentários referenciam tecnologias específicas (avx-http Router)
   - Especificação de requisitos de segurança

3. **Nomenclatura Profissional:**
   - Termos marketing substituídos por terminologia de engenharia
   - Foco em especificações técnicas ao invés de slogans
   - Linguagem orientada a desenvolvedores

---

## 📚 Referências

### Documentação Oficial

- 🦀 [Cargo Book - Publishing](https://doc.rust-lang.org/cargo/reference/publishing.html)
- 📦 [Crates.io Policies](https://crates.io/policies)
- 📖 [Manifest Format](https://doc.rust-lang.org/cargo/reference/manifest.html)
- 🔧 [API Guidelines](https://rust-lang.github.io/api-guidelines/)
- 📝 [Documentation Guidelines](https://doc.rust-lang.org/rustdoc/how-to-write-documentation.html)

### Versionamento Semântico

- 📊 [SemVer Specification](https://semver.org/)
- 🔢 [Cargo SemVer Compatibility](https://doc.rust-lang.org/cargo/reference/semver.html)

### Segurança e Criptografia

- 🔐 [RFC 7519 - JWT](https://tools.ietf.org/html/rfc7519)
- 🔒 [HMAC-SHA256 Specification](https://tools.ietf.org/html/rfc4868)
- 🛡️ [Rust Security Guidelines](https://anssi-fr.github.io/rust-guide/)

### Melhores Práticas

- ✨ [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- 🎯 [Effective Rust](https://www.lurklurk.org/effective-rust/)
- 📚 [Rust Design Patterns](https://rust-unofficial.github.io/patterns/)

### Ferramentas Úteis

- 🔍 [cargo-audit](https://github.com/RustSec/rustsec/tree/main/cargo-audit) - Verificação de vulnerabilidades
- 📊 [cargo-outdated](https://github.com/kbknapp/cargo-outdated) - Verificar dependências desatualizadas
- ⚡ [cargo-edit](https://github.com/killercup/cargo-edit) - Adicionar/remover/atualizar dependências
- 🎨 [cargo-make](https://github.com/sagiegurari/cargo-make) - Task runner
- 📈 [cargo-benchcmp](https://github.com/BurntSushi/cargo-benchcmp) - Comparar benchmarks

---

## 🆘 Suporte

### Problemas com o Script

Se encontrar problemas com o script de publicação:

1. **Verifique os logs:**
   ```powershell
   Get-Content publish-log-*.txt -Tail 50
   ```

2. **Execute em modo debug:**
   ```powershell
   $VerbosePreference = "Continue"
   .\publish-dependencies.ps1 -DryRun
   ```

3. **Reporte issues:**
   - Repository: https://github.com/avilaops/arxis
   - Issues: https://github.com/avilaops/arxis/issues

### Contatos

- 📧 **Email:** avilacloud@avila.cloud
- 🐛 **Bug Reports:** https://github.com/avilaops/arxis/issues
- 💬 **Discussions:** https://github.com/avilaops/arxis/discussions
- 📖 **Documentation:** https://docs.avila.cloud

---

## 📝 Notas Finais

### Checklist Final

Antes de considerar a publicação completa:

- [ ] Todas as dependências publicadas no crates.io
- [ ] Versões sincronizadas entre Cargo.toml local e crates.io
- [ ] Documentação atualizada (README, CHANGELOG)
- [ ] Testes passando em CI/CD
- [ ] Exemplos funcionando
- [ ] Benchmarks executados
- [ ] Security audit realizado (`cargo audit`)
- [ ] Licenças verificadas

### Próximos Passos

1. **CI/CD Pipeline:**
   - Configurar GitHub Actions para testes automatizados
   - Adicionar verificação de publicação em PRs
   - Automatizar release notes

2. **Documentação:**
   - Criar docs.rs para cada pacote
   - Adicionar exemplos práticos
   - Criar tutoriais de integração

3. **Monitoramento:**
   - Configurar alertas de download
   - Monitorar issues reportadas
   - Coletar feedback da comunidade

4. **Melhorias Futuras:**
   - Adicionar mais features
   - Otimizar performance
   - Expandir testes de integração

---

**Última atualização:** 2 de Dezembro de 2025
**Versão do Guia:** 2.0
**Mantenedor:** Avila Team

---

🌩️ **Avila Cloud** - Enterprise Cloud Infrastructure Platform
