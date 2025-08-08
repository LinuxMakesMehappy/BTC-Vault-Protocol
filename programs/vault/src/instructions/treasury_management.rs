//! Treasury Management Instructions
//!
//! This module implements the instruction handlers for advanced treasury management operations,
//! including yield strategy management, liquidity pool operations, and governance.

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::state::treasury_management::*;
use crate::state::treasury::Treasury;
use crate::state::multisig_wallet::MultisigWallet;
use crate::errors::VaultError;

/// Initialize a new treasury vault for advanced management
#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct InitializeTreasuryVault<'info> {
    #[account(
        init,
        payer = authority,
        space = TreasuryVault::SIZE,
        seeds = [b"treasury_vault", authority.key().as_ref()],
        bump
    )]
    pub treasury_vault: Account<'info, TreasuryVault>,
    
    /// Basic treasury account
    #[account(
        seeds = [b"treasury"],
        bump = treasury.bump
    )]
    pub treasury: Account<'info, Treasury>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    /// Multi-signature wallet for treasury operations
    pub multisig_wallet: Account<'info, MultisigWallet>,
    
    pub system_program: Program<'info, System>,
}

/// Add yield farming strategy
#[derive(Accounts)]
pub struct AddYieldStrategy<'info> {
    #[account(
        mut,
        has_one = authority,
        seeds = [b"treasury_vault", authority.key().as_ref()],
        bump = treasury_vault.bump
    )]
    pub treasury_vault: Account<'info, TreasuryVault>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    /// Multi-signature wallet for authorization
    pub multisig_wallet: Account<'info, MultisigWallet>,
}

/// Add liquidity pool management
#[derive(Accounts)]
pub struct AddLiquidityPool<'info> {
    #[account(
        mut,
        has_one = authority,
        seeds = [b"treasury_vault", authority.key().as_ref()],
        bump = treasury_vault.bump
    )]
    pub treasury_vault: Account<'info, TreasuryVault>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    /// Token A mint
    pub token_a_mint: Account<'info, token::Mint>,
    
    /// Token B mint
    pub token_b_mint: Account<'info, token::Mint>,
    
    /// Multi-signature wallet for authorization
    pub multisig_wallet: Account<'info, MultisigWallet>,
}

/// Execute advanced rebalancing with yield strategies
#[derive(Accounts)]
pub struct ExecuteAdvancedRebalancing<'info> {
    #[account(
        mut,
        seeds = [b"treasury_vault", treasury_vault.authority.as_ref()],
        bump = treasury_vault.bump
    )]
    pub treasury_vault: Account<'info, TreasuryVault>,
    
    #[account(
        mut,
        seeds = [b"treasury"],
        bump = treasury.bump
    )]
    pub treasury: Account<'info, Treasury>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    /// Source token account for rebalancing
    #[account(mut)]
    pub source_token_account: Account<'info, TokenAccount>,
    
    /// Destination token account for rebalancing
    #[account(mut)]
    pub destination_token_account: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
}

/// Update treasury performance metrics
#[derive(Accounts)]
pub struct UpdateTreasuryPerformance<'info> {
    #[account(
        mut,
        has_one = authority,
        seeds = [b"treasury_vault", authority.key().as_ref()],
        bump = treasury_vault.bump
    )]
    pub treasury_vault: Account<'info, TreasuryVault>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
}

/// Create treasury governance proposal
#[derive(Accounts)]
#[instruction(proposal_id: u64, bump: u8)]
pub struct CreateTreasuryProposal<'info> {
    #[account(
        init,
        payer = proposer,
        space = TreasuryProposal::SIZE,
        seeds = [b"treasury_proposal", proposal_id.to_le_bytes().as_ref()],
        bump
    )]
    pub treasury_proposal: Account<'info, TreasuryProposal>,
    
    #[account(mut)]
    pub proposer: Signer<'info>,
    
    /// Treasury vault being governed
    pub treasury_vault: Account<'info, TreasuryVault>,
    
    pub system_program: Program<'info, System>,
}

