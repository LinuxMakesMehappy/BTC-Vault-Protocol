//! Advanced Treasury Management State Definitions
//! 
//! This module extends the basic treasury functionality with advanced features
//! including yield farming strategies, liquidity management, and governance.

use anchor_lang::prelude::*;
use crate::state::treasury::*;
use crate::errors::VaultError;

/// Advanced treasury vault with yield farming and liquidity management
#[account]
#[derive(Debug)]
pub struct TreasuryVault {
    /// Basic treasury account
    pub treasury: Pubkey,
    /// Authority that can manage the treasury
    pub authority: Pubkey,
    /// Multi-signature wallet for treasury operations
    pub multisig_wallet: Pubkey,
    /// Total value locked in yield strategies (in USD, scaled by 1e6)
    pub total_yield_value: u64,
    /// Active yield farming strategies
    pub yield_strategies: Vec<YieldStrategy>,
    /// Liquidity pools managed by treasury
    pub liquidity_pools: Vec<LiquidityPoolInfo>,
    /// Risk management parameters
    pub risk_parameters: RiskParameters,
    /// Performance metrics
    pub performance_metrics: PerformanceMetrics,
    /// Rebalancing configuration
    pub rebalancing_config: RebalancingConfig,
    /// Emergency controls
    pub emergency_controls: EmergencyControls,
    /// Creation timestamp
    pub created_at: i64,
    /// Last update timestamp
    pub updated_at: i64,
    /// Account bump seed
    pub bump: u8,
}

/// Yield farming strategy configuration
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub struct YieldStrategy {
    /// Strategy identifier
    pub strategy_id: u64,
    /// Strategy name
    pub name: String,
    /// Protocol being used (e.g., "Orca", "Raydium", "Marinade")
    pub protocol: String,
    /// Strategy type
    pub strategy_type: StrategyType,
    /// Assets involved in the strategy
    pub assets: Vec<Pubkey>,
    /// Allocated amount (in USD, scaled by 1e6)
    pub allocated_amount: u64,
    /// Expected APY (scaled by 1e4, so 1000 = 10%)
    pub expected_apy: u16,
    /// Current APY (scaled by 1e4)
    pub current_apy: u16,
    /// Risk level (1-10, where 10 is highest risk)
    pub risk_level: u8,
    /// Strategy status
    pub status: StrategyStatus,
    /// Performance tracking
    pub performance: StrategyPerformance,
    /// Strategy parameters (protocol-specific)
    pub parameters: Vec<u8>,
    /// Creation timestamp
    pub created_at: i64,
    /// Last update timestamp
    pub updated_at: i64,
}

/// Types of yield farming strategies
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum StrategyType {
    /// Liquidity provision on DEXs
    LiquidityProvision,
    /// Lending on money markets
    Lending,
    /// Liquid staking
    LiquidStaking,
    /// Yield farming on protocols
    YieldFarming,
    /// Arbitrage opportunities
    Arbitrage,
    /// Market making
    MarketMaking,
}

/// Strategy execution status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum StrategyStatus {
    /// Strategy is active and earning yield
    Active,
    /// Strategy is paused
    Paused,
    /// Strategy is being wound down
    Unwinding,
    /// Strategy has been completed
    Completed,
    /// Strategy has failed or been emergency stopped
    Failed,
}

/// Strategy performance tracking
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub struct StrategyPerformance {
    /// Total returns generated (in USD, scaled by 1e6)
    pub total_returns: u64,
    /// Returns over last 24 hours
    pub daily_returns: i64,
    /// Returns over last 7 days
    pub weekly_returns: i64,
    /// Returns over last 30 days
    pub monthly_returns: i64,
    /// Maximum drawdown experienced
    pub max_drawdown: u16,
    /// Sharpe ratio (scaled by 1e4)
    pub sharpe_ratio: i16,
    /// Number of successful trades/operations
    pub successful_operations: u32,
    /// Number of failed operations
    pub failed_operations: u32,
    /// Last performance update
    pub last_updated: i64,
}

