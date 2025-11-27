# Domínio `ai`

O domínio **ai** reúne todas as bibliotecas e utilidades de Inteligência Artificial, ML e NLP
mantidas pela AVL Platform.

## Escopo

- `avila-ml`: núcleo de autograd, redes neurais e operadores diferenciáveis.
- `avila-clustering`: algoritmos de agrupamento (HDBSCAN, Birch, etc.).
- `avila-dataframe`: manipulação de dados tabulares com foco em workloads científicos.
- `avila-tokenizer`: tokenização de texto e utilidades de NLP.

## APIs estáveis

- APIs rotuladas como `stable` permanecem compatíveis dentro do mesmo *major*.
- Componentes marcados como `experimental` ou com o feature flag `nightly` podem quebrar entre versões.

## Política de versionamento

- `ai-suite` coordena lançamentos multiplos; jede crate mantém SemVer individual.
- Majors alinhados a entregas trimestrais.
- Minors por sprint (duas semanas) para novas features.
- Correções críticas liberadas via patch sob demanda.

## Publicação

- Registro padrão: **crates.io** (com documentação gerada em docs.rs).
- Artefatos também são espelhados no registro `avila` para dependências internas.

Consulte `ai/CHANGELOG.md` para o histórico consolidado e `docs/release-playbooks/ai.md`
para o ritual de publicação.
