use anchor_lang::prelude::*;
use crate::state::security_monitoring::*;
use crate::errors::VaultError;
use std::collections::HashMap;

#[derive(Accounts)]
pub struct InitializeSecurityMonitor<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + SecurityMonitor::MAX_SIZE,
        seeds = [b"security_monitor"],
        bump
    )]
    pub security_monitor: Account<'info, SecurityMonitor>,
    
    #[account(
        init,
        payer = authority,
        space = 8 + SecurityEventLog::MAX_SIZE,
        seeds = [b"security_events", security_monitor.key().as_ref()],
        bump
    )]
    pub event_log: Account<'info, SecurityEventLog>,
    
    #[account(
        init,
        payer = authority,
        space = 8 + UserBehaviorStore::MAX_SIZE,
        seeds = [b"user_behavior", security_monitor.key().as_ref()],
        bump
    )]
    pub behavior_store: Account<'info, UserBehaviorStore>,
    
    #[account(
        init,
        payer = authority,
        space = 8 + SecurityAlertStore::MAX_SIZE,
        seeds = [b"security_alerts", security_monitor.key().as_ref()],
        bump
    )]
    pub alert_store: Account<'info, SecurityAlertStore>,
    
    #[account(
        init,
        payer = authority,
        space = 8 + AnomalyRuleStore::MAX_SIZE,
        seeds = [b"anomaly_rules", security_monitor.key().as_ref()],
        bump
    )]
    pub rule_store: Account<'info, AnomalyRuleStore>,
    
    #[account(
        init,
        payer = authority,
        space = 8 + AuditTrailStore::MAX_SIZE,
        seeds = [b"audit_trail", security_monitor.key().as_ref()],
        bump
    )]
    pub audit_store: Account<'info, AuditTrailStore>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct LogSecurityEvent<'info> {
    #[account(
        mut,
        seeds = [b"security_monitor"],
        bump
    )]
    pub security_monitor: Account<'info, SecurityMonitor>,
    
    #[account(
        mut,
        seeds = [b"security_events", security_monitor.key().as_ref()],
        bump
    )]
    pub event_log: Account<'info, SecurityEventLog>,
    
    #[account(
        mut,
        seeds = [b"user_behavior", security_monitor.key().as_ref()],
        bump
    )]
    pub behavior_store: Account<'info, UserBehaviorStore>,
    
    #[account(
        mut,
        seeds = [b"security_alerts", security_monitor.key().as_ref()],
        bump
    )]
    pub alert_store: Account<'info, SecurityAlertStore>,
    
    #[account(
        mut,
        seeds = [b"anomaly_rules", security_monitor.key().as_ref()],
        bump
    )]
    pub rule_store: Account<'info, AnomalyRuleStore>,
    
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct CreateAuditTrail<'info> {
    #[account(
        mut,
        seeds = [b"security_monitor"],
        bump
    )]
    pub security_monitor: Account<'info, SecurityMonitor>,
    
    #[account(
        mut,
        seeds = [b"audit_trail", security_monitor.key().as_ref()],
        bump
    )]
    pub audit_store: Account<'info, AuditTrailStore>,
    
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct ManageSecurityAlert<'info> {
    #[account(
        mut,
        seeds = [b"security_monitor"],
        bump
    )]
    pub security_monitor: Account<'info, SecurityMonitor>,
    
    #[account(
        mut,
        seeds = [b"security_alerts", security_monitor.key().as_ref()],
        bump
    )]
    pub alert_store: Account<'info, SecurityAlertStore>,
    
    pub security_officer: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateAnomalyRules<'info> {
    #[account(
        mut,
        seeds = [b"security_monitor"],
        bump,
        has_one = authority
    )]
    pub security_monitor: Account<'info, SecurityMonitor>,
    
    #[account(
        mut,
        seeds = [b"anomaly_rules", security_monitor.key().as_ref()],
        bump
    )]
    pub rule_store: Account<'info, AnomalyRuleStore>,
    
    pub authority: Signer<'info>,
}

