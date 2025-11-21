# 🚀 Deploy Rápido - Arxis Landing Page (GitHub Pages)

## ⚡ Quick Start - GitHub Pages

### 1️⃣ Preparar Repositório

**Opção A: Novo Repositório**
```powershell
cd landing-page
git init
git add .
git commit -m "Initial commit - Arxis landing page"
git branch -M main
git remote add origin https://github.com/avilaops/arxis.git
git push -u origin main
```

**Opção B: Repositório Existente**
```powershell
cd landing-page
git add .
git commit -m "Add landing page"
git push
```

---

### 2️⃣ Ativar GitHub Pages

1. Ir em: `https://github.com/avilaops/arxis/settings/pages`
2. **Source**: Deploy from a branch
3. **Branch**: `main` / `/(root)`
4. **Save**
5. Aguardar ~2 minutos

✅ Site disponível em: `https://avilaops.github.io/arxis/`

---

### 3️⃣ Configurar Domínio Customizado

**No GitHub:**
1. Settings → Pages → Custom domain
2. Digite: `arxis.avilaops.com`
3. **Save** (cria arquivo `CNAME` automaticamente)
4. ✅ **Enforce HTTPS** (marcar)

**No DNS (Cloudflare, GoDaddy, etc):**
```
Type: CNAME
Name: arxis
Target: avilaops.github.io
Proxy/TTL: ON/Auto (se Cloudflare)
```

**Aguardar**: 5-10 minutos (Cloudflare) ou até 24-48h (outros DNS)

✅ **Pronto!** Acesse: `https://arxis.avilaops.com`

---

### 4️⃣ Atualizar Site (após mudanças)

```powershell
git add .
git commit -m "Update landing page"
git push
```

Deploy automático em ~1-2 minutos! 🚀

---

## 🧪 Testar Localmente (antes do deploy)

```powershell
cd landing-page
python -m http.server 8000
```

Acesse: `http://localhost:8000`

---

## 🔧 Troubleshooting GitHub Pages

### Site não carrega (404)
```
1. Verificar se branch está correto (main)
2. Verificar se pasta está correta (root /)
3. Aguardar até 10 minutos após ativar Pages
4. Verificar Actions: github.com/avilaops/arxis/actions
```

### Domínio customizado não funciona
```
1. Verificar CNAME no repositório
2. Verificar DNS com: nslookup arxis.avilaops.com
3. Aguardar propagação (5min-48h dependendo do provedor)
4. GitHub: Settings → Pages → verificar "DNS check successful"
```

### HTTPS não funciona
```
1. Aguardar até 24h após configurar domínio
2. Verificar se "Enforce HTTPS" está marcado
3. Se persistir, remover e readicionar domínio customizado
```

### CSS/JS não carregam
```
1. Verificar paths relativos (sem / no início)
2. Hard refresh: Ctrl+Shift+R
3. Verificar Console (F12) para erros
```

---

## 📊 Monitoramento

### Verificar Build do GitHub Pages
```
https://github.com/avilaops/arxis/actions
```

### Verificar DNS Propagação
```powershell
nslookup arxis.avilaops.com
```

Online: [dnschecker.org](https://dnschecker.org/)

---

## 🔄 Workflow Completo

```powershell
# 1. Fazer mudanças
code index.html

# 2. Testar localmente
python -m http.server 8000

# 3. Commit e push
git add .
git commit -m "Update: descrição"
git push

# 4. Aguardar deploy (~2min)
# 5. Verificar em: https://arxis.avilaops.com
```

---

## 📋 Checklist Pré-Deploy

- [ ] Testado em Chrome, Firefox, Safari
- [ ] Testado em mobile (responsivo)
- [ ] Todos os links funcionando
- [ ] Assets criados (favicon, og-image)
- [ ] Meta tags configuradas
- [ ] Performance testada (PageSpeed)

---

## 🎨 Assets Necessários

**Criar antes do deploy final:**

1. **favicon.ico** (16×16, 32×32, 48×48)
   - Use: [favicon.io](https://favicon.io/)

2. **og-image.png** (1200×630)
   - Design no Figma/Canva
   - Background: #0A1628
   - Logo + "ARXIS - Research-Grade Physics in Rust"

3. **twitter-card.png** (1200×600)
   - Similar ao og-image

Colocar em: `landing-page/assets/`

---

## 🔧 Configuração DNS Final

### Opção 1: Cloudflare (Recomendado)

```
Dashboard → avilaops.com → DNS → Add record

Type: CNAME
Name: arxis
Target: <seu-deploy>.netlify.app (ou vercel.app)
Proxy status: Proxied (ON)
TTL: Auto
```

### Opção 2: DNS Direto

```
Type: A
Name: arxis
Value: <IP do servidor>
TTL: 3600
```

---

## ✅ Verificação Final

Após deploy, verificar:

- [ ] Site acessível em `https://arxis.avilaops.com`
- [ ] HTTPS funcionando (cadeado verde)
- [ ] Todas as seções carregando
- [ ] Animações funcionando
- [ ] Links externos funcionando
- [ ] Responsividade mobile OK
- [ ] Performance > 90 (PageSpeed)
- [ ] OG tags funcionando (testar compartilhamento)

**Testar OG Tags:**
- [opengraph.xyz](https://www.opengraph.xyz/)
- [metatags.io](https://metatags.io/)

---

## 🐛 Problemas Comuns

### Site não carrega
- Verificar DNS (pode levar até 24h)
- Limpar cache: `ipconfig /flushdns` (Windows)
- Testar em modo anônimo

### CSS não aparece
- Verificar caminho do arquivo
- Hard refresh: Ctrl+Shift+R
- Verificar Console (F12) para erros

### Domínio não conecta
- Aguardar propagação DNS (até 48h)
- Verificar configuração no provider
- Testar com: `nslookup arxis.avilaops.com`

---

## 📞 Suporte

**Email**: nicolas@avila.inc
**WhatsApp**: +55 17 99781-1471

---

## 🎉 Sucesso!

Seu site está no ar em: **https://arxis.avilaops.com** 🚀

Próximos passos:
1. Anunciar nas redes sociais
2. Submeter ao Google Search Console
3. Compartilhar com a comunidade Rust
4. Adicionar Google Analytics (opcional)

**🏛️ Built by Avila**
