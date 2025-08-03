#!/bin/bash

# Vault Protocol Test Runner
# Comprehensive testing script for local development

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Default values
RUN_RUST=true
RUN_PYTHON=true
RUN_FRONTEND=true
RUN_INTEGRATION=false
VERBOSE=false

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --rust-only)
            RUN_RUST=true
            RUN_PYTHON=false
            RUN_FRONTEND=false
            shift
            ;;
        --python-only)
            RUN_RUST=false
            RUN_PYTHON=true
            RUN_FRONTEND=false
            shift
            ;;
        --frontend-only)
            RUN_RUST=false
            RUN_PYTHON=false
            RUN_FRONTEND=true
            shift
            ;;
        --integration)
            RUN_INTEGRATION=true
            shift
            ;;
        --verbose|-v)
            VERBOSE=true
            shift
            ;;
        --help|-h)
            echo "Usage: $0 [OPTIONS]"
            echo "Options:"
            echo "  --rust-only      Run only Rust tests"
            echo "  --python-only    Run only Python tests"
            echo "  --frontend-only  Run only frontend tests"
            echo "  --integration    Run integration tests"
            echo "  --verbose, -v    Verbose output"
            echo "  --help, -h       Show this help message"
            exit 0
            ;;
        *)
            print_error "Unknown option: $1"
            exit 1
            ;;
    esac
done

# Set verbose flag for commands
if [ "$VERBOSE" = true ]; then
    CARGO_FLAGS="--verbose"
    PYTEST_FLAGS="-v -s"
    NPM_FLAGS="--verbose"
else
    CARGO_FLAGS=""
    PYTEST_FLAGS="-v"
    NPM_FLAGS=""
fi

print_status "Starting Vault Protocol Test Suite..."

# Check prerequisites
print_status "Checking prerequisites..."

if ! command -v cargo &> /dev/null; then
    print_error "Cargo not found. Please install Rust."
    exit 1
fi

if ! command -v python3 &> /dev/null; then
    print_error "Python3 not found. Please install Python."
    exit 1
fi

if ! command -v node &> /dev/null; then
    print_error "Node.js not found. Please install Node.js."
    exit 1
fi

# Rust Tests
if [ "$RUN_RUST" = true ]; then
    print_status "Running Rust tests..."
    
    # Format check
    print_status "Checking Rust code formatting..."
    if cargo fmt --all --manifest-path programs/vault/Cargo.toml -- --check; then
        print_success "Rust formatting check passed"
    else
        print_warning "Rust formatting issues found. Run 'cargo fmt' to fix."
    fi
    
    # Clippy linting
    print_status "Running Clippy lints..."
    if cargo clippy --all-targets --all-features --manifest-path programs/vault/Cargo.toml -- -D warnings; then
        print_success "Clippy lints passed"
    else
        print_error "Clippy lints failed"
        exit 1
    fi
    
    # Unit tests
    print_status "Running Rust unit tests..."
    if cargo test --manifest-path programs/vault/Cargo.toml --lib $CARGO_FLAGS; then
        print_success "Rust unit tests passed"
    else
        print_error "Rust unit tests failed"
        exit 1
    fi
    
    # Build check
    print_status "Building Solana program..."
    cd programs/vault
    if anchor build; then
        print_success "Solana program build successful"
    else
        print_error "Solana program build failed"
        exit 1
    fi
    cd ../..
fi

# Python Tests
if [ "$RUN_PYTHON" = true ]; then
    print_status "Running Python tests..."
    
    # Install test dependencies if needed
    if [ ! -f ".venv/bin/activate" ]; then
        print_status "Creating Python virtual environment..."
        python3 -m venv .venv
        source .venv/bin/activate
        pip install pytest pytest-asyncio pytest-mock pytest-cov
    else
        source .venv/bin/activate
    fi
    
    # Run Python tests
    print_status "Running Python test suite..."
    if python -m pytest tests/ $PYTEST_FLAGS --tb=short; then
        print_success "Python tests passed"
    else
        print_error "Python tests failed"
        exit 1
    fi
    
    # Generate coverage report
    print_status "Generating test coverage report..."
    python -m pytest tests/ --cov=. --cov-report=html --cov-report=term-missing
    print_success "Coverage report generated in htmlcov/"
fi

# Frontend Tests
if [ "$RUN_FRONTEND" = true ]; then
    print_status "Running frontend tests..."
    
    cd frontend
    
    # Install dependencies
    if [ ! -d "node_modules" ]; then
        print_status "Installing frontend dependencies..."
        npm install $NPM_FLAGS
    fi
    
    # TypeScript check
    print_status "Running TypeScript checks..."
    if npm run type-check; then
        print_success "TypeScript checks passed"
    else
        print_error "TypeScript checks failed"
        exit 1
    fi
    
    # Linting
    print_status "Running ESLint..."
    if npm run lint; then
        print_success "Linting passed"
    else
        print_warning "Linting issues found"
    fi
    
    # Tests
    print_status "Running frontend tests..."
    if npm run test; then
        print_success "Frontend tests passed"
    else
        print_error "Frontend tests failed"
        exit 1
    fi
    
    # Build check
    print_status "Testing frontend build..."
    if npm run build; then
        print_success "Frontend build successful"
    else
        print_error "Frontend build failed"
        exit 1
    fi
    
    cd ..
fi

# Integration Tests
if [ "$RUN_INTEGRATION" = true ]; then
    print_status "Running integration tests..."
    
    # Start local validator
    print_status "Starting Solana local validator..."
    solana-test-validator --reset --quiet &
    VALIDATOR_PID=$!
    
    # Wait for validator to start
    sleep 5
    
    # Run integration tests
    cd programs/vault
    if anchor test --skip-local-validator; then
        print_success "Integration tests passed"
    else
        print_error "Integration tests failed"
        kill $VALIDATOR_PID
        exit 1
    fi
    cd ../..
    
    # Stop validator
    kill $VALIDATOR_PID
fi

print_success "All tests completed successfully! 🎉"

# Summary
echo ""
echo "=== Test Summary ==="
if [ "$RUN_RUST" = true ]; then
    echo "✅ Rust tests: PASSED"
fi
if [ "$RUN_PYTHON" = true ]; then
    echo "✅ Python tests: PASSED"
fi
if [ "$RUN_FRONTEND" = true ]; then
    echo "✅ Frontend tests: PASSED"
fi
if [ "$RUN_INTEGRATION" = true ]; then
    echo "✅ Integration tests: PASSED"
fi
echo "===================="