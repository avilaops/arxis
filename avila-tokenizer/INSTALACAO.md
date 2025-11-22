# 🏠 Instalação Local - avila-tokenizers

## Opções de Instalação

### 1️⃣ **Como Biblioteca Rust (Recomendado)**

Adicione ao `Cargo.toml` do seu projeto:

```toml
[dependencies]
avila-tokenizers = { path = "C:/Users/nicol/OneDrive/Avila/1.2 - Avilaops/Arxis/avila-tokenizer" }
```

**Vantagens:**
- ✅ Zero custo de runtime
- ✅ Type-safe
- ✅ Performance máxima
- ✅ Uso direto no código Rust

---

### 2️⃣ **Criar CLI (Ferramenta de Linha de Comando)**

Transformar em executável `.exe` que roda em qualquer lugar:

```bash
# 1. Adicionar ao Cargo.toml:
[[bin]]
name = "avila-tokenizer"
path = "src/bin/cli.rs"

# 2. Compilar
cargo build --release --bin avila-tokenizer

# 3. Instalar globalmente
cargo install --path . --bin avila-tokenizer

# 4. Usar de qualquer lugar
avila-tokenizer encode "Olá mundo" --model gpt2
```

**Resultado:**
- ✅ Executável: `~/.cargo/bin/avila-tokenizer.exe`
- ✅ Disponível globalmente no PATH
- ✅ Uso via terminal

---

### 3️⃣ **Criar DLL/Shared Library**

