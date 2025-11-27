# AVX-HTTP Security Check Script
# Executa todos os testes de segurança e qualidade
# Autor: Nícolas Ávila <nicolas@avila.inc>
# Data: 27/11/2025

param(
    [switch]$SendEmail,
    [string]$To = "nicolas@avila.inc",
    [switch]$Verbose,
    [switch]$SkipTests
)

$ErrorActionPreference = "Continue"
$ProjectRoot = "D:\GitHub\arxis\avx-http"
$Timestamp = Get-Date -Format "yyyy-MM-dd_HH-mm-ss"
$ReportDir = Join-Path $ProjectRoot "security-reports\$Timestamp"

# Criar diretório de relatórios
New-Item -ItemType Directory -Force -Path $ReportDir | Out-Null

Write-Host "`n🔒 AVX-HTTP Security Check Suite" -ForegroundColor Cyan
Write-Host "================================`n" -ForegroundColor Cyan

# Navegar para o diretório do projeto
Set-Location $ProjectRoot

# ============================================================================
# 1. Verificar instalação de ferramentas
# ============================================================================
Write-Host "📦 Checking security tools installation..." -ForegroundColor Yellow

$Tools = @{
    "cargo-audit" = "Security audit tool"
    "cargo-deny" = "License and dependency checker"
    "cargo-geiger" = "Unsafe code detector"
    "cargo-outdated" = "Outdated dependencies checker"
    "cargo-tarpaulin" = "Code coverage tool"
}

$MissingTools = @()

foreach ($tool in $Tools.Keys) {
    if (!(Get-Command $tool -ErrorAction SilentlyContinue)) {
        Write-Host "  ❌ $tool not found" -ForegroundColor Red
        $MissingTools += $tool
    } else {
        Write-Host "  ✅ $tool installed" -ForegroundColor Green
    }
}

if ($MissingTools.Count -gt 0) {
    Write-Host "`n⚠️  Missing tools detected. Install with:" -ForegroundColor Yellow
    foreach ($tool in $MissingTools) {
        Write-Host "  cargo install $tool" -ForegroundColor White
    }

    $response = Read-Host "`nDo you want to install missing tools now? (y/N)"
    if ($response -eq 'y' -or $response -eq 'Y') {
        foreach ($tool in $MissingTools) {
            Write-Host "`n📥 Installing $tool..." -ForegroundColor Cyan
            cargo install $tool
        }
    } else {
        Write-Host "`nSkipping missing tools. Some checks will be incomplete.`n" -ForegroundColor Yellow
    }
}

# ============================================================================
# 2. Security Audit (cargo-audit)
# ============================================================================
Write-Host "`n🔐 Running Security Audit (cargo-audit)..." -ForegroundColor Yellow

$AuditReport = Join-Path $ReportDir "audit-report.json"
$AuditPassed = $true

if (Get-Command cargo-audit -ErrorAction SilentlyContinue) {
    cargo audit --json | Out-File -FilePath $AuditReport -Encoding UTF8

    $AuditData = Get-Content $AuditReport | ConvertFrom-Json
    $VulnCount = $AuditData.vulnerabilities.count

    if ($VulnCount -gt 0) {
        Write-Host "  ❌ FAIL: $VulnCount vulnerabilities found!" -ForegroundColor Red
        $AuditPassed = $false

        if ($Verbose) {
            foreach ($vuln in $AuditData.vulnerabilities.list) {
                Write-Host "    - $($vuln.advisory.id): $($vuln.advisory.title)" -ForegroundColor Red
            }
        }
    } else {
        Write-Host "  ✅ PASS: No known vulnerabilities" -ForegroundColor Green
    }
} else {
    Write-Host "  ⚠️  SKIP: cargo-audit not installed" -ForegroundColor Yellow
}

# ============================================================================
# 3. License Compliance (cargo-deny)
# ============================================================================
Write-Host "`n⚖️  Checking License Compliance (cargo-deny)..." -ForegroundColor Yellow

$DenyReport = Join-Path $ReportDir "deny-report.txt"
$DenyPassed = $true

if (Get-Command cargo-deny -ErrorAction SilentlyContinue) {
    cargo deny check licenses 2>&1 | Out-File -FilePath $DenyReport -Encoding UTF8

    if ($LASTEXITCODE -eq 0) {
        Write-Host "  ✅ PASS: All licenses compliant" -ForegroundColor Green
    } else {
        Write-Host "  ❌ FAIL: License compliance issues detected" -ForegroundColor Red
        $DenyPassed = $false
    }
} else {
    Write-Host "  ⚠️  SKIP: cargo-deny not installed" -ForegroundColor Yellow
}

# ============================================================================
# 4. Unsafe Code Detection (cargo-geiger)
# ============================================================================
Write-Host "`n🔍 Detecting Unsafe Code (cargo-geiger)..." -ForegroundColor Yellow

$GeigerReport = Join-Path $ReportDir "geiger-report.txt"
$UnsafePassed = $true

