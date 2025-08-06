#!/usr/bin/env python3
"""
Chainlink Oracle Configuration

This module provides configuration for Chainlink oracle integration,
including price feed addresses, verification intervals, and oracle parameters.
"""

import os
from typing import Dict, List, Optional, Union
from dataclasses import dataclass, field
from enum import Enum
import json

class Network(Enum):
    """Supported blockchain networks"""
    MAINNET = "mainnet"
    DEVNET = "devnet"
    TESTNET = "testnet"
    LOCALNET = "localnet"

class PriceFeedType(Enum):
    """Types of price feeds supported"""
    BTC_USD = "BTC/USD"
    SOL_USD = "SOL/USD"
    ETH_USD = "ETH/USD"
    ATOM_USD = "ATOM/USD"
    USDC_USD = "USDC/USD"

@dataclass
class PriceFeedConfig:
    """Configuration for a single price feed"""
    feed_id: str
    address: str
    decimals: int
    heartbeat: int  # seconds
    deviation_threshold: float  # percentage
    min_responses: int
    max_response_time: int  # seconds
    description: str
    network: Network
    is_active: bool = True

@dataclass
class OracleValidationConfig:
    """Oracle validation parameters"""
    max_price_deviation: float = 5.0  # percentage
    max_staleness: int = 300  # seconds
    min_confidence_level: float = 95.0  # percentage
    retry_attempts: int = 3
    retry_delay: int = 5  # seconds
    circuit_breaker_threshold: int = 5  # consecutive failures
    recovery_time: int = 300  # seconds

@dataclass
class ChainlinkConfig:
    """Main Chainlink configuration"""
    network: Network
    rpc_endpoint: str
    price_feeds: Dict[PriceFeedType, PriceFeedConfig] = field(default_factory=dict)
    validation: OracleValidationConfig = field(default_factory=OracleValidationConfig)
    update_interval: int = 60  # seconds
    batch_size: int = 10
    timeout: int = 30  # seconds
    api_key: Optional[str] = None
    webhook_url: Optional[str] = None
    monitoring_enabled: bool = True
    logging_level: str = "INFO"

# Mainnet Price Feed Configurations
MAINNET_PRICE_FEEDS = {
    PriceFeedType.BTC_USD: PriceFeedConfig(
        feed_id="btc_usd_mainnet",
        address="GVXRSBjFk6e6J3NbVPXohDJetcTjaeeuykUpbQF8UoMU",  # Solana mainnet BTC/USD
        decimals=8,
        heartbeat=3600,  # 1 hour
        deviation_threshold=0.5,
        min_responses=3,
        max_response_time=30,
        description="Bitcoin to USD price feed on Solana mainnet",
        network=Network.MAINNET
    ),
    PriceFeedType.SOL_USD: PriceFeedConfig(
        feed_id="sol_usd_mainnet",
        address="H6ARHf6YXhGYeQfUzQNGk6rDNnLBQKrenN712K4AQJEG",  # Solana mainnet SOL/USD
        decimals=8,
        heartbeat=60,  # 1 minute
        deviation_threshold=0.1,
        min_responses=5,
        max_response_time=15,
        description="Solana to USD price feed on Solana mainnet",
        network=Network.MAINNET
    ),
    PriceFeedType.ETH_USD: PriceFeedConfig(
        feed_id="eth_usd_mainnet",
        address="JBu1AL4obBcCMqKBBxhpWCNUt136ijcuMZLFvTP7iWdB",  # Solana mainnet ETH/USD
        decimals=8,
        heartbeat=3600,  # 1 hour
        deviation_threshold=0.5,
        min_responses=3,
        max_response_time=30,
        description="Ethereum to USD price feed on Solana mainnet",
        network=Network.MAINNET
    ),
    PriceFeedType.USDC_USD: PriceFeedConfig(
        feed_id="usdc_usd_mainnet",
        address="Gnt27xtC473ZT2Mw5u8wZ68Z3gULkSTb5DuxJy7eJotD",  # Solana mainnet USDC/USD
        decimals=8,
        heartbeat=86400,  # 24 hours
        deviation_threshold=0.01,
        min_responses=3,
        max_response_time=30,
        description="USDC to USD price feed on Solana mainnet",
        network=Network.MAINNET
    )
}

