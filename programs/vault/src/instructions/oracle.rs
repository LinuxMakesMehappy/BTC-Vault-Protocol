use anchor_lang::prelude::*;
use crate::state::{oracle::*, btc_commitment::BTCCommitment, user_account::UserAccount};
use crate::errors::VaultError;

/// Initialize oracle with Chainlink feed address
#[derive(Accounts)]
pub struct InitializeOracle<'info> {
    #[account(
        init,
        payer = authority,
        space = OracleData::LEN,
        seeds = [b"oracle"],
        bump
    )]
    pub oracle_data: Account<'info, OracleData>,
    
    #[account(
        mut,
        constraint = authority.is_signer @ VaultError::MissingSigner
    )]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

/// Update BTC price from Chainlink oracle
#[derive(Accounts)]
pub struct UpdateBTCPrice<'info> {
    #[account(
        mut,
        seeds = [b"oracle"],
        bump
    )]
    pub oracle_data: Account<'info, OracleData>,
    
    /// Chainlink oracle account (in production, this would be the actual Chainlink feed)
    /// CHECK: This is the Chainlink BTC/USD price feed account
    pub chainlink_feed: AccountInfo<'info>,
    
    #[account(
        constraint = oracle_authority.is_signer @ VaultError::MissingSigner
    )]
    pub oracle_authority: Signer<'info>,
}

/// Verify BTC balance using oracle and ECDSA proof
#[derive(Accounts)]
pub struct VerifyBTCBalance<'info> {
    #[account(
        mut,
        seeds = [b"oracle"],
        bump
    )]
    pub oracle_data: Account<'info, OracleData>,
    
    #[account(
        mut,
        seeds = [b"btc_commitment", user.key().as_ref()],
        bump = btc_commitment.bump,
        constraint = btc_commitment.user_address == user.key() @ VaultError::UnauthorizedAccess
    )]
    pub btc_commitment: Account<'info, BTCCommitment>,
    
    #[account(
        mut,
        seeds = [b"user_account", user.key().as_ref()],
        bump = user_account.bump,
        constraint = user_account.owner == user.key() @ VaultError::UnauthorizedAccess
    )]
    pub user_account: Account<'info, UserAccount>,
    
    #[account(
        constraint = user.is_signer @ VaultError::MissingSigner
    )]
    pub user: Signer<'info>,
}

/// Oracle instruction implementations
impl<'info> InitializeOracle<'info> {
    pub fn process(ctx: Context<InitializeOracle>, btc_usd_feed: Pubkey) -> Result<()> {
        let oracle_data = &mut ctx.accounts.oracle_data;
        oracle_data.initialize(btc_usd_feed)?;
        
        msg!("Oracle initialized with BTC/USD feed: {}", btc_usd_feed);
        Ok(())
    }
}

impl<'info> UpdateBTCPrice<'info> {
    pub fn process(
        ctx: Context<UpdateBTCPrice>,
        price: u64,
        round_id: u64,
        timestamp: i64,
    ) -> Result<()> {
        let oracle_data = &mut ctx.accounts.oracle_data;
        
        // Validate timestamp is recent (within 5 minutes)
        let current_time = Clock::get()?.unix_timestamp;
        if current_time - timestamp > 300 {
            return Err(VaultError::OraclePriceUnavailable.into());
        }
        
        // Update price data
        oracle_data.update_btc_price(price, round_id)?;
        
        msg!("BTC price updated: ${} (round: {})", price as f64 / 100_000_000.0, round_id);
        Ok(())
    }
}

impl<'info> VerifyBTCBalance<'info> {
    pub fn process(
        ctx: Context<VerifyBTCBalance>,
        btc_address: String,
        expected_balance: u64,
        ecdsa_proof: Vec<u8>,
    ) -> Result<()> {
        let oracle_data = &mut ctx.accounts.oracle_data;
        let btc_commitment = &mut ctx.accounts.btc_commitment;
        let user_account = &mut ctx.accounts.user_account;
        
        // Check if we have cached verification
        if let Some(cached) = oracle_data.get_cached_utxo(&btc_address) {
            if cached.balance >= expected_balance {
                btc_commitment.verified = true;
                btc_commitment.last_verification = Clock::get()?.unix_timestamp;
                msg!("BTC balance verified from cache: {} satoshis", cached.balance);
                return Ok(());
            }
        }
        
        // Validate ECDSA proof to prevent spoofing
        let proof_valid = oracle_data.validate_ecdsa_proof(
            &btc_address,
            expected_balance,
            &ecdsa_proof,
        )?;
        
        if !proof_valid {
            return Err(VaultError::InvalidECDSAProof.into());
        }
        
        // In production, this would make an actual call to Chainlink UTXO oracle
        // For now, we simulate the verification process
        let verified_balance = Self::simulate_utxo_verification(&btc_address, expected_balance)?;
        
        // Cache the verification result
        use sha2::{Digest, Sha256};
        let proof_hash = Sha256::digest(&ecdsa_proof).into();
        oracle_data.cache_utxo_verification(
            btc_address.clone(),
            verified_balance,
            proof_hash,
            verified_balance >= expected_balance,
        )?;
        
        // Update commitment verification status
        if verified_balance >= expected_balance {
            btc_commitment.verified = true;
            btc_commitment.last_verification = Clock::get()?.unix_timestamp;
            user_account.last_activity = Clock::get()?.unix_timestamp;
            
            msg!("BTC balance verified: {} satoshis (required: {})", 
                 verified_balance, expected_balance);
        } else {
            btc_commitment.verified = false;
            msg!("BTC balance insufficient: {} satoshis (required: {})", 
                 verified_balance, expected_balance);
        }
        
        Ok(())
    }
    