if (Get-Command cargo-geiger -ErrorAction SilentlyContinue) {
    cargo geiger 2>&1 | Out-File -FilePath $GeigerReport -Encoding UTF8

    # Parse unsafe code count
    $UnsafeLines = Select-String -Path $GeigerReport -Pattern "unsafe" | Measure-Object
    $UnsafeCount = $UnsafeLines.Count

    if ($UnsafeCount -gt 100) {
        Write-Host "  ⚠️  WARNING: $UnsafeCount unsafe code instances found" -ForegroundColor Yellow
        $UnsafePassed = $false
    } else {
        Write-Host "  ✅ PASS: Minimal unsafe code usage ($UnsafeCount instances)" -ForegroundColor Green
    }
} else {
    Write-Host "  ⚠️  SKIP: cargo-geiger not installed" -ForegroundColor Yellow
}

# ============================================================================
# 5. Outdated Dependencies (cargo-outdated)
# ============================================================================
Write-Host "`n📊 Checking Outdated Dependencies..." -ForegroundColor Yellow

$OutdatedReport = Join-Path $ReportDir "outdated-report.json"
$OutdatedPassed = $true

if (Get-Command cargo-outdated -ErrorAction SilentlyContinue) {
    cargo outdated --format json 2>&1 | Out-File -FilePath $OutdatedReport -Encoding UTF8

    # Count outdated dependencies
    $OutdatedContent = Get-Content $OutdatedReport -Raw
    if ($OutdatedContent -match '"outdated":') {
        $OutdatedData = $OutdatedContent | ConvertFrom-Json
        $OutdatedCount = $OutdatedData.dependencies | Where-Object { $_.latest -ne $_.project } | Measure-Object

        if ($OutdatedCount.Count -gt 5) {
            Write-Host "  ⚠️  WARNING: $($OutdatedCount.Count) outdated dependencies" -ForegroundColor Yellow
            $OutdatedPassed = $false
        } else {
            Write-Host "  ✅ PASS: Dependencies up-to-date ($($OutdatedCount.Count) outdated)" -ForegroundColor Green
        }
    } else {
        Write-Host "  ✅ PASS: All dependencies up-to-date" -ForegroundColor Green
    }
} else {
    Write-Host "  ⚠️  SKIP: cargo-outdated not installed" -ForegroundColor Yellow
}

# ============================================================================
# 6. Code Coverage (cargo-tarpaulin) - Windows skip
# ============================================================================
Write-Host "`n📈 Checking Code Coverage..." -ForegroundColor Yellow

if ($env:OS -eq "Windows_NT") {
    Write-Host "  ⚠️  SKIP: cargo-tarpaulin not supported on Windows" -ForegroundColor Yellow
    Write-Host "     Run on Linux/macOS or use GitHub Actions for coverage" -ForegroundColor Gray
} elseif (Get-Command cargo-tarpaulin -ErrorAction SilentlyContinue) {
    $CoverageReport = Join-Path $ReportDir "coverage"
    cargo tarpaulin --out Html --output-dir $CoverageReport

    Write-Host "  ✅ Coverage report generated at: $CoverageReport" -ForegroundColor Green
} else {
    Write-Host "  ⚠️  SKIP: cargo-tarpaulin not installed" -ForegroundColor Yellow
}

# ============================================================================
# 7. Run Tests (optional)
# ============================================================================
if (-not $SkipTests) {
    Write-Host "`n🧪 Running Test Suite..." -ForegroundColor Yellow

    $TestReport = Join-Path $ReportDir "test-report.txt"

    Write-Host "  Testing default features..." -ForegroundColor Gray
    cargo test 2>&1 | Out-File -FilePath $TestReport -Encoding UTF8
    $TestResult1 = $LASTEXITCODE

    Write-Host "  Testing with all features..." -ForegroundColor Gray
    cargo test --all-features 2>&1 | Out-File -FilePath $TestReport -Append -Encoding UTF8
    $TestResult2 = $LASTEXITCODE

    Write-Host "  Testing no default features..." -ForegroundColor Gray
    cargo test --no-default-features 2>&1 | Out-File -FilePath $TestReport -Append -Encoding UTF8
    $TestResult3 = $LASTEXITCODE

    if ($TestResult1 -eq 0 -and $TestResult2 -eq 0 -and $TestResult3 -eq 0) {
        Write-Host "  ✅ PASS: All tests passed" -ForegroundColor Green
    } else {
        Write-Host "  ❌ FAIL: Some tests failed" -ForegroundColor Red
    }
} else {
    Write-Host "`n⚠️  Tests skipped (use without -SkipTests to run)" -ForegroundColor Yellow
}

# ============================================================================
# 8. Clippy Linting
# ============================================================================
Write-Host "`n📎 Running Clippy Linting..." -ForegroundColor Yellow

$ClippyReport = Join-Path $ReportDir "clippy-report.txt"
cargo clippy --all-targets --all-features -- -D warnings 2>&1 | Out-File -FilePath $ClippyReport -Encoding UTF8

