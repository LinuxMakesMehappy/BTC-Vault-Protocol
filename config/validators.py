#!/usr/bin/env python3
"""
Validator Configuration System

This module provides configuration for staking validator selection and parameters
across different blockchain networks (Solana, Ethereum, Cosmos).
"""

import os
from typing import Dict, List, Optional, Union
from dataclasses import dataclass, field
from enum import Enum
import json
from datetime import datetime, timedelta

class Network(Enum):
    """Supported blockchain networks"""
    SOLANA = "solana"
    ETHEREUM = "ethereum"
    COSMOS = "cosmos"

class ValidatorStatus(Enum):
    """Validator status options"""
    ACTIVE = "active"
    INACTIVE = "inactive"
    JAILED = "jailed"
    DELINQUENT = "delinquent"
    BLACKLISTED = "blacklisted"

class StakingStrategy(Enum):
    """Staking strategy types"""
    PERFORMANCE_BASED = "performance_based"
    DIVERSIFIED = "diversified"
    LOW_COMMISSION = "low_commission"
    HIGH_RELIABILITY = "high_reliability"
    CUSTOM = "custom"

@dataclass
class ValidatorMetrics:
    """Performance metrics for a validator"""
    uptime_percentage: float = 0.0
    commission_rate: float = 0.0
    apy: float = 0.0
    total_stake: int = 0
    delegator_count: int = 0
    skip_rate: float = 0.0
    vote_credits: int = 0
    epoch_credits: int = 0
    last_vote: Optional[int] = None
    delinquent_slots: int = 0
    performance_score: float = 0.0
    reliability_score: float = 0.0
    last_updated: Optional[str] = None

@dataclass
class ValidatorConfig:
    """Configuration for a single validator"""
    name: str
    address: str
    network: Network
    commission_rate: float
    status: ValidatorStatus = ValidatorStatus.ACTIVE
    metrics: ValidatorMetrics = field(default_factory=ValidatorMetrics)
    min_stake: int = 0
    max_stake: int = 0
    priority: int = 1  # 1 = highest priority
    allocation_percentage: float = 0.0
    auto_compound: bool = True
    monitoring_enabled: bool = True
    description: str = ""
    website: Optional[str] = None
    keybase_id: Optional[str] = None
    details: Optional[str] = None
    is_preferred: bool = False
    blacklisted: bool = False
    created_at: Optional[str] = None
    updated_at: Optional[str] = None

@dataclass
class NetworkStakingConfig:
    """Staking configuration for a specific network"""
    network: Network
    total_allocation_percentage: float
    strategy: StakingStrategy
    validators: List[ValidatorConfig] = field(default_factory=list)
    min_validators: int = 3
    max_validators: int = 10
    rebalance_threshold: float = 5.0  # percentage
    rebalance_frequency: int = 86400  # seconds (24 hours)
    min_stake_per_validator: int = 1000000  # minimum stake in native units
    max_stake_per_validator: int = 100000000  # maximum stake in native units
    emergency_unstake_enabled: bool = True
    auto_restake_enabled: bool = True
    performance_monitoring: bool = True
    slash_protection: bool = True

@dataclass
class StakingPoolConfig:
    """Overall staking pool configuration"""
    networks: Dict[Network, NetworkStakingConfig] = field(default_factory=dict)
    total_treasury_allocation: float = 100.0  # percentage
    rebalancing_enabled: bool = True
    global_rebalance_frequency: int = 86400  # seconds
    performance_tracking: bool = True
    risk_management: bool = True
    emergency_controls: bool = True
    governance_enabled: bool = True

