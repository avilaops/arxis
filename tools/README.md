# Domínio `tools`

O domínio **tools** reúne utilitários de linha de comando, scripts e automações que
suportam o ciclo de desenvolvimento da AVL Platform.

## Escopo

- `tools/xtask`: checklist automatizado e inspeções de qualidade.
- `examples/practical-cli`: coleções de CLIs demonstrativas e ferramentas internas.
- Scripts auxiliares de release localizados em `scripts/release`.

## APIs estáveis

- Em geral são binários internos; a superfície pública é definida por documentação específica.
- Quando uma ferramenta for aberta para a comunidade, a interface será tratada como estável e versionada via SemVer.

## Política de versionamento

- Cada utilitário possui *build number* independente.
- Releases coordenadas apenas quando há dependência entre ferramentas.
- Ferramentas internas mantêm versão espelhada no `RELEASES.md` para rastreabilidade.

## Publicação

- Registro padrão: `avila` (privado).
- Publicação em `crates.io` ocorre apenas mediante aprovação do comitê de produto.

Para orientações de release consulte `docs/release-playbooks/tools.md`.
