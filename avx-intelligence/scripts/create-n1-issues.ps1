# Criar todas as issues do Notebook 1
# Execute na Machine 1 (AVL-CONTROLLER)

param(
    [switch]$DryRun = $false
)

Write-Host "ARXIS GitHub Issues Creator - Notebook 1" -ForegroundColor Magenta
Write-Host ""

if ($DryRun) {
    Write-Host "MODO DRY RUN - Nenhuma issue sera criada" -ForegroundColor Yellow
    Write-Host ""
}

# Definicao dos 16 modulos do Notebook 1
$modules = @(
    @{name="avila-primitives"; area="foundation"; priority="high"; desc="Tipos primitivos base"},
    @{name="avila-error"; area="foundation"; priority="high"; desc="Sistema de erros"},
    @{name="avila-id"; area="foundation"; priority="high"; desc="Identificadores unicos"},
    @{name="avila-time"; area="foundation"; priority="high"; desc="Tipos temporais"},
    @{name="avila-atom"; area="foundation"; priority="medium"; desc="Atomic operations"},
    @{name="avila-cell"; area="foundation"; priority="medium"; desc="Cell types"},
    @{name="avila-nucleus"; area="foundation"; priority="medium"; desc="Nucleo de dados"},
    @{name="avila-cell-core"; area="foundation"; priority="medium"; desc="Core cell logic"},
    @{name="avila-serde"; area="core-types"; priority="high"; desc="Serializacao nativa"},
    @{name="avila-log"; area="core-types"; priority="high"; desc="Logging system"},
    @{name="avila-future"; area="core-types"; priority="medium"; desc="Async primitives"},
    @{name="avila-rand"; area="core-types"; priority="medium"; desc="Random number gen"},
    @{name="avila-rand-simple"; area="core-types"; priority="low"; desc="Simple RNG"},
    @{name="avila-regex"; area="core-types"; priority="medium"; desc="Pattern matching"},
    @{name="avila-crypto"; area="core-types"; priority="high"; desc="Criptografia"},
    @{name="avila-term"; area="core-types"; priority="low"; desc="Terminal utilities"}
)

function Create-Issue {
    param($Module)

    $title = "[N1] $($Module.name)"

    $body = @"
## Modulo: $($Module.name)

**Notebook**: Notebook 1 - Foundation
**Area**: $($Module.area)
**Prioridade**: $($Module.priority)
**Descricao**: $($Module.desc)

---

### Objetivos

- [ ] Estrutura basica criada (Cargo.toml, src/lib.rs)
- [ ] APIs publicas definidas e documentadas
- [ ] Testes unitarios implementados (>80% coverage)
- [ ] Testes de integracao (se aplicavel)
- [ ] Documentacao inline completa (100% public APIs)
- [ ] README.md com examples
- [ ] Benchmarks implementados (cargo bench)
- [ ] Clippy warnings = 0
- [ ] Publicado em crates.io

---

### Criterios de Aceitacao

**Qualidade**:
- Compila sem warnings
- cargo clippy -- -D warnings passa
- cargo fmt --check passa
- cargo test passa (todos os testes)
- cargo doc --no-deps gera docs completas

**Documentacao**:
- README.md presente com examples
- Todas as APIs publicas documentadas
- Examples em examples/ funcionais
- CHANGELOG.md criado

**Testing**:
- Coverage >80%
- Casos de erro cobertos
- Edge cases testados

---

### Referencias

- Manifesto: /NOTEBOOK1-MANIFESTO.md
- Strategy: /avx-intelligence/MASTER-STRATEGY.md
- Assignments: /avx-intelligence/NOTEBOOK-ASSIGNMENTS.md

---

**Machine**: Machine 3 (ALV-FACTORY)
**Copilot**: Seguir prompts em MACHINE3-PLAN.md
"@

    if ($DryRun) {
        Write-Host "  [DRY-RUN] Issue: $title" -ForegroundColor Cyan
        return
    }

    Write-Host "  Criando: $title" -ForegroundColor Green

    try {
        gh issue create `
            --title $title `
            --body $body `
            --label "notebook-1,area-$($Module.area),priority-$($Module.priority)" `
            2>&1 | Out-Null

        Start-Sleep -Milliseconds 500
    } catch {
        Write-Host "  ERRO ao criar issue: $_" -ForegroundColor Red
    }
}

# Executar
Write-Host "Notebook 1 - Foundation (16 modulos)" -ForegroundColor Cyan
Write-Host ""

foreach ($module in $modules) {
    Create-Issue -Module $module
}

Write-Host ""

if ($DryRun) {
    Write-Host "Preview completo: 16 issues seriam criadas" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "Execute sem -DryRun para criar de verdade:" -ForegroundColor White
    Write-Host "  .\create-n1-issues.ps1" -ForegroundColor Gray
} else {
    Write-Host "16 issues criadas com sucesso!" -ForegroundColor Green
    Write-Host ""
    Write-Host "Verifique em: https://github.com/avilaops/arxis/issues" -ForegroundColor White
}
