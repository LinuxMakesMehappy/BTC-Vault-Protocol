use anchor_lang::prelude::*;

/// Reward calculation and distribution state
#[account]
#[derive(Debug)]
pub struct RewardPool {
    pub total_rewards: u64,
    pub distributed_rewards: u64,
    pub user_share_bps: u16, // Basis points (10000 = 100%)
    pub protocol_share_bps: u16,
    pub last_distribution: i64,
    pub bump: u8,
}

/// Individual reward calculation
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct RewardCalculation {
    pub user: Pubkey,
    pub calculated_reward: u64,
    pub calculation_timestamp: i64,
    pub btc_commitment_amount: u64,
    pub staking_contribution: u64,
}

impl RewardPool {
    pub const LEN: usize = 8 + // discriminator
        8 + // total_rewards
        8 + // distributed_rewards
        2 + // user_share_bps
        2 + // protocol_share_bps
        8 + // last_distribution
        1; // bump
}