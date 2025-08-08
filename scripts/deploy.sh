#!/bin/bash

# Vault Protocol Deployment Script
# Comprehensive deployment with verification and rollback capabilities

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
DEPLOYMENT_LOG="$PROJECT_ROOT/.deployment.log"
BACKUP_DIR="$PROJECT_ROOT/.deployment-backups"

# Default values
NETWORK="localnet"
SKIP_VERIFICATION=false
SKIP_BACKUP=false
DRY_RUN=false
VERBOSE=false

print_status() {
    echo -e "${BLUE}[DEPLOY]${NC} $1" | tee -a "$DEPLOYMENT_LOG"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1" | tee -a "$DEPLOYMENT_LOG"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1" | tee -a "$DEPLOYMENT_LOG"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1" | tee -a "$DEPLOYMENT_LOG"
}

# Function to show usage
show_usage() {
    echo "Vault Protocol Deployment Script"
    echo ""
    echo "Usage: $0 [OPTIONS] [NETWORK]"
    echo ""
    echo "Networks:"
    echo "  localnet    Deploy to local validator (default)"
    echo "  devnet      Deploy to Solana devnet"
    echo "  testnet     Deploy to Solana testnet"
    echo "  mainnet     Deploy to Solana mainnet"
    echo ""
    echo "Options:"
    echo "  --skip-verification  Skip post-deployment verification"
    echo "  --skip-backup       Skip creating deployment backup"
    echo "  --dry-run           Show what would be deployed without executing"
    echo "  --verbose, -v       Verbose output"
    echo "  --help, -h          Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0 devnet                    # Deploy to devnet"
    echo "  $0 --dry-run mainnet         # Show mainnet deployment plan"
    echo "  $0 --skip-verification localnet  # Quick local deployment"
}

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        localnet|devnet|testnet|mainnet)
            NETWORK="$1"
            shift
            ;;
        --skip-verification)
            SKIP_VERIFICATION=true
            shift
            ;;
        --skip-backup)
            SKIP_BACKUP=true
            shift
            ;;
        --dry-run)
            DRY_RUN=true
            shift
            ;;
        --verbose|-v)
            VERBOSE=true
            shift
            ;;
        --help|-h)
            show_usage
            exit 0
            ;;
        *)
            print_error "Unknown option: $1"
            show_usage
            exit 1
            ;;
    esac
done

# Initialize deployment log
echo "=== Vault Protocol Deployment - $(date) ===" > "$DEPLOYMENT_LOG"
print_status "Starting deployment to $NETWORK"

# Validate prerequisites
validate_prerequisites() {
    print_status "Validating prerequisites..."
    
    # Check required tools
    local required_tools=("solana" "anchor" "cargo" "node" "npm")
    for tool in "${required_tools[@]}"; do
        if ! command -v "$tool" &> /dev/null; then
            print_error "$tool is not installed or not in PATH"
            exit 1
        fi
    done
    
    # Check Solana configuration
    local current_cluster=$(solana config get | grep "RPC URL" | awk '{print $3}')
    case $NETWORK in
        localnet)
            expected_url="http://localhost:8899"
            ;;
        devnet)
            expected_url="https://api.devnet.solana.com"
            ;;
        testnet)
            expected_url="https://api.testnet.solana.com"
            ;;
        mainnet)
            expected_url="https://api.mainnet-beta.solana.com"
            ;;
    esac
    
    if [[ "$current_cluster" != "$expected_url" ]]; then
        print_status "Switching Solana cluster to $NETWORK..."
        solana config set --url "$expected_url"
    fi
    
    # Check wallet balance for non-local deployments
    if [[ "$NETWORK" != "localnet" ]]; then
        local balance=$(solana balance --lamports 2>/dev/null || echo "0")
        local min_balance=1000000000  # 1 SOL in lamports
        
        if [[ "$balance" -lt "$min_balance" ]]; then
            print_error "Insufficient SOL balance for deployment. Need at least 1 SOL."
            print_status "Current balance: $(echo "scale=9; $balance / 1000000000" | bc) SOL"
            exit 1
        fi
    fi
    
    print_success "Prerequisites validated"
}

