# 📦 INSTALAÇÃO RÁPIDA - PROJETO ARXIS

## 🎯 Para instalar o projeto em qualquer máquina

### Opção 1: Instalação Completa Automática
```powershell
# Execute no PowerShell como Administrador
.\INSTALADOR-ARXIS.ps1 -ConfigureAll -TargetDrive "E:"
```

### Opção 2: Instalação Personalizada
```powershell
# Apenas copiar projeto (sem instalar Rust/VSCode)
.\INSTALADOR-ARXIS.ps1 -TargetDrive "E:" -SkipGitClone

# Instalar apenas Rust
.\INSTALADOR-ARXIS.ps1 -InstallRust

# Instalar apenas VSCode
.\INSTALADOR-ARXIS.ps1 -InstallVSCode
```

## 📋 Parâmetros Disponíveis

| Parâmetro | Descrição | Padrão |
|-----------|-----------|--------|
| `-TargetDrive` | Drive de destino | `E:` |
| `-ProjectName` | Nome da pasta | `arxis` |
| `-SkipGitClone` | Copiar ao invés de clonar | `false` |
| `-InstallRust` | Instalar Rust | `false` |
| `-InstallVSCode` | Instalar VSCode | `false` |
| `-ConfigureAll` | Instalar tudo automaticamente | `false` |

## 🔧 O que o instalador faz

1. ✅ Verifica requisitos (Git, Rust, Cargo, VSCode)
2. ✅ Instala Rust (se necessário)
3. ✅ Instala VSCode (se necessário)
4. ✅ Clona/copia o repositório
5. ✅ Instala extensões VSCode necessárias
6. ✅ Configura Rust toolchain
7. ✅ Compila módulos de fundação
8. ✅ Cria scripts de atalho

## 📁 Estrutura após instalação

```
E:\arxis\
├── notebook1-fundacao.code-workspace
├── notebook2-matematica.code-workspace
├── notebook3-data-ml.code-workspace
├── notebook4-database-cloud.code-workspace
├── notebook5-advanced.code-workspace
├── notebook6-coordenacao.code-workspace
├── NOTEBOOK1-MANIFESTO.md
├── NOTEBOOK2-MANIFESTO.md
├── NOTEBOOK3-MANIFESTO.md
├── NOTEBOOK4-MANIFESTO.md
├── NOTEBOOK5-MANIFESTO.md
├── NOTEBOOK6-MANIFESTO.md
├── ABRIR-TODOS-NOTEBOOKS.ps1
├── STATUS-MODULOS.ps1
└── 82 pastas de módulos (avila-*, avl-*, avx-*)
```

## 🚀 Após instalação

Execute os scripts de atalho:

```powershell
# Ver status de compilação
.\STATUS-MODULOS.ps1

# Abrir todos os 6 notebooks
.\ABRIR-TODOS-NOTEBOOKS.ps1

# Ou abrir apenas um específico
code notebook1-fundacao.code-workspace
```

## ⚙️ Requisitos Mínimos

- **SO:** Windows 10/11
- **RAM:** 16GB (recomendado 32GB para 6 notebooks simultâneos)
- **Disco:** 50GB livres
- **Internet:** Para clonar repositório e baixar dependências

## 🛠️ Troubleshooting

### Erro: "cannot be loaded because running scripts is disabled"
```powershell
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
```

### Erro: Rust não encontrado após instalação
```powershell
# Reinicie o PowerShell ou execute:
$env:PATH += ";$env:USERPROFILE\.cargo\bin"
```

### Erro: Compilação falha
```powershell
# Atualizar Rust para versão mais recente
rustup update stable
rustup default stable
```

## 📞 Suporte

- GitHub: https://github.com/avilaops/arxis
- Issues: https://github.com/avilaops/arxis/issues
- Documentação: Leia os 6 manifestos (NOTEBOOK*-MANIFESTO.md)
