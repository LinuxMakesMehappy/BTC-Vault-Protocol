"""
Security Monitoring and Logging Tests

Tests for Task 23: Add security monitoring and logging
- Security event logging for all sensitive operations
- Anomaly detection for unusual user behavior patterns
- Audit trail functionality for compliance and debugging
- Security monitoring tests and alert verification

Requirements: SR1, SR2, SR5
"""

import pytest
import asyncio
import json
import time
from datetime import datetime, timedelta
from concurrent.futures import ThreadPoolExecutor
from unittest.mock import Mock, patch
from typing import Dict, List, Optional, Any

# Mock Solana/Anchor imports for testing
class MockPubkey:
    def __init__(self, key: str):
        self.key = key
    
    def __str__(self):
        return self.key

class MockClock:
    @staticmethod
    def get():
        return Mock(unix_timestamp=int(time.time()))

class SecurityEventType:
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
    BTC_COMMITMENT = "BTCCommitment"
    BTC_COMMITMENT_UPDATE = "BTCCommitmentUpdate"
    REWARD_CLAIM = "RewardClaim"
    PAYMENT_REQUEST = "PaymentRequest"
    PAYMENT_PROCESSED = "PaymentProcessed"
    PAYMENT_FAILED = "PaymentFailed"
    MULTISIG_PROPOSAL = "MultisigProposal"
    MULTISIG_SIGNING = "MultisigSigning"
    MULTISIG_EXECUTION = "MultisigExecution"
    KEY_ROTATION = "KeyRotation"
    EMERGENCY_MODE = "EmergencyMode"
    KYC_SUBMISSION = "KYCSubmission"
    KYC_APPROVAL = "KYCApproval"
    KYC_REJECTION = "KYCRejection"
    COMPLIANCE_ALERT = "ComplianceAlert"
    ACCOUNT_FROZEN = "AccountFrozen"
    ACCOUNT_UNFROZEN = "AccountUnfrozen"
    AML_SCREENING = "AMLScreening"
    ORACLE_UPDATE = "OracleUpdate"
    ORACLE_FAILURE = "OracleFailure"
    SYSTEM_ERROR = "SystemError"
    SECURITY_VIOLATION = "SecurityViolation"
    UNUSUAL_LOGIN_LOCATION = "UnusualLoginLocation"
    UNUSUAL_LOGIN_TIME = "UnusualLoginTime"
    HIGH_FREQUENCY_TRANSACTIONS = "HighFrequencyTransactions"
    LARGE_AMOUNT_TRANSACTION = "LargeAmountTransaction"
    SUSPICIOUS_PATTERN = "SuspiciousPattern"
    VELOCITY_ALERT = "VelocityAlert"
    DEVICE_CHANGE = "DeviceChange"
    IP_CHANGE = "IPChange"

class SecurityLevel:
    LOW = "Low"
    MEDIUM = "Medium"
    HIGH = "High"
    CRITICAL = "Critical"

class AlertStatus:
    ACTIVE = "Active"
    INVESTIGATING = "Investigating"
    RESOLVED = "Resolved"
    FALSE_POSITIVE = "FalsePositive"

class SecurityEvent:
    def __init__(self, event_id: int, event_type: str, user: Optional[str], details: str):
        self.event_id = event_id
        self.event_type = event_type
        self.user = user
        self.timestamp = int(time.time())
        self.ip_address = None
        self.user_agent = None
        self.device_id = None
        self.session_id = None
        self.transaction_id = None
        self.amount = None
        self.details = details
        self.metadata = {}
        self.security_level = SecurityLevel.LOW
        self.requires_investigation = False

    def with_context(self, ip_address: Optional[str], user_agent: Optional[str], 
                    device_id: Optional[str], session_id: Optional[str]):
        self.ip_address = ip_address
        self.user_agent = user_agent
        self.device_id = device_id
        self.session_id = session_id
        return self

    def with_transaction(self, transaction_id: str, amount: Optional[int]):
        self.transaction_id = transaction_id
        self.amount = amount
        return self

    def with_security_level(self, level: str):
        self.security_level = level
        self.requires_investigation = level in [SecurityLevel.HIGH, SecurityLevel.CRITICAL]
        return self

    def add_metadata(self, key: str, value: str):
        self.metadata[key] = value
        return self

