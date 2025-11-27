# Domínio `geo`

O domínio **geo** cobre soluções de inteligência geográfica, análise de localização e
visualização cartográfica fornecidas pela AVL Platform.

## Escopo

- `avila-geo`: biblioteca base de cartografia e projeções.
- `avila-location`: motor de análise de potencial de mercado e seleção de pontos.
- `geospatial-analysis`: pipelines de geoprocessamento de alta performance.
- `avx-image`: renderização avançada (incluindo *face3d*, *physics-recognition* e outros projetos derivados).
- `data-extraction`: coletores e ETLs para fontes geo-comerciais.
- `financial-optimization`: modelos geo-financeiros.

## APIs estáveis

- APIs com `stable` no documento público mantêm compatibilidade durante um *major*.
- Componentes de projetos específicos (ex.: `face3d`) permanecem `internal` até maturação.

## Política de versionamento

- Bibliotecas genéricas publicadas com SemVer clássico.
- Projetos customizados mantêm trilha interna `0.x` para rastrear evolução.
- Futuro pacote `geo-suite` consolidará entregas maduras.

## Publicação

- Registro padrão: **crates.io** para bibliotecas reutilizáveis.
- Artefatos sob NDA permanecem no registro privado `avila`.

Histórico consolidado: `geo/CHANGELOG.md`.
Guias operacionais: `docs/release-playbooks/geo.md`.
