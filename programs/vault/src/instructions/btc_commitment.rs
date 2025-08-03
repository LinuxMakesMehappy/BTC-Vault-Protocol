use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::VaultError;

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

    // Validate ECDSA proof length
    if ecdsa_proof.is_empty() || ecdsa_proof.len() > 256 {
        return Err(VaultError::InvalidECDSAProof.into());
    }

    // Validate public key
    if public_key.is_empty() || public_key.len() > 65 {
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

    // Check if oracle data is stale
    if oracle_data.is_stale()? {
        msg!("Warning: Oracle data is stale, verification may be inaccurate");
    }

    // Check for cached UTXO verification
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

    // Validate ECDSA proof for anti-spoofing
    let proof_valid = oracle_data.validate_ecdsa_proof(
        &btc_commitment.btc_address,
        btc_commitment.amount,
        &btc_commitment.ecdsa_proof,
    )?;

    if !proof_valid {
        return Err(VaultError::InvalidECDSAProof.into());
    }

    // In production, this would call Chainlink oracle for UTXO verification
    // For now, we simulate the verification process
    let verified_balance = simulate_utxo_verification(
        &btc_commitment.btc_address,
        btc_commitment.amount,
    )?;

    // Cache the verification result
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
        
        msg!("BTC balance verified for user: {}, balance: {} satoshis (required: {})", 
             btc_commitment.user_address, verified_balance, btc_commitment.amount);
    } else {
        btc_commitment.verified = false;
        msg!("BTC balance insufficient for user: {}, balance: {} satoshis (required: {})", 
             btc_commitment.user_address, verified_balance, btc_commitment.amount);
    }

    Ok(())
}

/// Simulate UTXO verification (in production, this would call Chainlink oracle)
fn simulate_utxo_verification(btc_address: &str, expected_balance: u64) -> Result<u64> {
    // Validate BTC address format
    BTCCommitment::validate_btc_address(btc_address)?;
    
    // In production, this would:
    // 1. Call Chainlink UTXO verification oracle
    // 2. Verify the address exists on Bitcoin network
    // 3. Sum all UTXOs for the address
    // 4. Return the total balance with proper error handling
    
    // For simulation, we assume the balance is valid if address format is correct
    // and return the expected balance (in production, this would be the actual balance)
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

    // Validate new ECDSA proof
    if new_ecdsa_proof.is_empty() || new_ecdsa_proof.len() > 256 {
        return Err(VaultError::InvalidECDSAProof.into());
    }

    // Validate new public key
    if new_public_key.is_empty() || new_public_key.len() > 65 {
        return Err(VaultError::InvalidECDSAProof.into());
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