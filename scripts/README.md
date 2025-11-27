# 🚀 Script de Publicação Automatizada

Script PowerShell para publicar automaticamente as crates do workspace Arxis no crates.io.

## 📋 Pré-requisitos

1. **Rust e Cargo** instalados
2. **Token do crates.io** configurado:
   ```powershell
   cargo login
   ```
3. **PowerShell 5.1+** (Windows) ou **PowerShell Core 7+** (cross-platform)

## 🎯 Uso

### Modo Dry-Run (Simulação)

Testa tudo sem publicar de verdade:

```powershell
.\scripts\publish-crates.ps1 -DryRun
```

### Publicar Fase 1 (Crates Críticas)

```powershell
.\scripts\publish-crates.ps1 -Fase Fase1
```

**Crates da Fase 1:**
1. avila-dataframe (CRÍTICA)
2. avila-ml (CRÍTICA)
3. avx-api-core (CRÍTICA)
4. avx-gateway (CRÍTICA)
5. avila-geo (ALTA)

### Publicar Fase 2 (Crates Médias)

```powershell
.\scripts\publish-crates.ps1 -Fase Fase2
```

**Crates da Fase 2:**
6. avila-reduction
7. avila-tokenizer
8. avx-gpu

### Publicar Fase 3 (Crates Baixa Prioridade)

```powershell
.\scripts\publish-crates.ps1 -Fase Fase3
```

**Crates da Fase 3:**
9. avl-loadbalancer
10. avx-quantum-render

### Publicar TODAS as Fases

```powershell
.\scripts\publish-crates.ps1 -Fase Todas
```

### Pular Testes (não recomendado)

```powershell
.\scripts\publish-crates.ps1 -Fase Fase1 -SkipTests
```

## 🔍 O que o Script Faz

Para cada crate:

1. ✅ Verifica se já está publicada (pula se sim)
2. ✅ Valida existência de `Cargo.toml`
3. ✅ Executa testes (`cargo test --all-features`)
4. ✅ Executa clippy (`cargo clippy --all-features`)
5. ✅ Gera documentação (`cargo doc --no-deps`)
6. ✅ Publica no crates.io (`cargo publish`)
7. ⏳ Aguarda 30s para indexação

## 📊 Relatórios

O script gera:
- ✅ Log detalhado de cada etapa
- ✅ Resumo final (sucessos/falhas)
- ✅ Lista de crates que falharam

## 🔒 Segurança

- ⚠️ **Solicita confirmação** antes de publicar (exceto em dry-run)
- ⚠️ Para imediatamente se algum teste falhar
- ⚠️ Valida cada crate antes de publicar

## 🎨 Exemplos

### Testar Fase 1 sem publicar

```powershell
.\scripts\publish-crates.ps1 -DryRun -Fase Fase1
```

### Publicar Fase 1 pulando testes (emergência)

```powershell
.\scripts\publish-crates.ps1 -Fase Fase1 -SkipTests
```

### Publicar tudo de uma vez (cuidado!)

```powershell
.\scripts\publish-crates.ps1 -Fase Todas
```

## 📅 Cronograma Recomendado

### Semana 1 (até 1 Dez 2025)
```powershell
.\scripts\publish-crates.ps1 -DryRun -Fase Fase1  # Testar
.\scripts\publish-crates.ps1 -Fase Fase1          # Publicar
```

### Semanas 2-3 (8-21 Dez 2025)
```powershell
.\scripts\publish-crates.ps1 -DryRun -Fase Fase2  # Testar
.\scripts\publish-crates.ps1 -Fase Fase2          # Publicar
```

### Semana 4+ (22 Dez 2025+)
```powershell
.\scripts\publish-crates.ps1 -DryRun -Fase Fase3  # Testar
.\scripts\publish-crates.ps1 -Fase Fase3          # Publicar
```

## 🐛 Troubleshooting

### "Crate already exists"
A crate já foi publicada. O script pula automaticamente.

### "Tests failed"
- Rode `cargo test` manualmente no diretório da crate
- Corrija os testes antes de publicar
- Use `-SkipTests` apenas em emergências

### "Cargo.toml not found"
O diretório da crate não existe ou está mal estruturado.

### "Rate limit exceeded"
Aguarde alguns minutos. O crates.io tem limite de requisições.

## 📞 Suporte

**Maintainer**: Nícolas Ávila
**Email**: nicolas@avila.inc
**GitHub**: https://github.com/avilaops/arxis

## 📄 Licença

MIT OR Apache-2.0
