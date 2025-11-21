# Guia de Instalação - Rust e AvilaDB DataFrame

## 🦀 Instalação do Rust

### Windows

1. **Baixe o instalador**:
   - Acesse: https://rustup.rs/
   - Ou baixe diretamente: https://win.rustup.rs/x86_64

2. **Execute o instalador**:
   ```powershell
   # Baixar e executar
   Invoke-WebRequest -Uri https://win.rustup.rs/x86_64 -OutFile rustup-init.exe
   .\rustup-init.exe
   ```

3. **Siga as instruções**:
   - Pressione `1` para instalação padrão
   - Aguarde a instalação completar
   - **IMPORTANTE**: Feche e reabra o PowerShell após a instalação

4. **Verifique a instalação**:
   ```powershell
   # Reinicie o PowerShell primeiro!
   rustc --version
   cargo --version
   ```

   Saída esperada:
   ```
   rustc 1.83.0 (90b35a623 2024-11-26)
   cargo 1.83.0 (5ffbef321 2024-10-29)
   ```

### Linux/Mac

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustc --version
```

---

## 📦 Instalação do AvilaDB DataFrame

### 1. Clone o Repositório

```powershell
# PowerShell
cd "C:\Users\nicol\OneDrive\Avila\1.2 - Avilaops\1.2.7 - Identidade visual\Arxis"
git clone https://github.com/your-org/avila-dataframe.git
cd avila-dataframe
```

### 2. Build do Projeto

```powershell
# Build padrão (release)
cargo build --release

# Build com todas as features
cargo build --release --all-features

# Build apenas para desenvolvimento (mais rápido)
cargo build
```

### 3. Execute os Exemplos

```powershell
# Exemplo básico
cargo run --example basic_usage

# Tipos científicos
cargo run --example scientific_types

# Operações essenciais (filter, group_by, join, etc.)
cargo run --example essential_ops

# Análise de notas de estudantes
cargo run --example student_grades

# Demo completo de I/O (Parquet, CSV, HDF5, AvilaDB)
cargo run --example io_demo --release

# Com suporte HDF5
cargo run --example io_demo --release --features io-hdf5
```

---

## 🔧 Dependências Opcionais

### HDF5 (Dados Científicos)

**Windows**:
1. Baixe HDF5 precompilado: https://www.hdfgroup.org/downloads/hdf5/
2. Instale em `C:\Program Files\HDF5`
3. Configure variável de ambiente:
   ```powershell
   [System.Environment]::SetEnvironmentVariable('HDF5_DIR', 'C:\Program Files\HDF5', 'User')
   ```

**Linux (Ubuntu/Debian)**:
```bash
sudo apt-get install libhdf5-dev
```

**Mac**:
```bash
brew install hdf5
```

**Build com HDF5**:
```powershell
cargo build --features io-hdf5
cargo run --example io_demo --features io-hdf5
```

---

## 🚀 Uso no Seu Projeto

### 1. Adicione ao Cargo.toml

```toml
[dependencies]
avila-dataframe = "0.1.0"

# Ou via Git
avila-dataframe = { git = "https://github.com/your-org/avila-dataframe" }

# Com features opcionais
avila-dataframe = { version = "0.1.0", features = ["io-hdf5"] }
```

### 2. Código Básico

```rust
use avila_dataframe::prelude::*;