/// Liquidity pool information
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub struct LiquidityPoolInfo {
    /// Pool identifier
    pub pool_id: Pubkey,
    /// DEX protocol
    pub dex_protocol: String,
    /// Token pair
    pub token_a: Pubkey,
    pub token_b: Pubkey,
    /// Liquidity provided (in USD, scaled by 1e6)
    pub liquidity_provided: u64,
    /// Pool share percentage (scaled by 1e4)
    pub pool_share: u16,
    /// Fees earned (in USD, scaled by 1e6)
    pub fees_earned: u64,
    /// Impermanent loss (in USD, scaled by 1e6)
    pub impermanent_loss: i64,
    /// Pool status
    pub status: PoolStatus,
    /// Creation timestamp
    pub created_at: i64,
}

/// Liquidity pool status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum PoolStatus {
    /// Pool is active and providing liquidity
    Active,
    /// Pool is paused
    Paused,
    /// Liquidity is being withdrawn
    Withdrawing,
    /// Pool has been closed
    Closed,
}

/// Risk management parameters for the treasury
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub struct RiskParameters {
    /// Maximum allocation to any single strategy (scaled by 1e4)
    pub max_single_strategy_allocation: u16,
    /// Maximum allocation to high-risk strategies (scaled by 1e4)
    pub max_high_risk_allocation: u16,
    /// Maximum daily loss threshold (scaled by 1e4)
    pub max_daily_loss: u16,
    /// Maximum monthly loss threshold (scaled by 1e4)
    pub max_monthly_loss: u16,
    /// Minimum liquidity ratio to maintain (scaled by 1e4)
    pub min_liquidity_ratio: u16,
    /// Maximum leverage allowed
    pub max_leverage: u16,
    /// Value at Risk (VaR) limit (scaled by 1e6)
    pub var_limit: u64,
    /// Risk monitoring enabled
    pub risk_monitoring_enabled: bool,
    /// Last risk assessment timestamp
    pub last_risk_assessment: i64,
}

/// Treasury performance metrics
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub struct PerformanceMetrics {
    /// Total returns since inception (in USD, scaled by 1e6)
    pub total_returns: u64,
    /// Annualized return rate (scaled by 1e4)
    pub annualized_return: u16,
    /// Volatility measure (scaled by 1e4)
    pub volatility: u16,
    /// Sharpe ratio (scaled by 1e4)
    pub sharpe_ratio: i16,
    /// Maximum drawdown (scaled by 1e4)
    pub max_drawdown: u16,
    /// Win rate for strategies (scaled by 1e4)
    pub win_rate: u16,
    /// Average holding period (in days)
    pub avg_holding_period: u16,
    /// Total fees paid (in USD, scaled by 1e6)
    pub total_fees_paid: u64,
    /// Net profit after fees (in USD, scaled by 1e6)
    pub net_profit: u64,
    /// Performance attribution by strategy
    pub strategy_attribution: Vec<(u64, i64)>, // (strategy_id, contribution)
    /// Last performance calculation
    pub last_calculated: i64,
}

/// Automated rebalancing configuration
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub struct RebalancingConfig {
    /// Automatic rebalancing enabled
    pub auto_rebalancing_enabled: bool,
    /// Rebalancing frequency (in seconds)
    pub rebalancing_frequency: u32,
    /// Threshold for triggering rebalancing (scaled by 1e4)
    pub rebalancing_threshold: u16,
    /// Maximum slippage allowed during rebalancing (scaled by 1e4)
    pub max_slippage: u16,
    /// Minimum trade size for rebalancing (in USD, scaled by 1e6)
    pub min_trade_size: u64,
    /// Gas budget for rebalancing operations (in SOL, scaled by 1e9)
    pub gas_budget: u64,
    /// DEX preferences for rebalancing
    pub dex_preferences: Vec<DexPreference>,
    /// Last rebalancing timestamp
    pub last_rebalancing: i64,
    /// Next scheduled rebalancing
    pub next_rebalancing: i64,
}

