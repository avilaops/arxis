# Financial Optimization Agent 🇵🇹

**Expert Financial Optimization Specialist** focusing on tax optimization, cost minimization, financial modeling, and ROI analysis for Portugal expansion.

## 🎯 Core Capabilities

### 1. Portugal Tax System Optimization
- **IRC (Corporate Income Tax)**: Calculate effective rates with location-based incentives
- **SIFIDE II (R&D Tax Credit)**: Up to 82.5% credit on eligible R&D expenses
- **Patent Box Regime**: 50% exemption on qualified IP income
- **State & Municipal Surcharges**: Progressive taxation above €1.5M
- **Interior Incentives**: 12.5% IRC rate vs 21% standard

### 2. VAT Optimization
- **Cross-Border VAT**: B2B reverse charge, B2C destination principle
- **Intra-Community Supplies**: VAT-exempt transactions
- **VAT Recovery**: Optimize deductible expenses
- **OSS (One-Stop Shop)**: €10,000 threshold optimization

### 3. Cost Optimization Algorithms
- **Linear Programming**: Resource allocation across cities
- **Break-Even Analysis**: Units, revenue, margin of safety
- **Monte Carlo Simulation**: Financial risk analysis (10,000+ iterations)
- **Portfolio Optimization**: Sharpe/Sortino ratios, CAPM

### 4. Financial Valuation Models
- **DCF (Discounted Cash Flow)**: NPV, terminal value, enterprise value
- **IRR (Internal Rate of Return)**: Newton-Raphson convergence
- **WACC**: Weighted average cost of capital
- **Payback Period**: Simple & discounted

### 5. Corporate Structure Optimization
- **Single Entity**: Simple, low compliance
- **Holding Structure**: Tax-efficient dividends, IP protection
- **International**: Multi-jurisdiction optimization
- **Transfer Pricing**: Arms-length compliant (5-15% markup)

### 6. Financial Forecasting & Sensitivity
- **Revenue Forecasting**: Linear regression, time series
- **P&L Projections**: 5-year EBITDA, EBIT, net income
- **Sensitivity Analysis**: One-way, two-way, scenario analysis
- **Risk Analysis**: VaR, CVaR at 95% confidence

## 🚀 Quick Start

### Installation

```bash
# Clone repository
git clone https://github.com/avelan/copilot-agents
cd financial-optimization

# Install dependencies
cargo build --release

# Run server
cargo run --release
```

### Docker

```bash
docker build -t financial-optimization-agent .
docker run -p 3000:3000 financial-optimization-agent
```

## 📊 API Endpoints

### Tax Optimization

#### Calculate IRC (Corporate Income Tax)
```bash
POST /api/tax/portugal/irc
Content-Type: application/json

{
  "taxable_income": 2000000,
  "location": "interior"
}
```

**Response:**
```json
{
  "taxable_income": 2000000,
  "base_rate": 0.125,
  "state_surcharge": 0.0075,
  "municipal_surcharge": 0.015,
  "total_tax": 295000,
  "effective_rate": 0.1475
}
```

#### Calculate SIFIDE (R&D Tax Credit)
```bash
POST /api/tax/portugal/sifide
Content-Type: application/json

{
  "current_rd_expenses": 500000,
  "previous_2_years_avg": 400000
}
```

**Response:**
```json
{
  "base_credit": 162500,
  "incremental_credit": 50000,
  "total_credit": 212500,
  "credit_rate": 0.425
}
```

#### Calculate Patent Box
```bash
POST /api/tax/portugal/patent-box
Content-Type: application/json

{
  "ip_income": 1000000,
  "nexus_ratio": 0.8
}
```

**Response:**
```json
{
  "ip_income": 1000000,
  "qualified_income": 800000,
  "exempt_income": 400000,
  "tax_saving": 84000
}
```

### VAT Optimization

#### Optimize Cross-Border VAT
```bash
POST /api/vat/cross-border
Content-Type: application/json

{
  "revenue": 100000,
  "customer_location": { "EU": "Germany" },
  "service_type": "Software",
  "is_business_customer": true
}
```

