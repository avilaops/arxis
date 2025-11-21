# Deploy script para Landing Page Leptos
# Compila para WASM e copia para landing-page/

Write-Host "🦀 Compilando Landing Page em Rust + WASM..." -ForegroundColor Yellow
trunk build --release

if ($LASTEXITCODE -eq 0) {
    Write-Host "✅ Compilação bem-sucedida!" -ForegroundColor Green

    Write-Host "📦 Copiando arquivos para landing-page/..." -ForegroundColor Cyan

    # Copia arquivos do dist/ para landing-page/
    Copy-Item -Path "dist\*" -Destination "..\landing-page\" -Recurse -Force

    Write-Host "✅ Deploy concluído! Arquivos copiados para landing-page/" -ForegroundColor Green
    Write-Host "🚀 Agora faça: git add landing-page/ && git commit && git push" -ForegroundColor Yellow
}
else {
    Write-Host "❌ Erro na compilação!" -ForegroundColor Red
    exit 1
}
