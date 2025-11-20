# 🏛️ Arxis Landing Page

Landing page oficial do **Arxis** - The Mathematical Citadel

**URL**: [arxis.avilaops.com](https://arxis.avilaops.com)
**Hospedagem**: GitHub Pages

---

## 📋 Sobre

Landing page moderna e responsiva para o projeto **Arxis**, biblioteca Rust de física e matemática de nível científico. Design inspirado no tema "Cosmic Deep Space" com animações e interatividade.

**Deploy via GitHub Pages** - Configuração automática com domínio customizado.

### Características

- ✅ Design responsivo (mobile, tablet, desktop)
- ✅ Animações suaves e interativas
- ✅ Tema dark com paleta "Cosmic Deep Space"
- ✅ SEO otimizado com meta tags
- ✅ Performance otimizada
- ✅ Acessibilidade (WCAG 2.1 AA)
- ✅ Exemplos de código com syntax highlighting
- ✅ Parallax effects e animações astronômicas

---

## 🎨 Paleta de Cores

### Cores Principais
- **Deep Blue** (Background): `#0A1628`
- **Gravitational Wave Cyan** (Accent): `#00D4FF`
- **Quantum Gold** (Highlights): `#FFB800`
- **Neutron White** (Text): `#E8F1F5`
- **Event Horizon Purple** (Secondary): `#8B4FE8`

### Gradientes
```css
--gradient-primary: linear-gradient(135deg, #00D4FF, #8B4FE8);
--gradient-secondary: linear-gradient(135deg, #8B4FE8, #00D4FF);
```

---

## 📁 Estrutura de Arquivos

```
landing-page/
├── index.html          # Página principal
├── styles.css          # Estilos CSS
├── script.js           # JavaScript/interatividade
├── README.md           # Este arquivo
├── assets/             # Assets estáticos
│   ├── favicon.svg
│   ├── favicon.ico
│   ├── og-image.png
│   └── twitter-card.png
└── .htaccess          # Configuração Apache (opcional)
```

---

## 🚀 Deploy no GitHub Pages

### Setup Inicial

1. **Criar repositório** (se ainda não existir):
   - Nome sugerido: `arxis` ou `arxis-website`
   - Visibilidade: Public (para GitHub Pages gratuito)

2. **Push dos arquivos**:
```powershell
cd landing-page
git init
git add .
git commit -m "Initial commit - Arxis landing page"
git branch -M main
git remote add origin https://github.com/avilaops/arxis.git
git push -u origin main
```

3. **Ativar GitHub Pages**:
   - Ir em **Settings** → **Pages**
   - Source: Deploy from a branch
   - Branch: `main` / `/(root)`
   - Save

4. **Configurar domínio customizado**:
   - Em **Custom domain**: `arxis.avilaops.com`
   - Save (vai criar arquivo `CNAME` automaticamente)

5. **Habilitar HTTPS**:
   - ✅ Enforce HTTPS (marcar checkbox)

### Configuração DNS

No seu provedor DNS (ex: Cloudflare):

```
Type: CNAME
Name: arxis
Target: avilaops.github.io
Proxy: ON (recomendado)
TTL: Auto
```

**Aguardar**: Propagação DNS pode levar até 24-48h (geralmente 5-10min com Cloudflare)

### Verificar Deploy

- URL temporária: `https://avilaops.github.io/arxis/`
- URL final: `https://arxis.avilaops.com`

---

## 🔄 Atualizar Site (após mudanças)

```powershell
cd landing-page
git add .
git commit -m "Update: descrição das mudanças"
git push
```

GitHub Pages faz deploy automático em ~1-2 minutos.

---

## 📦 Alternativas de Deploy (Backup)

### Opção 2: Netlify

1. Fazer login em [netlify.com](https://netlify.com)
2. Clicar em "Add new site" → "Import an existing project"
3. Conectar ao GitHub e selecionar o repositório
4. Configurar:
   - **Build command**: (deixar vazio)
   - **Publish directory**: `landing-page`
5. Deploy!

**Domínio customizado**:
- Em Netlify: Settings → Domain Management → Add custom domain
- Adicionar `arxis.avilaops.com`
- Configurar DNS:
  - Tipo: `CNAME`
  - Nome: `arxis`
  - Valor: `<seu-site>.netlify.app`

### Opção 3: Vercel

1. Fazer login em [vercel.com](https://vercel.com)
2. Importar projeto do GitHub
3. Configurar:
   - **Root Directory**: `landing-page`
   - **Framework Preset**: Other
4. Deploy!

**Domínio customizado**:
- Settings → Domains → Add Domain
- Adicionar `arxis.avilaops.com`
- Seguir instruções de DNS

### Opção 4: AVL Cloud (Avila Cloud Platform)

1. Fazer login na plataforma AVL
2. Criar novo site estático
3. Upload dos arquivos ou connect ao GitHub
4. Configurar domínio `arxis.avilaops.com`
5. Deploy automático

### Opção 5: Servidor Próprio (Apache/Nginx)

#### Apache

```apache
# .htaccess
<IfModule mod_rewrite.c>
    RewriteEngine On
    RewriteBase /

    # Redirect to HTTPS
    RewriteCond %{HTTPS} off
    RewriteRule ^(.*)$ https://%{HTTP_HOST}%{REQUEST_URI} [L,R=301]

    # Remove .html extension
    RewriteCond %{REQUEST_FILENAME} !-f
    RewriteRule ^([^\.]+)$ $1.html [NC,L]
</IfModule>

# Compression
<IfModule mod_deflate.c>
    AddOutputFilterByType DEFLATE text/html text/css text/javascript application/javascript
</IfModule>

# Browser Caching
<IfModule mod_expires.c>
    ExpiresActive On
    ExpiresByType image/svg+xml "access plus 1 year"
    ExpiresByType text/css "access plus 1 month"
    ExpiresByType application/javascript "access plus 1 month"
</IfModule>
```

#### Nginx

```nginx
server {
    listen 80;
    server_name arxis.avilaops.com;
    return 301 https://$server_name$request_uri;
}

server {
    listen 443 ssl http2;
    server_name arxis.avilaops.com;

    root /var/www/arxis;
    index index.html;

    # SSL Configuration
    ssl_certificate /path/to/cert.pem;
    ssl_certificate_key /path/to/key.pem;

    # Compression
    gzip on;
    gzip_types text/css application/javascript image/svg+xml;

    # Caching
    location ~* \.(css|js|svg|png|jpg|jpeg|gif|ico)$ {
        expires 1y;
        add_header Cache-Control "public, immutable";
    }

    # SPA routing
    location / {
        try_files $uri $uri/ /index.html;
    }
}
```

---

## 🔧 Configuração de DNS

Para configurar o subdomínio `arxis.avilaops.com`:

### Cloudflare (Recomendado)

1. Fazer login em [cloudflare.com](https://cloudflare.com)
2. Selecionar domínio `avilaops.com`
3. DNS → Add record:
   - **Type**: `CNAME`
   - **Name**: `arxis`
   - **Target**: Seu servidor ou CDN
   - **Proxy status**: Proxied (laranja) ✅
   - **TTL**: Auto

### DNS Direto (sem CDN)

```
Type: CNAME
Host: arxis
Value: <seu-servidor>.com
TTL: 3600
```

ou

```
Type: A
Host: arxis
Value: <IP do servidor>
TTL: 3600
```

---

## 🧪 Testes Locais

### Servidor HTTP Simples

**Python:**
```powershell
cd landing-page
python -m http.server 8000
```

**Node.js:**
```powershell
npx http-server landing-page -p 8000
```

**PHP:**
```powershell
cd landing-page
php -S localhost:8000
```

Acesse: `http://localhost:8000`

---

## ✅ Checklist Pré-Deploy

- [ ] Testar em navegadores: Chrome, Firefox, Safari, Edge
- [ ] Testar responsividade: Mobile, Tablet, Desktop
- [ ] Validar HTML: [validator.w3.org](https://validator.w3.org/)
- [ ] Validar CSS: [jigsaw.w3.org/css-validator](https://jigsaw.w3.org/css-validator/)
- [ ] Testar performance: [PageSpeed Insights](https://pagespeed.web.dev/)
- [ ] Verificar SEO: [web.dev/measure](https://web.dev/measure/)
- [ ] Testar acessibilidade: [WAVE](https://wave.webaim.org/)
- [ ] Verificar links quebrados
- [ ] Criar assets:
  - [ ] favicon.svg (32×32, 64×64)
  - [ ] favicon.ico (multi-resolution)
  - [ ] og-image.png (1200×630)
  - [ ] twitter-card.png (1200×600)
- [ ] Configurar HTTPS/SSL
- [ ] Configurar domínio customizado
- [ ] Adicionar Google Analytics (opcional)

---

## 📊 Métricas de Performance

Alvo (PageSpeed Insights):
- **Performance**: 95+
- **Accessibility**: 100
- **Best Practices**: 95+
- **SEO**: 100

---

## 🎯 SEO

### Meta Tags Configuradas

```html
<!-- Primary Meta Tags -->
<title>Arxis - The Mathematical Citadel | Research-Grade Physics in Rust</title>
<meta name="description" content="...">
<meta name="keywords" content="...">

<!-- Open Graph / Facebook -->
<meta property="og:type" content="website">
<meta property="og:url" content="https://arxis.avilaops.com/">
<meta property="og:title" content="...">
<meta property="og:description" content="...">
<meta property="og:image" content="...">

<!-- Twitter -->
<meta property="twitter:card" content="summary_large_image">
<meta property="twitter:url" content="...">
<meta property="twitter:title" content="...">
<meta property="twitter:description" content="...">
<meta property="twitter:image" content="...">
```

### Sitemap XML

Criar arquivo `sitemap.xml`:
```xml
<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
  <url>
    <loc>https://arxis.avilaops.com/</loc>
    <lastmod>2025-11-20</lastmod>
    <changefreq>weekly</changefreq>
    <priority>1.0</priority>
  </url>
</urlset>
```

### robots.txt

```
User-agent: *
Allow: /
Sitemap: https://arxis.avilaops.com/sitemap.xml
```

---

## 🔒 Segurança

### Content Security Policy

Adicionar ao `<head>`:
```html
<meta http-equiv="Content-Security-Policy" content="
  default-src 'self';
  script-src 'self' 'unsafe-inline';
  style-src 'self' 'unsafe-inline' https://fonts.googleapis.com;
  font-src 'self' https://fonts.gstatic.com;
  img-src 'self' data: https:;
  connect-src 'self';">
```

### Headers de Segurança (.htaccess)

```apache
# Security Headers
Header set X-XSS-Protection "1; mode=block"
Header set X-Frame-Options "SAMEORIGIN"
Header set X-Content-Type-Options "nosniff"
Header set Referrer-Policy "strict-origin-when-cross-origin"
Header set Permissions-Policy "geolocation=(), microphone=(), camera=()"
```

---

## 📱 PWA (Progressive Web App) - Opcional

### manifest.json

```json
{
  "name": "Arxis - The Mathematical Citadel",
  "short_name": "Arxis",
  "description": "Research-Grade Physics & Mathematics in Rust",
  "start_url": "/",
  "display": "standalone",
  "background_color": "#0A1628",
  "theme_color": "#00D4FF",
  "icons": [
    {
      "src": "assets/icon-192.png",
      "sizes": "192x192",
      "type": "image/png"
    },
    {
      "src": "assets/icon-512.png",
      "sizes": "512x512",
      "type": "image/png"
    }
  ]
}
```

### Service Worker (sw.js)

```javascript
const CACHE_NAME = 'arxis-v1';
const urlsToCache = [
  '/',
  '/styles.css',
  '/script.js',
  '/assets/favicon.svg'
];

self.addEventListener('install', event => {
  event.waitUntil(
    caches.open(CACHE_NAME)
      .then(cache => cache.addAll(urlsToCache))
  );
});

self.addEventListener('fetch', event => {
  event.respondWith(
    caches.match(event.request)
      .then(response => response || fetch(event.request))
  );
});
```

---

## 🎨 Criação de Assets

### Favicon SVG

Já incluído no HTML como logo inline. Para criar arquivo separado:

```svg
<!-- assets/favicon.svg -->
<svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">
  <defs>
    <linearGradient id="grad" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" style="stop-color:#00D4FF;stop-opacity:1" />
      <stop offset="100%" style="stop-color:#8B4FE8;stop-opacity:1" />
    </linearGradient>
  </defs>
  <circle cx="50" cy="25" r="8" fill="url(#grad)" />
  <circle cx="75" cy="50" r="8" fill="url(#grad)" />
  <circle cx="50" cy="75" r="8" fill="url(#grad)" />
  <circle cx="25" cy="50" r="8" fill="url(#grad)" />
  <line x1="50" y1="25" x2="75" y2="50" stroke="url(#grad)" stroke-width="3"/>
  <line x1="75" y1="50" x2="50" y2="75" stroke="url(#grad)" stroke-width="3"/>
  <line x1="50" y1="75" x2="25" y2="50" stroke="url(#grad)" stroke-width="3"/>
  <line x1="25" y1="50" x2="50" y2="25" stroke="url(#grad)" stroke-width="3"/>
</svg>
```

### OG Image / Twitter Card

Criar em Figma, Canva ou similar:
- **Tamanho**: 1200×630px (OG) / 1200×600px (Twitter)
- **Background**: Deep Blue (#0A1628)
- **Logo**: Centro, tamanho grande
- **Texto**: "ARXIS - Research-Grade Physics in Rust"
- **Gradiente**: Cyan → Purple

---

## 🐛 Troubleshooting

### Estilos não carregam
- Verificar caminho do `styles.css` no HTML
- Verificar MIME type no servidor
- Limpar cache do navegador (Ctrl+Shift+R)

### JavaScript não funciona
- Abrir Console (F12) e verificar erros
- Verificar se `script.js` está carregando
- Verificar se está usando HTTPS (algumas APIs requerem)

### Fontes Google não carregam
- Verificar conexão com internet
- Verificar CSP (Content Security Policy)
- Usar fontes locais como fallback

### Animações lentas
- Reduzir complexidade das animações
- Usar `will-change` CSS property
- Testar em diferentes dispositivos

---

## 📞 Suporte

**Email**: nicolas@avila.inc
**WhatsApp**: +55 17 99781-1471
**GitHub**: [@avilaops](https://github.com/avilaops)
**Website**: [avila.cloud](https://avila.cloud)

---

## 📄 Licença

Landing page desenvolvida para o projeto **Arxis**.

- **Código**: MIT OR Apache-2.0
- **Assets**: CC BY-SA 4.0

---

## 🎉 Créditos

**Design & Development**: Nicolas Ávila (Avila)
**Projeto**: Arxis - The Mathematical Citadel
**Data**: Novembro 2025

---

**🏛️ Built by Avila**
