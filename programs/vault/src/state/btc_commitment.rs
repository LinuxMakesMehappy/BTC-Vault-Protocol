use anchor_lang::prelude::*;
use secp256k1::{ecdsa::Signature, Message, PublicKey, Secp256k1};
use sha2::{Digest, Sha256};
use crate::errors::VaultError;

#[account]
pub struct BTCCommitment {
    pub user_address: Pubkey,
    pub btc_address: String,
    pub amount: u64,
    pub ecdsa_proof: Vec<u8>,
    pub timestamp: i64,
    pub verified: bool,
    pub last_verification: i64,
    pub commitment_hash: [u8; 32],
    pub public_key: Vec<u8>,
    pub bump: u8,
}

impl BTCCommitment {
    pub const LEN: usize = 8 + // discriminator
        32 + // user_address
        4 + 64 + // btc_address (max 64 chars)
        8 + // amount
        4 + 256 + // ecdsa_proof (max 256 bytes)
        8 + // timestamp
        1 + // verified
        8 + // last_verification
        32 + // commitment_hash
        4 + 65 + // public_key (compressed: 33 bytes, uncompressed: 65 bytes)
        1; // bump

    /// Validates the BTC address format
    pub fn validate_btc_address(address: &str) -> Result<()> {
        // Check length constraints
        if address.len() < 26 || address.len() > 62 {
            return Err(VaultError::InvalidBTCAddress.into());
        }

        // Check for valid BTC address prefixes
        let valid_prefixes = ["1", "3", "bc1", "tb1", "2"];
        let has_valid_prefix = valid_prefixes.iter().any(|&prefix| address.starts_with(prefix));
        
        if !has_valid_prefix {
            return Err(VaultError::InvalidBTCAddress.into());
        }

        // For bc1 (Bech32) addresses, perform additional validation
        if address.starts_with("bc1") || address.starts_with("tb1") {
            if !Self::validate_bech32_address(address) {
                return Err(VaultError::InvalidBTCAddress.into());
            }
        }

        Ok(())
    }

    /// Validates Bech32 address format (simplified validation)
    fn validate_bech32_address(address: &str) -> bool {
        // Basic Bech32 validation - check character set
        let valid_chars = "qpzry9x8gf2tvdw0s3jn54khce6mua7l";
        let (_, data) = address.split_at(3); // Skip "bc1" or "tb1"
        
        data.chars().all(|c| valid_chars.contains(c.to_ascii_lowercase()))
    }

    /// Validates ECDSA proof for BTC commitment
    pub fn validate_ecdsa_proof(
        &self,
        message_data: &[u8],
        signature_bytes: &[u8],
        public_key_bytes: &[u8],
    ) -> Result<bool> {
        if signature_bytes.len() != 64 {
            return Err(VaultError::InvalidECDSAProof.into());
        }

        if public_key_bytes.is_empty() || public_key_bytes.len() > 65 {
            return Err(VaultError::InvalidECDSAProof.into());
        }

        let secp = Secp256k1::new();
        
        // Parse the public key
        let public_key = PublicKey::from_slice(public_key_bytes)
            .map_err(|_| VaultError::InvalidECDSAProof)?;

        // Parse the signature
        let signature = Signature::from_compact(signature_bytes)
            .map_err(|_| VaultError::InvalidECDSAProof)?;

        // Create message hash
        let message_hash = Sha256::digest(message_data);
        let message = Message::from_digest_slice(&message_hash)
            .map_err(|_| VaultError::InvalidECDSAProof)?;

        // Verify the signature
        match secp.verify_ecdsa(&message, &signature, &public_key) {
            Ok(()) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    /// Creates a commitment hash for the BTC commitment
    pub fn create_commitment_hash(
        user_address: &Pubkey,
        btc_address: &str,
        amount: u64,
        timestamp: i64,
    ) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(user_address.as_ref());
        hasher.update(btc_address.as_bytes());
        hasher.update(&amount.to_le_bytes());
        hasher.update(&timestamp.to_le_bytes());
        hasher.finalize().into()
    }

    /// Validates the entire commitment structure
    pub fn validate_commitment(&self) -> Result<()> {
        // Validate BTC address
        Self::validate_btc_address(&self.btc_address)?;

        // Validate amount (must be greater than 0)
        if self.amount == 0 {
            return Err(VaultError::InsufficientBalance.into());
        }

        // Validate ECDSA proof exists
        if self.ecdsa_proof.is_empty() {
            return Err(VaultError::InvalidECDSAProof.into());
        }

        // Validate public key exists
        if self.public_key.is_empty() {
            return Err(VaultError::InvalidECDSAProof.into());
        }

        // Validate timestamp (should not be in the future)
        let clock = Clock::get()?;
        if self.timestamp > clock.unix_timestamp {
            return Err(VaultError::SecurityViolation.into());
        }

        // Validate commitment hash
        let expected_hash = Self::create_commitment_hash(
            &self.user_address,
            &self.btc_address,
            self.amount,
            self.timestamp,
        );
        
        if self.commitment_hash != expected_hash {
            return Err(VaultError::SecurityViolation.into());
        }

        Ok(())
    }

    /// Prevents replay attacks by checking timestamp freshness
    pub fn validate_timestamp_freshness(&self, max_age_seconds: i64) -> Result<()> {
        let clock = Clock::get()?;
        let age = clock.unix_timestamp - self.timestamp;
        
        if age > max_age_seconds {
            return Err(VaultError::SecurityViolation.into());
        }
        
        Ok(())
    }

    /// Serializes commitment data for ECDSA signing
    pub fn serialize_for_signing(
        user_address: &Pubkey,
        btc_address: &str,
        amount: u64,
        timestamp: i64,
    ) -> Vec<u8> {
        let mut data = Vec::new();
        data.extend_from_slice(user_address.as_ref());
        data.extend_from_slice(btc_address.as_bytes());
        data.extend_from_slice(&amount.to_le_bytes());
        data.extend_from_slice(&timestamp.to_le_bytes());
        data
    }
}

#[cfg(test)]
#[path = "btc_commitment_tests.rs"]
mod tests;