/// DEX preference for trading
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub struct DexPreference {
    /// DEX name (e.g., "Jupiter", "Orca", "Raydium")
    pub dex_name: String,
    /// Priority (1 = highest priority)
    pub priority: u8,
    /// Maximum allocation to this DEX (scaled by 1e4)
    pub max_allocation: u16,
    /// Minimum liquidity required
    pub min_liquidity: u64,
}

/// Emergency controls for treasury management
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub struct EmergencyControls {
    /// Emergency pause enabled
    pub emergency_pause: bool,
    /// Emergency withdrawal enabled
    pub emergency_withdrawal: bool,
    /// Circuit breaker triggers
    pub circuit_breakers: Vec<CircuitBreaker>,
    /// Emergency contacts
    pub emergency_contacts: Vec<Pubkey>,
    /// Last emergency action timestamp
    pub last_emergency_action: i64,
}

/// Circuit breaker configuration
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub struct CircuitBreaker {
    /// Trigger condition
    pub condition: CircuitBreakerCondition,
    /// Threshold value
    pub threshold: u64,
    /// Action to take when triggered
    pub action: CircuitBreakerAction,
    /// Cooldown period (in seconds)
    pub cooldown_period: u32,
    /// Last triggered timestamp
    pub last_triggered: i64,
    /// Number of times triggered
    pub trigger_count: u32,
}

/// Circuit breaker trigger conditions
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum CircuitBreakerCondition {
    /// Daily loss exceeds threshold
    DailyLoss,
    /// Single strategy loss exceeds threshold
    SingleStrategyLoss,
    /// Liquidity drops below threshold
    LowLiquidity,
    /// Unusual trading volume
    HighVolume,
    /// Price deviation from oracle
    PriceDeviation,
}

/// Circuit breaker actions
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum CircuitBreakerAction {
    /// Pause all trading
    PauseTrading,
    /// Pause specific strategy
    PauseStrategy,
    /// Reduce position sizes
    ReducePositions,
    /// Emergency liquidation
    EmergencyLiquidation,
    /// Notify administrators
    NotifyAdmins,
}

/// Treasury governance proposal
#[account]
#[derive(Debug)]
pub struct TreasuryProposal {
    /// Proposal ID
    pub proposal_id: u64,
    /// Proposer
    pub proposer: Pubkey,
    /// Proposal title
    pub title: String,
    /// Proposal description
    pub description: String,
    /// Proposal type
    pub proposal_type: ProposalType,
    /// Proposal parameters
    pub parameters: Vec<u8>,
    /// Voting start time
    pub voting_start: i64,
    /// Voting end time
    pub voting_end: i64,
    /// Execution time (if approved)
    pub execution_time: i64,
    /// Votes in favor
    pub votes_for: u64,
    /// Votes against
    pub votes_against: u64,
    /// Total voting power
    pub total_voting_power: u64,
    /// Quorum threshold (scaled by 1e4)
    pub quorum_threshold: u16,
    /// Approval threshold (scaled by 1e4)
    pub approval_threshold: u16,
    /// Proposal status
    pub status: ProposalStatus,
    /// Creation timestamp
    pub created_at: i64,
    /// Last update timestamp
    pub updated_at: i64,
    /// Account bump seed
    pub bump: u8,
}

/// Types of treasury governance proposals
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum ProposalType {
    /// Add new yield strategy
    AddStrategy,
    /// Remove existing strategy
    RemoveStrategy,
    /// Update risk parameters
    RiskParameters,
    /// Emergency action
    EmergencyAction,
    /// Fee structure change
    FeeChange,
    /// Governance parameter change
    GovernanceChange,
}

/// Proposal execution status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum ProposalStatus {
    /// Proposal is active for voting
    Active,
    /// Proposal has been approved
    Approved,
    /// Proposal has been rejected
    Rejected,
    /// Proposal has been executed
    Executed,
    /// Proposal has been cancelled
    Cancelled,
    /// Proposal has expired
    Expired,
}

