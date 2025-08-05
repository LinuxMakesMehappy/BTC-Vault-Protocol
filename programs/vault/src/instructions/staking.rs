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
    let sol_usd_value = treasury.sol_balance;
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

    // Execute actual staking operations
    if sol_to_stake > 0 {
        stake_sol_assets(staking_pool, sol_to_stake)?;
    }
    
    if eth_to_stake > 0 {
        initiate_eth_l2_staking(staking_pool, eth_to_stake)?;
    }
    
    if atom_to_stake > 0 {
        initiate_atom_staking(staking_pool, atom_to_stake)?;
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

/// Execute SOL native staking with selected validators
fn stake_sol_assets(staking_pool: &mut StakingPool, amount_usd: u64) -> Result<()> {
    // Select best validators for SOL staking
    let selected_validators = staking_pool.select_best_sol_validators(3);
    
    if selected_validators.is_empty() {
        return Err(VaultError::NoValidatorsAvailable.into());
    }
    
    // Distribute stake amount across selected validators
    let stake_per_validator = amount_usd / selected_validators.len() as u64;
    let remainder = amount_usd % selected_validators.len() as u64;
    
    for (i, validator) in selected_validators.iter().enumerate() {
        let stake_amount = if i == 0 {
            stake_per_validator + remainder // Give remainder to first validator
        } else {
            stake_per_validator
        };
        
        // In production, this would create actual stake accounts
        // For now, we simulate the staking operation
        msg!("Staking {} USD worth of SOL to validator: {}", stake_amount, validator.address);
        
        // Update validator stake amount (this would be done by the staking program)
        // staking_pool.update_validator_stake(&validator.address, stake_amount)?;
    }
    
    msg!("SOL staking completed: {} USD distributed across {} validators", 
         amount_usd, selected_validators.len());
    
    Ok(())
}

/// Initiate ETH L2 staking on Arbitrum/Optimism
fn initiate_eth_l2_staking(staking_pool: &mut StakingPool, amount_usd: u64) -> Result<()> {
    // Select best ETH validators (liquid staking providers)
    let selected_validators = staking_pool.select_best_eth_validators(2);
    
    if selected_validators.is_empty() {
        return Err(VaultError::NoValidatorsAvailable.into());
    }
    
    // Split between Arbitrum and Optimism (50/50 for diversification)
    let arbitrum_amount = amount_usd / 2;
    let optimism_amount = amount_usd - arbitrum_amount;
    
    // Prepare cross-chain messages for ETH L2 staking
    let arbitrum_message = CrossChainMessage {
        target_chain: "arbitrum".to_string(),
        contract_address: "0x...".to_string(), // Lido/RocketPool on Arbitrum
        function_call: "stake".to_string(),
        amount: arbitrum_amount,
        validator: selected_validators[0].address.clone(),
    };
    
    let optimism_message = if selected_validators.len() > 1 {
        CrossChainMessage {
            target_chain: "optimism".to_string(),
            contract_address: "0x...".to_string(), // Lido/RocketPool on Optimism
            function_call: "stake".to_string(),
            amount: optimism_amount,
            validator: selected_validators[1].address.clone(),
        }
    } else {
        CrossChainMessage {
            target_chain: "optimism".to_string(),
            contract_address: "0x...".to_string(),
            function_call: "stake".to_string(),
            amount: optimism_amount,
            validator: selected_validators[0].address.clone(),
        }
    };
    
    // Queue cross-chain messages (in production, would use Wormhole or similar)
    queue_cross_chain_message(arbitrum_message)?;
    queue_cross_chain_message(optimism_message)?;
    
    msg!("ETH L2 staking initiated: {} USD to Arbitrum, {} USD to Optimism", 
         arbitrum_amount, optimism_amount);
    
    Ok(())
}

/// Initiate ATOM staking with Everstake and Osmosis
fn initiate_atom_staking(staking_pool: &mut StakingPool, amount_usd: u64) -> Result<()> {
    let config = &staking_pool.atom_config;
    
    // Calculate amounts for Everstake (20% of total) and Osmosis (10% of total)
    let everstake_amount = (amount_usd * config.everstake_allocation as u64) / StakingPool::ATOM_ALLOCATION_BPS as u64;
    let osmosis_amount = (amount_usd * config.osmosis_allocation as u64) / StakingPool::ATOM_ALLOCATION_BPS as u64;
    
    // Prepare cross-chain messages for ATOM staking
    let everstake_message = CrossChainMessage {
        target_chain: "cosmos".to_string(),
        contract_address: config.everstake_validator.clone(),
        function_call: "delegate".to_string(),
        amount: everstake_amount,
        validator: config.everstake_validator.clone(),
    };
    
    let osmosis_message = CrossChainMessage {
        target_chain: "osmosis".to_string(),
        contract_address: config.osmosis_validator.clone(),
        function_call: "delegate".to_string(),
        amount: osmosis_amount,
        validator: config.osmosis_validator.clone(),
    };
    
    // Queue cross-chain messages for ATOM staking
    queue_cross_chain_message(everstake_message)?;
    queue_cross_chain_message(osmosis_message)?;
    
    msg!("ATOM staking initiated: {} USD to Everstake, {} USD to Osmosis", 
         everstake_amount, osmosis_amount);
    
    Ok(())
}

/// Cross-chain message structure for L2 and Cosmos communication
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct CrossChainMessage {
    pub target_chain: String,
    pub contract_address: String,
    pub function_call: String,
    pub amount: u64,
    pub validator: String,
}

/// Queue cross-chain message for processing
fn queue_cross_chain_message(message: CrossChainMessage) -> Result<()> {
    // In production, this would:
    // 1. Serialize the message
    // 2. Submit to Wormhole or similar cross-chain protocol
    // 3. Handle confirmation and retry logic
    
    msg!("Queued cross-chain message: {} on {} for {} USD", 
         message.function_call, message.target_chain, message.amount);
    
    // For now, we simulate successful queuing
    Ok(())
}