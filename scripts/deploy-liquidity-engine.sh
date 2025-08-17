#!/bin/bash

# Primitive Liquidity Engine Deployment Script
# This script deploys the liquidity engine to the specified Solana cluster

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Default values
CLUSTER="devnet"
SKIP_BUILD=false
INITIALIZE=true
VERBOSE=false

# Program IDs (update these with actual deployed program IDs)
JUPITER_PROGRAM_ID="JUP4Fb2cqiRUcaTHdrPC8h2gNsA2ETXiPDD33WcGuJB"
SANCTUM_PROGRAM_ID="5ocnV1qiCgaQR8Jb8xWnVbApfaygJ8tNoZfgPwsgx9kx"
WORMHOLE_PROGRAM_ID="worm2ZoG2kUd4vFXhvjh93UUH596ayRfgQ2MgjNMTth"

print_usage() {
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  -c, --cluster CLUSTER    Target cluster (devnet, testnet, mainnet-beta) [default: devnet]"
    echo "  -s, --skip-build         Skip building the program"
    echo "  -n, --no-init           Skip initialization of the liquidity engine"
    echo "  -v, --verbose           Enable verbose output"
    echo "  -h, --help              Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0                      Deploy to devnet with default settings"
    echo "  $0 -c testnet -s        Deploy to testnet without rebuilding"
    echo "  $0 -c mainnet-beta -n   Deploy to mainnet without initialization"
}

log() {
    if [ "$VERBOSE" = true ]; then
        echo -e "${BLUE}[$(date +'%Y-%m-%d %H:%M:%S')] $1${NC}"
    fi
}

info() {
    echo -e "${GREEN}[INFO] $1${NC}"
}

warn() {
    echo -e "${YELLOW}[WARN] $1${NC}"
}

error() {
    echo -e "${RED}[ERROR] $1${NC}"
    exit 1
}

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -c|--cluster)
            CLUSTER="$2"
            shift 2
            ;;
        -s|--skip-build)
            SKIP_BUILD=true
            shift
            ;;
        -n|--no-init)
            INITIALIZE=false
            shift
            ;;
        -v|--verbose)
            VERBOSE=true
            shift
            ;;
        -h|--help)
            print_usage
            exit 0
            ;;
        *)
            error "Unknown option: $1"
            ;;
    esac
done

# Validate cluster
case $CLUSTER in
    devnet|testnet|mainnet-beta)
        ;;
    *)
        error "Invalid cluster: $CLUSTER. Must be one of: devnet, testnet, mainnet-beta"
        ;;
esac

info "Starting Primitive Liquidity Engine deployment to $CLUSTER"

# Check prerequisites
check_prerequisites() {
    info "Checking prerequisites..."
    
    # Check if Solana CLI is installed
    if ! command -v solana &> /dev/null; then
        error "Solana CLI is not installed. Please install it first."
    fi
    
    # Check if Anchor is installed
    if ! command -v anchor &> /dev/null; then
        error "Anchor CLI is not installed. Please install it first."
    fi
    
    # Check if Node.js is installed
    if ! command -v node &> /dev/null; then
        error "Node.js is not installed. Please install it first."
    fi
    
    # Check wallet
    WALLET_PATH=$(solana config get keypair | awk '{print $2}')
    if [ ! -f "$WALLET_PATH" ]; then
        error "Wallet not found at $WALLET_PATH. Please configure your wallet."
    fi
    
    log "Prerequisites check passed"
}

# Configure Solana for the target cluster
configure_solana() {
    info "Configuring Solana for $CLUSTER cluster..."
    
    case $CLUSTER in
        devnet)
            RPC_URL="https://api.devnet.solana.com"
            ;;
        testnet)
            RPC_URL="https://api.testnet.solana.com"
            ;;
        mainnet-beta)
            RPC_URL="https://api.mainnet-beta.solana.com"
            ;;
    esac
    
    solana config set --url "$RPC_URL"
    
    # Check balance
    BALANCE=$(solana balance --lamports | grep -o '[0-9]*')
    MIN_BALANCE=1000000000  # 1 SOL in lamports
    
    if [ "$BALANCE" -lt "$MIN_BALANCE" ]; then
        warn "Low balance detected: $(echo "scale=9; $BALANCE/1000000000" | bc) SOL"
        if [ "$CLUSTER" = "devnet" ]; then
            info "Requesting airdrop for devnet..."
            solana airdrop 2
        else
            error "Insufficient balance for deployment. Please fund your wallet."
        fi
    fi
    
    log "Solana configured for $CLUSTER"
}

# Build the program
build_program() {
    if [ "$SKIP_BUILD" = true ]; then
        info "Skipping build as requested"
        return
    fi
    
    info "Building the liquidity engine program..."
    
    # Clean previous builds
    if [ -d "target" ]; then
        rm -rf target
        log "Cleaned previous build artifacts"
    fi
    
    # Build the program
    anchor build
    
    if [ $? -ne 0 ]; then
        error "Program build failed"
    fi
    
    info "Program built successfully"
}

# Deploy the program
deploy_program() {
    info "Deploying the liquidity engine program to $CLUSTER..."
    
    # Update Anchor.toml for the target cluster
    sed -i.bak "s/cluster = \".*\"/cluster = \"$CLUSTER\"/" Anchor.toml
    
    # Deploy
    anchor deploy --provider.cluster "$CLUSTER"
    
    if [ $? -ne 0 ]; then
        error "Program deployment failed"
    fi
    
    # Get the deployed program ID
    PROGRAM_ID=$(solana address -k target/deploy/vault-keypair.json)
    info "Program deployed with ID: $PROGRAM_ID"
    
    # Update the program ID in the source code if needed
    log "Updating program ID in source files..."
    sed -i.bak "s/declare_id!(\".*\")/declare_id!(\"$PROGRAM_ID\")/" programs/vault/src/lib.rs
    
    log "Program deployment completed"
}

