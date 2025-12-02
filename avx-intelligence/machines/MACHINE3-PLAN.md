# 🏭 MACHINE 3 - ALV-FACTORY

## 🎯 Papel Estratégico
**Code Factory** - Produção massiva com múltiplos copilots paralelos

---

## 📋 Setup Inicial (Segunda-feira)

### 1. Capturar Logs
```powershell
cd d:\arxis\avx-intelligence\scripts
.\capture-logs.ps1 -All
```

### 2. VS Code Multi-Instance Setup
```powershell
# Abrir 3 VS Codes em workspaces diferentes
code d:\arxis\avila-ai-workspace
code d:\arxis\avila-core-workspace
code d:\arxis\avila-geo-workspace

# Cada VS Code = 1 Notebook ativo
```

### 3. Copilot Configuration
```json
// settings.json (cada VS Code)
{
  "github.copilot.enable": {
    "*": true,
    "rust": true
  },
  "github.copilot.advanced": {
    "debug.overrideEngine": "gpt-4",
    "inlineSuggest.enable": true
  },
  "editor.inlineSuggest.enabled": true,
  "editor.quickSuggestions": {
    "other": true,
    "comments": true,
    "strings": true
  }
}
```

---

## 🔄 Organização por VS Code

### VS Code 1 - Notebook 1 (Fundação)
**Workspace**: `avila-core-workspace`
**Status**: 🔴 ATIVO AGORA
**Prazo**: 31/Dez/2025

#### Semana 1 (02-08 Dez)
**Área 1 - Primitivos** (4 módulos):
- [ ] avila-primitives
- [ ] avila-error
- [ ] avila-id
- [ ] avila-time

**Copilot Prompts**:
```
@workspace Crie o módulo avila-primitives seguindo o padrão ARXIS.
Requisitos:
- Zero dependências externas
- Traits: Primitive, Copy, Clone
- 100% docs inline
- Testes completos
- Benchmarks básicos

Referência: /NOTEBOOK1-MANIFESTO.md
```

#### Semana 2 (09-15 Dez)
**Área 1 - Primitivos** (4 módulos restantes):
- [ ] avila-atom
- [ ] avila-cell
- [ ] avila-nucleus
- [ ] avila-cell-core

---

### VS Code 2 - Notebook 1 (Fundação)
**Workspace**: `avila-ai-workspace`
**Status**: 🔴 ATIVO AGORA
**Prazo**: 31/Dez/2025

#### Semana 1 (02-08 Dez)
**Área 2 - Core Types** (4 módulos):
- [ ] avila-serde
- [ ] avila-log
- [ ] avila-future
- [ ] avila-rand

**Copilot Prompts**:
```
@workspace Crie o módulo avila-serde para serialização nativa.
Requisitos:
- Traits próprios (não usar serde crate)
- Suporte JSON + binário
- Zero-copy quando possível
- Comparar performance vs serde
- Examples práticos

Referência: /NOTEBOOK1-MANIFESTO.md, /avx-intelligence/MASTER-STRATEGY.md
```

#### Semana 2 (09-15 Dez)
**Área 2 - Core Types** (4 módulos restantes):
- [ ] avila-rand-simple
- [ ] avila-regex
- [ ] avila-crypto
- [ ] avila-term

---

### VS Code 3 - Notebook 3 (Data/ML)
**Workspace**: `avila-geo-workspace`
**Status**: ⏸️ AGUARDAR (Iniciar ~15/Jan/2026)
**Prazo**: 28/Fev/2026

#### Quando Liberado
**Área 1 - Data Science**:
- avila-dataframe
- avila-clustering ⭐ (NASA priority)
- avila-reduction ⭐ (NASA priority)
- avila-telemetry
- avila-geo
- avila-image
- avila-vision
- avila-tokenizer ⭐ (NASA priority)

**Critérios NASA**:
- Benchmarks reproduzíveis
- Datasets científicos públicos
- Documentação formal
- Examples astrofísicos

---

## 🎯 Workflow de Produção

### 1. Preparação (15min)
```powershell
# Ler manifesto do notebook
code NOTEBOOK1-MANIFESTO.md

# Ler strategy
code avx-intelligence\MASTER-STRATEGY.md

# Criar branch
git checkout -b feat/avila-primitives
```

### 2. Desenvolvimento com Copilot (2-3h/módulo)
1. **Criar estrutura**:
   ```bash
   cargo new --lib avila-primitives
   cd avila-primitives
   ```

2. **Prompt Copilot**:
   - Abrir `lib.rs`
   - Escrever comentário do que precisa
   - Deixar Copilot gerar código base
   - Refinar e ajustar

3. **Iteração**:
   - Implementar APIs públicas
   - Adicionar testes
   - Escrever docs
   - Criar examples

### 3. Validação Local (30min)
```powershell
# Compilar
cargo build

# Testes
cargo test

# Clippy
cargo clippy -- -D warnings

# Fmt
cargo fmt --check

# Docs
cargo doc --no-deps --open
```

