# Quick Start - AVX Intelligence

## 🚀 Setup Inicial (5 minutos)

### Passo 1: Capturar logs pela primeira vez

Em **cada máquina**, abra PowerShell e execute:

```powershell
cd d:\arxis\avx-intelligence\scripts
.\capture-logs.ps1 -All
```

### Passo 2: Configurar captura automática

Agende captura a cada hora:

```powershell
# Criar tarefa agendada
$Action = New-ScheduledTaskAction -Execute "PowerShell.exe" -Argument "-File d:\arxis\avx-intelligence\scripts\capture-logs.ps1 -All"
$Trigger = New-ScheduledTaskTrigger -Daily -At 9am -RepetitionInterval (New-TimeSpan -Hours 1)
Register-ScheduledTask -TaskName "AVX-LogCapture" -Action $Action -Trigger $Trigger
```

### Passo 3: Exportar conversas do Copilot manualmente

Após cada sessão importante:
1. Abra VS Code Copilot Chat
2. `Ctrl+Shift+P` → "Export Chat History"
3. Salve em: `logs/copilots/{machine-id}/chat-{timestamp}.json`

### Passo 4: Sincronizar entre máquinas (opcional)

Se usar rede compartilhada:

```powershell
.\sync-machines.ps1 -Bidirectional
```

## 📊 Análise de Produtividade

### Ver métricas da última semana:

```powershell
# Contar commits
git log --since="7 days ago" --oneline | Measure-Object -Line

# Linhas adicionadas/removidas
git log --since="7 days ago" --numstat --pretty="%H" | awk 'NF==3 {plus+=$1; minus+=$2} END {print plus, minus}'

# Crates publicados
Get-ChildItem d:\arxis\*\Cargo.toml -Recurse | Select-String "version" | Measure-Object
```

## 🎯 Checklist Semanal

- [ ] Executar `capture-logs.ps1 -All` nas 3 máquinas
- [ ] Exportar conversas importantes do Copilot
- [ ] Revisar `logs/machines/*/metrics-*.json`
- [ ] Atualizar `targets.yml` com progresso
- [ ] Sincronizar máquinas com `sync-machines.ps1`
- [ ] Backup dos logs mais antigos de 30 dias

## 📈 Próximos Passos

1. **Implementar avx-inspector** (Semantic Kernel)
2. **Dashboard Grafana** para visualização
3. **Análise automática** de qualidade do código
4. **Benchmarks** semanais de performance
5. **Paper** sobre o método (target: ArXiv)

## 💡 Dicas

- Mantenha conversas do Copilot focadas e documentadas
- Use tags nos commits: `[avx]`, `[avila]`, `[nasa-ready]`
- Comente decisões importantes no código
- Salve outputs de benchmarks importantes
- Documente descobertas em `docs/insights/`

---
**Objetivo**: Dados suficientes para publicação científica até Mar 2026
