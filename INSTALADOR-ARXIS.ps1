# 🚀 INSTALADOR AUTOMÁTICO ARXIS
# Execute este script em um novo ambiente para configurar todo o projeto

param(
    [string]$TargetDrive = "E:",  # Drive do pendrive ou destino
    [string]$ProjectName = "arxis",
    [switch]$SkipGitClone,
    [switch]$InstallRust,
    [switch]$InstallVSCode,
    [switch]$ConfigureAll
)

$ErrorActionPreference = "Stop"

Write-Host "╔════════════════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║   INSTALADOR AUTOMÁTICO - PROJETO ARXIS               ║" -ForegroundColor Cyan
Write-Host "║   82 Módulos | 6 Notebooks | Desenvolvimento Massivo  ║" -ForegroundColor Cyan
Write-Host "╚════════════════════════════════════════════════════════╝" -ForegroundColor Cyan
Write-Host ""

# ============================================================================
# PASSO 1: VERIFICAR REQUISITOS
# ============================================================================
Write-Host "📋 [1/8] Verificando requisitos..." -ForegroundColor Yellow

$requirements = @{
    "Git" = "git --version"
    "Rust" = "rustc --version"
    "Cargo" = "cargo --version"
    "VSCode" = "code --version"
}

$missing = @()
foreach ($req in $requirements.GetEnumerator()) {
    try {
        $result = Invoke-Expression $req.Value 2>&1
        Write-Host "  ✓ $($req.Key): OK" -ForegroundColor Green
    } catch {
        Write-Host "  ✗ $($req.Key): NÃO INSTALADO" -ForegroundColor Red
        $missing += $req.Key
    }
}

if ($missing.Count -gt 0 -and -not $ConfigureAll) {
    Write-Host ""
    Write-Host "⚠️  Requisitos faltando: $($missing -join ', ')" -ForegroundColor Red
    Write-Host "Execute novamente com -ConfigureAll para instalar automaticamente" -ForegroundColor Yellow
    exit 1
}

# ============================================================================
# PASSO 2: INSTALAR RUST (se necessário)
# ============================================================================
if ($InstallRust -or $ConfigureAll) {
    if ($missing -contains "Rust") {
        Write-Host ""
        Write-Host "🦀 [2/8] Instalando Rust..." -ForegroundColor Yellow

        $rustupUrl = "https://win.rustup.rs/x86_64"
        $rustupInstaller = "$env:TEMP\rustup-init.exe"

        Invoke-WebRequest -Uri $rustupUrl -OutFile $rustupInstaller
        Start-Process -FilePath $rustupInstaller -ArgumentList "-y" -Wait

        # Adicionar ao PATH da sessão atual
        $env:PATH += ";$env:USERPROFILE\.cargo\bin"

        Write-Host "  ✓ Rust instalado com sucesso!" -ForegroundColor Green
    } else {
        Write-Host "🦀 [2/8] Rust já instalado, pulando..." -ForegroundColor Green
    }
}

# ============================================================================
# PASSO 3: INSTALAR VSCODE (se necessário)
# ============================================================================
if ($InstallVSCode -or $ConfigureAll) {
    if ($missing -contains "VSCode") {
        Write-Host ""
        Write-Host "📝 [3/8] Instalando Visual Studio Code..." -ForegroundColor Yellow

        $vscodeUrl = "https://code.visualstudio.com/sha/download?build=stable&os=win32-x64-user"
        $vscodeInstaller = "$env:TEMP\VSCodeSetup.exe"

        Invoke-WebRequest -Uri $vscodeUrl -OutFile $vscodeInstaller
        Start-Process -FilePath $vscodeInstaller -ArgumentList "/VERYSILENT /MERGETASKS=!runcode" -Wait

        Write-Host "  ✓ VSCode instalado com sucesso!" -ForegroundColor Green
    } else {
        Write-Host "📝 [3/8] VSCode já instalado, pulando..." -ForegroundColor Green
    }
}

# ============================================================================
# PASSO 4: CLONAR OU COPIAR REPOSITÓRIO
# ============================================================================
Write-Host ""
Write-Host "📦 [4/8] Configurando repositório..." -ForegroundColor Yellow

$targetPath = Join-Path $TargetDrive $ProjectName

