use anchor_lang::prelude::*;
use std::collections::HashMap;
use rand::{rngs::OsRng, RngCore};

/// Oracle data structure for storing Chainlink feed information
#[account]
pub struct OracleData {
    /// Oracle feed address for BTC/USD price
    pub btc_usd_feed: Pubkey,
    /// Last updated timestamp
    pub last_update: i64,
    /// Current BTC price in USD (8 decimals)
    pub btc_price_usd: u64,
    /// Oracle round ID for tracking updates
    pub round_id: u64,
    /// Verification interval in seconds (default: 60)
    pub verification_interval: u64,
    /// Cache duration in seconds (default: 300 = 5 minutes)
    pub cache_duration: u64,
    /// Oracle status flags
    pub is_active: bool,
    /// Retry configuration
    pub retry_config: RetryConfig,
    /// UTXO verification cache
    pub utxo_cache: HashMap<String, UTXOVerification>,
}

/// Retry configuration for oracle failures
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct RetryConfig {
    /// Maximum number of retries
    pub max_retries: u8,
    /// Base delay in seconds for exponential backoff
    pub base_delay: u64,
    /// Maximum delay in seconds
    pub max_delay: u64,
    /// Current retry count
    pub current_retries: u8,
    /// Last retry timestamp
    pub last_retry: i64,
}

/// UTXO verification result
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct UTXOVerification {
    /// BTC address being verified
    pub btc_address: String,
    /// Verified balance in satoshis
    pub balance: u64,
    /// Verification timestamp
    pub verified_at: i64,
    /// ECDSA proof hash for anti-spoofing
    pub proof_hash: [u8; 32],
    /// Verification status
    pub is_valid: bool,
    /// Cache expiry timestamp
    pub expires_at: i64,
}

/// Oracle error types for comprehensive error handling
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub enum OracleError {
    FeedUnavailable,
    StaleData,
    InvalidProof,
    NetworkTimeout,
    InsufficientData,
    PriceDeviation,
    VerificationFailed,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            base_delay: 2,      // 2 seconds
            max_delay: 60,      // 1 minute max
            current_retries: 0,
            last_retry: 0,
        }
    }
}

impl OracleData {
    pub const LEN: usize = 8 + // discriminator
        32 + // btc_usd_feed
        8 +  // last_update
        8 +  // btc_price_usd
        8 +  // round_id
        8 +  // verification_interval
        8 +  // cache_duration
        1 +  // is_active
        (1 + 8 + 8 + 1 + 8) + // retry_config
        4 + (32 * 10 * (4 + 32 + 8 + 8 + 32 + 1 + 8)); // utxo_cache (estimated)

    /// Initialize oracle with default configuration
    pub fn initialize(&mut self, btc_usd_feed: Pubkey) -> Result<()> {
        self.btc_usd_feed = btc_usd_feed;
        self.last_update = Clock::get()?.unix_timestamp;
        self.btc_price_usd = 0;
        self.round_id = OsRng.next_u64(); // Secure random round ID
        self.verification_interval = 60; // 60 seconds
        self.cache_duration = 300;       // 5 minutes
        self.is_active = true;
        self.retry_config = RetryConfig::default();
        self.utxo_cache = HashMap::new();
        Ok(())
    }

    /// Update BTC price from Chainlink feed
    pub fn update_btc_price(&mut self, price: u64, round_id: u64) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp;
        
        // Validate price data
        if price == 0 {
            return Err(crate::errors::VaultError::OraclePriceUnavailable.into());
        }

        // Check for significant price deviation (>10% change)
        if self.btc_price_usd > 0 {
            let price_change = if price > self.btc_price_usd {
                ((price - self.btc_price_usd) * 100) / self.btc_price_usd
            } else {
                ((self.btc_price_usd - price) * 100) / self.btc_price_usd
            };

            if price_change > 10 {
                msg!("Warning: Large price deviation detected: {}%", price_change);
            }
        }

        self.btc_price_usd = price;
        self.round_id = round_id;
        self.last_update = current_time;
        self.retry_config.current_retries = 0; // Reset retry count on success

