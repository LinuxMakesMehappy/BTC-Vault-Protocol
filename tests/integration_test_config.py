#!/usr/bin/env python3
"""
Integration Test Configuration
Configuration settings for comprehensive integration tests

Addresses Task 26: Create comprehensive integration tests
Requirements: FR7 - Testing and Development Infrastructure
"""

import os
from typing import Dict, Any, List

# Test Environment Configuration
TEST_CONFIG = {
    # System Resource Limits (optimized for low-resource systems)
    'system_limits': {
        'max_memory_mb': 4096,  # 4GB limit for 8GB systems
        'max_cpu_percent': 80,
        'max_execution_time_seconds': 600,  # 10 minutes
        'concurrent_workers': 4  # Conservative for 8GB RAM
    },
    
    # User Journey Test Configuration
    'user_journeys': {
        'test_user_count': 10,
        'max_concurrent_journeys': 5,
        'journey_timeout_seconds': 90,
        'btc_commitment_range': (0.1, 5.0),  # BTC amounts to test
        'kyc_threshold_btc': 1.0,
        'supported_payment_methods': ['BTC', 'USDC', 'auto_reinvest']
    },
    
    # Cross-Chain Integration Configuration
    'cross_chain': {
        'chains': {
            'ethereum': {
                'rpc_url': 'https://eth-mainnet.alchemyapi.io/v2/test',
                'chain_id': 1,
                'staking_contract': '0x1234567890123456789012345678901234567890',
                'allocation_percentage': 30
            },
            'arbitrum': {
                'rpc_url': 'https://arb1.arbitrum.io/rpc',
                'chain_id': 42161,
                'staking_contract': '0x2345678901234567890123456789012345678901',
                'allocation_percentage': 15
            },
            'optimism': {
                'rpc_url': 'https://mainnet.optimism.io',
                'chain_id': 10,
                'staking_contract': '0x3456789012345678901234567890123456789012',
                'allocation_percentage': 15
            },
            'cosmoshub': {
                'rpc_url': 'https://rpc-cosmoshub.blockapsis.com',
                'chain_id': 'cosmoshub-4',
                'validators': [
                    'cosmosvaloper1everstake',
                    'cosmosvaloper1cephalopod'
                ],
                'allocation_percentage': 20
            },
            'osmosis': {
                'rpc_url': 'https://rpc-osmosis.blockapsis.com',
                'chain_id': 'osmosis-1',
                'validators': [
                    'osmovaloper1osmosis'
                ],
                'allocation_percentage': 10
            }
        },
        'message_timeout_seconds': 30,
        'retry_attempts': 3,
        'retry_delay_seconds': 5
    },
    
    # Stress Testing Configuration
    'stress_testing': {
        'concurrent_user_levels': [10, 50, 100],  # Reduced for low-resource systems
        'operations_per_user': 5,
        'database_connection_limit': 50,  # Conservative limit
        'oracle_request_limit': 100,
        'oracle_concurrent_limit': 20,
        'stress_test_duration_seconds': 60,
        'acceptable_failure_rate': 0.05  # 5% failure rate acceptable
    },
    
    # Security Integration Configuration
    'security': {
        'multisig': {
            'threshold': 2,
            'total_signers': 3,
            'hsm_required_amount_usd': 100000,  # $100k
            'daily_limit_usd': 1000000,  # $1M
            'signature_timeout_hours': 24
        },
        'hsm': {
            'device_type': 'YubiHSM2',
            'firmware_version': '2.3.1',
            'attestation_required': True,
            'attestation_timeout_seconds': 30
        },
        'authentication': {
            'max_failed_attempts': 3,
            'lockout_duration_minutes': 30,
            'session_timeout_minutes': 60,
            'require_2fa': True,
            'supported_2fa_methods': ['totp', 'webauthn', 'passkey']
        },
        'audit': {
            'hash_algorithm': 'sha256',
            'retention_days': 2555,  # 7 years
            'integrity_check_interval_hours': 24
        }
    },
    
    # Mock Data Configuration
    'mock_data': {
        'btc_addresses': [
            'bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh',
            'bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4',
            'bc1qrp33g0q5c5txsp9arysrx4k6zdkfs4nce4xj0gdcccefvpysxf3qccfmv3'
        ],
        'user_ids': [f'test_user_{i:04d}' for i in range(1000)],
        'transaction_amounts': [0.1, 0.5, 1.0, 2.5, 5.0],
        'staking_yields': {
            'sol': 0.06,  # 6% APY
            'eth': 0.04,  # 4% APY
            'atom': 0.08  # 8% APY
        }
    },
    
    # Test Data Validation
    'validation': {
        'btc_address_regex': r'^(bc1|[13])[a-zA-HJ-NP-Z0-9]{25,62}$',
        'transaction_hash_regex': r'^[a-fA-F0-9]{64}$',
        'min_commitment_btc': 0.001,  # 0.001 BTC minimum
        'max_commitment_btc': 100.0,  # 100 BTC maximum for testing
        'precision_decimals': 8  # Bitcoin precision
    },
    
    # Performance Benchmarks
    'benchmarks': {
        'btc_commitment_max_time_ms': 500,
        'reward_calculation_max_time_ms': 100,
        'payment_processing_max_time_ms': 1000,
        'cross_chain_message_max_time_ms': 2000,
        'multisig_signing_max_time_ms': 3000,
        'kyc_verification_max_time_ms': 5000
    },
    
    # Error Simulation Configuration
    'error_simulation': {
        'oracle_failure_rate': 0.02,  # 2% failure rate
        'network_timeout_rate': 0.01,  # 1% timeout rate
        'database_error_rate': 0.005,  # 0.5% error rate
        'cross_chain_failure_rate': 0.03,  # 3% failure rate
        'hsm_error_rate': 0.001  # 0.1% error rate
    }
}

