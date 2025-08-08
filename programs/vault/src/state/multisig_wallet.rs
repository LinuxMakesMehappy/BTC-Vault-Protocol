use anchor_lang::prelude::*;
use crate::errors::VaultError;

/// HSM key information for Yubico HSM integration
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct HSMKeyInfo {
    pub key_id: u16,           // HSM key slot ID
    pub public_key: [u8; 32],  // Public key derived from HSM
    pub key_label: String,     // Human-readable key label
    pub created_at: i64,       // Key creation timestamp
    pub last_used: i64,        // Last usage timestamp
    pub is_active: bool,       // Whether key is active
    pub usage_count: u64,      // Number of times key has been used
}

/// Signer information with HSM integration
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct SignerInfo {
    pub pubkey: Pubkey,        // Solana public key
    pub hsm_key: Option<HSMKeyInfo>, // Associated HSM key (if any)
    pub role: SignerRole,      // Role of the signer
    pub added_at: i64,         // When signer was added
    pub last_signature: i64,   // Last signature timestamp
    pub is_active: bool,       // Whether signer is active
}

/// Roles for multisig signers
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum SignerRole {
    Admin,      // Full administrative access
    Operator,   // Operational transactions only
    Emergency,  // Emergency operations only
}

/// Transaction types for different operations
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum TransactionType {
    TreasuryTransfer,    // Treasury fund transfers
    StakingOperation,    // Staking/unstaking operations
    RewardDistribution,  // Reward distribution to users
    ConfigUpdate,        // Protocol configuration updates
    EmergencyAction,     // Emergency operations
    KeyRotation,         // Key rotation operations
}

/// Transaction priority levels
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum TransactionPriority {
    Low,      // Standard operations
    Medium,   // Important operations
    High,     // Critical operations
    Emergency, // Emergency operations
}

#[account]
pub struct MultisigWallet {
    pub signers: Vec<SignerInfo>,
    pub threshold: u8,              // 2-of-3 requirement
    pub transaction_count: u32,     // Total transactions proposed
    pub executed_count: u32,        // Total transactions executed
    pub hsm_enabled: bool,          // Whether HSM is required
    pub emergency_mode: bool,       // Emergency mode status
    pub last_key_rotation: i64,     // Last key rotation timestamp
    pub key_rotation_interval: i64, // Required rotation interval (seconds)
    pub created_at: i64,           // Wallet creation timestamp
    pub bump: u8,
}

impl MultisigWallet {
    pub const LEN: usize = 8 + // discriminator
        4 + (3 * (32 + (2 + 32 + 32 + 8 + 8 + 1 + 8) + 1 + 8 + 8 + 1)) + // signers with HSM info
        1 + // threshold
        4 + // transaction_count
        4 + // executed_count
        1 + // hsm_enabled
        1 + // emergency_mode
        8 + // last_key_rotation
        8 + // key_rotation_interval
        8 + // created_at
        1; // bump

    pub const MAX_SIGNERS: usize = 3;
    pub const REQUIRED_THRESHOLD: u8 = 2;
    pub const DEFAULT_KEY_ROTATION_INTERVAL: i64 = 7776000; // 90 days in seconds
    pub const EMERGENCY_THRESHOLD: u8 = 1; // Emergency operations need only 1 signature

    /// Initialize multisig wallet with HSM configuration
    pub fn initialize(
        &mut self,
        signers: Vec<SignerInfo>,
        hsm_enabled: bool,
        bump: u8,
    ) -> Result<()> {
        if signers.len() > Self::MAX_SIGNERS {
            return Err(VaultError::InvalidAllocation.into());
        }

        if signers.len() < Self::REQUIRED_THRESHOLD as usize {
            return Err(VaultError::MultisigThresholdNotMet.into());
        }

        let clock = Clock::get()?;
        
        self.signers = signers;
        self.threshold = Self::REQUIRED_THRESHOLD;
        self.transaction_count = 0;
        self.executed_count = 0;
        self.hsm_enabled = hsm_enabled;
        self.emergency_mode = false;
        self.last_key_rotation = clock.unix_timestamp;
        self.key_rotation_interval = Self::DEFAULT_KEY_ROTATION_INTERVAL;
        self.created_at = clock.unix_timestamp;
        self.bump = bump;

        Ok(())
    }

