# Domínio `core`

O domínio **core** concentra as bibliotecas fundamentais que dão suporte a todos os outros
componentes da plataforma AVL. Aqui ficam os pacotes responsáveis por matemática avançada,
processamento colunares, compressão e telemetria compartilhada.

## Escopo

- Estruturas de dados e algoritmos matemáticos (`avila-math`, `avila-linalg`).
- Formatos colunares e serialização de dados científicos (`avila-arrow`).
- Compressão, redução dimensional e pipelines numéricos (`avila-compress`, `avila-reduction`).
- Telemetria, métricas e utilidades de observabilidade (`avila-telemetry`).
- Metapacote `avila` para distribuir conjuntos coesos que combinam os módulos acima.

## APIs estáveis

- Interfaces públicas seguem **SemVer** rigoroso a partir da versão `0.5.0`.
- Funções marcadas como `experimental` podem sofrer alterações entre *minor releases*.
- Recursos internos (módulos `internal::*`) não fazem parte do contrato estável.

## Política de versionamento

- Versões independentes por pacote, coordenadas via `RELEASES.md`.
- Majors planejados semestrais (ex.: `1.0.0` após estabilização de álgebra tensorial).
- Minors quinzenais para evoluções de API.
- Patches sob demanda para correções.

## Publicação

- Registro primário: **crates.io**.
- Releases internas também são replicadas no registro privado `avila` para consumo por outros domínios.

Consulte `core/CHANGELOG.md` para o histórico consolidado e `docs/release-playbooks/core.md`
para o passo a passo de publicação.
