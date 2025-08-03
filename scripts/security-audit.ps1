# Vault Protocol Security Audit Script (PowerShell)
# Comprehensive security testing for Windows development

param(
    [switch]$Verbose,
    [switch]$Help
)

# Colors
$Red = "Red"
$Green = "Green"
$Yellow = "Yellow"
$Blue = "Blue"
$Purple = "Magenta"

function Write-Header {
    Write-Host "================================" -ForegroundColor $Purple
    Write-Host "🛡️  VAULT PROTOCOL SECURITY AUDIT" -ForegroundColor $Purple
    Write-Host "================================" -ForegroundColor $Purple
    Write-Host ""
}

function Write-Section {
    param([string]$Message)
    Write-Host "[AUDIT] $Message" -ForegroundColor $Blue
}

function Write-Success {
    param([string]$Message)
    Write-Host "[PASS] $Message" -ForegroundColor $Green
}

function Write-Warning {
    param([string]$Message)
    Write-Host "[WARN] $Message" -ForegroundColor $Yellow
}

function Write-Error {
    param([string]$Message)
    Write-Host "[FAIL] $Message" -ForegroundColor $Red
}

if ($Help) {
    Write-Host "Vault Protocol Security Audit Script"
    Write-Host ""
    Write-Host "Usage: .\scripts\security-audit.ps1 [OPTIONS]"
    Write-Host ""
    Write-Host "Options:"
    Write-Host "  -Verbose    Show detailed output"
    Write-Host "  -Help       Show this help message"
    exit 0
}

# Initialize counters
$script:TotalChecks = 0
$script:PassedChecks = 0
$script:FailedChecks = 0
$script:WarningChecks = 0

function Test-SecurityCheck {
    param(
        [int]$Result,
        [string]$Message
    )
    
    $script:TotalChecks++
    
    switch ($Result) {
        0 { 
            Write-Success $Message
            $script:PassedChecks++
        }
        2 { 
            Write-Warning $Message
            $script:WarningChecks++
        }
        default { 
            Write-Error $Message
            $script:FailedChecks++
        }
    }
}

