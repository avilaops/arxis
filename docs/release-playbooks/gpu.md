# Playbook de Release — Domínio `gpu`

1. Criar branch `release/gpu` a partir de `main`.
2. Revisar estado dos backends (wgpu, vulkan, cuda, metal, rocm) e selecionar quais entram no ciclo.
3. Atualizar versões nos crates relevantes (`Cargo.toml` dentro de `avx-gpu/*`).
4. Rodar `cargo test --workspace --manifest-path gpu/Cargo.toml` e benchmarks críticos (`cargo bench -p avx-gpu-core`).
5. Validar geração de shaders/artefatos (scripts específicos em `avx-gpu`).
6. Executar `tools/release/check-gpu.ps1` para validar toolchains (Rust + CUDA/Metal SDKs).
7. Publicar pacotes aprovados (`cargo publish` para crates.io ou `cargo publish --registry avila`).
8. Taggear `gpu-<crate>-vX.Y.Z` e atualizar `gpu/CHANGELOG.md`.
9. Atualizar `RELEASES.md` e comunicar squads dependentes (AI, Platform).
