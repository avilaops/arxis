#!/usr/bin/env pwsh
# ============================================
# 🚀 OPTIMIZE-VSCODE-CACHE.ps1
# ============================================
# Limpa cache do VS Code e otimiza performance
# Mantém aviladb.instructions.md intacto
#
# COMO USAR:
# 1. Feche o VS Code completamente
# 2. Abra PowerShell como Administrador
# 3. Cole este comando:
#    cd 'C:\Users\nicol\OneDrive\Avila\1.2 - Avilaops\Arxis\examples\practical-cli'; .\optimize-vscode-cache.ps1
#
# OU copie todo o conteúdo deste arquivo e cole no PowerShell Admin

Write-Host "`n🔧 OTIMIZADOR DE CACHE DO VS CODE`n" -ForegroundColor Cyan

$VSCODE_CACHE = "$env:APPDATA\Code\Cache"
$VSCODE_CACHEDDATA = "$env:APPDATA\Code\CachedData"
$VSCODE_CACHED_EXT = "$env:APPDATA\Code\CachedExtensions"
$VSCODE_CACHED_EXTVS = "$env:APPDATA\Code\CachedExtensionVSIXs"
$VSCODE_GPU_CACHE = "$env:APPDATA\Code\GPUCache"
$VSCODE_LOGS = "$env:APPDATA\Code\logs"
$VSCODE_STORAGE = "$env:APPDATA\Code\User\workspaceStorage"

$TEMP_RUST = "$env:TEMP\rust-analyzer"
$TEMP_CARGO = "$env:TEMP\cargo-*"

function Get-FolderSize {
    param([string]$Path)
    if (Test-Path $Path) {
        $size = (Get-ChildItem -Path $Path -Recurse -File -ErrorAction SilentlyContinue |
            Measure-Object -Property Length -Sum).Sum
        return [math]::Round($size / 1MB, 2)
    }
    return 0
}

function Remove-SafeCache {
    param([string]$Path, [string]$Name)

    if (Test-Path $Path) {
        $sizeBefore = Get-FolderSize -Path $Path
        Write-Host "  🗑️  $Name : $sizeBefore MB" -ForegroundColor Yellow

        try {
            Remove-Item -Path $Path -Recurse -Force -ErrorAction Stop
            Write-Host "     ✅ Removido!" -ForegroundColor Green
            return $sizeBefore
        }
        catch {
            Write-Host "     ⚠️  Erro: $_" -ForegroundColor Red
            return 0
        }
    }
    else {
        Write-Host "  ℹ️  $Name : Não existe" -ForegroundColor Gray
        return 0
    }
}

# ============================================
# Verifica se VS Code está rodando
# ============================================
$vscodeRunning = Get-Process -Name "Code" -ErrorAction SilentlyContinue

if ($vscodeRunning) {
    Write-Host "⚠️  VS CODE ESTÁ RODANDO!" -ForegroundColor Yellow
    Write-Host "   Feche o VS Code para melhor limpeza.`n" -ForegroundColor Yellow
    $response = Read-Host "Continuar mesmo assim? (s/n)"
    if ($response -ne 's') {
        Write-Host "`n❌ Operação cancelada." -ForegroundColor Red
        exit 0
    }
}

Write-Host "`n📊 CALCULANDO TAMANHO DOS CACHES...`n" -ForegroundColor Cyan

# ============================================
# Limpeza de Caches
# ============================================
$totalFreed = 0

Write-Host "🗂️  CACHES DO VS CODE:" -ForegroundColor Yellow
$totalFreed += Remove-SafeCache -Path $VSCODE_CACHE -Name "Cache Principal"
$totalFreed += Remove-SafeCache -Path $VSCODE_CACHEDDATA -Name "Cached Data"
$totalFreed += Remove-SafeCache -Path $VSCODE_CACHED_EXT -Name "Cached Extensions"
$totalFreed += Remove-SafeCache -Path $VSCODE_CACHED_EXTVS -Name "Cached VSIXs"
$totalFreed += Remove-SafeCache -Path $VSCODE_GPU_CACHE -Name "GPU Cache"

