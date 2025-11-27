# 📦 Tracking de Publicação - Crates.io

**Data**: 27 Nov 2025 | **Registry**: https://crates.io/users/Avilaops

## Status: 18/28 Publicadas (64.3%)

## ❌ Pendentes (10)

### 🧬 Avila - Scientific Computing (5 crates)

| Crate | Versão Local | Prioridade | Motivo |
|-------|--------------|------------|--------|
| **avila-dataframe** | 0.1.0 | 🔴 ALTA | Core data science - Polars integration |
| **avila-geo** | 0.1.0 | 🔴 ALTA | Geospatial computations & cartography |
| **avila-ml** | 0.1.0 | 🔴 ALTA | Machine learning core (substitui smartcore) |
| **avila-reduction** | 0.1.0 | 🟡 MÉDIA | Dimensionality reduction (PCA, t-SNE) |
| **avila-tokenizer** | 0.1.0 | 🟡 MÉDIA | Text tokenization (BPE, WordPiece, Unigram) |

### ☁️ AVL - Cloud Platform (1 crate)

| Crate | Versão Local | Prioridade | Motivo |
|-------|--------------|------------|--------|
| **avl-loadbalancer** | 0.1.0 | 🟢 BAIXA | L7 load balancer |

### 🎮 AVX - API Gateway & Rendering (4 crates)

| Crate | Versão Local | Prioridade | Motivo |
|-------|--------------|------------|--------|
| **avx-api-core** | 0.1.0 | 🔴 ALTA | Tipos fundamentais da API |
| **avx-gateway** | 0.1.0 | 🔴 ALTA | API Gateway HTTP/WebSocket |
| **avx-gpu** | 0.1.0 | 🟡 MÉDIA | Computação GPU |
| **avx-quantum-render** | 0.1.0 | 🟢 BAIXA | Renderer experimental QED |

## 🎯 Prioridades

### 🔴 Fase 1 - Crítica (até 1 Dez)

1. **avila-dataframe** - Bloqueador data science (dep: avila-arrow)
2. **avila-ml** - Bloqueador ML (dep: avila-math, avila-linalg)
3. **avx-api-core** - Bloqueador tipos API
4. **avx-gateway** - Bloqueador gateway (dep: avx-http, avx-api-core)
5. **avila-geo** - Geolocalização (dep: avila-math)

### 🟡 Fase 2 - Média (8-21 Dez)

6. **avila-reduction** - PCA, t-SNE (dep: avila-linalg)
7. **avila-tokenizer** - Tokenização NLP
8. **avx-gpu** - Computação GPU

### 🟢 Fase 3 - Baixa (22+ Dez)

9. **avl-loadbalancer** - Load balancer L7 (dep: avx-http)
10. **avx-quantum-render** - Renderer experimental (dep: avx-gpu)

---

## 🚀 Plano de Publicação

### Semana 1 (Imediato - até 1 Dez 2025)
```bash
# 1. avila-dataframe
cd d:\GitHub\arxis\avila-dataframe
cargo test
cargo publish

# 2. avila-ml
cd d:\GitHub\arxis\avila-ml
cargo test
cargo publish

# 3. avx-api-core
cd d:\GitHub\arxis\avx-api-core
cargo test
cargo publish

# 4. avx-gateway
cd d:\GitHub\arxis\avx-gateway
cargo test
cargo publish

# 5. avila-geo
cd d:\GitHub\arxis\avila-geo
cargo test
cargo publish
```

### Semana 2-3 (8-21 Dez 2025)
```bash
# 6. avila-reduction
cd d:\GitHub\arxis\avila-reduction
cargo test
cargo publish

# 7. avila-tokenizer
cd d:\GitHub\arxis\avila-tokenizer
cargo test
cargo publish

# 8. avx-gpu
cd d:\GitHub\arxis\avx-gpu
cargo test
cargo publish
```

## 🚀 Automação

