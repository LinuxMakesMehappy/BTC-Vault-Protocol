# Monitoring and Alerting Configuration
# This file contains configuration for system health monitoring, alerting thresholds,
# and notification settings for the Vault Protocol

import os
from typing import Dict, List, Optional
from dataclasses import dataclass
from enum import Enum

class AlertSeverity(Enum):
    """Alert severity levels"""
    LOW = "low"
    MEDIUM = "medium"
    HIGH = "high"
    CRITICAL = "critical"

class ComponentType(Enum):
    """System component types for monitoring"""
    ORACLE = "oracle"
    STAKING = "staking"
    SECURITY = "security"
    FRONTEND = "frontend"
    BACKEND = "backend"
    TREASURY = "treasury"
    PAYMENT = "payment"

@dataclass
class MonitoringThresholds:
    """Monitoring thresholds for various metrics"""
    # Oracle monitoring thresholds
    oracle_response_time_ms: int = 5000  # 5 seconds max response time
    oracle_failure_rate_percent: float = 5.0  # 5% max failure rate
    oracle_stale_data_minutes: int = 2  # 2 minutes max stale data
    
    # Staking monitoring thresholds
    staking_reward_variance_percent: float = 10.0  # 10% max variance from expected
    validator_uptime_percent: float = 95.0  # 95% min validator uptime
    slashing_event_threshold: int = 1  # Alert on any slashing event
    
    # Security monitoring thresholds
    failed_auth_attempts_per_hour: int = 10  # Max failed auth attempts
    suspicious_transaction_threshold: float = 1000.0  # USD value threshold
    multisig_timeout_hours: int = 24  # Max time for multisig approval
    
    # Performance monitoring thresholds
    frontend_load_time_ms: int = 3000  # 3 seconds max load time
    backend_response_time_ms: int = 1000  # 1 second max response time
    memory_usage_percent: float = 80.0  # 80% max memory usage
    cpu_usage_percent: float = 85.0  # 85% max CPU usage
    
    # Treasury monitoring thresholds
    treasury_balance_min_usd: float = 10000.0  # Min treasury balance
    deposit_delay_hours: int = 2  # Max delay for scheduled deposits
    rebalancing_variance_percent: float = 5.0  # Max allocation variance

@dataclass
class AlertChannel:
    """Alert notification channel configuration"""
    name: str
    type: str  # email, slack, webhook, sms
    endpoint: str
    enabled: bool = True
    severity_filter: List[AlertSeverity] = None

class MonitoringConfig:
    """Main monitoring configuration class"""
    
    def __init__(self):
        self.thresholds = MonitoringThresholds()
        self.alert_channels = self._load_alert_channels()
        self.monitoring_interval_seconds = 30
        self.alert_cooldown_minutes = 15  # Prevent alert spam
        self.health_check_endpoints = self._get_health_check_endpoints()
        
    def _load_alert_channels(self) -> List[AlertChannel]:
        """Load alert channels from environment variables"""
        channels = []
        
        # Email alerts
        if os.getenv('ALERT_EMAIL_ENDPOINT'):
            channels.append(AlertChannel(
                name="email",
                type="email",
                endpoint=os.getenv('ALERT_EMAIL_ENDPOINT'),
                severity_filter=[AlertSeverity.HIGH, AlertSeverity.CRITICAL]
            ))
        
        # Slack alerts
        if os.getenv('ALERT_SLACK_WEBHOOK'):
            channels.append(AlertChannel(
                name="slack",
                type="slack",
                endpoint=os.getenv('ALERT_SLACK_WEBHOOK'),
                severity_filter=[AlertSeverity.MEDIUM, AlertSeverity.HIGH, AlertSeverity.CRITICAL]
            ))
        
        # SMS alerts for critical issues
        if os.getenv('ALERT_SMS_ENDPOINT'):
            channels.append(AlertChannel(
                name="sms",
                type="sms",
                endpoint=os.getenv('ALERT_SMS_ENDPOINT'),
                severity_filter=[AlertSeverity.CRITICAL]
            ))
        
        # Custom webhook
        if os.getenv('ALERT_WEBHOOK_URL'):
            channels.append(AlertChannel(
                name="webhook",
                type="webhook",
                endpoint=os.getenv('ALERT_WEBHOOK_URL')
            ))
        
        return channels
    
    def _get_health_check_endpoints(self) -> Dict[str, str]:
        """Get health check endpoints for different components"""
        base_url = os.getenv('VAULT_API_BASE_URL', 'http://localhost:8080')
        frontend_url = os.getenv('FRONTEND_URL', 'http://localhost:3000')
        
        return {
            'solana_program': f"{base_url}/health/solana",
            'oracle_service': f"{base_url}/health/oracle",
            'staking_service': f"{base_url}/health/staking",
            'treasury_service': f"{base_url}/health/treasury",
            'payment_service': f"{base_url}/health/payment",
            'frontend': f"{frontend_url}/api/health",
            'database': f"{base_url}/health/database"
        }
    
    def get_component_thresholds(self, component: ComponentType) -> Dict:
        """Get monitoring thresholds for a specific component"""
        thresholds_map = {
            ComponentType.ORACLE: {
                'response_time_ms': self.thresholds.oracle_response_time_ms,
                'failure_rate_percent': self.thresholds.oracle_failure_rate_percent,
                'stale_data_minutes': self.thresholds.oracle_stale_data_minutes
            },
            ComponentType.STAKING: {
                'reward_variance_percent': self.thresholds.staking_reward_variance_percent,
                'validator_uptime_percent': self.thresholds.validator_uptime_percent,
                'slashing_threshold': self.thresholds.slashing_event_threshold
            },
            ComponentType.SECURITY: {
                'failed_auth_per_hour': self.thresholds.failed_auth_attempts_per_hour,
                'suspicious_tx_threshold': self.thresholds.suspicious_transaction_threshold,
                'multisig_timeout_hours': self.thresholds.multisig_timeout_hours
            },
            ComponentType.FRONTEND: {
                'load_time_ms': self.thresholds.frontend_load_time_ms,
                'memory_usage_percent': self.thresholds.memory_usage_percent
            },
            ComponentType.BACKEND: {
                'response_time_ms': self.thresholds.backend_response_time_ms,
                'cpu_usage_percent': self.thresholds.cpu_usage_percent,
                'memory_usage_percent': self.thresholds.memory_usage_percent
            },
            ComponentType.TREASURY: {
                'balance_min_usd': self.thresholds.treasury_balance_min_usd,
                'deposit_delay_hours': self.thresholds.deposit_delay_hours,
                'rebalancing_variance_percent': self.thresholds.rebalancing_variance_percent
            }
        }
        
        return thresholds_map.get(component, {})