# Solana Validator Configurations
SOLANA_VALIDATORS = [
    ValidatorConfig(
        name="Everstake",
        address="StepeLdhJ2znRjHcZdjwMWsC4nTRURNKQY8Nca82LJp",
        network=Network.SOLANA,
        commission_rate=7.0,
        status=ValidatorStatus.ACTIVE,
        metrics=ValidatorMetrics(
            uptime_percentage=99.5,
            commission_rate=7.0,
            apy=6.8,
            performance_score=95.0,
            reliability_score=98.0
        ),
        priority=1,
        allocation_percentage=20.0,
        is_preferred=True,
        description="High-performance validator with excellent uptime",
        website="https://everstake.one"
    ),
    ValidatorConfig(
        name="Chorus One",
        address="ChorusmmK7i1AxXeiTtQgQZhQNiXYU84ULeaYF1EH15n",
        network=Network.SOLANA,
        commission_rate=8.0,
        status=ValidatorStatus.ACTIVE,
        metrics=ValidatorMetrics(
            uptime_percentage=99.2,
            commission_rate=8.0,
            apy=6.5,
            performance_score=92.0,
            reliability_score=96.0
        ),
        priority=2,
        allocation_percentage=15.0,
        is_preferred=True,
        description="Institutional-grade validator with strong track record",
        website="https://chorus.one"
    ),
    ValidatorConfig(
        name="Shinobi Systems",
        address="Ninja1spj6n9t5hVYgF3PdnYz2PLnkt7rvaw3firmjs",
        network=Network.SOLANA,
        commission_rate=5.0,
        status=ValidatorStatus.ACTIVE,
        metrics=ValidatorMetrics(
            uptime_percentage=98.8,
            commission_rate=5.0,
            apy=7.2,
            performance_score=90.0,
            reliability_score=94.0
        ),
        priority=3,
        allocation_percentage=10.0,
        description="Low commission validator with good performance",
        website="https://shinobi-systems.com"
    ),
    ValidatorConfig(
        name="Figment",
        address="Figment1L1XXUsb5HEnYdvks8co2rHH5r9weTMzre5Z5HS",
        network=Network.SOLANA,
        commission_rate=10.0,
        status=ValidatorStatus.ACTIVE,
        metrics=ValidatorMetrics(
            uptime_percentage=99.1,
            commission_rate=10.0,
            apy=6.2,
            performance_score=88.0,
            reliability_score=95.0
        ),
        priority=4,
        allocation_percentage=5.0,
        description="Enterprise validator with comprehensive services",
        website="https://figment.io"
    )
]

# Ethereum Validator Configurations (for ETH 2.0 staking)
ETHEREUM_VALIDATORS = [
    ValidatorConfig(
        name="Lido",
        address="0xae7ab96520DE3A18E5e111B5EaAb095312D7fE84",  # Lido stETH contract
        network=Network.ETHEREUM,
        commission_rate=10.0,
        status=ValidatorStatus.ACTIVE,
        metrics=ValidatorMetrics(
            uptime_percentage=99.8,
            commission_rate=10.0,
            apy=4.5,
            performance_score=96.0,
            reliability_score=99.0
        ),
        priority=1,
        allocation_percentage=60.0,
        is_preferred=True,
        description="Largest liquid staking protocol for Ethereum",
        website="https://lido.fi"
    ),
    ValidatorConfig(
        name="Rocket Pool",
        address="0xae78736Cd615f374D3085123A210448E74Fc6393",  # Rocket Pool rETH contract
        network=Network.ETHEREUM,
        commission_rate=15.0,
        status=ValidatorStatus.ACTIVE,
        metrics=ValidatorMetrics(
            uptime_percentage=99.3,
            commission_rate=15.0,
            apy=4.2,
            performance_score=92.0,
            reliability_score=96.0
        ),
        priority=2,
        allocation_percentage=30.0,
        is_preferred=True,
        description="Decentralized liquid staking protocol",
        website="https://rocketpool.net"
    ),
    ValidatorConfig(
        name="Coinbase",
        address="0xBe9895146f7AF43049ca1c1AE358B0541Ea49704",  # Coinbase cbETH contract
        network=Network.ETHEREUM,
        commission_rate=25.0,
        status=ValidatorStatus.ACTIVE,
        metrics=ValidatorMetrics(
            uptime_percentage=99.0,
            commission_rate=25.0,
            apy=3.8,
            performance_score=85.0,
            reliability_score=93.0
        ),
        priority=3,
        allocation_percentage=10.0,
        description="Centralized exchange staking service",
        website="https://coinbase.com"
    )
]

