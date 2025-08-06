use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::VaultError;

#[derive(Accounts)]
pub struct InitializeKYCProfile<'info> {
    #[account(
        init,
        payer = user,
        space = KYCProfile::LEN,
        seeds = [b"kyc_profile", user.key().as_ref()],
        bump
    )]
    pub kyc_profile: Account<'info, KYCProfile>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct StartKYCVerification<'info> {
    #[account(
        mut,
        seeds = [b"kyc_profile", user.key().as_ref()],
        bump = kyc_profile.bump
    )]
    pub kyc_profile: Account<'info, KYCProfile>,
    
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct SubmitKYCDocument<'info> {
    #[account(
        mut,
        seeds = [b"kyc_profile", user.key().as_ref()],
        bump = kyc_profile.bump
    )]
    pub kyc_profile: Account<'info, KYCProfile>,
    
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct VerifyKYCDocument<'info> {
    #[account(
        mut,
        seeds = [b"kyc_profile", user.key().as_ref()],
        bump = kyc_profile.bump
    )]
    pub kyc_profile: Account<'info, KYCProfile>,
    
    #[account(
        seeds = [b"multisig_wallet"],
        bump = multisig_wallet.bump
    )]
    pub multisig_wallet: Account<'info, MultisigWallet>,
    
    #[account(mut)]
    pub compliance_officer: Signer<'info>,
    
    /// CHECK: User account being verified
    pub user: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdateComplianceScreening<'info> {
    #[account(
        mut,
        seeds = [b"kyc_profile", user.key().as_ref()],
        bump = kyc_profile.bump
    )]
    pub kyc_profile: Account<'info, KYCProfile>,
    
    #[account(
        seeds = [b"multisig_wallet"],
        bump = multisig_wallet.bump
    )]
    pub multisig_wallet: Account<'info, MultisigWallet>,
    
    #[account(mut)]
    pub compliance_officer: Signer<'info>,
    
    /// CHECK: User account being screened
    pub user: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ApproveKYC<'info> {
    #[account(
        mut,
        seeds = [b"kyc_profile", user.key().as_ref()],
        bump = kyc_profile.bump
    )]
    pub kyc_profile: Account<'info, KYCProfile>,
    
    #[account(
        seeds = [b"multisig_wallet"],
        bump = multisig_wallet.bump
    )]
    pub multisig_wallet: Account<'info, MultisigWallet>,
    
    #[account(mut)]
    pub compliance_officer: Signer<'info>,
    
    /// CHECK: User account being approved
    pub user: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct RejectKYC<'info> {
    #[account(
        mut,
        seeds = [b"kyc_profile", user.key().as_ref()],
        bump = kyc_profile.bump
    )]
    pub kyc_profile: Account<'info, KYCProfile>,
    
    #[account(
        seeds = [b"multisig_wallet"],
        bump = multisig_wallet.bump
    )]
    pub multisig_wallet: Account<'info, MultisigWallet>,
    
    #[account(mut)]
    pub compliance_officer: Signer<'info>,
    
    /// CHECK: User account being rejected
    pub user: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CheckCommitmentEligibility<'info> {
    #[account(
        seeds = [b"kyc_profile", user.key().as_ref()],
        bump = kyc_profile.bump
    )]
    pub kyc_profile: Account<'info, KYCProfile>,
    
    /// CHECK: User account being checked
    pub user: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SuspendKYC<'info> {
    #[account(
        mut,
        seeds = [b"kyc_profile", user.key().as_ref()],
        bump = kyc_profile.bump
    )]
    pub kyc_profile: Account<'info, KYCProfile>,
    
    #[account(
        seeds = [b"multisig_wallet"],
        bump = multisig_wallet.bump
    )]
    pub multisig_wallet: Account<'info, MultisigWallet>,
    
    #[account(mut)]
    pub compliance_officer: Signer<'info>,
    
    /// CHECK: User account being suspended
    pub user: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct GenerateComplianceReport<'info> {
    #[account(
        init,
        payer = compliance_officer,
        space = ComplianceReport::LEN,
        seeds = [b"compliance_report", &report_id.to_le_bytes()],
        bump
    )]
    pub compliance_report: Account<'info, ComplianceReport>,
    
    #[account(
        seeds = [b"multisig_wallet"],
        bump = multisig_wallet.bump
    )]
    pub multisig_wallet: Account<'info, MultisigWallet>,
    
    #[account(mut)]
    pub compliance_officer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