fn main() -> Result<()> {
    // Criar DataFrame
    let df = DataFrame::new(vec![
        Series::new("nome", vec!["Ana", "Bruno", "Carlos"]),
        Series::new("idade", vec![25.0, 30.0, 28.0]),
        Series::new("salario", vec![5000.0, 6500.0, 5800.0]),
    ])?;

    // Filtrar
    let filtered = df.filter(col("idade").gt(lit(26.0)))?;

    // Agrupar
    let grouped = df
        .group_by(&["nome"])?
        .agg(&[("salario", "mean")])?;

    // Salvar
    df.write_parquet("dados.parquet")?;

    println!("{}", df);
    Ok(())
}
```

---

## 🛠️ Troubleshooting

### Cargo não encontrado após instalação

**Problema**: `cargo: The term 'cargo' is not recognized...`

**Solução**:
1. **Feche e reabra o PowerShell** (IMPORTANTE!)
2. Ou adicione manualmente ao PATH:
   ```powershell
   $env:PATH += ";$env:USERPROFILE\.cargo\bin"
   ```
3. Ou use caminho completo:
   ```powershell
   & "$env:USERPROFILE\.cargo\bin\cargo.exe" build
   ```

### Erro de compilação com HDF5

**Problema**: `error: failed to run custom build command for 'hdf5-sys'`

**Solução**:
- **Opção 1**: Não use feature `io-hdf5`:
  ```powershell
  cargo build  # Sem --features io-hdf5
  ```
- **Opção 2**: Instale HDF5 (veja seção de dependências acima)

### Erro "linker not found"

**Problema**: `error: linker 'link.exe' not found`

**Solução** (Windows):
1. Instale Visual Studio Build Tools: https://visualstudio.microsoft.com/downloads/
2. Ou instale MinGW: https://www.mingw-w64.org/
3. Reinstale Rust: `rustup self update`

### Builds muito lentos

**Solução**:
```powershell
# Use build incremental (dev)
cargo build

# Ou use cache compartilhado
cargo install sccache
$env:RUSTC_WRAPPER = "sccache"
cargo build --release
```

---

## 📊 Verificar Instalação

Execute o script de verificação:

```powershell
# Verifica todas as dependências
.\scripts\check.ps1

# Ou manual
rustc --version
cargo --version
cargo build --release
cargo test
cargo run --example basic_usage
```

Saída esperada:
```
✅ Rust: 1.83.0
✅ Cargo: 1.83.0
✅ Build: OK
✅ Tests: 12 passed
✅ Examples: OK
```

---

## 🎯 Próximos Passos

Após a instalação:

1. **Explore os exemplos**:
   ```powershell
   cargo run --example basic_usage
   cargo run --example scientific_types
   cargo run --example astronomy_example
   cargo run --example student_grades
   cargo run --example io_demo --release
   ```

2. **Leia a documentação**:
   ```powershell
   cargo doc --open
   ```

3. **Execute os testes**:
   ```powershell
   cargo test
   ```

4. **Execute os benchmarks**:
   ```powershell
   cargo bench
   ```

5. **Crie seu primeiro projeto**:
   ```powershell
   cargo new meu-projeto
   cd meu-projeto
   # Adicione avila-dataframe ao Cargo.toml
   cargo run
   ```

---

## 🔥 AvilaDB Cloud (Opcional)

Para usar integração com AvilaDB:

1. **Crie conta**: https://console.avila.cloud/
2. **Obtenha auth key**: Dashboard → Settings → API Keys
3. **Configure no código**:
   ```rust
   use avila_dataframe::io::AvilaDbConfig;

   let config = AvilaDbConfig::new("sua-conta", "seu-db", "sua-collection")
       .with_endpoint("https://avila.cloud")
       .with_auth_key(std::env::var("AVILADB_KEY")?);

   df.write_aviladb(&config)?;
   ```

4. **Variável de ambiente**:
   ```powershell
   $env:AVILADB_KEY = "sua-chave-aqui"
   ```

---

## 📚 Recursos Adicionais

- **Documentação oficial Rust**: https://doc.rust-lang.org/
- **Rust Book (PT-BR)**: https://rust-br.github.io/rust-book-pt-br/
- **Cargo Book**: https://doc.rust-lang.org/cargo/
- **AvilaDB Docs**: https://docs.avila.cloud/
- **Apache Arrow**: https://arrow.apache.org/docs/

---

## 💬 Suporte

- **Issues**: https://github.com/your-org/avila-dataframe/issues
- **Discord**: https://discord.gg/avilacloud
- **Email**: suporte@avila.cloud

---

## 🇧🇷 Feito com 💚💛 no Brasil!

**AvilaDB DataFrame** - Data Science de verdade, sem frescura! 🚀
