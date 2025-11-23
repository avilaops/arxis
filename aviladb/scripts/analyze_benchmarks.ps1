#!/usr/bin/env pwsh
# AvilaDB Benchmark Analysis Script
# Analyzes Criterion benchmark results and generates reports

param(
    [string]$BenchmarkDir = "target/criterion",
    [string]$OutputDir = "benchmark_results",
    [switch]$GenerateHTML,
    [switch]$CompareWithPrevious
)

Write-Host "🚀 AvilaDB Benchmark Analysis" -ForegroundColor Cyan
Write-Host "================================" -ForegroundColor Cyan
Write-Host ""

# Create output directory
New-Item -ItemType Directory -Force -Path $OutputDir | Out-Null

# Function to parse Criterion JSON results
function Parse-CriterionResults {
    param([string]$Path)

    $results = @()

    Get-ChildItem -Path $Path -Recurse -Filter "estimates.json" | ForEach-Object {
        $json = Get-Content $_.FullName | ConvertFrom-Json
        $benchmarkName = $_.Directory.Parent.Name

        $result = [PSCustomObject]@{
            Name       = $benchmarkName
            Mean       = $json.mean.point_estimate / 1000000  # Convert to ms
            StdDev     = $json.std_dev.point_estimate / 1000000
            P50        = $json.median.point_estimate / 1000000
            P95        = if ($json.slope) { $json.slope.point_estimate / 1000000 } else { $null }
            Throughput = $json.throughput
        }

        $results += $result
    }

    return $results
}

# Parse results
Write-Host "📊 Parsing benchmark results..." -ForegroundColor Yellow

