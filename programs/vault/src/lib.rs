use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod errors;
pub mod crypto;
pub mod monitoring;
pub mod security;
pub mod traits;

use instructions::btc_commitment::*;
use instructions::oracle::*;
use instructions::staking::*;
use instructions::rewards::*;
use instructions::state_channel::*;
use instructions::enhanced_state_channel::*;
use instructions::multisig::*;
use instructions::payment::*;
use instructions::kyc::*;
use instructions::authentication::*;
use instructions::treasury_management::*;
use instructions::security_monitoring::*;
use crate::traits::PaymentType;
use crate::state::{StateChannelUpdate, SignerInfo, TransactionType, TransactionPriority, SignatureType, PaymentMethod, LightningConfig, UsdcConfig, ReinvestmentConfig};
use crate::state::rewards::RewardCalculation;
use crate::state::kyc_compliance::{KYCStatus, ComplianceRegion, KYCVerification, AMLScreening};
use crate::state::authentication::{AuthMethod, SessionStatus, SecurityEventType};
use crate::state::security_monitoring::{SecurityEventType as MonitoringEventType, SecurityLevel, AlertStatus};

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

    // KYC and compliance instructions
    pub fn initialize_compliance(
        ctx: Context<InitializeCompliance>,
        chainalysis_api_key: String,
    ) -> Result<()> {
        instructions::kyc::initialize_compliance(ctx, chainalysis_api_key)
    }

    pub fn initialize_user_compliance(
        ctx: Context<InitializeUserCompliance>,
        compliance_region: ComplianceRegion,
    ) -> Result<()> {
        instructions::kyc::initialize_user_compliance(ctx, compliance_region)
    }

    pub fn update_kyc_status(
        ctx: Context<UpdateKYCStatus>,
        new_status: KYCStatus,
        verification: Option<KYCVerification>,
    ) -> Result<()> {
        instructions::kyc::update_kyc_status(ctx, new_status, verification)
    }

    pub fn perform_aml_screening(
        ctx: Context<PerformAMLScreening>,
        screening_data: crate::instructions::kyc::AMLScreeningData,
    ) -> Result<()> {
        instructions::kyc::perform_aml_screening(ctx, screening_data)
    }

    pub fn validate_transaction(
        ctx: Context<ValidateTransaction>,
        transaction_type: crate::instructions::kyc::TransactionValidationType,
        amount: u64,
        destination: Option<String>,
    ) -> Result<()> {
        instructions::kyc::validate_transaction(ctx, transaction_type, amount, destination)
    }

    pub fn resolve_alert(
        ctx: Context<ResolveAlert>,
        alert_id: String,
        resolution_notes: String,
    ) -> Result<()> {
        instructions::kyc::resolve_alert(ctx, alert_id, resolution_notes)
    }

    pub fn freeze_account(
        ctx: Context<FreezeAccount>,
        reason: String,
    ) -> Result<()> {
        instructions::kyc::freeze_account(ctx, reason)
    }

    pub fn unfreeze_account(
        ctx: Context<FreezeAccount>,
    ) -> Result<()> {
        instructions::kyc::unfreeze_account(ctx)
    }

    pub fn update_compliance_config(
        ctx: Context<UpdateComplianceConfig>,
        screening_enabled: Option<bool>,
        auto_freeze_enabled: Option<bool>,
        manual_review_threshold: Option<u64>,
        enhanced_dd_threshold: Option<u64>,
    ) -> Result<()> {
        instructions::kyc::update_compliance_config(ctx, screening_enabled, auto_freeze_enabled, manual_review_threshold, enhanced_dd_threshold)
    }

    pub fn perform_compliance_review(
        ctx: Context<PerformAMLScreening>,
    ) -> Result<()> {
        instructions::kyc::perform_compliance_review(ctx)
    }

    pub fn get_compliance_summary(
        ctx: Context<ValidateTransaction>,
    ) -> Result<()> {
        instructions::kyc::get_compliance_summary(ctx)
    }

    // Authentication and 2FA instructions
    pub fn initialize_auth_config(
        ctx: Context<InitializeAuthConfig>,
    ) -> Result<()> {
        instructions::authentication::initialize_auth_config(ctx)
    }

    pub fn initialize_user_auth(
        ctx: Context<InitializeAuth>,
    ) -> Result<()> {
        instructions::authentication::initialize_user_auth(ctx)
    }

    pub fn add_auth_factor(
        ctx: Context<AddAuthFactor>,
        method: AuthMethod,
        identifier: String,
        secret_hash: [u8; 32],
        backup_codes: Vec<String>,
    ) -> Result<()> {
        instructions::authentication::add_auth_factor(ctx, method, identifier, secret_hash, backup_codes)
    }

    pub fn verify_auth_factor(
        ctx: Context<VerifyAuthFactor>,
        method: AuthMethod,
        identifier: String,
        provided_code: String,
    ) -> Result<()> {
        instructions::authentication::verify_auth_factor(ctx, method, identifier, provided_code)
    }

    pub fn create_session(
        ctx: Context<CreateSession>,
        device_id: String,
        ip_address: String,
        user_agent: String,
        auth_methods: Vec<AuthMethod>,
    ) -> Result<()> {
        instructions::authentication::create_session(ctx, device_id, ip_address, user_agent, auth_methods)
    }

    pub fn validate_session(
        ctx: Context<ValidateSession>,
        session_id: String,
    ) -> Result<()> {
        instructions::authentication::validate_session(ctx, session_id)
    }

    pub fn revoke_session(
        ctx: Context<RevokeSession>,
        session_id: String,
    ) -> Result<()> {
        instructions::authentication::revoke_session(ctx, session_id)
    }

    pub fn lock_account(
        ctx: Context<LockAccount>,
        reason: String,
    ) -> Result<()> {
        instructions::authentication::lock_account(ctx, reason)
    }

    pub fn unlock_account(
        ctx: Context<LockAccount>,
    ) -> Result<()> {
        instructions::authentication::unlock_account(ctx)
    }

    pub fn update_auth_config(
        ctx: Context<UpdateAuthConfig>,
        require_2fa_globally: Option<bool>,
        session_timeout_min: Option<u32>,
        session_timeout_max: Option<u32>,
        max_failed_attempts: Option<u32>,
        lockout_duration: Option<i64>,
    ) -> Result<()> {
        instructions::authentication::update_auth_config(ctx, require_2fa_globally, session_timeout_min, session_timeout_max, max_failed_attempts, lockout_duration)
    }

    pub fn check_2fa_requirement(
        ctx: Context<ValidateSession>,
        operation_type: String,
        amount: Option<u64>,
    ) -> Result<()> {
        instructions::authentication::check_2fa_requirement(ctx, operation_type, amount)
    }

    pub fn get_security_status(
        ctx: Context<ValidateSession>,
    ) -> Result<()> {
        instructions::authentication::get_security_status(ctx)
    }

    pub fn generate_backup_codes(
        ctx: Context<AddAuthFactor>,
    ) -> Result<Vec<String>> {
        instructions::authentication::generate_backup_codes(ctx)
    }

    pub fn verify_backup_code(
        ctx: Context<VerifyAuthFactor>,
        backup_code: String,
    ) -> Result<()> {
        instructions::authentication::verify_backup_code(ctx, backup_code)
    }

    pub fn update_security_settings(
        ctx: Context<AddAuthFactor>,
        require_2fa_for_all: Option<bool>,
        require_2fa_for_payments: Option<bool>,
        require_2fa_for_high_value: Option<bool>,
        session_timeout: Option<u32>,
        max_concurrent_sessions: Option<u8>,
        auto_lock_on_suspicious: Option<bool>,
    ) -> Result<()> {
        instructions::authentication::update_security_settings(ctx, require_2fa_for_all, require_2fa_for_payments, require_2fa_for_high_value, session_timeout, max_concurrent_sessions, auto_lock_on_suspicious)
    }

    // Treasury Management instructions
    pub fn initialize_treasury_vault(
        ctx: Context<InitializeTreasuryVault>,
    ) -> Result<()> {
        let bump = ctx.bumps.treasury_vault;
        instructions::treasury_management::InitializeTreasuryVault::process(ctx, bump)
    }

    pub fn add_yield_strategy(
        ctx: Context<AddYieldStrategy>,
        strategy_id: u64,
        name: String,
        protocol: String,
        strategy_type: crate::state::treasury_management::StrategyType,
        assets: Vec<Pubkey>,
        allocated_amount: u64,
        expected_apy: u16,
        risk_level: u8,
        parameters: Vec<u8>,
    ) -> Result<()> {
        instructions::treasury_management::AddYieldStrategy::process(ctx, strategy_id, name, protocol, strategy_type, assets, allocated_amount, expected_apy, risk_level, parameters)
    }

    pub fn add_liquidity_pool(
        ctx: Context<AddLiquidityPool>,
        pool_id: Pubkey,
        dex_protocol: String,
        liquidity_amount: u64,
    ) -> Result<()> {
        instructions::treasury_management::AddLiquidityPool::process(ctx, pool_id, dex_protocol, liquidity_amount)
    }

    pub fn execute_advanced_rebalancing(
        ctx: Context<ExecuteAdvancedRebalancing>,
        amount: u64,
        strategy_id: Option<u64>,
    ) -> Result<()> {
        instructions::treasury_management::ExecuteAdvancedRebalancing::process(ctx, amount, strategy_id)
    }

    pub fn update_treasury_performance(
        ctx: Context<UpdateTreasuryPerformance>,
        new_metrics: crate::state::treasury_management::PerformanceMetrics,
    ) -> Result<()> {
        instructions::treasury_management::UpdateTreasuryPerformance::process(ctx, new_metrics)
    }

    pub fn create_treasury_proposal(
        ctx: Context<CreateTreasuryProposal>,
        proposal_id: u64,
        title: String,
        description: String,
        proposal_type: crate::state::treasury_management::ProposalType,
        parameters: Vec<u8>,
        voting_duration: i64,
        quorum_threshold: u16,
        approval_threshold: u16,
    ) -> Result<()> {
        let bump = ctx.bumps.treasury_proposal;
        instructions::treasury_management::CreateTreasuryProposal::process(ctx, proposal_id, title, description, proposal_type, parameters, voting_duration, quorum_threshold, approval_threshold, bump)
    }

    pub fn vote_on_treasury_proposal(
        ctx: Context<VoteOnTreasuryProposal>,
        vote_for: bool,
        voting_power: u64,
    ) -> Result<()> {
        instructions::treasury_management::VoteOnTreasuryProposal::process(ctx, vote_for, voting_power)
    }

    pub fn emergency_pause_treasury(
        ctx: Context<EmergencyPauseTreasury>,
    ) -> Result<()> {
        instructions::treasury_management::EmergencyPauseTreasury::process(ctx)
    }

    pub fn update_risk_parameters(
        ctx: Context<UpdateRiskParameters>,
        new_risk_params: crate::state::treasury_management::RiskParameters,
    ) -> Result<()> {
        instructions::treasury_management::UpdateRiskParameters::process(ctx, new_risk_params)
    }

    // Enhanced State Channel instructions
    pub fn initialize_enhanced_state_channel(
        ctx: Context<InitializeEnhancedStateChannel>,
        channel_id: [u8; 32],
        participants: Vec<crate::state::enhanced_state_channel::ChannelParticipant>,
        config: crate::state::enhanced_state_channel::ChannelConfig,
    ) -> Result<()> {
        instructions::enhanced_state_channel::InitializeEnhancedStateChannel::process(ctx, channel_id, participants, config, ctx.bumps.enhanced_channel)
    }

    pub fn activate_enhanced_channel(
        ctx: Context<ActivateEnhancedChannel>,
    ) -> Result<()> {
        instructions::enhanced_state_channel::ActivateEnhancedChannel::process(ctx)
    }

    pub fn process_hft_operation(
        ctx: Context<ProcessHFTOperation>,
        operation: crate::state::enhanced_state_channel::HFTOperation,
    ) -> Result<()> {
        instructions::enhanced_state_channel::ProcessHFTOperation::process(ctx, operation)
    }

    pub fn process_micro_transaction(
        ctx: Context<ProcessMicroTransaction>,
        transaction: crate::state::enhanced_state_channel::MicroTransaction,
    ) -> Result<()> {
        instructions::enhanced_state_channel::ProcessMicroTransaction::process(ctx, transaction)
    }

    pub fn add_pending_operation(
        ctx: Context<AddPendingOperation>,
        operation: crate::state::enhanced_state_channel::PendingOperation,
    ) -> Result<()> {
        instructions::enhanced_state_channel::AddPendingOperation::process(ctx, operation)
    }

    pub fn confirm_operation(
        ctx: Context<ConfirmOperation>,
        operation_id: u64,
        signature: [u8; 64],
    ) -> Result<()> {
        instructions::enhanced_state_channel::ConfirmOperation::process(ctx, operation_id, signature)
    }

    pub fn initiate_dispute(
        ctx: Context<InitiateDispute>,
        disputed_state: [u8; 32],
        evidence: Vec<u8>,
        dispute_type: crate::state::enhanced_state_channel::DisputeType,
    ) -> Result<()> {
        instructions::enhanced_state_channel::InitiateDispute::process(ctx, disputed_state, evidence, dispute_type)
    }

    pub fn resolve_dispute(
        ctx: Context<ResolveDispute>,
        resolution: crate::state::enhanced_state_channel::DisputeResolution,
    ) -> Result<()> {
        instructions::enhanced_state_channel::ResolveDispute::process(ctx, resolution)
    }

    pub fn close_enhanced_channel(
        ctx: Context<CloseEnhancedChannel>,
    ) -> Result<()> {
        instructions::enhanced_state_channel::CloseEnhancedChannel::process(ctx)
    }

    pub fn batch_process_operations(
        ctx: Context<BatchProcessOperations>,
        operations: Vec<crate::state::enhanced_state_channel::HFTOperation>,
    ) -> Result<()> {
        instructions::enhanced_state_channel::BatchProcessOperations::process(ctx, operations)
    }

    // Security monitoring instructions
    pub fn initialize_security_monitor(
        ctx: Context<InitializeSecurityMonitor>,
    ) -> Result<()> {
        instructions::security_monitoring::initialize_security_monitor(ctx)
    }

    pub fn log_security_event(
        ctx: Context<LogSecurityEvent>,
        event_type: MonitoringEventType,
        user: Option<Pubkey>,
        details: String,
        ip_address: Option<String>,
        user_agent: Option<String>,
        device_id: Option<String>,
        session_id: Option<String>,
        transaction_id: Option<String>,
        amount: Option<u64>,
        metadata: std::collections::HashMap<String, String>,
    ) -> Result<()> {
        instructions::security_monitoring::log_security_event(
            ctx, event_type, user, details, ip_address, user_agent, 
            device_id, session_id, transaction_id, amount, metadata
        )
    }

    pub fn create_audit_trail(
        ctx: Context<CreateAuditTrail>,
        user: Option<Pubkey>,
        action: String,
        resource: String,
        success: bool,
        ip_address: Option<String>,
        user_agent: Option<String>,
        session_id: Option<String>,
        before_state: Option<String>,
        after_state: Option<String>,
        error_message: Option<String>,
        compliance_relevant: bool,
    ) -> Result<()> {
        instructions::security_monitoring::create_audit_trail(
            ctx, user, action, resource, success, ip_address, user_agent,
            session_id, before_state, after_state, error_message, compliance_relevant
        )
    }

    pub fn resolve_security_alert(
        ctx: Context<ManageSecurityAlert>,
        alert_id: u64,
        false_positive: bool,
        resolution_notes: String,
    ) -> Result<()> {
        instructions::security_monitoring::resolve_security_alert(ctx, alert_id, false_positive, resolution_notes)
    }

    pub fn assign_security_alert(
        ctx: Context<ManageSecurityAlert>,
        alert_id: u64,
        officer: Pubkey,
    ) -> Result<()> {
        instructions::security_monitoring::assign_security_alert(ctx, alert_id, officer)
    }

    pub fn add_anomaly_rule(
        ctx: Context<UpdateAnomalyRules>,
        name: String,
        description: String,
        event_types: Vec<MonitoringEventType>,
        threshold_value: f64,
        time_window_minutes: u32,
        severity: SecurityLevel,
        auto_block: bool,
    ) -> Result<()> {
        instructions::security_monitoring::add_anomaly_rule(
            ctx, name, description, event_types, threshold_value, 
            time_window_minutes, severity, auto_block
        )
    }

    pub fn update_security_config(
        ctx: Context<UpdateAnomalyRules>,
        retention_days: Option<u32>,
        max_events_per_user: Option<u32>,
        auto_block_enabled: Option<bool>,
        notification_webhook: Option<String>,
    ) -> Result<()> {
        instructions::security_monitoring::update_security_config(
            ctx, retention_days, max_events_per_user, auto_block_enabled, notification_webhook
        )
    }
}
