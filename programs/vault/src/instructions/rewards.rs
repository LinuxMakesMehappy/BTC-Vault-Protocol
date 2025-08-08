use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::VaultError;
use crate::traits::PaymentType;

#[derive(Accounts)]
pub struct CalculateRewards<'info> {
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
    
    #[account(mut)]
    pub authority: Signer<'info>,
}

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

#[derive(Accounts)]
pub struct ClaimRewards<'info> {
    #[account(
        mut,
        seeds = [b"user_account", user.key().as_ref()],
        bump = user_account.bump
    )]
    pub user_account: Account<'info, UserAccount>,
    
    #[account(
        mut,
        seeds = [b"treasury"],
        bump = treasury.bump
    )]
    pub treasury: Account<'info, Treasury>,
    
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateRewardRates<'info> {
    #[account(
        mut,
        seeds = [b"staking_pool"],
        bump = staking_pool.bump
    )]
    pub staking_pool: Account<'info, StakingPool>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
}

/// Calculate rewards based on staking performance and distribute according to 1:2 ratio
pub fn calculate_rewards(
    ctx: Context<CalculateRewards>,
    total_staking_rewards: u64,
    _total_btc_commitments: u64,
) -> Result<()> {
    let staking_pool = &mut ctx.accounts.staking_pool;
    let treasury = &mut ctx.accounts.treasury;

    // Validate inputs
    if total_staking_rewards == 0 {
        return Ok(()); // No rewards to calculate
    }

    // Calculate protocol share (50%) and user share (50%)
    let protocol_share = total_staking_rewards.checked_div(2).unwrap();
    let user_share = total_staking_rewards.checked_sub(protocol_share).unwrap();

    // Update staking pool rewards
    staking_pool.rewards_accumulated = staking_pool.rewards_accumulated
        .checked_add(total_staking_rewards).unwrap();
    
    // Update treasury balances
    treasury.staking_rewards = treasury.staking_rewards
        .checked_add(protocol_share).unwrap();
    treasury.user_rewards_pool = treasury.user_rewards_pool
        .checked_add(user_share).unwrap();

    // Update calculation timestamp
    let clock = Clock::get()?;
    staking_pool.last_reward_calculation = clock.unix_timestamp;

    msg!("Calculated rewards: Total {}, Protocol {}, Users {}", 
         total_staking_rewards, protocol_share, user_share);

    Ok(())
}

/// Distribute rewards to a specific user based on their BTC commitment
pub fn distribute_rewards(ctx: Context<DistributeRewards>) -> Result<()> {
    let staking_pool = &mut ctx.accounts.staking_pool;
    let treasury = &mut ctx.accounts.treasury;
    let user_account = &mut ctx.accounts.user_account;

    // Get user's BTC commitment amount
    let user_btc_commitment = user_account.btc_commitment_amount;
    
    if user_btc_commitment == 0 {
        return Err(VaultError::InsufficientBalance.into());
    }

    // Calculate user's reward based on 1:2 ratio
    // For every $1 of BTC committed, user gets rewards from $1 of staked assets
    let user_staked_equivalent = user_btc_commitment; // 1:1 mapping for staked assets
    
    // Calculate reward rate based on total staking performance
    let total_staked = staking_pool.total_staked;
    let total_user_rewards = treasury.user_rewards_pool;
    
    if total_staked == 0 {
        return Err(VaultError::StakingFailed.into());
    }

    // Calculate user's proportional share of rewards
    let user_reward_share = (user_staked_equivalent * total_user_rewards) / total_staked;
    
    // Apply 1:2 ratio: user gets rewards as if they staked equivalent amount
    let user_rewards = user_reward_share;

    // Validate sufficient rewards pool
    if treasury.user_rewards_pool < user_rewards {
        return Err(VaultError::InsufficientBalance.into());
    }

    // Update user reward balance
    user_account.reward_balance = user_account.reward_balance
        .checked_add(user_rewards).unwrap();
    
    // Deduct from treasury user rewards pool
    treasury.user_rewards_pool = treasury.user_rewards_pool
        .checked_sub(user_rewards).unwrap();

    // Update staking pool distributed amount
    staking_pool.rewards_distributed = staking_pool.rewards_distributed
        .checked_add(user_rewards).unwrap();

    msg!("Distributed {} rewards to user with {} BTC commitment", 
         user_rewards, user_btc_commitment);

    Ok(())
}

