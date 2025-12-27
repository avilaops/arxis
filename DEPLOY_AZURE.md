# 🚀 GUIA DE DEPLOY PARA O AZURE

## ✅ STATUS ATUAL

### 📦 CÓDIGO NO GITHUB
- ✅ Push concluído para https://github.com/avilaops/Arxis
- ✅ GitHub Actions iniciado automaticamente
- ✅ Todos os arquivos enviados (Email, Analytics, Clarity, Reset)

---

## 🔧 CONFIGURAÇÃO NECESSÁRIA NO AZURE

### 1️⃣ Configurar Variáveis de Ambiente - API Backend

**Acesse:**
1. https://portal.azure.com
2. **App Services** → **Arxis-API** → **Settings** → **Configuration**
3. Clique em **"+ New application setting"** para cada variável abaixo:

```
Clarity__ApiToken
= eyJhbGciOiJSUzI1NiIsImtpZCI6IjQ4M0FCMDhFNUYwRDMxNjdEOTRFMTQ3M0FEQTk2RTcyRDkwRUYwRkYiLCJ0eXAiOiJKV1QifQ.eyJqdGkiOiIzOTBjNWE2ZS05OTEwLTRkYmEtOWE1YS0wYzkwNjlmN2VlMGYiLCJzdWIiOiIzMTI1NzcyNDAwOTEzNTI5Iiwic2NvcGUiOiJEYXRhLkV4cG9ydCIsIm5iZiI6MTc2NjgzMDI5MywiZXhwIjo0OTIwNDMwMjkzLCJpYXQiOjE3NjY4MzAyOTMsImlzcyI6ImNsYXJpdHkiLCJhdWQiOiJjbGFyaXR5LmRhdGEtZXhwb3J0ZXIifQ.cSpYi8XSGd-ZWFJUHO2GpvYkUheg1FsWMb1eSh5Qt1XiKvsSyG3UUgJDnQRuiBJiXFmn_Abrgq2KNmQ5qiQ913P5YzbEhxiYJVRw4WmVlKww_0yo4YX1SXULRMShUNi_ur5NQVLqZPddEGAk-8RC__kz104drN5z_W0NSERojpkdTYLzjvScHc1wOatONs4CLLFiSxp8KYNU-YiFJG0s7iyfwJbSy2ZbQ18JWoQclYAlj2zCUF30uDf_Ewdih8Ls0N2FY8-60RazBrpNqic9kfd4PpUI46vLZhscfaBp6S1a-p5_USP4VdhjEGOuYDjQ8_xMc1E3QnVUF_HQm-spFQ

Clarity__ProjectId
= urzs0mg9yx

Email__SmtpHost
= smtp.porkbun.com

Email__SmtpPort
= 587

Email__EnableSsl
= true

Email__FromAddress
= nicolas@avila.inc

Email__FromName
= Nícolas Ávila - ARXIS

Email__SmtpUser
= nicolas@avila.inc

Email__SmtpPassword
= 7Aciqgr7@3278579
```

4. Clique em **"Save"** no topo
5. O App Service vai reiniciar automaticamente

---

## 📊 VERIFICAR DEPLOY

### GitHub Actions
**URL:** https://github.com/avilaops/Arxis/actions

Você verá:
- ✅ **Deploy Backend to Azure** - Deploy da API
- ✅ **Azure Static Web Apps CI/CD** - Deploy do Frontend

### Acompanhar Deploy:
1. Clique no workflow em execução
2. Veja os logs em tempo real
3. Aguarde conclusão (~ 2-5 minutos)

---

## 🌐 URLS DA APLICAÇÃO

### Backend API:
```
https://arxis-api.azurewebsites.net
https://arxis-api.azurewebsites.net/swagger
https://arxis-api.azurewebsites.net/health
```

### Frontend:
```
https://kind-sand-04db77a1e.1.azurestaticapps.net
```

### Domínio Customizado (se configurado):
```
https://arxis.avila.inc
```

---

## ✅ CHECKLIST DE VALIDAÇÃO

Após deploy completo, teste:

### API:
- [ ] https://arxis-api.azurewebsites.net/health retorna `Healthy`
- [ ] https://arxis-api.azurewebsites.net/swagger abre documentação
- [ ] POST /api/auth/login funciona
- [ ] POST /api/email/send-welcome funciona

### Frontend:
- [ ] Site carrega corretamente
- [ ] Login funciona
- [ ] Clarity está rastreando (verificar em clarity.microsoft.com)

### Analytics:
- [ ] Application Insights recebendo telemetria
- [ ] Clarity mostrando sessões
- [ ] Endpoints de analytics retornando dados

---

## 🐛 TROUBLESHOOTING

### API não inicia:
1. Verifique logs: App Service → Monitoring → Log stream
2. Confirme variáveis configuradas: Configuration → Application settings
3. Reinicie: Overview → Restart

### Frontend 404:
1. Verifique GitHub Actions do Static Web App
2. Confirme rota no Azure Static Web App
3. Veja logs no portal

### Email não envia:
1. Teste SMTP: Use endpoint /api/email/test
2. Verifique logs do App Service
3. Confirme credenciais Porkbun

### Clarity não rastreia:
1. Verifique se script está no index.html
2. Confirme Project ID: urzs0mg9yx
3. Limpe cache do navegador

---

## 📝 PRÓXIMOS PASSOS

1. ✅ Configurar variáveis no Azure (acima)
2. ⏳ Aguardar deploy do GitHub Actions (2-5 min)
3. ✅ Testar aplicação nos URLs acima
4. ✅ Verificar analytics no Clarity
5. ✅ Criar primeiro usuário ou resetar senha

---

## 🔗 LINKS IMPORTANTES

- **Portal Azure:** https://portal.azure.com
- **GitHub Repo:** https://github.com/avilaops/Arxis
- **GitHub Actions:** https://github.com/avilaops/Arxis/actions
- **Clarity Dashboard:** https://clarity.microsoft.com/projects/view/urzs0mg9yx
- **App Insights:** Portal Azure → Application Insights → Arxis

---

## 🎯 COMANDOS ÚTEIS

### Ver logs em tempo real:
```bash
# Instalar Azure CLI (se não tiver)
winget install Microsoft.AzureCLI

# Login
az login

# Ver logs
az webapp log tail --name Arxis-API --resource-group Arxis
```

### Reiniciar serviços:
```bash
# Reiniciar API
az webapp restart --name Arxis-API --resource-group Arxis

# Reiniciar Static Web App (via portal)
```

---

**🚀 TUDO PRONTO! Agora é só configurar as variáveis e testar!**
