"""
Performance optimization configuration for low-resource systems
Ensures compatibility with 8GB RAM and 256GB storage constraints
"""

from typing import Dict, Any, List
import os

class PerformanceConfig:
    """Configuration for performance optimizations"""
    
    # System resource constraints
    SYSTEM_CONSTRAINTS = {
        'max_memory_gb': 8,
        'max_storage_gb': 256,
        'max_cpu_cores': 4,
        'target_memory_usage_percent': 50,  # Use max 50% of system memory
    }
    
    # Frontend optimization settings
    FRONTEND_OPTIMIZATION = {
        'max_bundle_size_mb': 5,
        'max_chunk_size_kb': 244,  # 244KB chunks for better loading
        'enable_compression': True,
        'enable_tree_shaking': True,
        'enable_code_splitting': True,
        'image_optimization': {
            'formats': ['webp', 'avif'],
            'quality': 80,
            'sizes': [640, 750, 828, 1080, 1200],
        },
        'cache_headers': {
            'static_assets': 'public, max-age=31536000, immutable',
            'api_responses': 'public, max-age=300',  # 5 minutes
            'html_pages': 'public, max-age=60',     # 1 minute
        }
    }
    
    # Memory management settings
    MEMORY_MANAGEMENT = {
        'max_frontend_memory_mb': 512,
        'max_cache_memory_mb': 100,
        'max_user_data_memory_mb': 200,
        'garbage_collection_interval_ms': 300000,  # 5 minutes
        'memory_pool_settings': {
            'transaction_pool_size': 200,
            'user_data_pool_size': 500,
            'api_response_pool_size': 300,
        },
        'cleanup_thresholds': {
            'memory_usage_percent': 80,
            'inactive_user_hours': 24,
            'cache_entry_hours': 1,
        }
    }
    
    # Caching configuration
    CACHE_CONFIG = {
        'oracle_cache': {
            'max_entries': 1000,
            'max_memory_mb': 50,
            'ttl_settings': {
                'btc_price_ms': 60000,      # 1 minute
                'utxo_balance_ms': 300000,  # 5 minutes
                'user_data_ms': 120000,     # 2 minutes
                'treasury_data_ms': 600000, # 10 minutes
                'staking_data_ms': 300000,  # 5 minutes
            },
            'cleanup_interval_ms': 60000,  # 1 minute
        },
        'ui_state_cache': {
            'max_entries': 500,
            'max_memory_mb': 25,
            'compression_threshold_bytes': 10240,  # 10KB
            'persistent_keys': [
                'user_preferences',
                'wallet_connection',
                'language_setting',
                'theme_setting',
                'dashboard_layout',
            ]
        }
    }
    
    # Data structure optimization
    DATA_STRUCTURE_CONFIG = {
        'user_manager': {
            'max_users_in_memory': 10000,
            'batch_size': 100,
            'cleanup_interval_ms': 300000,  # 5 minutes
            'eviction_strategy': 'lru',
            'index_types': ['id', 'address', 'kyc_status', 'activity'],
        },
        'transaction_manager': {
            'max_transactions_in_memory': 5000,
            'batch_processing_size': 50,
            'retention_hours': 24,
        }
    }
    
    # Performance monitoring settings
    MONITORING_CONFIG = {
        'enable_performance_monitoring': True,
        'metrics_collection_interval_ms': 30000,  # 30 seconds
        'memory_sampling_interval_ms': 5000,      # 5 seconds
        'performance_thresholds': {
            'max_memory_usage_mb': 4096,    # 4GB total
            'max_load_time_ms': 3000,       # 3 seconds
            'max_render_time_ms': 100,      # 100ms
            'min_cache_hit_rate_percent': 70,
            'max_bundle_load_time_ms': 2000,
        },
        'alert_settings': {
            'memory_threshold_percent': 90,
            'performance_degradation_percent': 50,
            'cache_miss_rate_threshold_percent': 50,
        }
    }
    
    # Network optimization
    NETWORK_CONFIG = {
        'api_request_batching': {
            'enabled': True,
            'batch_size': 10,
            'batch_timeout_ms': 100,
        },
        'connection_pooling': {
            'max_connections': 20,
            'connection_timeout_ms': 5000,
            'idle_timeout_ms': 30000,
        },
        'retry_settings': {
            'max_retries': 3,
            'initial_delay_ms': 1000,
            'backoff_multiplier': 2,
            'max_delay_ms': 10000,
        }
    }
    
    # Storage optimization
    STORAGE_CONFIG = {
        'max_total_storage_mb': 10240,  # 10GB max for application
        'component_limits_mb': {
            'frontend_build': 100,
            'cache_data': 150,
            'user_data': 500,
            'logs': 100,
            'temp_files': 50,
        },
        'cleanup_settings': {
            'log_retention_days': 7,
            'temp_file_retention_hours': 24,
            'cache_cleanup_interval_hours': 6,
        }
    }
    
    @classmethod
    def get_config(cls) -> Dict[str, Any]:
        """Get complete performance configuration"""
        return {
            'system_constraints': cls.SYSTEM_CONSTRAINTS,
            'frontend_optimization': cls.FRONTEND_OPTIMIZATION,
            'memory_management': cls.MEMORY_MANAGEMENT,
            'cache_config': cls.CACHE_CONFIG,
            'data_structure_config': cls.DATA_STRUCTURE_CONFIG,
            'monitoring_config': cls.MONITORING_CONFIG,
            'network_config': cls.NETWORK_CONFIG,
            'storage_config': cls.STORAGE_CONFIG,
        }
    
    @classmethod
    def get_memory_limits(cls) -> Dict[str, int]:
        """Get memory limits for different components"""
        return {
            'total_system_mb': cls.SYSTEM_CONSTRAINTS['max_memory_gb'] * 1024,
            'target_usage_mb': int(cls.SYSTEM_CONSTRAINTS['max_memory_gb'] * 1024 * 
                                 cls.SYSTEM_CONSTRAINTS['target_memory_usage_percent'] / 100),
            'frontend_mb': cls.MEMORY_MANAGEMENT['max_frontend_memory_mb'],
            'cache_mb': cls.MEMORY_MANAGEMENT['max_cache_memory_mb'],
            'user_data_mb': cls.MEMORY_MANAGEMENT['max_user_data_memory_mb'],
        }
    
    @classmethod
    def get_cache_settings(cls) -> Dict[str, Any]:
        """Get cache configuration settings"""
        return cls.CACHE_CONFIG
    
    @classmethod
    def get_performance_thresholds(cls) -> Dict[str, Any]:
        """Get performance monitoring thresholds"""
        return cls.MONITORING_CONFIG['performance_thresholds']
    
    @classmethod
    def validate_system_compatibility(cls) -> Dict[str, bool]:
        """Validate system meets minimum requirements"""
        import psutil
        
        # Get system information
        memory_gb = psutil.virtual_memory().total / 1024 / 1024 / 1024
        disk_gb = psutil.disk_usage('/').total / 1024 / 1024 / 1024
        cpu_count = psutil.cpu_count()
        
        return {
            'memory_sufficient': memory_gb >= cls.SYSTEM_CONSTRAINTS['max_memory_gb'],
            'storage_sufficient': disk_gb >= cls.SYSTEM_CONSTRAINTS['max_storage_gb'],
            'cpu_sufficient': cpu_count >= cls.SYSTEM_CONSTRAINTS['max_cpu_cores'],
            'system_memory_gb': memory_gb,
            'system_storage_gb': disk_gb,
            'system_cpu_count': cpu_count,
        }

