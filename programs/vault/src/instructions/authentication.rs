use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::VaultError;

#[derive(Accounts)]
pub struct InitializeAuth<'info> {
    #[account(
        init,
        payer = user,
        space = UserAuth::LEN,
        seeds = [b"user_auth", user.key().as_ref()],
        bump
    )]
    pub user_auth: Account<'info, UserAuth>,
    
    #[account(
        seeds = [b"auth_config"],
        bump = auth_config.bump
    )]
    pub auth_config: Account<'info, AuthConfig>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeAuthConfig<'info> {
    #[account(
        init,
        payer = authority,
        space = AuthConfig::LEN,
        seeds = [b"auth_config"],
        bump
    )]
    pub auth_config: Account<'info, AuthConfig>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddAuthFactor<'info> {
    #[account(
        mut,
        seeds = [b"user_auth", user_auth.user.as_ref()],
        bump = user_auth.bump
    )]
    pub user_auth: Account<'info, UserAuth>,
    
    #[account(
        seeds = [b"auth_config"],
        bump = auth_config.bump
    )]
    pub auth_config: Account<'info, AuthConfig>,
    
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct VerifyAuthFactor<'info> {
    #[account(
        mut,
        seeds = [b"user_auth", user_auth.user.as_ref()],
        bump = user_auth.bump
    )]
    pub user_auth: Account<'info, UserAuth>,
    
    #[account(
        seeds = [b"auth_config"],
        bump = auth_config.bump
    )]
    pub auth_config: Account<'info, AuthConfig>,
    
    /// CHECK: User account for verification
    pub user: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CreateSession<'info> {
    #[account(
        mut,
        seeds = [b"user_auth", user_auth.user.as_ref()],
        bump = user_auth.bump
    )]
    pub user_auth: Account<'info, UserAuth>,
    
    #[account(
        seeds = [b"auth_config"],
        bump = auth_config.bump
    )]
    pub auth_config: Account<'info, AuthConfig>,
    
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct ValidateSession<'info> {
    #[account(
        mut,
        seeds = [b"user_auth", user_auth.user.as_ref()],
        bump = user_auth.bump
    )]
    pub user_auth: Account<'info, UserAuth>,
    
    #[account(
        seeds = [b"auth_config"],
        bump = auth_config.bump
    )]
    pub auth_config: Account<'info, AuthConfig>,
    
    /// CHECK: User account for session validation
    pub user: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct RevokeSession<'info> {
    #[account(
        mut,
        seeds = [b"user_auth", user_auth.user.as_ref()],
        bump = user_auth.bump
    )]
    pub user_auth: Account<'info, UserAuth>,
    
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct LockAccount<'info> {
    #[account(
        mut,
        seeds = [b"user_auth", user_auth.user.as_ref()],
        bump = user_auth.bump
    )]
    pub user_auth: Account<'info, UserAuth>,
    
    #[account(
        seeds = [b"auth_config"],
        bump = auth_config.bump
    )]
    pub auth_config: Account<'info, AuthConfig>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateAuthConfig<'info> {
    #[account(
        mut,
        seeds = [b"auth_config"],
        bump = auth_config.bump
    )]
    pub auth_config: Account<'info, AuthConfig>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
}

/// Initialize global authentication configuration
pub fn initialize_auth_config(
    ctx: Context<InitializeAuthConfig>,
) -> Result<()> {
    let auth_config = &mut ctx.accounts.auth_config;
    let authority = ctx.accounts.authority.key();
    
    auth_config.initialize(authority, ctx.bumps.auth_config)?;
    
    msg!("Authentication configuration initialized by authority: {}", authority);
    
    Ok(())
}

/// Initialize user authentication profile
pub fn initialize_user_auth(
    ctx: Context<InitializeAuth>,
) -> Result<()> {
    let user_auth = &mut ctx.accounts.user_auth;
    let user = ctx.accounts.user.key();
    
    user_auth.initialize(user, ctx.bumps.user_auth)?;
    
    msg!("User authentication profile initialized for user: {}", user);
    
    Ok(())
}

/// Add a new authentication factor
pub fn add_auth_factor(
    ctx: Context<AddAuthFactor>,
    method: AuthMethod,
    identifier: String,
    secret_hash: [u8; 32],
    backup_codes: Vec<String>,
) -> Result<()> {
    let user_auth = &mut ctx.accounts.user_auth;
    let auth_config = &ctx.accounts.auth_config;
    let user = ctx.accounts.user.key();
    
    // Verify user owns the account
    if user != user_auth.user {
        return Err(VaultError::UnauthorizedAccess.into());
    }
    
    // Check if method is allowed
    if !auth_config.allowed_auth_methods.contains(&method) {
        return Err(VaultError::AuthMethodNotAllowed.into());
    }
    
    user_auth.add_auth_factor(method, identifier, secret_hash, backup_codes)?;
    
    msg!("Authentication factor added for user: {}", user);
    
    Ok(())
}

