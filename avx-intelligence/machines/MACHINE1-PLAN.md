# 🖥️ MACHINE 1 - AVL-CONTROLLER

## 🎯 Papel Estratégico
**Coordenador Central** - Orquestração, CI/CD, Analytics e Notebook 6

---

## 📋 Setup Inicial (Segunda-feira)

### 1. Capturar Logs
```powershell
cd d:\arxis\avx-intelligence\scripts
.\capture-logs.ps1 -All
```

### 2. GitHub Project Setup
```powershell
# Instalar GitHub CLI (se necessário)
winget install GitHub.cli

# Login
gh auth login

# Criar issues para os 82 módulos
cd d:\arxis
gh issue create --title "[N1] avila-primitives - Tipos primitivos base" --label "notebook-1,priority-high,area-foundation"
gh issue create --title "[N1] avila-error - Sistema de erros" --label "notebook-1,priority-high,area-foundation"
# ... (script automatizado abaixo)
```

### 3. CI/CD Pipeline
```yaml
# .github/workflows/ci.yml
name: ARXIS CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo test --all
      - run: cargo clippy -- -D warnings
      - run: cargo fmt -- --check
```

---

## 🔄 Tarefas Contínuas

### Daily (8h, 12h, 18h)
```powershell
# Atualizar dashboard
cd d:\arxis\avx-intelligence
.\scripts\update-dashboard.ps1

# Sincronizar com outras máquinas
.\scripts\sync-machines.ps1 -Bidirectional

# Capturar métricas
.\scripts\capture-logs.ps1 -MetricsLogs
```

### Weekly (Sextas-feiras)
```powershell
# Gerar report
.\scripts\generate-weekly-report.ps1

# Backup logs
.\scripts\backup-logs.ps1

# Update strategy docs
# Editar: MASTER-STRATEGY.md, NOTEBOOK-ASSIGNMENTS.md
```

---

## 🎯 Notebook 6 - Responsabilidades

### Issue Management
**Template de Issue**:
```markdown
## Módulo: {CRATE_NAME}

**Notebook**: {1-5}
**Área**: {Foundation/Math/ML/Infra/Advanced}
**Prioridade**: {High/Medium/Low}
**Bloqueador**: {None / Aguardar X}

### Objetivos
- [ ] Estrutura básica criada
- [ ] APIs públicas definidas
- [ ] Testes implementados
- [ ] Docs completas
- [ ] Benchmarks (se aplicável)
- [ ] Publicado em crates.io

### Critérios de Aceitação
- Zero Clippy warnings
- 100% doc coverage
- Todos os testes passando
- README.md presente

### Dependências
- Depende de: {lista}
- Bloqueia: {lista}
```

### PR Review Checklist
- [ ] Código compila sem warnings
- [ ] Testes passam
- [ ] Docs atualizadas
- [ ] CHANGELOG.md atualizado
- [ ] Semver correto
- [ ] No breaking changes sem justificativa

---

## 📊 Analytics & Monitoring

### Dashboard Grafana (Local)
```powershell
# Instalar Grafana
winget install GrafanaLabs.Grafana

# Start service
net start grafana

# Acessar: http://localhost:3000
```

### Métricas a Monitorar
1. **Produtividade**
   - Commits/dia por notebook
   - Linhas adicionadas/removidas
   - Issues resolvidas/criadas
   - PRs merged/abertas

2. **Qualidade**
   - Test coverage por crate
   - Clippy warnings count
   - Doc coverage %
   - Build times

3. **Performance**
   - Benchmark results trend
   - Compilation times
   - CI/CD duration

4. **Copilots**
   - Instruction adherence score
   - Code correctness rate
   - Novelty vs consistency
   - Latency distributions

---

## 🔧 Ferramentas Instaladas

### Rust Tooling
```powershell
rustup update stable
rustup component add clippy rustfmt
cargo install cargo-nextest cargo-audit cargo-outdated
cargo install cargo-make cargo-watch
```

### Analytics Tools
```powershell
# Python para avx-inspector
python -m venv .venv
.venv\Scripts\Activate
pip install sentence-transformers hdbscan scikit-learn pandas fastapi
```

### Monitoring
```powershell
# Instalar Prometheus + Grafana
# (via Docker ou native Windows)
```

---

## 🚀 Scripts de Automação

### create-all-issues.ps1
```powershell
# Criar todas as 82 issues automaticamente
$notebooks = @(
    @{name="N1"; modules=@("avila-primitives","avila-error",...); label="notebook-1"},
    @{name="N2"; modules=@(...); label="notebook-2"},
    # ...
)

foreach ($nb in $notebooks) {
    foreach ($mod in $nb.modules) {
        gh issue create `
            --title "[$($nb.name)] $mod" `
            --label "$($nb.label),area-foundation" `
            --body "Implementar módulo $mod conforme manifesto"
    }
}
```

### update-dashboard.ps1
```powershell
# Atualizar métricas no dashboard
$metrics = @{}

# Contar issues por status
$metrics.open = (gh issue list --label notebook-1 --state open | Measure-Object).Count
$metrics.closed = (gh issue list --label notebook-1 --state closed | Measure-Object).Count

# Commits hoje
$metrics.commits_today = (git log --since="1 day ago" --oneline | Measure-Object).Count

# Salvar em JSON
$metrics | ConvertTo-Json | Out-File "logs\machines\avl-controller\dashboard-$(Get-Date -Format 'yyyyMMdd').json"
```

---

## 📅 Agenda Semanal

### Segunda
- 8:00 - Criar issues da semana
- 12:00 - Review PRs pendentes
- 18:00 - Status check notebooks

### Terça-Sexta
- 8:00 - Daily standup (async via issues)
- 12:00 - Review + merge PRs
- 15:00 - Analytics review
- 18:00 - Sync logs entre máquinas

### Sexta (Extra)
- 16:00 - Weekly report
- 17:00 - Planning próxima semana
- 18:00 - Backup e cleanup

---

## 🎯 KPIs Machine 1

### Diários
- Issues processadas: >5/dia
- PRs reviewed: >3/dia
- CI/CD runs: 100% green
- Logs sincronizados: 3x/dia

### Semanais
- Issues criadas: 16 (1 notebook/semana)
- Release published: 1
- Dashboard updated: 7 dias
- Report gerado: 1

---

**Machine Owner**: Coordenador Principal
**VS Code Setup**: 1 instância (Notebook 6)
**Role**: Command & Control
