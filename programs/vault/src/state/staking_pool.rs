use anchor_lang::prelude::*;
use crate::errors::VaultError;

/// Validator information for staking operations
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct ValidatorInfo {
    pub address: String,
    pub commission: u16,  // Basis points (e.g., 500 = 5%)
    pub stake_amount: u64,
    pub performance_score: u16,  // 0-10000 (100.00%)
    pub is_active: bool,
}

/// ATOM staking distribution configuration
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct AtomStakingConfig {
    pub everstake_allocation: u32,  // 2000 basis points (20%)
    pub osmosis_allocation: u32,    // 1000 basis points (10%)
    pub everstake_validator: String,
    pub osmosis_validator: String,
}

/// Asset allocation tracking
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct AssetAllocation {
    pub target_percentage: u32,  // Basis points
    pub current_amount: u64,
    pub target_amount: u64,
    pub last_rebalance: i64,
    pub deviation_threshold: u32,  // Basis points for rebalancing trigger
}

#[account]
pub struct StakingPool {
    pub total_staked: u64,
    pub total_treasury_value: u64,  // Total value of treasury assets in USD
    
    // Asset allocations (basis points)
    pub sol_allocation: AssetAllocation,
    pub eth_allocation: AssetAllocation,
    pub atom_allocation: AssetAllocation,
    
    // Current staked amounts
    pub sol_staked: u64,
    pub eth_staked: u64,
    pub atom_staked: u64,
    
    // Validator information
    pub sol_validators: Vec<ValidatorInfo>,
    pub eth_validators: Vec<ValidatorInfo>,
    pub atom_config: AtomStakingConfig,
    
    // Reward tracking
    pub rewards_accumulated: u64,
    pub rewards_distributed: u64,
    pub last_reward_calculation: i64,
    
    // Rebalancing
    pub last_rebalance: i64,
    pub rebalance_threshold: u32,  // Basis points
    pub auto_rebalance_enabled: bool,
    
    // Metadata
    pub last_update: i64,
    pub bump: u8,
}

impl StakingPool {
    pub const LEN: usize = 8 + // discriminator
        8 + // total_staked
        8 + // total_treasury_value
        (4 + 8 + 8 + 8 + 4) * 3 + // asset allocations (3 assets)
        8 * 3 + // staked amounts
        4 + (32 + 2 + 8 + 2 + 1) * 10 + // sol_validators (max 10)
        4 + (32 + 2 + 8 + 2 + 1) * 10 + // eth_validators (max 10)
        (4 + 4 + 32 + 32) + // atom_config
        8 * 3 + // reward tracking
        8 + 4 + 1 + // rebalancing
        8 + 1; // metadata

    // Allocation constants (basis points)
    pub const SOL_ALLOCATION_BPS: u32 = 4000; // 40%
    pub const ETH_ALLOCATION_BPS: u32 = 3000; // 30%
    pub const ATOM_ALLOCATION_BPS: u32 = 3000; // 30%
    pub const TOTAL_BPS: u32 = 10000; // 100%
    
    // ATOM sub-allocations
    pub const ATOM_EVERSTAKE_BPS: u32 = 2000; // 20% of total (66.67% of ATOM)
    pub const ATOM_OSMOSIS_BPS: u32 = 1000;   // 10% of total (33.33% of ATOM)
    
    // Rebalancing thresholds
    pub const DEFAULT_REBALANCE_THRESHOLD: u32 = 500; // 5%
    pub const MAX_DEVIATION_THRESHOLD: u32 = 200; // 2%

    /// Initialize the staking pool with default allocations
    pub fn initialize(&mut self, bump: u8) -> Result<()> {
        self.sol_allocation = AssetAllocation {
            target_percentage: Self::SOL_ALLOCATION_BPS,
            current_amount: 0,
            target_amount: 0,
            last_rebalance: 0,
            deviation_threshold: Self::MAX_DEVIATION_THRESHOLD,
        };
        
        self.eth_allocation = AssetAllocation {
            target_percentage: Self::ETH_ALLOCATION_BPS,
            current_amount: 0,
            target_amount: 0,
            last_rebalance: 0,
            deviation_threshold: Self::MAX_DEVIATION_THRESHOLD,
        };
        
        self.atom_allocation = AssetAllocation {
            target_percentage: Self::ATOM_ALLOCATION_BPS,
            current_amount: 0,
            target_amount: 0,
            last_rebalance: 0,
            deviation_threshold: Self::MAX_DEVIATION_THRESHOLD,
        };
        
        self.atom_config = AtomStakingConfig {
            everstake_allocation: Self::ATOM_EVERSTAKE_BPS,
            osmosis_allocation: Self::ATOM_OSMOSIS_BPS,
            everstake_validator: "cosmosvaloper1...".to_string(), // Placeholder
            osmosis_validator: "osmovaloper1...".to_string(),     // Placeholder
        };
        
        self.rebalance_threshold = Self::DEFAULT_REBALANCE_THRESHOLD;
        self.auto_rebalance_enabled = true;
        self.bump = bump;
        
        let clock = Clock::get()?;
        self.last_update = clock.unix_timestamp;
        
        Ok(())
    }

