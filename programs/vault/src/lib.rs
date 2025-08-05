use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod errors;
pub mod traits;

use instructions::btc_commitment::*;
use instructions::oracle::*;
use instructions::staking::*;
use instructions::rewards::*;
use instructions::state_channel::*;
use instructions::multisig::*;
use instructions::payment::*;
use crate::traits::PaymentType;
use crate::state::{StateChannelUpdate, RewardCalculation, SignerInfo, TransactionType, TransactionPriority, SignatureType, PaymentMethod, LightningConfig, UsdcConfig, ReinvestmentConfig};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod vault {
    use super::*;

    pub fn commit_btc(
        ctx: Context<CommitBTC>,
        amount: u64,
        btc_address: String,
        ecdsa_proof: Vec<u8>,
        public_key: Vec<u8>,
    ) -> Result<()> {
        instructions::btc_commitment::commit_btc(ctx, amount, btc_address, ecdsa_proof, public_key)
    }

    pub fn verify_balance(ctx: Context<VerifyBalance>) -> Result<()> {
        instructions::btc_commitment::verify_balance(ctx)
    }

    pub fn update_commitment(
        ctx: Context<UpdateCommitment>,
        new_amount: u64,
        new_ecdsa_proof: Vec<u8>,
        new_public_key: Vec<u8>,
    ) -> Result<()> {
        instructions::btc_commitment::update_commitment(ctx, new_amount, new_ecdsa_proof, new_public_key)
    }

    // Oracle instructions
    pub fn initialize_oracle(
        ctx: Context<InitializeOracle>,
        btc_usd_feed: Pubkey,
    ) -> Result<()> {
        instructions::oracle::InitializeOracle::process(ctx, btc_usd_feed)
    }

    pub fn update_btc_price(
        ctx: Context<UpdateBTCPrice>,
        price: u64,
        round_id: u64,
        timestamp: i64,
    ) -> Result<()> {
        instructions::oracle::UpdateBTCPrice::process(ctx, price, round_id, timestamp)
    }

    pub fn verify_btc_balance(
        ctx: Context<VerifyBTCBalance>,
        btc_address: String,
        expected_balance: u64,
        ecdsa_proof: Vec<u8>,
    ) -> Result<()> {
        instructions::oracle::VerifyBTCBalance::process(ctx, btc_address, expected_balance, ecdsa_proof)
    }

    // Staking instructions
    pub fn initialize_staking_pool(ctx: Context<InitializeStakingPool>) -> Result<()> {
        instructions::staking::initialize_staking_pool(ctx)
    }

    pub fn stake_protocol_assets(
        ctx: Context<StakeProtocolAssets>,
        total_treasury_usd: u64,
    ) -> Result<()> {
        instructions::staking::stake_protocol_assets(ctx, total_treasury_usd)
    }

    pub fn rebalance_allocations(ctx: Context<RebalanceAllocations>) -> Result<()> {
        instructions::staking::rebalance_allocations(ctx)
    }

    pub fn add_sol_validator(
        ctx: Context<AddValidator>,
        address: String,
        commission: u16,
        performance_score: u16,
    ) -> Result<()> {
        instructions::staking::add_sol_validator(ctx, address, commission, performance_score)
    }

    pub fn add_eth_validator(
        ctx: Context<AddValidator>,
        address: String,
        commission: u16,
        performance_score: u16,
    ) -> Result<()> {
        instructions::staking::add_eth_validator(ctx, address, commission, performance_score)
    }

    pub fn update_atom_config(
        ctx: Context<AddValidator>,
        everstake_validator: String,
        osmosis_validator: String,
    ) -> Result<()> {
        instructions::staking::update_atom_config(ctx, everstake_validator, osmosis_validator)
    }

    // Reward instructions
    pub fn calculate_rewards(
        ctx: Context<CalculateRewards>,
        total_staking_rewards: u64,
        total_btc_commitments: u64,
    ) -> Result<()> {
        instructions::rewards::calculate_rewards(ctx, total_staking_rewards, total_btc_commitments)
    }

    pub fn distribute_rewards(ctx: Context<DistributeRewards>) -> Result<()> {
        instructions::rewards::distribute_rewards(ctx)
    }

    pub fn claim_rewards(
        ctx: Context<ClaimRewards>,
        payment_type: PaymentType,
    ) -> Result<()> {
        instructions::rewards::claim_rewards(ctx, payment_type)
    }

    pub fn update_reward_rates(
        ctx: Context<UpdateRewardRates>,
        new_user_share_bps: u16,
    ) -> Result<()> {
        instructions::rewards::update_reward_rates(ctx, new_user_share_bps)
    }

    // State channel instructions
    pub fn initialize_state_channel(
        ctx: Context<InitializeStateChannel>,
        channel_id: [u8; 32],
        participants: Vec<Pubkey>,
        timeout_seconds: i64,
    ) -> Result<()> {
        instructions::state_channel::initialize_state_channel(ctx, channel_id, participants, timeout_seconds)
    }

    pub fn update_state_channel(
        ctx: Context<UpdateStateChannel>,
        update: StateChannelUpdate,
        signatures: Vec<Vec<u8>>,
    ) -> Result<()> {
        instructions::state_channel::update_state_channel(ctx, update, signatures)
    }

    pub fn settle_state_channel(
        ctx: Context<SettleStateChannel>,
        final_calculations: Vec<RewardCalculation>,
    ) -> Result<()> {
        instructions::state_channel::settle_state_channel(ctx, final_calculations)
    }

    pub fn challenge_state_channel(
        ctx: Context<ChallengeStateChannel>,
        disputed_state_hash: [u8; 32],
        evidence: Vec<u8>,
    ) -> Result<()> {
        instructions::state_channel::challenge_state_channel(ctx, disputed_state_hash, evidence)
    }

    // Multisig instructions
    pub fn initialize_multisig_wallet(
        ctx: Context<InitializeMultisigWallet>,
        signers: Vec<SignerInfo>,
        hsm_enabled: bool,
    ) -> Result<()> {
        instructions::multisig::initialize_multisig_wallet(ctx, signers, hsm_enabled)
    }

    pub fn propose_multisig_transaction(
        ctx: Context<ProposeMultisigTransaction>,
        transaction_type: TransactionType,
        priority: TransactionPriority,
        transaction_data: Vec<u8>,
    ) -> Result<()> {
        instructions::multisig::propose_transaction(ctx, transaction_type, priority, transaction_data)
    }

    pub fn sign_multisig_transaction(
        ctx: Context<SignMultisigTransaction>,
        signature_data: [u8; 64],
        hsm_signature: Option<Vec<u8>>,
        signature_type: SignatureType,
    ) -> Result<()> {
        instructions::multisig::sign_transaction(ctx, signature_data, hsm_signature, signature_type)
    }

    pub fn execute_multisig_transaction(
        ctx: Context<ExecuteMultisigTransaction>,
    ) -> Result<()> {
        instructions::multisig::execute_transaction(ctx)
    }

    pub fn rotate_multisig_keys(
        ctx: Context<RotateMultisigKeys>,
        new_signers: Vec<SignerInfo>,
    ) -> Result<()> {
        instructions::multisig::rotate_keys(ctx, new_signers)
    }

    pub fn activate_emergency_mode(
        ctx: Context<EmergencyAction>,
    ) -> Result<()> {
        instructions::multisig::activate_emergency_mode(ctx)
    }

    pub fn deactivate_emergency_mode(
        ctx: Context<EmergencyAction>,
    ) -> Result<()> {
        instructions::multisig::deactivate_emergency_mode(ctx)
    }

    // Payment system instructions
    pub fn initialize_payment_system(
        ctx: Context<InitializePaymentSystem>,
        lightning_config: LightningConfig,
        usdc_config: UsdcConfig,
    ) -> Result<()> {
        instructions::payment::initialize_payment_system(ctx, lightning_config, usdc_config)
    }

    pub fn initialize_user_preferences(
        ctx: Context<InitializeUserPreferences>,
        default_method: PaymentMethod,
    ) -> Result<()> {
        instructions::payment::initialize_user_preferences(ctx, default_method)
    }

    pub fn create_payment_request(
        ctx: Context<CreatePaymentRequest>,
        method: Option<PaymentMethod>,
        amount: u64,
        destination: String,
    ) -> Result<()> {
        instructions::payment::create_payment_request(ctx, method, amount, destination)
    }

    pub fn process_payment(
        ctx: Context<ProcessPayment>,
        payment_id: u64,
    ) -> Result<()> {
        instructions::payment::process_payment(ctx, payment_id)
    }

    pub fn approve_payment(
        ctx: Context<ApprovePayment>,
        payment_id: u64,
    ) -> Result<()> {
        instructions::payment::approve_payment(ctx, payment_id)
    }

    pub fn complete_payment(
        ctx: Context<ProcessPayment>,
        payment_id: u64,
        success: bool,
        failure_reason: Option<String>,
    ) -> Result<()> {
        instructions::payment::complete_payment(ctx, payment_id, success, failure_reason)
    }

    pub fn cancel_payment(
        ctx: Context<CreatePaymentRequest>,
        payment_id: u64,
    ) -> Result<()> {
        instructions::payment::cancel_payment(ctx, payment_id)
    }

    pub fn update_user_preferences(
        ctx: Context<UpdateUserPreferences>,
        default_method: Option<PaymentMethod>,
        lightning_address: Option<String>,
        usdc_address: Option<Pubkey>,
        reinvestment_config: Option<ReinvestmentConfig>,
    ) -> Result<()> {
        instructions::payment::update_user_preferences(ctx, default_method, lightning_address, usdc_address, reinvestment_config)
    }

    pub fn process_reinvestment(
        ctx: Context<ProcessReinvestment>,
    ) -> Result<()> {
        instructions::payment::process_reinvestment(ctx)
    }

    pub fn set_emergency_pause(
        ctx: Context<UpdatePaymentConfig>,
        paused: bool,
    ) -> Result<()> {
        instructions::payment::set_emergency_pause(ctx, paused)
    }
}