# Devnet Price Feed Configurations
DEVNET_PRICE_FEEDS = {
    PriceFeedType.BTC_USD: PriceFeedConfig(
        feed_id="btc_usd_devnet",
        address="HovQMDrbAgAYPCmHVSrezcSmkMtXSSUsLDFANExrZh2J",  # Solana devnet BTC/USD
        decimals=8,
        heartbeat=300,  # 5 minutes
        deviation_threshold=1.0,
        min_responses=1,
        max_response_time=60,
        description="Bitcoin to USD price feed on Solana devnet",
        network=Network.DEVNET
    ),
    PriceFeedType.SOL_USD: PriceFeedConfig(
        feed_id="sol_usd_devnet",
        address="J83w4HKfqxwcq3BEMMkPFSppX3gqekLyLJBexebFVkix",  # Solana devnet SOL/USD
        decimals=8,
        heartbeat=60,  # 1 minute
        deviation_threshold=0.5,
        min_responses=1,
        max_response_time=60,
        description="Solana to USD price feed on Solana devnet",
        network=Network.DEVNET
    ),
    PriceFeedType.ETH_USD: PriceFeedConfig(
        feed_id="eth_usd_devnet",
        address="EdVCmQ9FSPcVe5YySXDPCRmc8aDQLKJ9xvYBMZPie1Vw",  # Solana devnet ETH/USD
        decimals=8,
        heartbeat=300,  # 5 minutes
        deviation_threshold=1.0,
        min_responses=1,
        max_response_time=60,
        description="Ethereum to USD price feed on Solana devnet",
        network=Network.DEVNET
    )
}

# Default configurations for different networks
DEFAULT_CONFIGS = {
    Network.MAINNET: ChainlinkConfig(
        network=Network.MAINNET,
        rpc_endpoint="https://api.mainnet-beta.solana.com",
        price_feeds=MAINNET_PRICE_FEEDS,
        update_interval=60,
        batch_size=5,
        timeout=30,
        monitoring_enabled=True,
        logging_level="INFO"
    ),
    Network.DEVNET: ChainlinkConfig(
        network=Network.DEVNET,
        rpc_endpoint="https://api.devnet.solana.com",
        price_feeds=DEVNET_PRICE_FEEDS,
        update_interval=30,
        batch_size=10,
        timeout=60,
        monitoring_enabled=True,
        logging_level="DEBUG"
    ),
    Network.TESTNET: ChainlinkConfig(
        network=Network.TESTNET,
        rpc_endpoint="https://api.testnet.solana.com",
        price_feeds=DEVNET_PRICE_FEEDS,  # Use devnet feeds for testnet
        update_interval=30,
        batch_size=10,
        timeout=60,
        monitoring_enabled=True,
        logging_level="DEBUG"
    ),
    Network.LOCALNET: ChainlinkConfig(
        network=Network.LOCALNET,
        rpc_endpoint="http://127.0.0.1:8899",
        price_feeds={},  # No real price feeds for localnet
        update_interval=10,
        batch_size=1,
        timeout=10,
        monitoring_enabled=False,
        logging_level="DEBUG"
    )
}