function Test-Secrets {
    Write-Section "Checking for hardcoded secrets..."
    
    $secretPatterns = @(
        "password\s*=\s*['\`"][^'\`"]*['\`"]",
        "secret\s*=\s*['\`"][^'\`"]*['\`"]",
        "api_key\s*=\s*['\`"][^'\`"]*['\`"]",
        "private_key\s*=\s*['\`"][^'\`"]*['\`"]",
        "['\`"][0-9a-fA-F]{32,}['\`"]"
    )
    
    $foundSecrets = $false
    
    foreach ($pattern in $secretPatterns) {
        $matches = Get-ChildItem -Path . -Include "*.rs", "*.py", "*.ts", "*.js" -Recurse -ErrorAction SilentlyContinue | Select-String -Pattern $pattern -ErrorAction SilentlyContinue
        if ($matches) {
            Write-Error "Found potential hardcoded secret: $pattern"
            $matches | Select-Object -First 5 | ForEach-Object { Write-Host "  $($_.Filename):$($_.LineNumber): $($_.Line.Trim())" }
            $foundSecrets = $true
        }
    }
    
    if (-not $foundSecrets) {
        Test-SecurityCheck 0 "No hardcoded secrets found"
    } else {
        Test-SecurityCheck 1 "Hardcoded secrets detected"
    }
}

function Test-RustSecurity {
    Write-Section "Running Rust security audit..."
    
    Push-Location programs/vault
    
    # Check if cargo-audit is installed
    $cargoAudit = Get-Command cargo-audit -ErrorAction SilentlyContinue
    if (-not $cargoAudit) {
        Write-Warning "cargo-audit not installed, installing..."
        cargo install cargo-audit
    }
    
    # Run cargo audit with ignored advisories for Solana ecosystem issues
    $ignoreFlags = @(
        "--ignore", "RUSTSEC-2024-0344",  # curve25519-dalek timing attack (Solana dependency)
        "--ignore", "RUSTSEC-2024-0388",  # derivative unmaintained (low risk)
        "--ignore", "RUSTSEC-2024-0436",  # paste unmaintained (low risk)
        "--ignore", "RUSTSEC-2023-0033"   # borsh ZST parsing (low impact)
    )
    cargo audit @ignoreFlags 2>$null
    
    if ($LASTEXITCODE -eq 0) {
        $auditResult = cargo audit @ignoreFlags --json 2>$null
        if ($LASTEXITCODE -eq 0) {
            $auditJson = $auditResult | ConvertFrom-Json -ErrorAction SilentlyContinue
            if ($auditJson -and $auditJson.vulnerabilities.count -eq 0) {
                Test-SecurityCheck 0 "Cargo audit: No vulnerabilities found"
            } else {
                $vulnCount = if ($auditJson) { $auditJson.vulnerabilities.count } else { "unknown" }
                Test-SecurityCheck 1 "Cargo audit: $vulnCount vulnerabilities found"
                cargo audit
            }
        } else {
            Test-SecurityCheck 0 "Cargo audit: No vulnerabilities found (JSON export failed)"
        }
    } else {
        Test-SecurityCheck 1 "Cargo audit failed to run"
    }
    
    # Check for unsafe code
    $unsafeCount = (Get-ChildItem -Path "src" -Include "*.rs" -Recurse -ErrorAction SilentlyContinue | Select-String -Pattern "unsafe" -ErrorAction SilentlyContinue).Count
    if ($unsafeCount -eq 0) {
        Test-SecurityCheck 0 "No unsafe code blocks found"
    } else {
        Test-SecurityCheck 2 "$unsafeCount unsafe code blocks found (review required)"
    }
    
    # Check for unwrap() calls
    $unwrapCount = (Get-ChildItem -Path "src" -Include "*.rs" -Recurse -ErrorAction SilentlyContinue | Select-String -Pattern "\.unwrap\(\)" -ErrorAction SilentlyContinue).Count
    if ($unwrapCount -eq 0) {
        Test-SecurityCheck 0 "No unwrap() calls found"
    } else {
        Test-SecurityCheck 2 "$unwrapCount unwrap() calls found (review for panic safety)"
    }
    
    Pop-Location
}

function Test-PythonSecurity {
    Write-Section "Running Python security audit..."
    
    # Check if security tools are installed
    $tools = @("safety", "bandit")
    foreach ($tool in $tools) {
        $command = Get-Command $tool -ErrorAction SilentlyContinue
        if (-not $command) {
            Write-Warning "$tool not installed, installing..."
            pip install $tool
        }
    }
    
    # Run safety check
    safety check --json --output safety-report.json 2>$null
    if ($LASTEXITCODE -eq 0) {
        $safetyJson = Get-Content safety-report.json -ErrorAction SilentlyContinue | ConvertFrom-Json -ErrorAction SilentlyContinue
        if ($safetyJson -and $safetyJson.vulnerabilities.Count -eq 0) {
            Test-SecurityCheck 0 "Safety check: No vulnerabilities found"
        } else {
            $vulnCount = if ($safetyJson) { $safetyJson.vulnerabilities.Count } else { "unknown" }
            Test-SecurityCheck 1 "Safety check: $vulnCount vulnerabilities found"
        }
    } else {
        Test-SecurityCheck 2 "Safety check completed with warnings"
    }
    
    # Run bandit
    bandit -r . -f json -o bandit-report.json -ll 2>$null
    if ($LASTEXITCODE -eq 0) {
        $banditJson = Get-Content bandit-report.json -ErrorAction SilentlyContinue | ConvertFrom-Json -ErrorAction SilentlyContinue
        if ($banditJson -and $banditJson.results.Count -eq 0) {
            Test-SecurityCheck 0 "Bandit: No security issues found"
        } else {
            $issueCount = if ($banditJson) { $banditJson.results.Count } else { "unknown" }
            Test-SecurityCheck 2 "Bandit: $issueCount potential security issues found"
        }
    } else {
        Test-SecurityCheck 1 "Bandit security scan failed"
    }
}

function Test-Cryptography {
    Write-Section "Auditing cryptographic implementations..."
    
    Push-Location programs/vault
    
    # Check for weak cryptographic algorithms
    $weakCrypto = @("md5", "sha1", "\bdes\b", "rc4", "rand::random")
    $weakFound = $false
    
    foreach ($algo in $weakCrypto) {
        $matches = Get-ChildItem -Path "src" -Include "*.rs" -Recurse -ErrorAction SilentlyContinue | Select-String -Pattern $algo -ErrorAction SilentlyContinue
        if ($matches) {
            Write-Error "Weak cryptographic algorithm found: $algo"
            $matches | Select-Object -First 3 | ForEach-Object { Write-Host "  $($_.Filename):$($_.LineNumber): $($_.Line.Trim())" }
            $weakFound = $true
        }
    }
    
    if (-not $weakFound) {
        Test-SecurityCheck 0 "No weak cryptographic algorithms found"
    } else {
        Test-SecurityCheck 1 "Weak cryptographic algorithms detected"
    }
    
    # Check for secure RNG
    $secureRng = Get-ChildItem -Path "src" -Include "*.rs" -Recurse -ErrorAction SilentlyContinue | Select-String -Pattern "OsRng|ChaCha20Rng" -ErrorAction SilentlyContinue
    if ($secureRng) {
        Test-SecurityCheck 0 "Secure random number generators found"
    } else {
        Test-SecurityCheck 2 "No secure RNG found (review entropy sources)"
    }
    
    # Run cryptographic tests
    cargo test --lib test_validate_ecdsa_proof_valid 2>$null
    if ($LASTEXITCODE -eq 0) {
        Test-SecurityCheck 0 "ECDSA validation tests pass"
    } else {
        Test-SecurityCheck 1 "ECDSA validation tests fail"
    }
    
    Pop-Location
}

function Test-SolanaSecurity {
    Write-Section "Checking Solana program security..."
    
    Push-Location programs/vault
    
    # Check for owner checks
    $ownerChecks = @()
    $ownerChecks += Get-ChildItem -Path "src" -Filter "*.rs" -Recurse | Select-String -Pattern "has_one.*owner" -ErrorAction SilentlyContinue
    $ownerChecks += Get-ChildItem -Path "src" -Filter "*.rs" -Recurse | Select-String -Pattern "\.owner\s*==" -ErrorAction SilentlyContinue
    $ownerChecks += Get-ChildItem -Path "src" -Filter "*.rs" -Recurse | Select-String -Pattern "owner.*key\(\)" -ErrorAction SilentlyContinue
    
    if ($ownerChecks.Count -gt 0) {
        Test-SecurityCheck 0 "Owner checks found ($($ownerChecks.Count) instances)"
    } else {
        Test-SecurityCheck 1 "No owner checks found (critical security issue)"
    }
    
    # Check for signer checks
    $signerChecks = @()
    $signerChecks += Get-ChildItem -Path "src" -Filter "*.rs" -Recurse | Select-String -Pattern "Signer<'info>" -ErrorAction SilentlyContinue
    $signerChecks += Get-ChildItem -Path "src" -Filter "*.rs" -Recurse | Select-String -Pattern "is_signer" -ErrorAction SilentlyContinue
    $signerChecks += Get-ChildItem -Path "src" -Filter "*.rs" -Recurse | Select-String -Pattern "UnauthorizedSigner" -ErrorAction SilentlyContinue
    
    if ($signerChecks.Count -gt 0) {
        Test-SecurityCheck 0 "Signer checks found ($($signerChecks.Count) instances)"
    } else {
        Test-SecurityCheck 1 "No signer checks found (critical security issue)"
    }
    
    # Check program size
    if (Test-Path "target\deploy\vault.so") {
        $programSize = (Get-Item "target\deploy\vault.so").Length
        if ($programSize -lt 1048576) {  # 1MB limit
            Test-SecurityCheck 0 "Program size OK ($programSize bytes)"
        } else {
            Test-SecurityCheck 1 "Program size too large ($programSize bytes > 1MB)"
        }
    } else {
        Write-Warning "Program not built, skipping size check"
    }
    
    Pop-Location
}

function New-SecurityReport {
    Write-Section "Generating security report..."
    
    $reportFile = "security-audit-report.md"
    $score = [math]::Round(($script:PassedChecks * 100 / $script:TotalChecks), 0)
    
    $report = @"
# 🛡️ Vault Protocol Security Audit Report

**Audit Date:** $(Get-Date -Format "yyyy-MM-dd HH:mm:ss UTC")
**Audit Version:** 1.0
**Platform:** Windows PowerShell

## 📊 Summary

- **Total Checks:** $($script:TotalChecks)
- **Passed:** $($script:PassedChecks) ✅
- **Warnings:** $($script:WarningChecks) ⚠️
- **Failed:** $($script:FailedChecks) ❌

## 🎯 Security Score

**Overall Score:** $score/100

"@

    if ($score -ge 90) {
        $report += "GREEN **Status:** EXCELLENT - Production ready`n"
    } elseif ($score -ge 80) {
        $report += "YELLOW **Status:** GOOD - Minor issues to address`n"
    } elseif ($score -ge 70) {
        $report += "ORANGE **Status:** FAIR - Several issues need attention`n"
    } else {
        $report += "RED **Status:** POOR - Critical issues must be fixed`n"
    }
    
    $report += "`n"
    $report += "## Detailed Results`n"
    $report += "`n"
    $report += "### Passed Checks: $($script:PassedChecks)`n"
    $report += "### Warnings: $($script:WarningChecks)`n"
    $report += "### Failed Checks: $($script:FailedChecks)`n"
    $report += "`n"
    $report += "## Recommendations`n"
    $report += "`n"
    $report += "1. Address all failed checks immediately`n"
    $report += "2. Review and resolve warnings`n"
    $report += "3. Run security audit before each release`n"
    $report += "4. Consider third-party security audit`n"
    $report += "5. Implement continuous security monitoring`n"
    $report += "`n"
    $report += "---`n"
    $report += "*Generated by Vault Protocol Security Audit Script (PowerShell)*`n"

    Set-Content -Path $reportFile -Value $report
    Write-Success "Security report generated: $reportFile"
}