impl SecurityMonitor {
    pub const MAX_SIZE: usize = 32 + 8 + 8 + 8 + 1 + 4 + 4 + 1 + 4 + 100 + 32 * 10 + 8 + 8; // ~500 bytes
}

impl SecurityEventLog {
    pub const MAX_SIZE: usize = 32 + 4 + (SecurityEvent::MAX_SIZE * 1000) + 4 + 8 + 8; // ~1MB for 1000 events
}

impl SecurityEvent {
    pub const MAX_SIZE: usize = 8 + 1 + 33 + 8 + 4 + 100 + 4 + 100 + 4 + 50 + 4 + 50 + 4 + 50 + 8 + 4 + 200 + 4 + (50 * 10) + 1 + 1; // ~1KB per event
}

impl UserBehaviorStore {
    pub const MAX_SIZE: usize = 32 + 4 + (UserBehaviorProfile::MAX_SIZE * 100) + 8 + 8; // ~500KB for 100 users
}

impl UserBehaviorProfile {
    pub const MAX_SIZE: usize = 32 + 8 + 8 + 4 + (1 * 24) + 4 + (1 * 7) + 4 + (50 * 10) + 4 + (50 * 5) + 4 + (100 * 5) + 8 + 8 + 8 + 4 + (50 * 5) + 4 + 4 + 8 + 1 + 1 + 4 + 8; // ~5KB per profile
}

impl SecurityAlertStore {
    pub const MAX_SIZE: usize = 32 + 4 + (SecurityAlert::MAX_SIZE * 500) + 4 + 4 + 8 + 8; // ~500KB for 500 alerts
}

impl SecurityAlert {
    pub const MAX_SIZE: usize = 8 + 1 + 33 + 8 + 8 + 1 + 1 + 4 + 200 + 4 + (8 * 50) + 4 + (200 * 10) + 33 + 1 + 8 + 1; // ~1KB per alert
}

impl AnomalyRuleStore {
    pub const MAX_SIZE: usize = 32 + 4 + (AnomalyDetectionRule::MAX_SIZE * 50) + 4 + 8 + 8; // ~50KB for 50 rules
}

impl AnomalyDetectionRule {
    pub const MAX_SIZE: usize = 8 + 4 + 100 + 4 + 200 + 4 + (1 * 20) + 1 + 8 + 4 + 1 + 1 + 1; // ~1KB per rule
}

impl AuditTrailStore {
    pub const MAX_SIZE: usize = 32 + 4 + (AuditTrail::MAX_SIZE * 1000) + 4 + (AuditTrail::MAX_SIZE * 100) + 8 + 8 + 8; // ~1MB for 1000 trails
}

impl AuditTrail {
    pub const MAX_SIZE: usize = 8 + 33 + 4 + 100 + 4 + 100 + 8 + 4 + 100 + 4 + 100 + 4 + 50 + 4 + 1000 + 4 + 1000 + 1 + 4 + 200 + 1 + 8; // ~1KB per trail
}

