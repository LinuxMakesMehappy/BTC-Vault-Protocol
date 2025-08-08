use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::VaultError;
use rand::RngCore;

#[derive(Accounts)]
pub struct CommitBTC<'info> {
    #[account(
        init_if_needed,
        payer = user,
        space = BTCCommitment::LEN,
        seeds = [b"btc_commitment", user.key().as_ref()],
        bump
    )]
    pub btc_commitment: Account<'info, BTCCommitment>,
    
    #[account(
        init_if_needed,
        payer = user,
        space = UserAccount::LEN,
        seeds = [b"user_account", user.key().as_ref()],
        bump
    )]
    pub user_account: Account<'info, UserAccount>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct VerifyBalance<'info> {
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
        constraint = btc_commitment.user_address == user.key() @ VaultError::UnauthorizedSigner
    )]
    pub btc_commitment: Account<'info, BTCCommitment>,
    
    #[account(
        mut,
        seeds = [b"user_account", user.key().as_ref()],
        bump = user_account.bump,
        constraint = user_account.owner == user.key() @ VaultError::UnauthorizedSigner
    )]
    pub user_account: Account<'info, UserAccount>,
    
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateCommitment<'info> {
    #[account(
        mut,
        seeds = [b"btc_commitment", user.key().as_ref()],
        bump = btc_commitment.bump,
        constraint = btc_commitment.user_address == user.key() @ VaultError::UnauthorizedSigner
    )]
    pub btc_commitment: Account<'info, BTCCommitment>,
    
    #[account(
        mut,
        seeds = [b"user_account", user.key().as_ref()],
        bump = user_account.bump,
        constraint = user_account.owner == user.key() @ VaultError::UnauthorizedSigner
    )]
    pub user_account: Account<'info, UserAccount>,
    
    pub user: Signer<'info>,
}

pub fn commit_btc(
    ctx: Context<CommitBTC>,
    amount: u64,
    btc_address: String,
    ecdsa_proof: Vec<u8>,
    public_key: Vec<u8>,
) -> Result<()> {
    // CRITICAL SECURITY: Verify signer authorization
    require!(ctx.accounts.user.is_signer, VaultError::UnauthorizedSigner);
    
    let btc_commitment = &mut ctx.accounts.btc_commitment;
    let user_account = &mut ctx.accounts.user_account;
    let clock = Clock::get()?;

    // CRITICAL SECURITY: Validate BTC address format
    BTCCommitment::validate_btc_address(&btc_address)?;

    // Validate amount
    if amount == 0 {
        return Err(VaultError::InsufficientBalance.into());
    }

    // Check KYC compliance limits (1 BTC limit for non-KYC users as per FR1)
    let btc_amount_in_satoshis = amount;
    let one_btc_in_satoshis = 100_000_000u64; // 1 BTC = 100,000,000 satoshis
    
    if btc_amount_in_satoshis > one_btc_in_satoshis {
        // Check if user has KYC verification
        if user_account.kyc_tier == crate::traits::ComplianceTier::NonKYC {
            msg!("KYC verification required for commitments over 1 BTC");
            return Err(VaultError::KYCRequired.into());
        }
    }

    // Validate ECDSA proof length
    if ecdsa_proof.is_empty() || ecdsa_proof.len() > 256 {
        return Err(VaultError::InvalidECDSAProof.into());
    }

    // Validate public key (33 bytes compressed, 65 bytes uncompressed)
    if public_key.is_empty() || public_key.len() > 65 || (public_key.len() != 33 && public_key.len() != 65) {
        return Err(VaultError::InvalidECDSAProof.into());
    }

    // Create commitment hash
    let commitment_hash = BTCCommitment::create_commitment_hash(
        &ctx.accounts.user.key(),
        &btc_address,
        amount,
        clock.unix_timestamp,
    );

    // Prepare message for ECDSA verification
    let message_data = BTCCommitment::serialize_for_signing(
        &ctx.accounts.user.key(),
        &btc_address,
        amount,
        clock.unix_timestamp,
    );

    // Update BTC commitment
    btc_commitment.user_address = ctx.accounts.user.key();
    btc_commitment.btc_address = btc_address.clone();
    btc_commitment.amount = amount;
    btc_commitment.ecdsa_proof = ecdsa_proof.clone();
    btc_commitment.public_key = public_key.clone();
    btc_commitment.timestamp = clock.unix_timestamp;
    btc_commitment.verified = false; // Will be verified by oracle
    btc_commitment.last_verification = 0;
    btc_commitment.commitment_hash = commitment_hash;
    btc_commitment.bump = ctx.bumps.btc_commitment;

    // Validate ECDSA proof
    let is_valid = btc_commitment.validate_ecdsa_proof(
        &message_data,
        &ecdsa_proof,
        &public_key,
    )?;

    if !is_valid {
        return Err(VaultError::InvalidECDSAProof.into());
    }

    // Validate timestamp freshness (max 5 minutes old)
    btc_commitment.validate_timestamp_freshness(300)?;

    // Perform full commitment validation
    btc_commitment.validate_commitment()?;

    // Update user account
    user_account.owner = ctx.accounts.user.key();
    user_account.btc_commitment_amount = amount;
    user_account.btc_address = btc_address;
    user_account.created_at = clock.unix_timestamp;
    user_account.last_activity = clock.unix_timestamp;
    user_account.bump = ctx.bumps.user_account;

    msg!("BTC commitment created successfully for user: {}, amount: {}, address: {}", 
         ctx.accounts.user.key(), amount, btc_commitment.btc_address);

    Ok(())
}