/// Initialize a KYC profile for a user
pub fn initialize_kyc_profile(ctx: Context<InitializeKYCProfile>) -> Result<()> {
    let kyc_profile = &mut ctx.accounts.kyc_profile;
    let user = ctx.accounts.user.key();
    
    kyc_profile.initialize(user, ctx.bumps.kyc_profile)?;
    
    msg!("KYC profile initialized for user {}", user);
    
    Ok(())
}

/// Start KYC verification process for a specific tier
pub fn start_kyc_verification(
    ctx: Context<StartKYCVerification>,
    target_tier: KYCTier,
) -> Result<()> {
    let kyc_profile = &mut ctx.accounts.kyc_profile;
    
    kyc_profile.start_kyc_verification(target_tier)?;
    
    Ok(())
}

/// Submit a KYC document for verification
pub fn submit_kyc_document(
    ctx: Context<SubmitKYCDocument>,
    document_type: DocumentType,
    document_hash: [u8; 32],
) -> Result<()> {
    let kyc_profile = &mut ctx.accounts.kyc_profile;
    
    kyc_profile.submit_document(document_type, document_hash)?;
    
    Ok(())
}

/// Verify a submitted KYC document (compliance officer only)
pub fn verify_kyc_document(
    ctx: Context<VerifyKYCDocument>,
    document_type: DocumentType,
    expiry_date: Option<i64>,
) -> Result<()> {
    let kyc_profile = &mut ctx.accounts.kyc_profile;
    let multisig_wallet = &ctx.accounts.multisig_wallet;
    let compliance_officer = ctx.accounts.compliance_officer.key();
    
    // Verify compliance officer is authorized
    if !is_compliance_officer(&multisig_wallet, &compliance_officer)? {
        return Err(VaultError::UnauthorizedComplianceOfficer.into());
    }
    
    kyc_profile.verify_document(document_type, compliance_officer, expiry_date)?;
    
    Ok(())
}

/// Update compliance screening results from Chainalysis
pub fn update_compliance_screening(
    ctx: Context<UpdateComplianceScreening>,
    screening: ComplianceScreening,
) -> Result<()> {
    let kyc_profile = &mut ctx.accounts.kyc_profile;
    let multisig_wallet = &ctx.accounts.multisig_wallet;
    let compliance_officer = ctx.accounts.compliance_officer.key();
    
    // Verify compliance officer is authorized
    if !is_compliance_officer(&multisig_wallet, &compliance_officer)? {
        return Err(VaultError::UnauthorizedComplianceOfficer.into());
    }
    
    kyc_profile.update_compliance_screening(screening)?;
    
    Ok(())
}

/// Approve KYC for a specific tier (compliance officer only)
pub fn approve_kyc(
    ctx: Context<ApproveKYC>,
    tier: KYCTier,
    expiry_months: Option<u32>,
) -> Result<()> {
    let kyc_profile = &mut ctx.accounts.kyc_profile;
    let multisig_wallet = &ctx.accounts.multisig_wallet;
    let compliance_officer = ctx.accounts.compliance_officer.key();
    
    // Verify compliance officer is authorized
    if !is_compliance_officer(&multisig_wallet, &compliance_officer)? {
        return Err(VaultError::UnauthorizedComplianceOfficer.into());
    }
    
    kyc_profile.approve_kyc(tier, compliance_officer, expiry_months)?;
    
    Ok(())
}

/// Reject KYC application (compliance officer only)
pub fn reject_kyc(
    ctx: Context<RejectKYC>,
    reason: String,
) -> Result<()> {
    let kyc_profile = &mut ctx.accounts.kyc_profile;
    let multisig_wallet = &ctx.accounts.multisig_wallet;
    let compliance_officer = ctx.accounts.compliance_officer.key();
    
    // Verify compliance officer is authorized
    if !is_compliance_officer(&multisig_wallet, &compliance_officer)? {
        return Err(VaultError::UnauthorizedComplianceOfficer.into());
    }
    
    if reason.len() > KYCProfile::MAX_NOTES_LENGTH {
        return Err(VaultError::ReasonTooLong.into());
    }
    
    kyc_profile.reject_kyc(compliance_officer, reason)?;
    
    Ok(())
}

