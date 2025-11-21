# 🚀 Workflow de Publicação - Arxis

## Para Funcionários Novos: LEIA ANTES DE PUBLICAR

### ⚠️ IMPORTANTE: Não publicar manualmente!

Este repositório tem **automação completa** para publicação. Você **NÃO** deve rodar `cargo publish` manualmente.

---

## 📋 Checklist Pré-Publicação

Antes de criar uma release, verifique:

1. ✅ **CHANGELOG.md** atualizado com as mudanças da versão
2. ✅ Todas as versões nos `Cargo.toml` atualizadas (use `./scripts/bump-version.sh` se disponível)
3. ✅ Testes passando: `cargo test --workspace`
4. ✅ Build funcionando: `cargo build --workspace --release`
5. ✅ Commit e push de todas as mudanças

---

## 🎯 Como Publicar uma Nova Versão

### Passo 1: Criar e Publicar Tag

```bash
# Certifique-se de estar na branch main
git checkout main
git pull origin main

# Crie a tag com a versão (formato: v{major}.{minor}.{patch})
git tag v0.1.0

# Publique a tag
git push origin v0.1.0
```

### Passo 2: Aguardar Automação

O GitHub Actions vai **automaticamente**:

1. ✅ Criar uma GitHub Release
2. ✅ Publicar todos os crates no crates.io na ordem correta:
   - `avila-math`
   - `avila-telemetry`
   - `avx-config`
   - `avx-telemetry`
   - `avx-quantum-render`
   - `avx-image`
   - `avx-gateway`
   - `avx-api-core`
   - `avx-cli`
   - `avx-events`
   - `arxis_quaternions`
3. ✅ Compilar binários para Linux, Windows e macOS
4. ✅ Anexar binários na release

---

## 🔐 Secrets Necessários (Apenas Admins)

**Você não precisa configurar isso, mas saiba que existem:**

### GitHub Secrets Required:
- `GH_TOKEN` - Token do GitHub com permissão `repo` e `write:packages`
- `CARGO_REGISTRY_TOKEN` - Token do crates.io para publicação

**Localização:** https://github.com/avilaops/arxis/settings/secrets/actions

---

## 🔍 Monitorar Publicação

Após fazer push da tag, acompanhe em:

https://github.com/avilaops/arxis/actions

Você verá 3 jobs rodando:
1. **Create Release** - Cria a release no GitHub
2. **Publish to crates.io** - Publica os crates na ordem
3. **Build Binaries** - Compila para Linux/Windows/macOS

---

## ❌ O Que NÃO Fazer

### 🚫 NUNCA rode manualmente:
```bash
cargo publish -p avila-math  # ❌ NÃO FAÇA ISSO
```

### 🚫 NUNCA crie releases manualmente no GitHub UI

### 🚫 NUNCA faça push de tags sem atualizar CHANGELOG.md

---

## 🐛 Problemas Comuns

### Erro: "crate already exists"
**Solução:** Versão já publicada. Incremente a versão e crie nova tag.

### Erro: "failed to authenticate"
**Solução:** Token expirado. Contate admin para renovar `CARGO_REGISTRY_TOKEN`.

### Erro: "workspace has multiple roots"
**Solução:** Não adicione `[workspace]` em crates individuais. Workspace está no `Cargo.toml` raiz.

### Build falhou no CI
**Solução:** Rode `cargo build --workspace --release` localmente primeiro.

---

## 📦 Ordem de Dependências

Os crates são publicados nesta ordem porque têm dependências entre si:

```
avila-math (sem deps internas)
  ↓
avila-telemetry (depende de avila-math)
  ↓
avx-config
  ↓
avx-telemetry, avx-quantum-render, avx-image
  ↓
avx-gateway, avx-api-core
  ↓
avx-cli, avx-events
  ↓
arxis_quaternions
```

**Nunca altere a ordem no `release.yml` sem validar as dependências!**

---

## 📞 Dúvidas?

- Leia o código em `.github/workflows/release.yml`
- Pergunte no canal #dev-rust
- Contate: nicolas@avila.inc

---

## 🎓 Exemplo Completo

```bash
# 1. Atualizar versões
vim Cargo.toml  # Mudar version = "0.2.0"
vim CHANGELOG.md  # Adicionar mudanças

# 2. Commit
git add .
git commit -m "chore: Bump version to 0.2.0"
git push origin main

# 3. Tag e Release
git tag v0.2.0
git push origin v0.2.0

# 4. Aguardar ☕
# Acompanhe em: https://github.com/avilaops/arxis/actions
```

---

**Lembrete:** A automação existe para **proteger você** de erros. Confie no processo! 🛡️