    /// Check if key rotation is required
    pub fn needs_key_rotation(&self) -> Result<bool> {
        let clock = Clock::get()?;
        let time_since_rotation = clock.unix_timestamp - self.last_key_rotation;
        Ok(time_since_rotation >= self.key_rotation_interval)
    }

    /// Validate signer has required role for transaction type
    pub fn validate_signer_role(&self, signer: &Pubkey, tx_type: &TransactionType) -> Result<bool> {
        let signer_info = self.signers.iter()
            .find(|s| s.pubkey == *signer && s.is_active)
            .ok_or(VaultError::UnauthorizedSigner)?;

        let has_permission = match tx_type {
            TransactionType::EmergencyAction => {
                signer_info.role == SignerRole::Admin || signer_info.role == SignerRole::Emergency
            },
            TransactionType::KeyRotation | TransactionType::ConfigUpdate => {
                signer_info.role == SignerRole::Admin
            },
            _ => true, // All active signers can sign other transaction types
        };

        Ok(has_permission)
    }

    /// Get required threshold for transaction type
    pub fn get_required_threshold(&self, tx_type: &TransactionType, priority: &TransactionPriority) -> u8 {
        match (tx_type, priority) {
            (TransactionType::EmergencyAction, TransactionPriority::Emergency) => {
                if self.emergency_mode {
                    Self::EMERGENCY_THRESHOLD
                } else {
                    self.threshold
                }
            },
            _ => self.threshold,
        }
    }

    /// Activate emergency mode
    pub fn activate_emergency_mode(&mut self) -> Result<()> {
        self.emergency_mode = true;
        msg!("Emergency mode activated");
        Ok(())
    }

    /// Deactivate emergency mode
    pub fn deactivate_emergency_mode(&mut self) -> Result<()> {
        self.emergency_mode = false;
        msg!("Emergency mode deactivated");
        Ok(())
    }

    /// Rotate HSM keys
    pub fn rotate_keys(&mut self, new_signers: Vec<SignerInfo>) -> Result<()> {
        if new_signers.len() != self.signers.len() {
            return Err(VaultError::InvalidAllocation.into());
        }

        // Validate all new signers have HSM keys if HSM is enabled
        if self.hsm_enabled {
            for signer in &new_signers {
                if signer.hsm_key.is_none() {
                    return Err(VaultError::SecurityViolation.into());
                }
            }
        }

        let clock = Clock::get()?;
        
        // Deactivate old signers
        for signer in &mut self.signers {
            signer.is_active = false;
        }

        // Add new signers
        self.signers = new_signers;
        self.last_key_rotation = clock.unix_timestamp;

        msg!("Key rotation completed with {} new signers", self.signers.len());
        Ok(())
    }

    /// Update signer's last signature timestamp
    pub fn update_signer_usage(&mut self, signer_pubkey: &Pubkey) -> Result<()> {
        let clock = Clock::get()?;
        
        if let Some(signer) = self.signers.iter_mut().find(|s| s.pubkey == *signer_pubkey) {
            signer.last_signature = clock.unix_timestamp;
            
            // Update HSM key usage if applicable
            if let Some(ref mut hsm_key) = signer.hsm_key {
                hsm_key.last_used = clock.unix_timestamp;
                hsm_key.usage_count = hsm_key.usage_count.checked_add(1).unwrap();
            }
        }

        Ok(())
    }
}

#[account]
pub struct MultisigTransaction {
    pub multisig: Pubkey,
    pub transaction_id: u32,
    pub proposer: Pubkey,
    pub transaction_type: TransactionType,
    pub priority: TransactionPriority,
    pub transaction_data: Vec<u8>,
    pub signatures: Vec<MultisigSignature>,
    pub required_signatures: u8,
    pub executed: bool,
    pub cancelled: bool,
    pub expires_at: i64,           // Transaction expiration
    pub created_at: i64,
    pub executed_at: Option<i64>,  // When transaction was executed
    pub execution_result: Option<String>, // Execution result or error
    pub bump: u8,
}

/// Signature information with HSM validation
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct MultisigSignature {
    pub signer: Pubkey,
    pub signature: [u8; 64],
    pub hsm_signature: Option<Vec<u8>>, // HSM signature if applicable
    pub signed_at: i64,
    pub signature_type: SignatureType,
}

