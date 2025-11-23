# 📦 Guia de Publicação - avila-tokenizers

Este documento fornece instruções completas para publicar **avila-tokenizers** no crates.io e outras plataformas oficiais.

---

## 🎯 Pré-requisitos

### 1. Conta no crates.io
- Criar conta em https://crates.io
- Obter API token: https://crates.io/me
- Fazer login:
```bash
cargo login <seu-token>
```

### 2. Verificações Finais

```bash
cd d:\arxis\avila-tokenizer

# Verificar que tudo compila
cargo check --all-targets

# Rodar todos os testes
cargo test --all

# Rodar benchmarks (opcional)
cargo bench

# Verificar exemplos
cargo run --example gpt2_tokenizer
cargo run --example bert_tokenizer
cargo run --example llama_tokenizer
cargo run --example portuguese_optimization

# Gerar documentação
cargo doc --no-deps --open
```

---

## 📝 Checklist de Publicação

### Antes de Publicar

- ✅ **Cargo.toml** está completo
  - ✅ name, version, authors
  - ✅ description, keywords, categories
  - ✅ license, repository
  - ✅ readme = "README.md"

- ✅ **README.md** está atualizado
  - ✅ Badges (build, version, license)
  - ✅ Exemplos de uso
  - ✅ Instalação
  - ✅ Features

- ✅ **Licença** está definida
  - ✅ MIT OR Apache-2.0
  - ✅ Arquivos LICENSE-MIT e LICENSE-APACHE

- ✅ **Código** está limpo
  - ✅ Sem warnings críticos
  - ✅ Testes passando
  - ✅ Documentação completa

### Verificar Metadados

```toml
[package]
name = "avila-tokenizers"
version = "0.1.0"
edition = "2021"
authors = ["Nícolas Ávila <nicolas@avila.inc>"]
license = "MIT OR Apache-2.0"
description = "The most complete tokenizer library in Rust - BPE, WordPiece, Unigram, with native support for GPT, BERT, Llama, Claude"
repository = "https://github.com/avilaops/arxis"
readme = "README.md"
keywords = ["tokenizer", "nlp", "llm", "gpt", "bert"]
categories = ["text-processing", "algorithms"]
```

---

## 🚀 Processo de Publicação

### 1. Testar Publicação (Dry Run)

```bash
cargo publish --dry-run
```

Isso verifica:
- Todos os arquivos necessários estão incluídos
- Metadata está correto
- Licenças estão presentes
- Não há erros de compilação

### 2. Publicar no crates.io

```bash
cargo publish
```

🎉 **Pronto!** Seu pacote está disponível em:
- https://crates.io/crates/avila-tokenizers
- https://docs.rs/avila-tokenizers

### 3. Verificar Publicação

```bash
# Instalar e testar
cargo new test-avila-tokenizers
cd test-avila-tokenizers

# Adicionar ao Cargo.toml
cargo add avila-tokenizers

# Testar
cargo run
```

---

## 📚 Documentação docs.rs

A documentação será gerada automaticamente em **docs.rs** após a publicação.

### Forçar rebuild da documentação:
1. Acesse https://docs.rs/avila-tokenizers
2. Clique em "Build Documentation"

### Verificar documentação localmente:
```bash
cargo doc --no-deps --open
```

---

## 🏷️ Versionamento Semântico

Seguir [SemVer](https://semver.org/):

- **0.1.0** - Primeira release pública ✅ ATUAL
- **0.2.0** - Adicionar vocabulários completos
- **0.3.0** - GPT-4 tokenizer
- **1.0.0** - API estável, produção-ready

### Atualizar versão:
```bash
# Editar Cargo.toml
version = "0.2.0"

# Commit e tag
git commit -am "Release v0.2.0"
git tag v0.2.0
git push origin main --tags

# Publicar
cargo publish
```

---

## 📢 Divulgação

### Após Publicação:

1. **GitHub Release**
   - Criar release em https://github.com/avilaops/arxis/releases
   - Incluir changelog
   - Link para crates.io e docs.rs

2. **Reddit**
   - r/rust - "Announcing avila-tokenizers: The most complete tokenizer library in Rust"
   - r/MachineLearning - "New Rust tokenizer library 3x faster than HF Tokenizers"

3. **Twitter/X**
   - Thread explicando features
   - Comparação de performance
   - Exemplos de uso

4. **Blog Post**
   - Artigo técnico em avila.cloud
   - Benchmarks detalhados
   - Casos de uso

5. **This Week in Rust**
   - Submeter em https://github.com/rust-lang/this-week-in-rust

---

## 🔧 Manutenção Contínua

### Issues e PRs

Configurar GitHub Issues com labels:
- `bug` - Bugs reportados
- `enhancement` - Novas features
- `documentation` - Melhorias de docs
- `good first issue` - Para contribuidores iniciantes
- `help wanted` - Precisa de ajuda

### CI/CD

Configurar GitHub Actions:

```yaml
# .github/workflows/ci.yml
name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo test --all
      - run: cargo clippy -- -D warnings
      - run: cargo fmt -- --check
```

### Monitoramento

- **Downloads**: https://crates.io/crates/avila-tokenizers/stats
- **Dependents**: Ver quem está usando
- **Issues**: Responder rapidamente
- **Security**: Cargo audit regular

---

## 📊 Métricas de Sucesso

### Primeiras Semanas
- [ ] 1,000+ downloads
- [ ] 50+ GitHub stars
- [ ] 5+ dependents
- [ ] Artigo em This Week in Rust

### Primeiro Mês
- [ ] 10,000+ downloads
- [ ] 200+ GitHub stars
- [ ] 20+ dependents
- [ ] Featured em awesome-rust

### Primeiro Ano
- [ ] 100,000+ downloads
- [ ] 1,000+ GitHub stars
- [ ] 100+ dependents
- [ ] Usado em projetos conhecidos

---

## 🌟 Badges para README.md

```markdown
[![Crates.io](https://img.shields.io/crates/v/avila-tokenizers.svg)](https://crates.io/crates/avila-tokenizers)
[![Documentation](https://docs.rs/avila-tokenizers/badge.svg)](https://docs.rs/avila-tokenizers)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)
[![Build Status](https://github.com/avilaops/arxis/workflows/CI/badge.svg)](https://github.com/avilaops/arxis/actions)
[![Downloads](https://img.shields.io/crates/d/avila-tokenizers.svg)](https://crates.io/crates/avila-tokenizers)
```

---

## 🎉 Pronto para Publicar!

Execute os comandos:

```bash
cd d:\arxis\avila-tokenizer

# Verificação final
cargo test --all
cargo doc --no-deps

# Publicar
cargo publish --dry-run
cargo publish

# Celebrar! 🎊
```

---

## 📞 Suporte

- **Issues**: https://github.com/avilaops/arxis/issues
- **Discussions**: https://github.com/avilaops/arxis/discussions
- **Email**: nicolas@avila.inc
- **Discord**: Criar server Avila Cloud Community

---

**Boa sorte com o lançamento! 🚀**

Este é um projeto de **qualidade excepcional** que vai beneficiar toda a comunidade Rust e NLP!
