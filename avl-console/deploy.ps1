# Deploy script para Windows (PowerShell)
# AVL Console - Production Deployment

Write-Host "🚀 AVL Console - Production Deployment" -ForegroundColor Cyan
Write-Host "======================================" -ForegroundColor Cyan
Write-Host ""

# Check if .env file exists
if (-not (Test-Path .env)) {
    Write-Host "❌ Error: .env file not found" -ForegroundColor Red
    Write-Host "Copy .env.example to .env and configure it:"
    Write-Host "  Copy-Item .env.example .env"
    exit 1
}

# Load environment variables
Get-Content .env | ForEach-Object {
    if ($_ -match '^([^#][^=]+)=(.*)$') {
        $name = $matches[1].Trim()
        $value = $matches[2].Trim()
        Set-Item -Path "env:$name" -Value $value
    }
}

Write-Host "✓ Environment variables loaded" -ForegroundColor Green

# Check required variables
$requiredVars = @(
    "AVILADB_API_KEY",
    "AVL_AUTH_JWT_SECRET",
    "SESSION_SECRET"
)

$missingVars = @()
foreach ($var in $requiredVars) {
    if (-not (Test-Path "env:$var")) {
        $missingVars += $var
    }
}

if ($missingVars.Count -gt 0) {
    Write-Host "❌ Error: The following variables are not set:" -ForegroundColor Red
    $missingVars | ForEach-Object { Write-Host "  - $_" -ForegroundColor Red }
    exit 1
}

# Build Docker images
Write-Host ""
Write-Host "📦 Building Docker images..." -ForegroundColor Yellow
docker-compose build
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Failed to build Docker images" -ForegroundColor Red
    exit 1
}

Write-Host "✓ Docker images built successfully" -ForegroundColor Green

# Stop existing containers
Write-Host ""
Write-Host "🛑 Stopping existing containers..." -ForegroundColor Yellow
docker-compose down

# Start services
Write-Host ""
Write-Host "🚀 Starting AVL Platform services..." -ForegroundColor Cyan
docker-compose up -d
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Failed to start services" -ForegroundColor Red
    exit 1
}

# Wait for services to be healthy
Write-Host ""
Write-Host "⏳ Waiting for services to be healthy..." -ForegroundColor Yellow
Start-Sleep -Seconds 10

# Check health endpoints
$services = @{
    "AVL Console" = "http://localhost:3000/health"
    "AvilaDB" = "http://localhost:8000/health"
    "AVL Auth" = "http://localhost:8080/health"
    "AVX Telemetry" = "http://localhost:8888/health"
}

$allHealthy = $true
foreach ($service in $services.GetEnumerator()) {
    try {
        $response = Invoke-WebRequest -Uri $service.Value -Method Get -TimeoutSec 5 -UseBasicParsing
        if ($response.StatusCode -eq 200) {
            Write-Host "✓ $($service.Key) is healthy" -ForegroundColor Green
        }
        else {
            Write-Host "✗ $($service.Key) returned status $($response.StatusCode)" -ForegroundColor Red
            $allHealthy = $false
        }
    }
    catch {
        Write-Host "✗ $($service.Key) is not responding" -ForegroundColor Red
        $allHealthy = $false
    }
}

Write-Host ""
if ($allHealthy) {
    Write-Host "✅ Deployment successful!" -ForegroundColor Green
    Write-Host ""
    Write-Host "🌐 AVL Console: http://localhost:3000" -ForegroundColor Cyan
    Write-Host "📊 Metrics: http://localhost:9090" -ForegroundColor Cyan
    Write-Host "📈 Grafana: http://localhost:3001" -ForegroundColor Cyan
    Write-Host "📚 AvilaDB: http://localhost:8000" -ForegroundColor Cyan
    Write-Host "🔐 AVL Auth: http://localhost:8080" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "View logs with: docker-compose logs -f" -ForegroundColor Yellow
}
else {
    Write-Host "⚠️  Some services are not healthy" -ForegroundColor Yellow
    Write-Host "Check logs with: docker-compose logs" -ForegroundColor Yellow
    exit 1
}