```powershell
# Teste sem publicar
.\scripts\publish-crates.ps1 -DryRun -Fase Fase1

# Publicação real por fase
.\scripts\publish-crates.ps1 -Fase Fase1  # Crítica
.\scripts\publish-crates.ps1 -Fase Fase2  # Média
.\scripts\publish-crates.ps1 -Fase Fase3  # Baixa
```

**Documentação**: `scripts/README.md`

---

**Maintainer**: Nícolas Ávila | **Email**: nicolas@avila.inc  
**Atualizado**: 27 Nov 2025 | **Meta**: 28/28 até Jan 2026

---

## 🔗 Links Úteis

- **Registry**: https://crates.io/users/Avilaops
- **GitHub**: https://github.com/avilaops/arxis
- **Docs**: https://docs.avila.inc
- **Publishing Guide**: https://doc.rust-lang.org/cargo/reference/publishing.html

---

**Atualizado em**: 27 de novembro de 2025
**Próxima Revisão**: Após publicação dos pendentes

---

## 🚀 Estratégia Geral de Publicação

### Divisão em Workspaces por Domínio
- Estruturar o monorepo em `core/`, `ai/`, `geo/`, `platform/`, `gpu/` e `tools/`, cada qual com `Cargo.toml` próprio.
- Garantir que cada diretório mantenha um `README.md` descrevendo escopo, APIs estáveis e política de versionamento.

### Indexador de Releases
- Criar `RELEASES.md` na raiz com links para os changelogs de cada domínio (`core/CHANGELOG.md`, `ai/CHANGELOG.md`, etc.).
- Manter versionamento independente por domínio seguindo SemVer (`major.minor.patch`).

### Fluxos de Publicação por Domínio
- `core/` e `ai/`: publicáveis no crates.io (bibliotecas reutilizáveis).
- `geo/`: publicar apenas crates genéricos (`geo-core`, `geo-routing`); projetos específicos permanecem internos.
- `platform/` e `gpu/`: inicialmente em registry privado, com possibilidade futura de expor crates selecionados (ex.: `plt-config`).
- `tools/`: focado em binários internos; publicar no crates.io apenas se houver ganho externo claro.

### Configuração de Registries
- Definir o registry padrão por domínio via `[registry]` no `Cargo.toml`.
- Serviços internos utilizam registry privado (`[registries.avila]`).
- Bibliotecas públicas vão para crates.io com metadata completa, badges e documentação (`docs.rs`) habilitada.

### 📦 Pacotes e Bundles por Domínio

**Core**
- Crates individuais com prefixo `arx-*`, cada um com objetivo bem definido.
- Bundle opcional `arx-kit` reexportando subconjunto estável.
- Publicação dos crates individuais sob demanda; `arx-kit` apenas em releases sincronizadas (ex.: `0.5.0` combinando versões compatíveis).

**AI**
- Crates com prefixo `ai-*` e bundle `ai-suite`.
- Releases coordenadas por feature (RAG, Vision, Tabular, etc.), versionando `ai-suite` em cada milestone.

**Geo**
- `geo-core` e bibliotecas genéricas publicáveis.
- Projetos específicos (ex.: `face3d`) permanecem internos, com releases internos (`0.x`).
- Preparar `geo-suite` quando o portfólio estiver maduro (tile server, analytics, telemetry).

**Platform**
- Crates `plt-*` para serviços críticos (auth, config, queue, secrets).
- Releases internos em trem mensal (ex.: `Platform Release 2025.12`).
- Cada release deve gerar changelog resumido, documentação de upgrade e lista de breaking changes.
- Quando exposto publicamente, criar alias de marketing (ex.: `avila-auth-sdk`) reexportando o crate correspondente.

**GPU**
- Workspace `gpu/` com subcrates (`core`, `runtime`, `macros`, `backends`).
- Publicação gradual: `gpu-core` e `gpu-runtime` no crates.io; backends proprietários permanecem privados até a remoção de NDAs.
- Benchmarks e exemplos continuam internos, porém versionados para rastreabilidade.

