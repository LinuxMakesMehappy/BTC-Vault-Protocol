use anchor_lang::prelude::*;
use crate::errors::VaultError;

/// Authentication methods supported by the system
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum AuthMethod {
    TOTP,           // Time-based One-Time Password (Google Authenticator, Authy)
    WebAuthn,       // WebAuthn/FIDO2 (hardware keys, biometrics)
    SMS,            // SMS-based 2FA (less secure, fallback only)
    Email,          // Email-based 2FA (less secure, fallback only)
    Passkey,        // Platform passkeys (iOS/Android/Windows)
}

/// Session status for user authentication
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum SessionStatus {
    Active,         // Session is active and valid
    Expired,        // Session has expired
    Revoked,        // Session was manually revoked
    Compromised,    // Session marked as compromised
    Locked,         // Account locked due to security issues
}

/// Security event types for logging
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum SecurityEventType {
    LoginSuccess,           // Successful authentication
    LoginFailure,           // Failed authentication attempt
    TwoFactorSuccess,       // Successful 2FA verification
    TwoFactorFailure,       // Failed 2FA verification
    SessionCreated,         // New session created
    SessionExpired,         // Session expired
    SessionRevoked,         // Session manually revoked
    SuspiciousActivity,     // Suspicious activity detected
    AccountLocked,          // Account locked for security
    AccountUnlocked,        // Account unlocked
    PasswordChanged,        // Password/credentials changed
    TwoFactorEnabled,       // 2FA enabled
    TwoFactorDisabled,      // 2FA disabled
    DeviceRegistered,       // New device registered
    DeviceRevoked,          // Device access revoked
    CompromiseDetected,     // Wallet compromise detected
    RecoveryInitiated,      // Account recovery initiated
}

/// Authentication factor for multi-factor authentication
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct AuthFactor {
    pub method: AuthMethod,         // Authentication method
    pub identifier: String,         // Method-specific identifier (phone, email, device ID)
    pub secret_hash: [u8; 32],     // Hashed secret (for TOTP seed, etc.)
    pub backup_codes: Vec<String>,  // Backup recovery codes
    pub enabled: bool,              // Whether this factor is enabled
    pub verified: bool,             // Whether this factor is verified
    pub created_at: i64,           // Factor creation timestamp
    pub last_used: i64,            // Last successful use timestamp
    pub failure_count: u32,        // Consecutive failure count
    pub locked_until: Option<i64>, // Lock expiry timestamp
}

/// User session information
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct UserSession {
    pub session_id: String,         // Unique session identifier
    pub user: Pubkey,              // User public key
    pub device_id: String,         // Device identifier
    pub ip_address: String,        // IP address (hashed for privacy)
    pub user_agent_hash: [u8; 32], // User agent hash
    pub status: SessionStatus,      // Current session status
    pub created_at: i64,           // Session creation time
    pub last_activity: i64,        // Last activity timestamp
    pub expires_at: i64,           // Session expiry time
    pub auth_methods_used: Vec<AuthMethod>, // Methods used for this session
    pub permissions: Vec<String>,   // Session-specific permissions
    pub risk_score: u8,            // Risk assessment score (0-100)
}

/// Security event log entry
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct SecurityEvent {
    pub event_id: String,           // Unique event identifier
    pub user: Pubkey,              // User associated with event
    pub event_type: SecurityEventType, // Type of security event
    pub session_id: Option<String>, // Associated session ID
    pub device_id: Option<String>,  // Associated device ID
    pub ip_address_hash: [u8; 32], // Hashed IP address
    pub timestamp: i64,            // Event timestamp
    pub details: String,           // Event details/description
    pub risk_level: u8,            // Risk level (0-100)
    pub resolved: bool,            // Whether event was resolved
    pub resolved_at: Option<i64>,  // Resolution timestamp
    pub resolved_by: Option<Pubkey>, // Who resolved the event
}

