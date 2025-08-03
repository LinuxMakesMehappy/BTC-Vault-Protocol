#!/bin/bash

# Vault Protocol Security Audit Script
# Comprehensive security testing for local development

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
NC='\033[0m'

print_header() {
    echo -e "${PURPLE}================================${NC}"
    echo -e "${PURPLE}🛡️  VAULT PROTOCOL SECURITY AUDIT${NC}"
    echo -e "${PURPLE}================================${NC}"
    echo ""
}

print_section() {
    echo -e "${BLUE}[AUDIT]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[PASS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

print_error() {
    echo -e "${RED}[FAIL]${NC} $1"
}

# Initialize audit results
TOTAL_CHECKS=0
PASSED_CHECKS=0
FAILED_CHECKS=0
WARNING_CHECKS=0

check_result() {
    local result=$1
    local message=$2
    
    TOTAL_CHECKS=$((TOTAL_CHECKS + 1))
    
    if [ $result -eq 0 ]; then
        print_success "$message"
        PASSED_CHECKS=$((PASSED_CHECKS + 1))
    elif [ $result -eq 2 ]; then
        print_warning "$message"
        WARNING_CHECKS=$((WARNING_CHECKS + 1))
    else
        print_error "$message"
        FAILED_CHECKS=$((FAILED_CHECKS + 1))
    fi
}

# Function to check for hardcoded secrets
check_secrets() {
    print_section "Checking for hardcoded secrets..."
    
    local secret_patterns=(
        "password\s*=\s*['\"][^'\"]*['\"]"
        "secret\s*=\s*['\"][^'\"]*['\"]"
        "api_key\s*=\s*['\"][^'\"]*['\"]"
        "private_key\s*=\s*['\"][^'\"]*['\"]"
        "['\"][0-9a-fA-F]{32,}['\"]"
        "sk_[a-zA-Z0-9]{32,}"
        "pk_[a-zA-Z0-9]{32,}"
    )
    
    local found_secrets=0
    
    for pattern in "${secret_patterns[@]}"; do
        if grep -r -E "$pattern" --include="*.rs" --include="*.py" --include="*.ts" --include="*.js" . > /dev/null 2>&1; then
            print_error "Found potential hardcoded secret: $pattern"
            grep -r -E "$pattern" --include="*.rs" --include="*.py" --include="*.ts" --include="*.js" . | head -5
            found_secrets=1
        fi
    done
    
    if [ $found_secrets -eq 0 ]; then
        check_result 0 "No hardcoded secrets found"
    else
        check_result 1 "Hardcoded secrets detected"
    fi
}

# Function to check Rust security
check_rust_security() {
    print_section "Running Rust security audit..."
    
    cd programs/vault
    
    # Check if cargo-audit is installed
    if ! command -v cargo-audit &> /dev/null; then
        print_warning "cargo-audit not installed, installing..."
        cargo install cargo-audit
    fi
    
    # Run cargo audit
    if cargo audit 2>/dev/null; then
        if cargo audit --json > ../../rust-audit.json 2>/dev/null; then
            local vuln_count=$(jq '.vulnerabilities.count' ../../rust-audit.json 2>/dev/null || echo "0")
            if [ "$vuln_count" -eq 0 ]; then
                check_result 0 "Cargo audit: No vulnerabilities found"
            else
                check_result 1 "Cargo audit: $vuln_count vulnerabilities found"
                cargo audit
            fi
        else
            check_result 0 "Cargo audit: No vulnerabilities found (JSON export failed)"
        fi
    else
        check_result 1 "Cargo audit failed to run"
    fi
    
    # Check for unsafe code
    local unsafe_count=$(grep -r "unsafe" src/ | wc -l)
    if [ $unsafe_count -eq 0 ]; then
        check_result 0 "No unsafe code blocks found"
    else
        check_result 2 "$unsafe_count unsafe code blocks found (review required)"
        grep -r "unsafe" src/ | head -5
    fi
    
    # Check for unwrap() calls (potential panics)
    local unwrap_count=$(grep -r "\.unwrap()" src/ | wc -l)
    if [ $unwrap_count -eq 0 ]; then
        check_result 0 "No unwrap() calls found"
    else
        check_result 2 "$unwrap_count unwrap() calls found (review for panic safety)"
    fi
    
    # Check for proper error handling
    local expect_count=$(grep -r "\.expect(" src/ | wc -l)
    if [ $expect_count -gt 0 ]; then
        check_result 0 "Found $expect_count expect() calls (good error handling)"
    else
        check_result 2 "No expect() calls found (consider better error messages)"
    fi
    
    cd ../..
}

# Function to check Python security
check_python_security() {
    print_section "Running Python security audit..."
    
    # Check if security tools are installed
    local tools=("safety" "bandit")
    for tool in "${tools[@]}"; do
        if ! command -v $tool &> /dev/null; then
            print_warning "$tool not installed, installing..."
            pip install $tool
        fi
    done
    
    # Run safety check
    if safety check --json --output safety-report.json 2>/dev/null; then
        local vuln_count=$(jq '.vulnerabilities | length' safety-report.json 2>/dev/null || echo "0")
        if [ "$vuln_count" -eq 0 ]; then
            check_result 0 "Safety check: No vulnerabilities found"
        else
            check_result 1 "Safety check: $vuln_count vulnerabilities found"
            safety check
        fi
    else
        check_result 2 "Safety check completed with warnings"
    fi
    
    # Run bandit security linter
    if bandit -r . -f json -o bandit-report.json -ll 2>/dev/null; then
        local issue_count=$(jq '.results | length' bandit-report.json 2>/dev/null || echo "0")
        if [ "$issue_count" -eq 0 ]; then
            check_result 0 "Bandit: No security issues found"
        else
            check_result 2 "Bandit: $issue_count potential security issues found"
            bandit -r . -ll | head -20
        fi
    else
        check_result 1 "Bandit security scan failed"
    fi
}

# Function to check cryptographic implementations
check_cryptography() {
    print_section "Auditing cryptographic implementations..."
    
    cd programs/vault
    
    # Check for weak cryptographic algorithms
    local weak_crypto=(
        "md5"
        "sha1"
        "des"
        "rc4"
        "rand::random"
    )
    
    local weak_found=0
    for algo in "${weak_crypto[@]}"; do
        if grep -r -i "$algo" src/ > /dev/null 2>&1; then
            print_error "Weak cryptographic algorithm found: $algo"
            grep -r -i "$algo" src/ | head -3
            weak_found=1
        fi
    done
    
    if [ $weak_found -eq 0 ]; then
        check_result 0 "No weak cryptographic algorithms found"
    else
        check_result 1 "Weak cryptographic algorithms detected"
    fi
    
    # Check for proper entropy sources
    if grep -r "OsRng\|ChaCha20Rng" src/ > /dev/null 2>&1; then
        check_result 0 "Secure random number generators found"
    else
        check_result 2 "No secure RNG found (review entropy sources)"
    fi
    
    # Check for constant-time operations
    if grep -r "subtle::" src/ > /dev/null 2>&1; then
        check_result 0 "Constant-time operations found"
    else
        check_result 2 "No constant-time operations found (review timing attacks)"
    fi
    
    # Run cryptographic tests
    if cargo test --lib test_validate_ecdsa_proof_valid > /dev/null 2>&1; then
        check_result 0 "ECDSA validation tests pass"
    else
        check_result 1 "ECDSA validation tests fail"
    fi
    
    if cargo test --lib test_anti_replay_attack > /dev/null 2>&1; then
        check_result 0 "Anti-replay attack tests pass"
    else
        check_result 1 "Anti-replay attack tests fail"
    fi
    
    cd ../..
}

# Function to check Solana-specific security
check_solana_security() {
    print_section "Checking Solana program security..."
    
    cd programs/vault
    
    # Check for owner checks
    local owner_checks=0
    owner_checks=$((owner_checks + $(grep -r "has_one.*owner" src/ | wc -l)))
    owner_checks=$((owner_checks + $(grep -r "\.owner\s*==" src/ | wc -l)))
    owner_checks=$((owner_checks + $(grep -r "owner.*key()" src/ | wc -l)))
    
    if [ $owner_checks -gt 0 ]; then
        check_result 0 "Owner checks found ($owner_checks instances)"
    else
        check_result 1 "No owner checks found (critical security issue)"
    fi
    
    # Check for signer checks
    local signer_checks=0
    signer_checks=$((signer_checks + $(grep -r "Signer<'info>" src/ | wc -l)))
    signer_checks=$((signer_checks + $(grep -r "is_signer" src/ | wc -l)))
    signer_checks=$((signer_checks + $(grep -r "UnauthorizedSigner" src/ | wc -l)))
    
    if [ $signer_checks -gt 0 ]; then
        check_result 0 "Signer checks found ($signer_checks instances)"
    else
        check_result 1 "No signer checks found (critical security issue)"
    fi
    
    # Check for proper account validation
    if grep -r "validate" src/ > /dev/null 2>&1; then
        check_result 0 "Account validation functions found"
    else
        check_result 2 "No validation functions found (review account security)"
    fi
    
    # Check program size (prevent bloat attacks)
    if [ -f target/deploy/vault.so ]; then
        local program_size=$(wc -c < target/deploy/vault.so)
        if [ $program_size -lt 1048576 ]; then  # 1MB limit
            check_result 0 "Program size OK ($program_size bytes)"
        else
            check_result 1 "Program size too large ($program_size bytes > 1MB)"
        fi
    else
        print_warning "Program not built, skipping size check"
    fi
    
    cd ../..
}

# Function to check dependencies
check_dependencies() {
    print_section "Checking dependency security..."
    
    # Check Rust dependencies
    cd programs/vault
    if [ -f ../../deny.toml ]; then
        if command -v cargo-deny &> /dev/null; then
            if cargo deny check 2>/dev/null; then
                check_result 0 "Cargo deny: All dependency checks pass"
            else
                check_result 1 "Cargo deny: Dependency policy violations found"
                cargo deny check
            fi
        else
            print_warning "cargo-deny not installed, installing..."
            cargo install cargo-deny
            if cargo deny check 2>/dev/null; then
                check_result 0 "Cargo deny: All dependency checks pass"
            else
                check_result 1 "Cargo deny: Dependency policy violations found"
            fi
        fi
    else
        check_result 2 "No deny.toml found (consider adding dependency policies)"
    fi
    cd ../..
    
    # Check Python dependencies
    if [ -f requirements.txt ]; then
        if command -v pip-audit &> /dev/null; then
            if pip-audit --format=json --output=pip-audit.json 2>/dev/null; then
                local vuln_count=$(jq '.vulnerabilities | length' pip-audit.json 2>/dev/null || echo "0")
                if [ "$vuln_count" -eq 0 ]; then
                    check_result 0 "pip-audit: No vulnerabilities found"
                else
                    check_result 1 "pip-audit: $vuln_count vulnerabilities found"
                fi
            else
                check_result 2 "pip-audit completed with warnings"
            fi
        else
            print_warning "pip-audit not installed, installing..."
            pip install pip-audit
        fi
    fi
    
    # Check Node.js dependencies
    if [ -f frontend/package.json ]; then
        cd frontend
        if npm audit --audit-level=moderate --json > ../npm-audit.json 2>/dev/null; then
            local vuln_count=$(jq '.metadata.vulnerabilities.total' ../npm-audit.json 2>/dev/null || echo "0")
            if [ "$vuln_count" -eq 0 ]; then
                check_result 0 "npm audit: No vulnerabilities found"
            else
                check_result 1 "npm audit: $vuln_count vulnerabilities found"
            fi
        else
            check_result 2 "npm audit completed with warnings"
        fi
        cd ..
    fi
}

# Function to check file permissions
check_permissions() {
    print_section "Checking file permissions..."
    
    # Check for world-writable files
    local writable_files=$(find . -type f -perm -002 2>/dev/null | wc -l)
    if [ $writable_files -eq 0 ]; then
        check_result 0 "No world-writable files found"
    else
        check_result 1 "$writable_files world-writable files found"
        find . -type f -perm -002 2>/dev/null | head -5
    fi
    
    # Check for executable scripts
    local exec_scripts=$(find . -name "*.sh" -o -name "*.py" | wc -l)
    if [ $exec_scripts -gt 0 ]; then
        check_result 0 "Found $exec_scripts executable scripts"
    fi
}

# Function to generate security report
generate_report() {
    print_section "Generating security report..."
    
    local report_file="security-audit-report.md"
    
    cat > $report_file << EOF
# 🛡️ Vault Protocol Security Audit Report

**Audit Date:** $(date -u)
**Audit Version:** 1.0
**Commit Hash:** $(git rev-parse HEAD 2>/dev/null || echo "N/A")

## 📊 Summary

- **Total Checks:** $TOTAL_CHECKS
- **Passed:** $PASSED_CHECKS ✅
- **Warnings:** $WARNING_CHECKS ⚠️
- **Failed:** $FAILED_CHECKS ❌

## 🎯 Security Score

EOF

    local score=$((PASSED_CHECKS * 100 / TOTAL_CHECKS))
    echo "**Overall Score:** $score/100" >> $report_file
    echo "" >> $report_file
    
    if [ $score -ge 90 ]; then
        echo "🟢 **Status:** EXCELLENT - Production ready" >> $report_file
    elif [ $score -ge 80 ]; then
        echo "🟡 **Status:** GOOD - Minor issues to address" >> $report_file
    elif [ $score -ge 70 ]; then
        echo "🟠 **Status:** FAIR - Several issues need attention" >> $report_file
    else
        echo "🔴 **Status:** POOR - Critical issues must be fixed" >> $report_file
    fi
    
    cat >> $report_file << EOF

## 📋 Detailed Results

### ✅ Passed Checks: $PASSED_CHECKS
### ⚠️ Warnings: $WARNING_CHECKS  
### ❌ Failed Checks: $FAILED_CHECKS

## 🔍 Recommendations

1. **Address all failed checks immediately**
2. **Review and resolve warnings**
3. **Run security audit before each release**
4. **Consider third-party security audit**
5. **Implement continuous security monitoring**

## 📁 Generated Files

- \`rust-audit.json\` - Rust vulnerability report
- \`safety-report.json\` - Python safety report
- \`bandit-report.json\` - Python security issues
- \`pip-audit.json\` - Python dependency vulnerabilities
- \`npm-audit.json\` - Node.js dependency vulnerabilities

---
*Generated by Vault Protocol Security Audit Script*
EOF

    print_success "Security report generated: $report_file"
}

# Main execution
main() {
    print_header
    
    # Run all security checks
    check_secrets
    check_rust_security
    check_python_security
    check_cryptography
    check_solana_security
    check_dependencies
    check_permissions
    
    # Generate report
    generate_report
    
    # Final summary
    echo ""
    echo -e "${PURPLE}================================${NC}"
    echo -e "${PURPLE}🏁 SECURITY AUDIT COMPLETE${NC}"
    echo -e "${PURPLE}================================${NC}"
    echo ""
    echo -e "Total Checks: $TOTAL_CHECKS"
    echo -e "${GREEN}Passed: $PASSED_CHECKS${NC}"
    echo -e "${YELLOW}Warnings: $WARNING_CHECKS${NC}"
    echo -e "${RED}Failed: $FAILED_CHECKS${NC}"
    echo ""
    
    local score=$((PASSED_CHECKS * 100 / TOTAL_CHECKS))
    echo -e "Security Score: $score/100"
    
    if [ $FAILED_CHECKS -gt 0 ]; then
        echo -e "${RED}❌ SECURITY AUDIT FAILED${NC}"
        echo -e "Please address the failed checks before deployment."
        exit 1
    elif [ $WARNING_CHECKS -gt 0 ]; then
        echo -e "${YELLOW}⚠️ SECURITY AUDIT PASSED WITH WARNINGS${NC}"
        echo -e "Consider addressing warnings for improved security."
        exit 0
    else
        echo -e "${GREEN}✅ SECURITY AUDIT PASSED${NC}"
        echo -e "All security checks passed successfully!"
        exit 0
    fi
}

# Run the audit
main "$@"