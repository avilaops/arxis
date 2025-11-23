# Avila Convexa1D

🎵 **Processamento de Dados Sequenciais 1D para Áudio e Texto em Rust**

Biblioteca Rust para análise e processamento de dados sequenciais unidimensionais, com foco em áudio e texto. Implementa convolução temporal, extração de features e embeddings.

## 🚀 Características

- **Processamento de Áudio**
  - Geração de sinais (ondas senoidais)
  - Normalização e filtragem
  - Extração de features temporais (energia, ZCR, RMS)
  - MFCC (Mel-Frequency Cepstral Coefficients)
  - Análise espectral (espectrograma)
  - Cálculo de envelope

- **Processamento de Texto**
  - Tokenização e construção de vocabulário
  - Embeddings de palavras
  - Features textuais (densidade lexical, estatísticas)
  - Convolução temporal em sequências de texto
  - Conversão de texto para vetores

- **Operações Comuns**
  - Convolução 1D com kernels personalizados
  - Filtros gaussianos e de média móvel
  - Funções de ativação (ReLU, Sigmoid, Tanh)
  - Estruturas de dados sequenciais

## 📦 Instalação

Adicione ao seu `Cargo.toml`:

```toml
[dependencies]
avila-convexa1d = "0.1.0"
ndarray = "0.15"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

## 🎯 Uso Rápido

### Processamento de Áudio

```rust
use avila_convexa1d::{AudioProcessor, ConvolutionKernel};

fn main() {
    // Cria processador de áudio
    let processor = AudioProcessor::default();

    // Gera sinal senoidal (440Hz, 1 segundo)
    let signal = processor.generate_sine_wave(440.0, 1.0, 0.8);

    // Extrai features
    let features = processor.extract_features(&signal);
    println!("Energia: {:.4}", features.energy);
    println!("ZCR: {:.4}", features.zcr);
    println!("MFCC: {:?}", features.mfcc);

    // Aplica filtro gaussiano
    let kernel = ConvolutionKernel::gaussian(64, 10.0);
    let filtered = processor.temporal_convolution(&signal, &kernel);
}
```

### Processamento de Texto

```rust
use avila_convexa1d::TextProcessor;

fn main() {
    let mut processor = TextProcessor::new(16);

    // Constrói vocabulário
    let corpus = vec![
        "AvilaDB é o banco de dados do Brasil".to_string(),
        "Processamento de texto com Rust".to_string(),
    ];
    processor.build_vocab(&corpus);

    // Extrai features
    let text = "AvilaDB é incrível";
    let features = processor.extract_features(text);
    println!("Tokens: {}", features.token_count);
    println!("Densidade lexical: {:.3}", features.lexical_density);

    // Converte para embeddings
    if let Some(embeddings) = processor.text_to_embeddings(text) {
        println!("Shape: {}x{}", embeddings.nrows(), embeddings.ncols());
    }
}
```

## 🏗️ Arquitetura

```
avila-convexa1d/
├── src/
│   ├── lib.rs          # Módulo principal
│   ├── audio.rs        # Processamento de áudio
│   ├── text.rs         # Processamento de texto
│   ├── common.rs       # Estruturas e funções comuns
│   └── main.rs         # Exemplo completo
├── Cargo.toml
└── README.md
```

## 📊 Features de Áudio

| Feature           | Descrição                                        |
| ----------------- | ------------------------------------------------ |
| **Energia**       | Potência média do sinal                          |
| **ZCR**           | Taxa de cruzamento por zero (zero-crossing rate) |
| **RMS**           | Root Mean Square - amplitude efetiva             |
| **MFCC**          | Coeficientes cepstrais na escala Mel             |
| **Envelope**      | Envoltória do sinal                              |
| **Espectrograma** | Representação tempo-frequência                   |

## 📝 Features de Texto

| Feature              | Descrição                    |
| -------------------- | ---------------------------- |
| **Token Count**      | Número total de tokens       |
| **Unique Tokens**    | Vocabulário único            |
| **Avg Token Length** | Comprimento médio dos tokens |
| **Lexical Density**  | Razão tokens únicos / total  |
| **Embeddings**       | Representação vetorial densa |

## 🧪 Executar Exemplos

```bash
# Executar exemplo completo
cargo run