/// User authentication profile
#[account]
pub struct UserAuth {
    pub user: Pubkey,                      // User public key
    pub auth_factors: Vec<AuthFactor>,     // Configured authentication factors
    pub active_sessions: Vec<UserSession>, // Active user sessions
    pub security_events: Vec<SecurityEvent>, // Security event history
    pub account_status: AccountStatus,     // Current account status
    pub security_settings: SecuritySettings, // User security preferences
    pub compromise_indicators: Vec<CompromiseIndicator>, // Compromise detection data
    pub last_password_change: i64,         // Last credential change
    pub failed_attempts: u32,              // Recent failed login attempts
    pub locked_until: Option<i64>,         // Account lock expiry
    pub created_at: i64,                   // Account creation time
    pub updated_at: i64,                   // Last update time
    pub bump: u8,                          // PDA bump
}

/// Account security status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum AccountStatus {
    Active,         // Account is active and secure
    Locked,         // Account locked due to security issues
    Compromised,    // Account marked as compromised
    Recovery,       // Account in recovery mode
    Suspended,      // Account suspended by admin
    PendingVerification, // Pending 2FA setup
}

/// User security preferences
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct SecuritySettings {
    pub require_2fa_for_all: bool,         // Require 2FA for all operations
    pub require_2fa_for_payments: bool,    // Require 2FA for payments only
    pub require_2fa_for_high_value: bool,  // Require 2FA for high-value operations
    pub session_timeout: u32,              // Session timeout in seconds
    pub max_concurrent_sessions: u8,       // Maximum concurrent sessions
    pub enable_email_notifications: bool,  // Email security notifications
    pub enable_sms_notifications: bool,    // SMS security notifications
    pub trusted_devices: Vec<String>,      // Trusted device IDs
    pub ip_whitelist: Vec<String>,         // Whitelisted IP addresses (hashed)
    pub auto_lock_on_suspicious: bool,    // Auto-lock on suspicious activity
    pub backup_codes_generated: bool,     // Whether backup codes exist
}

/// Compromise detection indicators
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct CompromiseIndicator {
    pub indicator_type: CompromiseType,    // Type of compromise indicator
    pub detected_at: i64,                 // Detection timestamp
    pub confidence: u8,                   // Confidence level (0-100)
    pub details: String,                  // Indicator details
    pub resolved: bool,                   // Whether indicator was resolved
    pub false_positive: bool,             // Whether it was a false positive
}

/// Types of compromise indicators
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum CompromiseType {
    UnusualLocation,        // Login from unusual location
    UnusualDevice,          // Login from new/unusual device
    VelocityAnomaly,        // Unusual activity velocity
    PatternAnomaly,         // Unusual behavior pattern
    KnownMalware,          // Known malware signature
    SuspiciousTransaction,  // Suspicious transaction pattern
    CredentialLeak,        // Credentials found in breach data
    PhishingAttempt,       // Phishing attempt detected
    BruteForceAttack,      // Brute force attack detected
    SessionHijacking,      // Session hijacking attempt
}

impl UserAuth {
    pub const LEN: usize = 8 + // discriminator
        32 + // user
        4 + 10 * (1 + 4 + 64 + 32 + 4 + 10 * 64 + 1 + 1 + 8 + 8 + 4 + 9) + // auth_factors (max 10)
        4 + 5 * (4 + 64 + 32 + 4 + 64 + 4 + 64 + 32 + 1 + 8 + 8 + 8 + 4 + 10 * 1 + 4 + 10 * 64 + 1) + // active_sessions (max 5)
        4 + 100 * (4 + 64 + 32 + 1 + 9 + 4 + 64 + 4 + 64 + 32 + 8 + 4 + 256 + 1 + 1 + 9 + 33) + // security_events (max 100)
        1 + // account_status
        (1 + 1 + 1 + 4 + 1 + 1 + 1 + 4 + 10 * 64 + 4 + 10 * 64 + 1 + 1) + // security_settings
        4 + 20 * (1 + 8 + 1 + 4 + 256 + 1 + 1) + // compromise_indicators (max 20)
        8 + // last_password_change
        4 + // failed_attempts
        9 + // locked_until (optional)
        8 + // created_at
        8 + // updated_at
        1; // bump

