# Script para criar labels do GitHub para o repositório Arxis
# Requer: GitHub CLI (gh) instalado e autenticado

Write-Host "🏛️ Arxis - GitHub Labels Setup" -ForegroundColor Cyan
Write-Host "================================" -ForegroundColor Cyan
Write-Host ""

# Verificar se gh está instalado
if (-not (Get-Command gh -ErrorAction SilentlyContinue)) {
    Write-Host "❌ GitHub CLI (gh) não encontrado!" -ForegroundColor Red
    Write-Host "Instale com: winget install GitHub.cli" -ForegroundColor Yellow
    exit 1
}

# Verificar autenticação
$authStatus = gh auth status 2>&1
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ GitHub CLI não está autenticado!" -ForegroundColor Red
    Write-Host "Execute: gh auth login" -ForegroundColor Yellow
    exit 1
}

Write-Host "✅ GitHub CLI configurado corretamente" -ForegroundColor Green
Write-Host ""

# Função para criar label
function New-GitHubLabel {
    param(
        [string]$Name,
        [string]$Color,
        [string]$Description
    )

    Write-Host "Criando label: $Name" -ForegroundColor Cyan
    gh label create "$Name" --color "$Color" --description "$Description" --force 2>&1 | Out-Null

    if ($LASTEXITCODE -eq 0) {
        Write-Host "  ✅ $Name criada" -ForegroundColor Green
    }
    else {
        Write-Host "  ⚠️ $Name já existe (pulando)" -ForegroundColor Yellow
    }
}

Write-Host "📝 Criando Type Labels..." -ForegroundColor Magenta
New-GitHubLabel "bug" "d73a4a" "Something isn't working"
New-GitHubLabel "enhancement" "a2eeef" "New feature or request"
New-GitHubLabel "documentation" "0075ca" "Improvements or additions to documentation"
New-GitHubLabel "performance" "ff9800" "Performance related issue or optimization"
New-GitHubLabel "security" "d93f0b" "Security vulnerability or concern"
New-GitHubLabel "refactoring" "fbca04" "Code refactoring (no functional changes)"
New-GitHubLabel "testing" "1d76db" "Related to testing infrastructure"
Write-Host ""

Write-Host "🎯 Criando Priority Labels..." -ForegroundColor Magenta
New-GitHubLabel "priority: critical" "b60205" "Requires immediate attention"
New-GitHubLabel "priority: high" "d93f0b" "Should be addressed soon"
New-GitHubLabel "priority: medium" "fbca04" "Normal priority"
New-GitHubLabel "priority: low" "0e8a16" "Low priority, nice to have"
Write-Host ""

Write-Host "📊 Criando Status Labels..." -ForegroundColor Magenta
New-GitHubLabel "status: triage" "ededed" "Needs initial review and labeling"
New-GitHubLabel "status: accepted" "0e8a16" "Issue accepted and will be worked on"
New-GitHubLabel "status: in progress" "1d76db" "Currently being worked on"
New-GitHubLabel "status: blocked" "b60205" "Blocked by another issue or external factor"
New-GitHubLabel "status: needs info" "d876e3" "More information needed from issue author"
New-GitHubLabel "status: stale" "fef2c0" "No activity for extended period"
New-GitHubLabel "status: wontfix" "ffffff" "This will not be worked on"
New-GitHubLabel "status: duplicate" "cfd3d7" "This issue or PR already exists"
Write-Host ""

Write-Host "📦 Criando Crate Labels..." -ForegroundColor Magenta
New-GitHubLabel "crate: arxis" "006b75" "Main arxis_quaternions library"
New-GitHubLabel "crate: avila-math" "006b75" "Mathematical kernel (quaternions, tensors)"
New-GitHubLabel "crate: avila-telemetry" "006b75" "Time series & analytics"
New-GitHubLabel "crate: avila-compress" "006b75" "Compression library"
New-GitHubLabel "crate: avila-tokenizers" "006b75" "NLP tokenization"
New-GitHubLabel "crate: aviladb" "006b75" "Database system"
New-GitHubLabel "crate: avx-platform" "006b75" "AVL Platform crates"
New-GitHubLabel "crate: multiple" "006b75" "Affects multiple crates"
Write-Host ""

Write-Host "🔬 Criando Area Labels..." -ForegroundColor Magenta
New-GitHubLabel "area: LISA" "5319e7" "LISA mission & gravitational waves"
New-GitHubLabel "area: physics" "5319e7" "Physics implementations (GW, relativity, cosmology)"
New-GitHubLabel "area: quaternions" "5319e7" "Quaternion algebra (3D, dual, SO(4))"
New-GitHubLabel "area: tensors" "5319e7" "Tensor operations & ML"
New-GitHubLabel "area: 4D geometry" "5319e7" "4D geometry & polytopes"
New-GitHubLabel "area: API" "5319e7" "Public API design"
New-GitHubLabel "area: CI/CD" "5319e7" "Continuous integration & deployment"
New-GitHubLabel "area: benchmarks" "5319e7" "Performance benchmarks"
Write-Host ""

Write-Host "💪 Criando Difficulty Labels..." -ForegroundColor Magenta
New-GitHubLabel "good first issue" "7057ff" "Good for newcomers"
New-GitHubLabel "help wanted" "008672" "Extra attention is needed"
New-GitHubLabel "difficulty: easy" "c2e0c6" "Easy to implement"
New-GitHubLabel "difficulty: medium" "fbca04" "Moderate complexity"
New-GitHubLabel "difficulty: hard" "d93f0b" "Complex implementation required"
Write-Host ""

Write-Host "⭐ Criando Special Labels..." -ForegroundColor Magenta
New-GitHubLabel "breaking change" "b60205" "Changes that break backward compatibility"
New-GitHubLabel "dependencies" "0366d6" "Pull requests that update a dependency file"
New-GitHubLabel "pinned" "d4c5f9" "Important issue that should not be closed by stale bot"
New-GitHubLabel "work in progress" "fbca04" "PR is still being worked on"
New-GitHubLabel "needs review" "d876e3" "Waiting for code review"
New-GitHubLabel "question" "d876e3" "Further information is requested"
New-GitHubLabel "roadmap" "0e8a16" "Part of the project roadmap"
New-GitHubLabel "research" "5319e7" "Requires research or experimentation"
New-GitHubLabel "NASA mission" "000080" "Related to NASA/ESA missions (LISA, LIGO)"
Write-Host ""

Write-Host "✅ Labels criadas com sucesso!" -ForegroundColor Green
Write-Host ""
Write-Host "📋 Para visualizar todas as labels:" -ForegroundColor Cyan
Write-Host "   gh label list" -ForegroundColor White
Write-Host ""
Write-Host "🌐 Ou acesse no navegador:" -ForegroundColor Cyan
Write-Host "   https://github.com/avilaops/arxis/labels" -ForegroundColor White
Write-Host ""
Write-Host "🏛️ Arxis - The Mathematical Citadel" -ForegroundColor Cyan
