use anchor_lang::prelude::*;
use crate::errors::VaultError;

#[account]
pub struct Treasury {
    pub total_assets: u64,           // Total treasury value in USD (micro-dollars)
    pub sol_balance: u64,            // SOL balance in lamports
    pub eth_balance: u64,            // ETH balance in wei
    pub atom_balance: u64,           // ATOM balance in micro-ATOM
    pub staking_rewards: u64,        // Total staking rewards earned
    pub user_rewards_pool: u64,      // Pool of rewards available for users
    pub last_deposit: i64,           // Timestamp of last deposit
    pub next_deposit: i64,           // Timestamp of next scheduled deposit
    pub deposit_amount: u64,         // Deposit amount in USD (micro-dollars)
    pub deposit_frequency: u32,      // Deposit frequency in seconds
    pub total_deposits: u64,         // Total number of deposits made
    pub emergency_pause: bool,       // Emergency pause flag
    pub rebalance_threshold: u16,    // Rebalancing threshold in basis points (default 500 = 5%)
    pub min_deposit_amount: u64,     // Minimum deposit amount
    pub max_deposit_amount: u64,     // Maximum deposit amount
    pub created_at: i64,             // Treasury creation timestamp
    pub updated_at: i64,             // Last update timestamp
    pub bump: u8,                    // PDA bump
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
        8 + // total_deposits
        1 + // emergency_pause
        2 + // rebalance_threshold
        8 + // min_deposit_amount
        8 + // max_deposit_amount
        8 + // created_at
        8 + // updated_at
        1; // bump

    pub const DEPOSIT_FREQUENCY_SECONDS: u32 = 14 * 24 * 60 * 60; // 14 days
    pub const DEFAULT_DEPOSIT_AMOUNT: u64 = 50_000_000; // $50 USD in micro-dollars
    pub const DEFAULT_REBALANCE_THRESHOLD: u16 = 500; // 5% in basis points
    pub const MIN_DEPOSIT_AMOUNT: u64 = 10_000_000; // $10 USD minimum
    pub const MAX_DEPOSIT_AMOUNT: u64 = 1_000_000_000; // $1000 USD maximum

    /// Initialize treasury with default values
    pub fn initialize(&mut self, bump: u8) -> Result<()> {
        let clock = Clock::get()?;
        
        self.total_assets = 0;
        self.sol_balance = 0;
        self.eth_balance = 0;
        self.atom_balance = 0;
        self.staking_rewards = 0;
        self.user_rewards_pool = 0;
        self.last_deposit = 0;
        self.next_deposit = clock.unix_timestamp + Self::DEPOSIT_FREQUENCY_SECONDS as i64;
        self.deposit_amount = Self::DEFAULT_DEPOSIT_AMOUNT;
        self.deposit_frequency = Self::DEPOSIT_FREQUENCY_SECONDS;
        self.total_deposits = 0;
        self.emergency_pause = false;
        self.rebalance_threshold = Self::DEFAULT_REBALANCE_THRESHOLD;
        self.min_deposit_amount = Self::MIN_DEPOSIT_AMOUNT;
        self.max_deposit_amount = Self::MAX_DEPOSIT_AMOUNT;
        self.created_at = clock.unix_timestamp;
        self.updated_at = clock.unix_timestamp;
        self.bump = bump;
        
        Ok(())
    }

    /// Check if a deposit is due
    pub fn is_deposit_due(&self) -> Result<bool> {
        let clock = Clock::get()?;
        Ok(clock.unix_timestamp >= self.next_deposit && !self.emergency_pause)
    }

