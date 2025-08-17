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
    
    #[msg("CVE detected in dependency")]
    CveDetected,
    
    #[msg("WebAssembly execution failed")]
    WasmExecutionFailed,
    
    #[msg("Formal verification failed")]
    VerificationFailed,
    
    #[msg("Cryptographic operation failed")]
    CryptographicError,
    
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
    
    // KYC and compliance errors
    #[msg("KYC verification already in progress")]
    KYCAlreadyInProgress,
    
    #[msg("KYC already approved for this tier")]
    KYCAlreadyApproved,
    
    #[msg("Too many documents submitted")]
    TooManyDocuments,
    
    #[msg("Document type already exists")]
    DocumentTypeAlreadyExists,
    
    #[msg("Document not found")]
    DocumentNotFound,
    
    #[msg("Document already verified")]
    DocumentAlreadyVerified,
    
    #[msg("Compliance violation detected")]
    ComplianceViolation,
    
    #[msg("Invalid KYC status for this operation")]
    InvalidKYCStatus,
    
    #[msg("Required document missing for tier")]
    RequiredDocumentMissing,
    
    #[msg("Compliance screening required for this tier")]
    ComplianceScreeningRequired,
    
    #[msg("Unauthorized compliance officer")]
    UnauthorizedComplianceOfficer,
    
    #[msg("Commitment exceeds KYC limit")]
    CommitmentExceedsKYCLimit,
    
    #[msg("Daily transaction limit exceeded")]
    DailyLimitExceeded,
    
    #[msg("Reason text too long")]
    ReasonTooLong,
    
    #[msg("High-risk user restriction")]
    HighRiskUserRestriction,
    
    // Additional KYC and compliance errors
    #[msg("Restricted jurisdiction")]
    RestrictedJurisdiction,
    
    #[msg("AML screening disabled")]
    AMLScreeningDisabled,
    
    #[msg("Account frozen for compliance")]
    AccountFrozen,
    
    #[msg("Payment limit exceeded")]
    PaymentLimitExceeded,
    
    #[msg("Compliance alert queue full")]
    ComplianceAlertQueueFull,
    
    #[msg("Alert not found")]
    AlertNotFound,
    
    #[msg("Review not due")]
    ReviewNotDue,
    
    #[msg("KYC verification required")]
    KYCVerificationRequired,
    
    #[msg("Enhanced due diligence required")]
    EnhancedDueDiligenceRequired,
    
    #[msg("Manual review required")]
    ManualReviewRequired,
    
    // Authentication and 2FA errors
    #[msg("Two-factor authentication required")]
    TwoFactorRequired,
    
    #[msg("Invalid authentication code")]
    InvalidAuthCode,
    
    #[msg("Authentication factor not found")]
    AuthFactorNotFound,
    
    #[msg("Authentication factor already exists")]
    AuthFactorAlreadyExists,
    
    #[msg("Authentication factor is locked")]
    AuthFactorLocked,
    
    #[msg("Too many authentication factors")]
    TooManyAuthFactors,
    
    #[msg("Authentication method not allowed")]
    AuthMethodNotAllowed,
    
    #[msg("Session not found")]
    SessionNotFound,
    
    #[msg("Invalid session")]
    InvalidSession,
    
    #[msg("Account is locked")]
    AccountLocked,
    
    #[msg("Invalid backup code")]
    InvalidBackupCode,
    
    #[msg("Invalid session timeout")]
    InvalidSessionTimeout,
    
    #[msg("Invalid session limit")]
    InvalidSessionLimit,
    
    // Treasury management errors
    #[msg("Treasury is paused")]
    TreasuryPaused,
    
    #[msg("Deposit not due")]
    DepositNotDue,
    
    #[msg("Invalid deposit amount")]
    InvalidDepositAmount,
    
    #[msg("Invalid deposit frequency")]
    InvalidDepositFrequency,
    
    #[msg("Invalid rebalance threshold")]
    InvalidRebalanceThreshold,
    
    #[msg("Invalid withdrawal reason")]
    InvalidWithdrawalReason,
    
    #[msg("Invalid oracle price")]
    InvalidOraclePrice,
    
    #[msg("Math overflow")]
    MathOverflow,
    
    #[msg("Treasury balance insufficient")]
    TreasuryInsufficientBalance,
    
    #[msg("Rebalancing not needed")]
    RebalancingNotNeeded,
    
    #[msg("Asset allocation out of bounds")]
    AssetAllocationOutOfBounds,
    
    #[msg("Treasury not initialized")]
    TreasuryNotInitialized,
    
    #[msg("Invalid asset type")]
    InvalidAssetType,
    
    #[msg("Deposit limit exceeded")]
    DepositLimitExceeded,
    
    #[msg("Treasury emergency mode active")]
    TreasuryEmergencyMode,
    
    // Security monitoring errors
    #[msg("Security monitoring disabled")]
    SecurityMonitoringDisabled,
    
    #[msg("Event log full")]
    EventLogFull,
    
    #[msg("Invalid security level")]
    InvalidSecurityLevel,
    
    #[msg("Anomaly rule not found")]
    AnomalyRuleNotFound,
    
    #[msg("User behavior profile not found")]
    UserBehaviorProfileNotFound,
    
    #[msg("Security event not found")]
    SecurityEventNotFound,
    
    #[msg("Audit trail not found")]
    AuditTrailNotFound,
    
    #[msg("Invalid time window")]
    InvalidTimeWindow,
    
    #[msg("Invalid threshold value")]
    InvalidThresholdValue,
    
    #[msg("Security alert queue full")]
    SecurityAlertQueueFull,
    
    #[msg("Unauthorized security officer")]
    UnauthorizedSecurityOfficer,
    
    // Arithmetic and overflow errors
    #[msg("Arithmetic overflow")]
    ArithmeticOverflow,
    
    #[msg("Division by zero")]
    DivisionByZero,
    
    #[msg("Clock unavailable")]
    ClockUnavailable,
    
    // Liquidity Engine Errors
    #[msg("Liquidity engine is paused")]
    LiquidityEnginePaused,
    #[msg("Insufficient liquidity for instant unstaking")]
    InsufficientLiquidity,
    #[msg("Chain already supported")]
    ChainAlreadySupported,
    #[msg("Pool already exists")]
    PoolAlreadyExists,
    #[msg("Insufficient stake amount")]
    InsufficientStake,
    #[msg("Slippage tolerance exceeded")]
    SlippageExceeded,
    #[msg("Invalid swap route")]
    InvalidSwapRoute,
    #[msg("Bridge transfer failed")]
    BridgeTransferFailed,
    #[msg("Unsupported chain")]
    UnsupportedChain,
    #[msg("Reserve threshold not met")]
    ReserveThresholdNotMet,
}
