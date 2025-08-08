use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::VaultError;

#[derive(Accounts)]
#[instruction(channel_id: [u8; 32])]
pub struct InitializeStateChannel<'info> {
    #[account(
        init,
        payer = authority,
        space = StateChannel::LEN,
        seeds = [b"state_channel", channel_id.as_ref()],
        bump
    )]
    pub state_channel: Account<'info, StateChannel>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateStateChannel<'info> {
    #[account(
        mut,
        seeds = [b"state_channel", state_channel.channel_id.as_ref()],
        bump = state_channel.bump
    )]
    pub state_channel: Account<'info, StateChannel>,
    
    #[account(mut)]
    pub participant: Signer<'info>,
}

#[derive(Accounts)]
pub struct SettleStateChannel<'info> {
    #[account(
        mut,
        seeds = [b"state_channel", state_channel.channel_id.as_ref()],
        bump = state_channel.bump
    )]
    pub state_channel: Account<'info, StateChannel>,
    
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
pub struct ChallengeStateChannel<'info> {
    #[account(
        mut,
        seeds = [b"state_channel", state_channel.channel_id.as_ref()],
        bump = state_channel.bump
    )]
    pub state_channel: Account<'info, StateChannel>,
    
    #[account(mut)]
    pub challenger: Signer<'info>,
}

/// Initialize a new state channel for off-chain reward calculations
pub fn initialize_state_channel(
    ctx: Context<InitializeStateChannel>,
    channel_id: [u8; 32],
    participants: Vec<Pubkey>,
    timeout_seconds: i64,
) -> Result<()> {
    let state_channel = &mut ctx.accounts.state_channel;
    
    // Validate participants
    if participants.is_empty() || participants.len() > 10 {
        return Err(VaultError::InvalidAllocation.into());
    }
    
    // Ensure authority is a participant
    if !participants.contains(&ctx.accounts.authority.key()) {
        return Err(VaultError::UnauthorizedAccess.into());
    }
    
    state_channel.initialize(
        channel_id,
        participants,
        timeout_seconds,
        ctx.bumps.state_channel,
    )?;
    
    msg!("State channel {} initialized with {} participants", 
         bs58::encode(channel_id).into_string(), 
         state_channel.participants.len());
    
    Ok(())
}

/// Update state channel with new reward calculations
pub fn update_state_channel(
    ctx: Context<UpdateStateChannel>,
    update: StateChannelUpdate,
    signatures: Vec<Vec<u8>>,
) -> Result<()> {
    let state_channel = &mut ctx.accounts.state_channel;
    
    // Verify participant is authorized
    if !state_channel.participants.contains(&ctx.accounts.participant.key()) {
        return Err(VaultError::UnauthorizedAccess.into());
    }
    
    // Validate signatures
    if !state_channel.verify_signatures(update.new_state_hash, &signatures)? {
        return Err(VaultError::SecurityViolation.into());
    }
    
    state_channel.update_state(update, signatures)?;
    
    msg!("State channel updated to nonce {}", state_channel.nonce);
    
    Ok(())
}

/// Settle state channel and apply final reward calculations on-chain
pub fn settle_state_channel(
    ctx: Context<SettleStateChannel>,
    final_calculations: Vec<RewardCalculation>,
) -> Result<()> {
    let state_channel = &mut ctx.accounts.state_channel;
    let staking_pool = &mut ctx.accounts.staking_pool;
    let treasury = &mut ctx.accounts.treasury;
    
    // Validate channel can be settled
    state_channel.validate_state()?;
    
    // Validate calculations
    let total_rewards: u64 = final_calculations
        .iter()
        .map(|calc| calc.calculated_reward)
        .sum();
    
    if total_rewards > treasury.user_rewards_pool {
        return Err(VaultError::InsufficientBalance.into());
    }
    
    // Apply calculations to on-chain state
    for calculation in &final_calculations {
        // In production, this would update individual user accounts
        // For now, we update the aggregate tracking
        staking_pool.rewards_distributed = staking_pool.rewards_distributed
            .checked_add(calculation.calculated_reward).unwrap();
    }
    
    // Deduct from treasury user rewards pool
    treasury.user_rewards_pool = treasury.user_rewards_pool
        .checked_sub(total_rewards).unwrap();
    
    // Settle the channel
    state_channel.settle_channel(final_calculations.clone())?;
    
    msg!("State channel settled with {} reward calculations totaling {}", 
         final_calculations.len(), total_rewards);
    
    Ok(())
}

