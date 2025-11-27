# Playbook de Release — Domínio `core`

1. Abra branch `release/core` a partir de `main`.
2. Garanta que não há PRs pendentes com breaking changes.
3. Execute `cargo fmt`, `cargo clippy --all-targets` e `cargo test --workspace` no domínio via `cargo --manifest-path core/Cargo.toml`.
4. Atualize as versões desejadas em cada crate (`Cargo.toml`) e reflita no `core/CHANGELOG.md`.
5. Rode `tools/release/check-core.ps1` para validações automáticas.
6. Faça `cargo publish --dry-run` por crate destinada ao crates.io.
7. Solicite duas aprovações (Owner core + cross-domain).
8. Faça merge na `main` e gere a tag `core-<crate>-vX.Y.Z`.
9. Atualize `RELEASES.md` e publique as notas no GitHub Releases.
