# 🛠️ Comandos Úteis - Arxis Landing Page

## 🧪 Testes Locais

### Servidor HTTP Simples

**Python 3:**
```powershell
cd landing-page
python -m http.server 8000
```

**Node.js (http-server):**
```powershell
npx http-server landing-page -p 8000 -c-1
```

**PHP:**
```powershell
cd landing-page
php -S localhost:8000
```

**Acesso:** http://localhost:8000

---

## ✅ Validação

### HTML Validator
```powershell
# Online: https://validator.w3.org/
# Upload index.html ou cole URL
```

### CSS Validator
```powershell
# Online: https://jigsaw.w3.org/css-validator/
# Upload styles.css ou cole URL
```

### JavaScript Lint
```powershell
# Online: https://jshint.com/
# Cole código do script.js
```

---

## 📊 Performance

### PageSpeed Insights
```powershell
# Online: https://pagespeed.web.dev/
# Inserir: https://arxis.avilaops.com
```

### Lighthouse (Chrome DevTools)
```
1. Abrir site no Chrome
2. F12 (DevTools)
3. Aba "Lighthouse"
4. "Generate report"
```

### WebPageTest
```powershell
# Online: https://www.webpagetest.org/
# URL: https://arxis.avilaops.com
```

---

## 🔍 SEO Testing

### Google Search Console
```
1. https://search.google.com/search-console
2. Adicionar propriedade: arxis.avilaops.com
3. Verificar propriedade (DNS ou HTML)
4. Submeter sitemap: https://arxis.avilaops.com/sitemap.xml
```

### Meta Tags Tester
```powershell
# Open Graph: https://www.opengraph.xyz/
# Twitter Cards: https://cards-dev.twitter.com/validator
# Meta Tags: https://metatags.io/
```

---

## 🖼️ Asset Optimization

### Imagens

**TinyPNG (compress):**
```powershell
# Online: https://tinypng.com/
# Upload PNG/JPG files
# Download optimized versions
```

**ImageOptim (macOS):**
```bash
brew install imageoptim-cli
imageoptim assets/*.png
```

**Squoosh (online):**
```
https://squoosh.app/
```

### SVG

**SVGO (optimize SVG):**
```powershell
npx svgo assets/favicon.svg -o assets/favicon.min.svg
```

---

## 🌐 DNS Testing

### Verificar propagação DNS
```powershell
nslookup arxis.avilaops.com
```

### Verificar CNAME
```powershell
nslookup -type=CNAME arxis.avilaops.com
```

### DNS Checker Global
```
https://dnschecker.org/
Digite: arxis.avilaops.com
```

### Flush DNS Cache (Windows)
```powershell
ipconfig /flushdns
```

---

## 🔒 SSL/HTTPS Testing

### SSL Labs
```
https://www.ssllabs.com/ssltest/
Analyze: arxis.avilaops.com
```

### Check SSL Certificate
```powershell
# PowerShell
$url = "https://arxis.avilaops.com"
$req = [System.Net.HttpWebRequest]::Create($url)
$req.GetResponse()
```

---

## 📦 Compression Testing

### GZIP Test
```powershell
# Online: https://www.giftofspeed.com/gzip-test/
# URL: https://arxis.avilaops.com
```

### Check Response Headers
```powershell
curl -I https://arxis.avilaops.com
```

---

## 🐛 Debugging

### Chrome DevTools

**Network Tab:**
```
1. F12 → Network
2. Reload page (Ctrl+R)
3. Check load times, sizes
```

**Console:**
```
1. F12 → Console
2. Check for errors
```

**Lighthouse:**
```
1. F12 → Lighthouse
2. Select categories
3. Generate report
```

### Firefox DevTools
```
F12 → Network/Console/Inspector
```

---

## 📱 Mobile Testing

### Chrome Device Toolbar
```
1. F12 (DevTools)
2. Ctrl+Shift+M (Toggle device toolbar)
3. Select device (iPhone, iPad, etc)
```

### BrowserStack
```powershell
# Online: https://www.browserstack.com/
# Test on real devices
```

### Responsive Design Checker
```
https://responsivedesignchecker.com/
URL: https://arxis.avilaops.com
```

---

