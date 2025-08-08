use anchor_lang::prelude::*;
use std::collections::HashMap;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum SecurityEventType {
    // Authentication events
    LoginAttempt,
    LoginSuccess,
    LoginFailure,
    TwoFactorRequired,
    TwoFactorSuccess,
    TwoFactorFailure,
    SessionCreated,
    SessionExpired,
    SessionRevoked,
    AccountLocked,
    AccountUnlocked,
    
    // Transaction events
    BTCCommitment,
    BTCCommitmentUpdate,
    RewardClaim,
    PaymentRequest,
    PaymentProcessed,
    PaymentFailed,
    
    // Multisig events
    MultisigProposal,
    MultisigSigning,
    MultisigExecution,
    KeyRotation,
    EmergencyMode,
    
    // KYC and compliance events
    KYCSubmission,
    KYCApproval,
    KYCRejection,
    ComplianceAlert,
    AccountFrozen,
    AccountUnfrozen,
    AMLScreening,
    
    // Oracle and system events
    OracleUpdate,
    OracleFailure,
    SystemError,
    SecurityViolation,
    
    // Anomaly detection events
    UnusualLoginLocation,
    UnusualLoginTime,
    HighFrequencyTransactions,
    LargeAmountTransaction,
    SuspiciousPattern,
    VelocityAlert,
    DeviceChange,
    IPChange,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Copy)]