# Create deployment backup
create_backup() {
    if [[ "$SKIP_BACKUP" == "true" ]]; then
        print_status "Skipping backup creation"
        return
    fi
    
    print_status "Creating deployment backup..."
    
    mkdir -p "$BACKUP_DIR"
    local backup_name="backup-$(date +%Y%m%d-%H%M%S)-$NETWORK"
    local backup_path="$BACKUP_DIR/$backup_name"
    
    mkdir -p "$backup_path"
    
    # Backup current program binary if it exists
    if [[ -f "$PROJECT_ROOT/programs/vault/target/deploy/vault.so" ]]; then
        cp "$PROJECT_ROOT/programs/vault/target/deploy/vault.so" "$backup_path/"
        print_status "Backed up current program binary"
    fi
    
    # Backup Anchor.toml
    cp "$PROJECT_ROOT/Anchor.toml" "$backup_path/"
    
    # Backup deployment configuration
    if [[ -f "$PROJECT_ROOT/.deployment-config.json" ]]; then
        cp "$PROJECT_ROOT/.deployment-config.json" "$backup_path/"
    fi
    
    # Create backup manifest
    cat > "$backup_path/manifest.json" << EOF
{
    "timestamp": "$(date -Iseconds)",
    "network": "$NETWORK",
    "git_commit": "$(git rev-parse HEAD 2>/dev/null || echo 'unknown')",
    "git_branch": "$(git branch --show-current 2>/dev/null || echo 'unknown')",
    "solana_version": "$(solana --version)",
    "anchor_version": "$(anchor --version)"
}
EOF
    
    print_success "Backup created: $backup_path"
    echo "$backup_path" > "$PROJECT_ROOT/.last-backup"
}

# Build program
build_program() {
    print_status "Building Solana program..."
    
    cd "$PROJECT_ROOT/programs/vault"
    
    # Clean previous build
    if [[ "$VERBOSE" == "true" ]]; then
        anchor clean
        anchor build --verbose
    else
        anchor clean > /dev/null 2>&1
        anchor build
    fi
    
    # Verify build artifacts
    if [[ ! -f "target/deploy/vault.so" ]]; then
        print_error "Program binary not found after build"
        exit 1
    fi
    
    if [[ ! -f "target/idl/vault.json" ]]; then
        print_error "IDL file not found after build"
        exit 1
    fi
    
    # Get program size
    local program_size=$(stat -f%z "target/deploy/vault.so" 2>/dev/null || stat -c%s "target/deploy/vault.so")
    print_status "Program size: $program_size bytes"
    
    # Check program size limits
    local max_size=1048576  # 1MB limit for Solana programs
    if [[ "$program_size" -gt "$max_size" ]]; then
        print_warning "Program size ($program_size bytes) exceeds recommended limit ($max_size bytes)"
    fi
    
    cd "$PROJECT_ROOT"
    print_success "Program build completed"
}

# Deploy program
deploy_program() {
    print_status "Deploying program to $NETWORK..."
    
    if [[ "$DRY_RUN" == "true" ]]; then
        print_status "[DRY RUN] Would deploy program to $NETWORK"
        return
    fi
    
    cd "$PROJECT_ROOT/programs/vault"
    
    # Set deployment cluster
    anchor config set --provider.cluster "$NETWORK"
    
    # Deploy with appropriate flags
    local deploy_flags=""
    if [[ "$VERBOSE" == "true" ]]; then
        deploy_flags="--verbose"
    fi
    
    # Handle different network deployments
    case $NETWORK in
        localnet)
            # Start local validator if not running
            if ! solana cluster-version --url localhost &>/dev/null; then
                print_status "Starting local validator..."
                solana-test-validator --reset --quiet &
                VALIDATOR_PID=$!
                sleep 5
                
                # Store PID for cleanup
                echo $VALIDATOR_PID > "$PROJECT_ROOT/.validator.pid"
            fi
            
            anchor deploy $deploy_flags --provider.cluster localnet
            ;;
        devnet|testnet|mainnet)
            # Confirm deployment for non-local networks
            if [[ "$NETWORK" == "mainnet" ]]; then
                echo ""
                print_warning "⚠️  MAINNET DEPLOYMENT WARNING ⚠️"
                print_warning "You are about to deploy to Solana mainnet."
                print_warning "This will use real SOL and cannot be undone."
                echo ""
                read -p "Type 'DEPLOY TO MAINNET' to confirm: " confirmation
                
                if [[ "$confirmation" != "DEPLOY TO MAINNET" ]]; then
                    print_error "Deployment cancelled"
                    exit 1
                fi
            fi
            
            anchor deploy $deploy_flags --provider.cluster "$NETWORK"
            ;;
    esac
    
    cd "$PROJECT_ROOT"
    print_success "Program deployment completed"
}

