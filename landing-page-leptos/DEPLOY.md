# Landing Page Leptos - Instruções de Deploy

## 🚀 Como Rodar Localmente

```powershell
# 1. Instalar trunk (se ainda não tiver)
cargo install trunk

# 2. Adicionar target WASM
rustup target add wasm32-unknown-unknown

# 3. Rodar servidor de desenvolvimento
cd landing-page-leptos
trunk serve --open
```

Acesse: http://localhost:8080

## 📦 Build de Produção

```powershell
# Build otimizado
trunk build --release

# Os arquivos estarão em dist/
# Copiar para servidor web ou hospedar no GitHub Pages
```

## 🌐 Deploy GitHub Pages

1. Build:
```powershell
trunk build --release --public-url /arxis
```

2. Commit arquivos `dist/`:
```powershell
git add dist/
git commit -m "chore: Build landing page"
git push
```

3. Configurar GitHub Pages:
   - Settings → Pages
   - Source: Deploy from branch
   - Branch: main → /dist

## ✨ Funcionalidades

- 🌙/☀️ **Dark/Light Mode** - Toggle no navbar, persiste em localStorage
- 🇧🇷/🇺🇸 **PT-BR/EN** - Toggle de idioma no navbar
- 🎨 **Identidade Solar** - Branco, amarelo (#FFD700), laranja (#FF8C00)
- ⚡ **WASM** - Tudo compilado para WebAssembly
- 📱 **Responsive** - Mobile-first design

## 🛠️ Tecnologias

- Leptos 0.6 (Rust web framework)
- WebAssembly (wasm32-unknown-unknown)
- Trunk (WASM bundler)
- CSS Variables (theming)
- LocalStorage API (persistência)

---

**100% Rust, Zero JavaScript Manual**