**Response:**
```json
{
  "strategy": "Reverse Charge",
  "vat_amount": 0,
  "effective_rate": 0
}
```

### Cost Optimization

#### Break-Even Analysis
```bash
POST /api/optimization/break-even
Content-Type: application/json

{
  "fixed_costs": 500000,
  "variable_cost_per_unit": 50,
  "price_per_unit": 150
}
```

**Response:**
```json
{
  "break_even_units": 5000,
  "break_even_revenue": 750000,
  "contribution_margin_ratio": 0.6667,
  "margin_of_safety": 0.5
}
```

### Financial Valuation

#### DCF Valuation
```bash
POST /api/valuation/dcf
Content-Type: application/json

{
  "cash_flows": [1000000, 1200000, 1400000, 1600000, 1800000],
  "discount_rate": 0.10,
  "terminal_growth_rate": 0.03,
  "debt": 2000000,
  "cash": 500000,
  "shares_outstanding": 1000000
}
```

**Response:**
```json
{
  "npv": 18500000,
  "terminal_value": 25714285,
  "enterprise_value": 21200000,
  "pv_by_year": [909090, 991735, 1051770, 1092946, 1117420]
}
```

#### IRR Calculation
```bash
POST /api/valuation/irr
Content-Type: application/json

{
  "cash_flows": [-1000000, 300000, 400000, 500000, 400000]
}
```

**Response:**
```json
{
  "irr": 0.1489,
  "irr_percentage": "14.89%"
}
```

### Monte Carlo Simulation

```bash
POST /api/simulation/monte-carlo
Content-Type: application/json

{
  "base_revenue": 10000000,
  "base_costs": 6000000,
  "revenue_volatility": 1000000,
  "cost_volatility": 500000,
  "iterations": 10000
}
```

**Response:**
```json
{
  "mean": 4000000,
  "median": 4005000,
  "percentile_5": 2500000,
  "percentile_95": 5500000,
  "probability_loss": 0.0012,
  "standard_deviation": 1118033,
  "coefficient_of_variation": 0.2795
}
```

### Revenue Forecasting

```bash
POST /api/forecast/revenue
Content-Type: application/json

{
  "historical_data": [1000000, 1200000, 1400000, 1600000],
  "periods_ahead": 3
}
```

**Response:**
```json
{
  "forecast": [1800000, 2000000, 2200000],
  "historical": [1000000, 1200000, 1400000, 1600000]
}
```

## 🧮 Use Cases

### 1. Startup Launching in Portugal
**Objective**: Minimize tax burden while maximizing R&D incentives

```bash
# Calculate optimal location
curl -X POST http://localhost:3000/api/tax/effective-rate \
  -H "Content-Type: application/json" \
  -d '{
    "taxable_income": 500000,
    "location": "interior",
    "rd_expenses": 200000,
    "ip_income": 0,
    "nexus_ratio": 0
  }'
```

**Result**: Interior location with SIFIDE = **9.8% effective tax rate** vs 23.5% in Lisbon

### 2. SaaS Company Expanding to EU
**Objective**: Optimize VAT for B2B/B2C customers

```bash
# B2B customer (reverse charge)
curl -X POST http://localhost:3000/api/vat/cross-border \
  -H "Content-Type: application/json" \
  -d '{
    "revenue": 1000000,
    "customer_location": {"EU": "France"},
    "service_type": "Software",
    "is_business_customer": true
  }'
```

**Result**: 0% VAT (reverse charge) vs 20% French VAT = **€200k savings**

### 3. Tech Company with IP Portfolio
**Objective**: Maximize Patent Box benefits

```bash
curl -X POST http://localhost:3000/api/tax/portugal/patent-box \
  -H "Content-Type: application/json" \
  -d '{
    "ip_income": 5000000,
    "nexus_ratio": 0.9
  }'
```

**Result**: 50% exemption = **€472,500 annual tax savings**

### 4. Investment Decision Analysis
**Objective**: Evaluate project NPV and IRR

```bash
# Calculate IRR
curl -X POST http://localhost:3000/api/valuation/irr \
  -H "Content-Type: application/json" \
  -d '{
    "cash_flows": [-5000000, 1200000, 1500000, 1800000, 2000000, 2200000]
  }'
```

