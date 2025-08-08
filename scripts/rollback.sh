#!/bin/bash

# Vault Protocol Deployment Rollback Script
# Handles rollback to previous deployment state

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
BACKUP_DIR="$PROJECT_ROOT/.deployment-backups"
ROLLBACK_LOG="$PROJECT_ROOT/.rollback.log"

# Default values
NETWORK="localnet"
BACKUP_NAME=""
DRY_RUN=false
FORCE=false
VERBOSE=false

print_status() {
    echo -e "${BLUE}[ROLLBACK]${NC} $1" | tee -a "$ROLLBACK_LOG"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1" | tee -a "$ROLLBACK_LOG"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1" | tee -a "$ROLLBACK_LOG"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1" | tee -a "$ROLLBACK_LOG"
}

# Function to show usage
show_usage() {
    echo "Vault Protocol Deployment Rollback Script"
    echo ""
    echo "Usage: $0 [OPTIONS] [NETWORK] [BACKUP_NAME]"
    echo ""
    echo "Networks:"
    echo "  localnet    Rollback local deployment (default)"
    echo "  devnet      Rollback devnet deployment"
    echo "  testnet     Rollback testnet deployment"
    echo "  mainnet     Rollback mainnet deployment"
    echo ""
    echo "Options:"
    echo "  --backup <name>     Specific backup to rollback to"
    echo "  --list-backups      List available backups"
    echo "  --dry-run           Show what would be rolled back without executing"
    echo "  --force             Skip confirmation prompts"
    echo "  --verbose, -v       Verbose output"
    echo "  --help, -h          Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0 --list-backups                    # List available backups"
    echo "  $0 devnet backup-20240101-120000     # Rollback devnet to specific backup"
    echo "  $0 --dry-run mainnet                 # Show mainnet rollback plan"
}

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        localnet|devnet|testnet|mainnet)
            NETWORK="$1"
            shift
            ;;
        --backup)
            BACKUP_NAME="$2"
            shift 2
            ;;
        --list-backups)
            list_backups
            exit 0
            ;;
        --dry-run)
            DRY_RUN=true
            shift
            ;;
        --force)
            FORCE=true
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
        backup-*)
            BACKUP_NAME="$1"
            shift
            ;;
        *)
            print_error "Unknown option: $1"
            show_usage
            exit 1
            ;;
    esac
done

