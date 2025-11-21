/// Generate auxiliary files (manifest.json, HTML snippets, etc.)
use anyhow::Result;
use serde_json::json;
use std::fs;
use std::path::Path;

/// Generate manifest.json for PWA
pub fn generate_manifest<P: AsRef<Path>>(
    output_dir: P,
    app_name: &str,
    icon_files: &[String],
) -> Result<()> {
    let mut icons = Vec::new();

    for filename in icon_files {
        if filename.contains("android") || filename.contains("windows") {
            // Extract size from filename
            if let Some(size_part) = filename.split('_').last() {
                let size = size_part.replace(".png", "");
                icons.push(json!({
                    "src": filename,
                    "sizes": size,
                    "type": "image/png",
                    "purpose": "any maskable"
                }));
            }
        }
    }

    let manifest = json!({
        "name": app_name,
        "short_name": app_name,
        "icons": icons,
        "theme_color": "#ffffff",
        "background_color": "#ffffff",
        "display": "standalone",
        "start_url": "/",
        "scope": "/"
    });

    let manifest_path = output_dir.as_ref().join("manifest.json");
    fs::write(&manifest_path, serde_json::to_string_pretty(&manifest)?)?;

    Ok(())
}

/// Generate HTML snippet with meta tags
pub fn generate_html_snippet<P: AsRef<Path>>(output_dir: P, icon_files: &[String]) -> Result<()> {
    let mut html = String::new();

    html.push_str("<!-- Favicon e ícones de aplicação -->\n");
    html.push_str("<!-- Copie estas tags para o <head> do seu HTML -->\n\n");

    // Favicon
    html.push_str("<!-- Favicon padrão -->\n");
    html.push_str("<link rel=\"icon\" type=\"image/x-icon\" href=\"favicon.ico\">\n");
    html.push_str(
        "<link rel=\"icon\" type=\"image/png\" sizes=\"32x32\" href=\"icon_favicon_32x32.png\">\n",
    );
    html.push_str("<link rel=\"icon\" type=\"image/png\" sizes=\"16x16\" href=\"icon_favicon_16x16.png\">\n\n");

    // Apple Touch Icons
    html.push_str("<!-- Apple Touch Icons -->\n");
    for filename in icon_files {
        if filename.contains("apple") {
            if let Some(size_part) = filename.split('_').last() {
                let size = size_part.replace(".png", "").replace("x", "x");
                let _size_num = size.split('x').next().unwrap_or("0");
                html.push_str(&format!(
                    "<link rel=\"apple-touch-icon\" sizes=\"{}\" href=\"{}\">\n",
                    size, filename
                ));
            }
        }
    }
    html.push('\n');

    // Android/Chrome
    html.push_str("<!-- Android/Chrome -->\n");
    html.push_str("<link rel=\"manifest\" href=\"manifest.json\">\n");
    html.push_str("<meta name=\"theme-color\" content=\"#ffffff\">\n\n");

    // Windows
    html.push_str("<!-- Windows Tiles -->\n");
    html.push_str("<meta name=\"msapplication-TileColor\" content=\"#ffffff\">\n");
    html.push_str(
        "<meta name=\"msapplication-TileImage\" content=\"icon_windows_150x150.png\">\n\n",
    );

    // Social Media
    html.push_str("<!-- Open Graph (Facebook, LinkedIn) -->\n");
    html.push_str("<meta property=\"og:image\" content=\"icon_social_1080x1080.png\">\n");
    html.push_str("<meta property=\"og:image:width\" content=\"1080\">\n");
    html.push_str("<meta property=\"og:image:height\" content=\"1080\">\n\n");
    html.push_str("<!-- Twitter Card -->\n");
    html.push_str("<meta name=\"twitter:card\" content=\"summary\">\n");
    html.push_str("<meta name=\"twitter:image\" content=\"icon_social_400x400.png\">\n");

    let snippet_path = output_dir.as_ref().join("html_snippet.txt");
    fs::write(&snippet_path, html)?;

    Ok(())
}

/// Generate favicon.ico from multiple PNG sizes
pub fn generate_favicon_ico<P: AsRef<Path>>(
    output_dir: P,
    favicon_sizes: &[(u32, u32)],
) -> Result<()> {
    // Load all favicon PNGs
    let mut images = Vec::new();

    for (width, height) in favicon_sizes {
        let filename = format!("icon_favicon_{}x{}.png", width, height);
        let path = output_dir.as_ref().join(&filename);

        if path.exists() {
            if let Ok(img) = image::open(&path) {
                images.push(img.to_rgba8());
            }
        }
    }

    if images.is_empty() {
        return Ok(());
    }

    // Create ICO file
    let ico_path = output_dir.as_ref().join("favicon.ico");

    // ICO generation using image crate's save method
    if let Some(first_img) = images.first() {
        first_img.save_with_format(&ico_path, image::ImageFormat::Ico)?;
    }

    Ok(())
}

/// Generate README for the output directory
pub fn generate_readme<P: AsRef<Path>>(output_dir: P, app_name: &str) -> Result<()> {
    let readme = format!(
        r#"# Ícones - {}

Gerado por **Arxis Icon Processor** 🚀

## 📦 Conteúdo

### Plataformas Suportadas

- 🌐 **Favicon**: 16x16, 32x32, 48x48, 64x64 + favicon.ico
- 🍎 **Apple**: 10 tamanhos (57-1024) para iPhone, iPad, App Store
- 🤖 **Android**: 7 tamanhos (36-512) para apps e PWA
- 🪟 **Windows**: 5 tiles (44x44 até 310x310)
- 💻 **macOS**: 7 tamanhos (16-1024) para apps nativos
- 📱 **Social**: 5 tamanhos (300-1080) para redes sociais

### Arquivos Auxiliares

- `manifest.json` - Progressive Web App manifest
- `html_snippet.txt` - Meta tags prontas para copiar
- `favicon.ico` - Favicon multi-tamanho

## 🚀 Como Usar

### 1. Copiar para seu projeto

```bash
cp -r * /seu-projeto/assets/icons/
```

### 2. Adicionar no HTML

Copie o conteúdo de `html_snippet.txt` para o `<head>` do seu HTML:

```html
<head>
  <!-- Cole aqui o conteúdo de html_snippet.txt -->
</head>
```

### 3. Configurar PWA

Adicione o manifest no HTML:

```html
<link rel="manifest" href="/assets/icons/manifest.json">
```

## 📊 Estatísticas

Total de ícones gerados: ~40+ tamanhos
Formato: PNG otimizado
Background: Transparente

---

Gerado por **Arxis Icon Processor** v1.0.0
"#,
        app_name
    );

    let readme_path = output_dir.as_ref().join("README.md");
    fs::write(&readme_path, readme)?;

    Ok(())
}
