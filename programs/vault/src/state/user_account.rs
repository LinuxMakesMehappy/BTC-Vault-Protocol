use anchor_lang::prelude::*;
use crate::traits::{PaymentType, ComplianceTier};

#[account]
pub struct UserAccount {
    pub owner: Pubkey,
    pub btc_commitment_amount: u64,
    pub btc_address: String,
    pub reward_balance: u64,
    pub kyc_tier: ComplianceTier,
    pub payment_preference: PaymentType,
    pub auto_reinvest: bool,
    pub two_fa_enabled: bool,
    pub created_at: i64,
    pub last_activity: i64,
    pub bump: u8,
}

impl UserAccount {
    pub const LEN: usize = 8 + // discriminator
        32 + // owner
        8 + // btc_commitment_amount
        4 + 64 + // btc_address (max 64 chars)
        8 + // reward_balance
        1 + // kyc_tier
        1 + // payment_preference
        1 + // auto_reinvest
        1 + // two_fa_enabled
        8 + // created_at
        8 + // last_activity
        1; // bump
}