pub enum SecurityLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum AlertStatus {
    Active,
    Investigating,
    Resolved,
    FalsePositive,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct SecurityEvent {
    pub event_id: u64,
    pub event_type: SecurityEventType,
    pub user: Option<Pubkey>,
    pub timestamp: i64,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub device_id: Option<String>,
    pub session_id: Option<String>,
    pub transaction_id: Option<String>,
    pub amount: Option<u64>,
    pub details: String,
    pub metadata: HashMap<String, String>,
    pub security_level: SecurityLevel,
    pub requires_investigation: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct UserBehaviorProfile {
    pub user: Pubkey,
    pub created_at: i64,
    pub last_updated: i64,
    
    // Login patterns
    pub typical_login_hours: Vec<u8>, // Hours 0-23
    pub typical_login_days: Vec<u8>,  // Days 0-6 (Sunday-Saturday)
    pub common_locations: Vec<String>, // IP ranges or countries
    pub common_devices: Vec<String>,
    pub common_user_agents: Vec<String>,
    
    // Transaction patterns
    pub average_transaction_amount: u64,
    pub max_transaction_amount: u64,
    pub transaction_frequency: f64, // Transactions per day
    pub preferred_payment_methods: Vec<String>,
    
    // Risk indicators
    pub failed_login_attempts: u32,
    pub suspicious_activity_count: u32,
    pub last_suspicious_activity: Option<i64>,
    pub risk_score: u8, // 0-100
    pub is_high_risk: bool,
    
    // Compliance data
    pub kyc_tier: u8,
    pub compliance_alerts: u32,
    pub last_compliance_review: Option<i64>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct SecurityAlert {
    pub alert_id: u64,
    pub alert_type: SecurityEventType,
    pub user: Option<Pubkey>,
    pub created_at: i64,
    pub updated_at: i64,
    pub status: AlertStatus,
    pub security_level: SecurityLevel,
    pub description: String,
    pub related_events: Vec<u64>, // Event IDs
    pub investigation_notes: Vec<String>,
    pub assigned_to: Option<Pubkey>, // Security officer
    pub auto_resolved: bool,
    pub resolution_time: Option<i64>,
    pub false_positive: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct AnomalyDetectionRule {
    pub rule_id: u64,
    pub name: String,
    pub description: String,
    pub event_types: Vec<SecurityEventType>,
    pub enabled: bool,
    pub threshold_value: f64,
    pub time_window_minutes: u32,
    pub severity: SecurityLevel,
    pub auto_block: bool,
    pub notification_required: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct AuditTrail {
    pub trail_id: u64,
    pub user: Option<Pubkey>,
    pub action: String,
    pub resource: String,
    pub timestamp: i64,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub session_id: Option<String>,
    pub before_state: Option<String>, // JSON serialized state
    pub after_state: Option<String>,  // JSON serialized state
    pub success: bool,
    pub error_message: Option<String>,
    pub compliance_relevant: bool,
    pub retention_period: i64, // Seconds from creation
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct SecurityMetrics {
    pub total_events: u64,
    pub events_by_type: HashMap<String, u64>,
    pub active_alerts: u64,
    pub resolved_alerts: u64,
    pub false_positives: u64,
    pub high_risk_users: u64,
    pub blocked_transactions: u64,
    pub average_resolution_time: f64, // Minutes
    pub last_updated: i64,
}

#[account]
pub struct SecurityMonitor {
    pub authority: Pubkey,
    pub event_counter: u64,
    pub alert_counter: u64,
    pub audit_counter: u64,
    pub enabled: bool,
    pub retention_days: u32,
    pub max_events_per_user: u32,
    pub auto_block_enabled: bool,
    pub notification_webhook: Option<String>,
    pub emergency_contacts: Vec<Pubkey>,
    pub created_at: i64,
    pub last_maintenance: i64,
}

#[account]
pub struct SecurityEventLog {
    pub monitor: Pubkey,
    pub events: Vec<SecurityEvent>,
    pub max_size: u32,
    pub created_at: i64,
    pub last_updated: i64,
}

#[account]
pub struct UserBehaviorStore {
    pub monitor: Pubkey,
    pub profiles: HashMap<Pubkey, UserBehaviorProfile>,
    pub created_at: i64,
    pub last_updated: i64,
}

#[account]
pub struct SecurityAlertStore {
    pub monitor: Pubkey,
    pub alerts: Vec<SecurityAlert>,
    pub active_count: u32,
    pub resolved_count: u32,
    pub created_at: i64,
    pub last_updated: i64,
}

#[account]
pub struct AnomalyRuleStore {
    pub monitor: Pubkey,
    pub rules: Vec<AnomalyDetectionRule>,
    pub enabled_count: u32,
    pub created_at: i64,
    pub last_updated: i64,
}

#[account]
pub struct AuditTrailStore {
    pub monitor: Pubkey,
    pub trails: Vec<AuditTrail>,
    pub compliance_trails: Vec<AuditTrail>,
    pub retention_policy: i64, // Seconds
    pub created_at: i64,
    pub last_cleanup: i64,
}

impl SecurityEvent {
    pub fn new(
        event_id: u64,
        event_type: SecurityEventType,
        user: Option<Pubkey>,
        details: String,
    ) -> Self {
        Self {
            event_id,
            event_type,
            user,
            timestamp: Clock::get().unwrap().unix_timestamp,
            ip_address: None,
            user_agent: None,
            device_id: None,
            session_id: None,
            transaction_id: None,
            amount: None,
            details,
            metadata: HashMap::new(),
            security_level: SecurityLevel::Low,
            requires_investigation: false,
        }
    }

    pub fn with_context(
        mut self,
        ip_address: Option<String>,
        user_agent: Option<String>,
        device_id: Option<String>,
        session_id: Option<String>,
    ) -> Self {
        self.ip_address = ip_address;
        self.user_agent = user_agent;
        self.device_id = device_id;
        self.session_id = session_id;
        self
    }

    pub fn with_transaction(mut self, transaction_id: String, amount: Option<u64>) -> Self {
        self.transaction_id = Some(transaction_id);
        self.amount = amount;
        self
    }

    pub fn with_security_level(mut self, level: SecurityLevel) -> Self {
        self.security_level = level;
        self.requires_investigation = matches!(level, SecurityLevel::High | SecurityLevel::Critical);
        self
    }

    pub fn add_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

impl UserBehaviorProfile {
    pub fn new(user: Pubkey) -> Self {
        let now = Clock::get().unwrap().unix_timestamp;
        Self {
            user,
            created_at: now,
            last_updated: now,
            typical_login_hours: Vec::new(),
            typical_login_days: Vec::new(),
            common_locations: Vec::new(),
            common_devices: Vec::new(),
            common_user_agents: Vec::new(),
            average_transaction_amount: 0,
            max_transaction_amount: 0,
            transaction_frequency: 0.0,
            preferred_payment_methods: Vec::new(),
            failed_login_attempts: 0,
            suspicious_activity_count: 0,
            last_suspicious_activity: None,
            risk_score: 0,
            is_high_risk: false,
            kyc_tier: 0,
            compliance_alerts: 0,
            last_compliance_review: None,
        }
    }

    pub fn update_login_pattern(&mut self, hour: u8, day: u8, location: String, device: String, user_agent: String) {
        if !self.typical_login_hours.contains(&hour) {
            self.typical_login_hours.push(hour);
        }
        if !self.typical_login_days.contains(&day) {
            self.typical_login_days.push(day);
        }
        if !self.common_locations.contains(&location) {
            self.common_locations.push(location);
        }
        if !self.common_devices.contains(&device) {
            self.common_devices.push(device);
        }
        if !self.common_user_agents.contains(&user_agent) {
            self.common_user_agents.push(user_agent);
        }
        self.last_updated = Clock::get().unwrap().unix_timestamp;
    }

    pub fn update_transaction_pattern(&mut self, amount: u64, payment_method: String) {
        // Update average transaction amount
        if self.average_transaction_amount == 0 {
            self.average_transaction_amount = amount;
        } else {
            self.average_transaction_amount = (self.average_transaction_amount + amount) / 2;
        }

        // Update max transaction amount
        if amount > self.max_transaction_amount {
            self.max_transaction_amount = amount;
        }

        // Update preferred payment methods
        if !self.preferred_payment_methods.contains(&payment_method) {
            self.preferred_payment_methods.push(payment_method);
        }

        self.last_updated = Clock::get().unwrap().unix_timestamp;
    }

    pub fn calculate_risk_score(&mut self) -> u8 {
        let mut score = 0u8;

        // Failed login attempts (0-20 points)
        score += std::cmp::min(self.failed_login_attempts as u8 * 5, 20);

        // Suspicious activity count (0-30 points)
        score += std::cmp::min(self.suspicious_activity_count as u8 * 3, 30);

        // Recent suspicious activity (0-20 points)
        if let Some(last_suspicious) = self.last_suspicious_activity {
            let now = Clock::get().unwrap().unix_timestamp;
            let days_since = (now - last_suspicious) / 86400;
            if days_since < 7 {
                score += 20;
            } else if days_since < 30 {
                score += 10;
            }
        }

        // Compliance alerts (0-30 points)
        score += std::cmp::min(self.compliance_alerts as u8 * 5, 30);

        self.risk_score = std::cmp::min(score, 100);
        self.is_high_risk = self.risk_score >= 70;
        self.risk_score
    }

    pub fn is_anomalous_login(&self, hour: u8, day: u8, location: &str, device: &str) -> bool {
        let unusual_time = !self.typical_login_hours.contains(&hour) || !self.typical_login_days.contains(&day);
        let unusual_location = !self.common_locations.iter().any(|loc| loc.contains(location));
        let unusual_device = !self.common_devices.contains(&device.to_string());

        // Consider it anomalous if 2 or more factors are unusual
        [unusual_time, unusual_location, unusual_device].iter().filter(|&&x| x).count() >= 2
    }

    pub fn is_anomalous_transaction(&self, amount: u64) -> bool {
        // Transaction is anomalous if it's more than 5x the average or 2x the max
        amount > self.average_transaction_amount * 5 || amount > self.max_transaction_amount * 2
    }
}

impl SecurityAlert {
    pub fn new(
        alert_id: u64,
        alert_type: SecurityEventType,
        user: Option<Pubkey>,
        description: String,
        security_level: SecurityLevel,
    ) -> Self {
        let now = Clock::get().unwrap().unix_timestamp;
        Self {
            alert_id,
            alert_type,
            user,
            created_at: now,
            updated_at: now,
            status: AlertStatus::Active,
            security_level,
            description,
            related_events: Vec::new(),
            investigation_notes: Vec::new(),
            assigned_to: None,
            auto_resolved: false,
            resolution_time: None,
            false_positive: false,
        }
    }

    pub fn add_related_event(&mut self, event_id: u64) {
        if !self.related_events.contains(&event_id) {
            self.related_events.push(event_id);
        }
        self.updated_at = Clock::get().unwrap().unix_timestamp;
    }

    pub fn add_investigation_note(&mut self, note: String) {
        self.investigation_notes.push(note);
        self.updated_at = Clock::get().unwrap().unix_timestamp;
    }

    pub fn resolve(&mut self, false_positive: bool) {
        self.status = if false_positive { AlertStatus::FalsePositive } else { AlertStatus::Resolved };
        self.false_positive = false_positive;
        self.resolution_time = Some(Clock::get().unwrap().unix_timestamp);
        self.updated_at = Clock::get().unwrap().unix_timestamp;
    }

    pub fn assign_to(&mut self, officer: Pubkey) {
        self.assigned_to = Some(officer);
        self.status = AlertStatus::Investigating;
        self.updated_at = Clock::get().unwrap().unix_timestamp;
    }
}

impl AuditTrail {
    pub fn new(
        trail_id: u64,
        user: Option<Pubkey>,
        action: String,
        resource: String,
        success: bool,
    ) -> Self {
        Self {
            trail_id,
            user,
            action,
            resource,
            timestamp: Clock::get().unwrap().unix_timestamp,
            ip_address: None,
            user_agent: None,
            session_id: None,
            before_state: None,
            after_state: None,
            success,
            error_message: None,
            compliance_relevant: false,
            retention_period: 86400 * 365 * 7, // 7 years default
        }
    }

    pub fn with_context(
        mut self,
        ip_address: Option<String>,
        user_agent: Option<String>,
        session_id: Option<String>,
    ) -> Self {
        self.ip_address = ip_address;
        self.user_agent = user_agent;
        self.session_id = session_id;
        self
    }

    pub fn with_state_change(
        mut self,
        before_state: Option<String>,
        after_state: Option<String>,
    ) -> Self {
        self.before_state = before_state;
        self.after_state = after_state;
        self
    }

    pub fn with_error(mut self, error_message: String) -> Self {
        self.error_message = Some(error_message);
        self.success = false;
        self
    }

    pub fn mark_compliance_relevant(mut self) -> Self {
        self.compliance_relevant = true;
        self.retention_period = 86400 * 365 * 10; // 10 years for compliance
        self
    }
}