/// Verify an authentication factor
pub fn verify_auth_factor(
    ctx: Context<VerifyAuthFactor>,
    method: AuthMethod,
    identifier: String,
    provided_code: String,
) -> Result<()> {
    let user_auth = &mut ctx.accounts.user_auth;
    let user = ctx.accounts.user.key();
    
    // Verify user owns the account
    if user != user_auth.user {
        return Err(VaultError::UnauthorizedAccess.into());
    }
    
    let is_valid = user_auth.verify_auth_factor(method, identifier, provided_code)?;
    
    if !is_valid {
        return Err(VaultError::InvalidAuthCode.into());
    }
    
    msg!("Authentication factor verified for user: {}", user);
    
    Ok(())
}

/// Create a new user session
pub fn create_session(
    ctx: Context<CreateSession>,
    device_id: String,
    ip_address: String,
    user_agent: String,
    auth_methods: Vec<AuthMethod>,
) -> Result<()> {
    let user_auth = &mut ctx.accounts.user_auth;
    let auth_config = &ctx.accounts.auth_config;
    let user = ctx.accounts.user.key();
    
    // Verify user owns the account
    if user != user_auth.user {
        return Err(VaultError::UnauthorizedAccess.into());
    }
    
    // Check if account is locked
    if user_auth.is_locked() {
        return Err(VaultError::AccountLocked.into());
    }
    
    // Verify 2FA if required
    if auth_config.require_2fa_globally {
        let has_2fa = auth_methods.iter().any(|method| {
            matches!(method, AuthMethod::TOTP | AuthMethod::WebAuthn | AuthMethod::Passkey)
        });
        
        if !has_2fa {
            return Err(VaultError::TwoFactorRequired.into());
        }
    }
    
    // Detect potential compromise
    let compromise_indicators = user_auth.detect_compromise(&device_id, &ip_address, &user_agent)?;
    
    if !compromise_indicators.is_empty() {
        msg!("Compromise indicators detected: {:?}", compromise_indicators);
        
        // Still allow session creation but with higher risk score
        // In production, might require additional verification
    }
    
    let session_id = user_auth.create_session(device_id, ip_address, user_agent, auth_methods)?;
    
    msg!("Session created for user {}: {}", user, session_id);
    
    Ok(())
}

/// Validate a user session
pub fn validate_session(
    ctx: Context<ValidateSession>,
    session_id: String,
) -> Result<()> {
    let user_auth = &mut ctx.accounts.user_auth;
    let user = ctx.accounts.user.key();
    
    // Verify user owns the account
    if user != user_auth.user {
        return Err(VaultError::UnauthorizedAccess.into());
    }
    
    let is_valid = user_auth.validate_session(&session_id)?;
    
    if !is_valid {
        return Err(VaultError::InvalidSession.into());
    }
    
    msg!("Session validated for user {}: {}", user, session_id);
    
    Ok(())
}

/// Revoke a user session
pub fn revoke_session(
    ctx: Context<RevokeSession>,
    session_id: String,
) -> Result<()> {
    let user_auth = &mut ctx.accounts.user_auth;
    let user = ctx.accounts.user.key();
    
    // Verify user owns the account
    if user != user_auth.user {
        return Err(VaultError::UnauthorizedAccess.into());
    }
    
    user_auth.revoke_session(&session_id)?;
    
    msg!("Session revoked for user {}: {}", user, session_id);
    
    Ok(())
}

/// Lock a user account (admin only)
pub fn lock_account(
    ctx: Context<LockAccount>,
    reason: String,
) -> Result<()> {
    let user_auth = &mut ctx.accounts.user_auth;
    let auth_config = &ctx.accounts.auth_config;
    let authority = ctx.accounts.authority.key();
    
    // Verify authority
    if authority != auth_config.authority {
        return Err(VaultError::UnauthorizedAccess.into());
    }
    
    user_auth.lock_account(reason)?;
    
    msg!("Account locked for user {} by authority {}", user_auth.user, authority);
    
    Ok(())
}

