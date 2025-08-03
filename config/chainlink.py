"""
Chainlink Oracle Configuration for Vault Protocol

This configuration file contains Chainlink oracle feed addresses,
verification intervals, and retry logic parameters for BTC balance
verification and price feeds.
"""

import json
from typing import Dict, List, Optional
from dataclasses import dataclass, asdict
from enum import Enum

class Network(Enum):
    """Supported networks for Chainlink oracles"""
    MAINNET = "mainnet"
    DEVNET = "devnet"
    TESTNET = "testnet"

@dataclass
class ChainlinkFeed:
    """Chainlink price feed configuration"""
    name: str
    address: str
    decimals: int
    heartbeat: int  # seconds
    deviation_threshold: float  # percentage
    description: str

@dataclass
class RetryConfig:
    """Retry configuration for oracle failures"""
    max_retries: int = 3
    base_delay: int = 2  # seconds
    max_delay: int = 60  # seconds
    exponential_backoff: bool = True
    jitter: bool = True  # add randomness to prevent thundering herd

@dataclass
class OracleConfig:
    """Main oracle configuration"""
    network: Network
    retry_config: RetryConfig
    feeds: Dict[str, ChainlinkFeed]
    utxo_verification: Dict[str, any]
    verification_interval: int = 60  # seconds
    cache_duration: int = 300  # 5 minutes
    price_deviation_alert: float = 10.0  # 10% price change alert
    stale_data_threshold: int = 300  # 5 minutes

# Chainlink BTC/USD Price Feed Addresses
CHAINLINK_FEEDS = {
    Network.MAINNET: {
        "BTC_USD": ChainlinkFeed(
            name="BTC/USD",
            address="0xF4030086522a5bEEa4988F8cA5B36dbC97BeE88c",  # Ethereum mainnet
            decimals=8,
            heartbeat=3600,  # 1 hour
            deviation_threshold=0.5,  # 0.5%
            description="Bitcoin to USD price feed"
        ),
        "BTC_USD_SOLANA": ChainlinkFeed(
            name="BTC/USD",
            address="GVXRSBjFk6e6J3NbVPXohDJetcTjaeeuykUpbQF8UoMU",  # Solana mainnet
            decimals=8,
            heartbeat=60,  # 1 minute
            deviation_threshold=0.5,
            description="Bitcoin to USD price feed on Solana"
        )
    },
    Network.DEVNET: {
        "BTC_USD": ChainlinkFeed(
            name="BTC/USD",
            address="HovQMDrbAgAYPCmHVSrezcSmkMtXSSUsLDFANExrZh2J",  # Solana devnet
            decimals=8,
            heartbeat=60,
            deviation_threshold=1.0,  # More lenient for testing
            description="Bitcoin to USD price feed on Solana devnet"
        )
    },
    Network.TESTNET: {
        "BTC_USD": ChainlinkFeed(
            name="BTC/USD",
            address="2iceXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",  # Placeholder
            decimals=8,
            heartbeat=300,  # 5 minutes
            deviation_threshold=2.0,  # More lenient for testing
            description="Bitcoin to USD price feed on testnet"
        )
    }
}

# UTXO Verification Configuration
UTXO_VERIFICATION_CONFIG = {
    "providers": [
        {
            "name": "chainlink_utxo",
            "endpoint": "https://api.chainlink.com/utxo/verify",
            "api_key_required": True,
            "rate_limit": 100,  # requests per minute
            "timeout": 30,  # seconds
            "priority": 1
        },
        {
            "name": "blockstream",
            "endpoint": "https://blockstream.info/api",
            "api_key_required": False,
            "rate_limit": 60,
            "timeout": 15,
            "priority": 2
        },
        {
            "name": "blockchain_info",
            "endpoint": "https://blockchain.info/rawaddr",
            "api_key_required": False,
            "rate_limit": 300,
            "timeout": 10,
            "priority": 3
        }
    ],
    "verification_requirements": {
        "min_confirmations": 1,
        "max_age_hours": 24,
        "require_ecdsa_proof": True,
        "anti_spoofing_enabled": True
    }
}

# Default configurations for each network
DEFAULT_CONFIGS = {
    Network.MAINNET: OracleConfig(
        network=Network.MAINNET,
        verification_interval=60,
        cache_duration=300,
        price_deviation_alert=5.0,  # Stricter for mainnet
        stale_data_threshold=300,
        retry_config=RetryConfig(
            max_retries=3,
            base_delay=2,
            max_delay=60,
            exponential_backoff=True,
            jitter=True
        ),
        feeds=CHAINLINK_FEEDS[Network.MAINNET],
        utxo_verification=UTXO_VERIFICATION_CONFIG
    ),
    Network.DEVNET: OracleConfig(
        network=Network.DEVNET,
        verification_interval=30,  # More frequent for testing
        cache_duration=180,  # Shorter cache for testing
        price_deviation_alert=10.0,  # More lenient for testing
        stale_data_threshold=180,
        retry_config=RetryConfig(
            max_retries=5,  # More retries for unstable devnet
            base_delay=1,
            max_delay=30,
            exponential_backoff=True,
            jitter=False  # Disable jitter for predictable testing
        ),
        feeds=CHAINLINK_FEEDS[Network.DEVNET],
        utxo_verification=UTXO_VERIFICATION_CONFIG
    ),
    Network.TESTNET: OracleConfig(
        network=Network.TESTNET,
        verification_interval=120,  # Less frequent for testnet
        cache_duration=600,  # Longer cache for testnet
        price_deviation_alert=15.0,  # Very lenient for testnet
        stale_data_threshold=600,
        retry_config=RetryConfig(
            max_retries=2,
            base_delay=5,
            max_delay=120,
            exponential_backoff=True,
            jitter=True
        ),
        feeds=CHAINLINK_FEEDS[Network.TESTNET],
        utxo_verification=UTXO_VERIFICATION_CONFIG
    )
}

