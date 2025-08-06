use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::state::*;
use crate::errors::VaultError;

#[derive(Accounts)]
pub struct InitializeTreasury<'info> {
    #[account(
        init,
        payer = authority,
        space = Treasury::LEN,
        seeds = [b"treasury"],
        bump
    )]
    pub treasury: Account<'info, Treasury>,
    
    #[account(
        seeds = [b"multisig_wallet"],
        bump = multisig_wallet.bump
    )]
    pub multisig_wallet: Account<'info, MultisigWallet>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ProcessDeposit<'info> {
    #[account(
        mut,
        seeds = [b"treasury"],
        bump = treasury.bump
    )]
    pub treasury: Account<'info, Treasury>,
    
    #[account(
        seeds = [b"multisig_wallet"],
        bump = multisig_wallet.bump
    )]
    pub multisig_wallet: Account<'info, MultisigWallet>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    /// Oracle account for price feeds
    #[account(
        seeds = [b"oracle"],
        bump = oracle.bump
    )]
    pub oracle: Account<'info, Oracle>,
    
    /// System program for SOL transfers
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RebalanceTreasury<'info> {
    #[account(
        mut,
        seeds = [b"treasury"],
        bump = treasury.bump
    )]
    pub treasury: Account<'info, Treasury>,
    
    #[account(
        seeds = [b"multisig_wallet"],
        bump = multisig_wallet.bump
    )]
    pub multisig_wallet: Account<'info, MultisigWallet>,
    
    #[account(
        mut,
        seeds = [b"staking_pool"],
        bump = staking_pool.bump
    )]
    pub staking_pool: Account<'info, StakingPool>,
    
    #[account(
        seeds = [b"oracle"],
        bump = oracle.bump
    )]
    pub oracle: Account<'info, Oracle>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WithdrawFromTreasury<'info> {
    #[account(
        mut,
        seeds = [b"treasury"],
        bump = treasury.bump
    )]
    pub treasury: Account<'info, Treasury>,
    
    #[account(
        seeds = [b"multisig_wallet"],
        bump = multisig_wallet.bump
    )]
    pub multisig_wallet: Account<'info, MultisigWallet>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    /// CHECK: Recipient account for withdrawal
    #[account(mut)]
    pub recipient: AccountInfo<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateTreasuryConfig<'info> {
    #[account(
        mut,
        seeds = [b"treasury"],
        bump = treasury.bump
    )]
    pub treasury: Account<'info, Treasury>,
    
    #[account(
        seeds = [b"multisig_wallet"],
        bump = multisig_wallet.bump
    )]
    pub multisig_wallet: Account<'info, MultisigWallet>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct GetTreasuryStats<'info> {
    #[account(
        seeds = [b"treasury"],
        bump = treasury.bump
    )]
    pub treasury: Account<'info, Treasury>,
}

/// Initialize the treasury account with default configuration
pub fn initialize_treasury(ctx: Context<InitializeTreasury>) -> Result<()> {
    let treasury = &mut ctx.accounts.treasury;
    let multisig_wallet = &ctx.accounts.multisig_wallet;
    let authority = ctx.accounts.authority.key();
    
    // Verify authority is a multisig signer
    require!(
        is_multisig_signer(&multisig_wallet, &authority),
        VaultError::UnauthorizedAccess
    );
    
    let clock = Clock::get()?;
    
    // Initialize treasury with default values
    treasury.total_assets = 0;
    treasury.sol_balance = 0;
    treasury.eth_balance = 0;
    treasury.atom_balance = 0;
    treasury.staking_rewards = 0;
    treasury.user_rewards_pool = 0;
    treasury.last_deposit = 0;
    treasury.next_deposit = clock.unix_timestamp + Treasury::DEPOSIT_FREQUENCY_SECONDS as i64;
    treasury.deposit_amount = 50_000_000; // $50 USD in micro-dollars
    treasury.deposit_frequency = Treasury::DEPOSIT_FREQUENCY_SECONDS;
    treasury.bump = ctx.bumps.treasury;
    
    msg!("Treasury initialized with $50 biweekly deposits");
    
    Ok(())
}

