# Financial Optimization Agent - API Examples

## Portugal Tax Optimization Examples

### 1. Calculate IRC for Interior Region

**Scenario**: Company with €2M taxable income in Portugal interior

```bash
curl -X POST http://localhost:3000/api/tax/portugal/irc \
  -H "Content-Type: application/json" \
  -d '{
    "taxable_income": 2000000,
    "location": "interior"
  }'
```

**Response**:
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

**Savings**: 14.75% vs 23.5% in Lisbon = **€175k annual savings**

---

### 2. SIFIDE R&D Tax Credit

**Scenario**: Tech company with €500k R&D expenses (up from €400k)

```bash
curl -X POST http://localhost:3000/api/tax/portugal/sifide \
  -H "Content-Type: application/json" \
  -d '{
    "current_rd_expenses": 500000,
    "previous_2_years_avg": 400000
  }'
```

**Response**:
```json
{
  "base_credit": 162500,
  "incremental_credit": 50000,
  "total_credit": 212500,
  "credit_rate": 0.425
}
```

**Result**: **€212.5k tax credit** (42.5% of expenses)

---

### 3. Patent Box Optimization

**Scenario**: €1M IP income with 80% nexus ratio

```bash
curl -X POST http://localhost:3000/api/tax/portugal/patent-box \
  -H "Content-Type: application/json" \
  -d '{
    "ip_income": 1000000,
    "nexus_ratio": 0.8
  }'
```

**Response**:
```json
{
  "ip_income": 1000000,
  "qualified_income": 800000,
  "exempt_income": 400000,
  "tax_saving": 84000
}
```

**Result**: **€84k annual tax savings** from 50% exemption

---

## VAT Optimization Examples

### 4. B2B Cross-Border VAT (Reverse Charge)

**Scenario**: €100k software sale to German business

```bash
curl -X POST http://localhost:3000/api/vat/cross-border \
  -H "Content-Type: application/json" \
  -d '{
    "revenue": 100000,
    "customer_location": {"EU": "Germany"},
    "service_type": "Software",
    "is_business_customer": true
  }'
```

**Response**:
```json
{
  "strategy": "Reverse Charge",
  "vat_amount": 0,
  "effective_rate": 0
}
```

**Result**: **€0 VAT** (customer self-assesses)

---

### 5. B2C Cross-Border VAT

**Scenario**: €50k software sale to French consumers

```bash
curl -X POST http://localhost:3000/api/vat/cross-border \
  -H "Content-Type: application/json" \
  -d '{
    "revenue": 50000,
    "customer_location": {"EU": "France"},
    "service_type": "Software",
    "is_business_customer": false
  }'
```

**Response**:
```json
{
  "strategy": "Destination Principle",
  "vat_amount": 10000,
  "effective_rate": 0.20
}
```

**Result**: Apply **French 20% VAT** = €10k

---

## Financial Valuation Examples

### 6. DCF Valuation

**Scenario**: Value a company with 5-year cash flow projection

```bash
curl -X POST http://localhost:3000/api/valuation/dcf \
  -H "Content-Type: application/json" \
  -d '{
    "cash_flows": [1000000, 1200000, 1400000, 1600000, 1800000],
    "discount_rate": 0.10,
    "terminal_growth_rate": 0.03,
    "debt": 2000000,
    "cash": 500000,
    "shares_outstanding": 1000000
  }'
```

**Response**:
```json
{
  "npv": 18500000,
  "terminal_value": 25714285,
  "enterprise_value": 21200000,
  "pv_by_year": [909090, 991735, 1051770, 1092946, 1117420]
}
```

**Result**: **€18.5 per share** valuation

---

### 7. IRR Calculation

**Scenario**: €1M investment with annual returns

```bash
curl -X POST http://localhost:3000/api/valuation/irr \
  -H "Content-Type: application/json" \
  -d '{
    "cash_flows": [-1000000, 300000, 400000, 500000, 400000]
  }'
```

