# Script para publicar avila-browser no crates.io
# Execute este script após obter um token válido do crates.io

Write-Host "=== AVILA BROWSER - PUBLICAÇÃO NO CRATES.IO ===" -ForegroundColor Cyan
Write-Host ""

# Verificar se tem token configurado
$tokenFile = "$env:USERPROFILE\.cargo\credentials.toml"
if (-not (Test-Path $tokenFile)) {
    Write-Host "❌ Nenhum token encontrado!" -ForegroundColor Red
    Write-Host ""
    Write-Host "Por favor, siga estes passos:" -ForegroundColor Yellow
    Write-Host "1. Acesse: https://crates.io/settings/tokens" -ForegroundColor Yellow
    Write-Host "2. Crie um novo token com permissão 'publish-update'" -ForegroundColor Yellow
    Write-Host "3. Execute: cargo login <seu-token>" -ForegroundColor Yellow
    Write-Host "4. Execute este script novamente" -ForegroundColor Yellow
    exit 1
}

Write-Host "✓ Token encontrado" -ForegroundColor Green
Write-Host ""

# Verificar compilação
Write-Host "Verificando compilação..." -ForegroundColor Cyan
cargo check
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Erro na compilação!" -ForegroundColor Red
    exit 1
}
Write-Host "✓ Compilação OK" -ForegroundColor Green
Write-Host ""

# Executar testes
Write-Host "Executando testes..." -ForegroundColor Cyan
cargo test
if ($LASTEXITCODE -ne 0) {
    Write-Host "⚠️  Alguns testes falharam, mas continuando..." -ForegroundColor Yellow
}
Write-Host ""

# Build release
Write-Host "Compilando versão release..." -ForegroundColor Cyan
cargo build --release
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Erro no build release!" -ForegroundColor Red
    exit 1
}
Write-Host "✓ Build release OK" -ForegroundColor Green
Write-Host ""

# Verificar pacote
Write-Host "Verificando pacote..." -ForegroundColor Cyan
cargo package --allow-dirty
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Erro ao empacotar!" -ForegroundColor Red
    exit 1
}
Write-Host "✓ Pacote OK" -ForegroundColor Green
Write-Host ""

# Confirmar publicação
Write-Host "ATENÇÃO: Você está prestes a publicar 'avila-browser' no crates.io!" -ForegroundColor Yellow
Write-Host "Esta ação NÃO PODE ser desfeita!" -ForegroundColor Yellow
Write-Host ""
$confirm = Read-Host "Deseja continuar? (s/N)"

if ($confirm -ne "s" -and $confirm -ne "S") {
    Write-Host "Publicação cancelada." -ForegroundColor Yellow
    exit 0
}

# Publicar
Write-Host ""
Write-Host "Publicando no crates.io..." -ForegroundColor Cyan
cargo publish --allow-dirty

if ($LASTEXITCODE -eq 0) {
    Write-Host ""
    Write-Host "✓ ✓ ✓ SUCESSO! ✓ ✓ ✓" -ForegroundColor Green
    Write-Host ""
    Write-Host "avila-browser foi publicado com sucesso no crates.io!" -ForegroundColor Green
    Write-Host "Acesse: https://crates.io/crates/avila-browser" -ForegroundColor Cyan
    Write-Host "Documentação: https://docs.rs/avila-browser" -ForegroundColor Cyan
} else {
    Write-Host ""
    Write-Host "❌ Erro na publicação!" -ForegroundColor Red
    Write-Host ""
    Write-Host "Possíveis causas:" -ForegroundColor Yellow
    Write-Host "- Token expirado ou sem permissões" -ForegroundColor Yellow
    Write-Host "- Nome 'avila-browser' já registrado por outro usuário" -ForegroundColor Yellow
    Write-Host "- Problemas de conexão com crates.io" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "Soluções:" -ForegroundColor Cyan
    Write-Host "1. Verifique seu token em: https://crates.io/settings/tokens" -ForegroundColor Cyan
    Write-Host "2. Se o nome está registrado, mude em Cargo.toml para algo único" -ForegroundColor Cyan
    Write-Host "3. Tente novamente em alguns minutos" -ForegroundColor Cyan
}
