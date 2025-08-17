use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct LiquidityEngineState {
    pub authority: Pubkey,
    pub bump: u8,
    pub total_volume: u64,
    pub total_fees: u64,
    pub is_paused: bool,
    pub jupiter_program: Pubkey,
    pub sanctum_program: Pubkey,
    pub bridge_program: Pubkey,
    pub fee_rate: u16, // Basis points (100 = 1%)
    pub max_slippage: u16, // Basis points
    pub supported_chains: Vec<ChainConfig>,
    pub liquidity_pools: Vec<LiquidityPoolConfig>,
    pub reserve_threshold: u64, // Minimum reserve for instant unstaking
    pub last_rebalance: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct ChainConfig {
    pub chain_id: u16,
    pub bridge_program: Pubkey,
    pub is_active: bool,
    pub min_transfer_amount: u64,
    pub max_transfer_amount: u64,
    pub bridge_fee: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct LiquidityPoolConfig {
    pub token_mint: Pubkey,
    pub pool_address: Pubkey,
    pub reserve_address: Pubkey,
    pub total_liquidity: u64,
    pub utilization_rate: u16, // Basis points
    pub is_active: bool,
}

#[account]
#[derive(InitSpace)]
pub struct UserLiquidityPosition {
    pub user: Pubkey,
    pub jsol_staked: u64,
    pub sol_equivalent: u64,
    pub rewards_earned: u64,
    pub last_stake_timestamp: i64,
    pub last_unstake_timestamp: i64,
    pub total_volume: u64,
    pub fees_paid: u64,
}

#[account]
#[derive(InitSpace)]
pub struct LiquidityReserve {
    pub token_mint: Pubkey,
    pub reserve_amount: u64,
    pub target_reserve: u64,
    pub utilization_rate: u16,
    pub last_rebalance: i64,
    pub emergency_threshold: u64,
    pub is_emergency_mode: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct SwapRoute {
    pub input_mint: Pubkey,
    pub output_mint: Pubkey,
    pub amount_in: u64,
    pub minimum_amount_out: u64,
    pub route_data: Vec<u8>,
    pub estimated_gas: u64,
    pub price_impact: u16, // Basis points
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct CrossChainTransfer {
    pub source_chain: u16,
    pub target_chain: u16,
    pub token_mint: Pubkey,
    pub amount: u64,
    pub recipient: [u8; 32],
    pub bridge_fee: u64,
    pub estimated_time: u64, // Seconds
    pub status: TransferStatus,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub enum TransferStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Refunded,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct JupiterSwapData {
    pub route: Vec<SwapLeg>,
    pub slippage_bps: u16,
    pub compute_unit_limit: u32,
    pub priority_fee: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct SwapLeg {
    pub swap_program: Pubkey,
    pub accounts: Vec<SwapAccount>,
    pub data: Vec<u8>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct SwapAccount {
    pub pubkey: Pubkey,
    pub is_signer: bool,
    pub is_writable: bool,
}

impl LiquidityEngineState {
    pub const INIT_SPACE: usize = 32 + 1 + 8 + 8 + 1 + 32 + 32 + 32 + 2 + 2 + 4 + 10 * ChainConfig::INIT_SPACE + 4 + 20 * LiquidityPoolConfig::INIT_SPACE + 8 + 8;
    
    pub fn add_supported_chain(&mut self, chain_config: ChainConfig) -> Result<()> {
        require!(
            !self.supported_chains.iter().any(|c| c.chain_id == chain_config.chain_id),
            crate::errors::VaultError::ChainAlreadySupported
        );
        self.supported_chains.push(chain_config);
        Ok(())
    }
    
    pub fn add_liquidity_pool(&mut self, pool_config: LiquidityPoolConfig) -> Result<()> {
        require!(
            !self.liquidity_pools.iter().any(|p| p.token_mint == pool_config.token_mint),
            crate::errors::VaultError::PoolAlreadyExists
        );
        self.liquidity_pools.push(pool_config);
        Ok(())
    }
    
    pub fn get_chain_config(&self, chain_id: u16) -> Option<&ChainConfig> {
        self.supported_chains.iter().find(|c| c.chain_id == chain_id)
    }
    
    pub fn get_pool_config(&self, token_mint: &Pubkey) -> Option<&LiquidityPoolConfig> {
        self.liquidity_pools.iter().find(|p| &p.token_mint == token_mint)
    }
    
    pub fn calculate_fee(&self, amount: u64) -> u64 {
        amount
            .checked_mul(self.fee_rate as u64)
            .unwrap_or(0)
            .checked_div(10000)
            .unwrap_or(0)
    }
    
    pub fn is_slippage_acceptable(&self, expected: u64, actual: u64) -> bool {
        if actual >= expected {
            return true;
        }
        
        let slippage = expected
            .checked_sub(actual)
            .unwrap_or(0)
            .checked_mul(10000)
            .unwrap_or(0)
            .checked_div(expected)
            .unwrap_or(u64::MAX);
        
        slippage <= self.max_slippage as u64
    }
}

impl ChainConfig {
    pub const INIT_SPACE: usize = 2 + 32 + 1 + 8 + 8 + 8;
    
    pub fn is_transfer_amount_valid(&self, amount: u64) -> bool {
        amount >= self.min_transfer_amount && amount <= self.max_transfer_amount
    }
}

impl LiquidityPoolConfig {
    pub const INIT_SPACE: usize = 32 + 32 + 32 + 8 + 2 + 1;
    
    pub fn calculate_utilization(&self, current_liquidity: u64) -> u16 {
        if self.total_liquidity == 0 {
            return 0;
        }
        
        let utilized = self.total_liquidity.saturating_sub(current_liquidity);
        ((utilized as u128 * 10000) / self.total_liquidity as u128) as u16
    }
}

impl UserLiquidityPosition {
    pub const INIT_SPACE: usize = 32 + 8 + 8 + 8 + 8 + 8 + 8 + 8;
    
    pub fn calculate_current_value(&self, exchange_rate: u64) -> u64 {
        self.jsol_staked
            .checked_mul(exchange_rate)
            .unwrap_or(0)
            .checked_div(1_000_000_000) // 9 decimals
            .unwrap_or(0)
    }
    
    pub fn add_stake(&mut self, amount: u64, timestamp: i64) {
        self.jsol_staked = self.jsol_staked.saturating_add(amount);
        self.last_stake_timestamp = timestamp;
    }
    
    pub fn remove_stake(&mut self, amount: u64, timestamp: i64) -> Result<()> {
        require!(
            self.jsol_staked >= amount,
            crate::errors::VaultError::InsufficientStake
        );
        self.jsol_staked = self.jsol_staked.saturating_sub(amount);
        self.last_unstake_timestamp = timestamp;
        Ok(())
    }
}

impl LiquidityReserve {
    pub const INIT_SPACE: usize = 32 + 8 + 8 + 2 + 8 + 8 + 1;
    
    pub fn needs_rebalancing(&self) -> bool {
        self.reserve_amount < self.target_reserve || 
        self.utilization_rate > 8000 || // 80% utilization
        self.is_emergency_mode
    }
    
    pub fn check_emergency_threshold(&mut self) -> bool {
        if self.reserve_amount <= self.emergency_threshold {
            self.is_emergency_mode = true;
            true
        } else if self.reserve_amount >= self.target_reserve {
            self.is_emergency_mode = false;
            false
        } else {
            self.is_emergency_mode
        }
    }
}

impl Default for TransferStatus {
    fn default() -> Self {
        TransferStatus::Pending
    }
}