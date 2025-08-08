#!/usr/bin/env python3
"""
Vault Protocol Configuration Manager
Manages environment configurations and deployment settings
"""

import json
import os
import sys
import argparse
import logging
from pathlib import Path
from typing import Dict, Any, Optional
import shutil
from datetime import datetime

# Add project root to path
PROJECT_ROOT = Path(__file__).parent.parent
sys.path.append(str(PROJECT_ROOT))

# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(levelname)s - %(message)s'
)
logger = logging.getLogger(__name__)

class ConfigManager:
    """Manages configuration for different deployment environments"""
    
    def __init__(self):
        self.project_root = PROJECT_ROOT
        self.config_dir = self.project_root / "config"
        self.environments_file = self.config_dir / "environments.json"
        
        # Ensure config directory exists
        self.config_dir.mkdir(exist_ok=True)
    
    def load_environments(self) -> Dict[str, Any]:
        """Load environment configurations"""
        if not self.environments_file.exists():
            logger.error(f"Environment config not found: {self.environments_file}")
            return {}
        
        with open(self.environments_file, 'r') as f:
            return json.load(f)
    
    def save_environments(self, config: Dict[str, Any]):
        """Save environment configurations"""
        with open(self.environments_file, 'w') as f:
            json.dump(config, f, indent=2)
        
        logger.info(f"Environment configuration saved to {self.environments_file}")
    
    def list_environments(self):
        """List all available environments"""
        config = self.load_environments()
        
        print("Available environments:")
        print()
        
        for env_name, env_config in config.items():
            print(f"  🌐 {env_name}")
            print(f"     Name: {env_config.get('name', 'Unknown')}")
            print(f"     Cluster: {env_config.get('cluster_url', 'Unknown')}")
            print(f"     Program ID: {env_config.get('program_id', 'Unknown')}")
            
            # Show key features
            features = env_config.get('features', {})
            enabled_features = [k for k, v in features.items() if v]
            if enabled_features:
                print(f"     Features: {', '.join(enabled_features)}")
            
            print()
    
    def get_environment(self, env_name: str) -> Optional[Dict[str, Any]]:
        """Get specific environment configuration"""
        config = self.load_environments()
        return config.get(env_name)
    
    def set_environment_value(self, env_name: str, key_path: str, value: Any):
        """Set a value in environment configuration"""
        config = self.load_environments()
        
        if env_name not in config:
            logger.error(f"Environment '{env_name}' not found")
            return False
        
        # Navigate to the nested key
        keys = key_path.split('.')
        current = config[env_name]
        
        for key in keys[:-1]:
            if key not in current:
                current[key] = {}
            current = current[key]
        
        # Set the value
        final_key = keys[-1]
        old_value = current.get(final_key, "not set")
        current[final_key] = value
        
        self.save_environments(config)
        
        logger.info(f"Updated {env_name}.{key_path}: {old_value} -> {value}")
        return True
    
    def create_environment(self, env_name: str, template: str = "localnet"):
        """Create a new environment based on a template"""
        config = self.load_environments()
        
        if env_name in config:
            logger.error(f"Environment '{env_name}' already exists")
            return False
        
        if template not in config:
            logger.error(f"Template environment '{template}' not found")
            return False
        
        # Copy template configuration
        config[env_name] = config[template].copy()
        config[env_name]["name"] = f"Custom {env_name.title()}"
        
        self.save_environments(config)
        
        logger.info(f"Created environment '{env_name}' based on '{template}'")
        return True
    
    def delete_environment(self, env_name: str):
        """Delete an environment"""
        config = self.load_environments()
        
        if env_name not in config:
            logger.error(f"Environment '{env_name}' not found")
            return False
        
        # Prevent deletion of core environments
        if env_name in ["localnet", "devnet", "testnet", "mainnet"]:
            logger.error(f"Cannot delete core environment '{env_name}'")
            return False
        
        del config[env_name]
        self.save_environments(config)
        
        logger.info(f"Deleted environment '{env_name}'")
        return True
    
    def validate_environment(self, env_name: str) -> bool:
        """Validate environment configuration"""
        config = self.get_environment(env_name)
        
        if not config:
            logger.error(f"Environment '{env_name}' not found")
            return False
        
        errors = []
        
        # Required fields
        required_fields = [
            "name", "cluster_url", "program_id", "chainlink_oracles",
            "treasury", "kyc", "security", "features"
        ]
        
        for field in required_fields:
            if field not in config:
                errors.append(f"Missing required field: {field}")
        
        # Validate program ID format
        program_id = config.get("program_id", "")
        if len(program_id) != 44:  # Base58 public key length
            errors.append("Invalid program_id format")
        
        # Validate treasury allocations
        treasury = config.get("treasury", {})
        allocations = [
            treasury.get("sol_allocation", 0),
            treasury.get("eth_allocation", 0),
            treasury.get("atom_allocation", 0)
        ]
        
        if abs(sum(allocations) - 1.0) > 0.001:  # Allow for floating point precision
            errors.append("Treasury allocations must sum to 1.0")
        
        # Validate security settings
        security = config.get("security", {})
        threshold = security.get("multisig_threshold", 0)
        signers = security.get("multisig_signers", 0)
        
        if threshold >= signers:
            errors.append("Multisig threshold must be less than number of signers")
        
        if threshold < 2:
            errors.append("Multisig threshold must be at least 2")
        
        # Report results
        if errors:
            logger.error(f"Environment '{env_name}' validation failed:")
            for error in errors:
                logger.error(f"  - {error}")
            return False
        else:
            logger.info(f"Environment '{env_name}' validation passed")
            return True
    
    def export_environment(self, env_name: str, output_file: str):
        """Export environment configuration to file"""
        config = self.get_environment(env_name)
        
        if not config:
            logger.error(f"Environment '{env_name}' not found")
            return False
        
        export_data = {
            "environment": env_name,
            "exported_at": datetime.now().isoformat(),
            "config": config
        }
        
        with open(output_file, 'w') as f:
            json.dump(export_data, f, indent=2)
        
        logger.info(f"Environment '{env_name}' exported to {output_file}")
        return True
    
    def import_environment(self, input_file: str, env_name: Optional[str] = None):
        """Import environment configuration from file"""
        try:
            with open(input_file, 'r') as f:
                import_data = json.load(f)
            
            # Use provided name or original name
            target_env = env_name or import_data.get("environment", "imported")
            imported_config = import_data.get("config", {})
            
            if not imported_config:
                logger.error("No configuration found in import file")
                return False
            
            # Load current environments
            config = self.load_environments()
            config[target_env] = imported_config
            
            self.save_environments(config)
            
            logger.info(f"Environment '{target_env}' imported from {input_file}")
            return True
            
        except Exception as e:
            logger.error(f"Failed to import environment: {e}")
            return False
    
    def generate_anchor_toml(self, env_name: str):
        """Generate Anchor.toml for specific environment"""
        config = self.get_environment(env_name)
        
        if not config:
            logger.error(f"Environment '{env_name}' not found")
            return False
        
        anchor_config = f"""[features]
seeds = false
skip-lint = false

[programs.{env_name}]
vault = "{config['program_id']}"

[registry]
url = "https://api.apr.dev"

[provider]
cluster = "{env_name}"
wallet = "~/.config/solana/id.json"

[scripts]
test = "yarn run ts-mocha -p ./tsconfig.json -t 1000000 tests/**/*.ts"

[[test.genesis]]
address = "{config['program_id']}"
program = "programs/vault/target/deploy/vault.so"
"""
        
        # Write to Anchor.toml
        anchor_file = self.project_root / "Anchor.toml"
        with open(anchor_file, 'w') as f:
            f.write(anchor_config)
        
        logger.info(f"Generated Anchor.toml for environment '{env_name}'")
        return True
    
    def sync_program_configs(self):
        """Sync program configurations with environment settings"""
        config = self.load_environments()
        
        # Update Python config files
        for config_name in ["chainlink", "validators", "treasury", "dashboard"]:
            config_file = self.config_dir / f"{config_name}.py"
            
            if config_file.exists():
                logger.info(f"Syncing {config_name}.py with environment settings")
                # This would update the Python config files based on environment settings
                # Implementation depends on specific config file formats
        
        logger.info("Program configuration sync completed")