# Initialize the liquidity engine
initialize_engine() {
    if [ "$INITIALIZE" = false ]; then
        info "Skipping liquidity engine initialization as requested"
        return
    fi
    
    info "Initializing the liquidity engine..."
    
    # Create initialization script
    cat > temp_init.js << EOF
const anchor = require('@project-serum/anchor');
const { Connection, PublicKey, Keypair } = require('@solana/web3.js');

async function initializeLiquidityEngine() {
    // Setup
    const connection = new Connection('$RPC_URL', 'confirmed');
    const wallet = anchor.Wallet.local();
    const provider = new anchor.AnchorProvider(connection, wallet, {});
    anchor.setProvider(provider);
    
    // Load the program
    const programId = new PublicKey('$PROGRAM_ID');
    const idl = JSON.parse(require('fs').readFileSync('target/idl/vault.json', 'utf8'));
    const program = new anchor.Program(idl, programId, provider);
    
    try {
        // Initialize the liquidity engine
        const [liquidityEngine, bump] = await PublicKey.findProgramAddress(
            [Buffer.from('liquidity_engine')],
            programId
        );
        
        console.log('Liquidity Engine PDA:', liquidityEngine.toString());
        
        const tx = await program.methods
            .initializeLiquidityEngine()
            .accounts({
                liquidityEngine,
                authority: wallet.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .rpc();
            
        console.log('Initialization transaction:', tx);
        console.log('Liquidity Engine initialized successfully!');
        
        // Set program configurations
        console.log('Setting up program configurations...');
        
        // Add Jupiter program ID
        await program.methods
            .updateProgramConfig(
                new PublicKey('$JUPITER_PROGRAM_ID'),
                new PublicKey('$SANCTUM_PROGRAM_ID'),
                new PublicKey('$WORMHOLE_PROGRAM_ID')
            )
            .accounts({
                liquidityEngine,
                authority: wallet.publicKey,
            })
            .rpc();
            
        console.log('Program configuration completed!');
        
    } catch (error) {
        console.error('Initialization failed:', error);
        process.exit(1);
    }
}

initializeLiquidityEngine();
EOF
    
    # Run the initialization
    node temp_init.js
    
    if [ $? -ne 0 ]; then
        error "Liquidity engine initialization failed"
    fi
    
    # Clean up
    rm temp_init.js
    
    info "Liquidity engine initialized successfully"
}

# Verify deployment
verify_deployment() {
    info "Verifying deployment..."
    
    # Check if the program account exists
    PROGRAM_ACCOUNT=$(solana account "$PROGRAM_ID" --output json 2>/dev/null)
    
    if [ $? -ne 0 ]; then
        error "Program account verification failed"
    fi
    
    # Check if the liquidity engine is initialized (if initialization was run)
    if [ "$INITIALIZE" = true ]; then
        # This would require a more sophisticated check
        log "Liquidity engine initialization verification would go here"
    fi
    
    info "Deployment verification completed"
}

# Generate deployment summary
generate_summary() {
    info "Generating deployment summary..."
    
    cat > deployment-summary-$CLUSTER.md << EOF
# Primitive Liquidity Engine Deployment Summary

## Deployment Details
- **Date**: $(date)
- **Cluster**: $CLUSTER
- **Program ID**: $PROGRAM_ID
- **Deployer**: $(solana address)

## Program Configuration
- **Jupiter Program ID**: $JUPITER_PROGRAM_ID
- **Sanctum Program ID**: $SANCTUM_PROGRAM_ID
- **Wormhole Program ID**: $WORMHOLE_PROGRAM_ID

## Next Steps
1. Update frontend configuration with the new program ID
2. Run integration tests against the deployed program
3. Monitor program performance and logs
4. Update documentation with deployment-specific information

## Useful Commands
\`\`\`bash
# Check program account
solana account $PROGRAM_ID

# View program logs
solana logs $PROGRAM_ID

# Get liquidity engine state
anchor account liquidityEngineState <ENGINE_PDA>
\`\`\`

## Support
- Documentation: docs/LIQUIDITY_ENGINE.md
- Issues: Create GitHub issues for any problems
- Monitoring: Check Solana Explorer for transaction history
EOF
    
    info "Deployment summary saved to deployment-summary-$CLUSTER.md"
}

# Main deployment flow
main() {
    check_prerequisites
    configure_solana
    build_program
    deploy_program
    initialize_engine
    verify_deployment
    generate_summary
    
    info "🎉 Primitive Liquidity Engine deployment completed successfully!"
    info "Program ID: $PROGRAM_ID"
    info "Cluster: $CLUSTER"
    info "Summary: deployment-summary-$CLUSTER.md"
    
    if [ "$CLUSTER" = "mainnet-beta" ]; then
        warn "⚠️  MAINNET DEPLOYMENT COMPLETE ⚠️"
        warn "Please ensure thorough testing before announcing to users"
        warn "Monitor the program closely for the first few hours"
    fi
}

# Trap errors and cleanup
trap 'error "Deployment failed. Check the logs above for details."' ERR

# Run main function
main "$@"