class ChainlinkConfigManager:
    """Manager for Chainlink configuration"""
    
    def __init__(self, network: Network = Network.DEVNET):
        self.network = network
        self.config = DEFAULT_CONFIGS[network].copy() if network in DEFAULT_CONFIGS else DEFAULT_CONFIGS[Network.DEVNET]
        self._load_environment_overrides()
    
    def _load_environment_overrides(self):
        """Load configuration overrides from environment variables"""
        # RPC endpoint override
        rpc_override = os.getenv(f"CHAINLINK_RPC_{self.network.value.upper()}")
        if rpc_override:
            self.config.rpc_endpoint = rpc_override
        
        # API key override
        api_key = os.getenv("CHAINLINK_API_KEY")
        if api_key:
            self.config.api_key = api_key
        
        # Webhook URL override
        webhook_url = os.getenv("CHAINLINK_WEBHOOK_URL")
        if webhook_url:
            self.config.webhook_url = webhook_url
        
        # Update interval override
        update_interval = os.getenv("CHAINLINK_UPDATE_INTERVAL")
        if update_interval:
            try:
                self.config.update_interval = int(update_interval)
            except ValueError:
                pass
        
        # Monitoring toggle
        monitoring = os.getenv("CHAINLINK_MONITORING_ENABLED")
        if monitoring:
            self.config.monitoring_enabled = monitoring.lower() in ("true", "1", "yes")
        
        # Logging level override
        log_level = os.getenv("CHAINLINK_LOG_LEVEL")
        if log_level:
            self.config.logging_level = log_level.upper()
    
    def get_config(self) -> ChainlinkConfig:
        """Get the current configuration"""
        return self.config
    
    def get_price_feed(self, feed_type: PriceFeedType) -> Optional[PriceFeedConfig]:
        """Get configuration for a specific price feed"""
        return self.config.price_feeds.get(feed_type)
    
    def add_price_feed(self, feed_type: PriceFeedType, config: PriceFeedConfig):
        """Add or update a price feed configuration"""
        self.config.price_feeds[feed_type] = config
    
    def remove_price_feed(self, feed_type: PriceFeedType):
        """Remove a price feed configuration"""
        if feed_type in self.config.price_feeds:
            del self.config.price_feeds[feed_type]
    
    def update_validation_config(self, validation_config: OracleValidationConfig):
        """Update oracle validation configuration"""
        self.config.validation = validation_config
    
    def to_dict(self) -> Dict:
        """Convert configuration to dictionary"""
        def convert_enum(obj):
            if isinstance(obj, Enum):
                return obj.value
            elif isinstance(obj, dict):
                return {k: convert_enum(v) for k, v in obj.items()}
            elif isinstance(obj, list):
                return [convert_enum(item) for item in obj]
            elif hasattr(obj, '__dict__'):
                return {k: convert_enum(v) for k, v in obj.__dict__.items()}
            return obj
        
        return convert_enum(self.config.__dict__)
    
    def to_json(self, indent: int = 2) -> str:
        """Convert configuration to JSON string"""
        return json.dumps(self.to_dict(), indent=indent)
    
    def save_to_file(self, filepath: str):
        """Save configuration to JSON file"""
        with open(filepath, 'w') as f:
            f.write(self.to_json())
    
    @classmethod
    def load_from_file(cls, filepath: str) -> 'ChainlinkConfigManager':
        """Load configuration from JSON file"""
        with open(filepath, 'r') as f:
            data = json.load(f)
        
        # Convert back to proper types
        network = Network(data['network'])
        manager = cls(network)
        
        # Update configuration with loaded data
        manager.config.rpc_endpoint = data.get('rpc_endpoint', manager.config.rpc_endpoint)
        manager.config.update_interval = data.get('update_interval', manager.config.update_interval)
        manager.config.batch_size = data.get('batch_size', manager.config.batch_size)
        manager.config.timeout = data.get('timeout', manager.config.timeout)
        manager.config.api_key = data.get('api_key')
        manager.config.webhook_url = data.get('webhook_url')
        manager.config.monitoring_enabled = data.get('monitoring_enabled', manager.config.monitoring_enabled)
        manager.config.logging_level = data.get('logging_level', manager.config.logging_level)
        
        return manager
    
    def validate_config(self) -> List[str]:
        """Validate the current configuration and return any errors"""
        errors = []
        
        # Validate RPC endpoint
        if not self.config.rpc_endpoint:
            errors.append("RPC endpoint is required")
        
        # Validate update interval
        if self.config.update_interval <= 0:
            errors.append("Update interval must be positive")
        
        # Validate batch size
        if self.config.batch_size <= 0:
            errors.append("Batch size must be positive")
        
        # Validate timeout
        if self.config.timeout <= 0:
            errors.append("Timeout must be positive")
        
        # Validate price feeds
        if not self.config.price_feeds:
            errors.append("At least one price feed must be configured")
        
        for feed_type, feed_config in self.config.price_feeds.items():
            if not feed_config.address:
                errors.append(f"Price feed {feed_type.value} missing address")
            if feed_config.decimals < 0:
                errors.append(f"Price feed {feed_type.value} decimals must be non-negative")
            if feed_config.heartbeat <= 0:
                errors.append(f"Price feed {feed_type.value} heartbeat must be positive")
        
        # Validate oracle validation config
        validation = self.config.validation
        if validation.max_price_deviation <= 0:
            errors.append("Max price deviation must be positive")
        if validation.max_staleness <= 0:
            errors.append("Max staleness must be positive")
        if validation.min_confidence_level <= 0 or validation.min_confidence_level > 100:
            errors.append("Min confidence level must be between 0 and 100")
        
        return errors

# Utility functions
def get_default_config(network: Network = Network.DEVNET) -> ChainlinkConfig:
    """Get default configuration for a network"""
    return DEFAULT_CONFIGS.get(network, DEFAULT_CONFIGS[Network.DEVNET])

def create_config_manager(network: Network = Network.DEVNET) -> ChainlinkConfigManager:
    """Create a new configuration manager"""
    return ChainlinkConfigManager(network)

def load_config_from_env() -> ChainlinkConfigManager:
    """Load configuration from environment variables"""
    network_str = os.getenv("CHAINLINK_NETWORK", "devnet")
    try:
        network = Network(network_str.lower())
    except ValueError:
        network = Network.DEVNET
    
    return ChainlinkConfigManager(network)

# Example usage and testing
if __name__ == "__main__":
    # Create configuration manager
    config_manager = create_config_manager(Network.DEVNET)
    
    # Print current configuration
    print("Current Chainlink Configuration:")
    print(config_manager.to_json())
    
    # Validate configuration
    errors = config_manager.validate_config()
    if errors:
        print("\nConfiguration Errors:")
        for error in errors:
            print(f"  - {error}")
    else:
        print("\nConfiguration is valid!")
    
    # Example: Add custom price feed
    custom_feed = PriceFeedConfig(
        feed_id="custom_token_usd",
        address="CustomTokenPriceFeedAddress123456789",
        decimals=8,
        heartbeat=300,
        deviation_threshold=1.0,
        min_responses=1,
        max_response_time=60,
        description="Custom token to USD price feed",
        network=Network.DEVNET
    )
    
    # Add the custom feed (this would be a custom enum value in practice)
    print("\nAdding custom price feed...")
    
    # Save configuration to file
    config_manager.save_to_file("chainlink_config.json")
    print("Configuration saved to chainlink_config.json")
    
    # Load configuration from file
    loaded_manager = ChainlinkConfigManager.load_from_file("chainlink_config.json")
    print("Configuration loaded from file successfully")