**Response**:
```json
{
  "irr": 0.1489,
  "irr_percentage": "14.89%"
}
```

**Decision**: Invest if **WACC < 14.89%**

---

## Cost Optimization Examples

### 8. Break-Even Analysis

**Scenario**: New product launch

```bash
curl -X POST http://localhost:3000/api/optimization/break-even \
  -H "Content-Type: application/json" \
  -d '{
    "fixed_costs": 500000,
    "variable_cost_per_unit": 50,
    "price_per_unit": 150
  }'
```

**Response**:
```json
{
  "break_even_units": 5000,
  "break_even_revenue": 750000,
  "contribution_margin_ratio": 0.6667,
  "margin_of_safety": 0.5
}
```

**Result**: Need **5,000 units** to break even

---

### 9. Monte Carlo Risk Simulation

**Scenario**: Forecast profit with uncertainty

```bash
curl -X POST http://localhost:3000/api/simulation/monte-carlo \
  -H "Content-Type: application/json" \
  -d '{
    "base_revenue": 10000000,
    "base_costs": 6000000,
    "revenue_volatility": 1000000,
    "cost_volatility": 500000,
    "iterations": 10000
  }'
```

**Response**:
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

**Interpretation**:
- **Expected profit**: €4M
- **95% confidence**: €2.5M - €5.5M range
- **Risk of loss**: 0.12%

---

### 10. Revenue Forecasting

**Scenario**: Project next 3 years based on historical data

```bash
curl -X POST http://localhost:3000/api/forecast/revenue \
  -H "Content-Type: application/json" \
  -d '{
    "historical_data": [1000000, 1200000, 1400000, 1600000],
    "periods_ahead": 3
  }'
```

**Response**:
```json
{
  "forecast": [1800000, 2000000, 2200000],
  "historical": [1000000, 1200000, 1400000, 1600000]
}
```

**Result**: Linear growth of **€200k/year**

---

## Corporate Structure Examples

### 11. Optimize Corporate Structure

**Scenario**: Multi-country operations

```bash
curl -X POST http://localhost:3000/api/structure/optimize \
  -H "Content-Type: application/json" \
  -d '{
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
  }'
```

**Response** (abbreviated):
```json
[
  {
    "structure": {"Holding": {...}},
    "effective_tax_rate": 0.145,
    "compliance_cost": 25000,
    "total_score": 0.82,
    "pros": [
      "Tax-efficient dividend flow",
      "IP can be held at holding level",
      "Participation exemption benefits"
    ]
  }
]
```

**Recommendation**: **Holding structure** with Madeira holding company

---

## Complete Workflow Example

### Startup Optimization (All Services)

```bash
# Step 1: Calculate base IRC
curl -X POST http://localhost:3000/api/tax/portugal/irc \
  -H "Content-Type: application/json" \
  -d '{"taxable_income": 500000, "location": "lisbon"}'

# Step 2: Add SIFIDE credit
curl -X POST http://localhost:3000/api/tax/portugal/sifide \
  -H "Content-Type: application/json" \
  -d '{"current_rd_expenses": 200000, "previous_2_years_avg": 150000}'

# Step 3: Optimize with Patent Box
curl -X POST http://localhost:3000/api/tax/portugal/patent-box \
  -H "Content-Type: application/json" \
  -d '{"ip_income": 100000, "nexus_ratio": 0.8}'

# Step 4: Get comprehensive optimization
curl -X POST http://localhost:3000/api/tax/effective-rate \
  -H "Content-Type: application/json" \
  -d '{
    "taxable_income": 500000,
    "location": "interior",
    "rd_expenses": 200000,
    "ip_income": 100000,
    "nexus_ratio": 0.8
  }'
```

**Combined Result**: **9.8% effective tax rate** vs 23.5% = **€68.5k annual savings**

---

For more examples, see `examples/use_cases.json`
