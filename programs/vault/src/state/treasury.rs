use anchor_lang::prelude::*;

#[account]
pub struct Treasury {
    pub total_assets: u64,
    pub sol_balance: u64,
    pub eth_balance: u64,
    pub atom_balance: u64,
    pub staking_rewards: u64,
    pub user_rewards_pool: u64,
    pub last_deposit: i64,
    pub next_deposit: i64,
    pub deposit_amount: u64, // $50 USD equivalent
    pub deposit_frequency: u32, // 14 days in seconds
    pub bump: u8,
}

impl Treasury {
    pub const LEN: usize = 8 + // discriminator
        8 + // total_assets
        8 + // sol_balance
        8 + // eth_balance
        8 + // atom_balance
        8 + // staking_rewards
        8 + // user_rewards_pool
        8 + // last_deposit
        8 + // next_deposit
        8 + // deposit_amount
        4 + // deposit_frequency
        1; // bump

    pub const DEPOSIT_FREQUENCY_SECONDS: u32 = 14 * 24 * 60 * 60; // 14 days
}