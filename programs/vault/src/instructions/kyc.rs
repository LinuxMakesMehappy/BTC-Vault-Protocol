use anchor_lang::prelude::*;
use crate::state::*;
use crate::traits::ComplianceTier;

#[derive(Accounts)]
pub struct VerifyKYC<'info> {
    #[account(
        init_if_needed,
        payer = authority,
        space = KYCStatus::LEN,
        seeds = [b"kyc_status", user.key().as_ref()],
        bump
    )]
    pub kyc_status: Account<'info, KYCStatus>,
    
    #[account(
        mut,
        seeds = [b"user_account", user.key().as_ref()],
        bump = user_account.bump
    )]
    pub user_account: Account<'info, UserAccount>,
    
    /// CHECK: User being verified
    pub user: AccountInfo<'info>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn verify_user(
    ctx: Context<VerifyKYC>,
    documents: Vec<u8>,
) -> Result<()> {
    let kyc_status = &mut ctx.accounts.kyc_status;
    let user_account = &mut ctx.accounts.user_account;
    let clock = Clock::get()?;

    // Hash the documents for privacy
    let documents_hash = solana_program::hash::hash(&documents);

    // Initialize KYC status
    kyc_status.user = ctx.accounts.user.key();
    kyc_status.tier = ComplianceTier::KYCVerified;
    kyc_status.verification_date = clock.unix_timestamp;
    kyc_status.chainalysis_score = 0; // Would be set by Chainalysis integration
    kyc_status.commitment_limit = u64::MAX; // No limit for KYC verified users
    kyc_status.documents_hash = documents_hash.to_bytes();
    kyc_status.verified_by = ctx.accounts.authority.key();
    kyc_status.bump = ctx.bumps.kyc_status;

    // Update user account KYC tier
    user_account.kyc_tier = ComplianceTier::KYCVerified;

    Ok(())
}