/// Implementation of treasury management state
impl TreasuryVault {
    /// Size of the treasury vault account
    pub const SIZE: usize = 8 + // discriminator
        32 + // treasury
        32 + // authority
        32 + // multisig_wallet
        8 + // total_yield_value
        4 + (20 * 200) + // yield_strategies (max 20, ~200 bytes each)
        4 + (10 * 100) + // liquidity_pools (max 10, ~100 bytes each)
        200 + // risk_parameters
        300 + // performance_metrics
        200 + // rebalancing_config
        200 + // emergency_controls
        8 + // created_at
        8 + // updated_at
        1; // bump
    
    /// Initialize a new treasury vault
    pub fn initialize(
        &mut self,
        treasury: Pubkey,
        authority: Pubkey,
        multisig_wallet: Pubkey,
        bump: u8,
    ) -> Result<()> {
        self.treasury = treasury;
        self.authority = authority;
        self.multisig_wallet = multisig_wallet;
        self.total_yield_value = 0;
        self.yield_strategies = Vec::new();
        self.liquidity_pools = Vec::new();
        self.risk_parameters = RiskParameters::default();
        self.performance_metrics = PerformanceMetrics::default();
        self.rebalancing_config = RebalancingConfig::default();
        self.emergency_controls = EmergencyControls::default();
        self.created_at = Clock::get()?.unix_timestamp;
        self.updated_at = Clock::get()?.unix_timestamp;
        self.bump = bump;
        
        Ok(())
    }
    
    /// Add a new yield strategy
    pub fn add_yield_strategy(
        &mut self,
        strategy: YieldStrategy,
    ) -> Result<()> {
        require!(
            self.yield_strategies.len() < 20,
            TreasuryError::TooManyStrategies
        );
        
        // Validate risk level
        require!(
            strategy.risk_level <= 10,
            TreasuryError::InvalidRiskLevel
        );
        
        // Check risk limits
        let high_risk_allocation = self.calculate_high_risk_allocation(&strategy)?;
        require!(
            high_risk_allocation <= self.risk_parameters.max_high_risk_allocation,
            TreasuryError::RiskLimitExceeded
        );
        
        self.yield_strategies.push(strategy);
        self.updated_at = Clock::get()?.unix_timestamp;
        
        Ok(())
    }
    
    /// Add a new liquidity pool
    pub fn add_liquidity_pool(
        &mut self,
        pool_info: LiquidityPoolInfo,
    ) -> Result<()> {
        require!(
            self.liquidity_pools.len() < 10,
            TreasuryError::TooManyLiquidityPools
        );
        
        self.liquidity_pools.push(pool_info);
        self.updated_at = Clock::get()?.unix_timestamp;
        
        Ok(())
    }
    
    /// Calculate total portfolio value including yield strategies
    pub fn calculate_total_value(&self, treasury: &Treasury) -> u64 {
        treasury.total_assets + self.total_yield_value
    }
    
    /// Check if rebalancing is needed
    pub fn needs_rebalancing(&self) -> bool {
        if !self.rebalancing_config.auto_rebalancing_enabled {
            return false;
        }
        
        let current_time = Clock::get().map_err(|_| VaultError::ClockUnavailable)?.unix_timestamp;
        let time_since_last = current_time - self.rebalancing_config.last_rebalancing;
        
        // Check time-based rebalancing
        if time_since_last >= self.rebalancing_config.rebalancing_frequency as i64 {
            return true;
        }
        
        // Check performance-based rebalancing triggers
        self.check_performance_triggers()
    }
    
    /// Update performance metrics
    pub fn update_performance_metrics(
        &mut self,
        new_metrics: PerformanceMetrics,
    ) -> Result<()> {
        self.performance_metrics = new_metrics;
        self.updated_at = Clock::get()?.unix_timestamp;
        Ok(())
    }
    