# Function to list available backups
list_backups() {
    print_status "Available deployment backups:"
    echo ""
    
    if [[ ! -d "$BACKUP_DIR" ]]; then
        print_warning "No backup directory found"
        return
    fi
    
    local backups=($(ls -1 "$BACKUP_DIR" | grep "backup-" | sort -r))
    
    if [[ ${#backups[@]} -eq 0 ]]; then
        print_warning "No backups found"
        return
    fi
    
    for backup in "${backups[@]}"; do
        local backup_path="$BACKUP_DIR/$backup"
        local manifest_file="$backup_path/manifest.json"
        
        if [[ -f "$manifest_file" ]]; then
            local timestamp=$(jq -r '.timestamp' "$manifest_file" 2>/dev/null || echo "unknown")
            local network=$(jq -r '.network' "$manifest_file" 2>/dev/null || echo "unknown")
            local git_commit=$(jq -r '.git_commit' "$manifest_file" 2>/dev/null || echo "unknown")
            
            echo "  📦 $backup"
            echo "     Network: $network"
            echo "     Date: $timestamp"
            echo "     Commit: ${git_commit:0:8}"
            echo ""
        else
            echo "  📦 $backup (no manifest)"
            echo ""
        fi
    done
}

# Function to validate rollback prerequisites
validate_prerequisites() {
    print_status "Validating rollback prerequisites..."
    
    # Check required tools
    local required_tools=("solana" "anchor" "jq")
    for tool in "${required_tools[@]}"; do
        if ! command -v "$tool" &> /dev/null; then
            print_error "$tool is not installed or not in PATH"
            exit 1
        fi
    done
    
    # Check backup directory exists
    if [[ ! -d "$BACKUP_DIR" ]]; then
        print_error "Backup directory not found: $BACKUP_DIR"
        exit 1
    fi
    
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
    
    print_success "Prerequisites validated"
}

# Function to select backup
select_backup() {
    if [[ -n "$BACKUP_NAME" ]]; then
        # Use specified backup
        local backup_path="$BACKUP_DIR/$BACKUP_NAME"
        
        if [[ ! -d "$backup_path" ]]; then
            print_error "Backup not found: $BACKUP_NAME"
            exit 1
        fi
        
        echo "$backup_path"
        return
    fi
    
    # Find latest backup for the network
    local latest_backup=""
    local latest_timestamp=""
    
    for backup_dir in "$BACKUP_DIR"/backup-*; do
        if [[ ! -d "$backup_dir" ]]; then
            continue
        fi
        
        local manifest_file="$backup_dir/manifest.json"
        if [[ ! -f "$manifest_file" ]]; then
            continue
        fi
        
        local backup_network=$(jq -r '.network' "$manifest_file" 2>/dev/null || echo "")
        if [[ "$backup_network" != "$NETWORK" ]]; then
            continue
        fi
        
        local timestamp=$(jq -r '.timestamp' "$manifest_file" 2>/dev/null || echo "")
        if [[ -z "$latest_timestamp" ]] || [[ "$timestamp" > "$latest_timestamp" ]]; then
            latest_backup="$backup_dir"
            latest_timestamp="$timestamp"
        fi
    done
    
    if [[ -z "$latest_backup" ]]; then
        print_error "No backup found for network: $NETWORK"
        exit 1
    fi
    
    echo "$latest_backup"
}

# Function to confirm rollback
confirm_rollback() {
    local backup_path="$1"
    local backup_name=$(basename "$backup_path")
    
    if [[ "$FORCE" == "true" ]]; then
        return
    fi
    
    echo ""
    print_warning "⚠️  DEPLOYMENT ROLLBACK WARNING ⚠️"
    print_warning "You are about to rollback the deployment on $NETWORK"
    print_warning "This will replace the current program with: $backup_name"
    
    if [[ -f "$backup_path/manifest.json" ]]; then
        local timestamp=$(jq -r '.timestamp' "$backup_path/manifest.json" 2>/dev/null || echo "unknown")
        local git_commit=$(jq -r '.git_commit' "$backup_path/manifest.json" 2>/dev/null || echo "unknown")
        
        print_warning "Backup timestamp: $timestamp"
        print_warning "Backup git commit: $git_commit"
    fi
    
    echo ""
    
    if [[ "$NETWORK" == "mainnet" ]]; then
        read -p "Type 'ROLLBACK MAINNET' to confirm: " confirmation
        if [[ "$confirmation" != "ROLLBACK MAINNET" ]]; then
            print_error "Rollback cancelled"
            exit 1
        fi
    else
        read -p "Type 'ROLLBACK' to confirm: " confirmation
        if [[ "$confirmation" != "ROLLBACK" ]]; then
            print_error "Rollback cancelled"
            exit 1
        fi
    fi
}

# Function to create pre-rollback backup
create_pre_rollback_backup() {
    print_status "Creating pre-rollback backup..."
    
    local timestamp=$(date +%Y%m%d-%H%M%S)
    local backup_name="pre-rollback-$timestamp-$NETWORK"
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
    "backup_type": "pre-rollback",
    "git_commit": "$(git rev-parse HEAD 2>/dev/null || echo 'unknown')",
    "git_branch": "$(git branch --show-current 2>/dev/null || echo 'unknown')",
    "solana_version": "$(solana --version)",
    "anchor_version": "$(anchor --version)"
}
EOF
    
    print_success "Pre-rollback backup created: $backup_path"
}

# Function to restore from backup
restore_from_backup() {
    local backup_path="$1"
    
    print_status "Restoring from backup: $(basename "$backup_path")"
    
    if [[ "$DRY_RUN" == "true" ]]; then
        print_status "[DRY RUN] Would restore from backup: $backup_path"
        return
    fi
    
    # Restore program binary
    if [[ -f "$backup_path/vault.so" ]]; then
        mkdir -p "$PROJECT_ROOT/programs/vault/target/deploy"
        cp "$backup_path/vault.so" "$PROJECT_ROOT/programs/vault/target/deploy/"
        print_status "Restored program binary"
    else
        print_warning "No program binary found in backup"
    fi
    
    # Restore Anchor.toml
    if [[ -f "$backup_path/Anchor.toml" ]]; then
        cp "$backup_path/Anchor.toml" "$PROJECT_ROOT/"
        print_status "Restored Anchor.toml"
    else
        print_warning "No Anchor.toml found in backup"
    fi
    
    # Restore deployment configuration
    if [[ -f "$backup_path/.deployment-config.json" ]]; then
        cp "$backup_path/.deployment-config.json" "$PROJECT_ROOT/"
        print_status "Restored deployment configuration"
    fi
}

# Function to redeploy program
redeploy_program() {
    print_status "Redeploying program to $NETWORK..."
    
    if [[ "$DRY_RUN" == "true" ]]; then
        print_status "[DRY RUN] Would redeploy program to $NETWORK"
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
            anchor deploy $deploy_flags --provider.cluster "$NETWORK"
            ;;
    esac
    
    cd "$PROJECT_ROOT"
    print_success "Program redeployment completed"
}

# Function to verify rollback
verify_rollback() {
    print_status "Verifying rollback..."
    
    if [[ "$DRY_RUN" == "true" ]]; then
        print_status "[DRY RUN] Would verify rollback"
        return
    fi
    
    # Get program ID from Anchor.toml
    local program_id=$(grep -A 1 "\[programs\.$NETWORK\]" "$PROJECT_ROOT/Anchor.toml" | grep "vault" | cut -d'"' -f2)
    
    if [[ -z "$program_id" ]]; then
        print_error "Could not find program ID in Anchor.toml"
        exit 1
    fi
    
    # Check if program exists on chain
    local cluster_url
    case $NETWORK in
        localnet) cluster_url="http://localhost:8899" ;;
        devnet) cluster_url="https://api.devnet.solana.com" ;;
        testnet) cluster_url="https://api.testnet.solana.com" ;;
        mainnet) cluster_url="https://api.mainnet-beta.solana.com" ;;
    esac
    
    if solana program show "$program_id" --url "$cluster_url" &>/dev/null; then
        print_success "Rollback verified - program active on $NETWORK"
    else
        print_error "Rollback verification failed - program not found on $NETWORK"
        exit 1
    fi
}