    /// Calculate target allocations based on total treasury value
    pub fn calculate_target_allocations(&mut self, total_treasury_usd: u64) -> Result<()> {
        self.total_treasury_value = total_treasury_usd;
        
        // Calculate target amounts for each asset
        self.sol_allocation.target_amount = (total_treasury_usd * self.sol_allocation.target_percentage as u64) / Self::TOTAL_BPS as u64;
        self.eth_allocation.target_amount = (total_treasury_usd * self.eth_allocation.target_percentage as u64) / Self::TOTAL_BPS as u64;
        self.atom_allocation.target_amount = (total_treasury_usd * self.atom_allocation.target_percentage as u64) / Self::TOTAL_BPS as u64;
        
        Ok(())
    }

    /// Check if rebalancing is needed based on deviation thresholds
    pub fn needs_rebalancing(&self) -> Result<bool> {
        if self.total_treasury_value == 0 {
            return Ok(false);
        }

        // Check SOL deviation
        let sol_deviation = self.calculate_allocation_deviation(
            self.sol_allocation.current_amount,
            self.sol_allocation.target_amount,
        )?;
        
        if sol_deviation > self.sol_allocation.deviation_threshold {
            return Ok(true);
        }

        // Check ETH deviation
        let eth_deviation = self.calculate_allocation_deviation(
            self.eth_allocation.current_amount,
            self.eth_allocation.target_amount,
        )?;
        
        if eth_deviation > self.eth_allocation.deviation_threshold {
            return Ok(true);
        }

        // Check ATOM deviation
        let atom_deviation = self.calculate_allocation_deviation(
            self.atom_allocation.current_amount,
            self.atom_allocation.target_amount,
        )?;
        
        if atom_deviation > self.atom_allocation.deviation_threshold {
            return Ok(true);
        }

        Ok(false)
    }

    /// Calculate percentage deviation between current and target amounts
    fn calculate_allocation_deviation(&self, current: u64, target: u64) -> Result<u32> {
        if target == 0 {
            return Ok(0);
        }

        let deviation = if current > target {
            ((current - target) * Self::TOTAL_BPS as u64) / target
        } else {
            ((target - current) * Self::TOTAL_BPS as u64) / target
        };

        Ok(deviation as u32)
    }

    /// Get rebalancing requirements for each asset
    pub fn get_rebalancing_requirements(&self) -> Result<(i64, i64, i64)> {
        let sol_diff = self.sol_allocation.target_amount as i64 - self.sol_allocation.current_amount as i64;
        let eth_diff = self.eth_allocation.target_amount as i64 - self.eth_allocation.current_amount as i64;
        let atom_diff = self.atom_allocation.target_amount as i64 - self.atom_allocation.current_amount as i64;
        
        Ok((sol_diff, eth_diff, atom_diff))
    }

    /// Add a validator to the SOL validator set
    pub fn add_sol_validator(&mut self, validator: ValidatorInfo) -> Result<()> {
        if self.sol_validators.len() >= 10 {
            return Err(VaultError::CommitmentLimitExceeded.into());
        }
        
        // Validate validator info
        if validator.commission > 2000 { // Max 20% commission
            return Err(VaultError::InvalidAllocation.into());
        }
        
        self.sol_validators.push(validator);
        Ok(())
    }

    /// Add a validator to the ETH validator set
    pub fn add_eth_validator(&mut self, validator: ValidatorInfo) -> Result<()> {
        if self.eth_validators.len() >= 10 {
            return Err(VaultError::CommitmentLimitExceeded.into());
        }
        
        // Validate validator info
        if validator.commission > 1000 { // Max 10% commission for ETH
            return Err(VaultError::InvalidAllocation.into());
        }
        
        self.eth_validators.push(validator);
        Ok(())
    }

    /// Update ATOM staking configuration
    pub fn update_atom_config(&mut self, config: AtomStakingConfig) -> Result<()> {
        // Validate that allocations add up correctly
        if config.everstake_allocation + config.osmosis_allocation != Self::ATOM_ALLOCATION_BPS {
            return Err(VaultError::InvalidAllocation.into());
        }
        
        self.atom_config = config;
        Ok(())
    }