pub fn initialize_security_monitor(ctx: Context<InitializeSecurityMonitor>) -> Result<()> {
    let security_monitor = &mut ctx.accounts.security_monitor;
    let event_log = &mut ctx.accounts.event_log;
    let behavior_store = &mut ctx.accounts.behavior_store;
    let alert_store = &mut ctx.accounts.alert_store;
    let rule_store = &mut ctx.accounts.rule_store;
    let audit_store = &mut ctx.accounts.audit_store;
    
    let now = Clock::get()?.unix_timestamp;
    
    // Initialize security monitor
    security_monitor.authority = ctx.accounts.authority.key();
    security_monitor.event_counter = 0;
    security_monitor.alert_counter = 0;
    security_monitor.audit_counter = 0;
    security_monitor.enabled = true;
    security_monitor.retention_days = 365; // 1 year default
    security_monitor.max_events_per_user = 1000;
    security_monitor.auto_block_enabled = true;
    security_monitor.notification_webhook = None;
    security_monitor.emergency_contacts = Vec::new();
    security_monitor.created_at = now;
    security_monitor.last_maintenance = now;
    
    // Initialize event log
    event_log.monitor = security_monitor.key();
    event_log.events = Vec::new();
    event_log.max_size = 1000;
    event_log.created_at = now;
    event_log.last_updated = now;
    
    // Initialize behavior store
    behavior_store.monitor = security_monitor.key();
    behavior_store.profiles = HashMap::new();
    behavior_store.created_at = now;
    behavior_store.last_updated = now;
    
    // Initialize alert store
    alert_store.monitor = security_monitor.key();
    alert_store.alerts = Vec::new();
    alert_store.active_count = 0;
    alert_store.resolved_count = 0;
    alert_store.created_at = now;
    alert_store.last_updated = now;
    
    // Initialize rule store with default rules
    rule_store.monitor = security_monitor.key();
    rule_store.rules = create_default_anomaly_rules();
    rule_store.enabled_count = rule_store.rules.len() as u32;
    rule_store.created_at = now;
    rule_store.last_updated = now;
    
    // Initialize audit store
    audit_store.monitor = security_monitor.key();
    audit_store.trails = Vec::new();
    audit_store.compliance_trails = Vec::new();
    audit_store.retention_policy = 86400 * 365 * 7; // 7 years
    audit_store.created_at = now;
    audit_store.last_cleanup = now;
    
    Ok(())
}

pub fn log_security_event(
    ctx: Context<LogSecurityEvent>,
    event_type: SecurityEventType,
    user: Option<Pubkey>,
    details: String,
    ip_address: Option<String>,
    user_agent: Option<String>,
    device_id: Option<String>,
    session_id: Option<String>,
    transaction_id: Option<String>,
    amount: Option<u64>,
    metadata: HashMap<String, String>,
) -> Result<()> {
    let security_monitor = &mut ctx.accounts.security_monitor;
    let event_log = &mut ctx.accounts.event_log;
    let behavior_store = &mut ctx.accounts.behavior_store;
    let alert_store = &mut ctx.accounts.alert_store;
    let rule_store = &ctx.accounts.rule_store;
    
    require!(security_monitor.enabled, VaultError::SecurityViolation);
    
    // Create security event
    security_monitor.event_counter += 1;
    let mut event = SecurityEvent::new(
        security_monitor.event_counter,
        event_type.clone(),
        user,
        details,
    )
    .with_context(ip_address.clone(), user_agent.clone(), device_id.clone(), session_id.clone());
    
    if let Some(tx_id) = transaction_id {
        event = event.with_transaction(tx_id, amount);
    }
    
    for (key, value) in metadata {
        event = event.add_metadata(key, value);
    }
    
    // Determine security level based on event type
    let security_level = determine_security_level(&event_type);
    event = event.with_security_level(security_level);
    
    // Update user behavior profile if user is present
    if let Some(user_key) = user {
        update_user_behavior_profile(
            behavior_store,
            user_key,
            &event_type,
            &ip_address,
            &device_id,
            &user_agent,
            amount,
        )?;
        
        // Check for anomalies
        if let Some(profile) = behavior_store.profiles.get(&user_key) {
            if is_anomalous_behavior(profile, &event, &ip_address, &device_id) {
                create_security_alert(
                    security_monitor,
                    alert_store,
                    SecurityEventType::SuspiciousPattern,
                    Some(user_key),
                    format!("Anomalous behavior detected for user: {}", user_key),
                    SecurityLevel::Medium,
                    vec![event.event_id],
                )?;
            }
        }
    }
    
    // Check anomaly detection rules
    check_anomaly_rules(
        security_monitor,
        alert_store,
        rule_store,
        &event,
    )?;
    
    // Add event to log
    if event_log.events.len() >= event_log.max_size as usize {
        event_log.events.remove(0); // Remove oldest event
    }
    event_log.events.push(event);
    event_log.last_updated = Clock::get()?.unix_timestamp;
    
    Ok(())
}