class UserBehaviorProfile:
    def __init__(self, user: str):
        self.user = user
        self.created_at = int(time.time())
        self.last_updated = int(time.time())
        self.typical_login_hours = []
        self.typical_login_days = []
        self.common_locations = []
        self.common_devices = []
        self.common_user_agents = []
        self.average_transaction_amount = 0
        self.max_transaction_amount = 0
        self.transaction_frequency = 0.0
        self.preferred_payment_methods = []
        self.failed_login_attempts = 0
        self.suspicious_activity_count = 0
        self.last_suspicious_activity = None
        self.risk_score = 0
        self.is_high_risk = False
        self.kyc_tier = 0
        self.compliance_alerts = 0
        self.last_compliance_review = None

    def update_login_pattern(self, hour: int, day: int, location: str, device: str, user_agent: str):
        if hour not in self.typical_login_hours:
            self.typical_login_hours.append(hour)
        if day not in self.typical_login_days:
            self.typical_login_days.append(day)
        if location not in self.common_locations:
            self.common_locations.append(location)
        if device not in self.common_devices:
            self.common_devices.append(device)
        if user_agent not in self.common_user_agents:
            self.common_user_agents.append(user_agent)
        self.last_updated = int(time.time())

    def update_transaction_pattern(self, amount: int, payment_method: str):
        if self.average_transaction_amount == 0:
            self.average_transaction_amount = amount
        else:
            self.average_transaction_amount = (self.average_transaction_amount + amount) // 2
        
        if amount > self.max_transaction_amount:
            self.max_transaction_amount = amount
        
        if payment_method not in self.preferred_payment_methods:
            self.preferred_payment_methods.append(payment_method)
        
        self.last_updated = int(time.time())

    def calculate_risk_score(self) -> int:
        score = 0
        
        # Failed login attempts (0-20 points)
        score += min(self.failed_login_attempts * 5, 20)
        
        # Suspicious activity count (0-30 points)
        score += min(self.suspicious_activity_count * 3, 30)
        
        # Recent suspicious activity (0-20 points)
        if self.last_suspicious_activity:
            days_since = (int(time.time()) - self.last_suspicious_activity) // 86400
            if days_since < 7:
                score += 20
            elif days_since < 30:
                score += 10
        
        # Compliance alerts (0-30 points)
        score += min(self.compliance_alerts * 5, 30)
        
        self.risk_score = min(score, 100)
        self.is_high_risk = self.risk_score >= 70
        return self.risk_score

    def is_anomalous_login(self, hour: int, day: int, location: str, device: str) -> bool:
        unusual_time = hour not in self.typical_login_hours or day not in self.typical_login_days
        unusual_location = not any(loc in location for loc in self.common_locations)
        unusual_device = device not in self.common_devices
        
        # Consider it anomalous if 2 or more factors are unusual
        return sum([unusual_time, unusual_location, unusual_device]) >= 2

    def is_anomalous_transaction(self, amount: int) -> bool:
        # Transaction is anomalous if it's more than 5x the average or 2x the max
        return (amount > self.average_transaction_amount * 5 or 
                amount > self.max_transaction_amount * 2)

class SecurityAlert:
    def __init__(self, alert_id: int, alert_type: str, user: Optional[str], 
                 description: str, security_level: str):
        self.alert_id = alert_id
        self.alert_type = alert_type
        self.user = user
        self.created_at = int(time.time())
        self.updated_at = int(time.time())
        self.status = AlertStatus.ACTIVE
        self.security_level = security_level
        self.description = description
        self.related_events = []
        self.investigation_notes = []
        self.assigned_to = None
        self.auto_resolved = False
        self.resolution_time = None
        self.false_positive = False

    def add_related_event(self, event_id: int):
        if event_id not in self.related_events:
            self.related_events.append(event_id)
        self.updated_at = int(time.time())

    def add_investigation_note(self, note: str):
        self.investigation_notes.append(note)
        self.updated_at = int(time.time())

    def resolve(self, false_positive: bool = False):
        self.status = AlertStatus.FALSE_POSITIVE if false_positive else AlertStatus.RESOLVED
        self.false_positive = false_positive
        self.resolution_time = int(time.time())
        self.updated_at = int(time.time())

    def assign_to(self, officer: str):
        self.assigned_to = officer
        self.status = AlertStatus.INVESTIGATING
        self.updated_at = int(time.time())

class AnomalyDetectionRule:
    def __init__(self, rule_id: int, name: str, description: str, event_types: List[str],
                 threshold_value: float, time_window_minutes: int, severity: str,
                 auto_block: bool = False):
        self.rule_id = rule_id
        self.name = name
        self.description = description
        self.event_types = event_types
        self.enabled = True
        self.threshold_value = threshold_value
        self.time_window_minutes = time_window_minutes
        self.severity = severity
        self.auto_block = auto_block
        self.notification_required = severity in [SecurityLevel.HIGH, SecurityLevel.CRITICAL]

class AuditTrail:
    def __init__(self, trail_id: int, user: Optional[str], action: str, resource: str, success: bool):
        self.trail_id = trail_id
        self.user = user
        self.action = action
        self.resource = resource
        self.timestamp = int(time.time())
        self.ip_address = None
        self.user_agent = None
        self.session_id = None
        self.before_state = None
        self.after_state = None
        self.success = success
        self.error_message = None
        self.compliance_relevant = False
        self.retention_period = 86400 * 365 * 7  # 7 years default

    def with_context(self, ip_address: Optional[str], user_agent: Optional[str], session_id: Optional[str]):
        self.ip_address = ip_address
        self.user_agent = user_agent
        self.session_id = session_id
        return self

    def with_state_change(self, before_state: Optional[str], after_state: Optional[str]):
        self.before_state = before_state
        self.after_state = after_state
        return self

    def with_error(self, error_message: str):
        self.error_message = error_message
        self.success = False
        return self

    def mark_compliance_relevant(self):
        self.compliance_relevant = True
        self.retention_period = 86400 * 365 * 10  # 10 years for compliance
        return self

