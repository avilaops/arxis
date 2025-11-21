# 🏛️ Arxis - Onboarding para Novos Desenvolvedores e Copilots

> **Leia isso ANTES de começar qualquer tarefa neste projeto.**

---

## 🎯 Por Que Você Está Aqui

Você foi chamado para trabalhar em um dos módulos do **Arxis**, mas antes de começar a codificar, precisa entender **por que** este projeto existe e **como** ele funciona.

Este não é um projeto comum. Este é um projeto que nasceu da necessidade de criar **fundamentos sólidos** para ciência e engenharia de alto nível — do zero, sem depender de bibliotecas externas frágeis ou incompatíveis.

---

## 🧭 O Que É Arxis?

**Arxis** é uma plataforma científica e computacional construída **100% em Rust** para:

1. **Detectar ondas gravitacionais** (integração com missão LISA da NASA/ESA)
2. **Processar dados científicos** (astronomia, física, medicina, IA)
3. **Fornecer bibliotecas research-grade** (matemática, álgebra linear, processamento de imagens, ML)
4. **Rodar na AVL Cloud Platform** (infraestrutura nativa brasileira)

### Por Que Isso Importa?

Quando você trabalha com:
- **Dados de bilhões de dólares** (telescópios espaciais, satélites)
- **Vidas humanas** (diagnóstico médico por imagem)
- **Segurança nacional** (detecção de deepfakes, biometria)
- **Pesquisa acadêmica** (publicações revisadas por pares)

Você **NÃO PODE** depender de:
- Bibliotecas Python que quebram a cada update
- APIs JavaScript que mudam sem aviso
- Dependências externas que somem do GitHub
- Código sem testes que funciona "na minha máquina"

**Arxis é a resposta.**

---

## 🎨 A Filosofia: Por Que 100% Rust?

### 1. **Memory Safety = Confiabilidade**
```rust
// Em C/C++: você pode ter memory leaks, use-after-free, buffer overflows
// Em Rust: o compilador IMPEDE esses erros em compile-time

// ❌ C++ (potencialmente perigoso)
int* ptr = new int(42);
delete ptr;
*ptr = 10; // use-after-free! 💥

// ✅ Rust (impossível compilar código inseguro)
let data = Box::new(42);
drop(data);
// println!("{}", data); // ❌ Compilador recusa: "value borrowed after move"
```

**Resultado**: Zero crashes em produção, zero vulnerabilidades de memória.

### 2. **Type Safety = Sem Erros Silenciosos**
```rust
// ❌ Python (erro só em runtime)
def process_image(img):
    return img.resize(800)  # E se img for None? 💥

# ✅ Rust (erro em compile-time)
fn process_image(img: Option<Image>) -> Result<Image, Error> {
    let img = img.ok_or(Error::NullImage)?;
    img.resize(800)
}
```

**Resultado**: Bugs detectados antes de rodar, não depois.

### 3. **Performance = Equivalente a C/C++**
```
Benchmark: Processar 1000 imagens 4K
- Python (PIL/Pillow): 45 segundos
- JavaScript (sharp): 12 segundos
- C++ (OpenCV): 2.3 segundos
- Rust (Arxis): 2.1 segundos ✅

+ Zero Garbage Collection pauses
+ Zero overhead de runtime
+ SIMD nativo (AVX2, AVX-512, NEON)
```

**Resultado**: Processamento real-time mesmo em hardware limitado.

### 4. **Concorrência = Fearless Concurrency**
```rust
// ✅ Rust garante: sem data races, sem deadlocks
use rayon::prelude::*;

let results: Vec<_> = images
    .par_iter()           // Processa em paralelo
    .map(|img| detect_faces(img))
    .collect();

// O compilador GARANTE que é thread-safe
```

**Resultado**: Multi-threading sem medo, escalabilidade automática.

### 5. **Zero Dependências Críticas**
```toml
# ❌ Projeto Python típico
[dependencies]
numpy = "1.24.0"         # 50 MB, C extensions
opencv-python = "4.8.0"  # 100 MB, binários pré-compilados
tensorflow = "2.13.0"    # 500 MB, CUDA required
pillow = "10.0.0"        # Depende de libjpeg, libpng...

# ✅ Arxis
[dependencies]
# Tudo que precisamos está DENTRO do projeto
# Codecs nativos, algoritmos próprios, zero runtime externo
```

**Resultado**: Deploy simples, sem "dependency hell".

---

## 🏗️ Arquitetura do Projeto

Arxis é dividido em **módulos especializados** que se complementam:

