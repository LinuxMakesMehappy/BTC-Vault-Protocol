#!/usr/bin/env python3
"""
Treasury Configuration System

This module provides configuration for treasury management including
deposit schedules, allocation percentages, and treasury operations.
"""

import os
from typing import Dict, List, Optional, Union
from dataclasses import dataclass, field
from enum import Enum
import json
from datetime import datetime, timedelta

class AssetType(Enum):
    """Supported asset types in treasury"""
    SOL = "SOL"
    ETH = "ETH"
    ATOM = "ATOM"
    USDC = "USDC"
    BTC = "BTC"

class DepositFrequency(Enum):
    """Deposit frequency options"""
    DAILY = "daily"
    WEEKLY = "weekly"
    BIWEEKLY = "biweekly"
    MONTHLY = "monthly"
    CUSTOM = "custom"

class RebalanceStrategy(Enum):
    """Treasury rebalancing strategies"""
    STRICT = "strict"  # Maintain exact percentages
    THRESHOLD = "threshold"  # Rebalance when deviation exceeds threshold
    DRIFT = "drift"  # Allow controlled drift within bounds
    MANUAL = "manual"  # Manual rebalancing only

@dataclass
class AssetAllocation:
    """Asset allocation configuration"""
    asset_type: AssetType
    target_percentage: float
    min_percentage: float
    max_percentage: float
    current_percentage: float = 0.0
    current_balance: int = 0
    current_value_usd: float = 0.0
    last_updated: Optional[str] = None
    rebalance_priority: int = 1  # 1 = highest priority
    auto_compound: bool = True
    staking_enabled: bool = True
    
@dataclass
class DepositSchedule:
    """Deposit schedule configuration"""
    amount_usd: float
    frequency: DepositFrequency
    frequency_days: int  # For custom frequency
    next_deposit_date: Optional[str] = None
    last_deposit_date: Optional[str] = None
    total_deposits: float = 0.0
    deposit_count: int = 0
    auto_deposit_enabled: bool = True
    deposit_source: str = "manual"  # manual, bank_transfer, etc.
    
@dataclass
class RebalanceConfig:
    """Treasury rebalancing configuration"""
    strategy: RebalanceStrategy
    threshold_percentage: float = 5.0  # Deviation threshold for rebalancing
    min_rebalance_amount_usd: float = 100.0  # Minimum amount to trigger rebalance
    max_rebalance_frequency: int = 86400  # Maximum frequency in seconds (24 hours)
    auto_rebalance_enabled: bool = True
    emergency_rebalance_enabled: bool = True
    slippage_tolerance: float = 1.0  # Maximum slippage percentage
    gas_price_limit: float = 50.0  # Maximum gas price in gwei for ETH operations
    
@dataclass
class TreasuryLimits:
    """Treasury operational limits"""
    max_total_value_usd: float = 10000000.0  # $10M max treasury
    min_total_value_usd: float = 1000.0  # $1K min treasury
    max_single_deposit_usd: float = 10000.0  # $10K max single deposit
    min_single_deposit_usd: float = 10.0  # $10 min single deposit
    max_daily_deposits_usd: float = 1000.0  # $1K max daily deposits
    emergency_reserve_percentage: float = 10.0  # 10% emergency reserve
    max_staking_percentage: float = 90.0  # 90% max staked
    
@dataclass
class TreasuryConfig:
    """Main treasury configuration"""
    allocations: Dict[AssetType, AssetAllocation] = field(default_factory=dict)
    deposit_schedule: DepositSchedule = field(default_factory=lambda: DepositSchedule(50.0, DepositFrequency.BIWEEKLY, 14))
    rebalance_config: RebalanceConfig = field(default_factory=lambda: RebalanceConfig(RebalanceStrategy.THRESHOLD))
    limits: TreasuryLimits = field(default_factory=TreasuryLimits)
    total_value_usd: float = 0.0
    last_rebalance: Optional[str] = None
    performance_tracking: bool = True
    risk_monitoring: bool = True
    compliance_enabled: bool = True
    audit_logging: bool = True

