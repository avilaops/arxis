# 📘 GitHub Pages - Guia Completo

## 🎯 Objetivo

Hospedar a landing page do Arxis no GitHub Pages com domínio customizado `arxis.avilaops.com`.

---

## 📦 Opções de Deploy

### Opção 1: Deploy Manual (Recomendado - Mais Simples)

**Vantagens:**
- Setup super rápido (5 minutos)
- Sem configuração adicional
- Deploy automático via push

**Etapas:**

1. **Criar/usar repositório** (sugestão: `avilaops/arxis`)

2. **Push dos arquivos:**
```powershell
cd "c:\Users\nicol\OneDrive\Avila\1.2 - Avilaops\1.2.7 - Identidade visual\Arxis\landing-page"
git init
git add .
git commit -m "Add Arxis landing page"
git branch -M main
git remote add origin https://github.com/avilaops/arxis.git
git push -u origin main
```

3. **Ativar Pages:**
   - Ir em: Settings → Pages
   - Source: Deploy from a branch
   - Branch: `main` / `/(root)` ou `/landing-page`
   - Save

4. **Domínio customizado:**
   - Custom domain: `arxis.avilaops.com`
   - Save (cria CNAME automaticamente)
   - ✅ Enforce HTTPS

5. **Configurar DNS:**
```
Cloudflare (ou outro provedor):
Type: CNAME
Name: arxis
Target: avilaops.github.io
Proxy: ON
```

✅ **Pronto!** Site em: `https://arxis.avilaops.com`

---

### Opção 2: Deploy com GitHub Actions (Avançado)

**Vantagens:**
- Controle total do processo
- Build customizado
- Testes automáticos

**Desvantagens:**
- Mais complexo
- Requer configuração adicional

**Não recomendado para este projeto** (página estática simples)

---

## 🗂️ Estrutura do Repositório

### Opção A: Landing page na raiz
```
arxis/
├── index.html
├── styles.css
├── script.js
├── CNAME
└── ...
```

**GitHub Pages config:**
- Branch: `main`
- Folder: `/(root)`

---

### Opção B: Landing page em subpasta (ATUAL)
```
arxis/
├── landing-page/
│   ├── index.html
│   ├── styles.css
│   ├── script.js
│   ├── CNAME
│   └── ...
├── Cargo.toml
├── src/
└── ...
```

**GitHub Pages config:**
- Branch: `main`
- Folder: `/landing-page`

⚠️ **Importante**: Se usar subpasta, o CNAME deve estar dentro de `landing-page/`

---

## 🔧 Configuração DNS Detalhada

### Cloudflare

1. Dashboard → Domínio `avilaops.com`
2. DNS → Add record:
   ```
   Type: CNAME
   Name: arxis
   Target: avilaops.github.io
   Proxy status: Proxied (🟠 ON)
   TTL: Auto
   ```
3. Save

**Tempo de propagação**: ~5-10 minutos

---

### Outros Provedores (GoDaddy, Namecheap, etc)

```
Type: CNAME
Host: arxis
Points to: avilaops.github.io
TTL: 3600 (ou mínimo)
```

**Tempo de propagação**: 24-48 horas

---

## ✅ Verificação

### 1. Verificar DNS
```powershell
nslookup arxis.avilaops.com
```

Deve retornar: `avilaops.github.io`

### 2. Verificar HTTPS
```
https://arxis.avilaops.com
```

Deve mostrar cadeado verde 🔒

### 3. Verificar Build
```
https://github.com/avilaops/arxis/actions
```

Deve mostrar ✅ workflows bem-sucedidos

---

## 🔄 Workflow de Atualização

```powershell
# 1. Fazer mudanças nos arquivos
code landing-page/index.html

# 2. Testar localmente
cd landing-page
python -m http.server 8000

# 3. Commit e push
git add .
git commit -m "Update: adicionar nova seção"
git push

# 4. Aguardar deploy automático (1-2 minutos)
# 5. Verificar em: https://arxis.avilaops.com
```

---

## 🐛 Troubleshooting

### Problema: 404 Not Found

**Causas:**
- Pasta/branch incorretos
- Deploy ainda processando
- CNAME mal configurado

**Soluções:**
```
1. Verificar Settings → Pages → Source
2. Aguardar 10 minutos
3. Verificar se CNAME existe no repo
4. Checar Actions para erros de build
```

---

### Problema: Domínio não resolve

**Causas:**
- DNS não propagado
- CNAME incorreto
- Domínio não adicionado no GitHub

**Soluções:**
```powershell
# Verificar DNS
nslookup arxis.avilaops.com

# Verificar CNAME no repo
cat landing-page/CNAME

# Aguardar propagação (5min-48h)
```

---

### Problema: CSS/JS não carregam

**Causas:**
- Paths absolutos no HTML
- Cache do browser
- CORS issues

**Soluções:**
```
1. Usar paths relativos: href="styles.css" (sem /)
2. Hard refresh: Ctrl+Shift+R
3. Verificar Console (F12) para erros
```

---

### Problema: HTTPS não funciona

**Causas:**
- Certificado ainda provisionando
- "Enforce HTTPS" não marcado

**Soluções:**
```
1. Aguardar até 24h após adicionar domínio
2. Settings → Pages → ✅ Enforce HTTPS
3. Se persistir, remover e readicionar domínio
```

---

## 📊 Monitoramento

### GitHub Actions
```
URL: https://github.com/avilaops/arxis/actions
Verificar: builds com ✅
```

### Analytics (Opcional)

**Google Analytics:**
```html
<!-- Adicionar no <head> do index.html -->
<script async src="https://www.googletagmanager.com/gtag/js?id=G-XXXXXXXXXX"></script>
```

**GitHub Traffic:**
```
Repo → Insights → Traffic
```

---

## 🎯 Checklist Final

Antes de considerar pronto:

- [ ] Site carrega em `https://arxis.avilaops.com`
- [ ] HTTPS funcionando (cadeado verde)
- [ ] Todas as seções visíveis
- [ ] Links funcionando
- [ ] Responsivo (testar mobile)
- [ ] Console sem erros (F12)
- [ ] Performance > 90 (PageSpeed)
- [ ] DNS propagado globalmente
- [ ] Build do GitHub bem-sucedido

---

## 🚀 Deploy Checklist

```powershell
# 1. Verificar arquivos
ls landing-page/

# 2. Verificar CNAME
cat landing-page/CNAME
# Deve mostrar: arxis.avilaops.com

# 3. Commit tudo
git status
git add .
git commit -m "Prepare for GitHub Pages deploy"

# 4. Push
git push origin main

# 5. Ativar Pages (primeira vez)
# Settings → Pages → main / /landing-page

# 6. Adicionar domínio customizado
# Custom domain: arxis.avilaops.com

# 7. Configurar DNS
# CNAME: arxis → avilaops.github.io

# 8. Aguardar e verificar
# https://arxis.avilaops.com
```

---

## 📞 Suporte

**GitHub Pages Docs:**
- https://docs.github.com/pages

**Status GitHub:**
- https://www.githubstatus.com/

**Contato:**
- Email: nicolas@avila.inc
- WhatsApp: +55 17 99781-1471

---

**✅ Setup completo para GitHub Pages!**

**Próximo passo:** Executar os comandos de deploy e configurar DNS.
