# Vault Protocol Test Runner (PowerShell)
# Comprehensive testing script for local development on Windows

param(
    [switch]$RustOnly,
    [switch]$PythonOnly,
    [switch]$FrontendOnly,
    [switch]$Integration,
    [switch]$Verbose,
    [switch]$Help
)

# Colors for output
$Red = "Red"
$Green = "Green"
$Yellow = "Yellow"
$Blue = "Blue"

function Write-Status {
    param([string]$Message)
    Write-Host "[INFO] $Message" -ForegroundColor $Blue
}

function Write-Success {
    param([string]$Message)
    Write-Host "[SUCCESS] $Message" -ForegroundColor $Green
}

function Write-Warning {
    param([string]$Message)
    Write-Host "[WARNING] $Message" -ForegroundColor $Yellow
}

function Write-Error {
    param([string]$Message)
    Write-Host "[ERROR] $Message" -ForegroundColor $Red
}

if ($Help) {
    Write-Host "Usage: .\scripts\test.ps1 [OPTIONS]"
    Write-Host "Options:"
    Write-Host "  -RustOnly      Run only Rust tests"
    Write-Host "  -PythonOnly    Run only Python tests"
    Write-Host "  -FrontendOnly  Run only frontend tests"
    Write-Host "  -Integration   Run integration tests"
    Write-Host "  -Verbose       Verbose output"
    Write-Host "  -Help          Show this help message"
    exit 0
}

# Set default values
$RunRust = -not ($PythonOnly -or $FrontendOnly)
$RunPython = -not ($RustOnly -or $FrontendOnly)
$RunFrontend = -not ($RustOnly -or $PythonOnly)

if ($RustOnly) { $RunRust = $true; $RunPython = $false; $RunFrontend = $false }
if ($PythonOnly) { $RunRust = $false; $RunPython = $true; $RunFrontend = $false }
if ($FrontendOnly) { $RunRust = $false; $RunPython = $false; $RunFrontend = $true }

Write-Status "Starting Vault Protocol Test Suite..."

# Check prerequisites
Write-Status "Checking prerequisites..."

if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Error "Cargo not found. Please install Rust."
    exit 1
}

if (-not (Get-Command python -ErrorAction SilentlyContinue)) {
    Write-Error "Python not found. Please install Python."
    exit 1
}

if (-not (Get-Command node -ErrorAction SilentlyContinue)) {
    Write-Error "Node.js not found. Please install Node.js."
    exit 1
}

# Rust Tests
if ($RunRust) {
    Write-Status "Running Rust tests..."
    
    # Format check
    Write-Status "Checking Rust code formatting..."
    $formatResult = cargo fmt --all --manifest-path programs/vault/Cargo.toml -- --check
    if ($LASTEXITCODE -eq 0) {
        Write-Success "Rust formatting check passed"
    } else {
        Write-Warning "Rust formatting issues found. Run 'cargo fmt' to fix."
    }
    
    # Clippy linting
    Write-Status "Running Clippy lints..."
    cargo clippy --all-targets --all-features --manifest-path programs/vault/Cargo.toml -- -D warnings
    if ($LASTEXITCODE -ne 0) {
        Write-Error "Clippy lints failed"
        exit 1
    }
    Write-Success "Clippy lints passed"
    
    # Unit tests
    Write-Status "Running Rust unit tests..."
    $testArgs = @("test", "--manifest-path", "programs/vault/Cargo.toml", "--lib")
    if ($Verbose) { $testArgs += "--verbose" }
    
    & cargo @testArgs
    if ($LASTEXITCODE -ne 0) {
        Write-Error "Rust unit tests failed"
        exit 1
    }
    Write-Success "Rust unit tests passed"
    
    # Build check
    Write-Status "Building Solana program..."
    Push-Location programs/vault
    anchor build
    if ($LASTEXITCODE -ne 0) {
        Write-Error "Solana program build failed"
        Pop-Location
        exit 1
    }
    Write-Success "Solana program build successful"
    Pop-Location
}

# Python Tests
if ($RunPython) {
    Write-Status "Running Python tests..."
    
    # Run Python tests
    Write-Status "Running Python test suite..."
    $pytestArgs = @("-m", "pytest", "tests/")
    if ($Verbose) { $pytestArgs += @("-v", "-s") } else { $pytestArgs += "-v" }
    $pytestArgs += "--tb=short"
    
    & python @pytestArgs
    if ($LASTEXITCODE -ne 0) {
        Write-Error "Python tests failed"
        exit 1
    }
    Write-Success "Python tests passed"
    
    # Generate coverage report
    Write-Status "Generating test coverage report..."
    python -m pytest tests/ --cov=. --cov-report=html --cov-report=term-missing
    Write-Success "Coverage report generated in htmlcov/"
}

# Frontend Tests
if ($RunFrontend) {
    Write-Status "Running frontend tests..."
    
    Push-Location frontend
    
    # Install dependencies
    if (-not (Test-Path "node_modules")) {
        Write-Status "Installing frontend dependencies..."
        npm install
    }
    
    # TypeScript check
    Write-Status "Running TypeScript checks..."
    npm run type-check
    if ($LASTEXITCODE -ne 0) {
        Write-Error "TypeScript checks failed"
        Pop-Location
        exit 1
    }
    Write-Success "TypeScript checks passed"
    
    # Linting
    Write-Status "Running ESLint..."
    npm run lint
    if ($LASTEXITCODE -ne 0) {
        Write-Warning "Linting issues found"
    } else {
        Write-Success "Linting passed"
    }
    
    # Tests
    Write-Status "Running frontend tests..."
    npm run test
    if ($LASTEXITCODE -ne 0) {
        Write-Error "Frontend tests failed"
        Pop-Location
        exit 1
    }
    Write-Success "Frontend tests passed"
    
    # Build check
    Write-Status "Testing frontend build..."
    npm run build
    if ($LASTEXITCODE -ne 0) {
        Write-Error "Frontend build failed"
        Pop-Location
        exit 1
    }
    Write-Success "Frontend build successful"
    
    Pop-Location
}

# Integration Tests
if ($Integration) {
    Write-Status "Running integration tests..."
    
    # Start local validator
    Write-Status "Starting Solana local validator..."
    $validator = Start-Process -FilePath "solana-test-validator" -ArgumentList "--reset", "--quiet" -PassThru
    
    # Wait for validator to start
    Start-Sleep -Seconds 5
    
    # Run integration tests
    Push-Location programs/vault
    anchor test --skip-local-validator
    $integrationResult = $LASTEXITCODE
    Pop-Location
    
    # Stop validator
    Stop-Process -Id $validator.Id -Force
    
    if ($integrationResult -ne 0) {
        Write-Error "Integration tests failed"
        exit 1
    }
    Write-Success "Integration tests passed"
}

Write-Success "All tests completed successfully! 🎉"

# Summary
Write-Host ""
Write-Host "=== Test Summary ==="
if ($RunRust) { Write-Host "✅ Rust tests: PASSED" }
if ($RunPython) { Write-Host "✅ Python tests: PASSED" }
if ($RunFrontend) { Write-Host "✅ Frontend tests: PASSED" }
if ($Integration) { Write-Host "✅ Integration tests: PASSED" }
Write-Host "===================="