# Default Asset Allocations
DEFAULT_ALLOCATIONS = {
    AssetType.SOL: AssetAllocation(
        asset_type=AssetType.SOL,
        target_percentage=40.0,
        min_percentage=30.0,
        max_percentage=50.0,
        rebalance_priority=1,
        auto_compound=True,
        staking_enabled=True
    ),
    AssetType.ETH: AssetAllocation(
        asset_type=AssetType.ETH,
        target_percentage=30.0,
        min_percentage=20.0,
        max_percentage=40.0,
        rebalance_priority=2,
        auto_compound=True,
        staking_enabled=True
    ),
    AssetType.ATOM: AssetAllocation(
        asset_type=AssetType.ATOM,
        target_percentage=20.0,
        min_percentage=10.0,
        max_percentage=30.0,
        rebalance_priority=3,
        auto_compound=True,
        staking_enabled=True
    ),
    AssetType.USDC: AssetAllocation(
        asset_type=AssetType.USDC,
        target_percentage=8.0,
        min_percentage=5.0,
        max_percentage=15.0,
        rebalance_priority=4,
        auto_compound=False,
        staking_enabled=False
    ),
    AssetType.BTC: AssetAllocation(
        asset_type=AssetType.BTC,
        target_percentage=2.0,
        min_percentage=0.0,
        max_percentage=10.0,
        rebalance_priority=5,
        auto_compound=False,
        staking_enabled=False
    )
}

