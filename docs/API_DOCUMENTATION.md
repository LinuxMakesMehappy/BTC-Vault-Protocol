# Vault Protocol API Documentation

## Overview

The Vault Protocol provides a comprehensive API for non-custodial Bitcoin commitment and reward distribution. This documentation covers all program instructions, their parameters, and expected behaviors.

## Table of Contents

1. [BTC Commitment Instructions](#btc-commitment-instructions)
2. [Staking Instructions](#staking-instructions)
3. [Reward Instructions](#reward-instructions)
4. [Payment Instructions](#payment-instructions)
5. [Multisig Instructions](#multisig-instructions)
6. [KYC Instructions](#kyc-instructions)
7. [Authentication Instructions](#authentication-instructions)
8. [Treasury Instructions](#treasury-instructions)
9. [State Channel Instructions](#state-channel-instructions)
10. [Oracle Instructions](#oracle-instructions)
11. [Security Monitoring Instructions](#security-monitoring-instructions)
12. [Error Codes](#error-codes)

## BTC Commitment Instructions

### commit_btc

Commits Bitcoin to the protocol without transferring custody.

**Parameters:**
- `amount: u64` - Amount of BTC to commit (in satoshis)
- `btc_address: String` - User's Bitcoin address
- `ecdsa_proof: Vec<u8>` - ECDSA proof of ownership

**Accounts:**
- `commitment: Account<BTCCommitment>` - The commitment account to initialize
- `user: Signer` - The user making the commitment
- `oracle_feed: Account<OracleFeed>` - Chainlink oracle feed account

**Returns:**
- `commitment_id: Pubkey` - Unique identifier for the commitment

**Example:**
```rust
let commitment_result = vault_program.commit_btc(
    amount: 100_000_000, // 1 BTC in satoshis
    btc_address: "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh".to_string(),
    ecdsa_proof: proof_bytes,
)?;
```

### verify_balance

Verifies user's Bitcoin balance via Chainlink oracles.

**Parameters:**
- `user: Pubkey` - User's public key
- `btc_address: String` - Bitcoin address to verify

**Accounts:**
- `commitment: Account<BTCCommitment>` - User's commitment account
- `oracle_feed: Account<OracleFeed>` - Oracle feed account
- `user: Signer` - User account

**Returns:**
- `verified: bool` - Whether balance verification succeeded
- `balance: u64` - Current BTC balance in satoshis

### update_commitment

Updates an existing BTC commitment.

**Parameters:**
- `new_amount: u64` - New commitment amount
- `ecdsa_proof: Vec<u8>` - Updated ECDSA proof

**Accounts:**
- `commitment: Account<BTCCommitment>` - Existing commitment account
- `user: Signer` - User account
- `oracle_feed: Account<OracleFeed>` - Oracle feed account

## Staking Instructions

### stake_protocol_assets

Stakes protocol assets according to allocation strategy.

**Parameters:**
- `sol_amount: u64` - Amount of SOL to stake (40% allocation)
- `eth_amount: u64` - Amount of ETH to stake (30% allocation)
- `atom_amount: u64` - Amount of ATOM to stake (30% allocation)

**Accounts:**
- `staking_pool: Account<StakingPool>` - Protocol staking pool
- `treasury: Account<Treasury>` - Protocol treasury
- `authority: Signer` - Protocol authority

### claim_staking_rewards

Claims accumulated staking rewards from validators.

**Parameters:**
- `validator_addresses: Vec<Pubkey>` - List of validator addresses

**Accounts:**
- `staking_pool: Account<StakingPool>` - Staking pool account
- `rewards_pool: Account<RewardPool>` - Reward distribution pool
- `authority: Signer` - Protocol authority

## Reward Instructions

### calculate_rewards

Calculates user rewards based on BTC commitment and staking profits.

**Parameters:**
- `user: Pubkey` - User's public key
- `period_start: i64` - Reward calculation period start
- `period_end: i64` - Reward calculation period end

**Accounts:**
- `commitment: Account<BTCCommitment>` - User's commitment
- `reward_pool: Account<RewardPool>` - Global reward pool
- `user_rewards: Account<UserRewards>` - User's reward account

**Returns:**
- `reward_amount: u64` - Calculated reward amount

### distribute_rewards

Distributes rewards to users based on their commitments.

**Parameters:**
- `distribution_list: Vec<(Pubkey, u64)>` - List of users and reward amounts

**Accounts:**
- `reward_pool: Account<RewardPool>` - Global reward pool
- `treasury: Account<Treasury>` - Protocol treasury
- `authority: Signer` - Protocol authority

## Payment Instructions

### process_btc_payment

Processes BTC reward payments via Lightning Network.

**Parameters:**
- `user: Pubkey` - Recipient user
- `amount: u64` - Payment amount in satoshis
- `lightning_invoice: String` - Lightning Network invoice

**Accounts:**
- `user_rewards: Account<UserRewards>` - User's reward account
- `payment_system: Account<PaymentSystem>` - Payment processing account
- `authority: Signer` - Protocol authority

### process_usdc_payment

Processes USDC reward payments.

**Parameters:**
- `user: Pubkey` - Recipient user
- `amount: u64` - Payment amount in USDC (6 decimals)
- `usdc_address: Pubkey` - USDC token account

**Accounts:**
- `user_rewards: Account<UserRewards>` - User's reward account
- `usdc_mint: Account<Mint>` - USDC token mint
- `user_usdc_account: Account<TokenAccount>` - User's USDC account
- `protocol_usdc_account: Account<TokenAccount>` - Protocol's USDC account

### set_auto_reinvest

Configures automatic reward reinvestment.

**Parameters:**
- `enabled: bool` - Whether to enable auto-reinvestment
- `percentage: u8` - Percentage to reinvest (0-100)

**Accounts:**
- `user_rewards: Account<UserRewards>` - User's reward account
- `user: Signer` - User account

## Multisig Instructions

### create_multisig_wallet

Creates a new multisig wallet with HSM integration.

**Parameters:**
- `owners: Vec<Pubkey>` - List of owner public keys
- `threshold: u8` - Required signature threshold (2 for 2-of-3)
- `hsm_keys: Vec<YubicoHSMKey>` - HSM key identifiers

**Accounts:**
- `multisig_wallet: Account<MultisigWallet>` - Multisig wallet account
- `authority: Signer` - Creating authority

### propose_transaction

Proposes a new multisig transaction.

**Parameters:**
- `transaction_data: Vec<u8>` - Serialized transaction data
- `description: String` - Transaction description

**Accounts:**
- `multisig_wallet: Account<MultisigWallet>` - Multisig wallet
- `proposer: Signer` - Transaction proposer
- `pending_tx: Account<PendingTransaction>` - Pending transaction account

**Returns:**
- `transaction_id: u32` - Unique transaction identifier

### sign_transaction

Signs a pending multisig transaction.

**Parameters:**
- `transaction_id: u32` - Transaction to sign
- `signature: Vec<u8>` - HSM-generated signature

**Accounts:**
- `multisig_wallet: Account<MultisigWallet>` - Multisig wallet
- `pending_tx: Account<PendingTransaction>` - Pending transaction
- `signer: Signer` - Transaction signer

### execute_transaction

Executes a fully signed multisig transaction.

**Parameters:**
- `transaction_id: u32` - Transaction to execute

**Accounts:**
- `multisig_wallet: Account<MultisigWallet>` - Multisig wallet
- `pending_tx: Account<PendingTransaction>` - Pending transaction
- `executor: Signer` - Transaction executor

## KYC Instructions

### submit_kyc_documents

Submits KYC documents for verification.

**Parameters:**
- `documents: Vec<u8>` - Encrypted document data
- `document_types: Vec<DocumentType>` - Types of submitted documents

**Accounts:**
- `kyc_status: Account<KYCStatus>` - User's KYC status account
- `user: Signer` - User account

### verify_kyc_status

Verifies KYC status via Chainalysis integration.

**Parameters:**
- `user: Pubkey` - User to verify
- `chainalysis_data: Vec<u8>` - Chainalysis verification data

**Accounts:**
- `kyc_status: Account<KYCStatus>` - User's KYC status
- `authority: Signer` - KYC authority

### update_compliance_limits

Updates user's compliance limits based on KYC tier.

**Parameters:**
- `user: Pubkey` - User to update
- `new_limit: u64` - New commitment limit

**Accounts:**
- `kyc_status: Account<KYCStatus>` - User's KYC status
- `commitment: Account<BTCCommitment>` - User's commitment
- `authority: Signer` - Compliance authority

## Authentication Instructions

### initialize_2fa

Initializes two-factor authentication for a user.

**Parameters:**
- `totp_secret: Vec<u8>` - TOTP secret key
- `backup_codes: Vec<String>` - Backup recovery codes

**Accounts:**
- `auth_account: Account<AuthAccount>` - User's authentication account
- `user: Signer` - User account

### verify_2fa

Verifies two-factor authentication.

**Parameters:**
- `totp_code: String` - TOTP code from authenticator
- `operation_type: OperationType` - Type of operation being authenticated

**Accounts:**
- `auth_account: Account<AuthAccount>` - User's authentication account
- `user: Signer` - User account

**Returns:**
- `session_token: [u8; 32]` - Authentication session token

### setup_passkey

Sets up WebAuthn passkey authentication.

**Parameters:**
- `credential_id: Vec<u8>` - WebAuthn credential ID
- `public_key: Vec<u8>` - WebAuthn public key

**Accounts:**
- `auth_account: Account<AuthAccount>` - User's authentication account
- `user: Signer` - User account

## Treasury Instructions

### process_deposit

Processes biweekly treasury deposits.

**Parameters:**
- `amount_usd: u64` - Deposit amount in USD (typically $50)
- `asset_allocation: AssetAllocation` - SOL/ETH/ATOM allocation

**Accounts:**
- `treasury: Account<Treasury>` - Protocol treasury
- `deposit_account: Account<DepositAccount>` - Deposit processing account
- `authority: Signer` - Treasury authority

### rebalance_treasury

Rebalances treasury assets to maintain target allocations.

**Parameters:**
- `target_allocation: AssetAllocation` - Target allocation percentages

**Accounts:**
- `treasury: Account<Treasury>` - Protocol treasury
- `staking_pool: Account<StakingPool>` - Staking pool
- `authority: Signer` - Treasury authority

## State Channel Instructions

### create_state_channel

Creates a new state channel for off-chain computation.

**Parameters:**
- `participants: Vec<Pubkey>` - Channel participants
- `initial_state: Vec<u8>` - Initial channel state
- `timeout_blocks: u64` - Channel timeout in blocks

**Accounts:**
- `state_channel: Account<StateChannel>` - State channel account
- `creator: Signer` - Channel creator

### update_state_channel

Updates state channel with new state.

**Parameters:**
- `channel_id: [u8; 32]` - Channel identifier
- `new_state: Vec<u8>` - New channel state
- `signatures: Vec<Vec<u8>>` - Participant signatures

**Accounts:**
- `state_channel: Account<StateChannel>` - State channel account
- `participant: Signer` - Channel participant

### settle_state_channel

Settles state channel on-chain.

**Parameters:**
- `channel_id: [u8; 32]` - Channel identifier
- `final_state: Vec<u8>` - Final channel state

**Accounts:**
- `state_channel: Account<StateChannel>` - State channel account
- `participant: Signer` - Channel participant

## Oracle Instructions

### update_price_feed

Updates Chainlink price feed data.

**Parameters:**
- `feed_id: Pubkey` - Price feed identifier
- `price: u64` - New price data
- `timestamp: i64` - Price timestamp

**Accounts:**
- `oracle_feed: Account<OracleFeed>` - Oracle feed account
- `oracle_authority: Signer` - Oracle authority

### verify_utxo

Verifies Bitcoin UTXO via oracle.

**Parameters:**
- `btc_address: String` - Bitcoin address to verify
- `expected_balance: u64` - Expected balance in satoshis

**Accounts:**
- `oracle_feed: Account<OracleFeed>` - Oracle feed account
- `verification_account: Account<UTXOVerification>` - Verification account

## Security Monitoring Instructions

### log_security_event

Logs a security event for monitoring.

**Parameters:**
- `event_type: SecurityEventType` - Type of security event
- `user: Option<Pubkey>` - Associated user (if applicable)
- `details: String` - Event details

**Accounts:**
- `security_log: Account<SecurityLog>` - Security log account
- `authority: Signer` - Logging authority

### detect_anomaly

Detects anomalous behavior patterns.

**Parameters:**
- `user: Pubkey` - User to analyze
- `behavior_data: Vec<u8>` - Behavior pattern data

**Accounts:**
- `security_monitor: Account<SecurityMonitor>` - Security monitoring account
- `user_profile: Account<UserProfile>` - User behavior profile

## Error Codes

### VaultError

```rust
#[error_code]
pub enum VaultError {
    #[msg("Invalid BTC commitment amount")]
    InvalidCommitmentAmount = 6000,
    
    #[msg("ECDSA proof verification failed")]
    InvalidECDSAProof = 6001,
    
    #[msg("Oracle verification failed")]
    OracleVerificationFailed = 6002,
    
    #[msg("Insufficient staking rewards")]
    InsufficientStakingRewards = 6003,
    
    #[msg("Payment processing failed")]
    PaymentProcessingFailed = 6004,
    
    #[msg("Multisig threshold not met")]
    MultisigThresholdNotMet = 6005,
    
    #[msg("KYC verification required")]
    KYCVerificationRequired = 6006,
    
    #[msg("Two-factor authentication required")]
    TwoFactorAuthRequired = 6007,
    
    #[msg("Treasury balance insufficient")]
    TreasuryBalanceInsufficient = 6008,
    
    #[msg("State channel dispute")]
    StateChannelDispute = 6009,
    
    #[msg("Security violation detected")]
    SecurityViolation = 6010,
    
    #[msg("User not authorized")]
    Unauthorized = 6011,
    
    #[msg("Account frozen due to compliance")]
    AccountFrozen = 6012,
    
    #[msg("Invalid session token")]
    InvalidSessionToken = 6013,
    
    #[msg("Rate limit exceeded")]
    RateLimitExceeded = 6014,
    
    #[msg("Cross-chain operation failed")]
    CrossChainOperationFailed = 6015,
}
```

## Rate Limits

- BTC commitment: 1 per hour per user
- Reward claims: 1 per day per user
- KYC submissions: 3 per day per user
- Authentication attempts: 5 per minute per user
- Oracle queries: 1 per minute per commitment

## API Versioning

Current API version: `v1.0.0`

Version compatibility:
- Major version changes: Breaking changes requiring client updates
- Minor version changes: New features, backward compatible
- Patch version changes: Bug fixes, fully compatible

## Support

For API support and questions:
- Documentation: [docs.vaultprotocol.com](https://docs.vaultprotocol.com)
- Developer Discord: [discord.gg/vaultprotocol](https://discord.gg/vaultprotocol)
- GitHub Issues: [github.com/vaultprotocol/vault](https://github.com/vaultprotocol/vault)