# Executar testes
cargo test

# Executar com output detalhado
cargo run --release
```

## 📚 Casos de Uso

### 1. **Classificação de Áudio**
```rust
let processor = AudioProcessor::default();
let signal = load_audio_file("sample.wav");
let features = processor.extract_features(&signal);
// Usar features.to_vector() para ML
```

### 2. **Análise de Sentimento em Texto**
```rust
let mut processor = TextProcessor::new(32);
processor.build_vocab(&training_corpus);
let features = processor.extract_features(&user_input);
// Classificar baseado em features
```

### 3. **Detecção de Padrões Temporais**
```rust
let kernel = ConvolutionKernel::gaussian(128, 20.0);
let patterns = processor.temporal_convolution(&signal, &kernel);
// Identificar padrões recorrentes
```

## 🔧 Kernels de Convolução

```rust
// Filtro gaussiano (suavização)
let gaussian = ConvolutionKernel::gaussian(64, 10.0);

// Média móvel (smoothing)
let moving_avg = ConvolutionKernel::moving_average(5);

// Kernel customizado
let custom = ConvolutionKernel::new(
    vec![0.2, 0.6, 0.2],
    1,  // stride
    1   // padding
);
```

## 🎓 Conceitos Importantes

### Convolução 1D
Operação que desliza um kernel sobre o sinal, extraindo features locais:
- **Suavização**: Remove ruído
- **Detecção de bordas**: Identifica mudanças bruscas
- **Extração de features**: Captura padrões temporais

### Dados Sequenciais
Dados ordenados temporalmente onde a posição importa:
- **Áudio**: Amostras ao longo do tempo
- **Texto**: Sequência de palavras/tokens
- **Séries temporais**: Medições em intervalos

## 🤝 Integração com AVL Platform

Esta biblioteca é otimizada para trabalhar com **AvilaDB**:

```rust
use aviladb::AvilaClient;
use avila_convexa1d::{AudioProcessor, SequentialData};

#[tokio::main]
async fn main() {
    let client = AvilaClient::connect("http://localhost:8000").await?;
    let db = client.database("audioml").await?;
    let features_collection = db.collection("audio_features").await?;

    // Processa áudio
    let processor = AudioProcessor::default();
    let signal = load_audio();
    let features = processor.extract_features(&signal);

    // Salva no AvilaDB
    features_collection.insert(features).await?;
}
```

## 📈 Performance

- **Processamento de áudio**: ~10ms para 1s de áudio @ 44.1kHz
- **Tokenização de texto**: ~1ms para 1000 palavras
- **Convolução 1D**: O(n*m) onde n=tamanho do sinal, m=tamanho do kernel
- **Extração de MFCC**: ~50ms para 1s de áudio

## 🛠️ Desenvolvimento

```bash
# Compilar
cargo build --release

# Executar testes
cargo test

# Gerar documentação
cargo doc --open

# Verificar formatação
cargo fmt --check

# Linter
cargo clippy
```

## 📄 Licença

MIT License - Avila Cloud Platform

## 🌟 Características Futuras

- [ ] Suporte a WAV/MP3 nativo
- [ ] Transformada de Fourier otimizada (FFT)
- [ ] Modelos pré-treinados de embeddings (BERT, Word2Vec)
- [ ] Suporte a GPU via CUDA
- [ ] Streaming de áudio em tempo real
- [ ] Análise de fonemas e prosódia

## 📞 Suporte

- Documentação: [docs.avila.cloud](https://docs.avila.cloud)
- Issues: GitHub Issues
- Email: support@avila.cloud

---

**Feito com ❤️ pela equipe AVL Cloud Platform** 🇧🇷

*Processamento de dados sequenciais rápido e eficiente em Rust puro!*
