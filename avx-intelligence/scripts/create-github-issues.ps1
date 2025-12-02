# Script para criar todas as 82 issues do projeto ARXIS
# Execute na Machine 1 (AVL-CONTROLLER)

param(
    [switch]$DryRun = $false,
    [string]$Notebook = "all"  # all, n1, n2, n3, n4, n5
)

# Definição completa dos 82 módulos
$notebooks = @{
    "n1" = @{
        name = "Notebook 1 - Foundation"
        label = "notebook-1"
        modules = @(
            @{name="avila-primitives"; area="foundation"; priority="high"; desc="Tipos primitivos base"},
            @{name="avila-error"; area="foundation"; priority="high"; desc="Sistema de erros"},
            @{name="avila-id"; area="foundation"; priority="high"; desc="Identificadores únicos"},
            @{name="avila-time"; area="foundation"; priority="high"; desc="Tipos temporais"},
            @{name="avila-atom"; area="foundation"; priority="medium"; desc="Atomic operations"},
            @{name="avila-cell"; area="foundation"; priority="medium"; desc="Cell types"},
            @{name="avila-nucleus"; area="foundation"; priority="medium"; desc="Núcleo de dados"},
            @{name="avila-cell-core"; area="foundation"; priority="medium"; desc="Core cell logic"},
            @{name="avila-serde"; area="core-types"; priority="high"; desc="Serialização nativa"},
            @{name="avila-log"; area="core-types"; priority="high"; desc="Logging system"},
            @{name="avila-future"; area="core-types"; priority="medium"; desc="Async primitives"},
            @{name="avila-rand"; area="core-types"; priority="medium"; desc="Random number gen"},
            @{name="avila-rand-simple"; area="core-types"; priority="low"; desc="Simple RNG"},
            @{name="avila-regex"; area="core-types"; priority="medium"; desc="Pattern matching"},
            @{name="avila-crypto"; area="core-types"; priority="high"; desc="Criptografia"},
            @{name="avila-term"; area="core-types"; priority="low"; desc="Terminal utilities"}
        )
    }
    "n2" = @{
        name = "Notebook 2 - Mathematics"
        label = "notebook-2"
        modules = @(
            @{name="avila-math"; area="mathematics"; priority="high"; desc="Math fundamentals"},
            @{name="avila-numeric"; area="mathematics"; priority="high"; desc="Numeric types"},
            @{name="avila-linalg"; area="mathematics"; priority="high"; desc="Linear algebra"},
            @{name="avila-ndarray"; area="mathematics"; priority="high"; desc="N-dimensional arrays"},
            @{name="avila-equation"; area="mathematics"; priority="medium"; desc="Equation solver"},
            @{name="avila-calculus"; area="mathematics"; priority="medium"; desc="Calculus operations"},
            @{name="avila-lapack"; area="mathematics"; priority="medium"; desc="LAPACK bindings"},
            @{name="avila-blas"; area="mathematics"; priority="medium"; desc="BLAS interface"},
            @{name="avila-parallel"; area="parallel"; priority="high"; desc="Parallel computing"},
            @{name="avila-rayon-lite"; area="parallel"; priority="medium"; desc="Lightweight parallelism"},
            @{name="avila-simd"; area="parallel"; priority="high"; desc="SIMD operations"},
            @{name="avila-dsp"; area="parallel"; priority="medium"; desc="Digital signal processing"},
            @{name="avila-fft"; area="parallel"; priority="medium"; desc="Fast Fourier Transform"},
            @{name="avila-wavelet"; area="parallel"; priority="low"; desc="Wavelet transforms"},
            @{name="avila-worker"; area="parallel"; priority="medium"; desc="Worker pool"},
            @{name="avila-scheduler"; area="parallel"; priority="medium"; desc="Task scheduling"}
        )
    }
    "n3" = @{
        name = "Notebook 3 - Data & ML"
        label = "notebook-3"
        modules = @(
            @{name="avila-dataframe"; area="data-science"; priority="high"; desc="DataFrames"},
            @{name="avila-clustering"; area="data-science"; priority="high"; desc="Clustering algorithms ⭐NASA"},
            @{name="avila-reduction"; area="data-science"; priority="high"; desc="Dimensionality reduction ⭐NASA"},
            @{name="avila-telemetry"; area="data-science"; priority="medium"; desc="Telemetry analysis"},
            @{name="avila-geo"; area="data-science"; priority="medium"; desc="Geospatial data"},
            @{name="avila-image"; area="data-science"; priority="high"; desc="Image processing"},
            @{name="avila-vision"; area="data-science"; priority="high"; desc="Computer vision"},
            @{name="avila-tokenizer"; area="data-science"; priority="high"; desc="Text tokenization ⭐NASA"},
            @{name="avila-text"; area="ml"; priority="medium"; desc="Text processing"},
            @{name="avila-embeddings"; area="ml"; priority="high"; desc="Vector embeddings"},
            @{name="avila-vector-db"; area="ml"; priority="high"; desc="Vector database"},
            @{name="avila-search"; area="ml"; priority="medium"; desc="Search engine"},
            @{name="avila-rag"; area="ml"; priority="high"; desc="RAG pipeline"},
            @{name="avila-rerank"; area="ml"; priority="medium"; desc="Re-ranking"},
            @{name="avila-semantic"; area="ml"; priority="high"; desc="Semantic analysis"},
            @{name="avila-agent"; area="ml"; priority="high"; desc="AI agent framework"}
        )
    }
    "n4" = @{
        name = "Notebook 4 - Database & Cloud"
        label = "notebook-4"
        modules = @(
            @{name="avila-db"; area="database"; priority="high"; desc="AvilaDB core ⭐Flagship"},
            @{name="avila-wal"; area="database"; priority="high"; desc="Write-ahead log"},
            @{name="avila-index"; area="database"; priority="high"; desc="Indexing system"},
            @{name="avila-query"; area="database"; priority="high"; desc="Query engine"},
            @{name="avila-sql"; area="database"; priority="medium"; desc="SQL parser"},
            @{name="avila-storage"; area="database"; priority="high"; desc="Storage engine"},
            @{name="avila-cache"; area="database"; priority="high"; desc="Caching layer"},
            @{name="avila-replication"; area="database"; priority="medium"; desc="Replication"},
            @{name="avl-core"; area="cloud"; priority="high"; desc="AVL platform core"},
            @{name="avl-api"; area="cloud"; priority="high"; desc="AVL API"},
            @{name="avl-auth"; area="cloud"; priority="high"; desc="Authentication"},
            @{name="avl-registry"; area="cloud"; priority="medium"; desc="Service registry"},
            @{name="avx-gateway"; area="cloud"; priority="high"; desc="AVX Gateway"},
            @{name="avx-router"; area="cloud"; priority="medium"; desc="Request router"},
            @{name="avx-proxy"; area="cloud"; priority="medium"; desc="Proxy service"},
            @{name="avx-loadbalancer"; area="cloud"; priority="medium"; desc="Load balancer"}
        )
    }
    "n5" = @{
        name = "Notebook 5 - Advanced"
        label = "notebook-5"
        modules = @(
            @{name="avx-gpu"; area="gpu"; priority="high"; desc="GPU computing"},
            @{name="avx-quantum-render"; area="gpu"; priority="high"; desc="Quantum renderer ⭐Paper"},
            @{name="avx-compute"; area="gpu"; priority="medium"; desc="Compute shaders"},
            @{name="avx-graphics"; area="gpu"; priority="medium"; desc="Graphics pipeline"},
            @{name="avx-inference"; area="gpu"; priority="high"; desc="ML inference"},
            @{name="avx-cuda-bridge"; area="gpu"; priority="low"; desc="CUDA interface"},
            @{name="avx-vulkan"; area="gpu"; priority="medium"; desc="Vulkan backend"},
            @{name="avx-metal"; area="gpu"; priority="low"; desc="Metal backend"},
            @{name="avx-mcp"; area="ai"; priority="high"; desc="MCP server ⭐Copilot"},
            @{name="avx-copilot-ai"; area="ai"; priority="high"; desc="Copilot AI"},
            @{name="avx-tools"; area="ai"; priority="medium"; desc="AI tools"},
            @{name="avx-semantic-kernel"; area="ai"; priority="high"; desc="Semantic Kernel"},
            @{name="avx-llm-client"; area="ai"; priority="high"; desc="LLM client"},
            @{name="avx-prompt"; area="ai"; priority="medium"; desc="Prompt engineering"},
            @{name="avx-chains"; area="ai"; priority="medium"; desc="LLM chains"},
            @{name="avx-eval"; area="ai"; priority="medium"; desc="Model evaluation"}
        )
    }
}