/// Allow users to claim their accumulated rewards
pub fn claim_rewards(
    ctx: Context<ClaimRewards>,
    payment_type: PaymentType,
) -> Result<()> {
    let user_account = &mut ctx.accounts.user_account;
    let _treasury = &mut ctx.accounts.treasury;

    let claimable_rewards = user_account.reward_balance;
    
    if claimable_rewards == 0 {
        return Err(VaultError::InsufficientBalance.into());
    }

    // Process payment based on user preference
    match payment_type {
        PaymentType::BTC => {
            // Process Lightning Network payment (default)
            process_btc_payment(claimable_rewards)?;
        },
        PaymentType::USDC => {
            // Process USDC payment
            process_usdc_payment(claimable_rewards)?;
        },
        PaymentType::AutoReinvest => {
            // Auto-reinvest rewards back into the protocol
            process_auto_reinvestment(user_account, claimable_rewards)?;
        }
    }

    // Clear user's reward balance
    user_account.reward_balance = 0;

    // Update user's payment preference for future rewards
    user_account.payment_preference = payment_type;

    msg!("User claimed {} rewards via {:?}", claimable_rewards, payment_type);

    Ok(())
}

/// Update reward calculation rates and parameters
pub fn update_reward_rates(
    ctx: Context<UpdateRewardRates>,
    new_user_share_bps: u16, // Basis points (e.g., 5000 = 50%)
) -> Result<()> {
    let _staking_pool = &mut ctx.accounts.staking_pool;

    // Validate basis points (max 10000 = 100%)
    if new_user_share_bps > 10000 {
        return Err(VaultError::InvalidAllocation.into());
    }

    // For now, we maintain 50% user share as per requirements
    // This function allows for future flexibility
    if new_user_share_bps != 5000 {
        return Err(VaultError::InvalidAllocation.into());
    }

    msg!("Reward rates updated: User share {}%", new_user_share_bps / 100);

    Ok(())
}

/// Process BTC payment via Lightning Network with fallback
fn process_btc_payment(amount: u64) -> Result<()> {
    // In production, this would:
    // 1. Attempt Lightning Network payment
    // 2. Fallback to on-chain BTC if LN fails
    // 3. Queue for retry if both fail
    
    msg!("Processing BTC payment of {} via Lightning Network", amount);
    
    // Simulate Lightning Network payment
    // In reality, this would integrate with Lightning infrastructure
    Ok(())
}

/// Process USDC payment
fn process_usdc_payment(amount: u64) -> Result<()> {
    // In production, this would:
    // 1. Convert rewards to USDC equivalent
    // 2. Transfer USDC to user's wallet
    // 3. Handle conversion rate fluctuations
    
    msg!("Processing USDC payment of {} USD equivalent", amount);
    
    // Simulate USDC payment
    Ok(())
}

/// Process auto-reinvestment of rewards
fn process_auto_reinvestment(_user_account: &mut UserAccount, amount: u64) -> Result<()> {
    // In production, this would:
    // 1. Add rewards to user's effective BTC commitment
    // 2. Update staking allocations accordingly
    // 3. Compound future reward calculations
    
    msg!("Auto-reinvesting {} rewards for user", amount);
    
    // For now, we simulate by keeping rewards in the system
    // In full implementation, this would increase user's effective stake
    Ok(())
}

/// Calculate individual user rewards based on commitment and time
pub fn calculate_user_rewards(
    user_btc_commitment: u64,
    total_btc_commitments: u64,
    total_rewards_pool: u64,
    commitment_duration_days: u64,
) -> Result<u64> {
    if total_btc_commitments == 0 || user_btc_commitment == 0 {
        return Ok(0);
    }

    // Base reward calculation: proportional to commitment size
    let base_reward = (user_btc_commitment * total_rewards_pool) / total_btc_commitments;
    
    // Apply time-based multiplier (longer commitments get slightly higher rewards)
    let time_multiplier = if commitment_duration_days >= 365 {
        110 // 10% bonus for 1+ year commitments
    } else if commitment_duration_days >= 90 {
        105 // 5% bonus for 3+ month commitments
    } else {
        100 // No bonus for short-term commitments
    };
    
    let final_reward = (base_reward * time_multiplier) / 100;
    
    Ok(final_reward)
}

/// Validate reward distribution parameters
pub fn validate_reward_distribution(
    total_staking_rewards: u64,
    total_btc_commitments: u64,
    user_btc_commitment: u64,
) -> Result<()> {
    // Ensure we have staking rewards to distribute
    if total_staking_rewards == 0 {
        return Err(VaultError::RewardCalculationError.into());
    }

    // Ensure there are BTC commitments
    if total_btc_commitments == 0 {
        return Err(VaultError::RewardCalculationError.into());
    }

    // Ensure user has a valid commitment
    if user_btc_commitment == 0 {
        return Err(VaultError::InsufficientBalance.into());
    }

    // Ensure user commitment doesn't exceed total (sanity check)
    if user_btc_commitment > total_btc_commitments {
        return Err(VaultError::RewardCalculationError.into());
    }

    Ok(())
}