/// Types of signatures supported
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum SignatureType {
    Standard,  // Standard Ed25519 signature
    HSM,       // HSM-generated signature
    Emergency, // Emergency signature (relaxed validation)
}

impl MultisigTransaction {
    pub const LEN: usize = 8 + // discriminator
        32 + // multisig
        4 + // transaction_id
        32 + // proposer
        1 + // transaction_type
        1 + // priority
        4 + 2048 + // transaction_data (max 2KB)
        4 + (3 * (32 + 64 + 4 + 64 + 8 + 1)) + // signatures with HSM data
        1 + // required_signatures
        1 + // executed
        1 + // cancelled
        8 + // expires_at
        8 + // created_at
        9 + // executed_at (Option<i64>)
        4 + 256 + // execution_result (max 256 chars)
        1; // bump

    pub const DEFAULT_EXPIRATION_HOURS: i64 = 24; // 24 hours default expiration

    /// Initialize transaction with proper validation
    pub fn initialize(
        &mut self,
        multisig: Pubkey,
        transaction_id: u32,
        proposer: Pubkey,
        transaction_type: TransactionType,
        priority: TransactionPriority,
        transaction_data: Vec<u8>,
        required_signatures: u8,
        bump: u8,
    ) -> Result<()> {
        let clock = Clock::get()?;
        
        self.multisig = multisig;
        self.transaction_id = transaction_id;
        self.proposer = proposer;
        self.transaction_type = transaction_type;
        self.priority = priority;
        self.transaction_data = transaction_data;
        self.signatures = Vec::new();
        self.required_signatures = required_signatures;
        self.executed = false;
        self.cancelled = false;
        self.expires_at = clock.unix_timestamp + (Self::DEFAULT_EXPIRATION_HOURS * 3600);
        self.created_at = clock.unix_timestamp;
        self.executed_at = None;
        self.execution_result = None;
        self.bump = bump;

        Ok(())
    }

    /// Check if transaction has expired
    pub fn is_expired(&self) -> Result<bool> {
        let clock = Clock::get()?;
        Ok(clock.unix_timestamp > self.expires_at)
    }

    /// Check if transaction has enough signatures
    pub fn has_enough_signatures(&self) -> bool {
        self.signatures.len() >= self.required_signatures as usize
    }

    /// Add signature to transaction
    pub fn add_signature(&mut self, signature: MultisigSignature) -> Result<()> {
        // Check if signer already signed
        if self.signatures.iter().any(|s| s.signer == signature.signer) {
            return Err(VaultError::TransactionAlreadyExecuted.into());
        }

        // Validate signature is not expired
        if self.is_expired()? {
            return Err(VaultError::SecurityViolation.into());
        }

        self.signatures.push(signature);
        Ok(())
    }

    /// Mark transaction as executed
    pub fn mark_executed(&mut self, result: Option<String>) -> Result<()> {
        let clock = Clock::get()?;
        
        self.executed = true;
        self.executed_at = Some(clock.unix_timestamp);
        self.execution_result = result;

        Ok(())
    }

    /// Cancel transaction
    pub fn cancel(&mut self, reason: String) -> Result<()> {
        if self.executed {
            return Err(VaultError::TransactionAlreadyExecuted.into());
        }

        self.cancelled = true;
        self.execution_result = Some(format!("Cancelled: {}", reason));

        Ok(())
    }

    /// Validate transaction data integrity
    pub fn validate_transaction_data(&self) -> Result<()> {
        // Basic validation
        if self.transaction_data.is_empty() {
            return Err(VaultError::InvalidAllocation.into());
        }

        if self.transaction_data.len() > 2048 {
            return Err(VaultError::InvalidAllocation.into());
        }

        // Type-specific validation
        match self.transaction_type {
            TransactionType::TreasuryTransfer => {
                // Validate treasury transfer data format
                if self.transaction_data.len() < 40 { // Minimum: recipient (32) + amount (8)
                    return Err(VaultError::InvalidAllocation.into());
                }
            },
            TransactionType::KeyRotation => {
                // Validate key rotation data
                if self.transaction_data.len() < 96 { // Minimum: 3 pubkeys (32 each)
                    return Err(VaultError::InvalidAllocation.into());
                }
            },
            _ => {
                // Other transaction types have basic validation
            }
        }

        Ok(())
    }
}