    /// Calculate high-risk allocation percentage
    fn calculate_high_risk_allocation(&self, new_strategy: &YieldStrategy) -> Result<u16> {
        let total_allocation = self.yield_strategies.iter()
            .filter(|s| s.risk_level >= 7) // High risk threshold
            .map(|s| s.allocated_amount)
            .sum::<u64>() + if new_strategy.risk_level >= 7 { new_strategy.allocated_amount } else { 0 };
        
        let total_value = self.total_yield_value + new_strategy.allocated_amount;
        
        if total_value == 0 {
            return Ok(0);
        }
        
        Ok(((total_allocation * 10000) / total_value) as u16)
    }
    
    /// Check performance-based rebalancing triggers
    fn check_performance_triggers(&self) -> bool {
        // Check if any strategy is underperforming significantly
        for strategy in &self.yield_strategies {
            if strategy.status == StrategyStatus::Active {
                let performance_gap = strategy.expected_apy as i16 - strategy.current_apy as i16;
                if performance_gap > 500 { // 5% underperformance
                    return true;
                }
            }
        }
        
        false
    }
}

/// Default implementations
impl Default for RiskParameters {
    fn default() -> Self {
        Self {
            max_single_strategy_allocation: 2000, // 20%
            max_high_risk_allocation: 1500, // 15%
            max_daily_loss: 300, // 3%
            max_monthly_loss: 1000, // 10%
            min_liquidity_ratio: 1000, // 10%
            max_leverage: 200, // 2x
            var_limit: 500_000, // $500k
            risk_monitoring_enabled: true,
            last_risk_assessment: 0,
        }
    }
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            total_returns: 0,
            annualized_return: 0,
            volatility: 0,
            sharpe_ratio: 0,
            max_drawdown: 0,
            win_rate: 0,
            avg_holding_period: 0,
            total_fees_paid: 0,
            net_profit: 0,
            strategy_attribution: Vec::new(),
            last_calculated: 0,
        }
    }
}

impl Default for RebalancingConfig {
    fn default() -> Self {
        Self {
            auto_rebalancing_enabled: true,
            rebalancing_frequency: 86400, // 24 hours
            rebalancing_threshold: 500, // 5%
            max_slippage: 100, // 1%
            min_trade_size: 1000_000, // $1000
            gas_budget: 100_000_000, // 0.1 SOL
            dex_preferences: Vec::new(),
            last_rebalancing: 0,
            next_rebalancing: 0,
        }
    }
}

impl Default for EmergencyControls {
    fn default() -> Self {
        Self {
            emergency_pause: false,
            emergency_withdrawal: false,
            circuit_breakers: Vec::new(),
            emergency_contacts: Vec::new(),
            last_emergency_action: 0,
        }
    }
}

impl TreasuryProposal {
    pub const SIZE: usize = 8 + // discriminator
        8 + // proposal_id
        32 + // proposer
        4 + 100 + // title (max 100 chars)
        4 + 1000 + // description (max 1000 chars)
        1 + // proposal_type
        4 + 256 + // parameters (max 256 bytes)
        8 + // voting_start
        8 + // voting_end
        8 + // execution_time
        8 + // votes_for
        8 + // votes_against
        8 + // total_voting_power
        2 + // quorum_threshold
        2 + // approval_threshold
        1 + // status
        8 + // created_at
        8 + // updated_at
        1; // bump
}

/// Treasury management errors
#[error_code]
pub enum TreasuryError {
    #[msg("Too many yield strategies")]
    TooManyStrategies,
    
    #[msg("Too many liquidity pools")]
    TooManyLiquidityPools,
    
    #[msg("Invalid risk level")]
    InvalidRiskLevel,
    
    #[msg("Risk limit exceeded")]
    RiskLimitExceeded,
    
    #[msg("Emergency pause is active")]
    EmergencyPauseActive,
    
    #[msg("Unauthorized treasury operation")]
    UnauthorizedOperation,
    
    #[msg("Invalid rebalancing parameters")]
    InvalidRebalancingParameters,
    
    #[msg("Strategy not found")]
    StrategyNotFound,
    
    #[msg("Invalid proposal parameters")]
    InvalidProposalParameters,
    
    #[msg("Voting period has ended")]
    VotingPeriodEnded,
    
    #[msg("Insufficient voting power")]
    InsufficientVotingPower,
}
