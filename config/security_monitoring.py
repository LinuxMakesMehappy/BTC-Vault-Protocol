"""
Security Monitoring Configuration

Configuration settings for the security monitoring and logging system.
This file contains settings for anomaly detection rules, alert thresholds,
retention policies, and compliance requirements.

Task 23: Add security monitoring and logging
Requirements: SR1, SR2, SR5
"""

from typing import Dict, List, Any
from dataclasses import dataclass
from enum import Enum

class SecurityLevel(Enum):
    LOW = "Low"
    MEDIUM = "Medium"
    HIGH = "High"
    CRITICAL = "Critical"

class EventType(Enum):
    # Authentication events
    LOGIN_ATTEMPT = "LoginAttempt"
    LOGIN_SUCCESS = "LoginSuccess"
    LOGIN_FAILURE = "LoginFailure"
    TWO_FACTOR_REQUIRED = "TwoFactorRequired"
    TWO_FACTOR_SUCCESS = "TwoFactorSuccess"
    TWO_FACTOR_FAILURE = "TwoFactorFailure"
    SESSION_CREATED = "SessionCreated"
    SESSION_EXPIRED = "SessionExpired"
    SESSION_REVOKED = "SessionRevoked"
    ACCOUNT_LOCKED = "AccountLocked"
    ACCOUNT_UNLOCKED = "AccountUnlocked"
    
    # Transaction events
    BTC_COMMITMENT = "BTCCommitment"
    BTC_COMMITMENT_UPDATE = "BTCCommitmentUpdate"
    REWARD_CLAIM = "RewardClaim"
    PAYMENT_REQUEST = "PaymentRequest"
    PAYMENT_PROCESSED = "PaymentProcessed"
    PAYMENT_FAILED = "PaymentFailed"
    
    # Multisig events
    MULTISIG_PROPOSAL = "MultisigProposal"
    MULTISIG_SIGNING = "MultisigSigning"
    MULTISIG_EXECUTION = "MultisigExecution"
    KEY_ROTATION = "KeyRotation"
    EMERGENCY_MODE = "EmergencyMode"
    
    # KYC and compliance events
    KYC_SUBMISSION = "KYCSubmission"
    KYC_APPROVAL = "KYCApproval"
    KYC_REJECTION = "KYCRejection"
    COMPLIANCE_ALERT = "ComplianceAlert"
    ACCOUNT_FROZEN = "AccountFrozen"
    ACCOUNT_UNFROZEN = "AccountUnfrozen"
    AML_SCREENING = "AMLScreening"
    
    # Oracle and system events
    ORACLE_UPDATE = "OracleUpdate"
    ORACLE_FAILURE = "OracleFailure"
    SYSTEM_ERROR = "SystemError"
    SECURITY_VIOLATION = "SecurityViolation"
    
    # Anomaly detection events
    UNUSUAL_LOGIN_LOCATION = "UnusualLoginLocation"
    UNUSUAL_LOGIN_TIME = "UnusualLoginTime"
    HIGH_FREQUENCY_TRANSACTIONS = "HighFrequencyTransactions"
    LARGE_AMOUNT_TRANSACTION = "LargeAmountTransaction"
    SUSPICIOUS_PATTERN = "SuspiciousPattern"
    VELOCITY_ALERT = "VelocityAlert"
    DEVICE_CHANGE = "DeviceChange"
    IP_CHANGE = "IPChange"

@dataclass
class AnomalyRule:
    """Configuration for an anomaly detection rule"""
    rule_id: int
    name: str
    description: str
    event_types: List[EventType]
    enabled: bool
    threshold_value: float
    time_window_minutes: int
    severity: SecurityLevel
    auto_block: bool
    notification_required: bool

@dataclass
class SecurityMonitoringConfig:
    """Main configuration for security monitoring system"""
    
    # General settings
    enabled: bool = True
    retention_days: int = 365
    max_events_per_user: int = 1000
    auto_block_enabled: bool = True
    
    # Alert settings
    notification_webhook: str = None
    emergency_contacts: List[str] = None
    
    # Compliance settings
    compliance_retention_years: int = 10
    audit_trail_retention_years: int = 7
    
    # Performance settings
    max_concurrent_events: int = 100
    event_batch_size: int = 50
    cleanup_interval_hours: int = 24