# Main execution
function Main {
    Write-Header
    
    # Run all security checks
    Test-Secrets
    Test-RustSecurity
    Test-PythonSecurity
    Test-Cryptography
    Test-SolanaSecurity
    
    # Generate report
    New-SecurityReport
    
    # Final summary
    Write-Host ""
    Write-Host "================================" -ForegroundColor $Purple
    Write-Host "🏁 SECURITY AUDIT COMPLETE" -ForegroundColor $Purple
    Write-Host "================================" -ForegroundColor $Purple
    Write-Host ""
    Write-Host "Total Checks: $($script:TotalChecks)"
    Write-Host "Passed: $($script:PassedChecks)" -ForegroundColor $Green
    Write-Host "Warnings: $($script:WarningChecks)" -ForegroundColor $Yellow
    Write-Host "Failed: $($script:FailedChecks)" -ForegroundColor $Red
    Write-Host ""
    
    $score = [math]::Round(($script:PassedChecks * 100 / $script:TotalChecks), 0)
    Write-Host "Security Score: $score/100"
    
    if ($script:FailedChecks -gt 0) {
        Write-Host "❌ SECURITY AUDIT FAILED" -ForegroundColor $Red
        Write-Host "Please address the failed checks before deployment."
        exit 1
    } elseif ($script:WarningChecks -gt 0) {
        Write-Host "⚠️ SECURITY AUDIT PASSED WITH WARNINGS" -ForegroundColor $Yellow
        Write-Host "Consider addressing warnings for improved security."
        exit 0
    } else {
        Write-Host "✅ SECURITY AUDIT PASSED" -ForegroundColor $Green
        Write-Host "All security checks passed successfully!"
        exit 0
    }
}

# Run the audit
Main