# Cosmos Validator Configurations
COSMOS_VALIDATORS = [
    ValidatorConfig(
        name="Everstake",
        address="cosmosvaloper1tflk30mq5vgqjdly92kkhhq3raev2hnz6eete3",
        network=Network.COSMOS,
        commission_rate=5.0,
        status=ValidatorStatus.ACTIVE,
        metrics=ValidatorMetrics(
            uptime_percentage=99.7,
            commission_rate=5.0,
            apy=18.5,
            performance_score=97.0,
            reliability_score=99.0
        ),
        priority=1,
        allocation_percentage=40.0,
        is_preferred=True,
        description="Top-tier Cosmos validator with excellent performance",
        website="https://everstake.one"
    ),
    ValidatorConfig(
        name="Cephalopod Equipment",
        address="cosmosvaloper16k579jk6yt2cwmqx9dz5xvq9fug2tekvlu9qdv",
        network=Network.COSMOS,
        commission_rate=8.0,
        status=ValidatorStatus.ACTIVE,
        metrics=ValidatorMetrics(
            uptime_percentage=99.4,
            commission_rate=8.0,
            apy=17.8,
            performance_score=94.0,
            reliability_score=97.0
        ),
        priority=2,
        allocation_percentage=30.0,
        is_preferred=True,
        description="Reliable validator with strong community presence",
        website="https://cephalopod.equipment"
    ),
    ValidatorConfig(
        name="Osmosis Labs",
        address="cosmosvaloper14lultfckehtszvzw4ehu0apvsr77afvyju5zzy",
        network=Network.COSMOS,
        commission_rate=5.0,
        status=ValidatorStatus.ACTIVE,
        metrics=ValidatorMetrics(
            uptime_percentage=99.2,
            commission_rate=5.0,
            apy=18.2,
            performance_score=92.0,
            reliability_score=95.0
        ),
        priority=3,
        allocation_percentage=20.0,
        description="Core team validator for Osmosis DEX",
        website="https://osmosis.zone"
    ),
    ValidatorConfig(
        name="Figment",
        address="cosmosvaloper1hjct6q7npsspsg3dgvzk3sdf89spmlpfdn6m9d",
        network=Network.COSMOS,
        commission_rate=9.0,
        status=ValidatorStatus.ACTIVE,
        metrics=ValidatorMetrics(
            uptime_percentage=98.9,
            commission_rate=9.0,
            apy=17.5,
            performance_score=89.0,
            reliability_score=93.0
        ),
        priority=4,
        allocation_percentage=10.0,
        description="Enterprise validator with institutional focus",
        website="https://figment.io"
    )
]

# Default Network Configurations
DEFAULT_NETWORK_CONFIGS = {
    Network.SOLANA: NetworkStakingConfig(
        network=Network.SOLANA,
        total_allocation_percentage=40.0,  # 40% of total treasury
        strategy=StakingStrategy.PERFORMANCE_BASED,
        validators=SOLANA_VALIDATORS,
        min_validators=3,
        max_validators=8,
        rebalance_threshold=5.0,
        rebalance_frequency=86400,
        min_stake_per_validator=1000000,  # 1 SOL
        max_stake_per_validator=100000000000,  # 100,000 SOL
    ),
    Network.ETHEREUM: NetworkStakingConfig(
        network=Network.ETHEREUM,
        total_allocation_percentage=30.0,  # 30% of total treasury
        strategy=StakingStrategy.DIVERSIFIED,
        validators=ETHEREUM_VALIDATORS,
        min_validators=2,
        max_validators=5,
        rebalance_threshold=3.0,
        rebalance_frequency=604800,  # Weekly
        min_stake_per_validator=32000000000000000000,  # 32 ETH
        max_stake_per_validator=10000000000000000000000,  # 10,000 ETH
    ),
    Network.COSMOS: NetworkStakingConfig(
        network=Network.COSMOS,
        total_allocation_percentage=30.0,  # 30% of total treasury
        strategy=StakingStrategy.HIGH_RELIABILITY,
        validators=COSMOS_VALIDATORS,
        min_validators=3,
        max_validators=6,
        rebalance_threshold=4.0,
        rebalance_frequency=86400,
        min_stake_per_validator=1000000,  # 1 ATOM
        max_stake_per_validator=1000000000000,  # 1M ATOM
    )
}

