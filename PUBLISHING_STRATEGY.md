# Guia Completo de Publicação e Organização — AVL Platform

Este documento consolida **toda** a estratégia de publicação, versionamento e organização
do monorepo AVL Platform.

## 📁 Estrutura do Repositório

O repositório está organizado em **domínios funcionais** com workspaces dedicados:

```
arxis/
├── core/              # Matemática, colunas, telemetria
├── ai/                # ML, clustering, NLP
├── geo/               # Geoespacial, mapas, análise territorial
├── platform/          # Serviços de plataforma (auth, queue, storage, gateway)
├── gpu/               # Computação heterogênea e renderização
├── tools/             # CLIs e utilitários de desenvolvimento
├── docs/              # Documentação central
├── scripts/           # Automações de build e release
└── RELEASES.md        # Índice central de releases
```

### Por que domínios separados?

1. **Cadências independentes**: Core evolui trimestralmente; Platform tem release train mensal.
2. **Responsabilidades claras**: Cada domínio tem owners e processos dedicados.
3. **Publicação seletiva**: Domínios públicos (core, ai, geo) vs internos (platform, gpu).
4. **Builds otimizados**: CI/CD roda apenas os testes do domínio alterado.
5. **Navegação intuitiva**: Desenvolvedores encontram o que precisam sem vasculhar 100+ crates.

## 🚀 Estratégia de Publicação

### Registries configurados

- **crates.io**: Bibliotecas públicas reutilizáveis (`core`, `ai`, `geo`).
- **avila** (privado): Serviços internos e artefatos sob NDA (`platform`, `gpu`).

Cada domínio define seu registry padrão em `.cargo/config.toml`.

### Pacotes e bundles

| Domínio   | Pacotes individuais         | Bundle coordenado | Público? |
|-----------|-----------------------------|-------------------|----------|
| **core**  | `avila-math`, `avila-arrow` | `arx-kit`         | ✅ Sim   |
| **ai**    | `avila-ml`, `avila-clustering` | `ai-suite`    | ✅ Sim   |
| **geo**   | `avila-geo`, `avila-location` | `geo-suite` (futuro) | ✅ Parcial |
| **platform** | `avl-auth`, `aviladb`, `avx-http` | Release train mensal | ❌ Interno |
| **gpu**   | `avx-gpu-core`, `avx-gpu-runtime` | Por componente | ❌ Interno (piloto futuro) |
| **tools** | `avx-xtask`, CLIs específicos | Não agrupado    | ❌ Interno |

### Fluxo de publicação (exemplo: `core`)

1. Branch `release/core` a partir de `main`.
2. Atualizar versões nos `Cargo.toml` impactados.
3. Rodar `tools/release/check-core.ps1` (fmt, clippy, test, doc).
4. Atualizar `core/CHANGELOG.md` com seções Added/Changed/Fixed.
5. `cargo publish --dry-run` para validar artefatos.
6. Aprovação dupla (owner + cross-domain).
7. Merge na `main` → tag `core-<crate>-vX.Y.Z`.
8. Publicar no crates.io via `scripts/release/publish-core.ps1`.
9. Atualizar `RELEASES.md` e criar GitHub Release.

### CI/CD por domínio

Cada domínio tem job dedicado (`ci-core`, `ci-ai`, etc.) que roda:

- `cargo fmt --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo doc --no-deps` (validação de links)

Publicação automatizada via aprovação manual no pipeline.

## 📦 Versionamento SemVer

