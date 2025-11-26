#!/usr/bin/env pwsh
# Test script for Financial Optimization Agent

$BASE_URL = "http://localhost:3000"

Write-Host "🧪 Testing Financial Optimization Agent..." -ForegroundColor Cyan
Write-Host ""

# Test 1: Health Check
Write-Host "1️⃣ Testing Health Endpoint..." -ForegroundColor Yellow
$response = Invoke-RestMethod -Uri "$BASE_URL/health" -Method Get
Write-Host "✅ Health check passed" -ForegroundColor Green
Write-Host ($response | ConvertTo-Json -Depth 10)
Write-Host ""

# Test 2: IRC Calculation (Interior Region)
Write-Host "2️⃣ Testing IRC Calculation (Interior)..." -ForegroundColor Yellow
$ircRequest = @{
    taxable_income = 2000000
    location       = "interior"
} | ConvertTo-Json

$ircResponse = Invoke-RestMethod -Uri "$BASE_URL/api/tax/portugal/irc" -Method Post -Body $ircRequest -ContentType "application/json"
Write-Host "✅ IRC calculation completed" -ForegroundColor Green
Write-Host "Effective Rate: $($ircResponse.effective_rate * 100)%" -ForegroundColor Cyan
Write-Host "Total Tax: €$($ircResponse.total_tax)" -ForegroundColor Cyan
Write-Host ""

# Test 3: SIFIDE Tax Credit
Write-Host "3️⃣ Testing SIFIDE R&D Tax Credit..." -ForegroundColor Yellow
$sifideRequest = @{
    current_rd_expenses  = 500000
    previous_2_years_avg = 400000
} | ConvertTo-Json

$sifideResponse = Invoke-RestMethod -Uri "$BASE_URL/api/tax/portugal/sifide" -Method Post -Body $sifideRequest -ContentType "application/json"
Write-Host "✅ SIFIDE calculation completed" -ForegroundColor Green
Write-Host "Total Credit: €$($sifideResponse.total_credit)" -ForegroundColor Cyan
Write-Host "Credit Rate: $($sifideResponse.credit_rate * 100)%" -ForegroundColor Cyan
Write-Host ""

# Test 4: Patent Box
Write-Host "4️⃣ Testing Patent Box Calculation..." -ForegroundColor Yellow
$patentBoxRequest = @{
    ip_income   = 1000000
    nexus_ratio = 0.8
} | ConvertTo-Json

$patentBoxResponse = Invoke-RestMethod -Uri "$BASE_URL/api/tax/portugal/patent-box" -Method Post -Body $patentBoxRequest -ContentType "application/json"
Write-Host "✅ Patent Box calculation completed" -ForegroundColor Green
Write-Host "Tax Saving: €$($patentBoxResponse.tax_saving)" -ForegroundColor Cyan
Write-Host ""

# Test 5: Cross-Border VAT (B2B)
Write-Host "5️⃣ Testing Cross-Border VAT (B2B)..." -ForegroundColor Yellow
$vatB2BRequest = @{
    revenue              = 100000
    customer_location    = @{ EU = "Germany" }
    service_type         = "Software"
    is_business_customer = $true
} | ConvertTo-Json -Depth 10

$vatB2BResponse = Invoke-RestMethod -Uri "$BASE_URL/api/vat/cross-border" -Method Post -Body $vatB2BRequest -ContentType "application/json"
Write-Host "✅ VAT optimization completed" -ForegroundColor Green
Write-Host "Strategy: $($vatB2BResponse.strategy)" -ForegroundColor Cyan
Write-Host "VAT Amount: €$($vatB2BResponse.vat_amount)" -ForegroundColor Cyan
Write-Host ""

# Test 6: Break-Even Analysis
Write-Host "6️⃣ Testing Break-Even Analysis..." -ForegroundColor Yellow
$breakEvenRequest = @{
    fixed_costs            = 500000
    variable_cost_per_unit = 50
    price_per_unit         = 150
} | ConvertTo-Json

