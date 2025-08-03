# Requirements Document

## Introduction

The Vault Protocol is a security-focused, non-custodial, developer-controlled Bitcoin protocol that allows users to commit BTC without transferring custody while earning rewards from protocol-owned staking activities. The protocol verifies user BTC balances via Chainlink oracles every minute and stakes its own assets (SOL 40%, ETH 30%, ATOM 30%) to generate rewards that are shared with users at a 1:2 ratio. The system features a NextJS frontend with multi-language support, hardware wallet integration, and enterprise-grade security measures including HSM-backed multisig and optional KYC compliance.

## Requirements

### Requirement 1: Non-Custodial BTC Commitment System

**User Story:** As a Bitcoin holder, I want to commit my BTC to the protocol without transferring custody, so that I can earn staking rewards while maintaining full control of my assets.

#### Acceptance Criteria

1. WHEN a user commits BTC THEN the system SHALL verify the commitment using ECDSA proofs without requiring custody transfer
2. WHEN BTC balance verification occurs THEN the system SHALL use Chainlink oracles to check UTXO balances every 60 seconds
3. IF oracle verification fails THEN the system SHALL retry verification within 60 seconds
4. WHEN a commitment is made THEN the system SHALL store the commitment amount, user address, and timestamp on-chain
5. IF a user's BTC balance falls below their commitment THEN the system SHALL update their reward eligibility accordingly

### Requirement 2: Protocol Asset Staking and Reward Distribution

**User Story:** As a protocol participant, I want to receive rewards from protocol staking activities, so that I can earn yield on my committed BTC without active management.

#### Acceptance Criteria

1. WHEN the protocol stakes assets THEN it SHALL allocate SOL (40%), ETH (30%), and ATOM (30%) according to the specified distribution
2. WHEN ATOM staking occurs THEN the system SHALL distribute 20% to Cosmos Hub via Everstake/Cephalopod and 10% to Osmosis
3. WHEN staking rewards are generated THEN the system SHALL share 50% of profits with users at a 1:2 ratio (e.g., $1,000 BTC commitment = $1,000 in staked assets)
4. IF validators are slashed THEN the protocol SHALL absorb all losses without affecting user rewards
5. WHEN rewards are calculated THEN the system SHALL use state channels for off-chain computation with Solana/Cosmos settlement

### Requirement 3: Multi-Currency Reward Payment System

**User Story:** As a user receiving rewards, I want to choose between BTC and USDC payments with auto-reinvestment options, so that I can optimize my reward strategy.

#### Acceptance Criteria

1. WHEN rewards are paid THEN the system SHALL default to BTC payments via Lightning Network
2. WHEN a user selects USDC THEN the system SHALL provide USDC payments as an alternative option
3. WHEN auto-reinvestment is enabled THEN the system SHALL automatically reinvest rewards according to user preferences
4. WHEN Lightning Network payments fail THEN the system SHALL fallback to on-chain BTC transactions
5. IF payment processing fails THEN the system SHALL queue rewards for retry within 24 hours

### Requirement 4: Multi-Language Frontend with Wallet Integration

**User Story:** As a global user, I want to access the protocol through a localized interface with seamless wallet connectivity, so that I can interact with the system in my preferred language and wallet.

#### Acceptance Criteria

1. WHEN accessing the frontend THEN the system SHALL support English, Spanish, Chinese, and Japanese languages via i18next
2. WHEN connecting wallets THEN the system SHALL integrate BlueWallet and Ledger with one-click connect functionality
3. WHEN viewing the dashboard THEN the system SHALL display real-time user BTC balance, rewards, and treasury data
4. WHEN displaying treasury information THEN the system SHALL show total assets and rewards in USD while hiding allocation details
5. IF wallet connection fails THEN the system SHALL provide clear error messages and retry options

### Requirement 5: Enterprise Security and Compliance

**User Story:** As a security-conscious user, I want enterprise-grade security measures and optional compliance features, so that I can trust the protocol with my assets.

#### Acceptance Criteria

1. WHEN protocol assets are managed THEN the system SHALL use 2-of-3 multisig with Yubico HSMs (CC EAL5+)
2. WHEN smart contracts are deployed THEN they SHALL be audited by Certik and scanned with Slither
3. WHEN users exceed 1 BTC commitment THEN the system SHALL require KYC verification via Chainalysis
4. WHEN users stay below 1 BTC THEN the system SHALL allow non-KYC participation
5. WHEN user authentication occurs THEN the system SHALL require 2FA or passkey verification
6. IF a user's wallet is compromised THEN 2FA/passkey SHALL block unauthorized actions

### Requirement 6: Treasury Management and Funding

**User Story:** As the protocol operator, I want to fund and manage the treasury through regular deposits, so that the protocol can generate staking rewards sustainably.

#### Acceptance Criteria

1. WHEN treasury funding occurs THEN the system SHALL accept $50 biweekly deposits
2. WHEN deposits are processed THEN the system SHALL convert funds to SOL, ETH, and ATOM according to allocation percentages
3. WHEN treasury operations occur THEN the system SHALL avoid ICOs, grants, or external funding
4. WHEN treasury data is displayed THEN the system SHALL show total assets and USD rewards without revealing exact allocations
5. IF treasury balance falls below minimum thresholds THEN the system SHALL alert the operator

### Requirement 7: Testing and Development Infrastructure

**User Story:** As a developer, I want comprehensive testing capabilities that work on low-resource systems, so that I can ensure protocol reliability without expensive hardware.

#### Acceptance Criteria

1. WHEN tests are executed THEN the system SHALL run concurrent Python tests using pytest and ThreadPoolExecutor
2. WHEN testing components THEN the system SHALL cover BTC commitments, staking, rewards, treasury, and frontend functionality
3. WHEN running on low-resource PCs THEN the system SHALL optimize for 8GB RAM and 256GB storage constraints
4. WHEN configurations are needed THEN the system SHALL provide JSON-like Python configs editable by non-programmers
5. IF tests fail THEN the system SHALL provide clear error messages and debugging information

### Requirement 8: Oracle Integration and Reliability

**User Story:** As a protocol user, I want reliable BTC balance verification that prevents spoofing, so that my commitments are accurately tracked and rewarded.

#### Acceptance Criteria

1. WHEN balance verification occurs THEN the system SHALL use Chainlink oracles for BTC/USD and UTXO checks
2. WHEN oracle data is requested THEN the system SHALL verify ECDSA proofs to prevent spoofing
3. WHEN oracle feeds are unavailable THEN the system SHALL implement retry logic with exponential backoff
4. WHEN verification completes THEN the system SHALL cache results for up to 5 minutes to optimize performance
5. IF oracle manipulation is detected THEN the system SHALL halt operations and alert administrators