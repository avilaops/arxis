# AvilaDB Benchmark Commands
# Quick reference for running benchmarks

# Ensure scripts directory exists
New-Item -ItemType Directory -Force -Path scripts | Out-Null

# Define commands
$commands = @{
    "all" = "Run all benchmarks"
    "basic" = "Run basic CRUD operations"
    "compression" = "Run compression benchmarks"
    "vector" = "Run vector search benchmarks"
    "concurrency" = "Run concurrency tests"
    "latency" = "Run latency distribution tests"
    "workloads" = "Run real-world workload tests"
    "comparison" = "Run competitive comparison"
    "memory" = "Run memory profiling"
    "analyze" = "Analyze benchmark results"
    "report" = "Generate HTML report"
    "flamegraph" = "Generate CPU flamegraph"
    "clean" = "Clean benchmark artifacts"
}

function Show-Help {
    Write-Host "🚀 AvilaDB Benchmark Suite" -ForegroundColor Cyan
    Write-Host "============================" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "Available commands:" -ForegroundColor Yellow
    Write-Host ""

    foreach ($cmd in $commands.Keys | Sort-Object) {
        Write-Host ("  bench {0,-15} - {1}" -f $cmd, $commands[$cmd]) -ForegroundColor White
    }

    Write-Host ""
    Write-Host "Examples:" -ForegroundColor Yellow
    Write-Host "  .\bench.ps1 all         # Run all benchmarks" -ForegroundColor Gray
    Write-Host "  .\bench.ps1 basic       # Run CRUD operations only" -ForegroundColor Gray
    Write-Host "  .\bench.ps1 analyze     # Analyze results" -ForegroundColor Gray
    Write-Host "  .\bench.ps1 report      # Generate HTML report" -ForegroundColor Gray
    Write-Host ""
}

function Run-Benchmark {
    param([string]$Target)

    switch ($Target) {
        "all" {
            Write-Host "🚀 Running all benchmarks..." -ForegroundColor Cyan
            cargo bench
        }
        "basic" {
            Write-Host "📝 Running basic CRUD operations..." -ForegroundColor Cyan
            cargo bench --bench database_ops -- basic_ops
        }
        "compression" {
            Write-Host "📦 Running compression benchmarks..." -ForegroundColor Cyan
            cargo bench --bench database_ops -- compression
        }
        "vector" {
            Write-Host "🔍 Running vector search benchmarks..." -ForegroundColor Cyan
            cargo bench --bench database_ops -- vector_search
        }
        "concurrency" {
            Write-Host "⚡ Running concurrency tests..." -ForegroundColor Cyan
            cargo bench --bench database_ops -- concurrency
        }
        "latency" {
            Write-Host "📊 Running latency distribution tests..." -ForegroundColor Cyan
            cargo bench --bench database_ops -- latency
        }
        "workloads" {
            Write-Host "🎮 Running real-world workloads..." -ForegroundColor Cyan
            cargo bench --bench database_ops -- workloads
        }
        "comparison" {
            Write-Host "🥊 Running competitive comparison..." -ForegroundColor Cyan
            cargo bench --bench database_ops -- comparison
        }
        "memory" {
            Write-Host "💾 Running memory profiling..." -ForegroundColor Cyan
            cargo bench --bench database_ops -- memory
        }
        "analyze" {
            Write-Host "📈 Analyzing benchmark results..." -ForegroundColor Cyan
            if (Test-Path ".\scripts\analyze_benchmarks.ps1") {
                & ".\scripts\analyze_benchmarks.ps1"
            } else {
                Write-Host "❌ Analysis script not found" -ForegroundColor Red
            }
        }
        "report" {
            Write-Host "📄 Generating HTML report..." -ForegroundColor Cyan
            if (Test-Path ".\scripts\analyze_benchmarks.ps1") {
                & ".\scripts\analyze_benchmarks.ps1" -GenerateHTML
            } else {
                Write-Host "❌ Analysis script not found" -ForegroundColor Red
            }
        }
        "flamegraph" {
            Write-Host "🔥 Generating CPU flamegraph..." -ForegroundColor Cyan
            if (Get-Command "cargo-flamegraph" -ErrorAction SilentlyContinue) {
                cargo flamegraph --bench database_ops
            } else {
                Write-Host "Installing flamegraph..." -ForegroundColor Yellow
                cargo install flamegraph
                cargo flamegraph --bench database_ops
            }
        }
        "clean" {
            Write-Host "🧹 Cleaning benchmark artifacts..." -ForegroundColor Cyan
            Remove-Item -Recurse -Force -ErrorAction SilentlyContinue target/criterion
            Remove-Item -Recurse -Force -ErrorAction SilentlyContinue benchmark_results
            Write-Host "✅ Cleaned!" -ForegroundColor Green
        }
        default {
            Write-Host "❌ Unknown command: $Target" -ForegroundColor Red
            Write-Host ""
            Show-Help
            exit 1
        }
    }
}

# Main execution
if ($args.Count -eq 0) {
    Show-Help
} else {
    $command = $args[0]
    Run-Benchmark -Target $command
}