if (Test-Path $BenchmarkDir) {
    $results = Parse-CriterionResults -Path $BenchmarkDir

    if ($results.Count -eq 0) {
        Write-Host "❌ No benchmark results found. Run benchmarks first:" -ForegroundColor Red
        Write-Host "   cargo bench" -ForegroundColor White
        exit 1
    }

    Write-Host "✅ Found $($results.Count) benchmark results" -ForegroundColor Green
    Write-Host ""

    # Summary Report
    Write-Host "📈 BENCHMARK SUMMARY" -ForegroundColor Cyan
    Write-Host "====================" -ForegroundColor Cyan
    Write-Host ""

    # Group by category
    $categories = @{
        "CRUD Operations" = $results | Where-Object { $_.Name -match "insert|query|update|delete" }
        "Compression"     = $results | Where-Object { $_.Name -match "compression|lz4|zstd" }
        "Vector Search"   = $results | Where-Object { $_.Name -match "vector" }
        "Concurrency"     = $results | Where-Object { $_.Name -match "concurrent|mixed" }
        "Workloads"       = $results | Where-Object { $_.Name -match "workload|game|chat|iot" }
        "Comparison"      = $results | Where-Object { $_.Name -match "dynamodb|cosmosdb|brazil" }
    }

    foreach ($category in $categories.Keys) {
        $categoryResults = $categories[$category]

        if ($categoryResults.Count -gt 0) {
            Write-Host "$category" -ForegroundColor Yellow
            Write-Host ("-" * 80) -ForegroundColor Gray

            $categoryResults | Sort-Object Mean | Format-Table -Property @(
                @{Label = "Benchmark"; Expression = { $_.Name }; Width = 40 }
                @{Label = "Mean (ms)"; Expression = { "{0:N3}" -f $_.Mean }; Width = 15; Align = "Right" }
                @{Label = "StdDev (ms)"; Expression = { "{0:N3}" -f $_.StdDev }; Width = 15; Align = "Right" }
                @{Label = "Median (ms)"; Expression = { "{0:N3}" -f $_.P50 }; Width = 15; Align = "Right" }
            ) -AutoSize

            Write-Host ""
        }
    }

    # Performance Highlights
    Write-Host "⚡ PERFORMANCE HIGHLIGHTS" -ForegroundColor Cyan
    Write-Host "=========================" -ForegroundColor Cyan
    Write-Host ""

    # Fastest operations
    $fastest = $results | Sort-Object Mean | Select-Object -First 5
    Write-Host "🏆 Top 5 Fastest Operations:" -ForegroundColor Green
    $fastest | ForEach-Object {
        Write-Host ("  • {0,-40} {1,10:N3} ms" -f $_.Name, $_.Mean) -ForegroundColor White
    }
    Write-Host ""

    # Slowest operations
    $slowest = $results | Sort-Object Mean -Descending | Select-Object -First 5
    Write-Host "🐌 Top 5 Slowest Operations:" -ForegroundColor Red
    $slowest | ForEach-Object {
        Write-Host ("  • {0,-40} {1,10:N3} ms" -f $_.Name, $_.Mean) -ForegroundColor White
    }
    Write-Host ""

    # AvilaDB vs Competitors
    Write-Host "🥊 AVILADB vs COMPETITORS" -ForegroundColor Cyan
    Write-Host "==========================" -ForegroundColor Cyan
    Write-Host ""

    $aviladb = $results | Where-Object { $_.Name -match "aviladb" }
    $dynamodb = $results | Where-Object { $_.Name -match "dynamodb" }
    $cosmosdb = $results | Where-Object { $_.Name -match "cosmosdb" }

    if ($aviladb -and $dynamodb) {
        $improvement = (($dynamodb[0].Mean - $aviladb[0].Mean) / $dynamodb[0].Mean) * 100
        Write-Host ("  AvilaDB is {0:N1}% faster than DynamoDB" -f $improvement) -ForegroundColor Green
    }

    if ($aviladb -and $cosmosdb) {
        $improvement = (($cosmosdb[0].Mean - $aviladb[0].Mean) / $cosmosdb[0].Mean) * 100
        Write-Host ("  AvilaDB is {0:N1}% faster than Cosmos DB" -f $improvement) -ForegroundColor Green
    }
    Write-Host ""

    # Brazil Latency Comparison
    $brLatency = $results | Where-Object { $_.Name -match "brazil" }
    if ($brLatency.Count -gt 0) {
        Write-Host "🇧🇷 Brazil Latency:" -ForegroundColor Yellow
        $brLatency | ForEach-Object {
            $color = if ($_.Name -match "aviladb") { "Green" } else { "White" }
            Write-Host ("  • {0,-30} {1,8:N1} ms" -f $_.Name, $_.Mean) -ForegroundColor $color
        }
        Write-Host ""
    }

    # Export to JSON
    $jsonPath = Join-Path $OutputDir "benchmark_summary.json"
    $results | ConvertTo-Json -Depth 5 | Out-File $jsonPath
    Write-Host "💾 Results saved to: $jsonPath" -ForegroundColor Cyan

    # Generate CSV
    $csvPath = Join-Path $OutputDir "benchmark_summary.csv"
    $results | Export-Csv -Path $csvPath -NoTypeInformation
    Write-Host "💾 CSV exported to: $csvPath" -ForegroundColor Cyan

    # Generate HTML report
    if ($GenerateHTML) {
        Write-Host ""
        Write-Host "📄 Generating HTML report..." -ForegroundColor Yellow

        $htmlPath = Join-Path $OutputDir "benchmark_report.html"
        $html = @"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>AvilaDB Benchmark Report</title>
    <style>
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            max-width: 1200px;
            margin: 0 auto;
            padding: 20px;
            background: #0a0a0a;
            color: #e0e0e0;
        }
        h1 { color: #00d9ff; border-bottom: 3px solid #00d9ff; padding-bottom: 10px; }
        h2 { color: #00ff9f; margin-top: 40px; }
        table {
            width: 100%;
            border-collapse: collapse;
            margin: 20px 0;
            background: #1a1a1a;
        }
        th, td {
            padding: 12px;
            text-align: left;
            border-bottom: 1px solid #333;
        }
        th {
            background: #00d9ff;
            color: #0a0a0a;
            font-weight: bold;
        }
        tr:hover { background: #252525; }
        .highlight { color: #00ff9f; font-weight: bold; }
        .metric { font-family: 'Courier New', monospace; }
        .footer {
            margin-top: 50px;
            text-align: center;
            color: #666;
            border-top: 1px solid #333;
            padding-top: 20px;
        }
    </style>
</head>
<body>
    <h1>🚀 AvilaDB Benchmark Report</h1>
    <p>Generated on $(Get-Date -Format "yyyy-MM-dd HH:mm:ss")</p>

    <h2>📊 All Benchmarks</h2>
    <table>
        <tr>
            <th>Benchmark</th>
            <th>Mean (ms)</th>
            <th>StdDev (ms)</th>
            <th>Median (ms)</th>
        </tr>
"@

        foreach ($result in ($results | Sort-Object Mean)) {
            $html += @"
        <tr>
            <td>$($result.Name)</td>
            <td class="metric">$("{0:N3}" -f $result.Mean)</td>
            <td class="metric">$("{0:N3}" -f $result.StdDev)</td>
            <td class="metric">$("{0:N3}" -f $result.P50)</td>
        </tr>
"@
        }

        $html += @"
    </table>

    <h2>⚡ Top Performers</h2>
    <table>
        <tr><th>Operation</th><th>Latency</th></tr>
"@

        foreach ($result in ($fastest | Select-Object -First 5)) {
            $html += @"
        <tr>
            <td>$($result.Name)</td>
            <td class="highlight metric">$("{0:N3}" -f $result.Mean) ms</td>
        </tr>
"@
        }

        $html += @"
    </table>

    <div class="footer">
        <p><strong>AvilaDB</strong> - Globally distributed NoSQL optimized for Brazil 🇧🇷</p>
        <p>Visit <a href="https://avila.cloud" style="color: #00d9ff;">avila.cloud</a> for more information</p>
    </div>
</body>
</html>
"@

        $html | Out-File $htmlPath -Encoding UTF8
        Write-Host "✅ HTML report saved to: $htmlPath" -ForegroundColor Green

        # Open in browser
        Start-Process $htmlPath
    }

}
else {
    Write-Host "❌ Benchmark directory not found: $BenchmarkDir" -ForegroundColor Red
    Write-Host "   Run benchmarks first: cargo bench" -ForegroundColor White
    exit 1
}

Write-Host ""
Write-Host "✅ Analysis complete!" -ForegroundColor Green
Write-Host ""
Write-Host "Next steps:" -ForegroundColor Cyan
Write-Host "  • Review the results in $OutputDir" -ForegroundColor White
Write-Host "  • Compare with previous runs: ./analyze_benchmarks.ps1 -CompareWithPrevious" -ForegroundColor White
Write-Host "  • Generate HTML report: ./analyze_benchmarks.ps1 -GenerateHTML" -ForegroundColor White
Write-Host ""
