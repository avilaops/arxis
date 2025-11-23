#!/usr/bin/env bash
# Deploy script for AVL Console to production

set -e

echo "🚀 AVL Console - Production Deployment"
echo "======================================"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if .env file exists
if [ ! -f .env ]; then
    echo -e "${RED}❌ Error: .env file not found${NC}"
    echo "Copy .env.example to .env and configure it:"
    echo "  cp .env.example .env"
    exit 1
fi

# Load environment variables
export $(cat .env | grep -v '^#' | xargs)

# Check required variables
required_vars=(
    "AVILADB_API_KEY"
    "AVL_AUTH_JWT_SECRET"
    "SESSION_SECRET"
)

for var in "${required_vars[@]}"; do
    if [ -z "${!var}" ]; then
        echo -e "${RED}❌ Error: $var is not set in .env${NC}"
        exit 1
    fi
done

echo -e "${GREEN}✓${NC} Environment variables loaded"

# Build Docker images
echo ""
echo "📦 Building Docker images..."
docker-compose build

echo -e "${GREEN}✓${NC} Docker images built successfully"

# Stop existing containers
echo ""
echo "🛑 Stopping existing containers..."
docker-compose down

# Start services
echo ""
echo "🚀 Starting AVL Platform services..."
docker-compose up -d

# Wait for services to be healthy
echo ""
echo "⏳ Waiting for services to be healthy..."
sleep 10

# Check health endpoints
services=("avl-console:3000" "aviladb:8000" "avl-auth:8080" "avx-telemetry:8888")
all_healthy=true

for service in "${services[@]}"; do
    IFS=':' read -r name port <<< "$service"
    if curl -f -s "http://localhost:$port/health" > /dev/null; then
        echo -e "${GREEN}✓${NC} $name is healthy"
    else
        echo -e "${RED}✗${NC} $name is not responding"
        all_healthy=false
    fi
done

if [ "$all_healthy" = true ]; then
    echo ""
    echo -e "${GREEN}✅ Deployment successful!${NC}"
    echo ""
    echo "🌐 AVL Console: http://localhost:3000"
    echo "📊 Metrics: http://localhost:9090"
    echo "📈 Grafana: http://localhost:3001"
    echo "📚 AvilaDB: http://localhost:8000"
    echo "🔐 AVL Auth: http://localhost:8080"
    echo ""
    echo "View logs with: docker-compose logs -f"
else
    echo ""
    echo -e "${YELLOW}⚠️  Some services are not healthy${NC}"
    echo "Check logs with: docker-compose logs"
    exit 1
fi
