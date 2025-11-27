# Domínio `gpu`

O domínio **gpu** agrupa toda a stack de computação acelerada da AVL — compiladores,
runtimes, bibliotecas e backends específicos para GPUs, incluindo integrações com renderização quântica.

## Escopo

- `avx-gpu`: workspace com `core`, `runtime`, `std`, `macros` e backends (wgpu, vulkan, cuda, metal, rocm).
- `avx-quantum-render`: motor experimental de renderização quântica/óptica com integração direta ao pipeline GPU.

## APIs estáveis

- `avx-gpu-core` e `avx-gpu-runtime` definem as APIs públicas estáveis.
- Backends marcados como `experimental` podem quebrar entre versões.
- Benchmarks e exemplos não fazem parte da superfície oficial.

## Política de versionamento

- Releases internos quinzenais (`gpu-core 0.x` até estabilização).
- Publicação em **crates.io** começa pelos núcleos (`core` e `runtime`) quando atingirem `1.0`.
- Backends proprietários permanecem no registry privado `avila` até revisão contratual.

## Publicação

- Registro padrão configurado para `avila`.
- Crates prontos para comunidade devem definir `publish = ["crates-io"]` em seus manifestos individuais.

Detalhes de mudanças: `gpu/CHANGELOG.md`.
Passos de release: `docs/release-playbooks/gpu.md`.