### 4. Commit & Push (10min)
```powershell
git add .
git commit -m "feat(avila-primitives): implementa tipos primitivos base

- Define traits Primitive, Copy, Clone
- Adiciona 20+ testes
- Documentação completa
- Benchmarks estabelecidos

Refs: #1"

git push origin feat/avila-primitives
```

### 5. Criar PR (5min)
```powershell
gh pr create `
  --title "feat: avila-primitives - Tipos primitivos base" `
  --body "Implementa módulo conforme Notebook 1 manifesto. Closes #1" `
  --label "notebook-1,area-foundation"
```

### 6. Exportar Conversa Copilot (5min)
```
Ctrl+Shift+P > "Export Chat History"
Salvar em: logs/copilots/machine-3/vscode-1/chat-avila-primitives-{timestamp}.json
```

---

## 📊 Métricas de Produtividade

### Por Módulo
- **Tempo estimado**: 3-4 horas
- **Linhas de código**: 500-1500
- **Testes**: 10-30
- **Docs**: 100% APIs públicas
- **Examples**: 2-5

### Por Dia (8h trabalho)
- **Módulos completos**: 2-3
- **Commits**: 8-12
- **PRs**: 2-3
- **Linhas/hora**: 200-400

### Por Semana
- **Módulos**: 10-15
- **Crates publicados**: 4-8
- **Issues fechadas**: 10-15
- **Total linhas**: 8k-12k

---

## 🤖 Prompts Mestres por Tipo de Módulo

### Tipo 1: Primitivos
```
Crie o módulo {CRATE_NAME} para tipos primitivos do ecossistema ARXIS.

Contexto: Fundação zero-dependency, usado por todos os outros 81 módulos.

Estrutura necessária:
1. Traits principais (Primitive, Copy, Clone, etc.)
2. Implementations para tipos Rust std
3. Macros úteis se aplicável
4. Testes unitários extensivos
5. Benchmarks de performance
6. Examples práticos
7. README.md completo

Qualidade:
- Zero unsafe code
- 100% docs inline com exemplos
- Clippy compliant
- Semver desde v0.1.0

Referências:
- /NOTEBOOK1-MANIFESTO.md
- /avx-intelligence/MASTER-STRATEGY.md

Gerar código agora.
```

### Tipo 2: Sistema (Error, Log, etc.)
```
Crie o módulo {CRATE_NAME} para o sistema {CATEGORY} do ARXIS.

Requisitos específicos:
- API idiomática Rust
- Integração com std lib
- Performance otimizada
- Docs com use cases

Implementar:
1. Tipos principais
2. Traits/interfaces
3. Macros de conveniência
4. Conversões From/Into
5. Display/Debug
6. Serialization support (via avila-serde)

Testes devem cobrir:
- Happy paths
- Error cases
- Edge cases
- Integration examples

Gerar estrutura completa.
```

### Tipo 3: Matemática/Científico
```
Crie o módulo {CRATE_NAME} para computação {científica/matemática} de alto desempenho.

Meta NASA: Benchmarks reproduzíveis e documentação formal.

Implementar:
1. Tipos numéricos/estruturas de dados
2. Algoritmos core (com referências a papers)
3. SIMD optimizations onde aplicável
4. Testes com datasets conhecidos
5. Benchmarks vs bibliotecas estabelecidas
6. Documentação matemática (LaTeX inline)

Performance targets:
- Comparável ou superior a alternativas Python/C++
- Memory efficient
- Cache friendly

Examples devem incluir:
- Caso de uso científico real
- Visualização de resultados
- Comparação de performance

Gerar implementação production-ready.
```

---

## 📅 Rotina Diária

### 8:00 - 9:00: Setup
- [ ] Review issues do dia
- [ ] Ler manifestos relevantes
- [ ] Preparar prompts
- [ ] Criar branches

### 9:00 - 12:00: Produção Manhã
- [ ] Desenvolver 1-2 módulos
- [ ] Testes inline
- [ ] Docs inline
- [ ] Commits frequentes

### 12:00 - 13:00: Almoço + Review
- [ ] Code review mental
- [ ] Sync com Machine 1 (Notebook 6)
- [ ] Capturar logs

### 13:00 - 17:00: Produção Tarde
- [ ] Desenvolver 1-2 módulos
- [ ] Finalizar pendências
- [ ] Criar PRs
- [ ] Examples e polish

### 17:00 - 18:00: Wrap-up
- [ ] Exportar conversas Copilot
- [ ] Update status issues
- [ ] Push final
- [ ] Preparar próximo dia

---

## 🎯 KPIs Machine 3

### Diários
- Módulos iniciados: 2-3
- Módulos finalizados: 1-2
- PRs criados: 2-3
- Conversas Copilot exportadas: 2-3

### Semanais
- Módulos completos: 8-12
- Crates publicados: 4-8
- Linhas produzidas: 8k-12k
- Issues fechadas: 8-12

### Mensais
- Notebooks completos: 1
- Qualidade mantida: 100%
- Copilot efficiency: >0.8 (IA score)

---

**Machine Owner**: Factory Manager
**VS Codes**: 3 (Notebooks 1, 1, 3)
**Role**: Mass Production