/// Process a biweekly $50 deposit and convert to protocol assets
pub fn process_deposit(
    ctx: Context<ProcessDeposit>,
    deposit_amount_usd: u64, // Amount in USD (micro-dollars)
) -> Result<()> {
    let treasury = &mut ctx.accounts.treasury;
    let multisig_wallet = &ctx.accounts.multisig_wallet;
    let oracle = &ctx.accounts.oracle;
    let authority = ctx.accounts.authority.key();
    
    // Verify authority is a multisig signer
    require!(
        is_multisig_signer(&multisig_wallet, &authority),
        VaultError::UnauthorizedAccess
    );
    
    let clock = Clock::get()?;
    
    // Check if deposit is due
    require!(
        clock.unix_timestamp >= treasury.next_deposit,
        VaultError::DepositNotDue
    );
    
    // Validate deposit amount (should be $50 USD equivalent)
    require!(
        deposit_amount_usd == treasury.deposit_amount,
        VaultError::InvalidDepositAmount
    );
    
    // Calculate asset allocations based on current prices
    let allocations = calculate_asset_allocations(deposit_amount_usd, oracle)?;
    
    // Update treasury balances
    treasury.sol_balance = treasury.sol_balance
        .checked_add(allocations.sol_amount)
        .ok_or(VaultError::MathOverflow)?;
    
    treasury.eth_balance = treasury.eth_balance
        .checked_add(allocations.eth_amount)
        .ok_or(VaultError::MathOverflow)?;
    
    treasury.atom_balance = treasury.atom_balance
        .checked_add(allocations.atom_amount)
        .ok_or(VaultError::MathOverflow)?;
    
    treasury.total_assets = treasury.total_assets
        .checked_add(deposit_amount_usd)
        .ok_or(VaultError::MathOverflow)?;
    
    // Update deposit tracking
    treasury.last_deposit = clock.unix_timestamp;
    treasury.next_deposit = clock.unix_timestamp + treasury.deposit_frequency as i64;
    
    msg!(
        "Processed ${} deposit: {} SOL, {} ETH, {} ATOM",
        deposit_amount_usd as f64 / 1_000_000.0,
        allocations.sol_amount as f64 / 1_000_000_000.0, // Convert from lamports
        allocations.eth_amount as f64 / 1_000_000_000_000_000_000.0, // Convert from wei
        allocations.atom_amount as f64 / 1_000_000.0 // Convert from micro-ATOM
    );
    
    Ok(())
}

/// Rebalance treasury assets to maintain target allocations (SOL 40%, ETH 30%, ATOM 30%)
pub fn rebalance_treasury(ctx: Context<RebalanceTreasury>) -> Result<()> {
    let treasury = &mut ctx.accounts.treasury;
    let multisig_wallet = &ctx.accounts.multisig_wallet;
    let staking_pool = &mut ctx.accounts.staking_pool;
    let oracle = &ctx.accounts.oracle;
    let authority = ctx.accounts.authority.key();
    
    // Verify authority is a multisig signer
    require!(
        is_multisig_signer(&multisig_wallet, &authority),
        VaultError::UnauthorizedAccess
    );
    
    // Calculate current asset values in USD
    let current_values = calculate_current_asset_values(treasury, oracle)?;
    let total_value = current_values.sol_value_usd + current_values.eth_value_usd + current_values.atom_value_usd;
    
    // Calculate target allocations
    let target_sol_value = (total_value * 40) / 100; // 40%
    let target_eth_value = (total_value * 30) / 100; // 30%
    let target_atom_value = (total_value * 30) / 100; // 30%
    
    // Calculate rebalancing needs
    let sol_diff = target_sol_value as i64 - current_values.sol_value_usd as i64;
    let eth_diff = target_eth_value as i64 - current_values.eth_value_usd as i64;
    let atom_diff = target_atom_value as i64 - current_values.atom_value_usd as i64;
    
    // Only rebalance if deviation is > 5%
    let rebalance_threshold = (total_value * 5) / 100;
    
    if sol_diff.abs() as u64 > rebalance_threshold ||
       eth_diff.abs() as u64 > rebalance_threshold ||
       atom_diff.abs() as u64 > rebalance_threshold {
        
        // Perform rebalancing (simplified - in production would involve actual swaps)
        let rebalancing_actions = calculate_rebalancing_actions(
            &current_values,
            target_sol_value,
            target_eth_value,
            target_atom_value,
            oracle,
        )?;
        
        // Update treasury balances after rebalancing
        treasury.sol_balance = rebalancing_actions.new_sol_balance;
        treasury.eth_balance = rebalancing_actions.new_eth_balance;
        treasury.atom_balance = rebalancing_actions.new_atom_balance;
        
        // Update staking pool allocations
        staking_pool.sol_allocation = rebalancing_actions.new_sol_balance;
        staking_pool.eth_allocation = rebalancing_actions.new_eth_balance;
        staking_pool.atom_allocation = rebalancing_actions.new_atom_balance;
        
        msg!(
            "Treasury rebalanced: SOL {} -> {}, ETH {} -> {}, ATOM {} -> {}",
            current_values.sol_value_usd,
            target_sol_value,
            current_values.eth_value_usd,
            target_eth_value,
            current_values.atom_value_usd,
            target_atom_value
        );
    } else {
        msg!("Treasury within rebalancing threshold, no action needed");
    }
    
    Ok(())
}