# Environment-specific overrides
def get_test_config(environment: str = 'test') -> Dict[str, Any]:
    """Get test configuration for specific environment"""
    config = TEST_CONFIG.copy()
    
    if environment == 'ci':
        # CI environment optimizations
        config['system_limits']['max_execution_time_seconds'] = 300  # 5 minutes
        config['stress_testing']['concurrent_user_levels'] = [10, 25]  # Reduced load
        config['user_journeys']['test_user_count'] = 5
        
    elif environment == 'local':
        # Local development optimizations
        config['system_limits']['concurrent_workers'] = 2
        config['stress_testing']['concurrent_user_levels'] = [5, 10]
        config['user_journeys']['test_user_count'] = 3
        
    elif environment == 'staging':
        # Staging environment - full testing
        config['stress_testing']['concurrent_user_levels'] = [50, 100, 200]
        config['user_journeys']['test_user_count'] = 20
        
    return config

# Test data generators
def generate_test_btc_address(index: int) -> str:
    """Generate test BTC address"""
    addresses = TEST_CONFIG['mock_data']['btc_addresses']
    return addresses[index % len(addresses)]

def generate_test_user_id(index: int) -> str:
    """Generate test user ID"""
    return f"integration_test_user_{index:06d}"

def generate_test_commitment_amount(index: int) -> float:
    """Generate test commitment amount"""
    amounts = TEST_CONFIG['mock_data']['transaction_amounts']
    return amounts[index % len(amounts)]

# Validation functions
def validate_btc_address(address: str) -> bool:
    """Validate BTC address format"""
    import re
    pattern = TEST_CONFIG['validation']['btc_address_regex']
    return bool(re.match(pattern, address))

def validate_transaction_hash(tx_hash: str) -> bool:
    """Validate transaction hash format"""
    import re
    pattern = TEST_CONFIG['validation']['transaction_hash_regex']
    return bool(re.match(pattern, tx_hash))

def validate_commitment_amount(amount: float) -> bool:
    """Validate commitment amount"""
    min_amount = TEST_CONFIG['validation']['min_commitment_btc']
    max_amount = TEST_CONFIG['validation']['max_commitment_btc']
    return min_amount <= amount <= max_amount

# Performance monitoring
class PerformanceBenchmark:
    """Performance benchmark tracker"""
    
    def __init__(self, operation_name: str):
        self.operation_name = operation_name
        self.start_time = None
        self.end_time = None
        self.benchmark_ms = TEST_CONFIG['benchmarks'].get(
            f'{operation_name}_max_time_ms', 1000
        )
    
    def __enter__(self):
        import time
        self.start_time = time.time()
        return self
    
    def __exit__(self, exc_type, exc_val, exc_tb):
        import time
        self.end_time = time.time()
        
    def get_duration_ms(self) -> float:
        """Get operation duration in milliseconds"""
        if self.start_time and self.end_time:
            return (self.end_time - self.start_time) * 1000
        return 0
    
    def is_within_benchmark(self) -> bool:
        """Check if operation is within benchmark"""
        return self.get_duration_ms() <= self.benchmark_ms
    
    def get_performance_ratio(self) -> float:
        """Get performance ratio (actual/benchmark)"""
        duration = self.get_duration_ms()
        return duration / self.benchmark_ms if self.benchmark_ms > 0 else 0

# Resource monitoring
def get_system_resource_usage() -> Dict[str, Any]:
    """Get current system resource usage"""
    import psutil
    
    memory = psutil.virtual_memory()
    cpu_percent = psutil.cpu_percent(interval=1)
    
    return {
        'memory_used_mb': memory.used / (1024**2),
        'memory_available_mb': memory.available / (1024**2),
        'memory_percent': memory.percent,
        'cpu_percent': cpu_percent,
        'cpu_count': psutil.cpu_count(),
        'load_average': os.getloadavg() if hasattr(os, 'getloadavg') else [0, 0, 0]
    }

def is_within_resource_limits() -> bool:
    """Check if current resource usage is within limits"""
    usage = get_system_resource_usage()
    limits = TEST_CONFIG['system_limits']
    
    memory_ok = usage['memory_used_mb'] < limits['max_memory_mb']
    cpu_ok = usage['cpu_percent'] < limits['max_cpu_percent']
    
    return memory_ok and cpu_ok

# Export main configuration
__all__ = [
    'TEST_CONFIG',
    'get_test_config',
    'generate_test_btc_address',
    'generate_test_user_id', 
    'generate_test_commitment_amount',
    'validate_btc_address',
    'validate_transaction_hash',
    'validate_commitment_amount',
    'PerformanceBenchmark',
    'get_system_resource_usage',
    'is_within_resource_limits'
]