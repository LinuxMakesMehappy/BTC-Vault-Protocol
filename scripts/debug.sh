#!/bin/bash

# Vault Protocol Debug Helper
# Utilities for debugging and development

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

print_status() {
    echo -e "${BLUE}[DEBUG]${NC} $1"
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

# Function to start local Solana validator with debugging
start_validator() {
    print_status "Starting Solana validator with debug logging..."
    
    # Kill any existing validator
    pkill -f solana-test-validator || true
    
    # Start validator with debug settings
    solana-test-validator \
        --reset \
        --log \
        --rpc-port 8899 \
        --ws-port 8900 \
        --ledger .anchor/test-ledger \
        --bpf-program Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS programs/vault/target/deploy/vault.so &
    
    VALIDATOR_PID=$!
    echo $VALIDATOR_PID > .validator.pid
    
    print_success "Validator started with PID: $VALIDATOR_PID"
    print_status "RPC endpoint: http://localhost:8899"
    print_status "WebSocket endpoint: ws://localhost:8900"
    
    # Wait for validator to be ready
    print_status "Waiting for validator to be ready..."
    sleep 5
    
    # Check if validator is running
    if solana cluster-version --url localhost > /dev/null 2>&1; then
        print_success "Validator is ready!"
    else
        print_error "Validator failed to start properly"
        exit 1
    fi
}

# Function to stop validator
stop_validator() {
    if [ -f .validator.pid ]; then
        VALIDATOR_PID=$(cat .validator.pid)
        print_status "Stopping validator (PID: $VALIDATOR_PID)..."
        kill $VALIDATOR_PID || true
        rm .validator.pid
        print_success "Validator stopped"
    else
        print_warning "No validator PID file found"
        pkill -f solana-test-validator || true
    fi
}

# Function to deploy program with debugging
deploy_program() {
    print_status "Building and deploying program..."
    
    cd programs/vault
    
    # Build with debug info
    anchor build
    
    # Deploy to local validator
    anchor deploy --provider.cluster localnet
    
    print_success "Program deployed successfully"
    
    # Show program info
    solana program show Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS --url localhost
    
    cd ../..
}

# Function to run specific test with debugging
debug_test() {
    local test_name=$1
    
    if [ -z "$test_name" ]; then
        print_error "Please specify a test name"
        echo "Usage: $0 debug-test <test_name>"
        exit 1
    fi
    
    print_status "Running test with debugging: $test_name"
    
    # Set debug environment variables
    export RUST_LOG=debug
    export RUST_BACKTRACE=1
    
    # Run specific Rust test
    if [[ $test_name == *"rust"* ]]; then
        cargo test --manifest-path programs/vault/Cargo.toml $test_name -- --nocapture
    # Run specific Python test
    elif [[ $test_name == *"python"* ]]; then
        python -m pytest tests/ -k $test_name -v -s --tb=long
    else
        # Try both
        print_status "Trying Rust test first..."
        cargo test --manifest-path programs/vault/Cargo.toml $test_name -- --nocapture || {
            print_status "Rust test not found, trying Python test..."
            python -m pytest tests/ -k $test_name -v -s --tb=long
        }
    fi
}

# Function to show program logs
show_logs() {
    print_status "Showing recent program logs..."
    
    if [ -f .anchor/test-ledger/validator.log ]; then
        tail -f .anchor/test-ledger/validator.log | grep -E "(Program|Error|Panic)" --color=always
    else
        print_warning "No validator logs found. Make sure validator is running."
    fi
}

# Function to inspect account data
inspect_account() {
    local account_address=$1
    
    if [ -z "$account_address" ]; then
        print_error "Please specify an account address"
        echo "Usage: $0 inspect-account <account_address>"
        exit 1
    fi
    
    print_status "Inspecting account: $account_address"
    
    # Show account info
    solana account $account_address --url localhost --output json-compact | jq '.'
}

# Function to generate test keypairs
generate_keypairs() {
    local count=${1:-5}
    
    print_status "Generating $count test keypairs..."
    
    mkdir -p .debug/keypairs
    
    for i in $(seq 1 $count); do
        solana-keygen new --no-bip39-passphrase --silent --outfile .debug/keypairs/test-keypair-$i.json
        local pubkey=$(solana-keygen pubkey .debug/keypairs/test-keypair-$i.json)
        echo "Keypair $i: $pubkey"
        
        # Airdrop some SOL for testing
        solana airdrop 10 $pubkey --url localhost > /dev/null 2>&1 || true
    done
    
    print_success "Generated $count keypairs in .debug/keypairs/"
}

# Function to run performance benchmarks
benchmark() {
    print_status "Running performance benchmarks..."
    
    # Rust benchmarks
    print_status "Running Rust benchmarks..."
    cd programs/vault
    cargo bench || print_warning "No Rust benchmarks found"
    cd ../..
    
    # Python performance tests
    print_status "Running Python performance tests..."
    python -m pytest tests/ -k "performance or benchmark" -v || print_warning "No Python performance tests found"
}

# Function to check for common issues
health_check() {
    print_status "Running health check..."
    
    # Check Solana installation
    if command -v solana &> /dev/null; then
        print_success "Solana CLI: $(solana --version)"
    else
        print_error "Solana CLI not found"
    fi
    
    # Check Anchor installation
    if command -v anchor &> /dev/null; then
        print_success "Anchor CLI: $(anchor --version)"
    else
        print_error "Anchor CLI not found"
    fi
    
    # Check Rust installation
    if command -v cargo &> /dev/null; then
        print_success "Rust: $(rustc --version)"
    else
        print_error "Rust not found"
    fi
    
    # Check Python installation
    if command -v python3 &> /dev/null; then
        print_success "Python: $(python3 --version)"
    else
        print_error "Python not found"
    fi
    
    # Check Node.js installation
    if command -v node &> /dev/null; then
        print_success "Node.js: $(node --version)"
    else
        print_error "Node.js not found"
    fi
    
    # Check if validator is running
    if solana cluster-version --url localhost > /dev/null 2>&1; then
        print_success "Local validator is running"
    else
        print_warning "Local validator is not running"
    fi
}

# Main command dispatcher
case "$1" in
    "start-validator")
        start_validator
        ;;
    "stop-validator")
        stop_validator
        ;;
    "deploy")
        deploy_program
        ;;
    "debug-test")
        debug_test "$2"
        ;;
    "logs")
        show_logs
        ;;
    "inspect-account")
        inspect_account "$2"
        ;;
    "generate-keypairs")
        generate_keypairs "$2"
        ;;
    "benchmark")
        benchmark
        ;;
    "health-check")
        health_check
        ;;
    *)
        echo "Vault Protocol Debug Helper"
        echo ""
        echo "Usage: $0 <command> [args]"
        echo ""
        echo "Commands:"
        echo "  start-validator      Start local Solana validator with debug settings"
        echo "  stop-validator       Stop local Solana validator"
        echo "  deploy              Build and deploy program to local validator"
        echo "  debug-test <name>   Run specific test with debug output"
        echo "  logs                Show program logs in real-time"
        echo "  inspect-account <addr>  Inspect account data"
        echo "  generate-keypairs [count]  Generate test keypairs (default: 5)"
        echo "  benchmark           Run performance benchmarks"
        echo "  health-check        Check development environment"
        echo ""
        echo "Examples:"
        echo "  $0 start-validator"
        echo "  $0 debug-test test_validate_btc_address"
        echo "  $0 generate-keypairs 10"
        ;;
esac