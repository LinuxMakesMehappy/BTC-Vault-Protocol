use anchor_lang::prelude::*;
use crate::errors::VaultError;

/// State channel for off-chain reward calculations
#[account]
pub struct StateChannel {
    pub channel_id: [u8; 32],
    pub participants: Vec<Pubkey>,
    pub state_hash: [u8; 32],
    pub nonce: u64,
    pub timeout: i64,
    pub signatures: Vec<Vec<u8>>,
    pub is_active: bool,
    pub last_update: i64,
    pub dispute_period: i64,
    pub settlement_amount: u64,
    pub bump: u8,
}

/// State channel update for reward calculations
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct StateChannelUpdate {
    pub channel_id: [u8; 32],
    pub new_state_hash: [u8; 32],
    pub nonce: u64,
    pub reward_calculations: Vec<RewardCalculation>,
    pub timestamp: i64,
}

/// Individual reward calculation entry
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct RewardCalculation {
    pub user: Pubkey,
    pub btc_commitment: u64,
    pub calculated_reward: u64,
    pub calculation_timestamp: i64,
}

/// Dispute data for state channel challenges
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct DisputeData {
    pub challenger: Pubkey,
    pub disputed_state_hash: [u8; 32],
    pub evidence: Vec<u8>,
    pub challenge_timestamp: i64,
}

impl StateChannel {
    pub const LEN: usize = 8 + // discriminator
        32 + // channel_id
        4 + 32 * 10 + // participants (max 10)
        32 + // state_hash
        8 + // nonce
        8 + // timeout
        4 + (4 + 64) * 10 + // signatures (max 10, 64 bytes each)
        1 + // is_active
        8 + // last_update
        8 + // dispute_period
        8 + // settlement_amount
        1; // bump

    /// Initialize a new state channel
    pub fn initialize(
        &mut self,
        channel_id: [u8; 32],
        participants: Vec<Pubkey>,
        timeout_seconds: i64,
        bump: u8,
    ) -> Result<()> {
        if participants.len() > 10 {
            return Err(VaultError::InvalidAllocation.into());
        }

        let clock = Clock::get()?;
        
        self.channel_id = channel_id;
        self.participants = participants;
        self.state_hash = [0; 32]; // Initial empty state
        self.nonce = 0;
        self.timeout = clock.unix_timestamp + timeout_seconds;
        self.signatures = Vec::new();
        self.is_active = true;
        self.last_update = clock.unix_timestamp;
        self.dispute_period = 86400; // 24 hours in seconds
        self.settlement_amount = 0;
        self.bump = bump;

        Ok(())
    }

    /// Update state channel with new reward calculations
    pub fn update_state(
        &mut self,
        update: StateChannelUpdate,
        signatures: Vec<Vec<u8>>,
    ) -> Result<()> {
        // Validate channel is active
        if !self.is_active {
            return Err(VaultError::SecurityViolation.into());
        }

        // Validate nonce progression
        if update.nonce != self.nonce + 1 {
            return Err(VaultError::SecurityViolation.into());
        }

        // Validate channel ID matches
        if update.channel_id != self.channel_id {
            return Err(VaultError::SecurityViolation.into());
        }

        // Validate sufficient signatures (require majority)
        let required_signatures = (self.participants.len() / 2) + 1;
        if signatures.len() < required_signatures {
            return Err(VaultError::MultisigThresholdNotMet.into());
        }

        // Update state
        self.state_hash = update.new_state_hash;
        self.nonce = update.nonce;
        self.signatures = signatures;
        self.last_update = Clock::get()?.unix_timestamp;

        msg!("State channel {} updated to nonce {}", 
             bs58::encode(self.channel_id).into_string(), self.nonce);

        Ok(())
    }

