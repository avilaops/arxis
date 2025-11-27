#!/usr/bin/env pwsh
# Quick reference commands for Financial Optimization Agent

Write-Host "Financial Optimization Agent - Quick Commands" -ForegroundColor Cyan
Write-Host "=============================================" -ForegroundColor Cyan
Write-Host ""

Write-Host "📦 Installation & Setup" -ForegroundColor Yellow
Write-Host "  cargo build              # Build debug version"
Write-Host "  cargo build --release    # Build optimized version"
Write-Host "  cargo test               # Run all tests"
Write-Host "  cargo run                # Start server (debug)"
Write-Host "  cargo run --release      # Start server (production)"
Write-Host ""

Write-Host "🐳 Docker Commands" -ForegroundColor Yellow
Write-Host "  docker build -t finopt .              # Build image"
Write-Host "  docker run -p 3000:3000 finopt       # Run container"
Write-Host "  docker-compose up -d                  # Start with compose"
Write-Host "  docker-compose down                   # Stop compose"
Write-Host "  docker-compose logs -f                # View logs"
Write-Host ""

Write-Host "🧪 Testing" -ForegroundColor Yellow
Write-Host "  cargo test                           # All tests"
Write-Host "  cargo test portugal_tax::tests       # Tax tests"
Write-Host "  cargo test vat_optimizer::tests      # VAT tests"
Write-Host "  cargo test -- --nocapture            # Show output"
Write-Host "  pwsh ./test_api.ps1                  # API integration tests"
Write-Host ""

Write-Host "🔧 Development" -ForegroundColor Yellow
Write-Host "  cargo watch -x run       # Auto-reload on changes"
Write-Host "  cargo fmt                # Format code"
Write-Host "  cargo clippy             # Lint code"
Write-Host "  cargo doc --open         # Generate & open docs"
Write-Host ""

Write-Host "📡 API Testing (Quick)" -ForegroundColor Yellow
Write-Host "Health Check:"
Write-Host '  curl http://localhost:3000/health' -ForegroundColor Gray
Write-Host ""
Write-Host "IRC Calculation:"
Write-Host '  curl -X POST http://localhost:3000/api/tax/portugal/irc \' -ForegroundColor Gray
Write-Host '    -H "Content-Type: application/json" \' -ForegroundColor Gray
Write-Host '    -d ''{"taxable_income":2000000,"location":"interior"}''' -ForegroundColor Gray
Write-Host ""
Write-Host "Break-Even Analysis:"
Write-Host '  curl -X POST http://localhost:3000/api/optimization/break-even \' -ForegroundColor Gray
Write-Host '    -H "Content-Type: application/json" \' -ForegroundColor Gray
Write-Host '    -d ''{"fixed_costs":500000,"variable_cost_per_unit":50,"price_per_unit":150}''' -ForegroundColor Gray
Write-Host ""

Write-Host "📊 Common Use Cases" -ForegroundColor Yellow
Write-Host "  See examples/use_cases.json" -ForegroundColor Gray
Write-Host "  See examples/API_EXAMPLES.md" -ForegroundColor Gray
Write-Host ""

Write-Host "📚 Documentation" -ForegroundColor Yellow
Write-Host "  README.md           # Main documentation"
Write-Host "  SETUP.md            # Setup & installation guide"
Write-Host "  API_EXAMPLES.md     # API usage examples"
Write-Host "  CHANGELOG.md        # Version history"
Write-Host "  PROJECT_SUMMARY.md  # Implementation summary"
Write-Host ""

Write-Host "🔍 Useful Checks" -ForegroundColor Yellow
Write-Host "  cargo check          # Fast compilation check"
Write-Host "  cargo tree           # View dependency tree"
Write-Host "  cargo audit          # Security audit"
Write-Host "  cargo outdated       # Check for updates"
Write-Host ""

Write-Host "🚀 Production Deployment" -ForegroundColor Yellow
Write-Host "  1. Build release: cargo build --release"
Write-Host "  2. Test: cargo test"
Write-Host "  3. Run: ./target/release/financial-optimization-agent"
Write-Host "  Or use Docker: docker-compose -f docker-compose.prod.yml up -d"
Write-Host ""

Write-Host "💡 Tips" -ForegroundColor Green
Write-Host "  • Set RUST_LOG=debug for verbose logging"
Write-Host "  • Use .env file for configuration"
Write-Host "  • Check /health endpoint for service status"
Write-Host "  • Review test_api.ps1 for integration examples"
Write-Host ""

Write-Host "📞 Support" -ForegroundColor Cyan
Write-Host "  Email: dev@avila.cloud"
Write-Host "  Docs:  https://docs.avila.cloud/agents/financial-optimization"
Write-Host ""
