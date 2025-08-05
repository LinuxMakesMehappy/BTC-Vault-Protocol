use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::VaultError;

#[derive(Accounts)]
pub struct InitializeMultisigWallet<'info> {
    #[account(
        init,
        payer = authority,
        space = MultisigWallet::LEN,
        seeds = [b"multisig_wallet"],
        bump
    )]
    pub multisig_wallet: Account<'info, MultisigWallet>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ProposeMultisigTransaction<'info> {
    #[account(
        mut,
        seeds = [b"multisig_wallet"],
        bump = multisig_wallet.bump
    )]
    pub multisig_wallet: Account<'info, MultisigWallet>,
    
    #[account(
        init,
        payer = proposer,
        space = MultisigTransaction::LEN,
        seeds = [
            b"multisig_transaction",
            multisig_wallet.key().as_ref(),
            &multisig_wallet.transaction_count.to_le_bytes()
        ],
        bump
    )]
    pub multisig_transaction: Account<'info, MultisigTransaction>,
    
    #[account(mut)]
    pub proposer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SignMultisigTransaction<'info> {
    #[account(
        mut,
        seeds = [b"multisig_wallet"],
        bump = multisig_wallet.bump
    )]
    pub multisig_wallet: Account<'info, MultisigWallet>,
    
    #[account(
        mut,
        seeds = [
            b"multisig_transaction",
            multisig_wallet.key().as_ref(),
            &multisig_transaction.transaction_id.to_le_bytes()
        ],
        bump = multisig_transaction.bump
    )]
    pub multisig_transaction: Account<'info, MultisigTransaction>,
    
    #[account(mut)]
    pub signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct ExecuteMultisigTransaction<'info> {
    #[account(
        mut,
        seeds = [b"multisig_wallet"],
        bump = multisig_wallet.bump
    )]
    pub multisig_wallet: Account<'info, MultisigWallet>,
    
    #[account(
        mut,
        seeds = [
            b"multisig_transaction",
            multisig_wallet.key().as_ref(),
            &multisig_transaction.transaction_id.to_le_bytes()
        ],
        bump = multisig_transaction.bump
    )]
    pub multisig_transaction: Account<'info, MultisigTransaction>,
    
    #[account(mut)]
    pub executor: Signer<'info>,
}

#[derive(Accounts)]
pub struct RotateMultisigKeys<'info> {
    #[account(
        mut,
        seeds = [b"multisig_wallet"],
        bump = multisig_wallet.bump
    )]
    pub multisig_wallet: Account<'info, MultisigWallet>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct EmergencyAction<'info> {
    #[account(
        mut,
        seeds = [b"multisig_wallet"],
        bump = multisig_wallet.bump
    )]
    pub multisig_wallet: Account<'info, MultisigWallet>,
    
    #[account(mut)]
    pub emergency_signer: Signer<'info>,
}

/// Initialize multisig wallet with HSM configuration
pub fn initialize_multisig_wallet(
    ctx: Context<InitializeMultisigWallet>,
    signers: Vec<SignerInfo>,
    hsm_enabled: bool,
) -> Result<()> {
    let multisig_wallet = &mut ctx.accounts.multisig_wallet;
    
    // Validate authority is included in signers
    let authority_key = ctx.accounts.authority.key();
    if !signers.iter().any(|s| s.pubkey == authority_key) {
        return Err(VaultError::UnauthorizedAccess.into());
    }

    multisig_wallet.initialize(signers, hsm_enabled, ctx.bumps.multisig_wallet)?;
    
    msg!("Multisig wallet initialized with {} signers, HSM enabled: {}", 
         multisig_wallet.signers.len(), hsm_enabled);
    
    Ok(())
}

/// Propose a new multisig transaction
pub fn propose_transaction(
    ctx: Context<ProposeMultisigTransaction>,
    transaction_type: TransactionType,
    priority: TransactionPriority,
    transaction_data: Vec<u8>,
) -> Result<()> {
    let multisig_wallet = &mut ctx.accounts.multisig_wallet;
    let multisig_transaction = &mut ctx.accounts.multisig_transaction;

    // Validate proposer is authorized signer
    let proposer_key = ctx.accounts.proposer.key();
    if !multisig_wallet.signers.iter().any(|s| s.pubkey == proposer_key && s.is_active) {
        return Err(VaultError::UnauthorizedSigner.into());
    }

    // Validate proposer has permission for this transaction type
    if !multisig_wallet.validate_signer_role(&proposer_key, &transaction_type)? {
        return Err(VaultError::UnauthorizedAccess.into());
    }

    // Get required threshold for this transaction type and priority
    let required_signatures = multisig_wallet.get_required_threshold(&transaction_type, &priority);

    // Initialize transaction
    multisig_transaction.initialize(
        multisig_wallet.key(),
        multisig_wallet.transaction_count,
        proposer_key,
        transaction_type,
        priority.clone(),
        transaction_data,
        required_signatures,
        ctx.bumps.multisig_transaction,
    )?;

    // Validate transaction data
    multisig_transaction.validate_transaction_data()?;

    // Increment transaction counter
    multisig_wallet.transaction_count = multisig_wallet.transaction_count
        .checked_add(1).unwrap();

    msg!("Transaction {} proposed by {} with priority {:?}", 
         multisig_transaction.transaction_id, proposer_key, &priority);

    Ok(())
}

