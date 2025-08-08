use anchor_lang::prelude::*;

/// User account state for tracking user-specific data
#[account]
#[derive(Debug)]
pub struct UserAccount {
    pub owner: Pubkey,
    pub total_btc_committed: u64,
    pub total_rewards_earned: u64,
    pub total_rewards_claimed: u64,
    pub last_activity: i64,
    pub kyc_status: u8, // 0 = None, 1 = Basic, 2 = Enhanced
    pub kyc_tier: u8, // KYC tier level
    pub risk_score: u16,
    pub btc_commitment_amount: u64,
    pub btc_address: String,
    pub created_at: i64,
    pub bump: u8,
}

impl UserAccount {
    pub const LEN: usize = 8 + // discriminator
        32 + // owner
        8 + // total_btc_committed
        8 + // total_rewards_earned
        8 + // total_rewards_claimed
        8 + // last_activity
        1 + // kyc_status
        1 + // kyc_tier
        2 + // risk_score
        8 + // btc_commitment_amount
        64 + // btc_address (max length)
        8 + // created_at
        1; // bump
}