# Environment-specific configurations
ENVIRONMENT_CONFIGS = {
    'development': {
        'enable_debug_monitoring': True,
        'memory_limits_relaxed': True,
        'cache_ttl_reduced': True,
    },
    'testing': {
        'enable_performance_tests': True,
        'strict_memory_limits': True,
        'detailed_metrics': True,
    },
    'production': {
        'optimize_for_performance': True,
        'strict_resource_limits': True,
        'enable_monitoring_alerts': True,
    }
}

def get_environment_config(env: str = None) -> Dict[str, Any]:
    """Get environment-specific configuration"""
    if env is None:
        env = os.getenv('NODE_ENV', 'development')
    
    base_config = PerformanceConfig.get_config()
    env_config = ENVIRONMENT_CONFIGS.get(env, {})
    
    # Merge configurations
    merged_config = {**base_config}
    for key, value in env_config.items():
        if key in merged_config:
            if isinstance(merged_config[key], dict) and isinstance(value, dict):
                merged_config[key].update(value)
            else:
                merged_config[key] = value
        else:
            merged_config[key] = value
    
    return merged_config

# Export main configuration
performance_config = PerformanceConfig.get_config()

if __name__ == "__main__":
    # Validate system compatibility
    compatibility = PerformanceConfig.validate_system_compatibility()
    print("System Compatibility Check:")
    for key, value in compatibility.items():
        print(f"  {key}: {value}")
    
    # Print configuration summary
    config = PerformanceConfig.get_config()
    print(f"\nPerformance Configuration Summary:")
    print(f"  Max Memory Usage: {config['memory_management']['max_frontend_memory_mb']}MB")
    print(f"  Max Bundle Size: {config['frontend_optimization']['max_bundle_size_mb']}MB")
    print(f"  Cache Memory Limit: {config['cache_config']['oracle_cache']['max_memory_mb']}MB")
    print(f"  Max Users in Memory: {config['data_structure_config']['user_manager']['max_users_in_memory']}")