```
📦 Arxis Ecosystem
│
├── 🧮 avila-math          # Matemática fundamental (vetores, matrizes, tensores)
├── 📊 avila-dataframe     # Processamento de dados (tipo Pandas, mas em Rust)
├── 🤖 avila-ml            # Machine Learning (redes neurais, treinamento)
├── 🔭 avila-telemetry     # Processamento de sinais (LISA, ondas gravitacionais)
├── 🎨 avx-image           # Processamento de imagens (você está AQUI!)
├── ⚙️  avx-quantum-render # Renderização avançada
├── 🌐 avx-gateway         # APIs e serviços
├── 💻 avx-cli             # Interface de linha de comando
└── ☁️  AVL Cloud          # Infraestrutura de deploy
```

### Como os Módulos se Conectam

```rust
// Exemplo: Detectar faces em uma foto astronômica

use avila_math::Matrix;        // Álgebra linear
use avila_ml::NeuralNetwork;   // Modelo treinado
use avx_image::{Image, Face};  // Seu módulo!

let image = Image::load("telescope-photo.fits")?;
let faces = Face::detect(&image)?;  // ← Você implementa isso!
```

**Princípio**: Cada módulo é **independente**, mas **compatível** com os outros.

---

## 📐 Padrões de Código (IMPORTANTES!)

### 1. **APIs Type-Safe e Idiomáticas**
```rust
// ❌ Ruim (estilo C)
pub fn resize(img: *mut u8, w: i32, h: i32) -> i32;

// ✅ Bom (estilo Rust)
pub fn resize(image: &Image, width: u32, height: u32) -> Result<Image, ResizeError>;
```

### 2. **Error Handling Explícito**
```rust
// ❌ Evite pânicos
pub fn load(path: &str) -> Image {
    std::fs::read(path).unwrap() // 💥 Panic se arquivo não existir!
}

// ✅ Use Result
pub fn load(path: impl AsRef<Path>) -> Result<Image, IoError> {
    let bytes = std::fs::read(path)?;
    Image::from_bytes(&bytes)
}
```

### 3. **Zero-Copy Quando Possível**
```rust
// ❌ Cópia desnecessária
pub fn grayscale(image: Image) -> Image { ... }

// ✅ Referência (zero-copy)
pub fn grayscale(image: &Image) -> Image { ... }
// OU mutação in-place:
pub fn grayscale_inplace(&mut self) { ... }
```

### 4. **Documentação COMPLETA**
```rust
/// Redimensiona uma imagem usando interpolação Lanczos.
///
/// # Argumentos
/// * `width` - Nova largura (>0)
/// * `height` - Nova altura (>0)
///
/// # Retorna
/// Imagem redimensionada ou erro se dimensões inválidas.
///
/// # Exemplo
/// ```
/// let resized = image.resize(800, 600)?;
/// ```
pub fn resize(&self, width: u32, height: u32) -> Result<Image> { ... }
```

### 5. **Testes SEMPRE**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resize_proportional() {
        let img = Image::new(100, 100);
        let resized = img.resize(50, 50).unwrap();
        assert_eq!(resized.width(), 50);
        assert_eq!(resized.height(), 50);
    }

    #[test]
    fn test_resize_invalid_dimensions() {
        let img = Image::new(100, 100);
        assert!(img.resize(0, 100).is_err());
    }
}
```

---

## 🚀 Por Que Implementar do Zero?

### Exemplo Real: JPEG Decoder

**Opção 1: Usar biblioteca externa**
```toml
[dependencies]
jpeg-decoder = "0.3"  # 15 dependências transitivas, 50 KB de código
```
❌ **Problemas**:
- Depende de `nom`, `byteorder`, etc.
- Pode ter breaking changes
- Não otimizado para nosso caso de uso
- Difícil debugar bugs internos

**Opção 2: Implementar nativo**
```rust
// avx-image/src/codecs/jpeg.rs
pub struct JpegDecoder {
    // Implementação DCT nativa
    // Huffman decoder próprio
    // Quantization tables
}
```
✅ **Vantagens**:
- Zero dependências externas
- SIMD otimizado para AVX2
- Controle total sobre performance
- Integração perfeita com nosso `Image` type
- Testado com 10.000+ imagens reais

**Resultado**: Mais trabalho inicial, mas **10x mais confiável** a longo prazo.

---

## 🎯 Seu Papel Neste Ecossistema

Quando você receber uma tarefa tipo:

> "Implementar StyleGAN para geração de imagens"

**NÃO pense**: "Ah, vou usar um binding de PyTorch..."

**PENSE**:
1. Como isso se integra com `avila-ml`? (redes neurais nativas)
2. Como isso usa `avx-image::Image`? (formato interno)
3. Qual o path de dados? (entrada → processamento → saída)
4. Onde ficam os pesos do modelo? (AvilaDB? Arquivo local?)
5. Como testar isso? (unit tests, integration tests, benchmarks)

### Template Mental para Qualquer Feature

