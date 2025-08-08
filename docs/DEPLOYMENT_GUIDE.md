# Vault Protocol Deployment Guide

This guide covers the complete deployment process for the Vault Protocol across different environments.

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Environment Configuration](#environment-configuration)
3. [Deployment Process](#deployment-process)
4. [Verification](#verification)
5. [Rollback Procedures](#rollback-procedures)
6. [Database Migration](#database-migration)
7. [Troubleshooting](#troubleshooting)

## Prerequisites

### Required Tools

- **Solana CLI** (v1.16.0+)
- **Anchor CLI** (v0.28.0+)
- **Rust** (v1.70.0+)
- **Node.js** (v18.0.0+)
- **Python** (v3.9.0+)
- **Git**
- **jq** (for JSON processing)

### Installation Commands

```bash
# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/v1.16.0/install)"

# Install Anchor CLI
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
avm install latest
avm use latest

# Install Python dependencies
pip install pytest pytest-asyncio solana anchorpy

# Install Node.js dependencies
cd frontend && npm install
```

### Environment Setup

1. **Configure Solana CLI**:
   ```bash
   # Generate keypair (if needed)
   solana-keygen new --outfile ~/.config/solana/id.json
   
   # Set cluster (will be changed by deployment script)
   solana config set --url https://api.devnet.solana.com
   ```

2. **Fund Wallet** (for non-local deployments):
   ```bash
   # For devnet/testnet
   solana airdrop 2
   
   # For mainnet - transfer real SOL to your wallet
   ```

## Environment Configuration

### Available Environments

- **localnet**: Local development with test validator
- **devnet**: Solana devnet for development testing
- **testnet**: Solana testnet for pre-production testing
- **mainnet**: Solana mainnet for production deployment

### Configuration Management

Use the configuration manager to view and modify environment settings:

```bash
# List all environments
python scripts/config-manager.py list

# Show specific environment
python scripts/config-manager.py show devnet

# Modify configuration
python scripts/config-manager.py set devnet security.hsm_enabled true

# Validate configuration
python scripts/config-manager.py validate mainnet

# Generate Anchor.toml for environment
python scripts/config-manager.py anchor devnet
```

### Environment-Specific Settings

#### Localnet
- **Purpose**: Local development and testing
- **Features**: All features enabled, no external dependencies
- **Security**: Minimal security for fast iteration
- **HSM**: Disabled

#### Devnet
- **Purpose**: Development testing with real network
- **Features**: All features enabled, external integrations
- **Security**: Basic security measures
- **HSM**: Disabled
- **Oracles**: Chainlink testnet feeds

#### Testnet
- **Purpose**: Pre-production testing
- **Features**: Production-like configuration
- **Security**: Full security measures
- **HSM**: Enabled (YubiHSM2)
- **Oracles**: Chainlink testnet feeds

#### Mainnet
- **Purpose**: Production deployment
- **Features**: All production features
- **Security**: Maximum security
- **HSM**: Required (YubiHSM2)
- **Oracles**: Chainlink mainnet feeds
- **Monitoring**: Full monitoring and alerting

## Deployment Process

### 1. Pre-Deployment Checklist

- [ ] All tests passing (`make test`)
- [ ] Security audit completed
- [ ] Configuration validated
- [ ] Backup strategy in place
- [ ] Rollback plan prepared
- [ ] Team notified (for production)

### 2. Local Deployment

```bash
# Quick local deployment
./scripts/deploy.sh localnet

# With verbose output
./scripts/deploy.sh --verbose localnet

# Dry run to see what would be deployed
./scripts/deploy.sh --dry-run localnet
```

### 3. Development Network Deployment

```bash
# Deploy to devnet
./scripts/deploy.sh devnet

# Skip verification for faster deployment
./scripts/deploy.sh --skip-verification devnet

# Deploy without creating backup
./scripts/deploy.sh --skip-backup devnet
```

### 4. Production Deployment

```bash
# Deploy to testnet (pre-production)
./scripts/deploy.sh testnet

# Deploy to mainnet (requires confirmation)
./scripts/deploy.sh mainnet
```

### 5. Deployment Script Options

```bash
Usage: ./scripts/deploy.sh [OPTIONS] [NETWORK]

Options:
  --skip-verification  Skip post-deployment verification
  --skip-backup       Skip creating deployment backup
  --dry-run           Show what would be deployed without executing
  --verbose, -v       Verbose output
  --help, -h          Show help message

Examples:
  ./scripts/deploy.sh devnet                    # Deploy to devnet
  ./scripts/deploy.sh --dry-run mainnet         # Show mainnet deployment plan
  ./scripts/deploy.sh --skip-verification localnet  # Quick local deployment
```

## Verification

### Automated Verification

The deployment script automatically runs verification tests:

```bash
# Run deployment verification manually
DEPLOYMENT_NETWORK=devnet python -m pytest tests/test_deployment.py -v
```

### Manual Verification Steps

1. **Program Existence**:
   ```bash
   solana program show <PROGRAM_ID> --url <CLUSTER_URL>
   ```

2. **IDL Verification**:
   ```bash
   anchor idl fetch <PROGRAM_ID> --provider.cluster <NETWORK>
   ```

3. **Basic Functionality**:
   ```bash
   # Run basic interaction tests
   python tests/test_deployment.py
   ```

### Verification Checklist

- [ ] Program deployed and executable
- [ ] IDL accessible and correct
- [ ] Oracle feeds configured
- [ ] Security settings applied
- [ ] Cross-chain connections working (if applicable)
- [ ] Frontend can connect to program
- [ ] Database migration completed

## Rollback Procedures

### When to Rollback

- Deployment verification fails
- Critical bugs discovered post-deployment
- Performance issues detected
- Security vulnerabilities found

### Rollback Process

1. **List Available Backups**:
   ```bash
   ./scripts/rollback.sh --list-backups
   ```

2. **Rollback to Latest Backup**:
   ```bash
   ./scripts/rollback.sh devnet
   ```

3. **Rollback to Specific Backup**:
   ```bash
   ./scripts/rollback.sh devnet backup-20240101-120000
   ```

4. **Dry Run Rollback**:
   ```bash
   ./scripts/rollback.sh --dry-run mainnet
   ```

### Rollback Script Options

```bash
Usage: ./scripts/rollback.sh [OPTIONS] [NETWORK] [BACKUP_NAME]

Options:
  --backup <name>     Specific backup to rollback to
  --list-backups      List available backups
  --dry-run           Show what would be rolled back without executing
  --force             Skip confirmation prompts
  --verbose, -v       Verbose output
  --help, -h          Show help message
```

### Emergency Rollback

For critical production issues:

```bash
# Force rollback without confirmation (use with caution)
./scripts/rollback.sh --force mainnet

# Rollback and immediately verify
./scripts/rollback.sh mainnet && DEPLOYMENT_NETWORK=mainnet python -m pytest tests/test_deployment.py
```

## Database Migration

### Migration Overview

The Vault Protocol uses SQLite databases to store user data and state information. Migration scripts handle data transfer between deployments.

### Migration Commands

1. **Initialize Database**:
   ```bash
   python scripts/migrate.py --network devnet init
   ```

2. **Create Backup**:
   ```bash
   python scripts/migrate.py --network devnet backup
   ```

3. **Migrate Between Networks**:
   ```bash
   python scripts/migrate.py --network testnet migrate --from devnet
   ```

4. **Export User Data**:
   ```bash
   python scripts/migrate.py --network devnet export --output user-data.json
   ```

5. **Import User Data**:
   ```bash
   python scripts/migrate.py --network testnet import --input user-data.json
   ```

6. **Validate Data Integrity**:
   ```bash
   python scripts/migrate.py --network devnet validate
   ```

### Migration Best Practices

- Always create backups before migration
- Validate data integrity after migration
- Test migrations on non-production environments first
- Keep migration logs for audit purposes
- Plan for rollback scenarios

### Database Schema

The migration system manages these tables:

- `user_accounts`: User profile and settings
- `btc_commitments`: BTC commitment records
- `rewards`: Reward distribution history
- `treasury_operations`: Treasury management logs
- `kyc_records`: KYC verification data
- `security_events`: Security audit trail
- `state_channels`: State channel data

## Troubleshooting

### Common Issues

#### 1. Insufficient SOL Balance

**Error**: "Insufficient SOL balance for deployment"

**Solution**:
```bash
# Check balance
solana balance

# For devnet/testnet
solana airdrop 2

# For mainnet - transfer SOL to wallet
```

#### 2. Program Build Failures

**Error**: "Program binary not found after build"

**Solution**:
```bash
# Clean and rebuild
cd programs/vault
anchor clean
anchor build

# Check for Rust compilation errors
cargo check
```

#### 3. Oracle Feed Issues

**Error**: "Oracle feed not found"

**Solution**:
```bash
# Verify oracle configuration
python scripts/config-manager.py show devnet | jq '.chainlink_oracles'

# Update oracle addresses if needed
python scripts/config-manager.py set devnet chainlink_oracles.btc_usd "NEW_ADDRESS"
```

#### 4. Network Connectivity Issues

**Error**: "Failed to connect to cluster"

**Solution**:
```bash
# Check network status
solana cluster-version --url <CLUSTER_URL>

# Switch to different RPC endpoint
solana config set --url <ALTERNATIVE_URL>
```

#### 5. HSM Connection Issues (Production)

**Error**: "HSM connection failed"

**Solution**:
```bash
# Check HSM service status
systemctl status yubihsm-connector

# Restart HSM connector
sudo systemctl restart yubihsm-connector

# Verify HSM configuration
python scripts/config-manager.py show mainnet | jq '.security.hsm_config'
```

### Deployment Logs

Check deployment logs for detailed error information:

```bash
# View deployment log
tail -f .deployment.log

# View rollback log
tail -f .rollback.log

# View migration log
tail -f .migration.log
```

### Getting Help

1. **Check Documentation**: Review this guide and inline help
2. **Run Diagnostics**: Use `./scripts/debug.sh health-check`
3. **Check Logs**: Review deployment and application logs
4. **Validate Configuration**: Run configuration validation
5. **Test Connectivity**: Verify network and service connections

### Support Contacts

- **Development Issues**: Check GitHub issues
- **Security Issues**: Follow security reporting procedures
- **Production Issues**: Contact operations team

## Best Practices

### Pre-Deployment

1. **Test Thoroughly**: Run full test suite before deployment
2. **Review Changes**: Code review and security audit
3. **Plan Rollback**: Prepare rollback strategy
4. **Communicate**: Notify team of deployment plans

### During Deployment

1. **Monitor Progress**: Watch deployment logs
2. **Verify Each Step**: Don't skip verification
3. **Be Ready to Rollback**: Have rollback plan ready
4. **Document Issues**: Log any problems encountered

### Post-Deployment

1. **Verify Functionality**: Run comprehensive tests
2. **Monitor Performance**: Watch system metrics
3. **Check Logs**: Review for errors or warnings
4. **Update Documentation**: Document any changes

### Security Considerations

1. **Protect Private Keys**: Secure wallet and HSM keys
2. **Verify Checksums**: Validate program binary integrity
3. **Monitor Access**: Log all deployment activities
4. **Regular Audits**: Schedule security reviews

This deployment guide provides comprehensive coverage of the Vault Protocol deployment process. Follow these procedures carefully, especially for production deployments, and always have a rollback plan ready.