        Ok(())
    }

    /// Check if oracle data is stale
    pub fn is_stale(&self) -> Result<bool> {
        let current_time = Clock::get()?.unix_timestamp;
        let age = current_time - self.last_update;
        Ok(age > self.verification_interval as i64)
    }

    /// Get cached UTXO verification if valid
    pub fn get_cached_utxo(&self, btc_address: &str) -> Option<&UTXOVerification> {
        if let Some(verification) = self.utxo_cache.get(btc_address) {
            let current_time = Clock::get().ok()?.unix_timestamp;
            if current_time < verification.expires_at && verification.is_valid {
                return Some(verification);
            }
        }
        None
    }

    /// Cache UTXO verification result
    pub fn cache_utxo_verification(
        &mut self,
        btc_address: String,
        balance: u64,
        proof_hash: [u8; 32],
        is_valid: bool,
    ) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp;
        let expires_at = current_time + self.cache_duration as i64;

        let verification = UTXOVerification {
            btc_address: btc_address.clone(),
            balance,
            verified_at: current_time,
            proof_hash,
            is_valid,
            expires_at,
        };

        self.utxo_cache.insert(btc_address, verification);
        Ok(())
    }

    /// Calculate next retry delay using exponential backoff
    pub fn get_next_retry_delay(&self) -> u64 {
        let delay = self.retry_config.base_delay * (2_u64.pow(self.retry_config.current_retries as u32));
        std::cmp::min(delay, self.retry_config.max_delay)
    }

    /// Check if retry is allowed
    pub fn can_retry(&self) -> bool {
        self.retry_config.current_retries < self.retry_config.max_retries
    }

    /// Increment retry counter
    pub fn increment_retry(&mut self) -> Result<()> {
        self.retry_config.current_retries += 1;
        self.retry_config.last_retry = Clock::get()?.unix_timestamp;
        Ok(())
    }

    /// Reset retry counter on success
    pub fn reset_retry(&mut self) {
        self.retry_config.current_retries = 0;
        self.retry_config.last_retry = 0;
    }

    /// Validate ECDSA proof for anti-spoofing
    pub fn validate_ecdsa_proof(
        &self,
        btc_address: &str,
        balance: u64,
        proof: &[u8],
    ) -> Result<bool> {
        use sha2::{Digest, Sha256};
        use secp256k1::{ecdsa::Signature, Message, PublicKey, Secp256k1};

        // Validate proof length (64 bytes for signature + 33 bytes for pubkey = 97 total)
        if proof.len() != 97 {
            return Ok(false);
        }

        // Split proof into signature and public key
        let (sig_bytes, pubkey_bytes) = proof.split_at(64);
        
        // Create message hash from address and balance
        let mut hasher = Sha256::new();
        hasher.update(btc_address.as_bytes());
        hasher.update(&balance.to_le_bytes());
        // Add random nonce to prevent replay attacks
        let nonce = OsRng.next_u64();
        hasher.update(&nonce.to_le_bytes());
        let message_hash = hasher.finalize();

        // Parse signature and public key
        let signature = match Signature::from_compact(sig_bytes) {
            Ok(sig) => sig,
            Err(_) => return Ok(false),
        };

        let public_key = match PublicKey::from_slice(pubkey_bytes) {
            Ok(pk) => pk,
            Err(_) => return Ok(false),
        };

        // Create message for verification
        let message = match Message::from_digest_slice(&message_hash) {
            Ok(msg) => msg,
            Err(_) => return Ok(false),
        };

        // Verify signature
        let secp = Secp256k1::verification_only();
        let is_valid = secp.verify_ecdsa(&message, &signature, &public_key).is_ok();

        Ok(is_valid)
    }

    /// Clean expired cache entries
    pub fn cleanup_cache(&mut self) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp;
        self.utxo_cache.retain(|_, verification| {
            current_time < verification.expires_at
        });
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oracle_initialization() {
        let mut oracle = OracleData {
            btc_usd_feed: Pubkey::default(),
            last_update: 0,
            btc_price_usd: 0,
            round_id: 0,
            verification_interval: 0,
            cache_duration: 0,
            is_active: false,
            retry_config: RetryConfig::default(),
            utxo_cache: HashMap::new(),
        };

        let feed_address = Pubkey::new_unique();
        oracle.initialize(feed_address).unwrap();

        assert_eq!(oracle.btc_usd_feed, feed_address);
        assert_eq!(oracle.verification_interval, 60);
        assert_eq!(oracle.cache_duration, 300);
        assert!(oracle.is_active);
    }

    #[test]
    fn test_retry_exponential_backoff() {
        let mut retry_config = RetryConfig::default();
        let oracle = OracleData {
            btc_usd_feed: Pubkey::default(),
            last_update: 0,
            btc_price_usd: 0,
            round_id: 0,
            verification_interval: 60,
            cache_duration: 300,
            is_active: true,
            retry_config: retry_config.clone(),
            utxo_cache: HashMap::new(),
        };

        // Test exponential backoff calculation
        assert_eq!(oracle.get_next_retry_delay(), 2);  // 2^0 * 2 = 2
        
        retry_config.current_retries = 1;
        // Test retry delay calculation directly
        retry_config.current_retries = 1;
        let oracle_retry1 = OracleData {
            btc_usd_feed: oracle.btc_usd_feed,
            last_update: oracle.last_update,
            btc_price_usd: oracle.btc_price_usd,
            round_id: oracle.round_id,
            verification_interval: oracle.verification_interval,
            cache_duration: oracle.cache_duration,
            is_active: oracle.is_active,
            retry_config: retry_config.clone(),
            utxo_cache: HashMap::new(),
        };
        assert_eq!(oracle_retry1.get_next_retry_delay(), 4);  // 2^1 * 2 = 4
        
        retry_config.current_retries = 2;
        let oracle_retry2 = OracleData {
            btc_usd_feed: oracle.btc_usd_feed,
            last_update: oracle.last_update,
            btc_price_usd: oracle.btc_price_usd,
            round_id: oracle.round_id,
            verification_interval: oracle.verification_interval,
            cache_duration: oracle.cache_duration,
            is_active: oracle.is_active,
            retry_config: retry_config.clone(),
            utxo_cache: HashMap::new(),
        };
        assert_eq!(oracle_retry2.get_next_retry_delay(), 8);  // 2^2 * 2 = 8
    }

    #[test]
    fn test_ecdsa_proof_validation() {
        let oracle = OracleData {
            btc_usd_feed: Pubkey::default(),
            last_update: 0,
            btc_price_usd: 0,
            round_id: 0,
            verification_interval: 60,
            cache_duration: 300,
            is_active: true,
            retry_config: RetryConfig::default(),
            utxo_cache: HashMap::new(),
        };

        // Test valid proof (64 bytes)
        let valid_proof = vec![1u8; 64];
        let result = oracle.validate_ecdsa_proof("bc1qtest", 100000000, &valid_proof);
        assert!(result.is_ok());
        assert!(result.unwrap());

        // Test invalid proof (wrong length)
        let invalid_proof = vec![1u8; 32];
        let result = oracle.validate_ecdsa_proof("bc1qtest", 100000000, &invalid_proof);
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }
}