**Result**: IRR = **18.5%** → Invest if WACC < 18.5%

## 📈 Portugal Tax System Comparison

| Feature | Lisbon | Interior | Madeira | EU Average |
|---------|---------|----------|---------|------------|
| **IRC Rate** | 21% | 12.5% | 14% | 21.3% |
| **SIFIDE** | ✅ 82.5% | ✅ 82.5% | ✅ 82.5% | ❌ Limited |
| **Patent Box** | ✅ 50% | ✅ 50% | ✅ 50% | ~30% |
| **VAT** | 23% | 23% | 22% | 21% |
| **Effective Rate** (with incentives) | 15-17% | **9-11%** | 10-12% | 18-22% |

## 🔬 Advanced Features

### 1. Corporate Structure Optimization

```bash
POST /api/structure/optimize
Content-Type: application/json

{
  "revenue_by_country": {
    "Portugal": 2000000,
    "Spain": 1000000,
    "France": 500000
  },
  "objectives": {
    "minimize_tax": true,
    "minimize_compliance_cost": true,
    "maximize_flexibility": true,
    "weights": {
      "tax": 0.5,
      "compliance": 0.3,
      "flexibility": 0.2
    }
  }
}
```

**Returns**: Top 3 structures ranked by weighted score:
1. **Holding Structure** (Score: 0.82)
2. International Structure (Score: 0.75)
3. Single Entity (Score: 0.68)

### 2. Sensitivity Analysis

```bash
POST /api/sensitivity/analysis
Content-Type: application/json

{
  "base_revenue": 5000000,
  "base_costs": 3000000,
  "revenue_variation": 0.2,
  "cost_variation": 0.15
}
```

**Returns**: 5 scenarios (base, ±20% revenue, ±15% costs)

### 3. Risk Analysis (VaR/CVaR)

Uses Monte Carlo simulation to calculate:
- **Value at Risk (95%)**: Maximum expected loss
- **Conditional VaR**: Average loss in worst 5% scenarios
- **Probability of Loss**: < 1% for healthy businesses

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run specific module tests
cargo test portugal_tax::tests
cargo test vat_optimizer::tests
cargo test optimization::tests
cargo test financial_models::tests

# Run with output
cargo test -- --nocapture
```

## 📦 Dependencies

- **axum**: Web framework
- **tokio**: Async runtime
- **serde**: Serialization
- **good_lp**: Linear programming
- **rand**: Random number generation
- **statrs**: Statistical distributions
- **chrono**: Date/time handling

## 🛠️ Development

```bash
# Watch mode
cargo watch -x run

# Format code
cargo fmt

# Lint
cargo clippy

# Build for production
cargo build --release
```

## 📚 Documentation

```bash
# Generate docs
cargo doc --open
```

## 🌍 Portugal-Specific Expertise

### Interior Region Benefits
- **IRC Rate**: 12.5% (vs 21% standard)
- **EU Structural Funds**: Up to 45% grants
- **Social Security**: 50% reduction for new hires
- **RFAI**: Additional 10% investment deduction

### Startup Incentives
- **Startup Visa**: Fast-track residence
- **Tech Visa**: Expedited work permits
- **IRS Young Regime**: 50% income exemption (under 35)

### R&D Incentives
- **SIFIDE II**: 32.5% base + 50% incremental = **up to 82.5%**
- **Patent Box**: 50% exemption on IP income
- **Horizon Europe**: EU R&D funding

## 🔒 Security & Compliance

- ✅ GDPR compliant
- ✅ SOC 2 Type II (in progress)
- ✅ ISO 27001 aligned
- ✅ Arms-length transfer pricing (OECD guidelines)

## 📄 License

MIT License - See LICENSE file

## 🤝 Contributing

Contributions welcome! Please read CONTRIBUTING.md

## 📞 Support

- **Email**: dev@avila.cloud
- **Docs**: https://docs.avila.cloud/agents/financial-optimization
- **Slack**: #financial-optimization

---

**Built with ❤️ for Portugal expansion** 🇵🇹

**AVL Cloud Platform** - Database genuíno para Brasil e LATAM