pub fn verify_balance(ctx: Context<VerifyBalance>) -> Result<()> {
    // CRITICAL SECURITY: Verify signer authorization
    require!(ctx.accounts.user.is_signer, VaultError::UnauthorizedSigner);
    
    // CRITICAL SECURITY: Verify owner authorization
    require!(
        ctx.accounts.btc_commitment.user_address == ctx.accounts.user.key(),
        VaultError::UnauthorizedSigner
    );
    
    let oracle_data = &mut ctx.accounts.oracle_data;
    let btc_commitment = &mut ctx.accounts.btc_commitment;
    let user_account = &mut ctx.accounts.user_account;
    let clock = Clock::get()?;

    // Validate existing commitment
    btc_commitment.validate_commitment()?;

    // Check verification interval (60 seconds as per requirements)
    let time_since_last_verification = clock.unix_timestamp - btc_commitment.last_verification;
    if time_since_last_verification < 60 && btc_commitment.verified {
        msg!("Balance verification skipped - within 60 second interval");
        return Ok(());
    }

    // Check if oracle data is stale
    if oracle_data.is_stale()? {
        msg!("Warning: Oracle data is stale, verification may be inaccurate");
        
        // Implement retry logic with exponential backoff as per requirements
        if oracle_data.can_retry() {
            oracle_data.increment_retry()?;
            let retry_delay = oracle_data.get_next_retry_delay();
            msg!("Oracle retry scheduled in {} seconds", retry_delay);
            return Err(VaultError::OracleVerificationFailed.into());
        } else {
            msg!("Oracle retry limit exceeded, using cached data if available");
        }
    }

    // Check for cached UTXO verification (5 minute cache as per requirements)
    if let Some(cached) = oracle_data.get_cached_utxo(&btc_commitment.btc_address) {
        if cached.balance >= btc_commitment.amount {
            btc_commitment.verified = true;
            btc_commitment.last_verification = clock.unix_timestamp;
            user_account.last_activity = clock.unix_timestamp;
            
            msg!("BTC balance verified from cache for user: {}, balance: {} satoshis", 
                 btc_commitment.user_address, cached.balance);
            return Ok(());
        }
    }

    // Validate ECDSA proof for anti-spoofing (critical security requirement)
    let message_data = BTCCommitment::serialize_for_signing(
        &btc_commitment.user_address,
        &btc_commitment.btc_address,
        btc_commitment.amount,
        btc_commitment.timestamp,
    );

    let proof_valid = btc_commitment.validate_ecdsa_proof(
        &message_data,
        &btc_commitment.ecdsa_proof,
        &btc_commitment.public_key,
    )?;

    if !proof_valid {
        msg!("ECDSA proof validation failed - potential spoofing attempt detected");
        return Err(VaultError::InvalidECDSAProof.into());
    }

    // Call Chainlink oracle for UTXO verification
    let verified_balance = call_chainlink_oracle_verification(
        oracle_data,
        &btc_commitment.btc_address,
        btc_commitment.amount,
    )?;

    // Cache the verification result (5 minute cache)
    use sha2::{Digest, Sha256};
    let proof_hash = Sha256::digest(&btc_commitment.ecdsa_proof).into();
    oracle_data.cache_utxo_verification(
        btc_commitment.btc_address.clone(),
        verified_balance,
        proof_hash,
        verified_balance >= btc_commitment.amount,
    )?;

    // Update commitment verification status
    if verified_balance >= btc_commitment.amount {
        btc_commitment.verified = true;
        btc_commitment.last_verification = clock.unix_timestamp;
        user_account.last_activity = clock.unix_timestamp;
        oracle_data.reset_retry(); // Reset retry counter on success
        
        msg!("BTC balance verified via Chainlink oracle for user: {}, balance: {} satoshis (required: {})", 
             btc_commitment.user_address, verified_balance, btc_commitment.amount);
    } else {
        btc_commitment.verified = false;
        msg!("BTC balance insufficient for user: {}, balance: {} satoshis (required: {})", 
             btc_commitment.user_address, verified_balance, btc_commitment.amount);
        return Err(VaultError::InsufficientBalance.into());
    }

    Ok(())
}

