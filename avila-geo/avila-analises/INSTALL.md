# Guia de Instalação - Sistema de Análise Comportamental

## Pré-requisitos

### 1. Instalar Rust

#### Windows (PowerShell)
```powershell
# Baixar e executar rustup-init
Invoke-WebRequest -Uri "https://win.rustup.rs/x86_64" -OutFile "$env:TEMP\rustup-init.exe"
& "$env:TEMP\rustup-init.exe" -y

# Adicionar ao PATH
$env:Path += ";$env:USERPROFILE\.cargo\bin"

# Verificar instalação
cargo --version
rustc --version
```

#### Linux/macOS
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
cargo --version
```

### 2. Instalar Dependências do Sistema

#### Windows
```powershell
# Visual Studio Build Tools (necessário para compilar)
# Baixar de: https://visualstudio.microsoft.com/downloads/
# Ou instalar via winget:
winget install Microsoft.VisualStudio.2022.BuildTools
```

#### Linux (Ubuntu/Debian)
```bash
sudo apt update
sudo apt install build-essential pkg-config libssl-dev
```

#### macOS
```bash
xcode-select --install
```

## Instalação do Projeto

### 1. Clonar Repositório (ou usar projeto local)

```bash
cd d:\GitHub\arxis\avila-analises
```

### 2. Compilar Projeto

```bash
# Build em modo debug (mais rápido para desenvolvimento)
cargo build

# Build otimizado para produção
cargo build --release
```

### 3. Executar

```bash
# Modo debug
cargo run

# Modo release (otimizado)
cargo run --release
```

### 4. Executar Testes

```bash
# Todos os testes
cargo test

# Testes com output detalhado
cargo test -- --nocapture

# Teste específico
cargo test tracker::tests::test_event_tracking
```

## Estrutura de Diretórios Após Build

```
avila-analises/
├── Cargo.toml              # Configuração do projeto
├── Cargo.lock              # Lock de dependências
├── README.md               # Documentação principal
├── INSTALL.md              # Este arquivo
├── src/
│   ├── main.rs            # Ponto de entrada
│   ├── models.rs          # Estruturas de dados
│   ├── tracker.rs         # Sistema de tracking
│   ├── funnel.rs          # Análise de funil
│   ├── cohort.rs          # Análise de cohort
│   ├── segmentation.rs    # Segmentação
│   ├── prediction.rs      # ML e predições
│   ├── dashboard.rs       # Dashboard RT
│   └── examples.rs        # Exemplos de integração
└── target/                 # Binários compilados (gerado)
    ├── debug/
    │   └── avila-analises.exe
    └── release/
        └── avila-analises.exe
```

## Troubleshooting

### Erro: "cargo: command not found"

**Solução:**
```powershell
# Adicionar Cargo ao PATH
$env:Path += ";$env:USERPROFILE\.cargo\bin"

# Ou reiniciar o terminal após instalação do Rust
```

### Erro: "linker 'link.exe' not found"

**Solução Windows:**
Instale o Visual Studio Build Tools:
```powershell
winget install Microsoft.VisualStudio.2022.BuildTools --interactive
```

Durante a instalação, selecione:
- "Desktop development with C++"
- "C++ build tools"

### Erro: "failed to resolve: use of undeclared crate"

**Solução:**
```bash
# Limpar e reconstruir
cargo clean
cargo build
```

### Erro de Compilação em Testes

**Solução:**
```bash
# Atualizar dependências
cargo update

# Verificar versão do Rust
rustc --version

# Atualizar Rust se necessário
rustup update
```

## Configuração do Ambiente de Desenvolvimento

### VS Code

1. Instalar extensões:
   - `rust-analyzer` (análise de código Rust)
   - `CodeLLDB` (debugger)
   - `crates` (gerenciar dependências)

2. Configurar `.vscode/settings.json`:
```json
{
  "rust-analyzer.checkOnSave.command": "clippy",
  "rust-analyzer.cargo.allFeatures": true,
  "rust-analyzer.inlayHints.enable": true
}
```

3. Configurar `.vscode/launch.json` para debug:
```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "type": "lldb",
      "request": "launch",
      "name": "Debug",
      "cargo": {
        "args": ["build", "--bin=avila-analises"]
      },
      "args": [],
      "cwd": "${workspaceFolder}"
    }
  ]
}
```

### IntelliJ IDEA / CLion

1. Instalar plugin "Rust"
2. Importar projeto via `Cargo.toml`
3. Configurar run configuration para `main.rs`

## Verificação da Instalação

Execute este script para verificar se tudo está funcionando:

```bash
# 1. Verificar Rust
cargo --version
rustc --version

# 2. Verificar build
cargo check

# 3. Executar testes
cargo test

# 4. Executar aplicação
cargo run

# 5. Build otimizado
cargo build --release
```

Se todos os comandos executarem sem erro, a instalação está correta!

## Próximos Passos

1. **Executar aplicação de exemplo:**
   ```bash
   cargo run
   ```

2. **Explorar exemplos de integração:**
   ```bash
   cargo test examples::tests::test_examples -- --nocapture
   ```

3. **Configurar AvilaDB local:**
   ```bash
   # Via Docker
   docker run -p 8000:8000 avilacloud/aviladb-emulator:latest
   ```

4. **Integrar com seu projeto:**
   - Copie os módulos necessários
   - Ajuste `Cargo.toml` do seu projeto
   - Importe as funcionalidades

## Recursos Adicionais

- [Rust Book](https://doc.rust-lang.org/book/)
- [Cargo Guide](https://doc.rust-lang.org/cargo/)
- [AvilaDB Docs](https://docs.avila.cloud)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)

## Suporte

Para problemas ou dúvidas:
1. Verifique este guia de instalação
2. Consulte o README.md principal
3. Abra uma issue no repositório

---

**Instalação concluída com sucesso? Execute:**
```bash
cargo run
```

E veja o sistema de análise comportamental em ação! 🚀
