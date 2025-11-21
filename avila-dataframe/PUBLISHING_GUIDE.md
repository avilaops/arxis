# 📦 Guia de Publicação - crates.io

## ✅ Checklist Pré-Publicação

### 1. Metadados do Cargo.toml
- [x] `name` - avila-dataframe
- [x] `version` - 0.1.0
- [x] `edition` - 2021
- [x] `rust-version` - 1.70
- [x] `authors` - AVL Cloud Platform
- [x] `description` - Descrição clara e concisa
- [x] `documentation` - https://docs.rs/avila-dataframe
- [x] `homepage` - https://avila.cloud/aviladf
- [x] `repository` - GitHub URL
- [x] `license` - Apache-2.0
- [x] `keywords` - 5 palavras-chave relevantes
- [x] `categories` - Categorias do crates.io
- [x] `readme` - README.md
- [x] `exclude` - Arquivos desnecessários excluídos

### 2. Documentação
- [x] README.md com badges, exemplos e links
- [x] CHANGELOG.md com histórico de versões
- [x] LICENSE (Apache-2.0)
- [x] Doc comments em `lib.rs`
- [x] Exemplos funcionando (`examples/`)
- [x] Testes de integração (`tests/`)

### 3. Código
- [x] Zero warnings na compilação
- [x] Todos os testes passando
- [x] Exemplos compilando e executando
- [x] Benchmarks funcionando
- [x] Features configuradas corretamente

### 4. GitHub
- [ ] Repositório público
- [ ] README no repo
- [ ] LICENSE no repo
- [ ] GitHub Actions CI/CD (opcional)
- [ ] Release tags (v0.1.0)

## 🚀 Comandos para Publicação

### 1. Verificar o pacote
```bash
cargo package --list
```
Isso mostra todos os arquivos que serão incluídos no crate.

### 2. Build e teste local
```bash
cargo build --release
cargo test --all-features
cargo test --no-default-features
cargo doc --no-deps --open
```

### 3. Verificar warnings
```bash
cargo clippy -- -D warnings
cargo check --all-features
```

### 4. Fazer login no crates.io
```bash
cargo login <seu-token-do-crates-io>
```
Token obtido em: https://crates.io/me

### 5. Dry-run (teste sem publicar)
```bash
cargo publish --dry-run
```
Verifica se tudo está OK sem publicar de verdade.

### 6. PUBLICAR! 🎉
```bash
cargo publish
```

## 📋 Após Publicação

### 1. Verificar no crates.io
- https://crates.io/crates/avila-dataframe
- Aguardar ~5 minutos para docs.rs processar

### 2. Testar instalação
```bash
cargo new test-avila
cd test-avila
cargo add avila-dataframe
cargo build
```

### 3. Criar GitHub Release
```bash
git tag v0.1.0
git push origin v0.1.0
```
Depois criar release no GitHub com CHANGELOG.

### 4. Anunciar! 📣
- [ ] Tweet
- [ ] Discord #announcements
- [ ] Reddit r/rust
- [ ] Hacker News (optional)
- [ ] Blog post

## 🔄 Próximas Versões

Para publicar v0.1.1, v0.2.0, etc:

1. Atualizar `version` em `Cargo.toml`
2. Atualizar `CHANGELOG.md`
3. Commit: `git commit -am "Release v0.1.1"`
4. Tag: `git tag v0.1.1 && git push origin v0.1.1`
5. `cargo publish`

## ⚠️ Importante

- **NÃO é possível deletar** uma versão após publicar
- **NÃO é possível re-publicar** com mesmo número de versão
- Use `cargo yank` se houver problema grave
- Sempre teste com `--dry-run` primeiro!

## 📞 Suporte

Se algo der errado:
- docs.crates.io - Documentação oficial
- Discord Rust #crates-io
- support@crates.io

---

**Boa sorte! 🚀🇧🇷**