# Default anomaly detection rules
DEFAULT_ANOMALY_RULES = [
    AnomalyRule(
        rule_id=1,
        name="Multiple Failed Logins",
        description="Detect multiple failed login attempts within a short time window",
        event_types=[EventType.LOGIN_FAILURE],
        enabled=True,
        threshold_value=5.0,
        time_window_minutes=15,
        severity=SecurityLevel.MEDIUM,
        auto_block=True,
        notification_required=True
    ),
    AnomalyRule(
        rule_id=2,
        name="High Value Transactions",
        description="Detect unusually large transactions that may indicate compromise",
        event_types=[EventType.BTC_COMMITMENT, EventType.REWARD_CLAIM],
        enabled=True,
        threshold_value=100000.0,  # $100k USD equivalent
        time_window_minutes=60,
        severity=SecurityLevel.HIGH,
        auto_block=False,
        notification_required=True
    ),
    AnomalyRule(
        rule_id=3,
        name="Rapid Transaction Velocity",
        description="Detect high frequency transactions that may indicate automated attacks",
        event_types=[EventType.PAYMENT_REQUEST, EventType.REWARD_CLAIM],
        enabled=True,
        threshold_value=10.0,
        time_window_minutes=5,
        severity=SecurityLevel.MEDIUM,
        auto_block=False,
        notification_required=True
    ),
    AnomalyRule(
        rule_id=4,
        name="Unusual Login Location",
        description="Detect logins from unusual geographic locations",
        event_types=[EventType.LOGIN_SUCCESS],
        enabled=True,
        threshold_value=1.0,
        time_window_minutes=1440,  # 24 hours
        severity=SecurityLevel.LOW,
        auto_block=False,
        notification_required=False
    ),
    AnomalyRule(
        rule_id=5,
        name="Security Violations",
        description="Detect any security violations that require immediate attention",
        event_types=[EventType.SECURITY_VIOLATION],
        enabled=True,
        threshold_value=1.0,
        time_window_minutes=1,
        severity=SecurityLevel.CRITICAL,
        auto_block=True,
        notification_required=True
    ),
    AnomalyRule(
        rule_id=6,
        name="2FA Bypass Attempts",
        description="Detect attempts to bypass two-factor authentication",
        event_types=[EventType.TWO_FACTOR_FAILURE],
        enabled=True,
        threshold_value=3.0,
        time_window_minutes=10,
        severity=SecurityLevel.HIGH,
        auto_block=True,
        notification_required=True
    ),
    AnomalyRule(
        rule_id=7,
        name="Emergency Mode Activation",
        description="Monitor emergency mode activations for potential security incidents",
        event_types=[EventType.EMERGENCY_MODE],
        enabled=True,
        threshold_value=1.0,
        time_window_minutes=1,
        severity=SecurityLevel.CRITICAL,
        auto_block=False,
        notification_required=True
    ),
    AnomalyRule(
        rule_id=8,
        name="Oracle Manipulation",
        description="Detect potential oracle manipulation attempts",
        event_types=[EventType.ORACLE_FAILURE],
        enabled=True,
        threshold_value=3.0,
        time_window_minutes=30,
        severity=SecurityLevel.HIGH,
        auto_block=False,
        notification_required=True
    ),
    AnomalyRule(
        rule_id=9,
        name="Compliance Alert Clustering",
        description="Detect clustering of compliance alerts that may indicate systematic issues",
        event_types=[EventType.COMPLIANCE_ALERT],
        enabled=True,
        threshold_value=5.0,
        time_window_minutes=60,
        severity=SecurityLevel.MEDIUM,
        auto_block=False,
        notification_required=True
    ),
    AnomalyRule(
        rule_id=10,
        name="Account Freeze Pattern",
        description="Monitor patterns of account freezes that may indicate targeted attacks",
        event_types=[EventType.ACCOUNT_FROZEN],
        enabled=True,
        threshold_value=3.0,
        time_window_minutes=120,
        severity=SecurityLevel.MEDIUM,
        auto_block=False,
        notification_required=True
    )
]