/// Challenge a state channel update (dispute mechanism)
pub fn challenge_state_channel(
    ctx: Context<ChallengeStateChannel>,
    disputed_state_hash: [u8; 32],
    evidence: Vec<u8>,
) -> Result<()> {
    let state_channel = &mut ctx.accounts.state_channel;
    let challenger = ctx.accounts.challenger.key();
    
    let dispute_data = DisputeData {
        challenger,
        disputed_state_hash,
        evidence,
        challenge_timestamp: Clock::get()?.unix_timestamp,
    };
    
    state_channel.challenge_state(challenger, dispute_data)?;
    
    msg!("State channel challenged by {}", challenger);
    
    Ok(())
}

/// Process off-chain reward calculations and create state channel update
pub fn process_off_chain_rewards(
    users_and_commitments: Vec<(Pubkey, u64)>,
    total_staking_rewards: u64,
) -> Result<StateChannelUpdate> {
    let clock = Clock::get()?;
    let timestamp = clock.unix_timestamp;
    
    // Calculate rewards off-chain
    let calculations = OffChainRewardEngine::calculate_batch_rewards(
        &users_and_commitments,
        total_staking_rewards,
        timestamp,
    );
    
    // Generate state hash for the calculations
    let state_hash = StateChannel::calculate_state_hash(&calculations);
    
    // Create channel update
    let update = StateChannelUpdate {
        channel_id: [0; 32], // Would be set by caller
        new_state_hash: state_hash,
        nonce: 0, // Would be incremented by caller
        reward_calculations: calculations,
        timestamp,
    };
    
    Ok(update)
}

/// Validate state channel integrity and detect fraud
pub fn validate_channel_integrity(
    state_channel: &StateChannel,
    proposed_calculations: &[RewardCalculation],
) -> Result<bool> {
    // Validate channel is active
    if !state_channel.is_active {
        return Ok(false);
    }
    
    // Validate calculations hash matches state
    let calculated_hash = StateChannel::calculate_state_hash(proposed_calculations);
    if calculated_hash != state_channel.state_hash {
        return Ok(false);
    }
    
    // Validate total rewards don't exceed available pool
    let total_rewards: u64 = proposed_calculations
        .iter()
        .map(|calc| calc.calculated_reward)
        .sum();
    
    // Additional validation would check against treasury balance
    if total_rewards == 0 {
        return Ok(false);
    }
    
    Ok(true)
}

/// Generate fraud proof for invalid state transitions
pub fn generate_fraud_proof(
    state_channel: &StateChannel,
    invalid_calculations: &[RewardCalculation],
    valid_calculations: &[RewardCalculation],
) -> Result<Vec<u8>> {
    let invalid_hash = StateChannel::calculate_state_hash(invalid_calculations);
    let valid_hash = StateChannel::calculate_state_hash(valid_calculations);
    
    // Create evidence showing the invalid transition
    let mut evidence = Vec::new();
    
    // Add invalid calculations
    for calc in invalid_calculations {
        evidence.extend_from_slice(&calc.user.to_bytes());
        evidence.extend_from_slice(&calc.calculated_reward.to_le_bytes());
    }
    
    // Add valid calculations
    for calc in valid_calculations {
        evidence.extend_from_slice(&calc.user.to_bytes());
        evidence.extend_from_slice(&calc.calculated_reward.to_le_bytes());
    }
    
    let fraud_proof = state_channel.generate_fraud_proof(
        invalid_hash,
        valid_hash,
        evidence,
    )?;
    
    Ok(fraud_proof)
}

/// Batch process multiple reward calculations efficiently
pub fn batch_process_rewards(
    calculations: Vec<RewardCalculation>,
    max_batch_size: usize,
) -> Vec<Vec<RewardCalculation>> {
    calculations
        .chunks(max_batch_size)
        .map(|chunk| chunk.to_vec())
        .collect()
}

/// Monitor state channel health and detect issues
pub fn monitor_channel_health(state_channel: &StateChannel) -> ChannelHealthReport {
    let clock = Clock::get().unwrap();
    let current_time = clock.unix_timestamp;
    
    let status = state_channel.get_status();
    let time_since_update = current_time - state_channel.last_update;
    let time_until_timeout = state_channel.timeout - current_time;
    
    ChannelHealthReport {
        status,
        is_healthy: state_channel.is_active && time_until_timeout > 0,
        time_since_last_update: time_since_update,
        time_until_timeout,
        participant_count: state_channel.participants.len(),
        current_nonce: state_channel.nonce,
    }
}

/// Channel health monitoring report
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct ChannelHealthReport {
    pub status: ChannelStatus,
    pub is_healthy: bool,
    pub time_since_last_update: i64,
    pub time_until_timeout: i64,
    pub participant_count: usize,
    pub current_nonce: u64,
}
