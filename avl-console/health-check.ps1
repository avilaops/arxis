# Health check script para Windows (PowerShell)
# AVL Console - Production Health Check

Write-Host "🏥 AVL Console - Health Check" -ForegroundColor Cyan
Write-Host "=============================" -ForegroundColor Cyan
Write-Host ""

# Services to check
$services = @{
    "AVL Console" = "http://localhost:3000/health"
    "AvilaDB" = "http://localhost:8000/health"
    "AVL Auth" = "http://localhost:8080/health"
    "AVX Telemetry" = "http://localhost:8888/health"
    "Prometheus" = "http://localhost:9091/-/healthy"
    "Grafana" = "http://localhost:3001/api/health"
}

$healthyCount = 0
$totalCount = $services.Count
$allHealthy = $true

# Check each service
foreach ($service in $services.GetEnumerator()) {
    $name = $service.Key
    $url = $service.Value

    try {
        $response = Invoke-WebRequest -Uri $url -Method Get -TimeoutSec 5 -UseBasicParsing
        if ($response.StatusCode -eq 200) {
            Write-Host "✓ $name is healthy" -ForegroundColor Green
            $healthyCount++
        }
        else {
            Write-Host "✗ $name returned status $($response.StatusCode)" -ForegroundColor Red
            $allHealthy = $false
        }
    }
    catch {
        Write-Host "✗ $name is not responding" -ForegroundColor Red
        $allHealthy = $false
    }
}

Write-Host ""
Write-Host "Status: $healthyCount/$totalCount services healthy" -ForegroundColor $(if ($allHealthy) { "Green" } else { "Yellow" })

# Check Docker containers
Write-Host ""
Write-Host "📦 Docker Containers:" -ForegroundColor Cyan
try {
    docker-compose ps
}
catch {
    Write-Host "Docker Compose not running" -ForegroundColor Yellow
}

# Summary
Write-Host ""
if ($allHealthy) {
    Write-Host "✅ All systems operational!" -ForegroundColor Green
    exit 0
}
else {
    Write-Host "⚠️  Some services are unhealthy" -ForegroundColor Yellow
    Write-Host "Run: docker-compose logs <service-name>" -ForegroundColor Yellow
    exit 1
}