# Security level mappings for different event types
SECURITY_LEVEL_MAPPING = {
    # Critical events
    EventType.SECURITY_VIOLATION: SecurityLevel.CRITICAL,
    EventType.EMERGENCY_MODE: SecurityLevel.CRITICAL,
    
    # High severity events
    EventType.ACCOUNT_FROZEN: SecurityLevel.HIGH,
    EventType.KEY_ROTATION: SecurityLevel.HIGH,
    EventType.ORACLE_FAILURE: SecurityLevel.HIGH,
    EventType.LARGE_AMOUNT_TRANSACTION: SecurityLevel.HIGH,
    
    # Medium severity events
    EventType.LOGIN_FAILURE: SecurityLevel.MEDIUM,
    EventType.TWO_FACTOR_FAILURE: SecurityLevel.MEDIUM,
    EventType.COMPLIANCE_ALERT: SecurityLevel.MEDIUM,
    EventType.SUSPICIOUS_PATTERN: SecurityLevel.MEDIUM,
    EventType.HIGH_FREQUENCY_TRANSACTIONS: SecurityLevel.MEDIUM,
    EventType.UNUSUAL_LOGIN_LOCATION: SecurityLevel.MEDIUM,
    EventType.PAYMENT_FAILED: SecurityLevel.MEDIUM,
    
    # Low severity events (default for most events)
    EventType.LOGIN_SUCCESS: SecurityLevel.LOW,
    EventType.SESSION_CREATED: SecurityLevel.LOW,
    EventType.BTC_COMMITMENT: SecurityLevel.LOW,
    EventType.REWARD_CLAIM: SecurityLevel.LOW,
    EventType.PAYMENT_REQUEST: SecurityLevel.LOW,
    EventType.PAYMENT_PROCESSED: SecurityLevel.LOW,
    EventType.KYC_SUBMISSION: SecurityLevel.LOW,
    EventType.KYC_APPROVAL: SecurityLevel.LOW,
    EventType.ORACLE_UPDATE: SecurityLevel.LOW,
}

# Risk score calculation weights
RISK_SCORE_WEIGHTS = {
    "failed_login_attempts": 5,      # 5 points per failed login
    "suspicious_activity": 3,        # 3 points per suspicious activity
    "compliance_alerts": 5,          # 5 points per compliance alert
    "recent_suspicious_activity": 20, # 20 points if suspicious activity in last 7 days
    "kyc_tier_bonus": -10,          # -10 points for higher KYC tiers
    "account_age_bonus": -5,        # -5 points for accounts older than 6 months
}

# Risk score thresholds
RISK_SCORE_THRESHOLDS = {
    "low_risk": 0,
    "medium_risk": 30,
    "high_risk": 70,
    "critical_risk": 90
}

# Compliance settings
COMPLIANCE_SETTINGS = {
    "kyc_required_threshold": 100000,  # $100k USD equivalent
    "enhanced_dd_threshold": 500000,   # $500k USD equivalent
    "manual_review_threshold": 1000000, # $1M USD equivalent
    "auto_freeze_enabled": True,
    "screening_enabled": True,
    "retention_period_years": 10,
    "audit_trail_years": 7,
}

# Notification settings
NOTIFICATION_SETTINGS = {
    "webhook_enabled": False,
    "webhook_url": None,
    "email_enabled": False,
    "email_recipients": [],
    "sms_enabled": False,
    "sms_recipients": [],
    "slack_enabled": False,
    "slack_webhook": None,
}

# Performance and resource limits
PERFORMANCE_LIMITS = {
    "max_events_per_second": 100,
    "max_alerts_per_minute": 10,
    "max_concurrent_investigations": 5,
    "event_processing_timeout": 30,  # seconds
    "alert_processing_timeout": 60,  # seconds
    "cleanup_batch_size": 1000,
    "memory_limit_mb": 512,
}

# Audit trail settings
AUDIT_TRAIL_SETTINGS = {
    "enabled": True,
    "compliance_only": False,
    "include_state_changes": True,
    "include_metadata": True,
    "encrypt_sensitive_data": True,
    "retention_period_days": 365 * 7,  # 7 years
    "compliance_retention_days": 365 * 10,  # 10 years
    "cleanup_enabled": True,
    "cleanup_interval_hours": 24,
}

