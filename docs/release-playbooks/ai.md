# Playbook de Release — Domínio `ai`

1. Criar branch `release/ai` a partir de `main`.
2. Consolidar PRs de features planejadas para o bundle (ex.: `ai-suite`).
3. Atualizar versões das crates impactadas e sincronizar dependências internas via `cargo metadata`.
4. Executar `cargo test --workspace --manifest-path ai/Cargo.toml` e `cargo doc --no-deps`.
5. Rodar `tools/release/check-ai.ps1` para validações adicionais (lint, coverage, exemplos RAG/Vision).
6. Revisar e atualizar `ai/CHANGELOG.md` com seções Added/Changed/Fixed.
7. Realizar `cargo publish --dry-run` nas crates destinadas ao crates.io.
8. Obter duas aprovações (AI Squad + co-owner).
9. Merge na `main`, taggear `ai-suite-vX.Y.Z` e publicar notas associadas.
10. Atualizar `RELEASES.md` com links para as versões.
