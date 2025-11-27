# Política de Versionamento da AVL Platform

Este documento descreve como a organização aplica **SemVer** e como os domínios do monorepo
são versionados de forma coordenada.

## Princípios gerais

1. Cada crate segue [SemVer](https://semver.org/lang/pt-BR/) de forma independente, salvo indicação contrária.
2. Domínios possuem cadência própria de releases, documentada em `RELEASES.md` e nos *playbooks*.
3. Incrementos de versão exigem atualização explícita em `Cargo.toml`, `CHANGELOG.md` e `RELEASES.md`.
4. Breaking changes só são aceitos com *major bump* ou feature flag desativada por padrão.
5. Hotfixes devem ser lançados como patch (`x.y.z+1`) com changelog enxuto e tag GPG.

## Estratégia de branches

- `main`: sempre estável. Ganha tags diárias (`main-AAAA.MM.DD`).
- `release/<domínio>`: preparação de release por domínio (ex.: `release/core`).
- `feature/*`: desenvolvimento focado; merges via PR.

## Checklist de PR

- [ ] Bump de versão quando altera API pública.
- [ ] Atualização do changelog do domínio.
- [ ] Execução do pipeline de CI específico.
- [ ] Validação `cargo fmt`, `cargo clippy --all-targets`, `cargo test --all` (ou escopo equivalente).
- [ ] Atualização da documentação afetada.

## Matriz de suportes

| Domínio    | Cadência Major | Cadência Minor | Hotfix | Estratégia |
|------------|----------------|----------------|--------|------------|
| core       | Semestral      | Quinzenal      | Sob demanda | SemVer completo |
| ai         | Trimestral     | Quinzenal      | Sob demanda | SemVer completo |
| geo        | Semestral      | Mensal         | Sob demanda | SemVer + releases internas |
| platform   | Anual          | Mensal         | Sob demanda | Release train |
| gpu        | Sob definição  | Quinzenal      | Sob demanda | Interno + piloto público |
| tools      | Sob demanda    | Sob demanda    | Sob demanda | Versionamento leve |

## Tags de release

- `core-arx-math-vX.Y.Z`: release específico de crate core.
- `ai-suite-vX.Y.Z`: bundle coordenado de IA.
- `platform-AAAA.MM`: release train da plataforma.
- `gpu-core-vX.Y.Z`: principais componentes GPU.

## Artefatos gerados

- `RELEASES.md`: índice central dos changelogs.
- `docs/release-playbooks/<domínio>.md`: passo a passo operacional.
- `scripts/release/publish-<domínio>.ps1`: automatizações oficiais.

## Conformidade

- Todo bump de versão exige aprovação dupla (owner + cross-domain).
- As equipes devem registrar métricas de regressão utilizando `avl-observability`.
- Auditorias semestrais garantem que versões publicadas continuam reproduzíveis.
