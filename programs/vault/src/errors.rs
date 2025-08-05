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
    
    #[msg("No validators available for staking")]
    NoValidatorsAvailable,
    
    #[msg("Cross-chain message failed")]
    CrossChainMessageFailed,
    
    #[msg("HSM signature validation failed")]
    HSMValidationFailed,
    
    #[msg("Key rotation required")]
    KeyRotationRequired,
    
    #[msg("Emergency mode not active")]
    EmergencyModeNotActive,
    
    // Payment system errors
    #[msg("Payment system is paused")]
    PaymentSystemPaused,
    
    #[msg("Payment queue is full")]
    PaymentQueueFull,
    
    #[msg("Payment not found")]
    PaymentNotFound,
    
    #[msg("Invalid payment status")]
    InvalidPaymentStatus,
    
    #[msg("Payment amount too small")]
    PaymentAmountTooSmall,
    
    #[msg("Payment amount too large")]
    PaymentAmountTooLarge,
    
    #[msg("Invalid Lightning invoice")]
    InvalidLightningInvoice,
    
    #[msg("Invalid Solana address")]
    InvalidSolanaAddress,
    
    #[msg("Insufficient Lightning capacity")]
    InsufficientLightningCapacity,
    
    #[msg("Insufficient rewards for payment")]
    InsufficientRewards,
    
    #[msg("No payment destination configured")]
    NoPaymentDestination,
    
    #[msg("Missing token account")]
    MissingTokenAccount,
    
    #[msg("Missing token program")]
    MissingTokenProgram,
    
    #[msg("Payment does not require approval")]
    PaymentDoesNotRequireApproval,
    
    #[msg("Payment already completed")]
    PaymentAlreadyCompleted,
    
    #[msg("Payment in progress")]
    PaymentInProgress,
    
    #[msg("Invalid Lightning address")]
    InvalidLightningAddress,
    
    #[msg("Invalid reinvestment percentage")]
    InvalidReinvestmentPercentage,
    
    #[msg("Reinvestment not enabled")]
    ReinvestmentNotEnabled,
    
    #[msg("Insufficient reinvestment amount")]
    InsufficientReinvestmentAmount,
    
    #[msg("Reinvestment too frequent")]
    ReinvestmentTooFrequent,
}