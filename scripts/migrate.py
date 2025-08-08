#!/usr/bin/env python3
"""
Vault Protocol Database Migration Script
Handles user data and state migrations across deployments
"""

import json
import os
import sys
import argparse
import logging
from datetime import datetime, timezone
from pathlib import Path
from typing import Dict, List, Optional, Any
import sqlite3
import hashlib

# Add project root to path
PROJECT_ROOT = Path(__file__).parent.parent
sys.path.append(str(PROJECT_ROOT))

# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(levelname)s - %(message)s',
    handlers=[
        logging.FileHandler(PROJECT_ROOT / '.migration.log'),
        logging.StreamHandler()
    ]
)
logger = logging.getLogger(__name__)

class VaultMigration:
    """Handles database migrations for Vault Protocol"""
    
    def __init__(self, network: str = "localnet"):
        self.network = network
        self.db_path = PROJECT_ROOT / f".vault-{network}.db"
        self.backup_dir = PROJECT_ROOT / ".migration-backups"
        self.backup_dir.mkdir(exist_ok=True)
        
        # Load environment configuration
        self.config = self._load_environment_config()
        
        # Initialize database
        self._init_database()
    
    def _load_environment_config(self) -> Dict[str, Any]:
        """Load environment configuration"""
        config_path = PROJECT_ROOT / "config" / "environments.json"
        
        if not config_path.exists():
            logger.error(f"Environment config not found: {config_path}")
            sys.exit(1)
        
        with open(config_path, 'r') as f:
            config = json.load(f)
        
        if self.network not in config:
            logger.error(f"Network '{self.network}' not found in configuration")
            sys.exit(1)
        
        return config[self.network]
    
    def _init_database(self):
        """Initialize SQLite database with required tables"""
        logger.info(f"Initializing database: {self.db_path}")
        
        with sqlite3.connect(self.db_path) as conn:
            cursor = conn.cursor()
            
            # Migration tracking table
            cursor.execute("""
                CREATE TABLE IF NOT EXISTS migrations (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    version TEXT UNIQUE NOT NULL,
                    description TEXT NOT NULL,
                    applied_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                    checksum TEXT NOT NULL
                )
            """)
            
            # User accounts table
            cursor.execute("""
                CREATE TABLE IF NOT EXISTS user_accounts (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    public_key TEXT UNIQUE NOT NULL,
                    btc_address TEXT,
                    btc_commitment REAL DEFAULT 0.0,
                    reward_balance REAL DEFAULT 0.0,
                    kyc_status TEXT DEFAULT 'none',
                    kyc_tier TEXT DEFAULT 'non_kyc',
                    payment_preference TEXT DEFAULT 'btc',
                    auto_reinvest BOOLEAN DEFAULT FALSE,
                    two_fa_enabled BOOLEAN DEFAULT FALSE,
                    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
                )
            """)
            
            # BTC commitments table
            cursor.execute("""
                CREATE TABLE IF NOT EXISTS btc_commitments (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    user_public_key TEXT NOT NULL,
                    btc_address TEXT NOT NULL,
                    amount REAL NOT NULL,
                    ecdsa_proof TEXT NOT NULL,
                    verified BOOLEAN DEFAULT FALSE,
                    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                    verified_at TIMESTAMP,
                    FOREIGN KEY (user_public_key) REFERENCES user_accounts (public_key)
                )
            """)
            
            # Rewards table
            cursor.execute("""
                CREATE TABLE IF NOT EXISTS rewards (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    user_public_key TEXT NOT NULL,
                    amount REAL NOT NULL,
                    currency TEXT NOT NULL,
                    status TEXT DEFAULT 'pending',
                    payment_tx_hash TEXT,
                    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                    paid_at TIMESTAMP,
                    FOREIGN KEY (user_public_key) REFERENCES user_accounts (public_key)
                )
            """)
            
            # Treasury operations table
            cursor.execute("""
                CREATE TABLE IF NOT EXISTS treasury_operations (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    operation_type TEXT NOT NULL,
                    amount_usd REAL NOT NULL,
                    sol_amount REAL,
                    eth_amount REAL,
                    atom_amount REAL,
                    tx_hash TEXT,
                    status TEXT DEFAULT 'pending',
                    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                    completed_at TIMESTAMP
                )
            """)
            
            # KYC records table
            cursor.execute("""
                CREATE TABLE IF NOT EXISTS kyc_records (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    user_public_key TEXT NOT NULL,
                    document_hash TEXT NOT NULL,
                    verification_status TEXT DEFAULT 'pending',
                    chainalysis_score INTEGER,
                    verification_date TIMESTAMP,
                    expiry_date TIMESTAMP,
                    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                    FOREIGN KEY (user_public_key) REFERENCES user_accounts (public_key)
                )
            """)
            
            # Security events table
            cursor.execute("""
                CREATE TABLE IF NOT EXISTS security_events (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    user_public_key TEXT,
                    event_type TEXT NOT NULL,
                    severity TEXT NOT NULL,
                    description TEXT NOT NULL,
                    metadata TEXT,
                    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
                )
            """)
            
            # State channels table
            cursor.execute("""
                CREATE TABLE IF NOT EXISTS state_channels (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    channel_id TEXT UNIQUE NOT NULL,
                    participants TEXT NOT NULL,
                    state_hash TEXT NOT NULL,
                    nonce INTEGER NOT NULL,
                    timeout_at TIMESTAMP,
                    status TEXT DEFAULT 'open',
                    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
                )
            """)
            
            # Create indexes for performance
            cursor.execute("CREATE INDEX IF NOT EXISTS idx_user_accounts_public_key ON user_accounts (public_key)")
            cursor.execute("CREATE INDEX IF NOT EXISTS idx_btc_commitments_user ON btc_commitments (user_public_key)")
            cursor.execute("CREATE INDEX IF NOT EXISTS idx_rewards_user ON rewards (user_public_key)")
            cursor.execute("CREATE INDEX IF NOT EXISTS idx_kyc_records_user ON kyc_records (user_public_key)")
            cursor.execute("CREATE INDEX IF NOT EXISTS idx_security_events_user ON security_events (user_public_key)")
            cursor.execute("CREATE INDEX IF NOT EXISTS idx_state_channels_id ON state_channels (channel_id)")
            
            conn.commit()
            logger.info("Database initialized successfully")
    
    def create_backup(self) -> str:
        """Create a backup of the current database"""
        timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
        backup_name = f"vault-{self.network}-{timestamp}.db"
        backup_path = self.backup_dir / backup_name
        
        if self.db_path.exists():
            import shutil
            shutil.copy2(self.db_path, backup_path)
            logger.info(f"Database backup created: {backup_path}")
        else:
            logger.warning("No existing database to backup")
        
        return str(backup_path)
    
    def get_migration_version(self) -> str:
        """Get current migration version"""
        with sqlite3.connect(self.db_path) as conn:
            cursor = conn.cursor()
            cursor.execute("SELECT version FROM migrations ORDER BY applied_at DESC LIMIT 1")
            result = cursor.fetchone()
            return result[0] if result else "0.0.0"
    
    def apply_migration(self, version: str, description: str, sql_commands: List[str]) -> bool:
        """Apply a database migration"""
        logger.info(f"Applying migration {version}: {description}")
        
        # Calculate checksum
        checksum = hashlib.sha256(
            (version + description + "".join(sql_commands)).encode()
        ).hexdigest()
        
        try:
            with sqlite3.connect(self.db_path) as conn:
                cursor = conn.cursor()
                
                # Check if migration already applied
                cursor.execute("SELECT id FROM migrations WHERE version = ?", (version,))
                if cursor.fetchone():
                    logger.warning(f"Migration {version} already applied")
                    return True
                
                # Apply migration commands
                for command in sql_commands:
                    cursor.execute(command)
                
                # Record migration
                cursor.execute("""
                    INSERT INTO migrations (version, description, checksum)
                    VALUES (?, ?, ?)
                """, (version, description, checksum))
                
                conn.commit()
                logger.info(f"Migration {version} applied successfully")
                return True
                
        except Exception as e:
            logger.error(f"Failed to apply migration {version}: {e}")
            return False
    
    def migrate_user_data(self, source_network: str) -> bool:
        """Migrate user data from another network"""
        logger.info(f"Migrating user data from {source_network} to {self.network}")
        
        source_db = PROJECT_ROOT / f".vault-{source_network}.db"
        if not source_db.exists():
            logger.error(f"Source database not found: {source_db}")
            return False
        
        try:
            # Connect to both databases
            with sqlite3.connect(source_db) as source_conn, \
                 sqlite3.connect(self.db_path) as dest_conn:
                
                source_cursor = source_conn.cursor()
                dest_cursor = dest_conn.cursor()
                
                # Migrate user accounts
                source_cursor.execute("SELECT * FROM user_accounts")
                users = source_cursor.fetchall()
                
                for user in users:
                    dest_cursor.execute("""
                        INSERT OR REPLACE INTO user_accounts 
                        (public_key, btc_address, btc_commitment, reward_balance, 
                         kyc_status, kyc_tier, payment_preference, auto_reinvest, 
                         two_fa_enabled, created_at, updated_at)
                        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                    """, user[1:])  # Skip ID column
                
                # Migrate BTC commitments
                source_cursor.execute("SELECT * FROM btc_commitments")
                commitments = source_cursor.fetchall()
                
                for commitment in commitments:
                    dest_cursor.execute("""
                        INSERT OR REPLACE INTO btc_commitments
                        (user_public_key, btc_address, amount, ecdsa_proof, 
                         verified, created_at, verified_at)
                        VALUES (?, ?, ?, ?, ?, ?, ?)
                    """, commitment[1:])  # Skip ID column
                
                # Migrate KYC records
                source_cursor.execute("SELECT * FROM kyc_records")
                kyc_records = source_cursor.fetchall()
                
                for record in kyc_records:
                    dest_cursor.execute("""
                        INSERT OR REPLACE INTO kyc_records
                        (user_public_key, document_hash, verification_status,
                         chainalysis_score, verification_date, expiry_date, created_at)
                        VALUES (?, ?, ?, ?, ?, ?, ?)
                    """, record[1:])  # Skip ID column
                
                dest_conn.commit()
                logger.info(f"Successfully migrated {len(users)} users from {source_network}")
                return True
                
        except Exception as e:
            logger.error(f"Failed to migrate user data: {e}")
            return False
    
    def export_user_data(self, output_file: str) -> bool:
        """Export user data to JSON file"""
        logger.info(f"Exporting user data to {output_file}")
        
        try:
            with sqlite3.connect(self.db_path) as conn:
                conn.row_factory = sqlite3.Row
                cursor = conn.cursor()
                
                export_data = {
                    "export_timestamp": datetime.now(timezone.utc).isoformat(),
                    "network": self.network,
                    "users": [],
                    "commitments": [],
                    "rewards": [],
                    "kyc_records": []
                }
                
                # Export users
                cursor.execute("SELECT * FROM user_accounts")
                for row in cursor.fetchall():
                    export_data["users"].append(dict(row))
                
                # Export commitments
                cursor.execute("SELECT * FROM btc_commitments")
                for row in cursor.fetchall():
                    export_data["commitments"].append(dict(row))
                
                # Export rewards
                cursor.execute("SELECT * FROM rewards")
                for row in cursor.fetchall():
                    export_data["rewards"].append(dict(row))
                
                # Export KYC records (anonymized)
                cursor.execute("""
                    SELECT user_public_key, verification_status, chainalysis_score,
                           verification_date, expiry_date, created_at
                    FROM kyc_records
                """)
                for row in cursor.fetchall():
                    export_data["kyc_records"].append(dict(row))
                
                # Write to file
                with open(output_file, 'w') as f:
                    json.dump(export_data, f, indent=2, default=str)
                
                logger.info(f"User data exported successfully to {output_file}")
                return True
                
        except Exception as e:
            logger.error(f"Failed to export user data: {e}")
            return False
    
    def import_user_data(self, input_file: str) -> bool:
        """Import user data from JSON file"""
        logger.info(f"Importing user data from {input_file}")
        
        try:
            with open(input_file, 'r') as f:
                import_data = json.load(f)
            
            with sqlite3.connect(self.db_path) as conn:
                cursor = conn.cursor()
                
                # Import users
                for user in import_data.get("users", []):
                    cursor.execute("""
                        INSERT OR REPLACE INTO user_accounts 
                        (public_key, btc_address, btc_commitment, reward_balance, 
                         kyc_status, kyc_tier, payment_preference, auto_reinvest, 
                         two_fa_enabled, created_at, updated_at)
                        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                    """, (
                        user["public_key"], user["btc_address"], user["btc_commitment"],
                        user["reward_balance"], user["kyc_status"], user["kyc_tier"],
                        user["payment_preference"], user["auto_reinvest"], 
                        user["two_fa_enabled"], user["created_at"], user["updated_at"]
                    ))
                
                # Import commitments
                for commitment in import_data.get("commitments", []):
                    cursor.execute("""
                        INSERT OR REPLACE INTO btc_commitments
                        (user_public_key, btc_address, amount, ecdsa_proof, 
                         verified, created_at, verified_at)
                        VALUES (?, ?, ?, ?, ?, ?, ?)
                    """, (
                        commitment["user_public_key"], commitment["btc_address"],
                        commitment["amount"], commitment["ecdsa_proof"],
                        commitment["verified"], commitment["created_at"],
                        commitment.get("verified_at")
                    ))
                
                # Import rewards
                for reward in import_data.get("rewards", []):
                    cursor.execute("""
                        INSERT OR REPLACE INTO rewards
                        (user_public_key, amount, currency, status, 
                         payment_tx_hash, created_at, paid_at)
                        VALUES (?, ?, ?, ?, ?, ?, ?)
                    """, (
                        reward["user_public_key"], reward["amount"], reward["currency"],
                        reward["status"], reward.get("payment_tx_hash"),
                        reward["created_at"], reward.get("paid_at")
                    ))
                
                conn.commit()
                logger.info(f"Successfully imported data for {len(import_data.get('users', []))} users")
                return True
                
        except Exception as e:
            logger.error(f"Failed to import user data: {e}")
            return False
    
    def validate_data_integrity(self) -> bool:
        """Validate database integrity"""
        logger.info("Validating database integrity")
        
        try:
            with sqlite3.connect(self.db_path) as conn:
                cursor = conn.cursor()
                
                # Check for orphaned records
                cursor.execute("""
                    SELECT COUNT(*) FROM btc_commitments 
                    WHERE user_public_key NOT IN (SELECT public_key FROM user_accounts)
                """)
                orphaned_commitments = cursor.fetchone()[0]
                
                cursor.execute("""
                    SELECT COUNT(*) FROM rewards 
                    WHERE user_public_key NOT IN (SELECT public_key FROM user_accounts)
                """)
                orphaned_rewards = cursor.fetchone()[0]
                
                cursor.execute("""
                    SELECT COUNT(*) FROM kyc_records 
                    WHERE user_public_key NOT IN (SELECT public_key FROM user_accounts)
                """)
                orphaned_kyc = cursor.fetchone()[0]
                
                # Check data consistency
                cursor.execute("""
                    SELECT COUNT(*) FROM user_accounts 
                    WHERE btc_commitment < 0 OR reward_balance < 0
                """)
                negative_balances = cursor.fetchone()[0]
                
                # Report results
                issues = []
                if orphaned_commitments > 0:
                    issues.append(f"{orphaned_commitments} orphaned BTC commitments")
                if orphaned_rewards > 0:
                    issues.append(f"{orphaned_rewards} orphaned rewards")
                if orphaned_kyc > 0:
                    issues.append(f"{orphaned_kyc} orphaned KYC records")
                if negative_balances > 0:
                    issues.append(f"{negative_balances} accounts with negative balances")
                
                if issues:
                    logger.warning(f"Data integrity issues found: {', '.join(issues)}")
                    return False
                else:
                    logger.info("Database integrity validation passed")
                    return True
                    
        except Exception as e:
            logger.error(f"Failed to validate data integrity: {e}")
            return False