    pub const MAX_AUTH_FACTORS: usize = 10;
    pub const MAX_ACTIVE_SESSIONS: usize = 5;
    pub const MAX_SECURITY_EVENTS: usize = 100;
    pub const MAX_COMPROMISE_INDICATORS: usize = 20;
    pub const SESSION_TIMEOUT_DEFAULT: u32 = 3600; // 1 hour
    pub const MAX_FAILED_ATTEMPTS: u32 = 5;
    pub const LOCKOUT_DURATION: i64 = 900; // 15 minutes

    /// Initialize user authentication profile
    pub fn initialize(
        &mut self,
        user: Pubkey,
        bump: u8,
    ) -> Result<()> {
        let clock = Clock::get()?;
        
        self.user = user;
        self.auth_factors = Vec::new();
        self.active_sessions = Vec::new();
        self.security_events = Vec::new();
        self.account_status = AccountStatus::PendingVerification;
        
        // Default security settings
        self.security_settings = SecuritySettings {
            require_2fa_for_all: true,
            require_2fa_for_payments: true,
            require_2fa_for_high_value: true,
            session_timeout: Self::SESSION_TIMEOUT_DEFAULT,
            max_concurrent_sessions: 3,
            enable_email_notifications: true,
            enable_sms_notifications: false,
            trusted_devices: Vec::new(),
            ip_whitelist: Vec::new(),
            auto_lock_on_suspicious: true,
            backup_codes_generated: false,
        };
        
        self.compromise_indicators = Vec::new();
        self.last_password_change = clock.unix_timestamp;
        self.failed_attempts = 0;
        self.locked_until = None;
        self.created_at = clock.unix_timestamp;
        self.updated_at = clock.unix_timestamp;
        self.bump = bump;
        
        // Log account creation
        self.add_security_event(
            SecurityEventType::LoginSuccess,
            None,
            None,
            "Account created".to_string(),
            10, // Low risk
        )?;
        
        msg!("User authentication profile initialized for user: {}", user);
        
        Ok(())
    }
    
    /// Add a new authentication factor
    pub fn add_auth_factor(
        &mut self,
        method: AuthMethod,
        identifier: String,
        secret_hash: [u8; 32],
        backup_codes: Vec<String>,
    ) -> Result<()> {
        if self.auth_factors.len() >= Self::MAX_AUTH_FACTORS {
            return Err(VaultError::TooManyAuthFactors.into());
        }
        
        // Check if method already exists
        if self.auth_factors.iter().any(|f| f.method == method && f.identifier == identifier) {
            return Err(VaultError::AuthFactorAlreadyExists.into());
        }
        
        let clock = Clock::get()?;
        
        let factor = AuthFactor {
            method: method.clone(),
            identifier: identifier.clone(),
            secret_hash,
            backup_codes,
            enabled: true,
            verified: false, // Requires verification
            created_at: clock.unix_timestamp,
            last_used: 0,
            failure_count: 0,
            locked_until: None,
        };
        
        self.auth_factors.push(factor);
        self.updated_at = clock.unix_timestamp;
        
        // Log factor addition
        self.add_security_event(
            SecurityEventType::TwoFactorEnabled,
            None,
            None,
            format!("Authentication factor added: {:?}", method),
            20, // Medium risk
        )?;
        
        msg!("Authentication factor added for user {}: {:?}", self.user, method);
        
        Ok(())
    }
    
    /// Verify an authentication factor
    pub fn verify_auth_factor(
        &mut self,
        method: AuthMethod,
        identifier: String,
        provided_code: String,
    ) -> Result<bool> {
        let clock = Clock::get()?;
        
        // Find the authentication factor
        let factor = self.auth_factors.iter_mut()
            .find(|f| f.method == method && f.identifier == identifier)
            .ok_or(VaultError::AuthFactorNotFound)?;
        
        // Check if factor is locked
        if let Some(locked_until) = factor.locked_until {
            if clock.unix_timestamp < locked_until {
                return Err(VaultError::AuthFactorLocked.into());
            } else {
                factor.locked_until = None; // Unlock expired lock
            }
        }
        
        // Verify the code (simplified - in production would use proper TOTP/WebAuthn verification)
        let is_valid = self.verify_code(&factor.secret_hash, &provided_code, &method)?;
        
        if is_valid {
            factor.verified = true;
            factor.last_used = clock.unix_timestamp;
            factor.failure_count = 0;
            
            // Update account status if this was the first verification
            if self.account_status == AccountStatus::PendingVerification {
                self.account_status = AccountStatus::Active;
            }
            
            self.add_security_event(
                SecurityEventType::TwoFactorSuccess,
                None,
                None,
                format!("2FA verification successful: {:?}", method),
                10, // Low risk
            )?;
            
            msg!("2FA verification successful for user {}: {:?}", self.user, method);
            
        } else {
            factor.failure_count += 1;
            
            // Lock factor after too many failures
            if factor.failure_count >= 5 {
                factor.locked_until = Some(clock.unix_timestamp + 900); // 15 minutes
            }
            
            self.add_security_event(
                SecurityEventType::TwoFactorFailure,
                None,
                None,
                format!("2FA verification failed: {:?}", method),
                60, // High risk
            )?;
            
            msg!("2FA verification failed for user {}: {:?}", self.user, method);
        }
        
        self.updated_at = clock.unix_timestamp;
        
        Ok(is_valid)
    }
    