/// Withdraw funds from treasury (emergency or operational use)
pub fn withdraw_from_treasury(
    ctx: Context<WithdrawFromTreasury>,
    asset_type: TreasuryAssetType,
    amount: u64,
    reason: String,
) -> Result<()> {
    let treasury = &mut ctx.accounts.treasury;
    let multisig_wallet = &ctx.accounts.multisig_wallet;
    let authority = ctx.accounts.authority.key();
    
    // Verify authority is a multisig signer
    require!(
        is_multisig_signer(&multisig_wallet, &authority),
        VaultError::UnauthorizedAccess
    );
    
    // Validate withdrawal reason
    require!(
        !reason.is_empty() && reason.len() <= 256,
        VaultError::InvalidWithdrawalReason
    );
    
    // Check sufficient balance and update
    match asset_type {
        TreasuryAssetType::SOL => {
            require!(
                treasury.sol_balance >= amount,
                VaultError::InsufficientBalance
            );
            treasury.sol_balance = treasury.sol_balance
                .checked_sub(amount)
                .ok_or(VaultError::MathOverflow)?;
        },
        TreasuryAssetType::ETH => {
            require!(
                treasury.eth_balance >= amount,
                VaultError::InsufficientBalance
            );
            treasury.eth_balance = treasury.eth_balance
                .checked_sub(amount)
                .ok_or(VaultError::MathOverflow)?;
        },
        TreasuryAssetType::ATOM => {
            require!(
                treasury.atom_balance >= amount,
                VaultError::InsufficientBalance
            );
            treasury.atom_balance = treasury.atom_balance
                .checked_sub(amount)
                .ok_or(VaultError::MathOverflow)?;
        },
    }
    
    // Transfer funds to recipient (simplified - in production would handle actual transfers)
    msg!(
        "Treasury withdrawal: {} {:?} to {} - Reason: {}",
        amount,
        asset_type,
        ctx.accounts.recipient.key(),
        reason
    );
    
    Ok(())
}

/// Update treasury configuration parameters
pub fn update_treasury_config(
    ctx: Context<UpdateTreasuryConfig>,
    new_deposit_amount: Option<u64>,
    new_deposit_frequency: Option<u32>,
) -> Result<()> {
    let treasury = &mut ctx.accounts.treasury;
    let multisig_wallet = &ctx.accounts.multisig_wallet;
    let authority = ctx.accounts.authority.key();
    
    // Verify authority is a multisig signer
    require!(
        is_multisig_signer(&multisig_wallet, &authority),
        VaultError::UnauthorizedAccess
    );
    
    // Update deposit amount if provided
    if let Some(amount) = new_deposit_amount {
        require!(
            amount > 0 && amount <= 1_000_000_000, // Max $1000 USD
            VaultError::InvalidDepositAmount
        );
        treasury.deposit_amount = amount;
        msg!("Treasury deposit amount updated to ${}", amount as f64 / 1_000_000.0);
    }
    
    // Update deposit frequency if provided
    if let Some(frequency) = new_deposit_frequency {
        require!(
            frequency >= 86400 && frequency <= 2_592_000, // 1 day to 30 days
            VaultError::InvalidDepositFrequency
        );
        treasury.deposit_frequency = frequency;
        
        // Update next deposit time
        let clock = Clock::get()?;
        treasury.next_deposit = clock.unix_timestamp + frequency as i64;
        
        msg!("Treasury deposit frequency updated to {} seconds", frequency);
    }
    
    Ok(())
}

/// Get treasury statistics for public display
pub fn get_treasury_stats(ctx: Context<GetTreasuryStats>) -> Result<TreasuryStats> {
    let treasury = &ctx.accounts.treasury;
    
    // Return public treasury information (hide sensitive allocation details)
    let stats = TreasuryStats {
        total_assets_usd: treasury.total_assets,
        total_staking_rewards: treasury.staking_rewards,
        user_rewards_pool: treasury.user_rewards_pool,
        last_deposit_timestamp: treasury.last_deposit,
        next_deposit_timestamp: treasury.next_deposit,
        // Hide exact asset balances for security
        asset_count: 3, // SOL, ETH, ATOM
    };
    
    msg!("Treasury stats: ${} total assets, ${} user rewards pool",
         stats.total_assets_usd as f64 / 1_000_000.0,
         stats.user_rewards_pool as f64 / 1_000_000.0);
    
    Ok(stats)
}