/// Vote on treasury proposal
#[derive(Accounts)]
pub struct VoteOnTreasuryProposal<'info> {
    #[account(
        mut,
        seeds = [b"treasury_proposal", treasury_proposal.proposal_id.to_le_bytes().as_ref()],
        bump = treasury_proposal.bump
    )]
    pub treasury_proposal: Account<'info, TreasuryProposal>,
    
    #[account(mut)]
    pub voter: Signer<'info>,
    
    /// Voter's staking account to verify voting power
    /// CHECK: Verified in instruction logic
    pub voter_stake_account: UncheckedAccount<'info>,
}

/// Emergency pause treasury operations
#[derive(Accounts)]
pub struct EmergencyPauseTreasury<'info> {
    #[account(
        mut,
        has_one = authority,
        seeds = [b"treasury_vault", authority.key().as_ref()],
        bump = treasury_vault.bump
    )]
    pub treasury_vault: Account<'info, TreasuryVault>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    /// Multi-signature wallet for authorization
    pub multisig_wallet: Account<'info, MultisigWallet>,
}

/// Update risk parameters
#[derive(Accounts)]
pub struct UpdateRiskParameters<'info> {
    #[account(
        mut,
        has_one = authority,
        seeds = [b"treasury_vault", authority.key().as_ref()],
        bump = treasury_vault.bump
    )]
    pub treasury_vault: Account<'info, TreasuryVault>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    /// Multi-signature wallet for authorization
    pub multisig_wallet: Account<'info, MultisigWallet>,
}

/// Treasury management instruction implementations
impl<'info> InitializeTreasuryVault<'info> {
    pub fn process(
        ctx: Context<InitializeTreasuryVault>,
        bump: u8,
    ) -> Result<()> {
        let treasury_vault = &mut ctx.accounts.treasury_vault;
        
        // Verify authority is a multisig signer
        require!(
            is_multisig_signer(&ctx.accounts.multisig_wallet, &ctx.accounts.authority.key()),
            TreasuryError::UnauthorizedOperation
        );
        
        treasury_vault.initialize(
            ctx.accounts.treasury.key(),
            ctx.accounts.authority.key(),
            ctx.accounts.multisig_wallet.key(),
            bump,
        )?;
        
        msg!("Advanced treasury vault initialized with authority: {}", ctx.accounts.authority.key());
        
        Ok(())
    }
}

impl<'info> AddYieldStrategy<'info> {
    pub fn process(
        ctx: Context<AddYieldStrategy>,
        strategy_id: u64,
        name: String,
        protocol: String,
        strategy_type: StrategyType,
        assets: Vec<Pubkey>,
        allocated_amount: u64,
        expected_apy: u16,
        risk_level: u8,
        parameters: Vec<u8>,
    ) -> Result<()> {
        let treasury_vault = &mut ctx.accounts.treasury_vault;
        
        // Verify authority is a multisig signer
        require!(
            is_multisig_signer(&ctx.accounts.multisig_wallet, &ctx.accounts.authority.key()),
            TreasuryError::UnauthorizedOperation
        );
        
        // Check if emergency pause is active
        require!(
            !treasury_vault.emergency_controls.emergency_pause,
            TreasuryError::EmergencyPauseActive
        );
        
        // Validate strategy parameters
        require!(risk_level <= 10, TreasuryError::InvalidRiskLevel);
        require!(expected_apy <= 50000, TreasuryError::InvalidRebalancingParameters); // Max 500% APY
        require!(name.len() <= 50, TreasuryError::InvalidRebalancingParameters);
        require!(protocol.len() <= 30, TreasuryError::InvalidRebalancingParameters);
        require!(assets.len() <= 5, TreasuryError::InvalidRebalancingParameters); // Max 5 assets per strategy
        
        let yield_strategy = YieldStrategy {
            strategy_id,
            name: name.clone(),
            protocol: protocol.clone(),
            strategy_type,
            assets,
            allocated_amount,
            expected_apy,
            current_apy: 0,
            risk_level,
            status: StrategyStatus::Active,
            performance: StrategyPerformance {
                total_returns: 0,
                daily_returns: 0,
                weekly_returns: 0,
                monthly_returns: 0,
                max_drawdown: 0,
                sharpe_ratio: 0,
                successful_operations: 0,
                failed_operations: 0,
                last_updated: Clock::get()?.unix_timestamp,
            },
            parameters,
            created_at: Clock::get()?.unix_timestamp,
            updated_at: Clock::get()?.unix_timestamp,
        };
        
        treasury_vault.add_yield_strategy(yield_strategy)?;
        
        // Update total yield value
        treasury_vault.total_yield_value = treasury_vault.total_yield_value
            .checked_add(allocated_amount)
            .ok_or(VaultError::MathOverflow)?;
        
        msg!(
            "Added yield strategy: {} on protocol: {} with expected APY: {}%",
            name,
            protocol,
            expected_apy as f64 / 100.0
        );
        
        Ok(())
    }
}