# Global monitoring configuration instance
monitoring_config = MonitoringConfig()

# Alert message templates
ALERT_TEMPLATES = {
    'oracle_failure': {
        'title': 'Oracle Service Failure',
        'message': 'Oracle {oracle_name} has failed {failure_count} times in the last hour. Current failure rate: {failure_rate}%',
        'severity': AlertSeverity.HIGH
    },
    'oracle_stale_data': {
        'title': 'Oracle Data Stale',
        'message': 'Oracle {oracle_name} has not updated data for {minutes} minutes. Last update: {last_update}',
        'severity': AlertSeverity.MEDIUM
    },
    'staking_slashing': {
        'title': 'Validator Slashing Event',
        'message': 'Validator {validator} has been slashed. Amount: {amount} {token}. Reason: {reason}',
        'severity': AlertSeverity.CRITICAL
    },
    'staking_low_rewards': {
        'title': 'Low Staking Rewards',
        'message': 'Staking rewards for {token} are {variance}% below expected. Current APY: {current_apy}%',
        'severity': AlertSeverity.MEDIUM
    },
    'security_breach_attempt': {
        'title': 'Security Breach Attempt',
        'message': 'Multiple failed authentication attempts detected. User: {user}, Attempts: {attempts}, Time window: {time_window}',
        'severity': AlertSeverity.HIGH
    },
    'multisig_timeout': {
        'title': 'Multisig Transaction Timeout',
        'message': 'Multisig transaction {tx_id} has been pending for {hours} hours without sufficient signatures',
        'severity': AlertSeverity.HIGH
    },
    'treasury_low_balance': {
        'title': 'Treasury Low Balance',
        'message': 'Treasury balance has fallen below minimum threshold. Current: ${current_balance}, Minimum: ${min_balance}',
        'severity': AlertSeverity.CRITICAL
    },
    'performance_degradation': {
        'title': 'Performance Degradation',
        'message': '{component} performance has degraded. {metric}: {current_value} (threshold: {threshold})',
        'severity': AlertSeverity.MEDIUM
    },
    'system_health_critical': {
        'title': 'System Health Critical',
        'message': 'Critical system health issue detected in {component}. Status: {status}, Details: {details}',
        'severity': AlertSeverity.CRITICAL
    }
}

# Monitoring metrics collection intervals (in seconds)
METRIC_COLLECTION_INTERVALS = {
    'oracle_health': 30,
    'staking_performance': 60,
    'security_events': 10,
    'system_performance': 30,
    'treasury_status': 300,  # 5 minutes
    'user_activity': 60
}