    /// Challenge a state channel update (dispute mechanism)
    pub fn challenge_state(
        &mut self,
        challenger: Pubkey,
        _dispute_data: DisputeData,
    ) -> Result<()> {
        // Validate challenger is a participant
        if !self.participants.contains(&challenger) {
            return Err(VaultError::UnauthorizedAccess.into());
        }

        // Validate challenge is within dispute period
        let clock = Clock::get()?;
        if clock.unix_timestamp > self.last_update + self.dispute_period {
            return Err(VaultError::SecurityViolation.into());
        }

        // Mark channel as disputed (would trigger resolution process)
        self.is_active = false;

        msg!("State channel {} challenged by {}", 
             bs58::encode(self.channel_id).into_string(),
             challenger);

        Ok(())
    }

    /// Settle state channel and finalize rewards on-chain
    pub fn settle_channel(&mut self, final_calculations: Vec<RewardCalculation>) -> Result<()> {
        // Validate channel can be settled
        let clock = Clock::get()?;
        if clock.unix_timestamp < self.timeout {
            return Err(VaultError::SecurityViolation.into());
        }

        // Calculate total settlement amount
        let total_rewards: u64 = final_calculations
            .iter()
            .map(|calc| calc.calculated_reward)
            .sum();

        self.settlement_amount = total_rewards;
        self.is_active = false;

        msg!("State channel {} settled with {} total rewards", 
             bs58::encode(self.channel_id).into_string(), total_rewards);

        Ok(())
    }

    /// Validate state channel integrity
    pub fn validate_state(&self) -> Result<()> {
        // Check if channel has expired
        let clock = Clock::get()?;
        if clock.unix_timestamp > self.timeout && self.is_active {
            return Err(VaultError::SecurityViolation.into());
        }

        // Validate participants
        if self.participants.is_empty() || self.participants.len() > 10 {
            return Err(VaultError::InvalidAllocation.into());
        }

        // Validate nonce progression
        if self.nonce > 1000000 { // Prevent nonce overflow attacks
            return Err(VaultError::SecurityViolation.into());
        }

        Ok(())
    }

    /// Generate fraud proof for invalid state transitions
    pub fn generate_fraud_proof(
        &self,
        invalid_state_hash: [u8; 32],
        valid_state_hash: [u8; 32],
        evidence: Vec<u8>,
    ) -> Result<Vec<u8>> {
        // In production, this would generate cryptographic proof
        // that the state transition was invalid
        
        let mut proof = Vec::new();
        proof.extend_from_slice(&self.channel_id);
        proof.extend_from_slice(&invalid_state_hash);
        proof.extend_from_slice(&valid_state_hash);
        proof.extend_from_slice(&evidence);
        proof.extend_from_slice(&self.nonce.to_le_bytes());

        Ok(proof)
    }