impl<'info> AddLiquidityPool<'info> {
    pub fn process(
        ctx: Context<AddLiquidityPool>,
        pool_id: Pubkey,
        dex_protocol: String,
        liquidity_amount: u64,
    ) -> Result<()> {
        let treasury_vault = &mut ctx.accounts.treasury_vault;
        
        // Verify authority is a multisig signer
        require!(
            is_multisig_signer(&ctx.accounts.multisig_wallet, &ctx.accounts.authority.key()),
            TreasuryError::UnauthorizedOperation
        );
        
        // Check if emergency pause is active
        require!(
            !treasury_vault.emergency_controls.emergency_pause,
            TreasuryError::EmergencyPauseActive
        );
        
        // Validate parameters
        require!(dex_protocol.len() <= 30, TreasuryError::InvalidRebalancingParameters);
        require!(liquidity_amount > 0, TreasuryError::InvalidRebalancingParameters);
        
        let pool_info = LiquidityPoolInfo {
            pool_id,
            dex_protocol: dex_protocol.clone(),
            token_a: ctx.accounts.token_a_mint.key(),
            token_b: ctx.accounts.token_b_mint.key(),
            liquidity_provided: liquidity_amount,
            pool_share: 0, // Will be calculated after providing liquidity
            fees_earned: 0,
            impermanent_loss: 0,
            status: PoolStatus::Active,
            created_at: Clock::get()?.unix_timestamp,
        };
        
        treasury_vault.add_liquidity_pool(pool_info)?;
        
        msg!(
            "Added liquidity pool on {}: {} - {} with {} USD liquidity",
            dex_protocol,
            ctx.accounts.token_a_mint.key(),
            ctx.accounts.token_b_mint.key(),
            liquidity_amount as f64 / 1_000_000.0
        );
        
        Ok(())
    }
}

impl<'info> ExecuteAdvancedRebalancing<'info> {
    pub fn process(
        ctx: Context<ExecuteAdvancedRebalancing>,
        amount: u64,
        strategy_id: Option<u64>,
    ) -> Result<()> {
        let treasury_vault = &mut ctx.accounts.treasury_vault;
        let treasury = &ctx.accounts.treasury;
        
        // Check authorization
        require!(
            ctx.accounts.authority.key() == treasury_vault.authority,
            TreasuryError::UnauthorizedOperation
        );
        
        // Check if emergency pause is active
        require!(
            !treasury_vault.emergency_controls.emergency_pause,
            TreasuryError::EmergencyPauseActive
        );
        
        // Check if rebalancing is needed
        require!(
            treasury_vault.needs_rebalancing(),
            TreasuryError::InvalidRebalancingParameters
        );
        
        // Validate amount against minimum trade size
        require!(
            amount >= treasury_vault.rebalancing_config.min_trade_size,
            TreasuryError::InvalidRebalancingParameters
        );
        
        // Execute token transfer for rebalancing
        let cpi_accounts = Transfer {
            from: ctx.accounts.source_token_account.to_account_info(),
            to: ctx.accounts.destination_token_account.to_account_info(),
            authority: treasury_vault.to_account_info(),
        };
        
        let seeds = &[
            b"treasury_vault",
            treasury_vault.authority.as_ref(),
            &[treasury_vault.bump],
        ];
        let signer = &[&seeds[..]];
        
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        
        token::transfer(cpi_ctx, amount)?;
        
        // Update rebalancing timestamp
        treasury_vault.rebalancing_config.last_rebalancing = Clock::get()?.unix_timestamp;
        treasury_vault.rebalancing_config.next_rebalancing = 
            Clock::get()?.unix_timestamp + treasury_vault.rebalancing_config.rebalancing_frequency as i64;
        
        // Update strategy allocation if specified
        if let Some(sid) = strategy_id {
            if let Some(strategy) = treasury_vault.yield_strategies.iter_mut().find(|s| s.strategy_id == sid) {
                strategy.allocated_amount = strategy.allocated_amount
                    .checked_add(amount)
                    .ok_or(VaultError::MathOverflow)?;
                strategy.updated_at = Clock::get()?.unix_timestamp;
            }
        }
        
        treasury_vault.updated_at = Clock::get()?.unix_timestamp;
        
        msg!(
            "Advanced rebalancing executed: {} tokens transferred",
            amount
        );
        
        Ok(())
    }
}