function Create-Issue {
    param($Module, $NotebookLabel, $NotebookName)

    $title = "[$($NotebookLabel.ToUpper())] $($Module.name)"

    $body = @"
## 📦 Módulo: $($Module.name)

**Notebook**: $NotebookName
**Área**: $($Module.area)
**Prioridade**: $($Module.priority)
**Descrição**: $($Module.desc)

---

### 🎯 Objetivos

- [ ] Estrutura básica criada (Cargo.toml, src/lib.rs)
- [ ] APIs públicas definidas e documentadas
- [ ] Testes unitários implementados (>80% coverage)
- [ ] Testes de integração (se aplicável)
- [ ] Documentação inline completa (100% public APIs)
- [ ] README.md com examples
- [ ] Benchmarks implementados (cargo bench)
- [ ] Clippy warnings = 0
- [ ] Publicado em crates.io

---

### ✅ Critérios de Aceitação

**Qualidade**:
- ✅ Compila sem warnings
- ✅ \`cargo clippy -- -D warnings\` passa
- ✅ \`cargo fmt --check\` passa
- ✅ \`cargo test\` passa (todos os testes)
- ✅ \`cargo doc --no-deps\` gera docs completas

**Documentação**:
- ✅ README.md presente com examples
- ✅ Todas as APIs públicas documentadas
- ✅ Examples em \`examples/\` funcionais
- ✅ CHANGELOG.md criado

**Testing**:
- ✅ Coverage >80%
- ✅ Casos de erro cobertos
- ✅ Edge cases testados

---

### 📚 Referências

- Manifesto: \`/$NotebookLabel-MANIFESTO.md\`
- Strategy: \`/avx-intelligence/MASTER-STRATEGY.md\`
- Assignments: \`/avx-intelligence/NOTEBOOK-ASSIGNMENTS.md\`

---

### 🔗 Dependências

Veja o manifesto do notebook para dependências específicas.

---

**Machine**: Verificar assignments
**Copilot**: Seguir prompts em MACHINE3-PLAN.md
"@

    if ($DryRun) {
        Write-Host "  [DRY-RUN] Issue: $title" -ForegroundColor Cyan
        return
    }

    Write-Host "  Creating: $title" -ForegroundColor Green

    gh issue create `
        --title $title `
        --body $body `
        --label "$NotebookLabel,area-$($Module.area),priority-$($Module.priority)" `
        2>&1 | Out-Null

    Start-Sleep -Milliseconds 500  # Rate limiting
}

# Main execution
Write-Host "🚀 ARXIS GitHub Issues Creator" -ForegroundColor Magenta
Write-Host ""

if ($DryRun) {
    Write-Host "⚠️  DRY RUN MODE - Nenhuma issue será criada" -ForegroundColor Yellow
    Write-Host ""
}

$notebooksToProcess = if ($Notebook -eq "all") {
    @("n1", "n2", "n3", "n4", "n5")
} else {
    @($Notebook.ToLower())
}

foreach ($nbKey in $notebooksToProcess) {
    $nb = $notebooks[$nbKey]
    Write-Host "📘 $($nb.name) ($($nb.modules.Count) módulos)" -ForegroundColor Cyan
    Write-Host ""

    foreach ($module in $nb.modules) {
        Create-Issue -Module $module -NotebookLabel $nb.label -NotebookName $nb.name
    }

    Write-Host ""
}

$totalIssues = ($notebooksToProcess | ForEach-Object { $notebooks[$_].modules.Count } | Measure-Object -Sum).Sum

if ($DryRun) {
    Write-Host "✓ Preview completo: $totalIssues issues seriam criadas" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "Execute sem -DryRun para criar de verdade:" -ForegroundColor White
    Write-Host "  .\create-github-issues.ps1" -ForegroundColor Gray
} else {
    Write-Host "✓ $totalIssues issues criadas com sucesso!" -ForegroundColor Green
    Write-Host ""
    Write-Host "Verifique em: https://github.com/avilaops/arxis/issues" -ForegroundColor White
}
