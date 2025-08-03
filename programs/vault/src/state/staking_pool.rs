use anchor_lang::prelude::*;

#[account]
pub struct StakingPool {
    pub total_staked: u64,
    pub sol_allocation: u32,  // 40% = 4000 basis points
    pub eth_allocation: u32,  // 30% = 3000 basis points
    pub atom_allocation: u32, // 30% = 3000 basis points
    pub sol_staked: u64,
    pub eth_staked: u64,
    pub atom_staked: u64,
    pub rewards_accumulated: u64,
    pub last_update: i64,
    pub bump: u8,
}

impl StakingPool {
    pub const LEN: usize = 8 + // discriminator
        8 + // total_staked
        4 + // sol_allocation
        4 + // eth_allocation
        4 + // atom_allocation
        8 + // sol_staked
        8 + // eth_staked
        8 + // atom_staked
        8 + // rewards_accumulated
        8 + // last_update
        1; // bump

    pub const SOL_ALLOCATION_BPS: u32 = 4000; // 40%
    pub const ETH_ALLOCATION_BPS: u32 = 3000; // 30%
    pub const ATOM_ALLOCATION_BPS: u32 = 3000; // 30%
}