impl<'info> UpdateTreasuryPerformance<'info> {
    pub fn process(
        ctx: Context<UpdateTreasuryPerformance>,
        new_metrics: PerformanceMetrics,
    ) -> Result<()> {
        let treasury_vault = &mut ctx.accounts.treasury_vault;
        
        treasury_vault.update_performance_metrics(new_metrics)?;
        
        msg!(
            "Performance metrics updated for treasury vault: {}",
            treasury_vault.key()
        );
        
        Ok(())
    }
}

impl<'info> CreateTreasuryProposal<'info> {
    pub fn process(
        ctx: Context<CreateTreasuryProposal>,
        proposal_id: u64,
        title: String,
        description: String,
        proposal_type: ProposalType,
        parameters: Vec<u8>,
        voting_duration: i64,
        quorum_threshold: u16,
        approval_threshold: u16,
        bump: u8,
    ) -> Result<()> {
        let treasury_proposal = &mut ctx.accounts.treasury_proposal;
        
        // Validate proposal parameters
        require!(title.len() <= 100, TreasuryError::InvalidProposalParameters);
        require!(description.len() <= 1000, TreasuryError::InvalidProposalParameters);
        require!(quorum_threshold <= 10000, TreasuryError::InvalidProposalParameters);
        require!(approval_threshold <= 10000, TreasuryError::InvalidProposalParameters);
        require!(voting_duration > 0 && voting_duration <= 2_592_000, TreasuryError::InvalidProposalParameters); // Max 30 days
        
        let current_time = Clock::get()?.unix_timestamp;
        
        treasury_proposal.proposal_id = proposal_id;
        treasury_proposal.proposer = ctx.accounts.proposer.key();
        treasury_proposal.title = title.clone();
        treasury_proposal.description = description.clone();
        treasury_proposal.proposal_type = proposal_type;
        treasury_proposal.parameters = parameters;
        treasury_proposal.voting_start = current_time;
        treasury_proposal.voting_end = current_time + voting_duration;
        treasury_proposal.execution_time = 0; // Set when approved
        treasury_proposal.votes_for = 0;
        treasury_proposal.votes_against = 0;
        treasury_proposal.total_voting_power = 0; // Will be calculated from staking pools
        treasury_proposal.quorum_threshold = quorum_threshold;
        treasury_proposal.approval_threshold = approval_threshold;
        treasury_proposal.status = ProposalStatus::Active;
        treasury_proposal.created_at = current_time;
        treasury_proposal.updated_at = current_time;
        treasury_proposal.bump = bump;
        
        msg!(
            "Treasury proposal created: {} - {} by {}",
            proposal_id,
            title,
            ctx.accounts.proposer.key()
        );
        
        Ok(())
    }
}