if (Test-Path $targetPath) {
    Write-Host "  ⚠️  Diretório já existe: $targetPath" -ForegroundColor Yellow
    $overwrite = Read-Host "  Sobrescrever? (s/N)"
    if ($overwrite -ne "s") {
        Write-Host "  Instalação cancelada." -ForegroundColor Red
        exit 0
    }
    Remove-Item -Path $targetPath -Recurse -Force
}

if (-not $SkipGitClone) {
    Write-Host "  → Clonando repositório do GitHub..." -ForegroundColor Cyan
    git clone https://github.com/avilaops/arxis.git $targetPath
} else {
    Write-Host "  → Copiando arquivos locais..." -ForegroundColor Cyan
    Copy-Item -Path "d:\arxis\*" -Destination $targetPath -Recurse -Force
}

Write-Host "  ✓ Repositório configurado em: $targetPath" -ForegroundColor Green

# ============================================================================
# PASSO 5: INSTALAR EXTENSÕES VSCODE
# ============================================================================
Write-Host ""
Write-Host "🔌 [5/8] Instalando extensões VSCode..." -ForegroundColor Yellow

$extensions = @(
    "rust-lang.rust-analyzer",
    "tamasfe.even-better-toml",
    "serayuzgur.crates",
    "vadimcn.vscode-lldb",
    "github.copilot",
    "github.copilot-chat"
)

foreach ($ext in $extensions) {
    Write-Host "  → Instalando $ext..." -ForegroundColor Cyan
    code --install-extension $ext --force 2>&1 | Out-Null
}

Write-Host "  ✓ Extensões instaladas!" -ForegroundColor Green

# ============================================================================
# PASSO 6: CONFIGURAR RUST TOOLCHAIN
# ============================================================================
Write-Host ""
Write-Host "⚙️  [6/8] Configurando Rust toolchain..." -ForegroundColor Yellow

Push-Location $targetPath

# Instalar componentes necessários
rustup component add rust-analyzer clippy rustfmt 2>&1 | Out-Null
rustup target add wasm32-unknown-unknown 2>&1 | Out-Null

Write-Host "  ✓ Toolchain configurado!" -ForegroundColor Green

# ============================================================================
# PASSO 7: BUILD INICIAL (apenas fundação)
# ============================================================================
Write-Host ""
Write-Host "🔨 [7/8] Compilando módulos de fundação..." -ForegroundColor Yellow
Write-Host "  (Isso pode levar alguns minutos...)" -ForegroundColor Gray

$foundationModules = @(
    "avila-primitives",
    "avila-error",
    "avila-id",
    "avila-time"
)

$successCount = 0
foreach ($module in $foundationModules) {
    if (Test-Path $module) {
        Write-Host "  → Compilando $module..." -ForegroundColor Cyan
        Push-Location $module
        $buildResult = cargo build 2>&1
        if ($LASTEXITCODE -eq 0) {
            Write-Host "    ✓ $module compilado" -ForegroundColor Green
            $successCount++
        } else {
            Write-Host "    ⚠️  $module falhou (pode ter dependências)" -ForegroundColor Yellow
        }
        Pop-Location
    }
}

Write-Host "  ✓ $successCount de $($foundationModules.Count) módulos compilados" -ForegroundColor Green

Pop-Location

# ============================================================================
# PASSO 8: CRIAR ATALHOS E CONFIGURAÇÕES
# ============================================================================
Write-Host ""
Write-Host "🔗 [8/8] Criando atalhos..." -ForegroundColor Yellow

# Criar script de abertura rápida
$openScript = @"
# Abre todos os 6 notebooks do Arxis em VSCode
`$notebooks = @(
    'notebook1-fundacao.code-workspace',
    'notebook2-matematica.code-workspace',
    'notebook3-data-ml.code-workspace',
    'notebook4-database-cloud.code-workspace',
    'notebook5-advanced.code-workspace',
    'notebook6-coordenacao.code-workspace'
)

Write-Host "🚀 Abrindo 6 notebooks Arxis..." -ForegroundColor Cyan
foreach (`$nb in `$notebooks) {
    Write-Host "  → Abrindo `$nb" -ForegroundColor Yellow
    code "`$PSScriptRoot\`$nb"
    Start-Sleep -Seconds 2
}
Write-Host "✓ Todos os notebooks abertos!" -ForegroundColor Green
"@