    /// Create a new user session
    pub fn create_session(
        &mut self,
        device_id: String,
        ip_address: String,
        user_agent: String,
        auth_methods: Vec<AuthMethod>,
    ) -> Result<String> {
        if self.active_sessions.len() >= self.security_settings.max_concurrent_sessions as usize {
            // Remove oldest session
            self.active_sessions.sort_by_key(|s| s.last_activity);
            self.active_sessions.remove(0);
        }
        
        let clock = Clock::get()?;
        let session_id = format!("{}_{}", self.user.to_string()[..8].to_string(), clock.unix_timestamp);
        
        // Calculate risk score
        let risk_score = self.calculate_session_risk(&device_id, &ip_address, &user_agent)?;
        
        let session = UserSession {
            session_id: session_id.clone(),
            user: self.user,
            device_id: device_id.clone(),
            ip_address: self.hash_ip(&ip_address),
            user_agent_hash: self.hash_user_agent(&user_agent),
            status: SessionStatus::Active,
            created_at: clock.unix_timestamp,
            last_activity: clock.unix_timestamp,
            expires_at: clock.unix_timestamp + self.security_settings.session_timeout as i64,
            auth_methods_used: auth_methods.clone(),
            permissions: self.get_session_permissions(&auth_methods),
            risk_score,
        };
        
        self.active_sessions.push(session);
        self.updated_at = clock.unix_timestamp;
        
        // Log session creation
        self.add_security_event(
            SecurityEventType::SessionCreated,
            Some(session_id.clone()),
            Some(device_id),
            format!("Session created with methods: {:?}", auth_methods),
            risk_score,
        )?;
        
        msg!("Session created for user {}: {}", self.user, session_id);
        
        Ok(session_id)
    }
    
    /// Validate a user session
    pub fn validate_session(&mut self, session_id: &str) -> Result<bool> {
        let clock = Clock::get()?;
        
        let session = self.active_sessions.iter_mut()
            .find(|s| s.session_id == session_id)
            .ok_or(VaultError::SessionNotFound)?;
        
        // Check if session is expired
        if clock.unix_timestamp > session.expires_at {
            session.status = SessionStatus::Expired;
            
            self.add_security_event(
                SecurityEventType::SessionExpired,
                Some(session_id.to_string()),
                Some(session.device_id.clone()),
                "Session expired".to_string(),
                30, // Medium risk
            )?;
            
            return Ok(false);
        }
        
        // Check if session is compromised or locked
        if session.status != SessionStatus::Active {
            return Ok(false);
        }
        
        // Update last activity
        session.last_activity = clock.unix_timestamp;
        session.expires_at = clock.unix_timestamp + self.security_settings.session_timeout as i64;
        
        self.updated_at = clock.unix_timestamp;
        
        Ok(true)
    }
    
    /// Revoke a user session
    pub fn revoke_session(&mut self, session_id: &str) -> Result<()> {
        let session = self.active_sessions.iter_mut()
            .find(|s| s.session_id == session_id)
            .ok_or(VaultError::SessionNotFound)?;
        
        session.status = SessionStatus::Revoked;
        self.updated_at = Clock::get()?.unix_timestamp;
        
        self.add_security_event(
            SecurityEventType::SessionRevoked,
            Some(session_id.to_string()),
            Some(session.device_id.clone()),
            "Session manually revoked".to_string(),
            20, // Medium risk
        )?;
        
        msg!("Session revoked for user {}: {}", self.user, session_id);
        
        Ok(())
    }
    