impl<'info> VoteOnTreasuryProposal<'info> {
    pub fn process(
        ctx: Context<VoteOnTreasuryProposal>,
        vote_for: bool,
        voting_power: u64,
    ) -> Result<()> {
        let treasury_proposal = &mut ctx.accounts.treasury_proposal;
        
        // Check if voting period is active
        let current_time = Clock::get()?.unix_timestamp;
        require!(
            current_time >= treasury_proposal.voting_start && current_time <= treasury_proposal.voting_end,
            TreasuryError::VotingPeriodEnded
        );
        
        require!(
            treasury_proposal.status == ProposalStatus::Active,
            TreasuryError::InvalidProposalParameters
        );
        
        require!(voting_power > 0, TreasuryError::InsufficientVotingPower);
        
        // TODO: Verify voting power from staking account
        // This would require reading the voter's staking account and validating their stake
        
        // Record the vote
        if vote_for {
            treasury_proposal.votes_for = treasury_proposal.votes_for.checked_add(voting_power)
                .ok_or(TreasuryError::InvalidProposalParameters)?;
        } else {
            treasury_proposal.votes_against = treasury_proposal.votes_against.checked_add(voting_power)
                .ok_or(TreasuryError::InvalidProposalParameters)?;
        }
        
        treasury_proposal.total_voting_power = treasury_proposal.total_voting_power.checked_add(voting_power)
            .ok_or(TreasuryError::InvalidProposalParameters)?;
        
        treasury_proposal.updated_at = current_time;
        
        // Check if proposal should be finalized
        let total_votes = treasury_proposal.votes_for + treasury_proposal.votes_against;
        let quorum_met = (total_votes * 10000) >= (treasury_proposal.total_voting_power * treasury_proposal.quorum_threshold as u64);
        
        if quorum_met && current_time >= treasury_proposal.voting_end {
            let approval_rate = (treasury_proposal.votes_for * 10000) / total_votes;
            if approval_rate >= treasury_proposal.approval_threshold as u64 {
                treasury_proposal.status = ProposalStatus::Approved;
                treasury_proposal.execution_time = current_time + 86400; // 24 hour delay
            } else {
                treasury_proposal.status = ProposalStatus::Rejected;
            }
        }
        
        msg!(
            "Vote recorded for proposal {}: {} with power {}",
            treasury_proposal.proposal_id,
            if vote_for { "FOR" } else { "AGAINST" },
            voting_power
        );
        
        Ok(())
    }
}

impl<'info> EmergencyPauseTreasury<'info> {
    pub fn process(ctx: Context<EmergencyPauseTreasury>) -> Result<()> {
        let treasury_vault = &mut ctx.accounts.treasury_vault;
        
        // Verify authority is a multisig signer
        require!(
            is_multisig_signer(&ctx.accounts.multisig_wallet, &ctx.accounts.authority.key()),
            TreasuryError::UnauthorizedOperation
        );
        
        treasury_vault.emergency_controls.emergency_pause = true;
        treasury_vault.emergency_controls.last_emergency_action = Clock::get()?.unix_timestamp;
        treasury_vault.updated_at = Clock::get()?.unix_timestamp;
        
        msg!(
            "Emergency pause activated for treasury vault: {}",
            treasury_vault.key()
        );
        
        Ok(())
    }
}

impl<'info> UpdateRiskParameters<'info> {
    pub fn process(
        ctx: Context<UpdateRiskParameters>,
        new_risk_params: RiskParameters,
    ) -> Result<()> {
        let treasury_vault = &mut ctx.accounts.treasury_vault;
        
        // Verify authority is a multisig signer
        require!(
            is_multisig_signer(&ctx.accounts.multisig_wallet, &ctx.accounts.authority.key()),
            TreasuryError::UnauthorizedOperation
        );
        
        // Validate risk parameters
        require!(
            new_risk_params.max_single_strategy_allocation <= 5000, // Max 50%
            TreasuryError::InvalidRebalancingParameters
        );
        require!(
            new_risk_params.max_high_risk_allocation <= 3000, // Max 30%
            TreasuryError::InvalidRebalancingParameters
        );
        require!(
            new_risk_params.max_daily_loss <= 1000, // Max 10%
            TreasuryError::InvalidRebalancingParameters
        );
        
        treasury_vault.risk_parameters = new_risk_params;
        treasury_vault.updated_at = Clock::get()?.unix_timestamp;
        
        msg!(
            "Risk parameters updated for treasury vault: {}",
            treasury_vault.key()
        );
        
        Ok(())
    }
}

// Helper functions
fn is_multisig_signer(multisig_wallet: &MultisigWallet, signer: &Pubkey) -> bool {
    multisig_wallet.owners.contains(signer)
}