// Helper functions

fn is_multisig_signer(multisig_wallet: &MultisigWallet, signer: &Pubkey) -> bool {
    multisig_wallet.signers.iter().any(|s| s.pubkey == *signer && s.is_active)
}

fn calculate_asset_allocations(
    deposit_amount_usd: u64,
    oracle: &Oracle,
) -> Result<AssetAllocations> {
    // Calculate USD amounts for each asset (40% SOL, 30% ETH, 30% ATOM)
    let sol_usd_amount = (deposit_amount_usd * 40) / 100;
    let eth_usd_amount = (deposit_amount_usd * 30) / 100;
    let atom_usd_amount = (deposit_amount_usd * 30) / 100;
    
    // Convert USD amounts to native token amounts using oracle prices
    let sol_amount = if oracle.btc_usd_price > 0 {
        // Simplified: assume SOL price is BTC_price / 1000 (placeholder)
        let sol_price_usd = oracle.btc_usd_price / 1000;
        (sol_usd_amount * 1_000_000_000) / sol_price_usd // Convert to lamports
    } else {
        return Err(VaultError::InvalidOraclePrice.into());
    };
    
    // Simplified ETH and ATOM calculations (in production would use actual price feeds)
    let eth_amount = (eth_usd_amount * 1_000_000_000_000_000_000) / 2000_000_000; // Assume $2000 ETH
    let atom_amount = (atom_usd_amount * 1_000_000) / 10_000_000; // Assume $10 ATOM
    
    Ok(AssetAllocations {
        sol_amount,
        eth_amount,
        atom_amount,
    })
}

fn calculate_current_asset_values(
    treasury: &Treasury,
    oracle: &Oracle,
) -> Result<CurrentAssetValues> {
    // Calculate current USD values of treasury assets
    let sol_price_usd = oracle.btc_usd_price / 1000; // Simplified
    let eth_price_usd = 2000_000_000; // $2000 in micro-dollars
    let atom_price_usd = 10_000_000; // $10 in micro-dollars
    
    let sol_value_usd = (treasury.sol_balance * sol_price_usd) / 1_000_000_000; // From lamports
    let eth_value_usd = (treasury.eth_balance * eth_price_usd) / 1_000_000_000_000_000_000; // From wei
    let atom_value_usd = (treasury.atom_balance * atom_price_usd) / 1_000_000; // From micro-ATOM
    
    Ok(CurrentAssetValues {
        sol_value_usd,
        eth_value_usd,
        atom_value_usd,
    })
}

fn calculate_rebalancing_actions(
    current_values: &CurrentAssetValues,
    target_sol_value: u64,
    target_eth_value: u64,
    target_atom_value: u64,
    oracle: &Oracle,
) -> Result<RebalancingActions> {
    // Calculate new balances after rebalancing (simplified)
    let sol_price_usd = oracle.btc_usd_price / 1000;
    let eth_price_usd = 2000_000_000;
    let atom_price_usd = 10_000_000;
    
    let new_sol_balance = (target_sol_value * 1_000_000_000) / sol_price_usd;
    let new_eth_balance = (target_eth_value * 1_000_000_000_000_000_000) / eth_price_usd;
    let new_atom_balance = (target_atom_value * 1_000_000) / atom_price_usd;
    
    Ok(RebalancingActions {
        new_sol_balance,
        new_eth_balance,
        new_atom_balance,
    })
}

// Data structures

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum TreasuryAssetType {
    SOL,
    ETH,
    ATOM,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct TreasuryStats {
    pub total_assets_usd: u64,
    pub total_staking_rewards: u64,
    pub user_rewards_pool: u64,
    pub last_deposit_timestamp: i64,
    pub next_deposit_timestamp: i64,
    pub asset_count: u8,
}

#[derive(Debug)]
struct AssetAllocations {
    sol_amount: u64,
    eth_amount: u64,
    atom_amount: u64,
}

#[derive(Debug)]
struct CurrentAssetValues {
    sol_value_usd: u64,
    eth_value_usd: u64,
    atom_value_usd: u64,
}

#[derive(Debug)]
struct RebalancingActions {
    new_sol_balance: u64,
    new_eth_balance: u64,
    new_atom_balance: u64,
}