/// Check if user can commit a specific amount based on KYC status
pub fn check_commitment_eligibility(
    ctx: Context<CheckCommitmentEligibility>,
    amount: u64,
) -> Result<()> {
    let kyc_profile = &ctx.accounts.kyc_profile;
    
    let can_commit = kyc_profile.can_commit(amount)?;
    
    if !can_commit {
        return Err(VaultError::CommitmentExceedsKYCLimit.into());
    }
    
    msg!("User {} can commit {} satoshis", kyc_profile.user, amount);
    
    Ok(())
}

/// Suspend KYC due to compliance issues (compliance officer only)
pub fn suspend_kyc(
    ctx: Context<SuspendKYC>,
    reason: String,
) -> Result<()> {
    let kyc_profile = &mut ctx.accounts.kyc_profile;
    let multisig_wallet = &ctx.accounts.multisig_wallet;
    let compliance_officer = ctx.accounts.compliance_officer.key();
    
    // Verify compliance officer is authorized
    if !is_compliance_officer(&multisig_wallet, &compliance_officer)? {
        return Err(VaultError::UnauthorizedComplianceOfficer.into());
    }
    
    if reason.len() > KYCProfile::MAX_NOTES_LENGTH {
        return Err(VaultError::ReasonTooLong.into());
    }
    
    kyc_profile.suspend_kyc(compliance_officer, reason)?;
    
    Ok(())
}

/// Generate compliance report (compliance officer only)
pub fn generate_compliance_report(
    ctx: Context<GenerateComplianceReport>,
    report_id: u64,
    report_type: ComplianceReportType,
    period_start: i64,
    period_end: i64,
    total_users: u64,
    kyc_approved_users: u64,
    high_risk_users: u64,
    suspicious_activities: u64,
    data_hash: [u8; 32],
) -> Result<()> {
    let compliance_report = &mut ctx.accounts.compliance_report;
    let multisig_wallet = &ctx.accounts.multisig_wallet;
    let compliance_officer = ctx.accounts.compliance_officer.key();
    
    // Verify compliance officer is authorized
    if !is_compliance_officer(&multisig_wallet, &compliance_officer)? {
        return Err(VaultError::UnauthorizedComplianceOfficer.into());
    }
    
    let clock = Clock::get()?;
    
    compliance_report.report_id = report_id;
    compliance_report.report_type = report_type;
    compliance_report.period_start = period_start;
    compliance_report.period_end = period_end;
    compliance_report.total_users = total_users;
    compliance_report.kyc_approved_users = kyc_approved_users;
    compliance_report.high_risk_users = high_risk_users;
    compliance_report.suspicious_activities = suspicious_activities;
    compliance_report.generated_by = compliance_officer;
    compliance_report.generated_at = clock.unix_timestamp;
    compliance_report.data_hash = data_hash;
    compliance_report.bump = ctx.bumps.compliance_report;
    
    msg!("Compliance report {} generated by officer {}", report_id, compliance_officer);
    
    Ok(())
}

/// Integrate KYC check with BTC commitment
pub fn validate_btc_commitment_kyc(
    kyc_profile: &KYCProfile,
    commitment_amount: u64,
) -> Result<()> {
    // Check if user can commit this amount
    if !kyc_profile.can_commit(commitment_amount)? {
        return Err(VaultError::CommitmentExceedsKYCLimit.into());
    }
    
    // Check daily limit
    if !kyc_profile.check_daily_limit(commitment_amount)? {
        return Err(VaultError::DailyLimitExceeded.into());
    }
    
    // Check if KYC needs renewal
    if kyc_profile.needs_renewal()? {
        msg!("Warning: KYC for user {} needs renewal", kyc_profile.user);
    }
    
    Ok(())
}

