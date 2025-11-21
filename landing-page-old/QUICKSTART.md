# ⚡ GitHub Pages - Setup Rápido

## 🚀 Deploy em 3 Passos

### 1. Push para GitHub
```powershell
cd landing-page
git init
git add .
git commit -m "Initial commit - Arxis landing page"
git branch -M main
git remote add origin https://github.com/avilaops/arxis.git
git push -u origin main
```

### 2. Ativar GitHub Pages
1. Ir em: `https://github.com/avilaops/arxis/settings/pages`
2. **Source**: Deploy from a branch → **main** / **/(root)**
3. **Save**

### 3. Configurar Domínio
1. **Custom domain**: `arxis.avilaops.com` → Save
2. **DNS** (Cloudflare):
   ```
   Type: CNAME
   Name: arxis
   Target: avilaops.github.io
   ```

✅ **Pronto!** `https://arxis.avilaops.com`

---

## 🔄 Atualizar
```powershell
git add .
git commit -m "Update"
git push
```

Deploy automático em ~2 minutos!

---

## 📞 Problemas?

- **404**: Aguarde 10min ou verifique branch/pasta
- **DNS**: `nslookup arxis.avilaops.com`
- **Build**: github.com/avilaops/arxis/actions

Mais detalhes: `DEPLOY.md`