    /// Verify signatures for state channel update
    pub fn verify_signatures(
        &self,
        _state_hash: [u8; 32],
        signatures: &[Vec<u8>],
    ) -> Result<bool> {
        // In production, this would verify ECDSA signatures
        // against participant public keys
        
        let required_signatures = (self.participants.len() / 2) + 1;
        
        if signatures.len() < required_signatures {
            return Ok(false);
        }

        // Simulate signature verification
        for (i, signature) in signatures.iter().enumerate() {
            if i >= self.participants.len() {
                return Ok(false);
            }
            
            // In reality, would verify signature against participant's pubkey
            if signature.len() != 64 {
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Calculate state hash for reward calculations
    pub fn calculate_state_hash(calculations: &[RewardCalculation]) -> [u8; 32] {
        use solana_program::hash::hash;
        
        let mut data = Vec::new();
        
        for calc in calculations {
            data.extend_from_slice(&calc.user.to_bytes());
            data.extend_from_slice(&calc.btc_commitment.to_le_bytes());
            data.extend_from_slice(&calc.calculated_reward.to_le_bytes());
            data.extend_from_slice(&calc.calculation_timestamp.to_le_bytes());
        }
        
        let hash_result = hash(&data);
        hash_result.to_bytes()
    }

    /// Get channel status for monitoring
    pub fn get_status(&self) -> ChannelStatus {
        let clock = Clock::get().map_err(|_| VaultError::ClockUnavailable)?;
        
        if !self.is_active {
            ChannelStatus::Closed
        } else if clock.unix_timestamp > self.timeout {
            ChannelStatus::Expired
        } else if clock.unix_timestamp > self.last_update + self.dispute_period {
            ChannelStatus::Finalized
        } else {
            ChannelStatus::Active
        }
    }
}

/// State channel status enumeration
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum ChannelStatus {
    Active,
    Finalized,
    Expired,
    Closed,
}

/// Off-chain reward calculation engine
pub struct OffChainRewardEngine;

impl OffChainRewardEngine {
    /// Calculate rewards for multiple users off-chain
    pub fn calculate_batch_rewards(
        users: &[(Pubkey, u64)], // (user, btc_commitment)
        total_staking_rewards: u64,
        calculation_timestamp: i64,
    ) -> Vec<RewardCalculation> {
        let total_commitments: u64 = users.iter().map(|(_, commitment)| commitment).sum();
        
        if total_commitments == 0 {
            return Vec::new();
        }

        // 50% of staking rewards go to users
        let user_reward_pool = total_staking_rewards / 2;
        
        users.iter().map(|(user, commitment)| {
            let user_reward = (commitment * user_reward_pool) / total_commitments;
            
            RewardCalculation {
                user: *user,
                btc_commitment: *commitment,
                calculated_reward: user_reward,
                calculation_timestamp,
            }
        }).collect()
    }

    /// Validate off-chain calculations against on-chain state
    pub fn validate_calculations(
        calculations: &[RewardCalculation],
        expected_total: u64,
    ) -> bool {
        let calculated_total: u64 = calculations
            .iter()
            .map(|calc| calc.calculated_reward)
            .sum();
        
        // Allow for small rounding differences (within 1%)
        let tolerance = expected_total / 100;
        calculated_total.abs_diff(expected_total) <= tolerance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_channel_initialization() {
        let mut channel = StateChannel {
            channel_id: [0; 32],
            participants: Vec::new(),
            state_hash: [0; 32],
            nonce: 0,
            timeout: 0,
            signatures: Vec::new(),
            is_active: false,
            last_update: 0,
            dispute_period: 0,
            settlement_amount: 0,
            bump: 0,
        };

        let participants = vec![Pubkey::new_unique(), Pubkey::new_unique()];
        let channel_id = [1; 32];
        
        assert!(channel.initialize(channel_id, participants.clone(), 3600, 255).is_ok());
        assert_eq!(channel.channel_id, channel_id);
        assert_eq!(channel.participants, participants);
        assert!(channel.is_active);
    }

    #[test]
    fn test_reward_calculation_hash() {
        let calculations = vec![
            RewardCalculation {
                user: Pubkey::new_unique(),
                btc_commitment: 100000000, // 1 BTC
                calculated_reward: 50000000, // 0.5 BTC equivalent
                calculation_timestamp: 1640995200,
            }
        ];

        let hash1 = StateChannel::calculate_state_hash(&calculations);
        let hash2 = StateChannel::calculate_state_hash(&calculations);
        
        assert_eq!(hash1, hash2); // Same input should produce same hash
    }

    #[test]
    fn test_off_chain_reward_calculation() {
        let users = vec![
            (Pubkey::new_unique(), 100000000), // 1 BTC
            (Pubkey::new_unique(), 200000000), // 2 BTC
        ];
        
        let total_rewards = 150000000; // 1.5 BTC equivalent
        let timestamp = 1640995200;
        
        let calculations = OffChainRewardEngine::calculate_batch_rewards(
            &users, total_rewards, timestamp
        );
        
        assert_eq!(calculations.len(), 2);
        
        // First user should get 1/3 of user rewards (1 BTC out of 3 total)
        // User reward pool is 50% of total = 75000000
        // User 1: (100000000 / 300000000) * 75000000 = 25000000
        assert_eq!(calculations[0].calculated_reward, 25000000);
        
        // Second user should get 2/3 of user rewards
        // User 2: (200000000 / 300000000) * 75000000 = 50000000
        assert_eq!(calculations[1].calculated_reward, 50000000);
    }
}