    /// Simulate UTXO verification (in production, this would call Chainlink oracle)
    fn simulate_utxo_verification(btc_address: &str, expected_balance: u64) -> Result<u64> {
        // Validate BTC address format
        if !Self::is_valid_btc_address(btc_address) {
            return Err(VaultError::InvalidBTCAddress.into());
        }
        
        // In production, this would:
        // 1. Call Chainlink UTXO verification oracle
        // 2. Verify the address exists on Bitcoin network
        // 3. Sum all UTXOs for the address
        // 4. Return the total balance
        
        // For simulation, we assume the balance is valid if address format is correct
        // and return the expected balance (in production, this would be the actual balance)
        Ok(expected_balance)
    }
    
    /// Validate Bitcoin address format (simplified)
    fn is_valid_btc_address(address: &str) -> bool {
        // Basic validation for Bitcoin address formats
        if address.len() < 26 || address.len() > 62 {
            return false;
        }
        
        // Check for valid prefixes
        address.starts_with("1") ||      // P2PKH (Legacy)
        address.starts_with("3") ||      // P2SH (Script Hash)
        address.starts_with("bc1") ||    // Bech32 (Native SegWit)
        address.starts_with("tb1")       // Testnet Bech32
    }
}

/// Oracle management functions
pub struct OracleManager;

impl OracleManager {
    /// Handle oracle failure with retry logic
    pub fn handle_oracle_failure(
        oracle_data: &mut OracleData,
        error: OracleError,
    ) -> Result<()> {
        match error {
            OracleError::FeedUnavailable => {
                if oracle_data.can_retry() {
                    oracle_data.increment_retry()?;
                    let delay = oracle_data.get_next_retry_delay();
                    msg!("Oracle feed unavailable, retrying in {} seconds", delay);
                } else {
                    msg!("Oracle feed unavailable, max retries exceeded");
                    return Err(VaultError::OraclePriceUnavailable.into());
                }
            },
            OracleError::StaleData => {
                msg!("Oracle data is stale, using cached data if available");
                // Continue with cached data if within tolerance
            },
            OracleError::InvalidProof => {
                msg!("Invalid ECDSA proof detected - potential spoofing attempt");
                return Err(VaultError::SecurityViolation.into());
            },
            OracleError::NetworkTimeout => {
                if oracle_data.can_retry() {
                    oracle_data.increment_retry()?;
                    msg!("Network timeout, queuing retry");
                } else {
                    return Err(VaultError::OraclePriceUnavailable.into());
                }
            },
            _ => {
                msg!("Oracle error: {:?}", error);
                return Err(VaultError::OraclePriceUnavailable.into());
            }
        }
        Ok(())
    }
    
    /// Cleanup expired cache entries
    pub fn cleanup_oracle_cache(oracle_data: &mut OracleData) -> Result<()> {
        oracle_data.cleanup_cache()?;
        msg!("Oracle cache cleaned up");
        Ok(())
    }
    
    /// Get current BTC price with staleness check
    pub fn get_current_btc_price(oracle_data: &OracleData) -> Result<u64> {
        if oracle_data.is_stale()? {
            msg!("Warning: Oracle data is stale");
            // In production, trigger price update
        }
        
        if oracle_data.btc_price_usd == 0 {
            return Err(VaultError::OraclePriceUnavailable.into());
        }
        
        Ok(oracle_data.btc_price_usd)
    }
    
    /// Calculate USD value of BTC amount
    pub fn calculate_usd_value(
        oracle_data: &OracleData,
        btc_amount_satoshis: u64,
    ) -> Result<u64> {
        let btc_price = Self::get_current_btc_price(oracle_data)?;
        
        // Convert satoshis to BTC (divide by 100,000,000)
        // Multiply by USD price (8 decimals)
        // Result in USD cents (2 decimals)
        let usd_value = (btc_amount_satoshis * btc_price) / 100_000_000;
        
        Ok(usd_value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_btc_address_validation() {
        // Valid addresses
        assert!(VerifyBTCBalance::is_valid_btc_address("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa"));
        assert!(VerifyBTCBalance::is_valid_btc_address("3J98t1WpEZ73CNmQviecrnyiWrnqRhWNLy"));
        assert!(VerifyBTCBalance::is_valid_btc_address("bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4"));
        assert!(VerifyBTCBalance::is_valid_btc_address("tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx"));
        
        // Invalid addresses
        assert!(!VerifyBTCBalance::is_valid_btc_address(""));
        assert!(!VerifyBTCBalance::is_valid_btc_address("invalid"));
        assert!(!VerifyBTCBalance::is_valid_btc_address("2short"));
        assert!(!VerifyBTCBalance::is_valid_btc_address("toolongaddressthatexceedsmaximumlengthfortestingpurposes"));
    }

    #[test]
    fn test_usd_value_calculation() {
        let oracle_data = OracleData {
            btc_usd_feed: Pubkey::default(),
            last_update: 0,
            btc_price_usd: 5000000000000, // $50,000 with 8 decimals
            round_id: 1,
            verification_interval: 60,
            cache_duration: 300,
            is_active: true,
            retry_config: RetryConfig::default(),
            utxo_cache: std::collections::HashMap::new(),
        };

        // Test 1 BTC (100,000,000 satoshis) = $50,000
        let usd_value = OracleManager::calculate_usd_value(&oracle_data, 100_000_000).unwrap();
        assert_eq!(usd_value, 5000000000000); // $50,000 with 8 decimals

        // Test 0.1 BTC (10,000,000 satoshis) = $5,000
        let usd_value = OracleManager::calculate_usd_value(&oracle_data, 10_000_000).unwrap();
        assert_eq!(usd_value, 500000000000); // $5,000 with 8 decimals
    }
}
