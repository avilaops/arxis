# Domínio `platform`

O domínio **platform** consolida os serviços centrais da AVL Cloud Platform — autenticação,
filas, storage, gateway HTTP e componentes de observabilidade.

## Escopo

- Serviços `avl-*`: auth, queue, secrets, storage, observability, load balancer.
- Serviços `avx-*`: API core, gateway, eventos, telemetry e CLI.
- `aviladb`: banco de dados distribuído e vetorial.

## APIs estáveis

- APIs documentadas no portal de desenvolvedores (`avl-console`) seguem **SemVer** para clientes externos.
- Endpoints rotulados como `internal` podem mudar a qualquer momento.
- Cada serviço publica *OpenAPI* ou especificação gRPC atualizada a cada release train.

## Política de versionamento

- Release train mensal (`Platform Release AAAA.MM`).
- Tags individuais por serviço seguem `major.minor.patch`.
- Hotfixes geram patches fora do ciclo regular, com changelog dedicado.

## Publicação

- Registro padrão: **avila** (privado).
- Crates destinados a público externo ganham *alias* `avila-*-sdk` e são publicados em `crates.io` sob revisão executiva.

Histórico consolidado: `platform/CHANGELOG.md`.
Playbook e responsabilidades: `docs/release-playbooks/platform.md`.
