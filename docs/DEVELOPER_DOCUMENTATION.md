# Vault Protocol Developer Documentation

## Table of Contents

1. [Development Setup](#development-setup)
2. [Configuration Management](#configuration-management)
3. [Deployment Guide](#deployment-guide)
4. [Testing Framework](#testing-framework)
5. [Architecture Overview](#architecture-overview)
6. [API Integration](#api-integration)
7. [Security Implementation](#security-implementation)
8. [Performance Optimization](#performance-optimization)
9. [Monitoring and Logging](#monitoring-and-logging)
10. [Contributing Guidelines](#contributing-guidelines)

## Development Setup

### Prerequisites

- **Rust**: 1.70+ with Solana toolchain
- **Node.js**: 18+ for frontend development
- **Python**: 3.9+ for configuration and testing
- **Solana CLI**: Latest version
- **Anchor Framework**: 0.28+
- **Git**: Version control

### Environment Setup

```bash
# Clone repository
git clone https://github.com/vaultprotocol/vault.git
cd vault

# Install Rust and Solana
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
sh -c "$(curl -sSfL https://release.solana.com/v1.16.0/install)"

# Install Anchor
npm install -g @coral-xyz/anchor-cli

# Install Python dependencies
pip install -r requirements.txt

# Install frontend dependencies
cd frontend && npm install
```

### Local Development

```bash
# Start local Solana validator
solana-test-validator

# Build and deploy programs
anchor build
anchor deploy

# Start frontend development server
cd frontend && npm run dev

# Run Python configuration server
python scripts/config-manager.py
```

### Development Tools

- **IDE**: VS Code with Rust and Solana extensions
- **Debugging**: Solana logs and Anchor test framework
- **Testing**: pytest for Python, Jest for TypeScript
- **Linting**: Clippy for Rust, ESLint for TypeScript

## Configuration Management

### Configuration Architecture

The Vault Protocol uses a hierarchical configuration system:

1. **Environment Configs**: Base settings per environment
2. **Component Configs**: Specific component settings
3. **User Overrides**: Runtime configuration changes
4. **Security Configs**: Encrypted sensitive settings

### Environment Configuration

#### `config/environments.json`

```json
{
  "development": {
    "solana_cluster": "devnet",
    "chainlink_network": "testnet",
    "treasury_deposits": {
      "amount_usd": 50,
      "frequency_days": 14
    },
    "security": {
      "require_2fa": false,
      "kyc_threshold_btc": 1.0
    }
  },
  "testnet": {
    "solana_cluster": "testnet",
    "chainlink_network": "testnet",
    "treasury_deposits": {
      "amount_usd": 50,
      "frequency_days": 14
    },
    "security": {
      "require_2fa": true,
      "kyc_threshold_btc": 1.0
    }
  },
  "mainnet": {
    "solana_cluster": "mainnet-beta",
    "chainlink_network": "mainnet",
    "treasury_deposits": {
      "amount_usd": 50,
      "frequency_days": 14
    },
    "security": {
      "require_2fa": true,
      "kyc_threshold_btc": 1.0
    }
  }
}
```

### Component Configurations

#### Chainlink Oracle Configuration (`config/chainlink.py`)

```python
"""Chainlink Oracle Configuration"""

import os
from typing import Dict, List
from dataclasses import dataclass

@dataclass
class OracleConfig:
    """Oracle feed configuration"""
    feed_address: str
    update_interval: int  # seconds
    deviation_threshold: float  # percentage
    heartbeat_interval: int  # seconds

class ChainlinkConfig:
    """Chainlink oracle configuration manager"""
    
    def __init__(self, network: str = "mainnet"):
        self.network = network
        self.feeds = self._load_feeds()
    
    def _load_feeds(self) -> Dict[str, OracleConfig]:
        """Load oracle feed configurations"""
        if self.network == "mainnet":
            return {
                "BTC_USD": OracleConfig(
                    feed_address="GVXRSBjFk6e6J3NbVPXohDJetcTjaeeuykUpbQF8UoMU",
                    update_interval=60,
                    deviation_threshold=0.5,
                    heartbeat_interval=3600
                ),
                "ETH_USD": OracleConfig(
                    feed_address="JBu1AL4obBcCMqKBBxhpWCNUt136ijcuMZLFvTP7iWdB",
                    update_interval=60,
                    deviation_threshold=0.5,
                    heartbeat_interval=3600
                ),
                "SOL_USD": OracleConfig(
                    feed_address="H6ARHf6YXhGYeQfUzQNGk6rDNnLBQKrenN712K4AQJEG",
                    update_interval=60,
                    deviation_threshold=0.5,
                    heartbeat_interval=3600
                )
            }
        else:
            # Testnet/devnet configurations
            return {
                "BTC_USD": OracleConfig(
                    feed_address="HovQMDrbAgAYPCmHVSrezcSmkMtXSSUsLDFANExrPy2A",
                    update_interval=60,
                    deviation_threshold=1.0,
                    heartbeat_interval=3600
                )
            }
    
    def get_feed_config(self, pair: str) -> OracleConfig:
        """Get configuration for specific feed pair"""
        return self.feeds.get(pair)
    
    def update_feed_config(self, pair: str, config: OracleConfig):
        """Update feed configuration"""
        self.feeds[pair] = config
        self._save_config()
    
    def _save_config(self):
        """Save configuration to file"""
        # Implementation for persisting config changes
        pass

# Usage example
chainlink_config = ChainlinkConfig("mainnet")
btc_feed = chainlink_config.get_feed_config("BTC_USD")
```

#### Validator Configuration (`config/validators.py`)

```python
"""Validator Selection Configuration"""

from typing import Dict, List, Optional
from dataclasses import dataclass
from enum import Enum

class ValidatorNetwork(Enum):
    SOLANA = "solana"
    ETHEREUM = "ethereum"
    COSMOS = "cosmos"

@dataclass
class ValidatorInfo:
    """Validator information"""
    address: str
    name: str
    commission: float
    apy: float
    uptime: float
    stake_amount: int
    network: ValidatorNetwork

class ValidatorConfig:
    """Validator selection and management"""
    
    def __init__(self):
        self.validators = self._load_validators()
        self.allocations = self._load_allocations()
    
    def _load_validators(self) -> Dict[str, List[ValidatorInfo]]:
        """Load validator configurations"""
        return {
            "solana": [
                ValidatorInfo(
                    address="7Np41oeYqPefeNQEHSv1UDhYrehxin3NStELsSKCT4K2",
                    name="Solana Foundation",
                    commission=0.05,
                    apy=0.065,
                    uptime=0.999,
                    stake_amount=1000000,
                    network=ValidatorNetwork.SOLANA
                ),
                ValidatorInfo(
                    address="StepeLdhJ2znRjHcZdjwMWsC4nTRURNKQY8Nca82LJp",
                    name="Step Finance",
                    commission=0.03,
                    apy=0.068,
                    uptime=0.998,
                    stake_amount=800000,
                    network=ValidatorNetwork.SOLANA
                )
            ],
            "cosmos": [
                ValidatorInfo(
                    address="cosmosvaloper1sjllsnramtg3ewxqwwrwjxfgc4n4ef9u2lcnj0",
                    name="Everstake",
                    commission=0.05,
                    apy=0.12,
                    uptime=0.999,
                    stake_amount=500000,
                    network=ValidatorNetwork.COSMOS
                ),
                ValidatorInfo(
                    address="cosmosvaloper1c4k24jzduc365kywrsvf5ujz4ya6mwympnc4en",
                    name="Cephalopod Equipment",
                    commission=0.04,
                    apy=0.125,
                    uptime=0.998,
                    stake_amount=300000,
                    network=ValidatorNetwork.COSMOS
                )
            ]
        }
    
    def _load_allocations(self) -> Dict[str, Dict[str, float]]:
        """Load staking allocations"""
        return {
            "solana": {"total_percentage": 0.40},
            "ethereum": {"total_percentage": 0.30},
            "cosmos": {
                "total_percentage": 0.30,
                "hub_percentage": 0.20,  # Everstake/Cephalopod
                "osmosis_percentage": 0.10
            }
        }
    
    def get_validators(self, network: str) -> List[ValidatorInfo]:
        """Get validators for specific network"""
        return self.validators.get(network, [])
    
    def select_optimal_validators(self, network: str, amount: int) -> List[ValidatorInfo]:
        """Select optimal validators based on performance metrics"""
        validators = self.get_validators(network)
        
        # Sort by composite score (APY, uptime, low commission)
        def score(v: ValidatorInfo) -> float:
            return (v.apy * 0.4) + (v.uptime * 0.4) + ((1 - v.commission) * 0.2)
        
        return sorted(validators, key=score, reverse=True)
    
    def get_allocation(self, network: str) -> float:
        """Get allocation percentage for network"""
        return self.allocations.get(network, {}).get("total_percentage", 0.0)

# Usage example
validator_config = ValidatorConfig()
solana_validators = validator_config.select_optimal_validators("solana", 1000000)
```

#### Treasury Configuration (`config/treasury.py`)

```python
"""Treasury Management Configuration"""

from typing import Dict, List
from dataclasses import dataclass
from datetime import datetime, timedelta

@dataclass
class AssetAllocation:
    """Asset allocation configuration"""
    sol_percentage: float = 0.40
    eth_percentage: float = 0.30
    atom_percentage: float = 0.30

@dataclass
class DepositSchedule:
    """Deposit schedule configuration"""
    amount_usd: float = 50.0
    frequency_days: int = 14
    next_deposit: datetime = None

class TreasuryConfig:
    """Treasury management configuration"""
    
    def __init__(self):
        self.allocation = AssetAllocation()
        self.deposit_schedule = DepositSchedule()
        self.rebalance_threshold = 0.05  # 5% deviation triggers rebalance
        self.minimum_balances = self._load_minimum_balances()
    
    def _load_minimum_balances(self) -> Dict[str, float]:
        """Load minimum balance requirements"""
        return {
            "sol": 100.0,    # Minimum SOL balance
            "eth": 0.1,      # Minimum ETH balance
            "atom": 50.0,    # Minimum ATOM balance
            "usdc": 1000.0   # Minimum USDC for operations
        }
    
    def calculate_target_allocation(self, total_value_usd: float) -> Dict[str, float]:
        """Calculate target allocation in USD"""
        return {
            "sol": total_value_usd * self.allocation.sol_percentage,
            "eth": total_value_usd * self.allocation.eth_percentage,
            "atom": total_value_usd * self.allocation.atom_percentage
        }
    
    def needs_rebalancing(self, current_allocation: Dict[str, float], 
                         total_value: float) -> bool:
        """Check if treasury needs rebalancing"""
        target = self.calculate_target_allocation(total_value)
        
        for asset, current_value in current_allocation.items():
            target_value = target.get(asset, 0)
            if target_value > 0:
                deviation = abs(current_value - target_value) / target_value
                if deviation > self.rebalance_threshold:
                    return True
        return False
    
    def get_next_deposit_date(self) -> datetime:
        """Get next scheduled deposit date"""
        if self.deposit_schedule.next_deposit is None:
            return datetime.now() + timedelta(days=self.deposit_schedule.frequency_days)
        return self.deposit_schedule.next_deposit
    
    def update_deposit_schedule(self, amount_usd: float, frequency_days: int):
        """Update deposit schedule"""
        self.deposit_schedule.amount_usd = amount_usd
        self.deposit_schedule.frequency_days = frequency_days
        self.deposit_schedule.next_deposit = datetime.now() + timedelta(days=frequency_days)

# Usage example
treasury_config = TreasuryConfig()
target_allocation = treasury_config.calculate_target_allocation(100000)  # $100k
```

#### Dashboard Configuration (`config/dashboard.py`)

```python
"""Dashboard Display Configuration"""

from typing import Dict, List
from dataclasses import dataclass

@dataclass
class DisplaySettings:
    """Dashboard display settings"""
    refresh_interval: int = 30  # seconds
    show_allocations: bool = False  # Hide from users
    show_total_assets: bool = True
    show_rewards_usd: bool = True
    decimal_places: int = 6

@dataclass
class LanguageConfig:
    """Multi-language configuration"""
    default_language: str = "en"
    supported_languages: List[str] = None
    
    def __post_init__(self):
        if self.supported_languages is None:
            self.supported_languages = ["en", "es", "zh", "ja"]

class DashboardConfig:
    """Dashboard configuration manager"""
    
    def __init__(self):
        self.display = DisplaySettings()
        self.language = LanguageConfig()
        self.features = self._load_feature_flags()
    
    def _load_feature_flags(self) -> Dict[str, bool]:
        """Load feature flags"""
        return {
            "auto_reinvest": True,
            "lightning_payments": True,
            "usdc_payments": True,
            "kyc_verification": True,
            "hardware_wallets": True,
            "state_channels": True,
            "cross_chain_staking": True
        }
    
    def is_feature_enabled(self, feature: str) -> bool:
        """Check if feature is enabled"""
        return self.features.get(feature, False)
    
    def get_display_config(self) -> DisplaySettings:
        """Get display configuration"""
        return self.display
    
    def get_supported_languages(self) -> List[str]:
        """Get supported languages"""
        return self.language.supported_languages
    
    def update_feature_flag(self, feature: str, enabled: bool):
        """Update feature flag"""
        self.features[feature] = enabled

# Usage example
dashboard_config = DashboardConfig()
if dashboard_config.is_feature_enabled("lightning_payments"):
    # Enable Lightning Network payments
    pass
```

### Configuration Management Script

#### `scripts/config-manager.py`

```python
#!/usr/bin/env python3
"""Configuration Management Script"""

import json
import os
import sys
from typing import Dict, Any
import argparse

class ConfigManager:
    """Centralized configuration management"""
    
    def __init__(self, environment: str = "development"):
        self.environment = environment
        self.config_dir = "config"
        self.env_config = self._load_environment_config()
    
    def _load_environment_config(self) -> Dict[str, Any]:
        """Load environment-specific configuration"""
        env_file = os.path.join(self.config_dir, "environments.json")
        with open(env_file, 'r') as f:
            all_configs = json.load(f)
        return all_configs.get(self.environment, {})
    
    def get_config(self, component: str) -> Dict[str, Any]:
        """Get configuration for specific component"""
        config_file = os.path.join(self.config_dir, f"{component}.py")
        if os.path.exists(config_file):
            # Import and return configuration
            spec = importlib.util.spec_from_file_location(component, config_file)
            module = importlib.util.module_from_spec(spec)
            spec.loader.exec_module(module)
            return getattr(module, f"{component.title()}Config")()
        return {}
    
    def update_config(self, component: str, updates: Dict[str, Any]):
        """Update component configuration"""
        # Implementation for updating configurations
        pass
    
    def validate_config(self) -> bool:
        """Validate all configurations"""
        required_components = [
            "chainlink", "validators", "treasury", "dashboard"
        ]
        
        for component in required_components:
            try:
                config = self.get_config(component)
                if not config:
                    print(f"Missing configuration for {component}")
                    return False
            except Exception as e:
                print(f"Invalid configuration for {component}: {e}")
                return False
        
        return True
    
    def export_config(self, output_file: str):
        """Export all configurations to file"""
        all_configs = {}
        components = ["chainlink", "validators", "treasury", "dashboard"]
        
        for component in components:
            all_configs[component] = self.get_config(component)
        
        with open(output_file, 'w') as f:
            json.dump(all_configs, f, indent=2, default=str)

def main():
    parser = argparse.ArgumentParser(description="Vault Protocol Configuration Manager")
    parser.add_argument("--env", default="development", help="Environment")
    parser.add_argument("--validate", action="store_true", help="Validate configurations")
    parser.add_argument("--export", help="Export configurations to file")
    
    args = parser.parse_args()
    
    config_manager = ConfigManager(args.env)
    
    if args.validate:
        if config_manager.validate_config():
            print("All configurations are valid")
            sys.exit(0)
        else:
            print("Configuration validation failed")
            sys.exit(1)
    
    if args.export:
        config_manager.export_config(args.export)
        print(f"Configurations exported to {args.export}")

if __name__ == "__main__":
    main()
```

## Deployment Guide

### Deployment Architecture

The Vault Protocol uses a multi-stage deployment process:

1. **Local Development**: Local validator and test environment
2. **Devnet Deployment**: Solana devnet for initial testing
3. **Testnet Deployment**: Full testnet with external integrations
4. **Mainnet Deployment**: Production deployment with security audits

### Pre-Deployment Checklist

#### Security Requirements

- [ ] Smart contracts audited by Certik
- [ ] Slither static analysis passed
- [ ] All tests passing (unit, integration, security)
- [ ] HSM keys generated and secured
- [ ] Multisig wallet configured
- [ ] 2FA enabled for all operators

#### Configuration Requirements

- [ ] Environment configurations validated
- [ ] Oracle feeds configured and tested
- [ ] Validator selections finalized
- [ ] Treasury deposit schedule set
- [ ] KYC integration tested

#### Infrastructure Requirements

- [ ] Monitoring systems deployed
- [ ] Alerting configured
- [ ] Backup systems in place
- [ ] Rollback procedures tested
- [ ] Performance benchmarks met

### Deployment Scripts

#### `scripts/deploy.sh`

```bash
#!/bin/bash
"""Deployment Script for Vault Protocol"""

set -e

# Configuration
ENVIRONMENT=${1:-"devnet"}
PROGRAM_ID=""
UPGRADE_AUTHORITY=""

echo "Deploying Vault Protocol to $ENVIRONMENT"

# Validate environment
case $ENVIRONMENT in
    "devnet"|"testnet"|"mainnet")
        echo "Valid environment: $ENVIRONMENT"
        ;;
    *)
        echo "Invalid environment. Use: devnet, testnet, or mainnet"
        exit 1
        ;;
esac

# Load environment configuration
source config/environments/$ENVIRONMENT.env

# Pre-deployment checks
echo "Running pre-deployment checks..."
python scripts/config-manager.py --env $ENVIRONMENT --validate
if [ $? -ne 0 ]; then
    echo "Configuration validation failed"
    exit 1
fi

# Build programs
echo "Building Solana programs..."
anchor build

# Run tests
echo "Running test suite..."
anchor test --skip-local-validator
python -m pytest tests/ -v

# Deploy to Solana
echo "Deploying to Solana $ENVIRONMENT..."
if [ "$ENVIRONMENT" = "mainnet" ]; then
    # Mainnet deployment with upgrade authority
    solana program deploy \
        --url $SOLANA_RPC_URL \
        --keypair $DEPLOY_KEYPAIR \
        --program-id $PROGRAM_ID \
        --upgrade-authority $UPGRADE_AUTHORITY \
        target/deploy/vault.so
else
    # Devnet/testnet deployment
    anchor deploy --provider.cluster $ENVIRONMENT
fi

# Initialize program accounts
echo "Initializing program accounts..."
python scripts/initialize-accounts.py --env $ENVIRONMENT

# Deploy frontend
echo "Deploying frontend..."
cd frontend
npm run build
npm run deploy:$ENVIRONMENT

# Verify deployment
echo "Verifying deployment..."
python scripts/verify-deployment.py --env $ENVIRONMENT

# Update monitoring
echo "Updating monitoring configuration..."
python scripts/monitoring-service.py --env $ENVIRONMENT --update

echo "Deployment completed successfully!"
echo "Program ID: $(solana address -k target/deploy/vault-keypair.json)"
echo "Frontend URL: $FRONTEND_URL"
```

#### `scripts/rollback.sh`

```bash
#!/bin/bash
"""Rollback Script for Vault Protocol"""

set -e

ENVIRONMENT=${1:-"devnet"}
ROLLBACK_VERSION=${2:-""}

if [ -z "$ROLLBACK_VERSION" ]; then
    echo "Usage: $0 <environment> <rollback_version>"
    exit 1
fi

echo "Rolling back Vault Protocol on $ENVIRONMENT to version $ROLLBACK_VERSION"

# Verify rollback version exists
if [ ! -d "backups/$ROLLBACK_VERSION" ]; then
    echo "Rollback version $ROLLBACK_VERSION not found"
    exit 1
fi

# Stop current services
echo "Stopping current services..."
systemctl stop vault-monitoring
systemctl stop vault-frontend

# Rollback Solana program
echo "Rolling back Solana program..."
solana program deploy \
    --url $SOLANA_RPC_URL \
    --keypair $DEPLOY_KEYPAIR \
    --program-id $PROGRAM_ID \
    backups/$ROLLBACK_VERSION/vault.so

# Rollback frontend
echo "Rolling back frontend..."
cd frontend
cp -r ../backups/$ROLLBACK_VERSION/frontend/* .
npm run build
npm run deploy:$ENVIRONMENT

# Restore configuration
echo "Restoring configuration..."
cp backups/$ROLLBACK_VERSION/config/* config/

# Restart services
echo "Restarting services..."
systemctl start vault-monitoring
systemctl start vault-frontend

# Verify rollback
echo "Verifying rollback..."
python scripts/verify-deployment.py --env $ENVIRONMENT --version $ROLLBACK_VERSION

echo "Rollback completed successfully!"
```

### Database Migration

#### `scripts/migrate.py`

```python
#!/usr/bin/env python3
"""Database Migration Script"""

import os
import json
import sqlite3
from typing import List, Dict
from datetime import datetime

class DatabaseMigrator:
    """Handle database migrations for Vault Protocol"""
    
    def __init__(self, db_path: str = ".vault-localnet.db"):
        self.db_path = db_path
        self.migrations_dir = "migrations"
        self.connection = None
    
    def connect(self):
        """Connect to database"""
        self.connection = sqlite3.connect(self.db_path)
        self.connection.row_factory = sqlite3.Row
    
    def disconnect(self):
        """Disconnect from database"""
        if self.connection:
            self.connection.close()
    
    def get_current_version(self) -> int:
        """Get current database version"""
        try:
            cursor = self.connection.cursor()
            cursor.execute("SELECT version FROM schema_version ORDER BY version DESC LIMIT 1")
            result = cursor.fetchone()
            return result[0] if result else 0
        except sqlite3.OperationalError:
            # Table doesn't exist, version 0
            return 0
    
    def get_available_migrations(self) -> List[Dict]:
        """Get list of available migrations"""
        migrations = []
        if os.path.exists(self.migrations_dir):
            for filename in sorted(os.listdir(self.migrations_dir)):
                if filename.endswith('.sql'):
                    version = int(filename.split('_')[0])
                    migrations.append({
                        'version': version,
                        'filename': filename,
                        'path': os.path.join(self.migrations_dir, filename)
                    })
        return migrations
    
    def run_migration(self, migration: Dict):
        """Run a single migration"""
        print(f"Running migration {migration['version']}: {migration['filename']}")
        
        with open(migration['path'], 'r') as f:
            sql = f.read()
        
        cursor = self.connection.cursor()
        cursor.executescript(sql)
        
        # Record migration
        cursor.execute(
            "INSERT INTO schema_version (version, applied_at) VALUES (?, ?)",
            (migration['version'], datetime.now().isoformat())
        )
        
        self.connection.commit()
    
    def migrate(self):
        """Run all pending migrations"""
        self.connect()
        
        # Create schema_version table if it doesn't exist
        cursor = self.connection.cursor()
        cursor.execute("""
            CREATE TABLE IF NOT EXISTS schema_version (
                version INTEGER PRIMARY KEY,
                applied_at TEXT NOT NULL
            )
        """)
        self.connection.commit()
        
        current_version = self.get_current_version()
        available_migrations = self.get_available_migrations()
        
        pending_migrations = [
            m for m in available_migrations 
            if m['version'] > current_version
        ]
        
        if not pending_migrations:
            print("No pending migrations")
            return
        
        print(f"Running {len(pending_migrations)} migrations...")
        
        for migration in pending_migrations:
            try:
                self.run_migration(migration)
                print(f"Migration {migration['version']} completed")
            except Exception as e:
                print(f"Migration {migration['version']} failed: {e}")
                self.connection.rollback()
                raise
        
        print("All migrations completed successfully")
        self.disconnect()
    
    def rollback(self, target_version: int):
        """Rollback to specific version"""
        self.connect()
        current_version = self.get_current_version()
        
        if target_version >= current_version:
            print("Target version is not lower than current version")
            return
        
        # This is a simplified rollback - in production, you'd need
        # proper rollback scripts for each migration
        print(f"Rolling back from version {current_version} to {target_version}")
        
        cursor = self.connection.cursor()
        cursor.execute(
            "DELETE FROM schema_version WHERE version > ?",
            (target_version,)
        )
        self.connection.commit()
        
        print("Rollback completed - manual data restoration may be required")
        self.disconnect()

def main():
    import argparse
    
    parser = argparse.ArgumentParser(description="Database Migration Tool")
    parser.add_argument("--migrate", action="store_true", help="Run migrations")
    parser.add_argument("--rollback", type=int, help="Rollback to version")
    parser.add_argument("--db", default=".vault-localnet.db", help="Database path")
    
    args = parser.parse_args()
    
    migrator = DatabaseMigrator(args.db)
    
    if args.migrate:
        migrator.migrate()
    elif args.rollback is not None:
        migrator.rollback(args.rollback)
    else:
        print("Use --migrate or --rollback <version>")

if __name__ == "__main__":
    main()
```

### Verification Scripts

#### `scripts/verify-deployment.py`

```python
#!/usr/bin/env python3
"""Deployment Verification Script"""

import asyncio
import json
from solana.rpc.async_api import AsyncClient
from solana.publickey import PublicKey

class DeploymentVerifier:
    """Verify deployment integrity"""
    
    def __init__(self, environment: str):
        self.environment = environment
        self.rpc_urls = {
            "devnet": "https://api.devnet.solana.com",
            "testnet": "https://api.testnet.solana.com",
            "mainnet": "https://api.mainnet-beta.solana.com"
        }
        self.client = AsyncClient(self.rpc_urls[environment])
    
    async def verify_program_deployment(self, program_id: str) -> bool:
        """Verify program is deployed and executable"""
        try:
            pubkey = PublicKey(program_id)
            account_info = await self.client.get_account_info(pubkey)
            
            if account_info.value is None:
                print(f"Program {program_id} not found")
                return False
            
            if not account_info.value.executable:
                print(f"Program {program_id} is not executable")
                return False
            
            print(f"Program {program_id} verified successfully")
            return True
            
        except Exception as e:
            print(f"Program verification failed: {e}")
            return False
    
    async def verify_accounts_initialized(self, accounts: list) -> bool:
        """Verify required accounts are initialized"""
        for account_id in accounts:
            try:
                pubkey = PublicKey(account_id)
                account_info = await self.client.get_account_info(pubkey)
                
                if account_info.value is None:
                    print(f"Account {account_id} not initialized")
                    return False
                
                print(f"Account {account_id} verified")
                
            except Exception as e:
                print(f"Account verification failed for {account_id}: {e}")
                return False
        
        return True
    
    async def verify_oracle_feeds(self, feeds: dict) -> bool:
        """Verify oracle feeds are active"""
        for feed_name, feed_id in feeds.items():
            try:
                pubkey = PublicKey(feed_id)
                account_info = await self.client.get_account_info(pubkey)
                
                if account_info.value is None:
                    print(f"Oracle feed {feed_name} not found")
                    return False
                
                # Additional oracle-specific checks could go here
                print(f"Oracle feed {feed_name} verified")
                
            except Exception as e:
                print(f"Oracle feed verification failed for {feed_name}: {e}")
                return False
        
        return True
    
    async def run_verification(self, config_file: str) -> bool:
        """Run complete deployment verification"""
        with open(config_file, 'r') as f:
            config = json.load(f)
        
        print(f"Verifying deployment on {self.environment}")
        
        # Verify program deployment
        program_verified = await self.verify_program_deployment(
            config['program_id']
        )
        
        # Verify account initialization
        accounts_verified = await self.verify_accounts_initialized(
            config['required_accounts']
        )
        
        # Verify oracle feeds
        oracles_verified = await self.verify_oracle_feeds(
            config['oracle_feeds']
        )
        
        all_verified = program_verified and accounts_verified and oracles_verified
        
        if all_verified:
            print("✅ Deployment verification successful")
        else:
            print("❌ Deployment verification failed")
        
        await self.client.close()
        return all_verified

async def main():
    import argparse
    
    parser = argparse.ArgumentParser(description="Verify Vault Protocol Deployment")
    parser.add_argument("--env", required=True, help="Environment to verify")
    parser.add_argument("--config", help="Configuration file")
    
    args = parser.parse_args()
    
    config_file = args.config or f"config/deployment-{args.env}.json"
    
    verifier = DeploymentVerifier(args.env)
    success = await verifier.run_verification(config_file)
    
    exit(0 if success else 1)

if __name__ == "__main__":
    asyncio.run(main())
```

## Testing Framework

### Test Architecture

The Vault Protocol uses a comprehensive testing strategy:

1. **Unit Tests**: Individual component testing
2. **Integration Tests**: Cross-component interaction testing
3. **Security Tests**: Security vulnerability testing
4. **Performance Tests**: Load and stress testing
5. **End-to-End Tests**: Complete user journey testing

### Running Tests

```bash
# Run all tests
make test

# Run specific test categories
make test-unit
make test-integration
make test-security
make test-performance

# Run tests with coverage
make test-coverage

# Run tests in parallel
make test-parallel
```

### Test Configuration

#### `tests/integration_test_config.py`

```python
"""Integration Test Configuration"""

import os
from dataclasses import dataclass
from typing import Dict, List

@dataclass
class TestConfig:
    """Test environment configuration"""
    solana_cluster: str = "localnet"
    test_keypair_path: str = "tests/keypairs/test-keypair.json"
    program_id: str = ""
    oracle_mock: bool = True
    parallel_workers: int = 4
    timeout_seconds: int = 30

class IntegrationTestConfig:
    """Integration test configuration manager"""
    
    def __init__(self):
        self.config = TestConfig()
        self.test_accounts = self._generate_test_accounts()
        self.mock_data = self._load_mock_data()
    
    def _generate_test_accounts(self) -> Dict[str, str]:
        """Generate test account keypairs"""
        return {
            "user1": "tests/keypairs/user1.json",
            "user2": "tests/keypairs/user2.json",
            "treasury": "tests/keypairs/treasury.json",
            "multisig": "tests/keypairs/multisig.json"
        }
    
    def _load_mock_data(self) -> Dict[str, any]:
        """Load mock data for testing"""
        return {
            "btc_prices": [45000, 46000, 44500, 47000],
            "eth_prices": [3000, 3100, 2950, 3200],
            "sol_prices": [100, 105, 98, 110],
            "staking_rewards": [0.065, 0.068, 0.062, 0.070],
            "user_commitments": [
                {"user": "user1", "amount": 100000000},  # 1 BTC
                {"user": "user2", "amount": 50000000}    # 0.5 BTC
            ]
        }
    
    def get_test_config(self) -> TestConfig:
        """Get test configuration"""
        return self.config
    
    def get_test_account(self, name: str) -> str:
        """Get test account keypair path"""
        return self.test_accounts.get(name)
    
    def get_mock_data(self, data_type: str) -> any:
        """Get mock data for testing"""
        return self.mock_data.get(data_type)
```

This developer documentation provides comprehensive guidance for configuration, deployment, and testing of the Vault Protocol. The modular configuration system allows for easy customization across different environments while maintaining security and reliability standards.