# 🚀 Setup Local Otimizado (C:\Temp)
# Copia o projeto para C:\Temp e roda lá para máxima performance

Write-Host ""
Write-Host "╔═══════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║  🚀 SETUP LOCAL OTIMIZADO (C:\Temp)  🚀       ║" -ForegroundColor Yellow
Write-Host "╚═══════════════════════════════════════════════╝" -ForegroundColor Cyan
Write-Host ""

$SOURCE = $PWD.Path
$DEST = "C:\Temp\arxis-benchmark"

Write-Host "📁 Origem:  $SOURCE" -ForegroundColor Gray
Write-Host "📁 Destino: $DEST" -ForegroundColor Gray
Write-Host ""

# Criar diretório C:\Temp se não existe
if (-not (Test-Path "C:\Temp")) {
    Write-Host "📂 Criando C:\Temp..." -ForegroundColor Yellow
    New-Item -Path "C:\Temp" -ItemType Directory | Out-Null
}

# Remover instalação antiga se existir
if (Test-Path $DEST) {
    Write-Host "🗑️  Removendo instalação antiga..." -ForegroundColor Yellow
    Remove-Item -Path $DEST -Recurse -Force
}

Write-Host "📦 Copiando projeto para C:\Temp..." -ForegroundColor Cyan

# Copiar apenas o necessário (sem target/)
New-Item -Path $DEST -ItemType Directory | Out-Null

# Copiar arquivos essenciais
Copy-Item -Path "$SOURCE\Cargo.toml" -Destination $DEST
Copy-Item -Path "$SOURCE\src" -Destination "$DEST\src" -Recurse
Copy-Item -Path "$SOURCE\.cargo" -Destination "$DEST\.cargo" -Recurse -ErrorAction SilentlyContinue
Copy-Item -Path "$SOURCE\*.ps1" -Destination $DEST
Copy-Item -Path "$SOURCE\*.md" -Destination $DEST

# Copiar dependências
Write-Host "📦 Copiando dependências locais..." -ForegroundColor Cyan
$ROOT = Split-Path -Parent (Split-Path -Parent $SOURCE)

Copy-Item -Path "$ROOT\avila-compress" -Destination "C:\Temp\avila-compress" -Recurse -Force -ErrorAction SilentlyContinue
Copy-Item -Path "$ROOT\avx-http" -Destination "C:\Temp\avx-http" -Recurse -Force -ErrorAction SilentlyContinue
Copy-Item -Path "$ROOT\avila-arrow" -Destination "C:\Temp\avila-arrow" -Recurse -Force -ErrorAction SilentlyContinue

Write-Host "✅ Cópia concluída!" -ForegroundColor Green
Write-Host ""

# Atualizar Cargo.toml para usar paths locais de C:\Temp
Write-Host "🔧 Atualizando Cargo.toml..." -ForegroundColor Cyan

$cargoContent = Get-Content "$DEST\Cargo.toml" -Raw
$cargoContent = $cargoContent -replace '../../avila-compress', 'C:/Temp/avila-compress'
$cargoContent = $cargoContent -replace '../../avx-http', 'C:/Temp/avx-http'
$cargoContent = $cargoContent -replace '../../avila-arrow', 'C:/Temp/avila-arrow'
Set-Content -Path "$DEST\Cargo.toml" -Value $cargoContent

Write-Host "✅ Configuração atualizada!" -ForegroundColor Green
Write-Host ""

# Copiar arquivo de teste
if (Test-Path "$ROOT\README.md") {
    Copy-Item -Path "$ROOT\README.md" -Destination "C:\Temp\README.md"
    Write-Host "✅ Copiado README.md para C:\Temp\" -ForegroundColor Green
}

Write-Host ""
Write-Host "╔═══════════════════════════════════════════════╗" -ForegroundColor Green
Write-Host "║  ✅ SETUP COMPLETO! Próximos passos:         ║" -ForegroundColor Yellow
Write-Host "╚═══════════════════════════════════════════════╝" -ForegroundColor Green
Write-Host ""
Write-Host "1️⃣  Navegar para C:\Temp:" -ForegroundColor Cyan
Write-Host "   cd C:\Temp\arxis-benchmark" -ForegroundColor White
Write-Host ""
Write-Host "2️⃣  Compilar (primeira vez demora ~2 min):" -ForegroundColor Cyan
Write-Host "   cargo build --release" -ForegroundColor White
Write-Host ""
Write-Host "3️⃣  Rodar benchmark rápido:" -ForegroundColor Cyan
Write-Host "   .\quick-benchmark.ps1" -ForegroundColor White
Write-Host ""
Write-Host "4️⃣  Ou benchmark extremo (desabilite Defender primeiro):" -ForegroundColor Cyan
Write-Host "   .\disable-defender.ps1  # Como Admin" -ForegroundColor White
Write-Host "   .\extreme-benchmark.ps1" -ForegroundColor White
Write-Host "   .\enable-defender.ps1   # Como Admin" -ForegroundColor White
Write-Host ""

Write-Host "🔥 VANTAGENS de rodar em C:\Temp:" -ForegroundColor Yellow
Write-Host "   ✅ Sem overhead do OneDrive" -ForegroundColor Green
Write-Host "   ✅ I/O mais rápido (disco local)" -ForegroundColor Green
Write-Host "   ✅ Sem sincronização de arquivos temporários" -ForegroundColor Green
Write-Host "   ✅ Performance real do Rust!" -ForegroundColor Green
Write-Host ""

# Abrir nova janela no diretório
$openDir = Read-Host "Deseja abrir uma nova janela PowerShell em C:\Temp\arxis-benchmark? (S/N)"
if ($openDir -eq "S" -or $openDir -eq "s") {
    Start-Process powershell -ArgumentList "-NoExit", "-Command", "cd C:\Temp\arxis-benchmark; Write-Host '🚀 Pronto para compilar!' -ForegroundColor Green"
}

Write-Host ""
Write-Host "📍 Localização: C:\Temp\arxis-benchmark" -ForegroundColor Cyan
Write-Host ""