# Default configuration instance
DEFAULT_CONFIG = SecurityMonitoringConfig(
    enabled=True,
    retention_days=365,
    max_events_per_user=1000,
    auto_block_enabled=True,
    notification_webhook=None,
    emergency_contacts=[],
    compliance_retention_years=10,
    audit_trail_retention_years=7,
    max_concurrent_events=100,
    event_batch_size=50,
    cleanup_interval_hours=24
)

def get_security_level(event_type: EventType) -> SecurityLevel:
    """Get the security level for a given event type"""
    return SECURITY_LEVEL_MAPPING.get(event_type, SecurityLevel.LOW)

def calculate_risk_score(
    failed_logins: int,
    suspicious_activities: int,
    compliance_alerts: int,
    last_suspicious_timestamp: int = None,
    kyc_tier: int = 0,
    account_age_days: int = 0
) -> int:
    """Calculate risk score based on user behavior factors"""
    import time
    
    score = 0
    
    # Failed login attempts
    score += min(failed_logins * RISK_SCORE_WEIGHTS["failed_login_attempts"], 20)
    
    # Suspicious activity count
    score += min(suspicious_activities * RISK_SCORE_WEIGHTS["suspicious_activity"], 30)
    
    # Compliance alerts
    score += min(compliance_alerts * RISK_SCORE_WEIGHTS["compliance_alerts"], 30)
    
    # Recent suspicious activity
    if last_suspicious_timestamp:
        days_since = (int(time.time()) - last_suspicious_timestamp) // 86400
        if days_since < 7:
            score += RISK_SCORE_WEIGHTS["recent_suspicious_activity"]
        elif days_since < 30:
            score += RISK_SCORE_WEIGHTS["recent_suspicious_activity"] // 2
    
    # KYC tier bonus (higher tiers get lower risk scores)
    if kyc_tier > 0:
        score += RISK_SCORE_WEIGHTS["kyc_tier_bonus"] * kyc_tier
    
    # Account age bonus (older accounts get lower risk scores)
    if account_age_days > 180:  # 6 months
        score += RISK_SCORE_WEIGHTS["account_age_bonus"]
    
    return max(0, min(score, 100))

def get_risk_level(risk_score: int) -> str:
    """Get risk level based on risk score"""
    if risk_score >= RISK_SCORE_THRESHOLDS["critical_risk"]:
        return "critical"
    elif risk_score >= RISK_SCORE_THRESHOLDS["high_risk"]:
        return "high"
    elif risk_score >= RISK_SCORE_THRESHOLDS["medium_risk"]:
        return "medium"
    else:
        return "low"

def should_auto_block(event_type: EventType, risk_score: int) -> bool:
    """Determine if an event should trigger auto-blocking"""
    # Always block critical security violations
    if event_type == EventType.SECURITY_VIOLATION:
        return True
    
    # Block high-risk users for certain events
    if risk_score >= RISK_SCORE_THRESHOLDS["high_risk"]:
        high_risk_block_events = [
            EventType.LOGIN_FAILURE,
            EventType.TWO_FACTOR_FAILURE,
            EventType.LARGE_AMOUNT_TRANSACTION
        ]
        return event_type in high_risk_block_events
    
    return False

def get_retention_period(compliance_relevant: bool) -> int:
    """Get retention period in seconds based on compliance relevance"""
    if compliance_relevant:
        return COMPLIANCE_SETTINGS["retention_period_years"] * 365 * 24 * 3600
    else:
        return AUDIT_TRAIL_SETTINGS["retention_period_days"] * 24 * 3600

# Export configuration for use in other modules
__all__ = [
    'SecurityLevel',
    'EventType',
    'AnomalyRule',
    'SecurityMonitoringConfig',
    'DEFAULT_ANOMALY_RULES',
    'DEFAULT_CONFIG',
    'SECURITY_LEVEL_MAPPING',
    'RISK_SCORE_WEIGHTS',
    'RISK_SCORE_THRESHOLDS',
    'COMPLIANCE_SETTINGS',
    'NOTIFICATION_SETTINGS',
    'PERFORMANCE_LIMITS',
    'AUDIT_TRAIL_SETTINGS',
    'get_security_level',
    'calculate_risk_score',
    'get_risk_level',
    'should_auto_block',
    'get_retention_period'
]