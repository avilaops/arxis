# 📋 Template de Issues - Arxis Project

## Template: Novo Módulo

```markdown
## 📦 [Nome do Módulo]

**Notebook:** [1-6]
**Área:** [1-2]
**Prioridade:** [Alta/Média/Baixa]
**Depende de:** [Lista de módulos ou "Nenhum"]

### 🎯 Objetivo
[Descrição sucinta do propósito do módulo]

### ✅ Checklist de Implementação

#### Setup
- [ ] Criar estrutura de diretórios
- [ ] Criar `Cargo.toml` com metadados corretos
- [ ] Criar `README.md`
- [ ] Criar `src/lib.rs` com documentação

#### Core
- [ ] Implementar tipos principais
- [ ] Implementar traits necessários
- [ ] Implementar funções públicas
- [ ] Adicionar validações

#### Testes
- [ ] Testes unitários (cobertura > 80%)
- [ ] Testes de integração
- [ ] Doctests em exemplos
- [ ] Benchmarks (se aplicável)

#### Qualidade
- [ ] `cargo fmt` sem erros
- [ ] `cargo clippy` sem warnings
- [ ] `cargo doc` sem warnings
- [ ] CI passando

#### Finalização
- [ ] Exemplos em `examples/`
- [ ] Documentação completa
- [ ] Atualizar dependências em outros módulos
- [ ] Publicar em crates.io

### 📝 Notas
[Considerações técnicas, decisões de design, etc.]

### 🔗 Relacionado
- Relacionado com: #[número]
- Bloqueia: #[número]
- Bloqueado por: #[número]
```

---

## Labels Padrão

### Por Notebook
- `notebook-1-fundacao` 🔹
- `notebook-2-matematica` ➕
- `notebook-3-data-ml` 📊
- `notebook-4-database` 💾
- `notebook-5-advanced` 🚀
- `notebook-6-coordenacao` 🎯

### Por Tipo
- `feature` - Nova funcionalidade
- `bug` - Correção de bug
- `documentation` - Documentação
- `refactor` - Refatoração
- `test` - Testes
- `ci-cd` - Integração contínua
- `dependencies` - Gestão de dependências

### Por Prioridade
- `priority-critical` ⚡ - Bloqueia outros módulos
- `priority-high` 🔴 - Importante
- `priority-medium` 🟡 - Normal
- `priority-low` 🟢 - Pode esperar

### Por Status
- `status-blocked` - Aguardando dependências
- `status-in-progress` - Em desenvolvimento
- `status-review` - Aguardando review
- `status-ready` - Pronto para merge

---

## Workflow de Issues

1. **Criação:** Notebook 6 cria issue usando template
2. **Assign:** Desenvolvedores se atribuem
3. **Development:** Branch `feature/module-name`
4. **PR:** Pull request referenciando issue
5. **CI:** Testes automatizados
6. **Review:** Code review
7. **Merge:** Merge para `develop`
8. **Release:** Periodic releases para `main`