/// Integrate KYC check with payment system
pub fn validate_payment_kyc(
    kyc_profile: &KYCProfile,
    payment_amount: u64,
) -> Result<()> {
    // For payments, we use the same daily limit check
    if !kyc_profile.check_daily_limit(payment_amount)? {
        return Err(VaultError::DailyLimitExceeded.into());
    }
    
    // Additional checks for high-risk users
    if let Some(screening) = &kyc_profile.compliance_screening {
        if screening.risk_level == RiskLevel::High {
            // High-risk users have additional restrictions
            if payment_amount > 50_000_000 { // 0.5 BTC
                return Err(VaultError::HighRiskUserRestriction.into());
            }
        }
    }
    
    Ok(())
}

/// Chainalysis API integration for compliance screening
pub fn perform_chainalysis_screening(
    user_address: &Pubkey,
    btc_address: &str,
) -> Result<ComplianceScreening> {
    // In production, this would make actual API calls to Chainalysis
    // For now, we simulate the screening process
    
    let clock = Clock::get()?;
    
    // Simulate screening logic
    let risk_level = simulate_risk_assessment(btc_address);
    let sanctions_match = simulate_sanctions_check(btc_address);
    let pep_match = simulate_pep_check(user_address);
    let adverse_media = simulate_adverse_media_check(user_address);
    
    let screening = ComplianceScreening {
        screening_id: format!("CHA_{}", clock.unix_timestamp),
        risk_level,
        sanctions_match,
        pep_match,
        adverse_media,
        screening_date: clock.unix_timestamp,
        expiry_date: clock.unix_timestamp + (90 * 24 * 3600), // 90 days
        notes: "Automated screening via Chainalysis API".to_string(),
    };
    
    msg!("Chainalysis screening completed for user {} with risk level {:?}", 
         user_address, screening.risk_level);
    
    Ok(screening)
}

// Helper functions

fn is_compliance_officer(multisig_wallet: &MultisigWallet, officer: &Pubkey) -> Result<bool> {
    // Check if the officer is an authorized signer with compliance role
    let is_authorized = multisig_wallet.signers
        .iter()
        .any(|signer| {
            signer.pubkey == *officer && 
            signer.is_active && 
            matches!(signer.role, SignerRole::Admin | SignerRole::Operator)
        });
    
    Ok(is_authorized)
}

// Simulation functions for Chainalysis integration (replace with actual API calls in production)

fn simulate_risk_assessment(btc_address: &str) -> RiskLevel {
    // Simulate risk assessment based on address characteristics
    let address_hash = btc_address.len() % 4;
    match address_hash {
        0 => RiskLevel::Low,
        1 => RiskLevel::Medium,
        2 => RiskLevel::High,
        _ => RiskLevel::Low,
    }
}

fn simulate_sanctions_check(btc_address: &str) -> bool {
    // Simulate sanctions screening
    // In production, this would check against OFAC and other sanctions lists
    btc_address.contains("sanction") // Simple simulation
}

fn simulate_pep_check(user_address: &Pubkey) -> bool {
    // Simulate Politically Exposed Person check
    // In production, this would check against PEP databases
    user_address.to_string().contains("pep") // Simple simulation
}

fn simulate_adverse_media_check(user_address: &Pubkey) -> bool {
    // Simulate adverse media screening
    // In production, this would check news and media sources
    user_address.to_string().contains("adverse") // Simple simulation
}

/// Get KYC tier requirements for frontend display
pub fn get_kyc_tier_info(tier: KYCTier) -> (Vec<DocumentType>, u64, u64) {
    let requirements = KYCProfile::get_tier_requirements(&tier);
    let (commitment_limit, daily_limit) = KYCProfile::get_tier_limits(&tier);
    
    (requirements, commitment_limit, daily_limit)
}

/// Check if user needs to upgrade KYC tier for a commitment
pub fn check_kyc_upgrade_needed(
    current_tier: &KYCTier,
    commitment_amount: u64,
) -> Option<KYCTier> {
    let (current_limit, _) = KYCProfile::get_tier_limits(current_tier);
    
    if commitment_amount <= current_limit {
        return None; // No upgrade needed
    }
    
    // Determine minimum required tier
    if commitment_amount <= 1_000_000_000 { // 10 BTC
        Some(KYCTier::Basic)
    } else if commitment_amount <= 10_000_000_000 { // 100 BTC
        Some(KYCTier::Enhanced)
    } else {
        Some(KYCTier::Institutional)
    }
}