# Verify deployment
verify_deployment() {
    if [[ "$SKIP_VERIFICATION" == "true" ]]; then
        print_status "Skipping deployment verification"
        return
    fi
    
    print_status "Verifying deployment..."
    
    # Get program ID from Anchor.toml
    local program_id=$(grep -A 1 "\[programs\.$NETWORK\]" "$PROJECT_ROOT/Anchor.toml" | grep "vault" | cut -d'"' -f2)
    
    if [[ -z "$program_id" ]]; then
        print_error "Could not find program ID in Anchor.toml"
        exit 1
    fi
    
    print_status "Verifying program ID: $program_id"
    
    # Check if program exists on chain
    local cluster_url
    case $NETWORK in
        localnet) cluster_url="http://localhost:8899" ;;
        devnet) cluster_url="https://api.devnet.solana.com" ;;
        testnet) cluster_url="https://api.testnet.solana.com" ;;
        mainnet) cluster_url="https://api.mainnet-beta.solana.com" ;;
    esac
    
    if solana program show "$program_id" --url "$cluster_url" &>/dev/null; then
        print_success "Program verified on $NETWORK"
        
        # Get program info
        local program_info=$(solana program show "$program_id" --url "$cluster_url" --output json)
        local program_size=$(echo "$program_info" | jq -r '.programdataAddress // "unknown"')
        
        print_status "Program data address: $program_size"
    else
        print_error "Program verification failed - program not found on $NETWORK"
        exit 1
    fi
    
    # Verify IDL deployment
    print_status "Verifying IDL deployment..."
    cd "$PROJECT_ROOT/programs/vault"
    
    if anchor idl fetch "$program_id" --provider.cluster "$NETWORK" &>/dev/null; then
        print_success "IDL verified on $NETWORK"
    else
        print_warning "IDL not found on $NETWORK - this may be expected for new deployments"
    fi
    
    cd "$PROJECT_ROOT"
}

# Run deployment tests
run_deployment_tests() {
    print_status "Running deployment tests..."
    
    if [[ "$DRY_RUN" == "true" ]]; then
        print_status "[DRY RUN] Would run deployment tests"
        return
    fi
    
    # Run basic program interaction tests
    cd "$PROJECT_ROOT"
    
    # Set test environment
    export ANCHOR_PROVIDER_URL="$cluster_url"
    export ANCHOR_WALLET="$HOME/.config/solana/id.json"
    
    # Run deployment-specific tests
    if [[ -f "tests/test_deployment.py" ]]; then
        python -m pytest tests/test_deployment.py -v --tb=short
        print_success "Deployment tests passed"
    else
        print_warning "No deployment tests found"
    fi
}

# Save deployment configuration
save_deployment_config() {
    print_status "Saving deployment configuration..."
    
    local config_file="$PROJECT_ROOT/.deployment-config.json"
    
    cat > "$config_file" << EOF
{
    "last_deployment": {
        "timestamp": "$(date -Iseconds)",
        "network": "$NETWORK",
        "program_id": "$(grep -A 1 "\[programs\.$NETWORK\]" "$PROJECT_ROOT/Anchor.toml" | grep "vault" | cut -d'"' -f2)",
        "git_commit": "$(git rev-parse HEAD 2>/dev/null || echo 'unknown')",
        "git_branch": "$(git branch --show-current 2>/dev/null || echo 'unknown')",
        "deployer": "$(solana address)",
        "cluster_url": "$cluster_url"
    }
}
EOF
    
    print_success "Deployment configuration saved"
}

# Cleanup function
cleanup() {
    if [[ -f "$PROJECT_ROOT/.validator.pid" ]]; then
        local validator_pid=$(cat "$PROJECT_ROOT/.validator.pid")
        if kill -0 "$validator_pid" 2>/dev/null; then
            print_status "Stopping local validator..."
            kill "$validator_pid"
            rm "$PROJECT_ROOT/.validator.pid"
        fi
    fi
}

# Set trap for cleanup
trap cleanup EXIT

# Main deployment flow
main() {
    print_status "=== Vault Protocol Deployment ==="
    print_status "Network: $NETWORK"
    print_status "Dry run: $DRY_RUN"
    print_status "Skip verification: $SKIP_VERIFICATION"
    print_status "Skip backup: $SKIP_BACKUP"
    echo ""
    
    validate_prerequisites
    create_backup
    build_program
    deploy_program
    verify_deployment
    run_deployment_tests
    save_deployment_config
    
    print_success "=== Deployment completed successfully! ==="
    print_status "Network: $NETWORK"
    print_status "Program ID: $(grep -A 1 "\[programs\.$NETWORK\]" "$PROJECT_ROOT/Anchor.toml" | grep "vault" | cut -d'"' -f2)"
    print_status "Deployment log: $DEPLOYMENT_LOG"
    
    if [[ "$SKIP_BACKUP" != "true" ]]; then
        print_status "Backup location: $(cat "$PROJECT_ROOT/.last-backup" 2>/dev/null || echo 'none')"
    fi
}

# Run main function
main