pub fn create_audit_trail(
    ctx: Context<CreateAuditTrail>,
    user: Option<Pubkey>,
    action: String,
    resource: String,
    success: bool,
    ip_address: Option<String>,
    user_agent: Option<String>,
    session_id: Option<String>,
    before_state: Option<String>,
    after_state: Option<String>,
    error_message: Option<String>,
    compliance_relevant: bool,
) -> Result<()> {
    let security_monitor = &mut ctx.accounts.security_monitor;
    let audit_store = &mut ctx.accounts.audit_store;
    
    security_monitor.audit_counter += 1;
    
    let mut trail = AuditTrail::new(
        security_monitor.audit_counter,
        user,
        action,
        resource,
        success,
    )
    .with_context(ip_address, user_agent, session_id)
    .with_state_change(before_state, after_state);
    
    if let Some(error) = error_message {
        trail = trail.with_error(error);
    }
    
    if compliance_relevant {
        trail = trail.mark_compliance_relevant();
        audit_store.compliance_trails.push(trail.clone());
    }
    
    audit_store.trails.push(trail);
    
    Ok(())
}

pub fn resolve_security_alert(
    ctx: Context<ManageSecurityAlert>,
    alert_id: u64,
    false_positive: bool,
    resolution_notes: String,
) -> Result<()> {
    let alert_store = &mut ctx.accounts.alert_store;
    
    if let Some(alert) = alert_store.alerts.iter_mut().find(|a| a.alert_id == alert_id) {
        alert.resolve(false_positive);
        alert.add_investigation_note(resolution_notes);
        
        if alert.status == AlertStatus::Resolved {
            alert_store.resolved_count += 1;
            alert_store.active_count = alert_store.active_count.saturating_sub(1);
        }
        
        alert_store.last_updated = Clock::get()?.unix_timestamp;
    } else {
        return Err(VaultError::AlertNotFound.into());
    }
    
    Ok(())
}

pub fn assign_security_alert(
    ctx: Context<ManageSecurityAlert>,
    alert_id: u64,
    officer: Pubkey,
) -> Result<()> {
    let alert_store = &mut ctx.accounts.alert_store;
    
    if let Some(alert) = alert_store.alerts.iter_mut().find(|a| a.alert_id == alert_id) {
        alert.assign_to(officer);
        alert_store.last_updated = Clock::get()?.unix_timestamp;
    } else {
        return Err(VaultError::AlertNotFound.into());
    }
    
    Ok(())
}

pub fn add_anomaly_rule(
    ctx: Context<UpdateAnomalyRules>,
    name: String,
    description: String,
    event_types: Vec<SecurityEventType>,
    threshold_value: f64,
    time_window_minutes: u32,
    severity: SecurityLevel,
    auto_block: bool,
) -> Result<()> {
    let security_monitor = &mut ctx.accounts.security_monitor;
    let rule_store = &mut ctx.accounts.rule_store;
    
    let rule_id = rule_store.rules.len() as u64 + 1;
    let rule = AnomalyDetectionRule {
        rule_id,
        name,
        description,
        event_types,
        enabled: true,
        threshold_value,
        time_window_minutes,
        severity,
        auto_block,
        notification_required: matches!(severity, SecurityLevel::High | SecurityLevel::Critical),
    };
    
    rule_store.rules.push(rule);
    rule_store.enabled_count += 1;
    rule_store.last_updated = Clock::get()?.unix_timestamp;
    
    Ok(())
}