# Function to run rollback tests
run_rollback_tests() {
    print_status "Running rollback verification tests..."
    
    if [[ "$DRY_RUN" == "true" ]]; then
        print_status "[DRY RUN] Would run rollback tests"
        return
    fi
    
    # Set test environment
    export DEPLOYMENT_NETWORK="$NETWORK"
    
    # Run deployment tests to verify rollback
    cd "$PROJECT_ROOT"
    if [[ -f "tests/test_deployment.py" ]]; then
        python -m pytest tests/test_deployment.py -v --tb=short -k "test_program_exists_on_chain or test_cluster_connectivity"
        print_success "Rollback verification tests passed"
    else
        print_warning "No deployment tests found for verification"
    fi
}

# Function to update rollback log
update_rollback_log() {
    local backup_path="$1"
    local backup_name=$(basename "$backup_path")
    
    cat >> "$PROJECT_ROOT/.rollback-history.json" << EOF
{
    "timestamp": "$(date -Iseconds)",
    "network": "$NETWORK",
    "backup_restored": "$backup_name",
    "git_commit_before": "$(git rev-parse HEAD 2>/dev/null || echo 'unknown')",
    "operator": "$(whoami)",
    "success": true
}
EOF
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

# Main rollback flow
main() {
    # Initialize rollback log
    echo "=== Vault Protocol Rollback - $(date) ===" > "$ROLLBACK_LOG"
    
    print_status "=== Vault Protocol Deployment Rollback ==="
    print_status "Network: $NETWORK"
    print_status "Dry run: $DRY_RUN"
    print_status "Force: $FORCE"
    echo ""
    
    validate_prerequisites
    
    local backup_path=$(select_backup)
    local backup_name=$(basename "$backup_path")
    
    print_status "Selected backup: $backup_name"
    
    if [[ -f "$backup_path/manifest.json" ]]; then
        local timestamp=$(jq -r '.timestamp' "$backup_path/manifest.json" 2>/dev/null || echo "unknown")
        local git_commit=$(jq -r '.git_commit' "$backup_path/manifest.json" 2>/dev/null || echo "unknown")
        
        print_status "Backup timestamp: $timestamp"
        print_status "Backup git commit: $git_commit"
    fi
    
    confirm_rollback "$backup_path"
    create_pre_rollback_backup
    restore_from_backup "$backup_path"
    redeploy_program
    verify_rollback
    run_rollback_tests
    update_rollback_log "$backup_path"
    
    print_success "=== Rollback completed successfully! ==="
    print_status "Network: $NETWORK"
    print_status "Restored backup: $backup_name"
    print_status "Rollback log: $ROLLBACK_LOG"
}

# Handle list-backups command
if [[ "$1" == "--list-backups" ]]; then
    list_backups
    exit 0
fi

# Run main function
main