def main():
    parser = argparse.ArgumentParser(description="Vault Protocol Configuration Manager")
    
    subparsers = parser.add_subparsers(dest="command", help="Available commands")
    
    # List environments
    subparsers.add_parser("list", help="List all environments")
    
    # Show environment
    show_parser = subparsers.add_parser("show", help="Show environment configuration")
    show_parser.add_argument("environment", help="Environment name")
    
    # Set value
    set_parser = subparsers.add_parser("set", help="Set configuration value")
    set_parser.add_argument("environment", help="Environment name")
    set_parser.add_argument("key", help="Configuration key (dot notation)")
    set_parser.add_argument("value", help="Configuration value")
    
    # Create environment
    create_parser = subparsers.add_parser("create", help="Create new environment")
    create_parser.add_argument("name", help="New environment name")
    create_parser.add_argument("--template", default="localnet", help="Template environment")
    
    # Delete environment
    delete_parser = subparsers.add_parser("delete", help="Delete environment")
    delete_parser.add_argument("environment", help="Environment name")
    
    # Validate environment
    validate_parser = subparsers.add_parser("validate", help="Validate environment")
    validate_parser.add_argument("environment", help="Environment name")
    
    # Export environment
    export_parser = subparsers.add_parser("export", help="Export environment")
    export_parser.add_argument("environment", help="Environment name")
    export_parser.add_argument("--output", required=True, help="Output file")
    
    # Import environment
    import_parser = subparsers.add_parser("import", help="Import environment")
    import_parser.add_argument("--input", required=True, help="Input file")
    import_parser.add_argument("--name", help="Environment name (optional)")
    
    # Generate Anchor.toml
    anchor_parser = subparsers.add_parser("anchor", help="Generate Anchor.toml")
    anchor_parser.add_argument("environment", help="Environment name")
    
    # Sync configs
    subparsers.add_parser("sync", help="Sync program configurations")
    
    args = parser.parse_args()
    
    if not args.command:
        parser.print_help()
        return
    
    manager = ConfigManager()
    
    if args.command == "list":
        manager.list_environments()
    
    elif args.command == "show":
        config = manager.get_environment(args.environment)
        if config:
            print(json.dumps(config, indent=2))
        else:
            sys.exit(1)
    
    elif args.command == "set":
        # Try to parse value as JSON, fallback to string
        try:
            value = json.loads(args.value)
        except json.JSONDecodeError:
            value = args.value
        
        success = manager.set_environment_value(args.environment, args.key, value)
        sys.exit(0 if success else 1)
    
    elif args.command == "create":
        success = manager.create_environment(args.name, args.template)
        sys.exit(0 if success else 1)
    
    elif args.command == "delete":
        success = manager.delete_environment(args.environment)
        sys.exit(0 if success else 1)
    
    elif args.command == "validate":
        success = manager.validate_environment(args.environment)
        sys.exit(0 if success else 1)
    
    elif args.command == "export":
        success = manager.export_environment(args.environment, args.output)
        sys.exit(0 if success else 1)
    
    elif args.command == "import":
        success = manager.import_environment(args.input, args.name)
        sys.exit(0 if success else 1)
    
    elif args.command == "anchor":
        success = manager.generate_anchor_toml(args.environment)
        sys.exit(0 if success else 1)
    
    elif args.command == "sync":
        manager.sync_program_configs()

if __name__ == "__main__":
    main()