/// Unlock a user account (admin only)
pub fn unlock_account(
    ctx: Context<LockAccount>,
) -> Result<()> {
    let user_auth = &mut ctx.accounts.user_auth;
    let auth_config = &ctx.accounts.auth_config;
    let authority = ctx.accounts.authority.key();
    
    // Verify authority
    if authority != auth_config.authority {
        return Err(VaultError::UnauthorizedAccess.into());
    }
    
    user_auth.unlock_account(authority)?;
    
    msg!("Account unlocked for user {} by authority {}", user_auth.user, authority);
    
    Ok(())
}

/// Update authentication configuration
pub fn update_auth_config(
    ctx: Context<UpdateAuthConfig>,
    require_2fa_globally: Option<bool>,
    session_timeout_min: Option<u32>,
    session_timeout_max: Option<u32>,
    max_failed_attempts: Option<u32>,
    lockout_duration: Option<i64>,
) -> Result<()> {
    let auth_config = &mut ctx.accounts.auth_config;
    let authority = ctx.accounts.authority.key();
    
    auth_config.update_config(
        authority,
        require_2fa_globally,
        session_timeout_min,
        session_timeout_max,
        max_failed_attempts,
        lockout_duration,
    )?;
    
    msg!("Authentication configuration updated by authority: {}", authority);
    
    Ok(())
}

/// Check if user has required 2FA for operation
pub fn check_2fa_requirement(
    ctx: Context<ValidateSession>,
    operation_type: String,
    amount: Option<u64>,
) -> Result<()> {
    let user_auth = &ctx.accounts.user_auth;
    let user = ctx.accounts.user.key();
    
    // Verify user owns the account
    if user != user_auth.user {
        return Err(VaultError::UnauthorizedAccess.into());
    }
    
    let requires_2fa = user_auth.requires_2fa_for_operation(&operation_type, amount);
    
    if requires_2fa {
        let active_methods = user_auth.get_active_2fa_methods();
        if active_methods.is_empty() {
            return Err(VaultError::TwoFactorRequired.into());
        }
    }
    
    msg!("2FA requirement checked for user {}: required={}", user, requires_2fa);
    
    Ok(())
}

/// Get user security status
pub fn get_security_status(
    ctx: Context<ValidateSession>,
) -> Result<()> {
    let user_auth = &ctx.accounts.user_auth;
    let user = ctx.accounts.user.key();
    
    // Verify user owns the account
    if user != user_auth.user {
        return Err(VaultError::UnauthorizedAccess.into());
    }
    
    let active_2fa_methods = user_auth.get_active_2fa_methods();
    let active_sessions = user_auth.active_sessions.len();
    let recent_events = user_auth.security_events.iter()
        .filter(|e| e.timestamp > Clock::get().unwrap().unix_timestamp - 86400)
        .count();
    let unresolved_indicators = user_auth.compromise_indicators.iter()
        .filter(|i| !i.resolved)
        .count();
    
    msg!("Security status for user {}: 2FA methods: {}, Active sessions: {}, Recent events: {}, Unresolved indicators: {}",
         user, active_2fa_methods.len(), active_sessions, recent_events, unresolved_indicators);
    
    Ok(())
}

/// Middleware function to validate authentication for protected operations
pub fn validate_authenticated_operation(
    user_auth: &mut UserAuth,
    session_id: &str,
    operation_type: &str,
    amount: Option<u64>,
) -> Result<()> {
    // Check if account is locked
    if user_auth.is_locked() {
        return Err(VaultError::AccountLocked.into());
    }
    
    // Validate session
    let is_valid_session = user_auth.validate_session(session_id)?;
    if !is_valid_session {
        return Err(VaultError::InvalidSession.into());
    }
    
    // Check 2FA requirement
    if user_auth.requires_2fa_for_operation(operation_type, amount) {
        let active_methods = user_auth.get_active_2fa_methods();
        if active_methods.is_empty() {
            return Err(VaultError::TwoFactorRequired.into());
        }
        
        // In production, would also verify recent 2FA verification
        // For now, just check that 2FA is configured
    }
    
    // Log the operation attempt
    user_auth.add_security_event(
        SecurityEventType::LoginSuccess,
        Some(session_id.to_string()),
        None,
        format!("Authenticated operation: {}", operation_type),
        20, // Medium risk
    )?;
    
    Ok(())
}

