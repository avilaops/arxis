# 🔬 Performance Profiling Guide - Arxis CLI Tools

## 📊 Como Medir Desempenho Corretamente

### 1️⃣ **Usar o Hyperfine (Benchmark CLI)**

Melhor ferramenta para comparar performance de comandos CLI.

#### Instalação:
```powershell
# Via Chocolatey
choco install hyperfine

# Ou via Scoop
scoop install hyperfine

# Ou baixar direto: https://github.com/sharkdp/hyperfine/releases
```

#### Uso:
```powershell
# Benchmark da compressão
hyperfine --warmup 3 `
  ".\target\release\avila-compress-cli.exe compress README.md" `
  "7z a -tzip test.zip README.md"

# Resultado esperado:
# Benchmark 1: avila-compress-cli
#   Time (mean ± σ):      20.5 ms ±   2.1 ms    [User: 5.2 ms, System: 8.1 ms]
#   Range (min … max):    18.1 ms …  25.3 ms    100 runs
```

---

### 2️⃣ **Profiling com Cargo Flamegraph**

Identifica onde o código está gastando tempo.

#### Instalação:
```powershell
cargo install flamegraph
cargo install cargo-flamegraph
```

#### Uso:
```powershell
cd examples\practical-cli

# Gerar flamegraph da compressão
cargo flamegraph --bin avila-compress-cli -- compress ..\..\README.md

# Abre flamegraph.svg no browser
start flamegraph.svg
```

---

### 3️⃣ **Measure-Command (PowerShell Nativo)**

Rápido para testes iniciais.

```powershell
# Medir tempo de compressão
Measure-Command {
    .\target\release\avila-compress-cli.exe compress README.md
} | Select-Object TotalMilliseconds

# Comparar com 7-Zip
Measure-Command {
    7z a -tzip test.zip README.md
} | Select-Object TotalMilliseconds

# Comparar com WinRAR
Measure-Command {
    rar a test.rar README.md
} | Select-Object TotalMilliseconds
```

---

### 4️⃣ **Profiling com Windows Performance Analyzer**

Para análise profunda de CPU/memória.

```powershell
# Instalar Windows Performance Toolkit
# https://learn.microsoft.com/en-us/windows-hardware/test/wpt/

# Capturar trace
wpr -start CPU -start FileIO

# Rodar comando
.\target\release\avila-compress-cli.exe compress large_file.dat

# Parar captura
wpr -stop trace.etl

# Analisar com WPA
wpa trace.etl
```

---

### 5️⃣ **Benchmark Script Automatizado**

```powershell
# benchmark-all.ps1
$FILES = @(
    @{Name="Small (40KB)"; File="README.md"},
    @{Name="Medium (1MB)"; File="test_1mb.dat"},
    @{Name="Large (100MB)"; File="test_100mb.dat"}
)

Write-Host "📊 Benchmark Results`n" -ForegroundColor Cyan

foreach ($test in $FILES) {
    Write-Host "Testing: $($test.Name)" -ForegroundColor Yellow

    # Avila Compress
    $avilaTime = (Measure-Command {
        .\target\release\avila-compress-cli.exe compress $test.File -o test.avz
    }).TotalMilliseconds

    $avilaSize = (Get-Item test.avz).Length

    # 7-Zip
    $zipTime = (Measure-Command {
        7z a -tzip -mx1 test.zip $test.File | Out-Null
    }).TotalMilliseconds

    $zipSize = (Get-Item test.zip).Length

    Write-Host "  Avila: $avilaTime ms ($avilaSize bytes)" -ForegroundColor Green
    Write-Host "  7-Zip: $zipTime ms ($zipSize bytes)" -ForegroundColor White
    Write-Host "  Speedup: $([math]::Round($zipTime/$avilaTime, 2))x`n" -ForegroundColor Cyan

    Remove-Item test.avz, test.zip
}
```

---

## 🐌 **Por Que Está Lento? Checklist de Diagnóstico**

### ✅ **1. Compilou com --release?**
```powershell
# ❌ ERRADO (debug = 10-100x mais lento!)
cargo build