    /// Process a deposit and update balances
    pub fn process_deposit(
        &mut self,
        sol_amount: u64,
        eth_amount: u64,
        atom_amount: u64,
        deposit_value_usd: u64,
    ) -> Result<()> {
        if self.emergency_pause {
            return Err(VaultError::TreasuryPaused.into());
        }

        let clock = Clock::get()?;
        
        // Update asset balances
        self.sol_balance = self.sol_balance
            .checked_add(sol_amount)
            .ok_or(VaultError::MathOverflow)?;
        
        self.eth_balance = self.eth_balance
            .checked_add(eth_amount)
            .ok_or(VaultError::MathOverflow)?;
        
        self.atom_balance = self.atom_balance
            .checked_add(atom_amount)
            .ok_or(VaultError::MathOverflow)?;
        
        self.total_assets = self.total_assets
            .checked_add(deposit_value_usd)
            .ok_or(VaultError::MathOverflow)?;
        
        // Update deposit tracking
        self.last_deposit = clock.unix_timestamp;
        self.next_deposit = clock.unix_timestamp + self.deposit_frequency as i64;
        self.total_deposits = self.total_deposits
            .checked_add(1)
            .ok_or(VaultError::MathOverflow)?;
        
        self.updated_at = clock.unix_timestamp;
        
        Ok(())
    }

    /// Update asset balances after rebalancing
    pub fn update_balances(
        &mut self,
        new_sol_balance: u64,
        new_eth_balance: u64,
        new_atom_balance: u64,
    ) -> Result<()> {
        self.sol_balance = new_sol_balance;
        self.eth_balance = new_eth_balance;
        self.atom_balance = new_atom_balance;
        self.updated_at = Clock::get()?.unix_timestamp;
        
        Ok(())
    }

    /// Add staking rewards to the treasury
    pub fn add_staking_rewards(&mut self, rewards_amount: u64) -> Result<()> {
        self.staking_rewards = self.staking_rewards
            .checked_add(rewards_amount)
            .ok_or(VaultError::MathOverflow)?;
        
        // 50% of rewards go to user pool
        let user_share = rewards_amount / 2;
        self.user_rewards_pool = self.user_rewards_pool
            .checked_add(user_share)
            .ok_or(VaultError::MathOverflow)?;
        
        self.updated_at = Clock::get()?.unix_timestamp;
        
        Ok(())
    }

    /// Withdraw from user rewards pool
    pub fn withdraw_user_rewards(&mut self, amount: u64) -> Result<()> {
        if self.user_rewards_pool < amount {
            return Err(VaultError::InsufficientBalance.into());
        }
        
        self.user_rewards_pool = self.user_rewards_pool
            .checked_sub(amount)
            .ok_or(VaultError::MathOverflow)?;
        
        self.updated_at = Clock::get()?.unix_timestamp;
        
        Ok(())
    }

    /// Set emergency pause status
    pub fn set_emergency_pause(&mut self, paused: bool) -> Result<()> {
        self.emergency_pause = paused;
        self.updated_at = Clock::get()?.unix_timestamp;
        
        Ok(())
    }

    /// Update treasury configuration
    pub fn update_config(
        &mut self,
        deposit_amount: Option<u64>,
        deposit_frequency: Option<u32>,
        rebalance_threshold: Option<u16>,
    ) -> Result<()> {
        if let Some(amount) = deposit_amount {
            if amount < self.min_deposit_amount || amount > self.max_deposit_amount {
                return Err(VaultError::InvalidDepositAmount.into());
            }
            self.deposit_amount = amount;
        }
        
        if let Some(frequency) = deposit_frequency {
            if frequency < 86400 || frequency > 2_592_000 { // 1 day to 30 days
                return Err(VaultError::InvalidDepositFrequency.into());
            }
            self.deposit_frequency = frequency;
            
            // Update next deposit time
            let clock = Clock::get()?;
            self.next_deposit = clock.unix_timestamp + frequency as i64;
        }
        
        if let Some(threshold) = rebalance_threshold {
            if threshold > 2000 { // Max 20%
                return Err(VaultError::InvalidRebalanceThreshold.into());
            }
            self.rebalance_threshold = threshold;
        }
        
        self.updated_at = Clock::get()?.unix_timestamp;
        
        Ok(())
    }

