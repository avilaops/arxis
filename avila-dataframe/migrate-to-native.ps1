# Script para migrar para versão 100% Rust Nativa

Write-Host "`n🦀 AvilaDB DataFrame - Migração para 100% Rust Nativo" -ForegroundColor Cyan
Write-Host "=" * 60 -ForegroundColor Cyan

# Backup da versão antiga
Write-Host "`n1️⃣  Fazendo backup da versão Arrow..." -ForegroundColor Yellow
Copy-Item "Cargo.toml" "Cargo.toml.arrow" -Force
Copy-Item "src/lib.rs" "src/lib_arrow.rs" -Force
Write-Host "   ✅ Backup criado: Cargo.toml.arrow, src/lib_arrow.rs" -ForegroundColor Green

# Substituir para versão nativa
Write-Host "`n2️⃣  Ativando versão nativa..." -ForegroundColor Yellow
Copy-Item "Cargo.toml.native" "Cargo.toml" -Force
Copy-Item "src/lib_native.rs" "src/lib.rs" -Force

# Atualizar módulos
Write-Host "   📦 Atualizando módulos..." -ForegroundColor Yellow
Copy-Item "src/core/mod_native.rs" "src/core/mod.rs" -Force
Copy-Item "src/ops/mod_native.rs" "src/ops/mod.rs" -Force
Copy-Item "src/io/mod_native.rs" "src/io/mod.rs" -Force
Copy-Item "src/error_native.rs" "src/error.rs" -Force
Write-Host "   ✅ Módulos atualizados" -ForegroundColor Green

# Limpar build anterior
Write-Host "`n3️⃣  Limpando build anterior..." -ForegroundColor Yellow
if (Test-Path "target") {
    Remove-Item "target" -Recurse -Force -ErrorAction SilentlyContinue
    Write-Host "   ✅ Target limpo" -ForegroundColor Green
}
else {
    Write-Host "   ℹ️  Nada para limpar" -ForegroundColor Gray
}

# Build nova versão
Write-Host "`n4️⃣  Compilando versão nativa..." -ForegroundColor Yellow
Write-Host "   ⏳ Aguarde (deve levar ~30 segundos)...`n" -ForegroundColor Gray

$buildStart = Get-Date
cargo build --release 2>&1 | Out-Null
$buildEnd = Get-Date
$buildTime = ($buildEnd - $buildStart).TotalSeconds

if ($LASTEXITCODE -eq 0) {
    Write-Host "   ✅ Build sucesso em $([math]::Round($buildTime, 1))s!" -ForegroundColor Green

    # Tamanho do binário
    $binaryPath = "target/release/avila_dataframe.dll"
    if (Test-Path $binaryPath) {
        $size = (Get-Item $binaryPath).Length / 1MB
        Write-Host "   📦 Tamanho: $([math]::Round($size, 2)) MB" -ForegroundColor Green
    }
}
else {
    Write-Host "   ❌ Build falhou. Veja erros acima." -ForegroundColor Red
    exit 1
}

# Executar exemplo
Write-Host "`n5️⃣  Executando exemplo quickstart..." -ForegroundColor Yellow
Write-Host "=" * 60 -ForegroundColor Cyan
Write-Host ""

cargo run --example quickstart_native --release

if ($LASTEXITCODE -eq 0) {
    Write-Host "`n=" * 60 -ForegroundColor Cyan
    Write-Host "✅ MIGRAÇÃO COMPLETA!" -ForegroundColor Green
    Write-Host "=" * 60 -ForegroundColor Cyan
    Write-Host ""
    Write-Host "📊 Comparação:" -ForegroundColor Cyan
    Write-Host "   Versão Arrow:  ~320s build, ~127 MB binary" -ForegroundColor Gray
    Write-Host "   Versão Nativa: ~$([math]::Round($buildTime, 1))s build" -ForegroundColor Green
    Write-Host ""
    Write-Host "🎯 Próximos passos:" -ForegroundColor Cyan
    Write-Host "   1. Testar com seus dados" -ForegroundColor White
    Write-Host "   2. cargo test" -ForegroundColor White
    Write-Host "   3. Integrar com AvilaDB" -ForegroundColor White
    Write-Host ""
    Write-Host "🔄 Para voltar para Arrow:" -ForegroundColor Cyan
    Write-Host "   Copy-Item Cargo.toml.arrow Cargo.toml -Force" -ForegroundColor White
    Write-Host "   Copy-Item src/lib_arrow.rs src/lib.rs -Force" -ForegroundColor White
    Write-Host ""
    Write-Host "🔥 Destruindo a concorrência! 🇧🇷" -ForegroundColor Yellow
}
else {
    Write-Host "`n❌ Exemplo falhou. Debug necessário." -ForegroundColor Red
}