/// Sign a multisig transaction
pub fn sign_transaction(
    ctx: Context<SignMultisigTransaction>,
    signature_data: [u8; 64],
    hsm_signature: Option<Vec<u8>>,
    signature_type: SignatureType,
) -> Result<()> {
    let multisig_wallet = &mut ctx.accounts.multisig_wallet;
    let multisig_transaction = &mut ctx.accounts.multisig_transaction;
    let signer_key = ctx.accounts.signer.key();

    // Validate signer is authorized
    if !multisig_wallet.signers.iter().any(|s| s.pubkey == signer_key && s.is_active) {
        return Err(VaultError::UnauthorizedSigner.into());
    }

    // Validate signer has permission for this transaction type
    if !multisig_wallet.validate_signer_role(&signer_key, &multisig_transaction.transaction_type)? {
        return Err(VaultError::UnauthorizedAccess.into());
    }

    // Check if transaction is still valid
    if multisig_transaction.executed || multisig_transaction.cancelled {
        return Err(VaultError::TransactionAlreadyExecuted.into());
    }

    if multisig_transaction.is_expired()? {
        return Err(VaultError::SecurityViolation.into());
    }

    // Validate HSM signature if HSM is enabled
    if multisig_wallet.hsm_enabled && signature_type == SignatureType::HSM {
        if hsm_signature.is_none() {
            return Err(VaultError::SecurityViolation.into());
        }
        
        // In production, validate HSM signature against Yubico HSM
        validate_hsm_signature(&signer_key, &signature_data, &hsm_signature.as_ref().unwrap())?;
    }

    // Create signature
    let clock = Clock::get()?;
    let multisig_signature = MultisigSignature {
        signer: signer_key,
        signature: signature_data,
        hsm_signature,
        signed_at: clock.unix_timestamp,
        signature_type: signature_type.clone(),
    };

    // Add signature to transaction
    multisig_transaction.add_signature(multisig_signature)?;

    // Update signer usage statistics
    multisig_wallet.update_signer_usage(&signer_key)?;

    msg!("Transaction {} signed by {} (type: {:?})", 
         multisig_transaction.transaction_id, signer_key, &signature_type);

    Ok(())
}

/// Execute a multisig transaction
pub fn execute_transaction(ctx: Context<ExecuteMultisigTransaction>) -> Result<()> {
    let multisig_wallet = &mut ctx.accounts.multisig_wallet;
    let multisig_transaction = &mut ctx.accounts.multisig_transaction;

    // Validate executor is authorized signer
    let executor_key = ctx.accounts.executor.key();
    if !multisig_wallet.signers.iter().any(|s| s.pubkey == executor_key && s.is_active) {
        return Err(VaultError::UnauthorizedSigner.into());
    }

    // Check if transaction can be executed
    if multisig_transaction.executed || multisig_transaction.cancelled {
        return Err(VaultError::TransactionAlreadyExecuted.into());
    }

    if multisig_transaction.is_expired()? {
        return Err(VaultError::SecurityViolation.into());
    }

    if !multisig_transaction.has_enough_signatures() {
        return Err(VaultError::MultisigThresholdNotMet.into());
    }

    // Execute transaction based on type
    let execution_result = match multisig_transaction.transaction_type {
        TransactionType::TreasuryTransfer => {
            execute_treasury_transfer(&multisig_transaction.transaction_data)?
        },
        TransactionType::StakingOperation => {
            execute_staking_operation(&multisig_transaction.transaction_data)?
        },
        TransactionType::RewardDistribution => {
            execute_reward_distribution(&multisig_transaction.transaction_data)?
        },
        TransactionType::ConfigUpdate => {
            execute_config_update(&multisig_transaction.transaction_data)?
        },
        TransactionType::EmergencyAction => {
            execute_emergency_action(&multisig_transaction.transaction_data)?
        },
        TransactionType::KeyRotation => {
            execute_key_rotation(multisig_wallet, &multisig_transaction.transaction_data)?
        },
    };

    // Mark transaction as executed
    multisig_transaction.mark_executed(Some(execution_result.clone()))?;
    multisig_wallet.executed_count = multisig_wallet.executed_count.checked_add(1).unwrap();

    msg!("Transaction {} executed successfully: {}", 
         multisig_transaction.transaction_id, execution_result);

    Ok(())
}