/// Generate backup codes for 2FA recovery
pub fn generate_backup_codes(
    ctx: Context<AddAuthFactor>,
) -> Result<Vec<String>> {
    let user_auth = &mut ctx.accounts.user_auth;
    let user = ctx.accounts.user.key();
    
    // Verify user owns the account
    if user != user_auth.user {
        return Err(VaultError::UnauthorizedAccess.into());
    }
    
    // Generate 10 backup codes (simplified - would use cryptographically secure random)
    let mut backup_codes = Vec::new();
    let clock = Clock::get()?;
    let base = clock.unix_timestamp as u64;
    
    for i in 0..10 {
        let code = format!("{:08x}", base.wrapping_add(i * 12345));
        backup_codes.push(code);
    }
    
    // Update security settings
    user_auth.security_settings.backup_codes_generated = true;
    user_auth.updated_at = clock.unix_timestamp;
    
    // Log backup code generation
    user_auth.add_security_event(
        SecurityEventType::TwoFactorEnabled,
        None,
        None,
        "Backup codes generated".to_string(),
        30, // Medium risk
    )?;
    
    msg!("Backup codes generated for user: {}", user);
    
    Ok(backup_codes)
}

/// Verify backup code for account recovery
pub fn verify_backup_code(
    ctx: Context<VerifyAuthFactor>,
    backup_code: String,
) -> Result<()> {
    let user_auth = &mut ctx.accounts.user_auth;
    let user = ctx.accounts.user.key();
    
    // Verify user owns the account
    if user != user_auth.user {
        return Err(VaultError::UnauthorizedAccess.into());
    }
    
    // Find backup code in any auth factor
    let mut code_found = false;
    for factor in &mut user_auth.auth_factors {
        if let Some(pos) = factor.backup_codes.iter().position(|c| c == &backup_code) {
            // Remove used backup code
            factor.backup_codes.remove(pos);
            code_found = true;
            break;
        }
    }
    
    if !code_found {
        user_auth.add_security_event(
            SecurityEventType::LoginFailure,
            None,
            None,
            "Invalid backup code used".to_string(),
            70, // High risk
        )?;
        
        return Err(VaultError::InvalidBackupCode.into());
    }
    
    // Unlock account if it was locked
    if user_auth.account_status == AccountStatus::Locked {
        user_auth.account_status = AccountStatus::Active;
        user_auth.locked_until = None;
    }
    
    user_auth.add_security_event(
        SecurityEventType::RecoveryInitiated,
        None,
        None,
        "Account recovered using backup code".to_string(),
        40, // Medium-high risk
    )?;
    
    user_auth.updated_at = Clock::get()?.unix_timestamp;
    
    msg!("Account recovered using backup code for user: {}", user);
    
    Ok(())
}

/// Update user security settings
pub fn update_security_settings(
    ctx: Context<AddAuthFactor>,
    require_2fa_for_all: Option<bool>,
    require_2fa_for_payments: Option<bool>,
    require_2fa_for_high_value: Option<bool>,
    session_timeout: Option<u32>,
    max_concurrent_sessions: Option<u8>,
    auto_lock_on_suspicious: Option<bool>,
) -> Result<()> {
    let user_auth = &mut ctx.accounts.user_auth;
    let user = ctx.accounts.user.key();
    
    // Verify user owns the account
    if user != user_auth.user {
        return Err(VaultError::UnauthorizedAccess.into());
    }
    
    let settings = &mut user_auth.security_settings;
    
    if let Some(require_all) = require_2fa_for_all {
        settings.require_2fa_for_all = require_all;
    }
    
    if let Some(require_payments) = require_2fa_for_payments {
        settings.require_2fa_for_payments = require_payments;
    }
    
    if let Some(require_high_value) = require_2fa_for_high_value {
        settings.require_2fa_for_high_value = require_high_value;
    }
    
    if let Some(timeout) = session_timeout {
        // Validate timeout is within allowed range
        let auth_config = &ctx.accounts.auth_config;
        if timeout >= auth_config.session_timeout_min && timeout <= auth_config.session_timeout_max {
            settings.session_timeout = timeout;
        } else {
            return Err(VaultError::InvalidSessionTimeout.into());
        }
    }
    
    if let Some(max_sessions) = max_concurrent_sessions {
        if max_sessions > 0 && max_sessions <= 10 {
            settings.max_concurrent_sessions = max_sessions;
        } else {
            return Err(VaultError::InvalidSessionLimit.into());
        }
    }
    
    if let Some(auto_lock) = auto_lock_on_suspicious {
        settings.auto_lock_on_suspicious = auto_lock;
    }
    
    user_auth.updated_at = Clock::get()?.unix_timestamp;
    
    user_auth.add_security_event(
        SecurityEventType::LoginSuccess,
        None,
        None,
        "Security settings updated".to_string(),
        20, // Medium risk
    )?;
    
    msg!("Security settings updated for user: {}", user);
    
    Ok(())
}