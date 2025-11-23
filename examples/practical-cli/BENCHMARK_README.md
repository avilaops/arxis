# 🔥 Performance Testing Scripts

## 📋 Scripts Disponíveis

### 1. `quick-benchmark.ps1` ⚡
Benchmark rápido e seguro (COM Windows Defender ativo).

**Uso:**
```powershell
.\quick-benchmark.ps1
```

**O que faz:**
- 10 iterações de compressão
- Estatísticas (média, min, max)
- Detecta tipo de CPU e disco
- Recomendações automáticas

---

### 2. `disable-defender.ps1` 🛡️
Desabilita Windows Defender temporariamente.

**⚠️ REQUER ADMINISTRADOR!**

**Uso:**
```powershell
# Clique direito no PowerShell > Executar como Administrador
.\disable-defender.ps1
```

**O que faz:**
- Desabilita Real-Time Protection
- Desabilita Behavior Monitoring
- Desabilita IOAV Protection
- Desabilita Script Scanning

**Segurança:**
- Pede confirmação antes de executar
- Mostra status antes e depois
- Defender reativa automaticamente ao reiniciar

---

### 3. `extreme-benchmark.ps1` 🔥
Benchmark EXTREMO para medir performance real sem overhead.

**⚠️ Execute APÓS desabilitar o Defender!**

**Uso:**
```powershell
.\extreme-benchmark.ps1
```

**O que faz:**
- **Teste 1:** 20 iterações com arquivo pequeno (README.md)
- **Teste 2:** Arquivo de 10 MB (throughput real)
- **Teste 3:** STRESS TEST com 100 MB
- Comparação com LZ4/Zstd/7-Zip

**Resultados esperados:**
- Arquivo pequeno: 15-25 ms
- Arquivo 10 MB: 100-200 MB/s throughput
- Arquivo 100 MB: 200-500 MB/s throughput

---

### 4. `enable-defender.ps1` ✅
Reativa Windows Defender.

**⚠️ REQUER ADMINISTRADOR!**

**Uso:**
```powershell
.\enable-defender.ps1
```

---

## 🎯 Fluxo Completo

```powershell
# 1. Abrir PowerShell como Administrador
# Botão direito > Executar como Administrador

# 2. Navegar até o diretório
cd "C:\Users\nicol\OneDrive\Avila\1.2 - Avilaops\Arxis\examples\practical-cli"

# 3. Desabilitar Defender
.\disable-defender.ps1

# 4. Rodar benchmark extremo
.\extreme-benchmark.ps1

# 5. Reativar Defender
.\enable-defender.ps1
```

---

## 📊 Interpretando Resultados

### Throughput Esperado (LZ4):

| Cenário      | Throughput     | Veredicto                |
| ------------ | -------------- | ------------------------ |
| > 200 MB/s   | 🔥 ULTRA RÁPIDO | Rust nativo competitivo! |
| 100-200 MB/s | ✅ MUITO BOM    | Performance excelente    |
| 50-100 MB/s  | 👍 BOM          | Performance adequada     |
| < 50 MB/s    | ⚠️ MÉDIO        | Gargalo de I/O           |

### Comparação com Outras Ferramentas:

| Ferramenta       | Throughput Típico  |
| ---------------- | ------------------ |
| LZ4 (C)          | 300-500 MB/s       |
| Zstd (fast)      | 200-400 MB/s       |
| 7-Zip (LZMA)     | 20-50 MB/s         |
| **ARXIS (Rust)** | **200-500 MB/s** 🚀 |

---

## 🐛 Troubleshooting

### "Precisa ser executado como Administrador"
✅ Clique com botão direito no PowerShell e selecione **"Executar como Administrador"**

### "Execution Policy bloqueou o script"
```powershell
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
```

### Defender não desabilita
Algumas empresas bloqueiam via Group Policy. Tente desabilitar manualmente:
1. Windows Security
2. Virus & threat protection
3. Manage settings
4. Desligar "Real-time protection"

---

## 📚 Documentação Adicional

- **PERFORMANCE_GUIDE.md** - Guia completo de profiling
- **GUIA_DE_USO.md** - Tutorial de uso das ferramentas CLI

---

## ⚠️ IMPORTANTE

- **Sempre reative o Windows Defender após os testes!**
- Não deixe seu PC desprotegido
- Use apenas para testes de performance
- Defender reativa automaticamente ao reiniciar

---

**🔥 Agora você tem tudo para medir a VERDADEIRA performance! 🚀**
