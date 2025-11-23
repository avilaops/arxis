#!/usr/bin/env bash
# Health check script for AVL Console production deployment

set -e

GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo "🏥 AVL Console - Health Check"
echo "============================="
echo ""

# Services to check
declare -A services=(
    ["AVL Console"]="http://localhost:3000/health"
    ["AvilaDB"]="http://localhost:8000/health"
    ["AVL Auth"]="http://localhost:8080/health"
    ["AVX Telemetry"]="http://localhost:8888/health"
    ["Prometheus"]="http://localhost:9091/-/healthy"
    ["Grafana"]="http://localhost:3001/api/health"
)

all_healthy=true
healthy_count=0
total_count=${#services[@]}

# Check each service
for service in "${!services[@]}"; do
    url="${services[$service]}"

    if response=$(curl -s -f -w "\n%{http_code}" "$url" 2>/dev/null); then
        status_code=$(echo "$response" | tail -n1)
        if [ "$status_code" -eq 200 ]; then
            echo -e "${GREEN}✓${NC} $service is healthy"
            ((healthy_count++))
        else
            echo -e "${RED}✗${NC} $service returned status $status_code"
            all_healthy=false
        fi
    else
        echo -e "${RED}✗${NC} $service is not responding"
        all_healthy=false
    fi
done

echo ""
echo "Status: $healthy_count/$total_count services healthy"

# Check Docker containers
echo ""
echo "📦 Docker Containers:"
docker-compose ps --format "table {{.Name}}\t{{.Status}}\t{{.Ports}}" 2>/dev/null || echo "Docker Compose not running"

# Summary
echo ""
if [ "$all_healthy" = true ]; then
    echo -e "${GREEN}✅ All systems operational!${NC}"
    exit 0
else
    echo -e "${YELLOW}⚠️  Some services are unhealthy${NC}"
    echo "Run: docker-compose logs <service-name>"
    exit 1
fi