    /// Detect potential account compromise
    pub fn detect_compromise(
        &mut self,
        device_id: &str,
        ip_address: &str,
        user_agent: &str,
    ) -> Result<Vec<CompromiseType>> {
        let mut indicators = Vec::new();
        let clock = Clock::get()?;
        
        // Check for unusual location (simplified - would use GeoIP in production)
        if !self.is_known_location(ip_address) {
            indicators.push(CompromiseType::UnusualLocation);
        }
        
        // Check for unusual device
        if !self.security_settings.trusted_devices.contains(&device_id.to_string()) {
            indicators.push(CompromiseType::UnusualDevice);
        }
        
        // Check for velocity anomalies
        let recent_sessions = self.active_sessions.iter()
            .filter(|s| s.created_at > clock.unix_timestamp - 3600) // Last hour
            .count();
        
        if recent_sessions > 5 {
            indicators.push(CompromiseType::VelocityAnomaly);
        }
        
        // Check for pattern anomalies (simplified)
        if self.security_events.iter()
            .filter(|e| e.timestamp > clock.unix_timestamp - 3600 && e.event_type == SecurityEventType::LoginFailure)
            .count() > 3 {
            indicators.push(CompromiseType::BruteForceAttack);
        }
        
        // Add compromise indicators
        for indicator_type in &indicators {
            if self.compromise_indicators.len() < Self::MAX_COMPROMISE_INDICATORS {
                let indicator = CompromiseIndicator {
                    indicator_type: indicator_type.clone(),
                    detected_at: clock.unix_timestamp,
                    confidence: 75, // Medium confidence
                    details: format!("Detected during session validation"),
                    resolved: false,
                    false_positive: false,
                };
                
                self.compromise_indicators.push(indicator);
            }
        }
        
        // Auto-lock if configured and high-risk indicators found
        if self.security_settings.auto_lock_on_suspicious && !indicators.is_empty() {
            let high_risk_indicators = [
                CompromiseType::KnownMalware,
                CompromiseType::CredentialLeak,
                CompromiseType::SessionHijacking,
                CompromiseType::BruteForceAttack,
            ];
            
            if indicators.iter().any(|i| high_risk_indicators.contains(i)) {
                self.lock_account("Suspicious activity detected".to_string())?;
            }
        }
        
        if !indicators.is_empty() {
            self.add_security_event(
                SecurityEventType::CompromiseDetected,
                None,
                Some(device_id.to_string()),
                format!("Compromise indicators: {:?}", indicators),
                80, // High risk
            )?;
        }
        
        Ok(indicators)
    }
    
    /// Lock the user account
    pub fn lock_account(&mut self, reason: String) -> Result<()> {
        let clock = Clock::get()?;
        
        self.account_status = AccountStatus::Locked;
        self.locked_until = Some(clock.unix_timestamp + Self::LOCKOUT_DURATION);
        
        // Revoke all active sessions
        for session in &mut self.active_sessions {
            session.status = SessionStatus::Revoked;
        }
        
        self.add_security_event(
            SecurityEventType::AccountLocked,
            None,
            None,
            reason,
            90, // Very high risk
        )?;
        
        self.updated_at = clock.unix_timestamp;
        
        msg!("Account locked for user {}", self.user);
        
        Ok(())
    }
    
    /// Unlock the user account
    pub fn unlock_account(&mut self, admin: Pubkey) -> Result<()> {
        self.account_status = AccountStatus::Active;
        self.locked_until = None;
        self.failed_attempts = 0;
        
        self.add_security_event(
            SecurityEventType::AccountUnlocked,
            None,
            None,
            format!("Account unlocked by admin: {}", admin),
            20, // Medium risk
        )?;
        
        self.updated_at = Clock::get()?.unix_timestamp;
        
        msg!("Account unlocked for user {} by admin {}", self.user, admin);
        
        Ok(())
    }
    