pub fn update_security_config(
    ctx: Context<UpdateAnomalyRules>,
    retention_days: Option<u32>,
    max_events_per_user: Option<u32>,
    auto_block_enabled: Option<bool>,
    notification_webhook: Option<String>,
) -> Result<()> {
    let security_monitor = &mut ctx.accounts.security_monitor;
    
    if let Some(retention) = retention_days {
        security_monitor.retention_days = retention;
    }
    
    if let Some(max_events) = max_events_per_user {
        security_monitor.max_events_per_user = max_events;
    }
    
    if let Some(auto_block) = auto_block_enabled {
        security_monitor.auto_block_enabled = auto_block;
    }
    
    if let Some(webhook) = notification_webhook {
        security_monitor.notification_webhook = Some(webhook);
    }
    
    Ok(())
}

// Helper functions

fn create_default_anomaly_rules() -> Vec<AnomalyDetectionRule> {
    vec![
        AnomalyDetectionRule {
            rule_id: 1,
            name: "Multiple Failed Logins".to_string(),
            description: "Detect multiple failed login attempts".to_string(),
            event_types: vec![SecurityEventType::LoginFailure],
            enabled: true,
            threshold_value: 5.0,
            time_window_minutes: 15,
            severity: SecurityLevel::Medium,
            auto_block: true,
            notification_required: true,
        },
        AnomalyDetectionRule {
            rule_id: 2,
            name: "High Value Transactions".to_string(),
            description: "Detect unusually large transactions".to_string(),
            event_types: vec![SecurityEventType::BTCCommitment, SecurityEventType::RewardClaim],
            enabled: true,
            threshold_value: 100000.0, // $100k USD equivalent
            time_window_minutes: 60,
            severity: SecurityLevel::High,
            auto_block: false,
            notification_required: true,
        },
        AnomalyDetectionRule {
            rule_id: 3,
            name: "Rapid Transaction Velocity".to_string(),
            description: "Detect high frequency transactions".to_string(),
            event_types: vec![SecurityEventType::PaymentRequest, SecurityEventType::RewardClaim],
            enabled: true,
            threshold_value: 10.0,
            time_window_minutes: 5,
            severity: SecurityLevel::Medium,
            auto_block: false,
            notification_required: true,
        },
        AnomalyDetectionRule {
            rule_id: 4,
            name: "Unusual Login Location".to_string(),
            description: "Detect logins from unusual locations".to_string(),
            event_types: vec![SecurityEventType::LoginSuccess],
            enabled: true,
            threshold_value: 1.0,
            time_window_minutes: 1440, // 24 hours
            severity: SecurityLevel::Low,
            auto_block: false,
            notification_required: false,
        },
        AnomalyDetectionRule {
            rule_id: 5,
            name: "Security Violations".to_string(),
            description: "Detect security violations".to_string(),
            event_types: vec![SecurityEventType::SecurityViolation],
            enabled: true,
            threshold_value: 1.0,
            time_window_minutes: 1,
            severity: SecurityLevel::Critical,
            auto_block: true,
            notification_required: true,
        },
    ]
}

fn determine_security_level(event_type: &SecurityEventType) -> SecurityLevel {
    match event_type {
        SecurityEventType::SecurityViolation => SecurityLevel::Critical,
        SecurityEventType::EmergencyMode | SecurityEventType::AccountFrozen => SecurityLevel::High,
        SecurityEventType::LoginFailure | SecurityEventType::TwoFactorFailure | 
        SecurityEventType::ComplianceAlert | SecurityEventType::SuspiciousPattern => SecurityLevel::Medium,
        _ => SecurityLevel::Low,
    }
}