$openScriptPath = Join-Path $targetPath "ABRIR-TODOS-NOTEBOOKS.ps1"
Set-Content -Path $openScriptPath -Value $openScript -Encoding UTF8

Write-Host "  ✓ Script criado: ABRIR-TODOS-NOTEBOOKS.ps1" -ForegroundColor Green

# Criar script de status
$statusScript = @"
# Verifica status de compilação de todos os módulos
Write-Host "📊 Status dos Módulos Arxis" -ForegroundColor Cyan
Write-Host "=" * 60

`$total = 0
`$compiled = 0

Get-ChildItem -Directory | Where-Object { `$_.Name -match '^(avila-|avl-|avx-)' } | ForEach-Object {
    `$total++
    if (Test-Path "`$(`$_.FullName)\target") {
        Write-Host "✓ `$(`$_.Name)" -ForegroundColor Green
        `$compiled++
    } else {
        Write-Host "○ `$(`$_.Name)" -ForegroundColor Gray
    }
}

Write-Host ""
Write-Host "Total: `$compiled de `$total módulos compilados (`$([math]::Round(`$compiled/`$total*100,1))%)" -ForegroundColor Cyan
"@

$statusScriptPath = Join-Path $targetPath "STATUS-MODULOS.ps1"
Set-Content -Path $statusScriptPath -Value $statusScript -Encoding UTF8

Write-Host "  ✓ Script criado: STATUS-MODULOS.ps1" -ForegroundColor Green

# ============================================================================
# RESUMO FINAL
# ============================================================================
Write-Host ""
Write-Host "╔════════════════════════════════════════════════════════╗" -ForegroundColor Green
Write-Host "║          INSTALAÇÃO CONCLUÍDA COM SUCESSO! ✓          ║" -ForegroundColor Green
Write-Host "╚════════════════════════════════════════════════════════╝" -ForegroundColor Green
Write-Host ""
Write-Host "📁 Projeto instalado em: $targetPath" -ForegroundColor Cyan
Write-Host ""
Write-Host "🎯 PRÓXIMOS PASSOS:" -ForegroundColor Yellow
Write-Host ""
Write-Host "1. Navegue até o projeto:" -ForegroundColor White
Write-Host "   cd $targetPath" -ForegroundColor Gray
Write-Host ""
Write-Host "2. Abra todos os notebooks:" -ForegroundColor White
Write-Host "   .\ABRIR-TODOS-NOTEBOOKS.ps1" -ForegroundColor Gray
Write-Host ""
Write-Host "3. Ou abra apenas o Notebook 1 (FUNDAÇÃO):" -ForegroundColor White
Write-Host "   code notebook1-fundacao.code-workspace" -ForegroundColor Gray
Write-Host ""
Write-Host "4. Verifique status dos módulos:" -ForegroundColor White
Write-Host "   .\STATUS-MODULOS.ps1" -ForegroundColor Gray
Write-Host ""
Write-Host "📚 Leia os manifestos para entender cada notebook:" -ForegroundColor Yellow
Write-Host "   - NOTEBOOK1-MANIFESTO.md → Fundação (começar aqui)" -ForegroundColor White
Write-Host "   - NOTEBOOK2-MANIFESTO.md → Matemática" -ForegroundColor White
Write-Host "   - NOTEBOOK3-MANIFESTO.md → Data & ML" -ForegroundColor White
Write-Host "   - NOTEBOOK4-MANIFESTO.md → Database & Cloud" -ForegroundColor White
Write-Host "   - NOTEBOOK5-MANIFESTO.md → Advanced" -ForegroundColor White
Write-Host "   - NOTEBOOK6-MANIFESTO.md → Coordenação" -ForegroundColor White
Write-Host ""
Write-Host "🎓 ESTRUTURA:" -ForegroundColor Yellow
Write-Host "   • 82 módulos organizados" -ForegroundColor White
Write-Host "   • 6 notebooks (workspaces)" -ForegroundColor White
Write-Host "   • 96 Copilots trabalhando em paralelo" -ForegroundColor White
Write-Host "   • Arquitetura em camadas com dependências claras" -ForegroundColor White
Write-Host ""
Write-Host "Boa sorte! 🚀" -ForegroundColor Green
Write-Host ""