    /// Add a security event to the log
    pub fn add_security_event(
        &mut self,
        event_type: SecurityEventType,
        session_id: Option<String>,
        device_id: Option<String>,
        details: String,
        risk_level: u8,
    ) -> Result<()> {
        if self.security_events.len() >= Self::MAX_SECURITY_EVENTS {
            // Remove oldest event to make space
            self.security_events.remove(0);
        }
        
        let clock = Clock::get()?;
        let event_id = format!("{}_{}", self.user.to_string()[..8].to_string(), clock.unix_timestamp);
        
        let event = SecurityEvent {
            event_id,
            user: self.user,
            event_type,
            session_id,
            device_id,
            ip_address_hash: [0; 32], // Would be populated with actual IP hash
            timestamp: clock.unix_timestamp,
            details,
            risk_level,
            resolved: false,
            resolved_at: None,
            resolved_by: None,
        };
        
        self.security_events.push(event);
        
        Ok(())
    }
    
    /// Check if user has required 2FA for operation
    pub fn requires_2fa_for_operation(&self, operation_type: &str, amount: Option<u64>) -> bool {
        match operation_type {
            "payment" => self.security_settings.require_2fa_for_payments,
            "high_value" => {
                if let Some(amt) = amount {
                    self.security_settings.require_2fa_for_high_value && amt > 100_000_000 // 1 BTC
                } else {
                    false
                }
            },
            _ => self.security_settings.require_2fa_for_all,
        }
    }
    
    /// Get active 2FA methods for user
    pub fn get_active_2fa_methods(&self) -> Vec<AuthMethod> {
        self.auth_factors.iter()
            .filter(|f| f.enabled && f.verified)
            .map(|f| f.method.clone())
            .collect()
    }
    
    /// Check if account is currently locked
    pub fn is_locked(&self) -> bool {
        match self.account_status {
            AccountStatus::Locked | AccountStatus::Compromised | AccountStatus::Suspended => true,
            _ => {
                if let Some(locked_until) = self.locked_until {
                    Clock::get().unwrap().unix_timestamp < locked_until
                } else {
                    false
                }
            }
        }
    }
    
    // Helper methods
    
    fn verify_code(&self, secret_hash: &[u8; 32], provided_code: &str, method: &AuthMethod) -> Result<bool> {
        // Simplified verification - in production would implement proper TOTP/WebAuthn
        match method {
            AuthMethod::TOTP => {
                // Would use TOTP library to verify time-based code
                Ok(provided_code.len() == 6 && provided_code.chars().all(|c| c.is_ascii_digit()))
            },
            AuthMethod::WebAuthn => {
                // Would use WebAuthn library to verify signature
                Ok(provided_code.len() > 10)
            },
            AuthMethod::Passkey => {
                // Would use platform passkey verification
                Ok(provided_code.len() > 10)
            },
            _ => Ok(provided_code.len() >= 4), // Simplified for SMS/Email
        }
    }
    
    fn calculate_session_risk(&self, device_id: &str, ip_address: &str, user_agent: &str) -> Result<u8> {
        let mut risk_score = 0u8;
        
        // Unknown device adds risk
        if !self.security_settings.trusted_devices.contains(&device_id.to_string()) {
            risk_score += 30;
        }
        
        // Unknown location adds risk
        if !self.is_known_location(ip_address) {
            risk_score += 25;
        }
        
        // Recent failed attempts add risk
        if self.failed_attempts > 0 {
            risk_score += (self.failed_attempts * 10).min(40) as u8;
        }
        
        // Recent compromise indicators add risk
        let recent_indicators = self.compromise_indicators.iter()
            .filter(|i| !i.resolved && i.detected_at > Clock::get().unwrap().unix_timestamp - 86400)
            .count();
        
        risk_score += (recent_indicators * 15).min(30) as u8;
        
        Ok(risk_score.min(100))
    }
    
    fn hash_ip(&self, ip_address: &str) -> String {
        // Would use proper hashing in production
        format!("hashed_{}", ip_address.len())
    }
    
    fn hash_user_agent(&self, user_agent: &str) -> [u8; 32] {
        // Would use proper hashing in production
        let mut hash = [0u8; 32];
        let bytes = user_agent.as_bytes();
        for (i, &byte) in bytes.iter().take(32).enumerate() {
            hash[i] = byte;
        }
        hash
    }
    