# ✅ CORRETO
cargo build --release
```

### ✅ **2. Antivírus Está Bloqueando?**
Windows Defender pode tornar I/O 10x mais lento.

```powershell
# Adicionar exclusão temporária
Add-MpPreference -ExclusionPath "C:\Users\nicol\OneDrive\Avila\1.2 - Avilaops\Arxis"
```

### ✅ **3. OneDrive Está Sincronizando?**
OneDrive pode interceptar file I/O.

```powershell
# Ver status
Get-ItemProperty -Path "Registry::HKCU\Software\Microsoft\OneDrive" -Name "EnableFileSync"

# Pausar temporariamente para testes
# Botão direito no ícone OneDrive > Pausar sincronização > 2 horas
```

### ✅ **4. Disco SSD ou HDD?**
HDD pode ser 100x mais lento que SSD para operações pequenas.

```powershell
# Verificar tipo de disco
Get-PhysicalDisk | Select FriendlyName, MediaType, OperationalStatus
```

### ✅ **5. Tamanho do Arquivo**
Arquivos muito pequenos (<10KB) têm overhead de startup alto.

```powershell
# Criar arquivo de teste maior
$null = New-Item -Path test_large.dat -ItemType File -Value ([byte[]]::new(10MB))

# Benchmark
Measure-Command {
    .\target\release\avila-compress-cli.exe compress test_large.dat
}
```

---

## 🔥 **Otimizações Avançadas**

### **1. Link-Time Optimization (LTO)**

Adicione ao `Cargo.toml`:
```toml
[profile.release]
lto = "fat"           # Link-Time Optimization
codegen-units = 1     # Melhor otimização (mais lento de compilar)
opt-level = 3         # Máxima otimização
```

### **2. CPU-Specific Optimizations**

```powershell
# Compilar para CPU específica (AVX2, SSE4.2, etc)
$env:RUSTFLAGS="-C target-cpu=native"
cargo build --release
```

### **3. Profile-Guided Optimization (PGO)**

```powershell
# 1. Compilar com instrumentação
$env:RUSTFLAGS="-C profile-generate=pgo-data"
cargo build --release

# 2. Rodar para coletar dados
.\target\release\avila-compress-cli.exe compress README.md

# 3. Recompilar com otimizações
$env:RUSTFLAGS="-C profile-use=pgo-data\merged.profdata"
cargo build --release
```

---

## 📈 **Métricas Esperadas (Hardware Moderno)**

### **Compressão LZ4:**
- **Throughput:** 500-1000 MB/s
- **Latência (10KB):** < 1ms
- **Latência (1MB):** 1-5ms
- **Latência (100MB):** 100-200ms

### **HTTP Benchmark:**
- **Localhost:** < 1ms por request
- **Internet (Brasil):** 50-200ms
- **RPS:** 10k-100k req/s

### **Workspace Indexer:**
- **Small (1k files):** < 100ms
- **Medium (10k files):** < 1s
- **Large (100k files):** < 10s

---

## 🚨 **Red Flags (Sinais de Problema)**

1. **Compressão 10KB demora >100ms:** Antivírus ou disco lento
2. **HTTP localhost >10ms:** Firewall ou problema de rede
3. **Indexer <1k files/s:** Disco HDD ou muitas syscalls

---

## 🎯 **Próximos Passos**

1. **Instale Hyperfine:** `choco install hyperfine`
2. **Rode benchmark:** `hyperfine ".\avila-compress-cli.exe compress README.md"`
3. **Compare com 7-Zip:** Veja se realmente está lento
4. **Profile com flamegraph:** `cargo flamegraph --bin avila-compress-cli`
5. **Reporte resultados:** Compartilhe os números!

---

**🔬 Agora você tem ferramentas para medir tudo! 📊**