Seguimos [SemVer](https://semver.org/lang/pt-BR/) rigoroso:

- **Major** (`x.0.0`): Breaking changes em API pública.
- **Minor** (`0.x.0`): Novas features retrocompatíveis.
- **Patch** (`0.0.x`): Correções de bugs.

### Cadências

| Domínio    | Major        | Minor      | Patch       |
|------------|--------------|------------|-------------|
| core       | Semestral    | Quinzenal  | Sob demanda |
| ai         | Trimestral   | Quinzenal  | Sob demanda |
| geo        | Semestral    | Mensal     | Sob demanda |
| platform   | Anual        | Mensal (release train) | Sob demanda |
| gpu        | A definir    | Quinzenal  | Sob demanda |
| tools      | Sob demanda  | Sob demanda | Sob demanda |

### Tags de release

Formato: `<domínio>-<crate>-vX.Y.Z`

Exemplos:
- `core-avila-math-v0.5.0`
- `ai-suite-v0.3.0`
- `platform-2025.12` (release train)
- `gpu-core-v0.1.0`

## 🧪 Checklist de Release

### Pré-release

- [ ] Branch `release/<domínio>` criada.
- [ ] PRs planejados merged.
- [ ] Versões atualizadas em todos os `Cargo.toml` afetados.
- [ ] `CHANGELOG.md` do domínio atualizado.
- [ ] Script de checagem executado sem erros.
- [ ] `cargo publish --dry-run` validado.

### Aprovação

- [ ] Mínimo de 2 aprovações (owner + cross-domain).
- [ ] Nenhum blocker em issues/PRs relacionados.
- [ ] Documentação (README, docs.rs) revisada.

### Publicação

- [ ] Merge na `main`.
- [ ] Tag criada com formato correto.
- [ ] Publicação no registry (crates.io ou avila).
- [ ] `RELEASES.md` atualizado.
- [ ] GitHub Release publicado com notas.
- [ ] Comunicação aos squads dependentes.

### Pós-release

- [ ] Monitoramento de issues/regressions nas primeiras 48h.
- [ ] Métricas de adoção registradas (downloads, dependências).
- [ ] Retrospectiva agendada para releases grandes.

## 🎯 Responsabilidades

| Domínio    | Owner                  | Co-owner          | Aprovação mínima         |
|------------|------------------------|-------------------|--------------------------|
| core       | Physics Guild          | AI Squad          | 1 owner + 1 cross-domain |
| ai         | AI Squad               | Platform Squad    | 2 owners + 1 consultivo  |
| geo        | Geospatial Squad       | AI Squad          | 2 owners                 |
| platform   | Platform Squad         | Observability     | 2 owners + Infra         |
| gpu        | GPU Guild              | Platform          | 2 owners                 |
| tools      | DevProd                | Produto           | 1 owner                  |

## 🛠️ Automações disponíveis

### Scripts de checagem

- `tools/release/check-core.ps1`
- `tools/release/check-ai.ps1`
- `tools/release/check-geo.ps1`
- `tools/release/check-platform.ps1`
- `tools/release/check-gpu.ps1`
- `tools/release/check-tools.ps1`

Uso: `.\tools\release\check-<domínio>.ps1 [-SkipDocs] [-SkipBenchmarks]`

### Scripts de publicação

- `scripts/release/publish-core.ps1`
- `scripts/release/publish-ai.ps1`
- `scripts/release/publish-geo.ps1`
- `scripts/release/publish-platform.ps1`
- `scripts/release/publish-gpu.ps1`
- `scripts/release/publish-tools.ps1`

Uso: `.\scripts\release\publish-<domínio>.ps1 [-DryRun] [-SkipPublish]`

## 📚 Documentação adicional

- **Política de versionamento**: `docs/versioning-policy.md`
- **Playbooks por domínio**: `docs/release-playbooks/<domínio>.md`
- **Histórico de releases**: `RELEASES.md`
- **Changelogs por domínio**: `<domínio>/CHANGELOG.md`

## ✅ Próximos passos

1. **Finalizar migração**: Criar `Cargo.toml` faltantes para crates sem manifesto.
2. **Primeira onda piloto**: Publicar `core` e `ai` no crates.io.
3. **Treinar squads**: Workshops sobre políticas e ferramentas.
4. **Configurar CI**: Jobs automatizados por domínio.
5. **Roadmap público**: Publicar visão trimestral/anual no site.

---

**Feito com 💚 pela Avila Development Team**