## 🚀 Deploy Commands

### Git

**Initial commit:**
```powershell
cd landing-page
git init
git add .
git commit -m "Initial commit - Arxis landing page"
git branch -M main
git remote add origin https://github.com/avilaops/arxis-landing.git
git push -u origin main
```

**Update:**
```powershell
git add .
git commit -m "Update landing page"
git push
```

### Netlify CLI

**Install:**
```powershell
npm install -g netlify-cli
```

**Login:**
```powershell
netlify login
```

**Deploy:**
```powershell
cd landing-page
netlify deploy --prod
```

### Vercel CLI

**Install:**
```powershell
npm install -g vercel
```

**Deploy:**
```powershell
cd landing-page
vercel --prod
```

---

## 📊 Analytics

### Google Analytics Setup
```html
<!-- Add to <head> in index.html -->
<script async src="https://www.googletagmanager.com/gtag/js?id=GA_MEASUREMENT_ID"></script>
<script>
  window.dataLayer = window.dataLayer || [];
  function gtag(){dataLayer.push(arguments);}
  gtag('js', new Date());
  gtag('config', 'GA_MEASUREMENT_ID');
</script>
```

### Plausible Analytics (Privacy-friendly)
```html
<!-- Add to <head> -->
<script defer data-domain="arxis.avilaops.com" src="https://plausible.io/js/script.js"></script>
```

---

## 🔧 Maintenance

### Check for Broken Links
```powershell
# Online: https://www.deadlinkchecker.com/
# URL: https://arxis.avilaops.com
```

### Check Accessibility
```powershell
# WAVE: https://wave.webaim.org/
# URL: https://arxis.avilaops.com
```

### Check Mobile-Friendliness
```powershell
# Google: https://search.google.com/test/mobile-friendly
# URL: https://arxis.avilaops.com
```

---

## 📈 Monitoring

### Uptime Monitor (Free)
```
UptimeRobot: https://uptimerobot.com/
Add monitor: https://arxis.avilaops.com
Check every: 5 minutes
```

### Performance Monitor
```
PageSpeed Insights API
Lighthouse CI
Web Vitals tracking
```

---

## 🎨 Design Tools

### Favicon Generator
```
https://favicon.io/favicon-converter/
Upload logo → Generate → Download
```

### OG Image Generator
```
https://www.opengraph.xyz/
Design card → Export PNG
```

### Gradient Generator
```
https://cssgradient.io/
Create gradient → Copy CSS
```

### Color Picker
```
https://coolors.co/
Generate palettes
```

---

## 📚 Resources

### Documentation
- [MDN Web Docs](https://developer.mozilla.org/)
- [Web.dev](https://web.dev/)
- [Can I Use](https://caniuse.com/)

### Learning
- [CSS-Tricks](https://css-tricks.com/)
- [JavaScript.info](https://javascript.info/)
- [A11y Project](https://www.a11yproject.com/)

### Tools
- [Lighthouse](https://github.com/GoogleChrome/lighthouse)
- [Webpack](https://webpack.js.org/)
- [Vite](https://vitejs.dev/)

---

## 🆘 Quick Fixes

### CSS not loading
```powershell
# Hard refresh
Ctrl + Shift + R (Windows/Linux)
Cmd + Shift + R (Mac)
```

### JavaScript errors
```powershell
# Check console
F12 → Console
# Check file path in Network tab
```

### Images not showing
```powershell
# Check path
# Check file extension (case-sensitive on Linux)
# Verify file exists
```

### Mobile menu not working
```powershell
# Check viewport meta tag
# Test in Chrome DevTools mobile mode
# Check JavaScript errors in console
```

---

## ✨ Pro Tips

```powershell
# Open DevTools with page
chrome.exe --auto-open-devtools-for-tabs

# Disable cache in DevTools
F12 → Network → Disable cache (checkbox)

# Capture screenshot (full page)
F12 → Ctrl+Shift+P → "Capture full size screenshot"

# Find TODO comments
grep -r "TODO" .

# Check file sizes
Get-ChildItem -Recurse | Select-Object Name, Length | Sort-Object Length -Descending
```

---

**🏛️ Arxis Landing Page**
**Comandos para desenvolvimento, teste e deploy**
