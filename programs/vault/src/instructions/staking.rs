use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::VaultError;

#[derive(Accounts)]
pub struct InitializeStakingPool<'info> {
    #[account(
        init,
        payer = authority,
        space = StakingPool::LEN,
        seeds = [b"staking_pool"],
        bump
    )]
    pub staking_pool: Account<'info, StakingPool>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct StakeProtocolAssets<'info> {
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
pub struct RebalanceAllocations<'info> {
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
pub struct AddValidator<'info> {
    #[account(
        mut,
        seeds = [b"staking_pool"],
        bump = staking_pool.bump
    )]
    pub staking_pool: Account<'info, StakingPool>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
}

/// Initialize the staking pool with default allocations
pub fn initialize_staking_pool(ctx: Context<InitializeStakingPool>) -> Result<()> {
    let staking_pool = &mut ctx.accounts.staking_pool;
    staking_pool.initialize(ctx.bumps.staking_pool)?;
    
    msg!("Staking pool initialized with allocations: SOL 40%, ETH 30%, ATOM 30%");
    Ok(())
}

/// Stake protocol assets according to target allocations
pub fn stake_protocol_assets(
    ctx: Context<StakeProtocolAssets>,
    total_treasury_usd: u64,
) -> Result<()> {
    let staking_pool = &mut ctx.accounts.staking_pool;
    let treasury = &mut ctx.accounts.treasury;

    // Calculate target allocations based on treasury value
    staking_pool.calculate_target_allocations(total_treasury_usd)?;
    
    // Get current treasury balances (in USD equivalent)
    let sol_usd_value = treasury.sol_balance; // Assume already in USD for simplicity
    let eth_usd_value = treasury.eth_balance;
    let atom_usd_value = treasury.atom_balance;
    
    // Calculate amounts to stake based on target allocations
    let sol_to_stake = if sol_usd_value > staking_pool.sol_allocation.target_amount {
        staking_pool.sol_allocation.target_amount
    } else {
        sol_usd_value
    };
    
    let eth_to_stake = if eth_usd_value > staking_pool.eth_allocation.target_amount {
        staking_pool.eth_allocation.target_amount
    } else {
        eth_usd_value
    };
    
    let atom_to_stake = if atom_usd_value > staking_pool.atom_allocation.target_amount {
        staking_pool.atom_allocation.target_amount
    } else {
        atom_usd_value
    };

    // Validate we have sufficient treasury funds
    if treasury.sol_balance < sol_to_stake {
        return Err(VaultError::InsufficientBalance.into());
    }
    if treasury.eth_balance < eth_to_stake {
        return Err(VaultError::InsufficientBalance.into());
    }
    if treasury.atom_balance < atom_to_stake {
        return Err(VaultError::InsufficientBalance.into());
    }

    // Update staked amounts
    staking_pool.sol_staked = staking_pool.sol_staked.checked_add(sol_to_stake).unwrap();
    staking_pool.eth_staked = staking_pool.eth_staked.checked_add(eth_to_stake).unwrap();
    staking_pool.atom_staked = staking_pool.atom_staked.checked_add(atom_to_stake).unwrap();
    
    // Update current allocations
    let sol_staked = staking_pool.sol_staked;
    let eth_staked = staking_pool.eth_staked;
    let atom_staked = staking_pool.atom_staked;
    staking_pool.update_current_allocations(sol_staked, eth_staked, atom_staked)?;

    // Update treasury balances
    treasury.sol_balance = treasury.sol_balance.checked_sub(sol_to_stake).unwrap();
    treasury.eth_balance = treasury.eth_balance.checked_sub(eth_to_stake).unwrap();
    treasury.atom_balance = treasury.atom_balance.checked_sub(atom_to_stake).unwrap();
    treasury.last_deposit = Clock::get()?.unix_timestamp;

    msg!("Staked assets: SOL {} USD, ETH {} USD, ATOM {} USD", 
         sol_to_stake, eth_to_stake, atom_to_stake);

    Ok(())
}