    /// Get current asset allocation percentages
    pub fn get_allocation_percentages(&self, oracle_prices: &OraclePrices) -> Result<AllocationPercentages> {
        if self.total_assets == 0 {
            return Ok(AllocationPercentages {
                sol_percentage: 0,
                eth_percentage: 0,
                atom_percentage: 0,
            });
        }
        
        // Calculate current USD values
        let sol_value = (self.sol_balance * oracle_prices.sol_usd_price) / 1_000_000_000; // From lamports
        let eth_value = (self.eth_balance * oracle_prices.eth_usd_price) / 1_000_000_000_000_000_000; // From wei
        let atom_value = (self.atom_balance * oracle_prices.atom_usd_price) / 1_000_000; // From micro-ATOM
        
        let total_value = sol_value + eth_value + atom_value;
        
        if total_value == 0 {
            return Ok(AllocationPercentages {
                sol_percentage: 0,
                eth_percentage: 0,
                atom_percentage: 0,
            });
        }
        
        Ok(AllocationPercentages {
            sol_percentage: ((sol_value * 10000) / total_value) as u16, // Basis points
            eth_percentage: ((eth_value * 10000) / total_value) as u16,
            atom_percentage: ((atom_value * 10000) / total_value) as u16,
        })
    }

    /// Check if rebalancing is needed
    pub fn needs_rebalancing(&self, oracle_prices: &OraclePrices) -> Result<bool> {
        let current_allocations = self.get_allocation_percentages(oracle_prices)?;
        
        // Target allocations in basis points
        const TARGET_SOL: u16 = 4000; // 40%
        const TARGET_ETH: u16 = 3000; // 30%
        const TARGET_ATOM: u16 = 3000; // 30%
        
        let sol_deviation = if current_allocations.sol_percentage > TARGET_SOL {
            current_allocations.sol_percentage - TARGET_SOL
        } else {
            TARGET_SOL - current_allocations.sol_percentage
        };
        
        let eth_deviation = if current_allocations.eth_percentage > TARGET_ETH {
            current_allocations.eth_percentage - TARGET_ETH
        } else {
            TARGET_ETH - current_allocations.eth_percentage
        };
        
        let atom_deviation = if current_allocations.atom_percentage > TARGET_ATOM {
            current_allocations.atom_percentage - TARGET_ATOM
        } else {
            TARGET_ATOM - current_allocations.atom_percentage
        };
        
        Ok(sol_deviation > self.rebalance_threshold ||
           eth_deviation > self.rebalance_threshold ||
           atom_deviation > self.rebalance_threshold)
    }

    /// Get treasury statistics for public display
    pub fn get_public_stats(&self) -> TreasuryPublicStats {
        TreasuryPublicStats {
            total_assets_usd: self.total_assets,
            total_staking_rewards: self.staking_rewards,
            user_rewards_pool: self.user_rewards_pool,
            total_deposits: self.total_deposits,
            last_deposit: self.last_deposit,
            next_deposit: self.next_deposit,
            emergency_pause: self.emergency_pause,
        }
    }
}

/// Oracle prices for asset valuation
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct OraclePrices {
    pub sol_usd_price: u64,  // SOL price in micro-dollars
    pub eth_usd_price: u64,  // ETH price in micro-dollars
    pub atom_usd_price: u64, // ATOM price in micro-dollars
}

/// Current allocation percentages in basis points
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct AllocationPercentages {
    pub sol_percentage: u16,  // SOL allocation in basis points
    pub eth_percentage: u16,  // ETH allocation in basis points
    pub atom_percentage: u16, // ATOM allocation in basis points
}

/// Public treasury statistics (hides sensitive allocation details)
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct TreasuryPublicStats {
    pub total_assets_usd: u64,
    pub total_staking_rewards: u64,
    pub user_rewards_pool: u64,
    pub total_deposits: u64,
    pub last_deposit: i64,
    pub next_deposit: i64,
    pub emergency_pause: bool,
}