def main():
    parser = argparse.ArgumentParser(description="Vault Protocol Database Migration Tool")
    parser.add_argument("--network", default="localnet", 
                       choices=["localnet", "devnet", "testnet", "mainnet"],
                       help="Target network")
    
    subparsers = parser.add_subparsers(dest="command", help="Available commands")
    
    # Init command
    subparsers.add_parser("init", help="Initialize database")
    
    # Backup command
    subparsers.add_parser("backup", help="Create database backup")
    
    # Migrate command
    migrate_parser = subparsers.add_parser("migrate", help="Migrate data between networks")
    migrate_parser.add_argument("--from", dest="source_network", required=True,
                               help="Source network to migrate from")
    
    # Export command
    export_parser = subparsers.add_parser("export", help="Export user data")
    export_parser.add_argument("--output", required=True, help="Output file path")
    
    # Import command
    import_parser = subparsers.add_parser("import", help="Import user data")
    import_parser.add_argument("--input", required=True, help="Input file path")
    
    # Validate command
    subparsers.add_parser("validate", help="Validate database integrity")
    
    # Version command
    subparsers.add_parser("version", help="Show current migration version")
    
    args = parser.parse_args()
    
    if not args.command:
        parser.print_help()
        return
    
    migration = VaultMigration(args.network)
    
    if args.command == "init":
        logger.info("Database initialization completed")
    
    elif args.command == "backup":
        backup_path = migration.create_backup()
        print(f"Backup created: {backup_path}")
    
    elif args.command == "migrate":
        success = migration.migrate_user_data(args.source_network)
        sys.exit(0 if success else 1)
    
    elif args.command == "export":
        success = migration.export_user_data(args.output)
        sys.exit(0 if success else 1)
    
    elif args.command == "import":
        success = migration.import_user_data(args.input)
        sys.exit(0 if success else 1)
    
    elif args.command == "validate":
        success = migration.validate_data_integrity()
        sys.exit(0 if success else 1)
    
    elif args.command == "version":
        version = migration.get_migration_version()
        print(f"Current migration version: {version}")

if __name__ == "__main__":
    main()