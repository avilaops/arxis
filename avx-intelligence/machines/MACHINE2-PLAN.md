# 💻 MACHINE 2 - AVILA-RUNTIME

## 🎯 Papel Estratégico
**Build Engine** - Compilação, testes, benchmarks e validação de performance

---

## 📋 Setup Inicial (Segunda-feira)

### 1. Capturar Logs
```powershell
cd d:\arxis\avx-intelligence\scripts
.\capture-logs.ps1 -All
```

### 2. Rust Toolchain Completo
```powershell
# Stable + Nightly
rustup install stable nightly
rustup default stable

# Components
rustup component add clippy rustfmt llvm-tools-preview

# Tools
cargo install cargo-nextest      # Testes rápidos
cargo install cargo-llvm-cov      # Coverage
cargo install cargo-benchcmp     # Benchmark comparison
cargo install cargo-criterion    # Benchmarking
cargo install cargo-audit        # Security audit
```

### 3. Build Environment
```powershell
# Configurar otimizações
# Criar .cargo\config.toml
[build]
rustflags = ["-C", "target-cpu=native"]

[profile.release]
lto = "thin"
codegen-units = 1
```

---

## 🔄 Tarefas por Fase

### FASE 1 (Dez 2025): Notebook 1 Support
**Notebooks ativos**: 1

#### Daily Tasks
```powershell
# 1. Pull latest code
cd d:\arxis
git pull origin main

# 2. Build tudo
cargo build --all --release

# 3. Rodar testes
cargo nextest run --all

# 4. Clippy check
cargo clippy --all -- -D warnings

# 5. Coverage report
cargo llvm-cov --all --html
# Output: target/llvm-cov/html/index.html
```

#### Benchmarks (Semanais)
```powershell
# Rodar benchmarks de todos os crates
cd avila-primitives
cargo bench

cd ../avila-error
cargo bench

# Comparar com semana anterior
cargo benchcmp old.txt new.txt
```

---

### FASE 2 (Jan 2026): Notebooks 1+2
**Notebooks ativos**: 1, 2 (Matemática)

#### Build Strategy
```powershell
# Build paralelo por workspace
cd d:\arxis\avila-core-workspace
cargo build --release --jobs 8

# Rodar testes matemáticos específicos
cargo nextest run --package avila-math --package avila-linalg
```

#### Performance Testing
```powershell
# Benchmarks matemáticos críticos
cd avila-ndarray
cargo bench -- --warm-up-time 3 --measurement-time 10

# SIMD verification
cargo build --target-feature=+avx2
cargo test --release
```

---

### FASE 3 (Fev 2026): Notebooks 1+2+3
**Notebooks ativos**: 1, 2, 3 (Data/ML)

#### ML Benchmarks
```powershell
# Clustering performance
cd avila-clustering
cargo bench -- kmeans hdbscan

# Tokenizer speed
cd avila-tokenizer
cargo bench -- --save-baseline v0.1.0
```

#### Memory Profiling
```powershell
# Instalar heaptrack
# Rodar com profiling
cargo build --release
heaptrack ./target/release/benchmark
```

---

### FASE 4 (Mar-Jun 2026): Full Stack
**Notebooks ativos**: 1, 2, 3, 4, 5

#### Integration Tests
```powershell
# Testes end-to-end
cd d:\arxis
cargo nextest run --workspace --test-threads 8

# Smoke tests
.\avila-scripts\run-smoke-tests.ps1
```

#### Load Testing
```powershell
# AvilaDB load test
cd avila-db
cargo run --release --bin load-test -- --duration 60s --rps 1000

# AVX Gateway stress test
cd avx-gateway
cargo run --release --bin stress-test -- --connections 100
```

---

## 📊 Performance Monitoring

### Benchmark Dashboard
```powershell
# Gerar relatório HTML de benchmarks
cargo criterion --message-format json > bench-results.json
python ..\avx-intelligence\analytics\visualize-benchmarks.py
```

### Métricas a Coletar

#### Build Metrics
- Tempo de compilação total
- Tempo por crate
- Cache hit rate
- Incremental build time

#### Test Metrics
- Testes executados
- Tempo de execução
- Testes falhando
- Coverage %

#### Performance Metrics
- Throughput (ops/sec)
- Latency (p50, p95, p99)
- Memory usage
- CPU utilization

