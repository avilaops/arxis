# GitHub Pages Setup Guide

## Configuração do GitHub Pages para browser.avila.inc

### Passo 1: Ativar GitHub Pages

1. Acesse: https://github.com/avilaops/arxis/settings/pages
2. Em **Source**, selecione: `Deploy from a branch`
3. Em **Branch**, selecione: `main`
4. Em **Folder**, selecione: `/avila-browser/docs`
5. Clique em **Save**

### Passo 2: Configurar Custom Domain

1. Na mesma página de Settings > Pages
2. Em **Custom domain**, digite: `browser.avila.inc`
3. Clique em **Save**
4. Aguarde verificação DNS

### Passo 3: Configurar DNS (no seu provedor de domínio)

Adicione os seguintes registros DNS para `avila.inc`:

```
Type: CNAME
Name: browser
Value: avilaops.github.io
TTL: 3600
```

**OU** se usar apex domain:

```
Type: A
Name: browser
Values:
  185.199.108.153
  185.199.109.153
  185.199.110.153
  185.199.111.153
TTL: 3600
```

### Passo 4: Ativar HTTPS

1. Volte para Settings > Pages
2. Marque **Enforce HTTPS**
3. Aguarde certificado SSL ser provisionado (pode levar alguns minutos)

### Passo 5: Verificar Deploy

Após alguns minutos, acesse:
- https://browser.avila.inc
- https://avilaops.github.io/arxis/avila-browser/docs/

O site deve estar disponível!

### Arquivos Importantes

- `/avila-browser/docs/index.html` - Landing page principal
- `/avila-browser/docs/CNAME` - Configuração do custom domain
- `/avila-browser/.github/workflows/deploy.yml` - GitHub Actions para deploy automático

### Deploy Automático

O GitHub Actions já está configurado para fazer deploy automático a cada push na branch `main`.

Workflow: `.github/workflows/deploy.yml`

### Troubleshooting

**Problema: 404 Not Found**
- Verifique se o path está correto: `/avila-browser/docs`
- Verifique se o CNAME file existe

**Problema: DNS não resolve**
- Aguarde propagação DNS (pode levar até 48h)
- Verifique configuração no provedor de DNS

**Problema: HTTPS não funciona**
- Aguarde provisionamento do certificado (até 24h)
- Verifique se DNS está correto

### Links Úteis

- Documentação GitHub Pages: https://docs.github.com/en/pages
- Status do GitHub: https://www.githubstatus.com/
- DNS Checker: https://dnschecker.org/

### Próximos Passos

1. Criar release no GitHub com os binários
2. Atualizar links de download na landing page
3. Adicionar Google Analytics (opcional)
4. Adicionar sitemap.xml e robots.txt
