use anchor_lang::prelude::*;

/// Core trait for BTC commitment operations
pub trait BTCCommitmentInterface {
    fn commit_btc(amount: u64, btc_address: String, ecdsa_proof: Vec<u8>) -> Result<()>;
    fn verify_balance(user: Pubkey) -> Result<bool>;
    fn update_commitment(user: Pubkey, new_amount: u64) -> Result<()>;
}

/// Core trait for staking operations
pub trait StakingInterface {
    fn stake_protocol_assets(sol_amount: u64, eth_amount: u64, atom_amount: u64) -> Result<()>;
    fn claim_staking_rewards() -> Result<u64>;
    fn rebalance_allocations() -> Result<()>;
    fn handle_slashing(validator: String, amount: u64) -> Result<()>;
}

/// Core trait for reward distribution
pub trait RewardInterface {
    fn calculate_user_rewards(user: Pubkey) -> Result<u64>;
    fn distribute_rewards(payment_type: PaymentType) -> Result<()>;
    fn set_auto_reinvest(user: Pubkey, enabled: bool) -> Result<()>;
}

/// Core trait for multisig operations
pub trait MultisigInterface {
    fn propose_transaction(tx: Vec<u8>) -> Result<u32>;
    fn sign_transaction(tx_id: u32, signature: Vec<u8>) -> Result<()>;
    fn execute_transaction(tx_id: u32) -> Result<()>;
    fn rotate_keys(new_keys: Vec<Pubkey>) -> Result<()>;
}

/// Core trait for KYC operations
pub trait KYCInterface {
    fn verify_user(user: Pubkey, documents: Vec<u8>) -> Result<()>;
    fn check_compliance(user: Pubkey, amount: u64) -> Result<bool>;
    fn update_limits(user: Pubkey, new_limit: u64) -> Result<()>;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum PaymentType {
    BTC,
    USDC,
    AutoReinvest,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum ComplianceTier {
    NonKYC,
    KYCVerified,
}
