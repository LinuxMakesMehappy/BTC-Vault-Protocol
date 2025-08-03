use anchor_lang::prelude::*;
use crate::traits::ComplianceTier;

#[account]
pub struct KYCStatus {
    pub user: Pubkey,
    pub tier: ComplianceTier,
    pub verification_date: i64,
    pub chainalysis_score: u8,
    pub commitment_limit: u64,
    pub documents_hash: [u8; 32],
    pub verified_by: Pubkey,
    pub bump: u8,
}

impl KYCStatus {
    pub const LEN: usize = 8 + // discriminator
        32 + // user
        1 + // tier
        8 + // verification_date
        1 + // chainalysis_score
        8 + // commitment_limit
        32 + // documents_hash
        32 + // verified_by
        1; // bump

    pub const NON_KYC_LIMIT: u64 = 100_000_000; // 1 BTC in satoshis
}