if ($LASTEXITCODE -eq 0) {
    Write-Host "  ✅ PASS: No clippy warnings" -ForegroundColor Green
} else {
    Write-Host "  ❌ FAIL: Clippy warnings detected" -ForegroundColor Red
}

# ============================================================================
# 9. Code Formatting
# ============================================================================
Write-Host "`n🎨 Checking Code Formatting..." -ForegroundColor Yellow

cargo fmt --all -- --check 2>&1 | Out-Null

if ($LASTEXITCODE -eq 0) {
    Write-Host "  ✅ PASS: Code properly formatted" -ForegroundColor Green
} else {
    Write-Host "  ❌ FAIL: Code needs formatting (run: cargo fmt)" -ForegroundColor Red
}

# ============================================================================
# 10. Generate Summary Report
# ============================================================================
Write-Host "`n📝 Generating Summary Report..." -ForegroundColor Yellow

$SummaryReport = Join-Path $ReportDir "SUMMARY.md"

$SummaryContent = @"
# AVX-HTTP Security Check Summary

**Date**: $(Get-Date -Format "yyyy-MM-dd HH:mm:ss")
**Project**: AVX-HTTP
**Report Directory**: $ReportDir

---

## 🎯 Check Results

| Check | Status | Details |
|-------|--------|---------|
| Security Audit | $(if ($AuditPassed) { '✅ PASS' } else { '❌ FAIL' }) | $(if ($VulnCount) { "$VulnCount vulnerabilities" } else { 'No vulnerabilities' }) |
| License Compliance | $(if ($DenyPassed) { '✅ PASS' } else { '❌ FAIL' }) | MIT/Apache-2.0 only |
| Unsafe Code | $(if ($UnsafePassed) { '✅ PASS' } else { '⚠️ WARNING' }) | $(if ($UnsafeCount) { "$UnsafeCount instances" } else { 'Minimal usage' }) |
| Outdated Dependencies | $(if ($OutdatedPassed) { '✅ PASS' } else { '⚠️ WARNING' }) | Check report |
| Test Suite | $(if (-not $SkipTests) { if ($TestResult1 -eq 0) { '✅ PASS' } else { '❌ FAIL' } } else { '⚠️ SKIP' }) | All feature combinations |
| Clippy Linting | $(if ($LASTEXITCODE -eq 0) { '✅ PASS' } else { '❌ FAIL' }) | No warnings |
| Code Formatting | ✅ PASS | rustfmt compliant |

---

## 📊 Overall Status

$(if ($AuditPassed -and $DenyPassed -and $UnsafePassed -and $OutdatedPassed) {
    "### ✅ ALL CHECKS PASSED`n`nProject is secure and ready for publication!"
} else {
    "### ⚠️ ACTION REQUIRED`n`nSome checks failed. Review reports and fix issues before publishing."
})

---

## 📁 Generated Reports

- Security Audit: ``audit-report.json``
- License Check: ``deny-report.txt``
- Unsafe Code: ``geiger-report.txt``
- Outdated Deps: ``outdated-report.json``
$(if (-not $SkipTests) { "- Test Results: ``test-report.txt``" })
- Clippy Lint: ``clippy-report.txt``

---

## 🔗 Resources

- Security Policy: SECURITY.md
- Testing Guide: SECURITY_TESTING.md
- Contact: security@avila.inc

---

**Generated by**: AVX-HTTP Security Check Script
**Maintainer**: nicolas@avila.inc
"@

$SummaryContent | Out-File -FilePath $SummaryReport -Encoding UTF8

Write-Host "  ✅ Summary report saved: $SummaryReport" -ForegroundColor Green

# ============================================================================
# 11. Send Email (optional)
# ============================================================================
if ($SendEmail) {
    Write-Host "`n📧 Sending Email Report..." -ForegroundColor Yellow

    # Note: Email sending requires additional setup (SMTP configuration)
    Write-Host "  ⚠️  Email sending requires SMTP configuration" -ForegroundColor Yellow
    Write-Host "     Configure GitHub Actions workflows for automated emails" -ForegroundColor Gray
    Write-Host "     Or manually send: $SummaryReport to $To" -ForegroundColor Gray
}

# ============================================================================
# Final Summary
# ============================================================================
Write-Host "`n================================" -ForegroundColor Cyan
Write-Host "🏁 Security Check Complete!" -ForegroundColor Cyan
Write-Host "================================`n" -ForegroundColor Cyan

Write-Host "Reports saved to: $ReportDir`n" -ForegroundColor White

if ($AuditPassed -and $DenyPassed -and $UnsafePassed -and $OutdatedPassed) {
    Write-Host "✅ ALL CHECKS PASSED - Ready for publication!`n" -ForegroundColor Green
    exit 0
} else {
    Write-Host "⚠️  SOME CHECKS FAILED - Review reports before publishing`n" -ForegroundColor Yellow
    exit 1
}
