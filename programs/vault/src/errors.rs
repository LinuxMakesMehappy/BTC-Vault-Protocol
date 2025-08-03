use anchor_lang::prelude::*;

#[error_code]
pub enum VaultError {
    #[msg("Invalid BTC address format")]
    InvalidBTCAddress,
    
    #[msg("Invalid ECDSA proof")]
    InvalidECDSAProof,
    
    #[msg("Oracle verification failed")]
    OracleVerificationFailed,
    
    #[msg("Insufficient balance")]
    InsufficientBalance,
    
    #[msg("KYC verification required")]
    KYCRequired,
    
    #[msg("Commitment limit exceeded")]
    CommitmentLimitExceeded,
    
    #[msg("Multisig threshold not met")]
    MultisigThresholdNotMet,
    
    #[msg("Transaction already executed")]
    TransactionAlreadyExecuted,
    
    #[msg("Unauthorized signer")]
    UnauthorizedSigner,
    
    #[msg("Invalid allocation percentages")]
    InvalidAllocation,
    
    #[msg("Staking operation failed")]
    StakingFailed,
    
    #[msg("Reward calculation error")]
    RewardCalculationError,
    
    #[msg("Payment processing failed")]
    PaymentFailed,
    
    #[msg("Security violation detected")]
    SecurityViolation,
    
    #[msg("Unauthorized access - owner verification failed")]
    UnauthorizedAccess,
    
    #[msg("Missing required signer")]
    MissingSigner,
    
    #[msg("Oracle price feed unavailable")]
    OraclePriceUnavailable,
}