Write-Host "`n📝 LOGS DO VS CODE:" -ForegroundColor Yellow
if (Test-Path $VSCODE_LOGS) {
    $logSize = Get-FolderSize -Path $VSCODE_LOGS
    Write-Host "  📁 Logs: $logSize MB" -ForegroundColor Cyan
    Write-Host "     ℹ️  Mantendo logs (útil para debug)" -ForegroundColor Gray
}

Write-Host "`n🦀 CACHES DO RUST-ANALYZER:" -ForegroundColor Yellow
$totalFreed += Remove-SafeCache -Path $TEMP_RUST -Name "rust-analyzer temp"

# Cargo temp files
if (Test-Path "$env:TEMP\cargo-*") {
    $cargoTempSize = 0
    Get-ChildItem -Path "$env:TEMP\cargo-*" -Directory | ForEach-Object {
        $size = Get-FolderSize -Path $_.FullName
        $cargoTempSize += $size
        Remove-Item -Path $_.FullName -Recurse -Force -ErrorAction SilentlyContinue
    }
    if ($cargoTempSize -gt 0) {
        Write-Host "  🗑️  cargo-* temp: $cargoTempSize MB" -ForegroundColor Yellow
        Write-Host "     ✅ Removido!" -ForegroundColor Green
        $totalFreed += $cargoTempSize
    }
}

# ============================================
# Workspace Storage (cuidado aqui!)
# ============================================
Write-Host "`n💾 WORKSPACE STORAGE:" -ForegroundColor Yellow
if (Test-Path $VSCODE_STORAGE) {
    $storageSize = Get-FolderSize -Path $VSCODE_STORAGE
    Write-Host "  📁 Workspace Storage: $storageSize MB" -ForegroundColor Cyan
    Write-Host "     ⚠️  MANTENDO (contém histórico e cache de projetos)" -ForegroundColor Yellow
    Write-Host "     ℹ️  Para limpar manualmente: Remove-Item '$VSCODE_STORAGE' -Recurse" -ForegroundColor Gray
}

# ============================================
# PROTEGENDO aviladb.instructions.md
# ============================================
Write-Host "`n🛡️  VERIFICANDO ARQUIVOS PROTEGIDOS:" -ForegroundColor Green
$aviladbInstructions = "$env:APPDATA\Code\User\prompts\aviladb.instructions.md"
if (Test-Path $aviladbInstructions) {
    $instructionsSize = (Get-Item $aviladbInstructions).Length
    Write-Host "  ✅ aviladb.instructions.md : $([math]::Round($instructionsSize/1KB, 2)) KB" -ForegroundColor Green
    Write-Host "     INTACTO!" -ForegroundColor Green
}
else {
    Write-Host "  ⚠️  aviladb.instructions.md NÃO ENCONTRADO!" -ForegroundColor Yellow
}

# ============================================
# Resumo Final
# ============================================
Write-Host "`n" + "="*50 -ForegroundColor Cyan
Write-Host "📊 RESUMO DA LIMPEZA" -ForegroundColor Cyan
Write-Host "="*50 -ForegroundColor Cyan
Write-Host "`n  🗑️  Espaço liberado: $([math]::Round($totalFreed, 2)) MB" -ForegroundColor Green

if ($totalFreed -gt 100) {
    Write-Host "  🔥 EXCELENTE! Muito espaço recuperado!" -ForegroundColor Green
}
elseif ($totalFreed -gt 50) {
    Write-Host "  ✅ BOM! Cache limpo com sucesso!" -ForegroundColor Green
}
else {
    Write-Host "  ℹ️  Cache já estava relativamente limpo." -ForegroundColor Cyan
}

Write-Host "`n🚀 PRÓXIMOS PASSOS:" -ForegroundColor Yellow
Write-Host "  1. Reabra o VS Code" -ForegroundColor White
Write-Host "  2. O cache será reconstruído automaticamente" -ForegroundColor White
Write-Host "  3. Primeira abertura pode ser mais lenta" -ForegroundColor White
Write-Host "  4. Performance melhorará após reindexação`n" -ForegroundColor White

Write-Host "✅ CONCLUÍDO!`n" -ForegroundColor Green

# Pausa para você ver os resultados antes de fechar
Write-Host "Pressione qualquer tecla para fechar..." -ForegroundColor Cyan
$null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")
