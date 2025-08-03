use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
pub struct DistributeRewards<'info> {
    #[account(
        mut,
        seeds = [b"staking_pool"],
        bump = staking_pool.bump
    )]
    pub staking_pool: Account<'info, StakingPool>,
    
    #[account(
        mut,
        seeds = [b"treasury"],
        bump = treasury.bump
    )]
    pub treasury: Account<'info, Treasury>,
    
    #[account(
        mut,
        seeds = [b"user_account", user.key().as_ref()],
        bump = user_account.bump
    )]
    pub user_account: Account<'info, UserAccount>,
    
    pub user: Signer<'info>,
    pub authority: Signer<'info>,
}

pub fn distribute_rewards(ctx: Context<DistributeRewards>) -> Result<()> {
    let staking_pool = &mut ctx.accounts.staking_pool;
    let treasury = &mut ctx.accounts.treasury;
    let user_account = &mut ctx.accounts.user_account;

    // Calculate user rewards based on 1:2 ratio
    // For every $1 of BTC committed, user gets rewards from $1 of staked assets
    let user_btc_commitment = user_account.btc_commitment_amount;
    let total_rewards = staking_pool.rewards_accumulated;
    
    // 50% of staking profits go to users
    let user_share = total_rewards.checked_div(2).unwrap();
    
    // Calculate this user's portion based on their commitment
    // This is simplified - full implementation would track all users
    let user_rewards = user_share; // Placeholder calculation
    
    // Update user reward balance
    user_account.reward_balance = user_account.reward_balance
        .checked_add(user_rewards).unwrap();
    
    // Update treasury user rewards pool
    treasury.user_rewards_pool = treasury.user_rewards_pool
        .checked_add(user_rewards).unwrap();

    Ok(())
}