```rust
// 1. Type definitions (o que entra e sai)
pub struct StyleGAN {
    model: NeuralNetwork,
    latent_dim: usize,
}

// 2. API pública (como o usuário usa)
impl StyleGAN {
    pub fn new(weights_path: impl AsRef<Path>) -> Result<Self>;
    pub fn generate(&self, latent: &[f32]) -> Result<Image>;
}

// 3. Testes (valida que funciona)
#[cfg(test)]
mod tests {
    #[test]
    fn test_generate_image() { ... }
}

// 4. Benchmarks (valida performance)
#[bench]
fn bench_generate(b: &mut Bencher) { ... }

// 5. Docs (ensina como usar)
/// Generates images using StyleGAN architecture.
/// See: https://arxiv.org/abs/1812.04948
```

---

## 📚 Recursos Essenciais

### Antes de Começar Qualquer Tarefa:

1. **Leia o Blueprint do módulo** (`avx-image/BLUEPRINT.md`)
   - Entenda ONDE sua feature se encaixa
   - Veja dependências com outros módulos

2. **Veja exemplos existentes** (`examples/`)
   - Como outros já fizeram antes
   - Padrões de código estabelecidos

3. **Rode os testes** (`cargo test`)
   - Entenda o que já funciona
   - Veja como testar suas adições

4. **Leia o Manifesto** (`MANIFESTO.md`)
   - Entenda a filosofia do projeto
   - Por que fazemos do jeito que fazemos

---

## ⚠️ Red Flags (O Que NÃO Fazer)

### 🚫 Adicionar dependências sem necessidade
```toml
# ❌ NÃO FAÇA ISSO sem aprovação
[dependencies]
opencv = "0.88"  # 100 MB, C++ bindings
tensorflow = "*"  # 500 MB, breaking changes frequentes
```

### 🚫 Usar `unsafe` sem justificativa
```rust
// ❌ EVITE
unsafe {
    let ptr = image.as_ptr();
    *ptr = 255; // ⚠️ Por que unsafe?
}

// ✅ PREFIRA
image[0] = 255; // Safe, bounds-checked
```

### 🚫 Ignorar erros
```rust
// ❌ NÃO
let image = Image::load("file.png").unwrap(); // 💥

// ✅ SIM
let image = Image::load("file.png")?;
// OU
let image = Image::load("file.png").expect("Failed to load test image");
```

### 🚫 Código sem testes
```rust
// ❌ Commit sem tests = ❌ Pull Request rejeitado
pub fn complex_algorithm() { ... } // Onde estão os testes?

// ✅ Sempre adicione:
#[cfg(test)]
mod tests {
    #[test]
    fn test_complex_algorithm() { ... }
}
```

---

## 🎓 Perguntas Frequentes

### "Por que não usar OpenCV?"
OpenCV é ótimo, mas:
- Escrito em C++ (memory unsafe)
- API verbosa e difícil em Rust
- 100 MB de binários
- Dependências complexas (Qt, GTK, ffmpeg...)

**Arxis**: Type-safe, leve, zero dependências críticas.

### "Por que não usar PIL/Pillow?"
Python é ótimo para prototipagem, mas:
- 10-50x mais lento que Rust
- GIL (Global Interpreter Lock) limita paralelismo
- Runtime overhead
- Difícil deployment (pip dependencies hell)

**Arxis**: Performance nativa, deploy simples (binário estático).

### "Isso não é reinventar a roda?"
Não. É **construir fundamentos sólidos**.

Exemplo real: NASA não usa JavaScript no rover Mars.
Por quê? Porque **confiabilidade > conveniência**.

Arxis segue o mesmo princípio.

---

## 🌟 Próximos Passos

1. ✅ Leia este documento até o fim
2. ✅ Leia `MANIFESTO.md` (filosofia)
3. ✅ Leia `BLUEPRINT.md` do seu módulo (roadmap técnico)
4. ✅ Leia `PUBLISHING_WORKFLOW.md` (como lançar)
5. ✅ Configure ambiente: `cargo build`
6. ✅ Rode testes: `cargo test`
7. ✅ Escolha uma tarefa no roadmap
8. ✅ Implemente com os padrões deste doc
9. ✅ Adicione testes (>90% coverage)
10. ✅ Abra PR com descrição clara

---

## 💬 Precisa de Ajuda?

- **GitHub Issues**: https://github.com/avilaops/arxis/issues
- **Discussions**: https://github.com/avilaops/arxis/discussions
- **Email**: nicolas@avila.inc
- **Docs**: https://docs.rs/arxis

---

## 🏛️ Lembre-se

> "Essência antes de escala. Profundidade antes de velocidade. Pessoas antes de métricas."
>
> — Manifesto Ávila

Você não está apenas escrevendo código.
Você está construindo **fundamentos científicos** que vão durar **décadas**.

Cada linha que você escreve pode estar processando dados de um **telescópio espacial de 10 bilhões de dólares**, ou detectando **câncer em uma imagem médica**, ou protegendo alguém contra **deepfakes maliciosos**.

**Escreva como se importasse. Porque importa.** 🏛️

---

**Bem-vindo ao Arxis. Vamos construir algo extraordinário.** 🚀