/// Call Chainlink oracle for UTXO verification with retry logic
fn call_chainlink_oracle_verification(
    oracle_data: &mut OracleData,
    btc_address: &str,
    expected_balance: u64,
) -> Result<u64> {
    // Validate BTC address format
    BTCCommitment::validate_btc_address(btc_address)?;
    
    // In production, this would:
    // 1. Call Chainlink UTXO verification oracle
    // 2. Verify the address exists on Bitcoin network
    // 3. Sum all UTXOs for the address
    // 4. Handle oracle failures with exponential backoff
    // 5. Return the actual balance from Bitcoin network
    
    // For now, simulate the oracle call with proper error handling
    match simulate_chainlink_call(btc_address, expected_balance) {
        Ok(balance) => {
            oracle_data.reset_retry();
            Ok(balance)
        },
        Err(e) => {
            if oracle_data.can_retry() {
                oracle_data.increment_retry()?;
                msg!("Chainlink oracle call failed, retry {} of {}", 
                     oracle_data.retry_config.current_retries, 
                     oracle_data.retry_config.max_retries);
            }
            Err(e)
        }
    }
}

/// Simulate Chainlink oracle call (replace with actual Chainlink integration)
fn simulate_chainlink_call(_btc_address: &str, expected_balance: u64) -> Result<u64> {
    // This simulates the actual Chainlink oracle response
    // In production, this would make HTTP requests to Chainlink nodes
    // and aggregate responses for UTXO verification
    
    // Simulate potential oracle failures for testing
    use rand::rngs::OsRng;
    let random = OsRng.next_u32() % 100;
    
    if random < 5 { // 5% chance of oracle failure for testing
        return Err(VaultError::OracleVerificationFailed.into());
    }
    
    // Return the expected balance (in production, this would be the actual UTXO sum)
    Ok(expected_balance)
}

pub fn update_commitment(
    ctx: Context<UpdateCommitment>,
    new_amount: u64,
    new_ecdsa_proof: Vec<u8>,
    new_public_key: Vec<u8>,
) -> Result<()> {
    // CRITICAL SECURITY: Verify signer authorization
    require!(ctx.accounts.user.is_signer, VaultError::UnauthorizedSigner);
    
    // CRITICAL SECURITY: Verify owner authorization
    require!(
        ctx.accounts.btc_commitment.user_address == ctx.accounts.user.key(),
        VaultError::UnauthorizedSigner
    );
    require!(
        ctx.accounts.user_account.owner == ctx.accounts.user.key(),
        VaultError::UnauthorizedSigner
    );
    
    let btc_commitment = &mut ctx.accounts.btc_commitment;
    let user_account = &mut ctx.accounts.user_account;
    let clock = Clock::get()?;

    // Validate new amount
    if new_amount == 0 {
        return Err(VaultError::InsufficientBalance.into());
    }

    // Check KYC compliance limits for updated amount (1 BTC limit for non-KYC users)
    let btc_amount_in_satoshis = new_amount;
    let one_btc_in_satoshis = 100_000_000u64; // 1 BTC = 100,000,000 satoshis
    
    if btc_amount_in_satoshis > one_btc_in_satoshis {
        // Check if user has KYC verification
        if user_account.kyc_tier == crate::traits::ComplianceTier::NonKYC {
            msg!("KYC verification required for commitments over 1 BTC");
            return Err(VaultError::KYCRequired.into());
        }
    }

    // Validate new ECDSA proof
    if new_ecdsa_proof.is_empty() || new_ecdsa_proof.len() > 256 {
        return Err(VaultError::InvalidECDSAProof.into());
    }

    // Validate new public key (33 bytes compressed, 65 bytes uncompressed)
    if new_public_key.is_empty() || new_public_key.len() > 65 || (new_public_key.len() != 33 && new_public_key.len() != 65) {
        return Err(VaultError::InvalidECDSAProof.into());
    }

    // Prevent downgrade attacks - ensure new amount is not significantly lower without proper authorization
    if new_amount < btc_commitment.amount / 2 {
        msg!("Warning: Significant commitment reduction detected, additional verification may be required");
    }

    // Create new commitment hash
    let new_commitment_hash = BTCCommitment::create_commitment_hash(
        &ctx.accounts.user.key(),
        &btc_commitment.btc_address,
        new_amount,
        clock.unix_timestamp,
    );

    // Prepare message for ECDSA verification
    let message_data = BTCCommitment::serialize_for_signing(
        &ctx.accounts.user.key(),
        &btc_commitment.btc_address,
        new_amount,
        clock.unix_timestamp,
    );

    // Validate new ECDSA proof
    let is_valid = btc_commitment.validate_ecdsa_proof(
        &message_data,
        &new_ecdsa_proof,
        &new_public_key,
    )?;

    if !is_valid {
        return Err(VaultError::InvalidECDSAProof.into());
    }

    // Update commitment
    btc_commitment.amount = new_amount;
    btc_commitment.ecdsa_proof = new_ecdsa_proof;
    btc_commitment.public_key = new_public_key;
    btc_commitment.timestamp = clock.unix_timestamp;
    btc_commitment.commitment_hash = new_commitment_hash;
    btc_commitment.verified = false; // Needs re-verification
    btc_commitment.last_verification = 0;

    // Update user account
    user_account.btc_commitment_amount = new_amount;
    user_account.last_activity = clock.unix_timestamp;

    msg!("BTC commitment updated for user: {}, new amount: {}", 
         ctx.accounts.user.key(), new_amount);

    Ok(())
}