class ValidatorConfigManager:
    """Manager for validator configurations"""
    
    def __init__(self):
        self.config = StakingPoolConfig(
            networks=DEFAULT_NETWORK_CONFIGS.copy()
        )
        self._load_environment_overrides()
    
    def _load_environment_overrides(self):
        """Load configuration overrides from environment variables"""
        # Global rebalance frequency
        rebalance_freq = os.getenv("VALIDATOR_REBALANCE_FREQUENCY")
        if rebalance_freq:
            try:
                self.config.global_rebalance_frequency = int(rebalance_freq)
            except ValueError:
                pass
        
        # Performance tracking toggle
        perf_tracking = os.getenv("VALIDATOR_PERFORMANCE_TRACKING")
        if perf_tracking:
            self.config.performance_tracking = perf_tracking.lower() in ("true", "1", "yes")
        
        # Risk management toggle
        risk_mgmt = os.getenv("VALIDATOR_RISK_MANAGEMENT")
        if risk_mgmt:
            self.config.risk_management = risk_mgmt.lower() in ("true", "1", "yes")
    
    def get_config(self) -> StakingPoolConfig:
        """Get the current staking pool configuration"""
        return self.config
    
    def get_network_config(self, network: Network) -> Optional[NetworkStakingConfig]:
        """Get configuration for a specific network"""
        return self.config.networks.get(network)
    
    def get_validators(self, network: Network, active_only: bool = True) -> List[ValidatorConfig]:
        """Get validators for a specific network"""
        network_config = self.get_network_config(network)
        if not network_config:
            return []
        
        validators = network_config.validators
        if active_only:
            validators = [v for v in validators if v.status == ValidatorStatus.ACTIVE and not v.blacklisted]
        
        return sorted(validators, key=lambda x: x.priority)
    
    def get_preferred_validators(self, network: Network) -> List[ValidatorConfig]:
        """Get preferred validators for a network"""
        validators = self.get_validators(network, active_only=True)
        return [v for v in validators if v.is_preferred]
    
    def add_validator(self, network: Network, validator: ValidatorConfig):
        """Add a validator to a network"""
        if network not in self.config.networks:
            self.config.networks[network] = NetworkStakingConfig(
                network=network,
                total_allocation_percentage=0.0,
                strategy=StakingStrategy.CUSTOM
            )
        
        validator.created_at = datetime.now().isoformat()
        validator.updated_at = datetime.now().isoformat()
        self.config.networks[network].validators.append(validator)
    
    def remove_validator(self, network: Network, validator_address: str):
        """Remove a validator from a network"""
        network_config = self.get_network_config(network)
        if network_config:
            network_config.validators = [
                v for v in network_config.validators 
                if v.address != validator_address
            ]
    
    def update_validator_metrics(self, network: Network, validator_address: str, metrics: ValidatorMetrics):
        """Update metrics for a specific validator"""
        network_config = self.get_network_config(network)
        if network_config:
            for validator in network_config.validators:
                if validator.address == validator_address:
                    validator.metrics = metrics
                    validator.updated_at = datetime.now().isoformat()
                    break
    
    def blacklist_validator(self, network: Network, validator_address: str, reason: str = ""):
        """Blacklist a validator"""
        network_config = self.get_network_config(network)
        if network_config:
            for validator in network_config.validators:
                if validator.address == validator_address:
                    validator.blacklisted = True
                    validator.status = ValidatorStatus.BLACKLISTED
                    validator.description = f"BLACKLISTED: {reason}"
                    validator.updated_at = datetime.now().isoformat()
                    break
    
    def calculate_optimal_allocation(self, network: Network) -> Dict[str, float]:
        """Calculate optimal allocation percentages for validators"""
        validators = self.get_validators(network, active_only=True)
        network_config = self.get_network_config(network)
        
        if not validators or not network_config:
            return {}
        
        strategy = network_config.strategy
        allocations = {}
        
        if strategy == StakingStrategy.PERFORMANCE_BASED:
            # Allocate based on performance scores
            total_score = sum(v.metrics.performance_score for v in validators)
            for validator in validators:
                if total_score > 0:
                    allocations[validator.address] = (validator.metrics.performance_score / total_score) * 100
        
        elif strategy == StakingStrategy.DIVERSIFIED:
            # Equal allocation across all validators
            allocation_per_validator = 100.0 / len(validators)
            for validator in validators:
                allocations[validator.address] = allocation_per_validator
        
        elif strategy == StakingStrategy.LOW_COMMISSION:
            # Favor validators with lower commission rates
            # Invert commission rates for scoring (lower commission = higher score)
            commission_scores = [100 - v.commission_rate for v in validators]
            total_score = sum(commission_scores)
            for i, validator in enumerate(validators):
                if total_score > 0:
                    allocations[validator.address] = (commission_scores[i] / total_score) * 100
        
        elif strategy == StakingStrategy.HIGH_RELIABILITY:
            # Allocate based on reliability scores
            total_score = sum(v.metrics.reliability_score for v in validators)
            for validator in validators:
                if total_score > 0:
                    allocations[validator.address] = (validator.metrics.reliability_score / total_score) * 100
        
        else:  # CUSTOM or fallback
            # Use predefined allocation percentages
            for validator in validators:
                allocations[validator.address] = validator.allocation_percentage
        
        return allocations
    
    def get_staking_summary(self) -> Dict:
        """Get a summary of staking configuration"""
        summary = {
            "total_networks": len(self.config.networks),
            "total_allocation": self.config.total_treasury_allocation,
            "networks": {}
        }
        
        for network, network_config in self.config.networks.items():
            active_validators = self.get_validators(network, active_only=True)
            preferred_validators = self.get_preferred_validators(network)
            
            summary["networks"][network.value] = {
                "allocation_percentage": network_config.total_allocation_percentage,
                "strategy": network_config.strategy.value,
                "total_validators": len(network_config.validators),
                "active_validators": len(active_validators),
                "preferred_validators": len(preferred_validators),
                "avg_commission": sum(v.commission_rate for v in active_validators) / len(active_validators) if active_validators else 0,
                "avg_performance": sum(v.metrics.performance_score for v in active_validators) / len(active_validators) if active_validators else 0,
                "rebalance_frequency": network_config.rebalance_frequency
            }
        
        return summary
    
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
    def load_from_file(cls, filepath: str) -> 'ValidatorConfigManager':
        """Load configuration from JSON file"""
        with open(filepath, 'r') as f:
            data = json.load(f)
        
        manager = cls()
        # Implementation would reconstruct the configuration from JSON
        # This is simplified for brevity
        return manager
    
    def validate_config(self) -> List[str]:
        """Validate the current configuration and return any errors"""
        errors = []
        
        # Validate total allocation
        total_allocation = sum(
            network_config.total_allocation_percentage 
            for network_config in self.config.networks.values()
        )
        
        if abs(total_allocation - self.config.total_treasury_allocation) > 0.01:
            errors.append(f"Network allocations ({total_allocation}%) don't match total treasury allocation ({self.config.total_treasury_allocation}%)")
        
        # Validate each network
        for network, network_config in self.config.networks.items():
            active_validators = self.get_validators(network, active_only=True)
            
            if len(active_validators) < network_config.min_validators:
                errors.append(f"{network.value}: Not enough active validators ({len(active_validators)} < {network_config.min_validators})")
            
            if len(active_validators) > network_config.max_validators:
                errors.append(f"{network.value}: Too many active validators ({len(active_validators)} > {network_config.max_validators})")
            
            # Validate validator allocations
            total_validator_allocation = sum(v.allocation_percentage for v in active_validators)
            if abs(total_validator_allocation - 100.0) > 0.01:
                errors.append(f"{network.value}: Validator allocations don't sum to 100% ({total_validator_allocation}%)")
        
        return errors