fn update_user_behavior_profile(
    behavior_store: &mut Account<UserBehaviorStore>,
    user: Pubkey,
    event_type: &SecurityEventType,
    ip_address: &Option<String>,
    device_id: &Option<String>,
    user_agent: &Option<String>,
    amount: Option<u64>,
) -> Result<()> {
    let now = Clock::get()?.unix_timestamp;
    let hour = ((now % 86400) / 3600) as u8;
    let day = ((now / 86400 + 4) % 7) as u8; // Unix epoch was Thursday
    
    let profile = behavior_store.profiles.entry(user).or_insert_with(|| UserBehaviorProfile::new(user));
    
    match event_type {
        SecurityEventType::LoginSuccess => {
            if let (Some(ip), Some(device), Some(ua)) = (ip_address, device_id, user_agent) {
                profile.update_login_pattern(hour, day, ip.clone(), device.clone(), ua.clone());
            }
        },
        SecurityEventType::LoginFailure => {
            profile.failed_login_attempts += 1;
        },
        SecurityEventType::PaymentRequest | SecurityEventType::RewardClaim => {
            if let Some(amt) = amount {
                profile.update_transaction_pattern(amt, "BTC".to_string()); // Default to BTC
            }
        },
        SecurityEventType::SuspiciousPattern | SecurityEventType::SecurityViolation => {
            profile.suspicious_activity_count += 1;
            profile.last_suspicious_activity = Some(now);
        },
        SecurityEventType::ComplianceAlert => {
            profile.compliance_alerts += 1;
        },
        _ => {}
    }
    
    profile.calculate_risk_score();
    behavior_store.last_updated = now;
    
    Ok(())
}

fn is_anomalous_behavior(
    profile: &UserBehaviorProfile,
    event: &SecurityEvent,
    ip_address: &Option<String>,
    device_id: &Option<String>,
) -> bool {
    let now = Clock::get().unwrap().unix_timestamp;
    let hour = ((now % 86400) / 3600) as u8;
    let day = ((now / 86400 + 4) % 7) as u8;
    
    match event.event_type {
        SecurityEventType::LoginSuccess => {
            if let (Some(ip), Some(device)) = (ip_address, device_id) {
                profile.is_anomalous_login(hour, day, ip, device)
            } else {
                false
            }
        },
        SecurityEventType::PaymentRequest | SecurityEventType::RewardClaim => {
            if let Some(amount) = event.amount {
                profile.is_anomalous_transaction(amount)
            } else {
                false
            }
        },
        _ => false,
    }
}

fn check_anomaly_rules(
    security_monitor: &mut Account<SecurityMonitor>,
    alert_store: &mut Account<SecurityAlertStore>,
    rule_store: &Account<AnomalyRuleStore>,
    event: &SecurityEvent,
) -> Result<()> {
    for rule in &rule_store.rules {
        if !rule.enabled || !rule.event_types.contains(&event.event_type) {
            continue;
        }
        
        let should_trigger = match event.event_type {
            SecurityEventType::SecurityViolation => true,
            _ => {
                if let Some(amount) = event.amount {
                    amount as f64 >= rule.threshold_value
                } else {
                    false
                }
            }
        };
        
        if should_trigger {
            create_security_alert(
                security_monitor,
                alert_store,
                event.event_type.clone(),
                event.user,
                format!("Anomaly rule triggered: {}", rule.name),
                rule.severity.clone(),
                vec![event.event_id],
            )?;
        }
    }
    
    Ok(())
}

fn create_security_alert(
    security_monitor: &mut Account<SecurityMonitor>,
    alert_store: &mut Account<SecurityAlertStore>,
    alert_type: SecurityEventType,
    user: Option<Pubkey>,
    description: String,
    security_level: SecurityLevel,
    related_events: Vec<u64>,
) -> Result<()> {
    security_monitor.alert_counter += 1;
    
    let mut alert = SecurityAlert::new(
        security_monitor.alert_counter,
        alert_type,
        user,
        description,
        security_level,
    );
    
    for event_id in related_events {
        alert.add_related_event(event_id);
    }
    
    alert_store.alerts.push(alert);
    alert_store.active_count += 1;
    alert_store.last_updated = Clock::get()?.unix_timestamp;
    
    Ok(())
}
