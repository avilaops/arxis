# Playbook de Release — Domínio `geo`

1. Branch `release/geo` criada a partir de `main`.
2. Validar dependências externas (dados, licenças) antes do freeze.
3. Rodar `cargo test --workspace --manifest-path geo/Cargo.toml`.
4. Executar benchmarks críticos (`cargo bench -p geospatial-analysis` quando aplicável).
5. Atualizar versões dos pacotes liberados publicamente (`geo-core`, etc.) e do `geo/CHANGELOG.md`.
6. Executar `tools/release/check-geo.ps1` para garantir integridade de assets e dados auxiliares.
7. Publicar pacotes genéricos em `crates.io` (`cargo publish`) e internos no registry `avila`.
8. Gerar documentação atualizada (`cargo doc --no-deps -p avila-geo`).
9. Obter aprovações do Geospatial Squad.
10. Merge na `main`, criar tag `geo-<pacote>-vX.Y.Z` e atualizar `RELEASES.md`.