Para integração com outras linguagens (Python, Node.js, C#):

```toml
[lib]
crate-type = ["cdylib", "rlib"]
```

```bash
cargo build --release --lib
```

**Resultado:**
- Windows: `avila_tokenizers.dll`
- Linux: `libavila_tokenizers.so`
- macOS: `libavila_tokenizers.dylib`

**Uso em Python:**
```python
from ctypes import CDLL
lib = CDLL("./avila_tokenizers.dll")
```

---

### 4️⃣ **Publicar no Crates.io (Público)**

Disponibilizar para toda comunidade Rust:

```bash
# 1. Login
cargo login

# 2. Publicar
cargo publish

# 3. Qualquer um pode usar:
# Cargo.toml
[dependencies]
avila-tokenizers = "0.1.0"
```

---

### 5️⃣ **Registry Privado (AVL Interno)**

Usar registry Git privado da AVL:

```toml
[dependencies]
avila-tokenizers = { git = "https://github.com/avila/avila-tokenizers", branch = "main" }
```

Ou registry interno:
```toml
[registries.avl]
index = "https://registry.avila.cloud/git/index"

[dependencies]
avila-tokenizers = { version = "0.1.0", registry = "avl" }
```

---

## 🚀 Instalação Rápida (Escolha uma)

### A) **Uso em Projeto Rust**
```bash
cd seu-projeto
```

Adicione ao `Cargo.toml`:
```toml
[dependencies]
avila-tokenizers = { path = "../avila-tokenizer" }
```

### B) **CLI Global**
```bash
cd avila-tokenizer

# Criar arquivo src/bin/cli.rs (ver exemplo abaixo)
cargo install --path .

# Usar
avila-tokenizer --help
```

### C) **DLL para Python/Node.js**
```bash
# Adicionar ao Cargo.toml:
[lib]
crate-type = ["cdylib"]

# Compilar
cargo build --release

# DLL estará em:
# target/release/avila_tokenizers.dll
```

---

## 📝 Exemplo: CLI Completo

Crie `src/bin/cli.rs`:

```rust
use avila_tokenizers::models::{GPT2Tokenizer, BertTokenizer, LlamaTokenizer};
use clap::Parser;

#[derive(Parser)]
#[command(name = "avila-tokenizer")]
#[command(about = "Tokenizador AVL - GPT2, BERT, Llama")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Parser)]
enum Commands {
    /// Codificar texto em token IDs
    Encode {
        /// Texto para codificar
        text: String,

        /// Modelo: gpt2, bert-base-uncased, llama-2-7b
        #[arg(short, long, default_value = "gpt2")]
        model: String,
    },

    /// Decodificar token IDs em texto
    Decode {
        /// IDs separados por vírgula (ex: 15496,995)
        ids: String,

        /// Modelo
        #[arg(short, long, default_value = "gpt2")]
        model: String,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Encode { text, model } => {
            let tokens = match model.as_str() {
                "gpt2" | "gpt2-small" => {
                    let mut tokenizer = GPT2Tokenizer::from_pretrained("gpt2")?;
                    tokenizer.encode(&text)
                }
                "bert-base-uncased" | "bert" => {
                    let tokenizer = BertTokenizer::from_pretrained("bert-base-uncased")?;
                    tokenizer.encode(&text)
                }
                "llama-2-7b" | "llama2" => {
                    let tokenizer = LlamaTokenizer::from_pretrained("llama-2-7b")?;
                    tokenizer.encode(&text)
                }
                _ => panic!("Modelo desconhecido: {}", model),
            };

            println!("Texto: {}", text);
            println!("Tokens: {:?}", tokens);
            println!("Count: {}", tokens.len());
        }

        Commands::Decode { ids, model } => {
            let token_ids: Vec<u32> = ids
                .split(',')
                .map(|s| s.trim().parse().expect("ID inválido"))
                .collect();

            let text = match model.as_str() {
                "gpt2" | "gpt2-small" => {
                    let tokenizer = GPT2Tokenizer::from_pretrained("gpt2")?;
                    tokenizer.decode(&token_ids)?
                }
                "bert-base-uncased" | "bert" => {
                    let tokenizer = BertTokenizer::from_pretrained("bert-base-uncased")?;
                    tokenizer.decode(&token_ids)?
                }
                "llama-2-7b" | "llama2" => {
                    let tokenizer = LlamaTokenizer::from_pretrained("llama-2-7b")?;
                    tokenizer.decode(&token_ids)?
                }
                _ => panic!("Modelo desconhecido: {}", model),
            };

            println!("IDs: {:?}", token_ids);
            println!("Texto: {}", text);
        }
    }

    Ok(())
}
```

Adicione ao `Cargo.toml`:
```toml
[[bin]]
name = "avila-tokenizer"
path = "src/bin/cli.rs"

[dependencies]
clap = { version = "4.5", features = ["derive"] }
```

**Instalar:**
```bash
cargo install --path .
```

**Usar:**
```bash
avila-tokenizer encode "Olá mundo!" --model gpt2
avila-tokenizer decode "15496,995" --model gpt2
```

---

## 🔧 Integração com Outras Linguagens

### Python (via PyO3)

Criar binding Python nativo:

```toml
[lib]
name = "avila_tokenizers"
crate-type = ["cdylib"]

[dependencies]
pyo3 = { version = "0.20", features = ["extension-module"] }
```

```rust
use pyo3::prelude::*;

#[pyclass]
struct PyGPT2Tokenizer {
    inner: GPT2Tokenizer,
}

#[pymethods]
impl PyGPT2Tokenizer {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self {
            inner: GPT2Tokenizer::from_pretrained("gpt2")?,
        })
    }

    fn encode(&mut self, text: &str) -> Vec<u32> {
        self.inner.encode(text)
    }
}

#[pymodule]
fn avila_tokenizers(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyGPT2Tokenizer>()?;
    Ok(())
}
```

**Uso:**
```python
import avila_tokenizers

tokenizer = avila_tokenizers.PyGPT2Tokenizer()
tokens = tokenizer.encode("Hello world")
print(tokens)
```

### Node.js (via Neon)

```bash
npm install -g neon-cli
neon new avila-tokenizers-node
```

### C# (.NET)

Usar P/Invoke para chamar DLL.

---

## 📍 Localização dos Artefatos

Após compilação:

```
avila-tokenizer/
├── target/
│   ├── release/
│   │   ├── libavila_tokenizers.rlib    # Biblioteca Rust
│   │   ├── avila_tokenizers.dll        # DLL (se cdylib)
│   │   └── avila-tokenizer.exe         # CLI (se bin)
│   └── doc/
│       └── avila_tokenizers/           # Documentação
└── ~/.cargo/bin/                       # Se instalado globalmente
    └── avila-tokenizer.exe
```

---

## ✅ Recomendações por Caso de Uso

| Caso de Uso      | Método Recomendado                 |
| ---------------- | ---------------------------------- |
| **Projeto Rust** | Path dependency no Cargo.toml      |
| **CLI/Terminal** | `cargo install --path .`           |
| **Python**       | PyO3 binding                       |
| **Node.js**      | Neon binding                       |
| **C/C++**        | cdylib (.dll/.so)                  |
| **AVL Interno**  | Git dependency ou registry privado |
| **Público**      | Publicar no crates.io              |

---

## 🎯 Qual você quer implementar?

Diga qual caso de uso você precisa e eu crio os arquivos necessários! 🚀
