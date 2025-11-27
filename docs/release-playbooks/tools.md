# Playbook de Release — Domínio `tools`

1. Criar branch `release/tools` a partir de `main`.
2. Definir quais CLIs ou automações entrarão no pacote.
3. Atualizar versões nos manifestos (`tools/xtask`, `examples/practical-cli`, etc.).
4. Rodar `cargo test --workspace --manifest-path tools/Cargo.toml` e validar utilitários críticos manualmente.
5. Executar `tools/release/check-tools.ps1` (lint, testes de integração, validação de help `--help`).
6. Atualizar `tools/CHANGELOG.md` e `RELEASES.md`.
7. Avaliar se algum binário deve ser publicado via `cargo install` (`cargo publish` quando aplicável).
8. Obter aprovação do time DevProd.
9. Merge na `main`, criar tag `tools-<nome>-vX.Y.Z` e publicar notas.