# Utility functions
def create_validator_manager() -> ValidatorConfigManager:
    """Create a new validator configuration manager"""
    return ValidatorConfigManager()

def load_validator_config_from_env() -> ValidatorConfigManager:
    """Load validator configuration from environment variables"""
    return ValidatorConfigManager()

# Example usage and testing
if __name__ == "__main__":
    # Create validator configuration manager
    validator_manager = create_validator_manager()
    
    # Print staking summary
    print("Validator Staking Summary:")
    summary = validator_manager.get_staking_summary()
    print(json.dumps(summary, indent=2))
    
    # Print optimal allocations for each network
    print("\nOptimal Validator Allocations:")
    for network in [Network.SOLANA, Network.ETHEREUM, Network.COSMOS]:
        allocations = validator_manager.calculate_optimal_allocation(network)
        if allocations:
            print(f"\n{network.value.upper()}:")
            for address, percentage in allocations.items():
                validator_name = next(
                    (v.name for v in validator_manager.get_validators(network) if v.address == address),
                    "Unknown"
                )
                print(f"  {validator_name}: {percentage:.2f}%")
    
    # Validate configuration
    errors = validator_manager.validate_config()
    if errors:
        print("\nConfiguration Errors:")
        for error in errors:
            print(f"  - {error}")
    else:
        print("\nValidator configuration is valid!")
    
    # Save configuration to file
    validator_manager.save_to_file("validator_config.json")
    print("\nConfiguration saved to validator_config.json")