**Tools**
- CLIs (`cli-*`) publicadas via `cargo install` quando agregarem valor externo (ex.: `cli-telemetry`).
- Ferramentas internas (benchmarks, simuladores) permanecem no monorepo, com versionamento para reprodutibilidade.

### 🧪 Pipeline e QA

**Branch Strategy**
- `main` sempre estável, com tags diárias (`main-YYYY.MM.DD`).
- Branches dedicadas por domínio (`release/core`, `release/ai`, etc.) para preparar releases.
- PRs exigem bump de versão e atualização do changelog correspondente.

**CI/CD por Domínio**
- Jobs dedicados (`ci-core`, `ci-ai`, `ci-platform`, ...).
- Cada job executa `cargo fmt`, `cargo clippy`, `cargo test` e `cargo doc`.
- Publicação automatizada com `cargo release` ou script específico, exigindo dois aprovadores por domínio.

**Checklists de Release**
- Verificar bump de versão (`Cargo.toml`, `Cargo.lock`).
- Atualizar changelog com seções `Added/Changed/Fixed/Deprecated`.
- Validar compatibilidade com dependentes (`cargo tree --workspace`).
- Gerar docs (`cargo doc --no-deps`).
- Gerar pacote (`cargo package`) e revisar artefatos incluídos.
- Assinar tags (`git tag` + assinatura GPG quando aplicável).

### 💡 Passo a Passo de Publicação (Exemplo `arx-math`)
1. Criar branch `release/core`.
2. Agregar PRs pendentes relevantes.
3. Atualizar versão `0.x.y → 0.x.(y+1)` no `Cargo.toml`.
4. Executar `tools/release/check-core.ps1`, garantindo:
	- `cargo test -p arx-math`
	- `cargo doc -p arx-math`
	- `cargo publish --dry-run`
5. Atualizar `core/CHANGELOG.md`.
6. Fazer merge na `main`.
7. Permitir que o CI publique automaticamente (ou executar `cargo publish`).
8. Criar tag `core-arx-math-v0.x.(y+1)` assinada.
9. Publicar release no GitHub com notas.
10. Atualizar `RELEASES.md` com o link correspondente.
11. Repetir o fluxo para os demais domínios.

### 🔄 Planejamento de Versões Futuras
- Majors trimestrais para domínios estáveis (`core 1.0`, `platform 1.0`).
- Minors a cada sprint (duas semanas), alinhadas às entregas de features.
- Patches sob demanda (hotfixes).
- Manter `roadmap.md` por domínio com horizonte de 6/12/24 meses.

### 🧭 Diretórios Auxiliares
- `docs/release-playbooks/` contendo guias rápidos por domínio (checklists, contatos, links).
- `scripts/release/` com automações (ex.: `publish-core.ps1`, `publish-platform.ps1`).
- `docs/versioning-policy.md` detalhando SemVer, estratégia de branches e política de suporte.

### 🧾 Matriz de Responsabilidades

| Domínio   | Owner             | Co-owner         | Aprovação mínima                |
|-----------|-------------------|------------------|---------------------------------|
| core      | Physics Guild     | AI Squad         | 1 owner + 1 cross-domain        |
| ai        | AI Squad          | Platform Squad   | 2 owners + 1 parecer consultivo |
| geo       | Geospatial Squad  | AI Squad         | 2 owners                        |
| platform  | Platform Squad    | Observability    | 2 owners + Infra                |
| gpu       | GPU Guild         | Platform Squad   | 2 owners                        |
| tools     | DevProd           | Produto          | 1 owner                         |

### ✅ Checklist Final
- [ ] Estruturar workspaces por domínio.
- [ ] Definir metadata padrão (authors, license, repository).
- [ ] Configurar registries (crates.io e privado).
- [ ] Criar scripts de release e pipelines de CI por domínio.
- [ ] Elaborar templates de changelog e playbooks.
- [ ] Treinar squads nas políticas e responsabilidades.
- [ ] Executar onda piloto de releases.
- [ ] Publicar guia “Como contribuir/publicar” para a organização.
