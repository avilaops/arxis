# 📦 Guia de Publicação no crates.io

## Pré-requisitos

1. **Conta no crates.io**
   - Acesse: https://crates.io/
   - Faça login com GitHub

2. **API Token**
   - Vá em: https://crates.io/settings/tokens
   - Clique em "New Token"
   - Copie o token gerado

## Passo a Passo

### 1. Fazer Login
```bash
cargo login <SEU_TOKEN_AQUI>
```

### 2. Verificar se está tudo OK (dry-run)
```bash
cargo publish --dry-run
```

### 3. Publicar de verdade! 🚀
```bash
cargo publish
```

## ⚠️ IMPORTANTE - Antes de Publicar

### Verificar se o nome está disponível
```bash
cargo search face3d-rs
```

Se já existir, você precisará mudar o nome no `Cargo.toml`.

### Alternativas de nome (caso `face3d-rs` esteja ocupado):
- `face3d-modeling`
- `face-3dmm`
- `morphable-faces`
- `flame-face-model`
- `avila-face3d`

### Criar repositório no GitHub (recomendado)

Antes de publicar, é bom ter o código no GitHub:

```bash
# Inicializar git
git init

# Adicionar todos os arquivos
git add .

# Commit inicial
git commit -m "Initial commit: face3d-rs v0.1.0"

# Criar repositório no GitHub e adicionar remote
git remote add origin https://github.com/SEU_USUARIO/face3d-rs.git

# Push
git branch -M main
git push -u origin main
```

Depois atualize o `Cargo.toml` com a URL real do repositório.

## Após Publicação

1. Verificar em: https://crates.io/crates/face3d-rs
2. Documentação será gerada automaticamente em: https://docs.rs/face3d-rs
3. Aguardar ~10 minutos para indexação

## Publicar Nova Versão

1. Atualizar versão no `Cargo.toml`
2. Fazer commit das mudanças
3. Criar tag git: `git tag v0.1.1`
4. Push da tag: `git push --tags`
5. Publicar: `cargo publish`

## Badge para README

Adicione ao README.md:
```markdown
[![Crates.io](https://img.shields.io/crates/v/face3d-rs.svg)](https://crates.io/crates/face3d-rs)
[![Documentation](https://docs.rs/face3d-rs/badge.svg)](https://docs.rs/face3d-rs)
[![License](https://img.shields.io/crates/l/face3d-rs.svg)](https://github.com/avila-cloud/face3d-rs/blob/main/LICENSE)
```

## Troubleshooting

### "name is already taken"
Mude o nome no `Cargo.toml` e tente novamente.

### "token not found"
Execute `cargo login` novamente.

### "documentation failed to build"
Verifique com `cargo doc --no-deps` localmente.

---

**Boa sorte com a publicação! 🦀🚀**
