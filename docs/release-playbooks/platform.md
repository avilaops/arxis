# Playbook de Release — Domínio `platform`

1. Iniciar branch `release/platform` a partir de `main` na semana -1 do train.
2. Congelar escopo e garantir que migrações e breaking changes estão documentados.
3. Executar `cargo test --workspace --manifest-path platform/Cargo.toml`.
4. Rodar pipelines complementares (integração end-to-end, testes de contrato, smoke tests em staging).
5. Atualizar versão de cada serviço (`Cargo.toml`) e registrar no `platform/CHANGELOG.md`.
6. Gerar documentação de upgrade (manual + TL;DR) em `docs/releases/platform-AAAA.MM.md`.
7. Executar `tools/release/check-platform.ps1` (clippy, fmt, generate-openapi, verificação de migrations).
8. Aprovação mínima: 2 owners + representante de Infra.
9. Merge na `main`, tag `platform-AAAA.MM`, publicar release notes e atualizar `RELEASES.md`.
10. Disparar roll-out automatizado via pipeline de deploy.