    fn is_known_location(&self, ip_address: &str) -> bool {
        // Simplified - would use GeoIP and user's known locations
        self.security_settings.ip_whitelist.contains(&self.hash_ip(ip_address))
    }
    
    fn get_session_permissions(&self, auth_methods: &[AuthMethod]) -> Vec<String> {
        let mut permissions = vec!["read".to_string()];
        
        // Grant additional permissions based on auth methods used
        if auth_methods.contains(&AuthMethod::TOTP) || auth_methods.contains(&AuthMethod::WebAuthn) {
            permissions.push("write".to_string());
            permissions.push("payment".to_string());
        }
        
        if auth_methods.contains(&AuthMethod::WebAuthn) || auth_methods.contains(&AuthMethod::Passkey) {
            permissions.push("admin".to_string());
        }
        
        permissions
    }
}

/// Global authentication configuration
#[account]
pub struct AuthConfig {
    pub authority: Pubkey,                 // System authority
    pub require_2fa_globally: bool,       // Global 2FA requirement
    pub allowed_auth_methods: Vec<AuthMethod>, // Allowed authentication methods
    pub session_timeout_min: u32,         // Minimum session timeout
    pub session_timeout_max: u32,         // Maximum session timeout
    pub max_failed_attempts: u32,         // Max failed attempts before lockout
    pub lockout_duration: i64,            // Lockout duration in seconds
    pub enable_compromise_detection: bool, // Enable automatic compromise detection
    pub security_event_retention: u32,    // Security event retention in days
    pub created_at: i64,                  // Configuration creation time
    pub updated_at: i64,                  // Last update time
    pub bump: u8,                         // PDA bump
}

impl AuthConfig {
    pub const LEN: usize = 8 + // discriminator
        32 + // authority
        1 + // require_2fa_globally
        4 + 10 * 1 + // allowed_auth_methods (max 10)
        4 + // session_timeout_min
        4 + // session_timeout_max
        4 + // max_failed_attempts
        8 + // lockout_duration
        1 + // enable_compromise_detection
        4 + // security_event_retention
        8 + // created_at
        8 + // updated_at
        1; // bump

    /// Initialize authentication configuration
    pub fn initialize(
        &mut self,
        authority: Pubkey,
        bump: u8,
    ) -> Result<()> {
        let clock = Clock::get()?;
        
        self.authority = authority;
        self.require_2fa_globally = true;
        self.allowed_auth_methods = vec![
            AuthMethod::TOTP,
            AuthMethod::WebAuthn,
            AuthMethod::Passkey,
        ];
        self.session_timeout_min = 300;  // 5 minutes
        self.session_timeout_max = 86400; // 24 hours
        self.max_failed_attempts = 5;
        self.lockout_duration = 900; // 15 minutes
        self.enable_compromise_detection = true;
        self.security_event_retention = 2555; // 7 years
        self.created_at = clock.unix_timestamp;
        self.updated_at = clock.unix_timestamp;
        self.bump = bump;
        
        Ok(())
    }
    
    /// Update authentication configuration
    pub fn update_config(
        &mut self,
        authority: Pubkey,
        require_2fa_globally: Option<bool>,
        session_timeout_min: Option<u32>,
        session_timeout_max: Option<u32>,
        max_failed_attempts: Option<u32>,
        lockout_duration: Option<i64>,
    ) -> Result<()> {
        if authority != self.authority {
            return Err(VaultError::UnauthorizedAccess.into());
        }
        
        if let Some(require_2fa) = require_2fa_globally {
            self.require_2fa_globally = require_2fa;
        }
        
        if let Some(timeout_min) = session_timeout_min {
            self.session_timeout_min = timeout_min;
        }
        
        if let Some(timeout_max) = session_timeout_max {
            self.session_timeout_max = timeout_max;
        }
        
        if let Some(max_attempts) = max_failed_attempts {
            self.max_failed_attempts = max_attempts;
        }
        
        if let Some(lockout) = lockout_duration {
            self.lockout_duration = lockout;
        }
        
        self.updated_at = Clock::get()?.unix_timestamp;
        
        Ok(())
    }
}