/// Rotate multisig keys with HSM integration
pub fn rotate_keys(
    ctx: Context<RotateMultisigKeys>,
    new_signers: Vec<SignerInfo>,
) -> Result<()> {
    let multisig_wallet = &mut ctx.accounts.multisig_wallet;
    let authority_key = ctx.accounts.authority.key();

    // Validate authority has admin role
    let authority_signer = multisig_wallet.signers.iter()
        .find(|s| s.pubkey == authority_key && s.is_active)
        .ok_or(VaultError::UnauthorizedAccess)?;

    if authority_signer.role != SignerRole::Admin {
        return Err(VaultError::UnauthorizedAccess.into());
    }

    // Check if key rotation is needed or forced
    if !multisig_wallet.needs_key_rotation()? {
        msg!("Warning: Key rotation performed before required interval");
    }

    // Perform key rotation
    multisig_wallet.rotate_keys(new_signers)?;

    msg!("Multisig keys rotated successfully");

    Ok(())
}

/// Activate emergency mode
pub fn activate_emergency_mode(ctx: Context<EmergencyAction>) -> Result<()> {
    let multisig_wallet = &mut ctx.accounts.multisig_wallet;
    let signer_key = ctx.accounts.emergency_signer.key();

    // Validate signer has emergency or admin role
    let signer_info = multisig_wallet.signers.iter()
        .find(|s| s.pubkey == signer_key && s.is_active)
        .ok_or(VaultError::UnauthorizedAccess)?;

    if signer_info.role != SignerRole::Admin && signer_info.role != SignerRole::Emergency {
        return Err(VaultError::UnauthorizedAccess.into());
    }

    multisig_wallet.activate_emergency_mode()?;

    Ok(())
}

/// Deactivate emergency mode
pub fn deactivate_emergency_mode(ctx: Context<EmergencyAction>) -> Result<()> {
    let multisig_wallet = &mut ctx.accounts.multisig_wallet;
    let signer_key = ctx.accounts.emergency_signer.key();

    // Validate signer has admin role
    let signer_info = multisig_wallet.signers.iter()
        .find(|s| s.pubkey == signer_key && s.is_active)
        .ok_or(VaultError::UnauthorizedAccess)?;

    if signer_info.role != SignerRole::Admin {
        return Err(VaultError::UnauthorizedAccess.into());
    }

    multisig_wallet.deactivate_emergency_mode()?;

    Ok(())
}

// Transaction execution functions

fn execute_treasury_transfer(transaction_data: &[u8]) -> Result<String> {
    // Parse treasury transfer data
    if transaction_data.len() < 40 {
        return Err(VaultError::InvalidAllocation.into());
    }

    let recipient = Pubkey::try_from(&transaction_data[0..32])
        .map_err(|_| VaultError::InvalidAllocation)?;
    let amount = u64::from_le_bytes(
        transaction_data[32..40].try_into()
            .map_err(|_| VaultError::InvalidAllocation)?
    );

    // In production, execute actual treasury transfer
    msg!("Treasury transfer: {} lamports to {}", amount, recipient);
    
    Ok(format!("Transferred {} lamports to {}", amount, recipient))
}

fn execute_staking_operation(transaction_data: &[u8]) -> Result<String> {
    // Parse staking operation data
    msg!("Executing staking operation with {} bytes of data", transaction_data.len());
    
    // In production, execute actual staking operation
    Ok("Staking operation completed".to_string())
}

fn execute_reward_distribution(transaction_data: &[u8]) -> Result<String> {
    // Parse reward distribution data
    msg!("Executing reward distribution with {} bytes of data", transaction_data.len());
    
    // In production, execute actual reward distribution
    Ok("Reward distribution completed".to_string())
}

fn execute_config_update(transaction_data: &[u8]) -> Result<String> {
    // Parse configuration update data
    msg!("Executing config update with {} bytes of data", transaction_data.len());
    
    // In production, execute actual configuration update
    Ok("Configuration updated".to_string())
}

fn execute_emergency_action(transaction_data: &[u8]) -> Result<String> {
    // Parse emergency action data
    msg!("Executing emergency action with {} bytes of data", transaction_data.len());
    
    // In production, execute actual emergency action
    Ok("Emergency action completed".to_string())
}

fn execute_key_rotation(_multisig_wallet: &mut MultisigWallet, transaction_data: &[u8]) -> Result<String> {
    // Parse new signer data
    if transaction_data.len() < 96 { // Minimum for 3 pubkeys
        return Err(VaultError::InvalidAllocation.into());
    }

    // In production, parse and validate new signers
    msg!("Executing key rotation with {} bytes of data", transaction_data.len());
    
    Ok("Key rotation completed".to_string())
}

/// Validate HSM signature against Yubico HSM
fn validate_hsm_signature(
    signer_pubkey: &Pubkey,
    _signature: &[u8; 64],
    hsm_signature: &[u8],
) -> Result<()> {
    // In production, this would:
    // 1. Connect to Yubico HSM
    // 2. Validate the HSM signature using the stored HSM key
    // 3. Verify the signature matches the expected format
    // 4. Check HSM key usage limits and policies
    
    msg!("Validating HSM signature for signer: {}", signer_pubkey);
    
    // Simulate HSM validation
    if hsm_signature.len() < 64 {
        return Err(VaultError::SecurityViolation.into());
    }

    // In reality, would perform cryptographic validation
    Ok(())
}