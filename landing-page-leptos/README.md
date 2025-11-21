# Arxis Landing Page - Leptos (Rust + WASM)

Landing page moderna construída 100% em **Rust** usando **Leptos** framework (WebAssembly).

## ✨ Recursos

- ✅ **Dark/Light Mode Toggle** - Tema claro/escuro com persistência em localStorage
- ✅ **PT-BR/EN Toggle** - Alternância entre português e inglês
- ✅ **Identidade Visual Solar** - Branco (#FFFFFF), Amarelo Solar (#FFD700), Laranja Estelar (#FF8C00)
- ✅ **100% Rust + WASM** - Zero JavaScript manual, tudo em Rust
- ✅ **Client-Side Rendering** - Renderização no cliente via WebAssembly
- ✅ **Responsive Design** - Layout totalmente responsivo

## 🚀 Como Rodar

### Pré-requisitos
```powershell
# Instalar trunk (bundler para WASM)
cargo install trunk

# Adicionar target wasm32
rustup target add wasm32-unknown-unknown
```

### Desenvolvimento
```powershell
cd landing-page-leptos
trunk serve --open
```

Acesse: `http://localhost:8080`

### Build de Produção
```powershell
trunk build --release
```

Os arquivos estarão em `dist/`

## 📁 Estrutura

```
landing-page-leptos/
├── src/
│   ├── lib.rs              # App principal
│   ├── main.rs             # Entry point
│   ├── theme.rs            # Sistema de temas (Light/Dark)
│   ├── i18n.rs             # Traduções (PT-BR/EN)
│   └── components/
│       ├── mod.rs
│       ├── navbar.rs       # Navbar com toggles
│       ├── hero.rs         # Hero section
│       ├── features.rs     # Feature cards
│       ├── architecture.rs # Arquitetura
│       ├── code_examples.rs# Exemplos de código
│       ├── stats.rs        # Estatísticas
│       ├── contact.rs      # Contato
│       └── footer.rs       # Footer
├── style.css               # Estilos com CSS variables
├── index.html              # HTML base
└── Cargo.toml
```

## 🎨 Tema de Cores

### Light Mode
- Background: `#FFFFFF` (branco)
- Texto: `#1A1A1A` (quase preto)
- Acentos: `#FF8C00` (laranja), `#FFD700` (amarelo)

### Dark Mode
- Background: `#0A0A0A` (preto profundo)
- Texto: `#FFFFFF` (branco)
- Acentos: `#FFD700` (amarelo solar)

## 🌐 Internacionalização

Toggles entre:
- 🇧🇷 **PT-BR** - Português Brasileiro
- 🇺🇸 **EN** - English

Traduções definidas em `src/i18n.rs`

## 🛠️ Tecnologias

- **Leptos 0.6** - Framework Rust para web apps
- **WebAssembly** - Compilação para WASM
- **LocalStorage API** - Persistência de preferências
- **CSS Variables** - Tema dinâmico
- **Trunk** - Build tool para Rust WASM

## 📦 Deploy

Gerar build:
```powershell
trunk build --release --public-url /arxis
```

Os arquivos em `dist/` podem ser servidos por qualquer servidor estático:
- GitHub Pages
- Netlify
- Vercel
- Cloudflare Pages

## 🏛️ Arquitetura

```
Rust Source (src/)
       ↓
Leptos Components (Reactive)
       ↓
WebAssembly (wasm32-unknown-unknown)
       ↓
Browser (DOM manipulation)
```

## 📄 Licença

MIT / Apache-2.0

---

**Built with 🦀 Rust + ⚡ WebAssembly**
