# ⚡ QUICK START - PENDRIVE BOOTABLE

## 🎯 Uso Imediato do Pendrive

1. **Conecte o pendrive** em qualquer PC Windows
2. **Abra PowerShell como Administrador**
3. **Navegue até a letra do pendrive:**
   ```powershell
   cd E:\  # ou F:\ dependendo da letra
   ```
4. **Execute o instalador:**
   ```powershell
   .\INSTALADOR-ARXIS.ps1 -ConfigureAll
   ```

Pronto! O projeto será configurado automaticamente.

---

## 📦 Conteúdo do Pendrive

```
📁 ARXIS (raiz do pendrive)
│
├── 📜 INSTALADOR-ARXIS.ps1          ← Script principal
├── 📜 README-INSTALACAO.md          ← Instruções detalhadas
├── 📜 QUICK-START-PENDRIVE.md       ← Este arquivo
│
├── 📁 notebook1-fundacao.code-workspace
├── 📁 notebook2-matematica.code-workspace
├── 📁 notebook3-data-ml.code-workspace
├── 📁 notebook4-database-cloud.code-workspace
├── 📁 notebook5-advanced.code-workspace
├── 📁 notebook6-coordenacao.code-workspace
│
├── 📄 NOTEBOOK1-MANIFESTO.md        ← Leia primeiro!
├── 📄 NOTEBOOK2-MANIFESTO.md
├── 📄 NOTEBOOK3-MANIFESTO.md
├── 📄 NOTEBOOK4-MANIFESTO.md
├── 📄 NOTEBOOK5-MANIFESTO.md
├── 📄 NOTEBOOK6-MANIFESTO.md
│
└── 📁 82 módulos (avila-*, avl-*, avx-*)
```

---

## ⚡ Comandos Rápidos

### Cenário 1: Nova máquina (sem nada instalado)
```powershell
# Instala TUDO: Rust + VSCode + Projeto
.\INSTALADOR-ARXIS.ps1 -ConfigureAll
```

### Cenário 2: Máquina já com Rust e VSCode
```powershell
# Apenas copia e configura projeto
.\INSTALADOR-ARXIS.ps1
```

### Cenário 3: Instalar em outro drive
```powershell
# Instalar em D:\ ao invés de C:\
.\INSTALADOR-ARXIS.ps1 -TargetDrive "D:" -ConfigureAll
```

### Cenário 4: Copiar do pendrive ao invés de clonar
```powershell
# Útil sem internet
.\INSTALADOR-ARXIS.ps1 -SkipGitClone -ConfigureAll
```

---

## 🎯 Após Instalação

O projeto estará em `C:\arxis` (ou drive escolhido).

**Abra o Notebook 1 para começar:**
```powershell
cd C:\arxis
code notebook1-fundacao.code-workspace
```

Ou abra **todos os 6 notebooks de uma vez:**
```powershell
.\ABRIR-TODOS-NOTEBOOKS.ps1
```

---

## 📊 Verificar Status

```powershell
.\STATUS-MODULOS.ps1
```

Mostra quantos módulos já foram compilados.

---

## 🧭 Ordem de Trabalho

1. **Notebook 1 - Fundação** ← COMECE AQUI
2. **Notebook 2 - Matemática** (aguarda 50% do N1)
3. **Notebook 3 - Data/ML** (aguarda 50% do N2)
4. **Notebook 4 - Database** (aguarda base estável)
5. **Notebook 5 - Advanced** (aguarda base estável)
6. **Notebook 6 - Coordenação** (você, supervisionando tudo)

---

## 💡 Dicas

- **Copilots:** Cada notebook abre 8 instâncias de Copilot (16 por notebook)
- **RAM:** 32GB recomendado para rodar os 6 notebooks simultaneamente
- **Ordem:** Sempre comece pelo Notebook 1 (fundação)
- **Manifestos:** Leia os manifestos antes de começar cada notebook

---

## ⚠️ Troubleshooting

### PowerShell não executa scripts
```powershell
Set-ExecutionPolicy RemoteSigned -Scope CurrentUser
```

### Rust não encontrado
```powershell
# Reinicie PowerShell ou adicione ao PATH:
$env:PATH += ";$env:USERPROFILE\.cargo\bin"
```

### Sem internet para clonar
```powershell
# Use o flag -SkipGitClone para copiar do pendrive
.\INSTALADOR-ARXIS.ps1 -SkipGitClone -ConfigureAll
```

---

## 🚀 Pronto para começar!

Execute: `.\INSTALADOR-ARXIS.ps1 -ConfigureAll`

Boa sorte com o desenvolvimento! 💪