class TreasuryConfigManager:
    """Manager for treasury configuration"""
    
    def __init__(self):
        self.config = TreasuryConfig(
            allocations=DEFAULT_ALLOCATIONS.copy()
        )
        self._load_environment_overrides()
    
    def _load_environment_overrides(self):
        """Load configuration overrides from environment variables"""
        # Deposit amount override
        deposit_amount = os.getenv("TREASURY_DEPOSIT_AMOUNT")
        if deposit_amount:
            try:
                self.config.deposit_schedule.amount_usd = float(deposit_amount)
            except ValueError:
                pass
        
        # Deposit frequency override
        deposit_freq = os.getenv("TREASURY_DEPOSIT_FREQUENCY")
        if deposit_freq:
            try:
                self.config.deposit_schedule.frequency = DepositFrequency(deposit_freq.lower())
            except ValueError:
                pass
        
        # Auto rebalance toggle
        auto_rebalance = os.getenv("TREASURY_AUTO_REBALANCE")
        if auto_rebalance:
            self.config.rebalance_config.auto_rebalance_enabled = auto_rebalance.lower() in ("true", "1", "yes")
        
        # Rebalance threshold override
        rebalance_threshold = os.getenv("TREASURY_REBALANCE_THRESHOLD")
        if rebalance_threshold:
            try:
                self.config.rebalance_config.threshold_percentage = float(rebalance_threshold)
            except ValueError:
                pass
    
    def get_config(self) -> TreasuryConfig:
        """Get the current treasury configuration"""
        return self.config
    
    def get_asset_allocation(self, asset_type: AssetType) -> Optional[AssetAllocation]:
        """Get allocation configuration for a specific asset"""
        return self.config.allocations.get(asset_type)
    
    def update_asset_allocation(self, asset_type: AssetType, allocation: AssetAllocation):
        """Update allocation for a specific asset"""
        allocation.last_updated = datetime.now().isoformat()
        self.config.allocations[asset_type] = allocation
    
    def calculate_rebalance_needed(self) -> Dict[AssetType, float]:
        """Calculate how much rebalancing is needed for each asset"""
        rebalance_amounts = {}
        threshold = self.config.rebalance_config.threshold_percentage
        
        for asset_type, allocation in self.config.allocations.items():
            deviation = abs(allocation.current_percentage - allocation.target_percentage)
            if deviation > threshold:
                target_value = (allocation.target_percentage / 100.0) * self.config.total_value_usd
                current_value = allocation.current_value_usd
                rebalance_amounts[asset_type] = target_value - current_value
        
        return rebalance_amounts
    
    def get_next_deposit_date(self) -> Optional[datetime]:
        """Calculate the next deposit date based on schedule"""
        schedule = self.config.deposit_schedule
        if not schedule.auto_deposit_enabled:
            return None
        
        if schedule.last_deposit_date:
            last_deposit = datetime.fromisoformat(schedule.last_deposit_date)
        else:
            last_deposit = datetime.now()
        
        if schedule.frequency == DepositFrequency.DAILY:
            return last_deposit + timedelta(days=1)
        elif schedule.frequency == DepositFrequency.WEEKLY:
            return last_deposit + timedelta(weeks=1)
        elif schedule.frequency == DepositFrequency.BIWEEKLY:
            return last_deposit + timedelta(weeks=2)
        elif schedule.frequency == DepositFrequency.MONTHLY:
            return last_deposit + timedelta(days=30)
        elif schedule.frequency == DepositFrequency.CUSTOM:
            return last_deposit + timedelta(days=schedule.frequency_days)
        
        return None
    
    def update_balances(self, balances: Dict[AssetType, Dict[str, float]]):
        """Update current balances and calculate percentages"""
        total_value = 0.0
        
        # Calculate total value
        for asset_type, balance_info in balances.items():
            if asset_type in self.config.allocations:
                self.config.allocations[asset_type].current_balance = balance_info.get('balance', 0)
                self.config.allocations[asset_type].current_value_usd = balance_info.get('value_usd', 0.0)
                total_value += balance_info.get('value_usd', 0.0)
        
        self.config.total_value_usd = total_value
        
        # Calculate percentages
        if total_value > 0:
            for allocation in self.config.allocations.values():
                allocation.current_percentage = (allocation.current_value_usd / total_value) * 100.0
                allocation.last_updated = datetime.now().isoformat()
    
    def get_treasury_summary(self) -> Dict:
        """Get a summary of treasury status"""
        summary = {
            "total_value_usd": self.config.total_value_usd,
            "last_rebalance": self.config.last_rebalance,
            "next_deposit": self.get_next_deposit_date().isoformat() if self.get_next_deposit_date() else None,
            "rebalance_needed": len(self.calculate_rebalance_needed()) > 0,
            "assets": {}
        }
        
        for asset_type, allocation in self.config.allocations.items():
            deviation = abs(allocation.current_percentage - allocation.target_percentage)
            summary["assets"][asset_type.value] = {
                "target_percentage": allocation.target_percentage,
                "current_percentage": allocation.current_percentage,
                "deviation": deviation,
                "current_value_usd": allocation.current_value_usd,
                "needs_rebalance": deviation > self.config.rebalance_config.threshold_percentage,
                "staking_enabled": allocation.staking_enabled
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
    
    def validate_config(self) -> List[str]:
        """Validate the current configuration and return any errors"""
        errors = []
        
        # Validate total allocation percentages
        total_target = sum(allocation.target_percentage for allocation in self.config.allocations.values())
        if abs(total_target - 100.0) > 0.01:
            errors.append(f"Target allocations don't sum to 100% ({total_target}%)")
        
        # Validate individual allocations
        for asset_type, allocation in self.config.allocations.items():
            if allocation.target_percentage < 0 or allocation.target_percentage > 100:
                errors.append(f"{asset_type.value}: Target percentage must be between 0-100%")
            
            if allocation.min_percentage > allocation.max_percentage:
                errors.append(f"{asset_type.value}: Min percentage cannot exceed max percentage")
            
            if allocation.target_percentage < allocation.min_percentage or allocation.target_percentage > allocation.max_percentage:
                errors.append(f"{asset_type.value}: Target percentage outside min/max bounds")
        
        # Validate deposit schedule
        if self.config.deposit_schedule.amount_usd <= 0:
            errors.append("Deposit amount must be positive")
        
        if self.config.deposit_schedule.frequency_days <= 0:
            errors.append("Custom deposit frequency must be positive")
        
        # Validate limits
        if self.config.limits.max_total_value_usd <= self.config.limits.min_total_value_usd:
            errors.append("Max treasury value must exceed min treasury value")
        
        if self.config.limits.max_single_deposit_usd <= self.config.limits.min_single_deposit_usd:
            errors.append("Max single deposit must exceed min single deposit")
        
        return errors

# Utility functions
def create_treasury_manager() -> TreasuryConfigManager:
    """Create a new treasury configuration manager"""
    return TreasuryConfigManager()

def load_treasury_config_from_env() -> TreasuryConfigManager:
    """Load treasury configuration from environment variables"""
    return TreasuryConfigManager()

# Example usage and testing
if __name__ == "__main__":
    # Create treasury configuration manager
    treasury_manager = create_treasury_manager()
    
    # Print treasury summary
    print("Treasury Configuration Summary:")
    summary = treasury_manager.get_treasury_summary()
    print(json.dumps(summary, indent=2))
    
    # Example: Update balances
    example_balances = {
        AssetType.SOL: {"balance": 1000, "value_usd": 50000},
        AssetType.ETH: {"balance": 20, "value_usd": 40000},
        AssetType.ATOM: {"balance": 5000, "value_usd": 25000},
        AssetType.USDC: {"balance": 8000, "value_usd": 8000},
        AssetType.BTC: {"balance": 0.1, "value_usd": 2000}
    }
    
    treasury_manager.update_balances(example_balances)
    print(f"\nUpdated treasury total value: ${treasury_manager.config.total_value_usd:,.2f}")
    
    # Check rebalancing needs
    rebalance_needed = treasury_manager.calculate_rebalance_needed()
    if rebalance_needed:
        print("\nRebalancing needed:")
        for asset_type, amount in rebalance_needed.items():
            print(f"  {asset_type.value}: ${amount:,.2f}")
    else:
        print("\nNo rebalancing needed")
    
    # Validate configuration
    errors = treasury_manager.validate_config()
    if errors:
        print("\nConfiguration Errors:")
        for error in errors:
            print(f"  - {error}")
    else:
        print("\nTreasury configuration is valid!")
    
    # Save configuration to file
    treasury_manager.save_to_file("treasury_config.json")
    print("\nConfiguration saved to treasury_config.json")