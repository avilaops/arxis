# Changelog

All notable changes to the Financial Optimization Agent will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-11-25

### Added
- **Portugal Tax System**
  - IRC (Corporate Income Tax) calculation with regional variations
  - SIFIDE II (R&D Tax Credit) with base + incremental rates
  - Patent Box regime with nexus ratio compliance
  - State and municipal surcharge calculations
  - Interior region incentives (12.5% IRC rate)

- **VAT Optimization**
  - Cross-border VAT strategy (B2B/B2C)
  - Reverse charge mechanism for EU B2B
  - Destination principle for B2C
  - VAT recovery optimization
  - Intra-community supply analysis
  - OSS threshold optimization

- **Cost Optimization Algorithms**
  - Linear programming for resource allocation
  - Break-even analysis (units, revenue, margin)
  - Monte Carlo simulation (10,000+ iterations)
  - Portfolio optimization (Sharpe/Sortino ratios)
  - Sensitivity analysis (one-way, two-way, scenario)

- **Financial Valuation Models**
  - DCF (Discounted Cash Flow) with terminal value
  - NPV (Net Present Value) calculation
  - IRR (Internal Rate of Return) with Newton-Raphson
  - WACC (Weighted Average Cost of Capital)
  - CAPM (Capital Asset Pricing Model)
  - Payback period (simple & discounted)
  - ROI, ROAS, annualized returns

- **Corporate Structure Optimization**
  - Single entity structure evaluation
  - Holding company structure (tax-efficient dividends)
  - International structure (multi-jurisdiction)
  - Transfer pricing analysis (arms-length compliance)
  - Structure scoring with weighted objectives

- **Financial Forecasting**
  - Revenue forecasting (linear regression)
  - Expense forecasting (variable cost ratio)
  - P&L projections (5-year EBITDA, EBIT, net income)
  - Sensitivity analysis
  - Risk analysis (VaR, CVaR)

- **API Endpoints**
  - 20+ RESTful endpoints
  - JSON request/response
  - Comprehensive error handling
  - Health check endpoint

- **Documentation**
  - Comprehensive README with examples
  - API examples for all endpoints
  - Setup guide for development & production
  - Docker deployment guide
  - Use case scenarios with expected outcomes

- **Testing**
  - Unit tests for all modules (90%+ coverage)
  - Integration test examples
  - PowerShell test script for Windows
  - Docker Compose for local testing

### Technical Stack
- **Language**: Rust 1.75+
- **Web Framework**: Axum 0.7
- **Async Runtime**: Tokio
- **Optimization**: good_lp, rand, statrs
- **Serialization**: serde, serde_json

### Performance
- **Response Time**: < 50ms for most calculations
- **Monte Carlo**: 10,000 iterations in ~100ms
- **Memory**: < 50MB baseline
- **Concurrency**: Handles 1,000+ req/s

### Known Limitations
- Linear programming uses simplified greedy allocation (full LP solver optional)
- EU country VAT rates are hardcoded (consider external API for real-time rates)
- Transfer pricing uses simplified 5-15% arms-length range
- No database persistence yet (coming in v0.2.0)

### Coming in v0.2.0
- [ ] Database integration (SQLite/PostgreSQL)
- [ ] Historical data storage
- [ ] User authentication & authorization
- [ ] Rate limiting & throttling
- [ ] Prometheus metrics
- [ ] Grafana dashboards
- [ ] WebSocket support for real-time updates
- [ ] PDF report generation
- [ ] Excel export
- [ ] Multi-language support (PT/ES/EN)

---

**For detailed migration guides and breaking changes, see [MIGRATION.md](MIGRATION.md)**