$breakEvenResponse = Invoke-RestMethod -Uri "$BASE_URL/api/optimization/break-even" -Method Post -Body $breakEvenRequest -ContentType "application/json"
Write-Host "✅ Break-even analysis completed" -ForegroundColor Green
Write-Host "Break-even Units: $($breakEvenResponse.break_even_units)" -ForegroundColor Cyan
Write-Host "Break-even Revenue: €$($breakEvenResponse.break_even_revenue)" -ForegroundColor Cyan
Write-Host ""

# Test 7: DCF Valuation
Write-Host "7️⃣ Testing DCF Valuation..." -ForegroundColor Yellow
$dcfRequest = @{
    cash_flows           = @(1000000, 1200000, 1400000, 1600000, 1800000)
    discount_rate        = 0.10
    terminal_growth_rate = 0.03
    debt                 = 2000000
    cash                 = 500000
    shares_outstanding   = 1000000
} | ConvertTo-Json

$dcfResponse = Invoke-RestMethod -Uri "$BASE_URL/api/valuation/dcf" -Method Post -Body $dcfRequest -ContentType "application/json"
Write-Host "✅ DCF valuation completed" -ForegroundColor Green
Write-Host "NPV: €$($dcfResponse.npv)" -ForegroundColor Cyan
Write-Host "Enterprise Value: €$($dcfResponse.enterprise_value)" -ForegroundColor Cyan
Write-Host ""

# Test 8: IRR Calculation
Write-Host "8️⃣ Testing IRR Calculation..." -ForegroundColor Yellow
$irrRequest = @{
    cash_flows = @(-1000000, 300000, 400000, 500000, 400000)
} | ConvertTo-Json

$irrResponse = Invoke-RestMethod -Uri "$BASE_URL/api/valuation/irr" -Method Post -Body $irrRequest -ContentType "application/json"
Write-Host "✅ IRR calculation completed" -ForegroundColor Green
Write-Host "IRR: $($irrResponse.irr_percentage)" -ForegroundColor Cyan
Write-Host ""

# Test 9: Monte Carlo Simulation
Write-Host "9️⃣ Testing Monte Carlo Simulation..." -ForegroundColor Yellow
$monteCarloRequest = @{
    base_revenue       = 10000000
    base_costs         = 6000000
    revenue_volatility = 1000000
    cost_volatility    = 500000
    iterations         = 10000
} | ConvertTo-Json

$monteCarloResponse = Invoke-RestMethod -Uri "$BASE_URL/api/simulation/monte-carlo" -Method Post -Body $monteCarloRequest -ContentType "application/json"
Write-Host "✅ Monte Carlo simulation completed" -ForegroundColor Green
Write-Host "Mean Profit: €$($monteCarloResponse.mean)" -ForegroundColor Cyan
Write-Host "5th Percentile: €$($monteCarloResponse.percentile_5)" -ForegroundColor Cyan
Write-Host "95th Percentile: €$($monteCarloResponse.percentile_95)" -ForegroundColor Cyan
Write-Host "Probability of Loss: $($monteCarloResponse.probability_loss * 100)%" -ForegroundColor Cyan
Write-Host ""

# Test 10: Revenue Forecast
Write-Host "🔟 Testing Revenue Forecast..." -ForegroundColor Yellow
$forecastRequest = @{
    historical_data = @(1000000, 1200000, 1400000, 1600000)
    periods_ahead   = 3
} | ConvertTo-Json

$forecastResponse = Invoke-RestMethod -Uri "$BASE_URL/api/forecast/revenue" -Method Post -Body $forecastRequest -ContentType "application/json"
Write-Host "✅ Revenue forecast completed" -ForegroundColor Green
Write-Host "Forecast:" -ForegroundColor Cyan
foreach ($value in $forecastResponse.forecast) {
    Write-Host "  €$value" -ForegroundColor Cyan
}
Write-Host ""

Write-Host "🎉 All tests completed successfully!" -ForegroundColor Green