    /// Select best validators based on performance and commission
    pub fn select_best_sol_validators(&self, count: usize) -> Vec<&ValidatorInfo> {
        let mut validators: Vec<&ValidatorInfo> = self.sol_validators
            .iter()
            .filter(|v| v.is_active)
            .collect();
        
        // Sort by performance score (descending) and commission (ascending)
        validators.sort_by(|a, b| {
            let score_cmp = b.performance_score.cmp(&a.performance_score);
            if score_cmp == std::cmp::Ordering::Equal {
                a.commission.cmp(&b.commission)
            } else {
                score_cmp
            }
        });
        
        validators.into_iter().take(count).collect()
    }

    /// Select best ETH validators
    pub fn select_best_eth_validators(&self, count: usize) -> Vec<&ValidatorInfo> {
        let mut validators: Vec<&ValidatorInfo> = self.eth_validators
            .iter()
            .filter(|v| v.is_active)
            .collect();
        
        // Sort by performance score (descending) and commission (ascending)
        validators.sort_by(|a, b| {
            let score_cmp = b.performance_score.cmp(&a.performance_score);
            if score_cmp == std::cmp::Ordering::Equal {
                a.commission.cmp(&b.commission)
            } else {
                score_cmp
            }
        });
        
        validators.into_iter().take(count).collect()
    }

    /// Update current allocation amounts
    pub fn update_current_allocations(&mut self, sol_amount: u64, eth_amount: u64, atom_amount: u64) -> Result<()> {
        self.sol_allocation.current_amount = sol_amount;
        self.eth_allocation.current_amount = eth_amount;
        self.atom_allocation.current_amount = atom_amount;
        
        self.total_staked = sol_amount + eth_amount + atom_amount;
        
        let clock = Clock::get()?;
        self.last_update = clock.unix_timestamp;
        
        Ok(())
    }

    /// Mark rebalancing as completed
    pub fn mark_rebalanced(&mut self) -> Result<()> {
        let clock = Clock::get()?;
        self.last_rebalance = clock.unix_timestamp;
        self.sol_allocation.last_rebalance = clock.unix_timestamp;
        self.eth_allocation.last_rebalance = clock.unix_timestamp;
        self.atom_allocation.last_rebalance = clock.unix_timestamp;
        Ok(())
    }

    /// Validate allocation percentages
    pub fn validate_allocations(&self) -> Result<()> {
        let total = self.sol_allocation.target_percentage 
            + self.eth_allocation.target_percentage 
            + self.atom_allocation.target_percentage;
        
        if total != Self::TOTAL_BPS {
            return Err(VaultError::InvalidAllocation.into());
        }
        
        Ok(())
    }

    /// Update validator stake amount
    pub fn update_validator_stake(&mut self, validator_address: &str, amount: u64) -> Result<()> {
        // Update SOL validator stake
        for validator in &mut self.sol_validators {
            if validator.address == validator_address {
                validator.stake_amount = validator.stake_amount.checked_add(amount).unwrap();
                return Ok(());
            }
        }
        
        // Update ETH validator stake
        for validator in &mut self.eth_validators {
            if validator.address == validator_address {
                validator.stake_amount = validator.stake_amount.checked_add(amount).unwrap();
                return Ok(());
            }
        }
        
        Err(VaultError::NoValidatorsAvailable.into())
    }

    /// Get total staked amount across all validators
    pub fn get_total_validator_stakes(&self) -> u64 {
        let sol_total: u64 = self.sol_validators.iter().map(|v| v.stake_amount).sum();
        let eth_total: u64 = self.eth_validators.iter().map(|v| v.stake_amount).sum();
        sol_total + eth_total
    }

    /// Deactivate underperforming validators
    pub fn deactivate_validator(&mut self, validator_address: &str) -> Result<()> {
        // Deactivate SOL validator
        for validator in &mut self.sol_validators {
            if validator.address == validator_address {
                validator.is_active = false;
                msg!("Deactivated SOL validator: {}", validator_address);
                return Ok(());
            }
        }
        
        // Deactivate ETH validator
        for validator in &mut self.eth_validators {
            if validator.address == validator_address {
                validator.is_active = false;
                msg!("Deactivated ETH validator: {}", validator_address);
                return Ok(());
            }
        }
        
        Err(VaultError::NoValidatorsAvailable.into())
    }

    /// Update validator performance score
    pub fn update_validator_performance(&mut self, validator_address: &str, new_score: u16) -> Result<()> {
        // Update SOL validator performance
        for validator in &mut self.sol_validators {
            if validator.address == validator_address {
                validator.performance_score = new_score;
                return Ok(());
            }
        }
        
        // Update ETH validator performance
        for validator in &mut self.eth_validators {
            if validator.address == validator_address {
                validator.performance_score = new_score;
                return Ok(());
            }
        }
        
        Err(VaultError::NoValidatorsAvailable.into())
    }
}



#[cfg(test)]
#[path = "staking_pool_tests.rs"]
mod tests;