/// Rebalance allocations to maintain target percentages
pub fn rebalance_allocations(ctx: Context<RebalanceAllocations>) -> Result<()> {
    let staking_pool = &mut ctx.accounts.staking_pool;
    let treasury = &mut ctx.accounts.treasury;

    // Check if rebalancing is needed
    if !staking_pool.needs_rebalancing()? {
        msg!("No rebalancing needed - allocations within threshold");
        return Ok(());
    }

    // Get rebalancing requirements
    let (sol_diff, eth_diff, atom_diff) = staking_pool.get_rebalancing_requirements()?;
    
    msg!("Rebalancing requirements: SOL {}, ETH {}, ATOM {}", sol_diff, eth_diff, atom_diff);

    // Execute rebalancing (simplified - in production would involve actual staking/unstaking)
    if sol_diff > 0 {
        // Need to stake more SOL
        let amount_to_stake = sol_diff as u64;
        if treasury.sol_balance >= amount_to_stake {
            staking_pool.sol_staked = staking_pool.sol_staked.checked_add(amount_to_stake).unwrap();
            treasury.sol_balance = treasury.sol_balance.checked_sub(amount_to_stake).unwrap();
        }
    } else if sol_diff < 0 {
        // Need to unstake SOL
        let amount_to_unstake = (-sol_diff) as u64;
        if staking_pool.sol_staked >= amount_to_unstake {
            staking_pool.sol_staked = staking_pool.sol_staked.checked_sub(amount_to_unstake).unwrap();
            treasury.sol_balance = treasury.sol_balance.checked_add(amount_to_unstake).unwrap();
        }
    }

    if eth_diff > 0 {
        // Need to stake more ETH
        let amount_to_stake = eth_diff as u64;
        if treasury.eth_balance >= amount_to_stake {
            staking_pool.eth_staked = staking_pool.eth_staked.checked_add(amount_to_stake).unwrap();
            treasury.eth_balance = treasury.eth_balance.checked_sub(amount_to_stake).unwrap();
        }
    } else if eth_diff < 0 {
        // Need to unstake ETH
        let amount_to_unstake = (-eth_diff) as u64;
        if staking_pool.eth_staked >= amount_to_unstake {
            staking_pool.eth_staked = staking_pool.eth_staked.checked_sub(amount_to_unstake).unwrap();
            treasury.eth_balance = treasury.eth_balance.checked_add(amount_to_unstake).unwrap();
        }
    }

    if atom_diff > 0 {
        // Need to stake more ATOM
        let amount_to_stake = atom_diff as u64;
        if treasury.atom_balance >= amount_to_stake {
            staking_pool.atom_staked = staking_pool.atom_staked.checked_add(amount_to_stake).unwrap();
            treasury.atom_balance = treasury.atom_balance.checked_sub(amount_to_stake).unwrap();
        }
    } else if atom_diff < 0 {
        // Need to unstake ATOM
        let amount_to_unstake = (-atom_diff) as u64;
        if staking_pool.atom_staked >= amount_to_unstake {
            staking_pool.atom_staked = staking_pool.atom_staked.checked_sub(amount_to_unstake).unwrap();
            treasury.atom_balance = treasury.atom_balance.checked_add(amount_to_unstake).unwrap();
        }
    }

    // Update current allocations and mark as rebalanced
    let sol_staked = staking_pool.sol_staked;
    let eth_staked = staking_pool.eth_staked;
    let atom_staked = staking_pool.atom_staked;
    staking_pool.update_current_allocations(sol_staked, eth_staked, atom_staked)?;
    staking_pool.mark_rebalanced()?;

    msg!("Rebalancing completed");
    Ok(())
}

/// Add a SOL validator to the staking pool
pub fn add_sol_validator(
    ctx: Context<AddValidator>,
    address: String,
    commission: u16,
    performance_score: u16,
) -> Result<()> {
    let staking_pool = &mut ctx.accounts.staking_pool;
    
    let validator = ValidatorInfo {
        address,
        commission,
        stake_amount: 0,
        performance_score,
        is_active: true,
    };
    
    staking_pool.add_sol_validator(validator)?;
    
    msg!("SOL validator added successfully");
    Ok(())
}

/// Add an ETH validator to the staking pool
pub fn add_eth_validator(
    ctx: Context<AddValidator>,
    address: String,
    commission: u16,
    performance_score: u16,
) -> Result<()> {
    let staking_pool = &mut ctx.accounts.staking_pool;
    
    let validator = ValidatorInfo {
        address,
        commission,
        stake_amount: 0,
        performance_score,
        is_active: true,
    };
    
    staking_pool.add_eth_validator(validator)?;
    
    msg!("ETH validator added successfully");
    Ok(())
}

/// Update ATOM staking configuration
pub fn update_atom_config(
    ctx: Context<AddValidator>,
    everstake_validator: String,
    osmosis_validator: String,
) -> Result<()> {
    let staking_pool = &mut ctx.accounts.staking_pool;
    
    let config = AtomStakingConfig {
        everstake_allocation: StakingPool::ATOM_EVERSTAKE_BPS,
        osmosis_allocation: StakingPool::ATOM_OSMOSIS_BPS,
        everstake_validator,
        osmosis_validator,
    };
    
    staking_pool.update_atom_config(config)?;
    
    msg!("ATOM staking configuration updated");
    Ok(())
}