---

## 🎯 Notebooks Responsáveis

### Notebook 2 - Matemática (A partir de ~15/Dez)
**VS Code 1** - Coordenar desenvolvimento quando liberado

**Primeiros 4 módulos**:
1. avila-math
2. avila-numeric
3. avila-linalg
4. avila-ndarray

**Critérios específicos**:
- Benchmarks vs NumPy/SciPy
- SIMD otimizado
- Zero-copy arrays
- Documentação matemática (LaTeX inline)

### Notebook 5 - Advanced (A partir de ~15/Mar)
**VS Code 2** - Coordenar quando base estável

**Primeiros 4 módulos**:
1. avx-gpu
2. avx-quantum-render
3. avx-mcp
4. avx-copilot-ai

**Critérios específicos**:
- GPU compute funcional
- Paper research ready
- Benchmarks GPU vs CPU

---

## 🔧 Scripts Customizados

### benchmark-all.ps1
```powershell
# Rodar todos os benchmarks e salvar resultados
param([string]$Baseline = "main")

$crates = Get-ChildItem -Directory | Where-Object { Test-Path "$_\Cargo.toml" }

foreach ($crate in $crates) {
    Push-Location $crate
    Write-Host "Benchmarking $($crate.Name)..." -ForegroundColor Cyan
    
    cargo bench --message-format json `
        | Tee-Object -FilePath "..\bench-results\$($crate.Name)-$(Get-Date -Format 'yyyyMMdd').json"
    
    Pop-Location
}

Write-Host "✓ Benchmarks concluídos" -ForegroundColor Green
```

### test-coverage-report.ps1
```powershell
# Gerar relatório de cobertura completo
cargo llvm-cov clean
cargo llvm-cov --all --html

# Mover para pasta de reports
$date = Get-Date -Format "yyyyMMdd"
Copy-Item -Path "target\llvm-cov\html" -Destination "..\reports\coverage-$date" -Recurse

Write-Host "Coverage report: ..\reports\coverage-$date\index.html" -ForegroundColor Green
```

### validate-performance.ps1
```powershell
# Validar que não houve regressão de performance
param([double]$Threshold = 0.05)  # 5% regression limit

$current = Get-Content "bench-current.json" | ConvertFrom-Json
$baseline = Get-Content "bench-baseline.json" | ConvertFrom-Json

$regressions = @()

foreach ($bench in $current) {
    $base = $baseline | Where-Object { $_.name -eq $bench.name }
    if ($base) {
        $change = ($bench.time - $base.time) / $base.time
        if ($change -gt $Threshold) {
            $regressions += [PSCustomObject]@{
                Benchmark = $bench.name
                Regression = "$([math]::Round($change * 100, 2))%"
                Current = "$($bench.time)ns"
                Baseline = "$($base.time)ns"
            }
        }
    }
}

if ($regressions.Count -gt 0) {
    Write-Host "⚠ Performance regressions detected:" -ForegroundColor Red
    $regressions | Format-Table
    exit 1
} else {
    Write-Host "✓ No performance regressions" -ForegroundColor Green
}
```

---

## 📅 Agenda Semanal

### Segunda
- 9:00 - Build full workspace
- 14:00 - Rodar testes completos
- 17:00 - Baseline benchmarks

### Terça-Quinta
- 9:00 - Incremental builds
- 12:00 - Testes de módulos novos
- 15:00 - Clippy + fmt check
- 17:00 - Quick benchmarks

### Sexta
- 9:00 - Full rebuild
- 11:00 - Coverage report
- 14:00 - Benchmark comparison
- 16:00 - Performance validation
- 18:00 - Weekly metrics

---

## 🎯 KPIs Machine 2

### Diários
- Build success rate: 95%+
- Test pass rate: 100%
- Build time: <5min (incremental)
- Clippy warnings: 0

### Semanais
- Full build: <30min
- Coverage: >80%
- Benchmarks executed: 100%
- Performance validated: No regressions >5%

### Mensais
- Benchmark improvements: Track
- Memory leaks: 0
- Security audits: Pass
- Release builds: Optimized

---

**Machine Owner**: Build & Performance Engineer
**VS Codes**: 2 (Notebooks 2 e 5 quando ativos)
**Role**: Validation & Performance
