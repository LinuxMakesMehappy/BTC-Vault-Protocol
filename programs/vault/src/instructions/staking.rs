use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
pub struct StakeProtocolAssets<'info> {
    #[account(
        init_if_needed,
        payer = authority,
        space = StakingPool::LEN,
        seeds = [b"staking_pool"],
        bump
    )]
    pub staking_pool: Account<'info, StakingPool>,
    
    #[account(
        mut,
        seeds = [b"treasury"],
        bump = treasury.bump
    )]
    pub treasury: Account<'info, Treasury>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn stake_protocol_assets(
    ctx: Context<StakeProtocolAssets>,
    sol_amount: u64,
    eth_amount: u64,
    atom_amount: u64,
) -> Result<()> {
    let staking_pool = &mut ctx.accounts.staking_pool;
    let treasury = &mut ctx.accounts.treasury;
    let clock = Clock::get()?;

    // Initialize staking pool if needed
    if staking_pool.total_staked == 0 {
        staking_pool.sol_allocation = StakingPool::SOL_ALLOCATION_BPS;
        staking_pool.eth_allocation = StakingPool::ETH_ALLOCATION_BPS;
        staking_pool.atom_allocation = StakingPool::ATOM_ALLOCATION_BPS;
        staking_pool.bump = ctx.bumps.staking_pool;
    }

    // Update staked amounts
    staking_pool.sol_staked = staking_pool.sol_staked.checked_add(sol_amount).unwrap();
    staking_pool.eth_staked = staking_pool.eth_staked.checked_add(eth_amount).unwrap();
    staking_pool.atom_staked = staking_pool.atom_staked.checked_add(atom_amount).unwrap();
    
    staking_pool.total_staked = staking_pool.sol_staked
        .checked_add(staking_pool.eth_staked).unwrap()
        .checked_add(staking_pool.atom_staked).unwrap();
    
    staking_pool.last_update = clock.unix_timestamp;

    // Update treasury balances
    treasury.sol_balance = treasury.sol_balance.checked_sub(sol_amount).unwrap();
    treasury.eth_balance = treasury.eth_balance.checked_sub(eth_amount).unwrap();
    treasury.atom_balance = treasury.atom_balance.checked_sub(atom_amount).unwrap();

    Ok(())
}