class ChainlinkConfig:
    """Chainlink configuration manager"""
    
    def __init__(self, network: Network = Network.DEVNET):
        self.network = network
        self.config = DEFAULT_CONFIGS[network]
    
    def get_btc_usd_feed(self) -> ChainlinkFeed:
        """Get BTC/USD price feed configuration"""
        if "BTC_USD_SOLANA" in self.config.feeds:
            return self.config.feeds["BTC_USD_SOLANA"]
        return self.config.feeds["BTC_USD"]
    
    def get_verification_interval(self) -> int:
        """Get oracle verification interval in seconds"""
        return self.config.verification_interval
    
    def get_cache_duration(self) -> int:
        """Get cache duration in seconds"""
        return self.config.cache_duration
    
    def get_retry_config(self) -> RetryConfig:
        """Get retry configuration"""
        return self.config.retry_config
    
    def get_utxo_providers(self) -> List[Dict]:
        """Get UTXO verification providers"""
        return self.config.utxo_verification["providers"]
    
    def should_alert_price_deviation(self, old_price: float, new_price: float) -> bool:
        """Check if price deviation exceeds alert threshold"""
        if old_price == 0:
            return False
        
        deviation = abs(new_price - old_price) / old_price * 100
        return deviation > self.config.price_deviation_alert
    
    def is_data_stale(self, timestamp: int, current_time: int) -> bool:
        """Check if oracle data is stale"""
        age = current_time - timestamp
        return age > self.config.stale_data_threshold
    
    def calculate_retry_delay(self, attempt: int) -> int:
        """Calculate retry delay with exponential backoff"""
        retry_config = self.config.retry_config
        
        if not retry_config.exponential_backoff:
            return retry_config.base_delay
        
        delay = retry_config.base_delay * (2 ** attempt)
        delay = min(delay, retry_config.max_delay)
        
        if retry_config.jitter:
            import random
            jitter = random.uniform(0.5, 1.5)
            delay = int(delay * jitter)
        
        return delay
    
    def to_dict(self) -> Dict:
        """Convert configuration to dictionary"""
        return asdict(self.config)
    
    def to_json(self) -> str:
        """Convert configuration to JSON string"""
        return json.dumps(self.to_dict(), indent=2, default=str)
    
    def save_to_file(self, filename: str):
        """Save configuration to JSON file"""
        with open(filename, 'w') as f:
            f.write(self.to_json())
    
    @classmethod
    def load_from_file(cls, filename: str) -> 'ChainlinkConfig':
        """Load configuration from JSON file"""
        with open(filename, 'r') as f:
            data = json.load(f)
        
        # Reconstruct the configuration
        network = Network(data['network'])
        config = cls(network)
        
        # Override with loaded data
        config.config.verification_interval = data.get('verification_interval', 60)
        config.config.cache_duration = data.get('cache_duration', 300)
        config.config.price_deviation_alert = data.get('price_deviation_alert', 10.0)
        
        return config

# Utility functions for easy access
def get_config(network: str = "devnet") -> ChainlinkConfig:
    """Get Chainlink configuration for specified network"""
    network_enum = Network(network.lower())
    return ChainlinkConfig(network_enum)

def get_btc_feed_address(network: str = "devnet") -> str:
    """Get BTC/USD feed address for specified network"""
    config = get_config(network)
    return config.get_btc_usd_feed().address

def get_verification_settings(network: str = "devnet") -> Dict:
    """Get verification settings for specified network"""
    config = get_config(network)
    return {
        "interval": config.get_verification_interval(),
        "cache_duration": config.get_cache_duration(),
        "retry_config": asdict(config.get_retry_config())
    }

# Example usage and testing
if __name__ == "__main__":
    # Example: Create configuration for devnet
    config = ChainlinkConfig(Network.DEVNET)
    
    print("=== Chainlink Oracle Configuration ===")
    print(f"Network: {config.network.value}")
    print(f"BTC/USD Feed: {config.get_btc_usd_feed().address}")
    print(f"Verification Interval: {config.get_verification_interval()}s")
    print(f"Cache Duration: {config.get_cache_duration()}s")
    print(f"Retry Config: {config.get_retry_config()}")
    
    # Test price deviation alert
    old_price = 50000.0
    new_price = 55000.0
    should_alert = config.should_alert_price_deviation(old_price, new_price)
    print(f"Price change alert (${old_price} -> ${new_price}): {should_alert}")
    
    # Test retry delay calculation
    for attempt in range(4):
        delay = config.calculate_retry_delay(attempt)
        print(f"Retry attempt {attempt}: {delay}s delay")
    
    # Save configuration to file
    config.save_to_file("chainlink_config.json")
    print("\nConfiguration saved to chainlink_config.json")