class SecurityMonitoringSystem:
    def __init__(self):
        self.event_counter = 0
        self.alert_counter = 0
        self.audit_counter = 0
        self.enabled = True
        self.retention_days = 365
        self.max_events_per_user = 1000
        self.auto_block_enabled = True
        self.notification_webhook = None
        self.emergency_contacts = []
        
        self.events = []
        self.user_profiles = {}
        self.alerts = []
        self.anomaly_rules = self._create_default_rules()
        self.audit_trails = []
        self.compliance_trails = []

    def _create_default_rules(self) -> List[AnomalyDetectionRule]:
        return [
            AnomalyDetectionRule(
                1, "Multiple Failed Logins", "Detect multiple failed login attempts",
                [SecurityEventType.LOGIN_FAILURE], 5.0, 15, SecurityLevel.MEDIUM, True
            ),
            AnomalyDetectionRule(
                2, "High Value Transactions", "Detect unusually large transactions",
                [SecurityEventType.BTC_COMMITMENT, SecurityEventType.REWARD_CLAIM],
                100000.0, 60, SecurityLevel.HIGH, False
            ),
            AnomalyDetectionRule(
                3, "Rapid Transaction Velocity", "Detect high frequency transactions",
                [SecurityEventType.PAYMENT_REQUEST, SecurityEventType.REWARD_CLAIM],
                10.0, 5, SecurityLevel.MEDIUM, False
            ),
            AnomalyDetectionRule(
                4, "Security Violations", "Detect security violations",
                [SecurityEventType.SECURITY_VIOLATION], 1.0, 1, SecurityLevel.CRITICAL, True
            ),
        ]

    def log_security_event(self, event_type: str, user: Optional[str], details: str,
                          ip_address: Optional[str] = None, user_agent: Optional[str] = None,
                          device_id: Optional[str] = None, session_id: Optional[str] = None,
                          transaction_id: Optional[str] = None, amount: Optional[int] = None,
                          metadata: Optional[Dict[str, str]] = None) -> SecurityEvent:
        if not self.enabled:
            raise ValueError("Security monitoring is disabled")
        
        self.event_counter += 1
        event = SecurityEvent(self.event_counter, event_type, user, details)
        
        if ip_address or user_agent or device_id or session_id:
            event.with_context(ip_address, user_agent, device_id, session_id)
        
        if transaction_id or amount:
            event.with_transaction(transaction_id or "", amount)
        
        if metadata:
            for key, value in metadata.items():
                event.add_metadata(key, value)
        
        # Determine security level
        security_level = self._determine_security_level(event_type)
        event.with_security_level(security_level)
        
        # Check for anomalies BEFORE updating user behavior profile
        if user and user in self.user_profiles:
            profile = self.user_profiles[user]
            # Only check for anomalies if we have established some patterns
            if (len(profile.common_locations) > 0 or len(profile.common_devices) > 0 or 
                profile.average_transaction_amount > 0):
                if self._is_anomalous_behavior(profile, event, ip_address, device_id):
                    self._create_security_alert(
                        SecurityEventType.SUSPICIOUS_PATTERN, user,
                        f"Anomalous behavior detected for user: {user}",
                        SecurityLevel.MEDIUM, [event.event_id]
                    )
        
        # Update user behavior profile AFTER anomaly check
        if user:
            self._update_user_behavior_profile(user, event_type, ip_address, device_id, user_agent, amount)
        
        # Check anomaly detection rules
        self._check_anomaly_rules(event)
        
        # Add event to log
        if len(self.events) >= self.max_events_per_user:
            self.events.pop(0)  # Remove oldest event
        self.events.append(event)
        
        return event

    def create_audit_trail(self, user: Optional[str], action: str, resource: str, success: bool,
                          ip_address: Optional[str] = None, user_agent: Optional[str] = None,
                          session_id: Optional[str] = None, before_state: Optional[str] = None,
                          after_state: Optional[str] = None, error_message: Optional[str] = None,
                          compliance_relevant: bool = False) -> AuditTrail:
        self.audit_counter += 1
        
        trail = AuditTrail(self.audit_counter, user, action, resource, success)
        
        if ip_address or user_agent or session_id:
            trail.with_context(ip_address, user_agent, session_id)
        
        if before_state or after_state:
            trail.with_state_change(before_state, after_state)
        
        if error_message:
            trail.with_error(error_message)
        
        if compliance_relevant:
            trail.mark_compliance_relevant()
            self.compliance_trails.append(trail)
        
        self.audit_trails.append(trail)
        return trail

    def resolve_security_alert(self, alert_id: int, false_positive: bool = False, 
                              resolution_notes: str = "") -> bool:
        alert = next((a for a in self.alerts if a.alert_id == alert_id), None)
        if not alert:
            return False
        
        alert.resolve(false_positive)
        if resolution_notes:
            alert.add_investigation_note(resolution_notes)
        
        return True

    def assign_security_alert(self, alert_id: int, officer: str) -> bool:
        alert = next((a for a in self.alerts if a.alert_id == alert_id), None)
        if not alert:
            return False
        
        alert.assign_to(officer)
        return True

    def add_anomaly_rule(self, name: str, description: str, event_types: List[str],
                        threshold_value: float, time_window_minutes: int, severity: str,
                        auto_block: bool = False) -> AnomalyDetectionRule:
        rule_id = len(self.anomaly_rules) + 1
        rule = AnomalyDetectionRule(rule_id, name, description, event_types,
                                   threshold_value, time_window_minutes, severity, auto_block)
        self.anomaly_rules.append(rule)
        return rule

    def get_user_risk_score(self, user: str) -> int:
        if user not in self.user_profiles:
            return 0
        return self.user_profiles[user].calculate_risk_score()

    def get_security_metrics(self) -> Dict[str, Any]:
        active_alerts = sum(1 for alert in self.alerts if alert.status == AlertStatus.ACTIVE)
        resolved_alerts = sum(1 for alert in self.alerts if alert.status == AlertStatus.RESOLVED)
        false_positives = sum(1 for alert in self.alerts if alert.false_positive)
        high_risk_users = sum(1 for profile in self.user_profiles.values() if profile.is_high_risk)
        
        return {
            "total_events": len(self.events),
            "active_alerts": active_alerts,
            "resolved_alerts": resolved_alerts,
            "false_positives": false_positives,
            "high_risk_users": high_risk_users,
            "total_users": len(self.user_profiles),
            "total_audit_trails": len(self.audit_trails),
            "compliance_trails": len(self.compliance_trails),
        }

    def _determine_security_level(self, event_type: str) -> str:
        if event_type in [SecurityEventType.SECURITY_VIOLATION, SecurityEventType.EMERGENCY_MODE]:
            return SecurityLevel.CRITICAL
        elif event_type in [SecurityEventType.ACCOUNT_FROZEN, SecurityEventType.KEY_ROTATION]:
            return SecurityLevel.HIGH
        elif event_type in [SecurityEventType.LOGIN_FAILURE, SecurityEventType.TWO_FACTOR_FAILURE,
                           SecurityEventType.COMPLIANCE_ALERT, SecurityEventType.SUSPICIOUS_PATTERN]:
            return SecurityLevel.MEDIUM
        else:
            return SecurityLevel.LOW

    def _update_user_behavior_profile(self, user: str, event_type: str, ip_address: Optional[str],
                                     device_id: Optional[str], user_agent: Optional[str], amount: Optional[int]):
        if user not in self.user_profiles:
            self.user_profiles[user] = UserBehaviorProfile(user)
        
        profile = self.user_profiles[user]
        now = int(time.time())
        hour = (now % 86400) // 3600
        day = (now // 86400 + 4) % 7  # Unix epoch was Thursday
        
        if event_type == SecurityEventType.LOGIN_SUCCESS:
            if ip_address and device_id and user_agent:
                profile.update_login_pattern(hour, day, ip_address, device_id, user_agent)
        elif event_type == SecurityEventType.LOGIN_FAILURE:
            profile.failed_login_attempts += 1
        elif event_type in [SecurityEventType.PAYMENT_REQUEST, SecurityEventType.REWARD_CLAIM]:
            if amount:
                profile.update_transaction_pattern(amount, "BTC")
        elif event_type in [SecurityEventType.SUSPICIOUS_PATTERN, SecurityEventType.SECURITY_VIOLATION]:
            profile.suspicious_activity_count += 1
            profile.last_suspicious_activity = now
        elif event_type == SecurityEventType.COMPLIANCE_ALERT:
            profile.compliance_alerts += 1
        
        # Always recalculate risk score after updates
        profile.calculate_risk_score()

    def _is_anomalous_behavior(self, profile: UserBehaviorProfile, event: SecurityEvent,
                              ip_address: Optional[str], device_id: Optional[str]) -> bool:
        now = int(time.time())
        hour = (now % 86400) // 3600
        day = (now // 86400 + 4) % 7
        
        if event.event_type == SecurityEventType.LOGIN_SUCCESS:
            if ip_address and device_id:
                return profile.is_anomalous_login(hour, day, ip_address, device_id)
        elif event.event_type in [SecurityEventType.PAYMENT_REQUEST, SecurityEventType.REWARD_CLAIM]:
            if event.amount:
                return profile.is_anomalous_transaction(event.amount)
        
        return False

    def _check_anomaly_rules(self, event: SecurityEvent):
        for rule in self.anomaly_rules:
            if not rule.enabled or event.event_type not in rule.event_types:
                continue
            
            should_trigger = False
            
            # Check different rule types
            if event.event_type == SecurityEventType.SECURITY_VIOLATION:
                should_trigger = True
            elif event.event_type == SecurityEventType.LOGIN_FAILURE:
                # Count recent failed logins for this user
                if event.user:
                    recent_failures = sum(1 for e in self.events 
                                        if e.user == event.user and 
                                           e.event_type == SecurityEventType.LOGIN_FAILURE and
                                           (event.timestamp - e.timestamp) <= rule.time_window_minutes * 60)
                    should_trigger = recent_failures >= rule.threshold_value
            elif event.amount:
                should_trigger = event.amount >= rule.threshold_value
            
            if should_trigger:
                self._create_security_alert(
                    event.event_type, event.user,
                    f"Anomaly rule triggered: {rule.name}",
                    rule.severity, [event.event_id]
                )

    def _create_security_alert(self, alert_type: str, user: Optional[str], description: str,
                              security_level: str, related_events: List[int]):
        self.alert_counter += 1
        alert = SecurityAlert(self.alert_counter, alert_type, user, description, security_level)
        
        for event_id in related_events:
            alert.add_related_event(event_id)
        
        self.alerts.append(alert)

class TestSecurityMonitoring:
    """Test suite for security monitoring and logging functionality"""
    
    @pytest.fixture
    def security_system(self):
        """Create a fresh security monitoring system for each test"""
        return SecurityMonitoringSystem()
    
    @pytest.fixture
    def sample_user(self):
        """Sample user for testing"""
        return "user123"
    
    @pytest.fixture
    def sample_context(self):
        """Sample context data for testing"""
        return {
            "ip_address": "192.168.1.100",
            "user_agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
            "device_id": "device_abc123",
            "session_id": "session_xyz789"
        }

    def test_initialize_security_monitor(self, security_system):
        """Test security monitor initialization"""
        assert security_system.enabled is True
        assert security_system.retention_days == 365
        assert security_system.max_events_per_user == 1000
        assert security_system.auto_block_enabled is True
        assert len(security_system.anomaly_rules) == 4  # Default rules
        assert security_system.event_counter == 0
        assert security_system.alert_counter == 0
        assert security_system.audit_counter == 0

    def test_log_security_event_basic(self, security_system, sample_user):
        """Test basic security event logging"""
        event = security_system.log_security_event(
            SecurityEventType.LOGIN_SUCCESS,
            sample_user,
            "User logged in successfully"
        )
        
        assert event.event_id == 1
        assert event.event_type == SecurityEventType.LOGIN_SUCCESS
        assert event.user == sample_user
        assert event.details == "User logged in successfully"
        assert event.security_level == SecurityLevel.LOW
        assert len(security_system.events) == 1

    def test_log_security_event_with_context(self, security_system, sample_user, sample_context):
        """Test security event logging with context information"""
        event = security_system.log_security_event(
            SecurityEventType.LOGIN_SUCCESS,
            sample_user,
            "User logged in successfully",
            **sample_context
        )
        
        assert event.ip_address == sample_context["ip_address"]
        assert event.user_agent == sample_context["user_agent"]
        assert event.device_id == sample_context["device_id"]
        assert event.session_id == sample_context["session_id"]

    def test_log_security_event_with_transaction(self, security_system, sample_user):
        """Test security event logging with transaction information"""
        event = security_system.log_security_event(
            SecurityEventType.BTC_COMMITMENT,
            sample_user,
            "User committed BTC",
            transaction_id="tx_123",
            amount=50000  # $50k
        )
        
        assert event.transaction_id == "tx_123"
        assert event.amount == 50000
        assert event.security_level == SecurityLevel.LOW

    def test_log_security_event_with_metadata(self, security_system, sample_user):
        """Test security event logging with metadata"""
        metadata = {"btc_address": "bc1q...", "commitment_type": "initial"}
        
        event = security_system.log_security_event(
            SecurityEventType.BTC_COMMITMENT,
            sample_user,
            "User committed BTC",
            metadata=metadata
        )
        
        assert event.metadata == metadata

    def test_security_level_determination(self, security_system, sample_user):
        """Test automatic security level determination"""
        # Critical level event
        critical_event = security_system.log_security_event(
            SecurityEventType.SECURITY_VIOLATION,
            sample_user,
            "Security violation detected"
        )
        assert critical_event.security_level == SecurityLevel.CRITICAL
        assert critical_event.requires_investigation is True
        
        # High level event
        high_event = security_system.log_security_event(
            SecurityEventType.EMERGENCY_MODE,
            sample_user,
            "Emergency mode activated"
        )
        assert high_event.security_level == SecurityLevel.HIGH
        assert high_event.requires_investigation is True
        
        # Medium level event
        medium_event = security_system.log_security_event(
            SecurityEventType.LOGIN_FAILURE,
            sample_user,
            "Login failed"
        )
        assert medium_event.security_level == SecurityLevel.MEDIUM
        
        # Low level event
        low_event = security_system.log_security_event(
            SecurityEventType.LOGIN_SUCCESS,
            sample_user,
            "Login successful"
        )
        assert low_event.security_level == SecurityLevel.LOW

    def test_user_behavior_profile_creation(self, security_system, sample_user, sample_context):
        """Test user behavior profile creation and updates"""
        # Log a login event to create profile
        security_system.log_security_event(
            SecurityEventType.LOGIN_SUCCESS,
            sample_user,
            "User logged in",
            **sample_context
        )
        
        assert sample_user in security_system.user_profiles
        profile = security_system.user_profiles[sample_user]
        assert profile.user == sample_user
        assert len(profile.common_locations) == 1
        assert len(profile.common_devices) == 1
        assert len(profile.common_user_agents) == 1

    def test_user_behavior_profile_login_pattern(self, security_system, sample_user, sample_context):
        """Test user behavior profile login pattern tracking"""
        # Log multiple login events
        for i in range(3):
            security_system.log_security_event(
                SecurityEventType.LOGIN_SUCCESS,
                sample_user,
                f"Login {i}",
                **sample_context
            )
        
        profile = security_system.user_profiles[sample_user]
        assert len(profile.typical_login_hours) >= 1
        assert len(profile.typical_login_days) >= 1
        assert sample_context["ip_address"] in profile.common_locations
        assert sample_context["device_id"] in profile.common_devices

    def test_user_behavior_profile_transaction_pattern(self, security_system, sample_user):
        """Test user behavior profile transaction pattern tracking"""
        # Log transaction events
        amounts = [1000, 2000, 3000]
        for amount in amounts:
            security_system.log_security_event(
                SecurityEventType.PAYMENT_REQUEST,
                sample_user,
                f"Payment request for {amount}",
                amount=amount
            )
        
        profile = security_system.user_profiles[sample_user]
        assert profile.average_transaction_amount > 0
        assert profile.max_transaction_amount == 3000
        assert "BTC" in profile.preferred_payment_methods

    def test_user_behavior_profile_risk_calculation(self, security_system, sample_user):
        """Test user behavior profile risk score calculation"""
        # Create profile with some risk factors
        profile = UserBehaviorProfile(sample_user)
        profile.failed_login_attempts = 3
        profile.suspicious_activity_count = 2
        profile.compliance_alerts = 1
        profile.last_suspicious_activity = int(time.time()) - 86400  # 1 day ago
        
        risk_score = profile.calculate_risk_score()
        
        # Expected: 3*5 + 2*3 + 1*5 + 20 = 15 + 6 + 5 + 20 = 46
        assert risk_score == 46
        assert profile.is_high_risk is False
        
        # Test high risk scenario
        profile.failed_login_attempts = 4  # 20 points
        profile.suspicious_activity_count = 10  # 30 points (capped)
        profile.compliance_alerts = 6  # 30 points (capped)
        
        risk_score = profile.calculate_risk_score()
        assert risk_score >= 70
        assert profile.is_high_risk is True

    def test_anomalous_login_detection(self, security_system, sample_user, sample_context):
        """Test anomalous login behavior detection"""
        # Establish normal pattern
        for _ in range(5):
            security_system.log_security_event(
                SecurityEventType.LOGIN_SUCCESS,
                sample_user,
                "Normal login",
                **sample_context
            )
        
        # Log anomalous login (different IP, device, and user agent)
        anomalous_context = {
            "ip_address": "10.0.0.1",  # Different IP
            "device_id": "different_device",  # Different device
            "user_agent": "Different User Agent",  # Different user agent
            "session_id": "session_new"
        }
        
        security_system.log_security_event(
            SecurityEventType.LOGIN_SUCCESS,
            sample_user,
            "Anomalous login",
            **anomalous_context
        )
        
        # Should create a suspicious pattern alert
        suspicious_alerts = [a for a in security_system.alerts 
                           if a.alert_type == SecurityEventType.SUSPICIOUS_PATTERN]
        assert len(suspicious_alerts) >= 1

    def test_anomalous_transaction_detection(self, security_system, sample_user):
        """Test anomalous transaction behavior detection"""
        # Establish normal transaction pattern
        normal_amounts = [1000, 1200, 800, 1100, 900]
        for amount in normal_amounts:
            security_system.log_security_event(
                SecurityEventType.PAYMENT_REQUEST,
                sample_user,
                f"Normal payment {amount}",
                amount=amount
            )
        
        # Log anomalous transaction (much larger than normal)
        security_system.log_security_event(
            SecurityEventType.PAYMENT_REQUEST,
            sample_user,
            "Large payment",
            amount=50000  # Much larger than normal
        )
        
        # Should create a suspicious pattern alert
        suspicious_alerts = [a for a in security_system.alerts 
                           if a.alert_type == SecurityEventType.SUSPICIOUS_PATTERN]
        assert len(suspicious_alerts) >= 1

    def test_anomaly_rule_triggering(self, security_system, sample_user):
        """Test anomaly detection rule triggering"""
        # Trigger multiple failed logins rule
        for i in range(6):  # Threshold is 5
            security_system.log_security_event(
                SecurityEventType.LOGIN_FAILURE,
                sample_user,
                f"Failed login attempt {i}"
            )
        
        # Should create alert for multiple failed logins
        failed_login_alerts = [a for a in security_system.alerts 
                              if SecurityEventType.LOGIN_FAILURE in a.alert_type]
        assert len(failed_login_alerts) >= 1

    def test_high_value_transaction_rule(self, security_system, sample_user):
        """Test high value transaction anomaly rule"""
        # Log high value transaction
        security_system.log_security_event(
            SecurityEventType.BTC_COMMITMENT,
            sample_user,
            "High value BTC commitment",
            amount=150000  # Above $100k threshold
        )
        
        # Should create alert for high value transaction
        high_value_alerts = [a for a in security_system.alerts 
                           if a.alert_type == SecurityEventType.BTC_COMMITMENT]
        assert len(high_value_alerts) >= 1
        assert high_value_alerts[0].security_level == SecurityLevel.HIGH

    def test_security_violation_rule(self, security_system, sample_user):
        """Test security violation rule triggering"""
        security_system.log_security_event(
            SecurityEventType.SECURITY_VIOLATION,
            sample_user,
            "Security violation detected"
        )
        
        # Should create critical alert
        violation_alerts = [a for a in security_system.alerts 
                          if a.alert_type == SecurityEventType.SECURITY_VIOLATION]
        assert len(violation_alerts) >= 1
        assert violation_alerts[0].security_level == SecurityLevel.CRITICAL

    def test_audit_trail_creation(self, security_system, sample_user, sample_context):
        """Test audit trail creation"""
        trail = security_system.create_audit_trail(
            sample_user,
            "UPDATE_PROFILE",
            "user_profile",
            True,
            ip_address=sample_context["ip_address"],
            user_agent=sample_context["user_agent"],
            session_id=sample_context["session_id"],
            before_state='{"name": "old"}',
            after_state='{"name": "new"}'
        )
        
        assert trail.trail_id == 1
        assert trail.user == sample_user
        assert trail.action == "UPDATE_PROFILE"
        assert trail.resource == "user_profile"
        assert trail.success is True
        assert trail.ip_address == sample_context["ip_address"]
        assert trail.before_state == '{"name": "old"}'
        assert trail.after_state == '{"name": "new"}'
        assert len(security_system.audit_trails) == 1

    def test_audit_trail_with_error(self, security_system, sample_user):
        """Test audit trail creation with error"""
        trail = security_system.create_audit_trail(
            sample_user,
            "DELETE_ACCOUNT",
            "user_account",
            False,
            error_message="Insufficient permissions"
        )
        
        assert trail.success is False
        assert trail.error_message == "Insufficient permissions"

    def test_compliance_audit_trail(self, security_system, sample_user):
        """Test compliance-relevant audit trail"""
        trail = security_system.create_audit_trail(
            sample_user,
            "KYC_VERIFICATION",
            "kyc_document",
            True,
            compliance_relevant=True
        )
        
        assert trail.compliance_relevant is True
        assert trail.retention_period == 86400 * 365 * 10  # 10 years
        assert len(security_system.compliance_trails) == 1

    def test_security_alert_management(self, security_system, sample_user):
        """Test security alert creation and management"""
        # Create an alert by triggering a rule
        security_system.log_security_event(
            SecurityEventType.SECURITY_VIOLATION,
            sample_user,
            "Test violation"
        )
        
        assert len(security_system.alerts) == 1
        alert = security_system.alerts[0]
        assert alert.status == AlertStatus.ACTIVE
        
        # Assign alert to security officer
        officer = "security_officer_1"
        success = security_system.assign_security_alert(alert.alert_id, officer)
        assert success is True
        assert alert.assigned_to == officer
        assert alert.status == AlertStatus.INVESTIGATING
        
        # Resolve alert
        success = security_system.resolve_security_alert(
            alert.alert_id, 
            false_positive=False, 
            resolution_notes="Investigated and resolved"
        )
        assert success is True
        assert alert.status == AlertStatus.RESOLVED
        assert "Investigated and resolved" in alert.investigation_notes

    def test_security_alert_false_positive(self, security_system, sample_user):
        """Test marking security alert as false positive"""
        # Create an alert
        security_system.log_security_event(
            SecurityEventType.SECURITY_VIOLATION,
            sample_user,
            "Test violation"
        )
        
        alert = security_system.alerts[0]
        
        # Mark as false positive
        success = security_system.resolve_security_alert(
            alert.alert_id, 
            false_positive=True,
            resolution_notes="False positive - legitimate activity"
        )
        
        assert success is True
        assert alert.status == AlertStatus.FALSE_POSITIVE
        assert alert.false_positive is True

    def test_custom_anomaly_rule(self, security_system):
        """Test adding custom anomaly detection rule"""
        rule = security_system.add_anomaly_rule(
            "Custom Rule",
            "Detect custom pattern",
            [SecurityEventType.PAYMENT_REQUEST],
            5000.0,  # $5k threshold
            30,      # 30 minute window
            SecurityLevel.MEDIUM,
            auto_block=True
        )
        
        assert rule.rule_id == 5  # After 4 default rules
        assert rule.name == "Custom Rule"
        assert rule.threshold_value == 5000.0
        assert rule.auto_block is True
        assert len(security_system.anomaly_rules) == 5

    def test_security_metrics(self, security_system, sample_user):
        """Test security metrics calculation"""
        # Generate some activity
        security_system.log_security_event(
            SecurityEventType.LOGIN_SUCCESS, sample_user, "Login"
        )
        security_system.log_security_event(
            SecurityEventType.SECURITY_VIOLATION, sample_user, "Violation"
        )
        security_system.create_audit_trail(
            sample_user, "ACTION", "resource", True, compliance_relevant=True
        )
        
        metrics = security_system.get_security_metrics()
        
        assert metrics["total_events"] == 2
        assert metrics["active_alerts"] >= 1
        assert metrics["total_users"] == 1
        assert metrics["total_audit_trails"] == 1
        assert metrics["compliance_trails"] == 1

    def test_concurrent_event_logging(self, security_system):
        """Test concurrent security event logging"""
        def log_events(user_id: str, count: int):
            events = []
            for i in range(count):
                event = security_system.log_security_event(
                    SecurityEventType.LOGIN_SUCCESS,
                    f"user_{user_id}",
                    f"Login {i}"
                )
                events.append(event)
            return events
        
        # Log events concurrently from multiple threads
        with ThreadPoolExecutor(max_workers=5) as executor:
            futures = []
            for user_id in range(5):
                future = executor.submit(log_events, str(user_id), 10)
                futures.append(future)
            
            # Wait for all threads to complete
            results = [future.result() for future in futures]
        
        # Verify all events were logged
        total_events = sum(len(events) for events in results)
        assert total_events == 50
        assert len(security_system.events) == 50
        assert len(security_system.user_profiles) == 5

    def test_event_retention_limit(self, security_system, sample_user):
        """Test event retention limit enforcement"""
        # Set low limit for testing
        security_system.max_events_per_user = 5
        
        # Log more events than the limit
        for i in range(10):
            security_system.log_security_event(
                SecurityEventType.LOGIN_SUCCESS,
                sample_user,
                f"Login {i}"
            )
        
        # Should only keep the most recent events
        assert len(security_system.events) == 5
        # Last event should be "Login 9"
        assert security_system.events[-1].details == "Login 9"

    def test_disabled_security_monitoring(self, security_system, sample_user):
        """Test behavior when security monitoring is disabled"""
        security_system.enabled = False
        
        with pytest.raises(ValueError, match="Security monitoring is disabled"):
            security_system.log_security_event(
                SecurityEventType.LOGIN_SUCCESS,
                sample_user,
                "Login attempt"
            )

    @pytest.mark.asyncio
    async def test_async_event_processing(self, security_system):
        """Test asynchronous event processing"""
        async def log_async_event(user: str, event_type: str, details: str):
            # Simulate async processing
            await asyncio.sleep(0.01)
            return security_system.log_security_event(event_type, user, details)
        
        # Process multiple events asynchronously
        tasks = []
        for i in range(10):
            task = log_async_event(
                f"user_{i % 3}",
                SecurityEventType.LOGIN_SUCCESS,
                f"Async login {i}"
            )
            tasks.append(task)
        
        events = await asyncio.gather(*tasks)
        
        assert len(events) == 10
        assert len(security_system.events) == 10
        assert len(security_system.user_profiles) == 3

    def test_security_event_serialization(self, security_system, sample_user):
        """Test security event data serialization for storage"""
        event = security_system.log_security_event(
            SecurityEventType.BTC_COMMITMENT,
            sample_user,
            "BTC commitment",
            ip_address="192.168.1.1",
            transaction_id="tx_123",
            amount=25000,
            metadata={"btc_address": "bc1q..."}
        )
        
        # Convert to dict for serialization
        event_dict = {
            "event_id": event.event_id,
            "event_type": event.event_type,
            "user": event.user,
            "timestamp": event.timestamp,
            "ip_address": event.ip_address,
            "transaction_id": event.transaction_id,
            "amount": event.amount,
            "details": event.details,
            "metadata": event.metadata,
            "security_level": event.security_level
        }
        
        # Verify serialization
        json_str = json.dumps(event_dict)
        deserialized = json.loads(json_str)
        
        assert deserialized["event_type"] == SecurityEventType.BTC_COMMITMENT
        assert deserialized["user"] == sample_user
        assert deserialized["amount"] == 25000

    def test_compliance_reporting(self, security_system, sample_user):
        """Test compliance reporting functionality"""
        # Create compliance-relevant events and trails
        security_system.log_security_event(
            SecurityEventType.KYC_SUBMISSION,
            sample_user,
            "KYC documents submitted"
        )
        
        security_system.create_audit_trail(
            sample_user,
            "KYC_APPROVAL",
            "kyc_document",
            True,
            compliance_relevant=True
        )
        
        security_system.create_audit_trail(
            sample_user,
            "LARGE_TRANSACTION",
            "btc_commitment",
            True,
            compliance_relevant=True,
            before_state='{"amount": 0}',
            after_state='{"amount": 100000}'
        )
        
        # Generate compliance report
        compliance_events = [e for e in security_system.events 
                           if e.event_type in [SecurityEventType.KYC_SUBMISSION,
                                             SecurityEventType.KYC_APPROVAL,
                                             SecurityEventType.COMPLIANCE_ALERT]]
        
        compliance_trails = security_system.compliance_trails
        
        assert len(compliance_events) == 1
        assert len(compliance_trails) == 2
        
        # Verify retention periods
        for trail in compliance_trails:
            assert trail.retention_period == 86400 * 365 * 10  # 10 years

if __name__ == "__main__":
    # Run the tests
    pytest.main([__file__, "-v", "--tb=short"])