use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::state::*;
use crate::errors::VaultError;

#[derive(Accounts)]
pub struct InitializePaymentSystem<'info> {
    #[account(
        init,
        payer = authority,
        space = PaymentSystem::LEN,
        seeds = [b\"payment_system\"],
        bump
    )]
    pub payment_system: Account<'info, PaymentSystem>,
    
    #[account(
        seeds = [b\"multisig_wallet\"],
        bump = multisig_wallet.bump
    )]
    pub multisig_wallet: Account<'info, MultisigWallet>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeUserPreferences<'info> {
    #[account(
        init,
        payer = user,
        space = UserPaymentPreferences::LEN,
        seeds = [b\"user_preferences\", user.key().as_ref()],
        bump
    )]
    pub user_preferences: Account<'info, UserPaymentPreferences>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreatePaymentRequest<'info> {
    #[account(
        mut,
        seeds = [b\"payment_system\"],
        bump = payment_system.bump
    )]
    pub payment_system: Account<'info, PaymentSystem>,
    
    #[account(
        seeds = [b\"user_preferences\", user.key().as_ref()],
        bump = user_preferences.bump
    )]
    pub user_preferences: Account<'info, UserPaymentPreferences>,
    
    #[account(
        mut,
        seeds = [b\"rewards\", user.key().as_ref()],
        bump = user_rewards.bump
    )]
    pub user_rewards: Account<'info, UserRewards>,
    
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct ProcessPayment<'info> {
    #[account(
        mut,
        seeds = [b\"payment_system\"],
        bump = payment_system.bump
    )]
    pub payment_system: Account<'info, PaymentSystem>,
    
    #[account(
        mut,
        seeds = [b\"treasury\"],
        bump = treasury.bump
    )]
    pub treasury: Account<'info, Treasury>,
    
    /// USDC token accounts (optional, only for USDC payments)
    #[account(mut)]
    pub treasury_usdc_ata: Option<Account<'info, TokenAccount>>,
    
    #[account(mut)]
    pub recipient_usdc_ata: Option<Account<'info, TokenAccount>>,
    
    #[account(mut)]
    pub processor: Signer<'info>,
    pub token_program: Option<Program<'info, Token>>,
}

#[derive(Accounts)]
pub struct ApprovePayment<'info> {
    #[account(
        mut,
        seeds = [b\"payment_system\"],
        bump = payment_system.bump
    )]
    pub payment_system: Account<'info, PaymentSystem>,
    
    #[account(
        mut,
        seeds = [b\"multisig_wallet\"],
        bump = multisig_wallet.bump
    )]
    pub multisig_wallet: Account<'info, MultisigWallet>,
    
    #[account(mut)]
    pub approver: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdatePaymentConfig<'info> {
    #[account(
        mut,
        seeds = [b\"payment_system\"],
        bump = payment_system.bump
    )]
    pub payment_system: Account<'info, PaymentSystem>,
    
    #[account(
        mut,
        seeds = [b\"multisig_wallet\"],
        bump = multisig_wallet.bump
    )]
    pub multisig_wallet: Account<'info, MultisigWallet>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateUserPreferences<'info> {
    #[account(
        mut,
        seeds = [b\"user_preferences\", user.key().as_ref()],
        bump = user_preferences.bump
    )]
    pub user_preferences: Account<'info, UserPaymentPreferences>,
    
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct ProcessReinvestment<'info> {
    #[account(
        mut,
        seeds = [b\"payment_system\"],
        bump = payment_system.bump
    )]
    pub payment_system: Account<'info, PaymentSystem>,
    
    #[account(
        mut,
        seeds = [b\"user_preferences\", user.key().as_ref()],
        bump = user_preferences.bump
    )]
    pub user_preferences: Account<'info, UserPaymentPreferences>,
    
    #[account(
        mut,
        seeds = [b\"rewards\", user.key().as_ref()],
        bump = user_rewards.bump
    )]
    pub user_rewards: Account<'info, UserRewards>,
    
    #[account(
        mut,
        seeds = [b\"staking_pool\"],
        bump = staking_pool.bump
    )]
    pub staking_pool: Account<'info, StakingPool>,
    
    /// CHECK: User account for reinvestment
    pub user: AccountInfo<'info>,
}

/// Initialize the payment system with Lightning and USDC configurations
pub fn initialize_payment_system(
    ctx: Context<InitializePaymentSystem>,
    lightning_config: LightningConfig,
    usdc_config: UsdcConfig,
) -> Result<()> {
    let payment_system = &mut ctx.accounts.payment_system;
    let multisig_wallet = ctx.accounts.multisig_wallet.key();
    
    payment_system.initialize(
        lightning_config,
        usdc_config,
        multisig_wallet,
        ctx.bumps.payment_system,
    )?;
    
    msg!(\"Payment system initialized with Lightning and USDC support\");
    
    Ok(())
}

/// Initialize user payment preferences
pub fn initialize_user_preferences(
    ctx: Context<InitializeUserPreferences>,
    default_method: PaymentMethod,
) -> Result<()> {
    let user_preferences = &mut ctx.accounts.user_preferences;
    let user = ctx.accounts.user.key();
    
    user_preferences.initialize(
        user,
        default_method,
        ctx.bumps.user_preferences,
    )?;
    
    msg!(\"User payment preferences initialized for {}\", user);
    
    Ok(())
}

/// Create a payment request for reward distribution
pub fn create_payment_request(
    ctx: Context<CreatePaymentRequest>,
    method: Option<PaymentMethod>,
    amount: u64,
    destination: String,
) -> Result<()> {
    let payment_system = &mut ctx.accounts.payment_system;
    let user_preferences = &ctx.accounts.user_preferences;
    let user_rewards = &mut ctx.accounts.user_rewards;
    let user = ctx.accounts.user.key();
    
    // Verify user has sufficient rewards
    if user_rewards.pending_rewards < amount {
        return Err(VaultError::InsufficientRewards.into());
    }
    
    // Use provided method or user's default
    let payment_method = method.unwrap_or(user_preferences.default_method.clone());
    
    // Validate destination based on method and user preferences
    let final_destination = match payment_method {
        PaymentMethod::Lightning => {
            if destination.is_empty() {
                user_preferences.lightning_address.clone()
                    .ok_or(VaultError::NoPaymentDestination)?
            } else {
                destination
            }
        },
        PaymentMethod::USDC => {
            if destination.is_empty() {
                user_preferences.usdc_address
                    .ok_or(VaultError::NoPaymentDestination)?
                    .to_string()
            } else {
                destination
            }
        },
    };
    
    // Create payment request
    let payment_id = payment_system.create_payment_request(
        user,
        payment_method,
        amount,
        final_destination,
    )?;
    
    // Deduct from pending rewards
    user_rewards.pending_rewards = user_rewards.pending_rewards
        .checked_sub(amount).unwrap();
    user_rewards.last_claim_request = Clock::get()?.unix_timestamp;
    
    msg!(\"Payment request {} created for user {} (amount: {})\", 
         payment_id, user, amount);
    
    Ok(())
}

/// Process a payment request (Lightning or USDC)
pub fn process_payment(
    ctx: Context<ProcessPayment>,
    payment_id: u64,
) -> Result<()> {
    let payment_system = &mut ctx.accounts.payment_system;
    let treasury = &mut ctx.accounts.treasury;
    
    // Get payment request
    let payment = payment_system.get_payment_request(payment_id)
        .ok_or(VaultError::PaymentNotFound)?
        .clone();
    
    // Verify payment is ready for processing
    if payment.status != PaymentStatus::Pending && payment.status != PaymentStatus::Processing {
        return Err(VaultError::InvalidPaymentStatus.into());
    }
    
    // Process based on payment method
    match payment.method {
        PaymentMethod::Lightning => {
            process_lightning_payment(payment_system, &payment)?;
        },
        PaymentMethod::USDC => {
            process_usdc_payment(
                ctx.accounts.treasury_usdc_ata.as_ref()
                    .ok_or(VaultError::MissingTokenAccount)?,
                ctx.accounts.recipient_usdc_ata.as_ref()
                    .ok_or(VaultError::MissingTokenAccount)?,
                ctx.accounts.token_program.as_ref()
                    .ok_or(VaultError::MissingTokenProgram)?,
                &payment,
                treasury,
            )?;
        },
    }
    
    // Mark payment as processing
    payment_system.process_payment(payment_id)?;
    
    Ok(())
}

/// Approve a large payment that requires multisig
pub fn approve_payment(
    ctx: Context<ApprovePayment>,
    payment_id: u64,
) -> Result<()> {
    let payment_system = &mut ctx.accounts.payment_system;
    let multisig_wallet = &ctx.accounts.multisig_wallet;
    let approver = ctx.accounts.approver.key();
    
    // Verify approver is a multisig owner
    if !multisig_wallet.signers.iter().any(|s| s.pubkey == approver && s.is_active) {
        return Err(VaultError::UnauthorizedSigner.into());
    }
    
    // Get payment request
    let payment = payment_system.get_payment_request(payment_id)
        .ok_or(VaultError::PaymentNotFound)?;
    
    if !payment.multisig_required {
        return Err(VaultError::PaymentDoesNotRequireApproval.into());
    }
    
    if payment.status != PaymentStatus::Pending {
        return Err(VaultError::InvalidPaymentStatus.into());
    }
    
    // In production, this would integrate with the multisig system
    // For now, we simulate approval
    msg!(\"Payment {} approved by multisig signer {}\", payment_id, approver);
    
    // Mark payment as ready for processing
    payment_system.process_payment(payment_id)?;
    
    Ok(())
}

/// Complete a payment (success or failure)
pub fn complete_payment(
    ctx: Context<ProcessPayment>,
    payment_id: u64,
    success: bool,
    failure_reason: Option<String>,
) -> Result<()> {
    let payment_system = &mut ctx.accounts.payment_system;
    
    payment_system.complete_payment(payment_id, success, failure_reason)?;
    
    if success {
        msg!(\"Payment {} completed successfully\", payment_id);
    } else {
        msg!(\"Payment {} failed\", payment_id);
    }
    
    Ok(())
}

/// Cancel a payment request
pub fn cancel_payment(
    ctx: Context<CreatePaymentRequest>,
    payment_id: u64,
) -> Result<()> {
    let payment_system = &mut ctx.accounts.payment_system;
    let user_rewards = &mut ctx.accounts.user_rewards;
    let user = ctx.accounts.user.key();
    
    // Get payment details before cancellation
    let payment = payment_system.get_payment_request(payment_id)
        .ok_or(VaultError::PaymentNotFound)?
        .clone();
    
    if payment.user != user {
        return Err(VaultError::UnauthorizedAccess.into());
    }
    
    // Cancel the payment
    payment_system.cancel_payment(payment_id, user)?;
    
    // Refund the amount to pending rewards
    user_rewards.pending_rewards = user_rewards.pending_rewards
        .checked_add(payment.amount).unwrap();
    
    msg!(\"Payment {} cancelled and amount refunded to user {}\", payment_id, user);
    
    Ok(())
}

/// Update Lightning Network configuration
pub fn update_lightning_config(
    ctx: Context<UpdatePaymentConfig>,
    config: LightningConfig,
) -> Result<()> {
    let payment_system = &mut ctx.accounts.payment_system;
    let multisig_wallet = &ctx.accounts.multisig_wallet;
    let authority = ctx.accounts.authority.key();
    
    // Verify authority is a multisig owner with admin role
    if !multisig_wallet.signers.iter().any(|s| s.pubkey == authority && s.is_active) {
        return Err(VaultError::UnauthorizedAccess.into());
    }
    
    payment_system.update_lightning_config(config)?;
    
    Ok(())
}

/// Update USDC configuration
pub fn update_usdc_config(
    ctx: Context<UpdatePaymentConfig>,
    config: UsdcConfig,
) -> Result<()> {
    let payment_system = &mut ctx.accounts.payment_system;
    let multisig_wallet = &ctx.accounts.multisig_wallet;
    let authority = ctx.accounts.authority.key();
    
    // Verify authority is a multisig owner with admin role
    if !multisig_wallet.signers.iter().any(|s| s.pubkey == authority && s.is_active) {
        return Err(VaultError::UnauthorizedAccess.into());
    }
    
    payment_system.update_usdc_config(config)?;
    
    Ok(())
}

/// Update user payment preferences
pub fn update_user_preferences(
    ctx: Context<UpdateUserPreferences>,
    default_method: Option<PaymentMethod>,
    lightning_address: Option<String>,
    usdc_address: Option<Pubkey>,
    reinvestment_config: Option<ReinvestmentConfig>,
) -> Result<()> {
    let user_preferences = &mut ctx.accounts.user_preferences;
    
    if let Some(method) = default_method {
        user_preferences.update_default_method(method)?;
    }
    
    if lightning_address.is_some() {
        user_preferences.update_lightning_address(lightning_address)?;
    }
    
    if usdc_address.is_some() {
        user_preferences.update_usdc_address(usdc_address)?;
    }
    
    if let Some(config) = reinvestment_config {
        user_preferences.update_reinvestment_config(config)?;
    }
    
    msg!(\"User preferences updated for {}\", user_preferences.user);
    
    Ok(())
}

/// Process automatic reinvestment
pub fn process_reinvestment(
    ctx: Context<ProcessReinvestment>,
) -> Result<()> {
    let user_preferences = &ctx.accounts.user_preferences;
    let user_rewards = &mut ctx.accounts.user_rewards;
    let staking_pool = &mut ctx.accounts.staking_pool;
    let user = ctx.accounts.user.key();
    
    // Check if reinvestment is enabled
    if !user_preferences.reinvestment_config.enabled {
        return Err(VaultError::ReinvestmentNotEnabled.into());
    }
    
    // Check if user has sufficient rewards
    let reinvestment_amount = user_rewards.pending_rewards
        .checked_mul(user_preferences.reinvestment_config.percentage as u64)
        .unwrap()
        .checked_div(100)
        .unwrap();
    
    if reinvestment_amount < user_preferences.reinvestment_config.min_threshold {
        return Err(VaultError::InsufficientReinvestmentAmount.into());
    }
    
    // Check compounding frequency
    let clock = Clock::get()?;
    let time_since_last = clock.unix_timestamp - user_rewards.last_compound;
    if time_since_last < user_preferences.reinvestment_config.compound_frequency as i64 {
        return Err(VaultError::ReinvestmentTooFrequent.into());
    }
    
    // Execute reinvestment by adding to staking pool
    staking_pool.total_staked = staking_pool.total_staked
        .checked_add(reinvestment_amount).unwrap();
    
    // Update user rewards
    user_rewards.pending_rewards = user_rewards.pending_rewards
        .checked_sub(reinvestment_amount).unwrap();
    user_rewards.total_reinvested = user_rewards.total_reinvested
        .checked_add(reinvestment_amount).unwrap();
    user_rewards.last_compound = clock.unix_timestamp;
    
    msg!(\"Reinvestment processed for user {}: {} tokens reinvested\", 
         user, reinvestment_amount);
    
    Ok(())
}

/// Emergency pause/unpause payment system
pub fn set_emergency_pause(
    ctx: Context<UpdatePaymentConfig>,
    paused: bool,
) -> Result<()> {
    let payment_system = &mut ctx.accounts.payment_system;
    let multisig_wallet = &ctx.accounts.multisig_wallet;
    let authority = ctx.accounts.authority.key();
    
    // Verify authority is a multisig owner
    if !multisig_wallet.signers.iter().any(|s| s.pubkey == authority && s.is_active) {
        return Err(VaultError::UnauthorizedAccess.into());
    }
    
    payment_system.set_emergency_pause(paused)?;
    
    Ok(())
}

// Helper functions for payment processing

fn process_lightning_payment(
    payment_system: &mut PaymentSystem,
    payment: &PaymentRequest,
) -> Result<()> {
    // In production, this would:
    // 1. Connect to Lightning Network node
    // 2. Decode the Lightning invoice
    // 3. Check route availability and fees
    // 4. Send the payment
    // 5. Monitor payment status
    
    msg!(\"Processing Lightning payment: {} sats to {}\", 
         payment.amount, payment.destination);
    
    // Simulate Lightning payment validation
    if !payment.destination.starts_with(\"lnbc\") && !payment.destination.starts_with(\"lntb\") {
        return Err(VaultError::InvalidLightningInvoice.into());
    }
    
    // Check against Lightning configuration
    if payment.amount > payment_system.lightning_config.max_payment_amount {
        return Err(VaultError::PaymentAmountTooLarge.into());
    }
    
    if payment.amount < payment_system.lightning_config.min_payment_amount {
        return Err(VaultError::PaymentAmountTooSmall.into());
    }
    
    Ok(())
}

fn process_usdc_payment(
    treasury_ata: &Account<TokenAccount>,
    recipient_ata: &Account<TokenAccount>,
    token_program: &Program<Token>,
    payment: &PaymentRequest,
    treasury: &mut Account<Treasury>,
) -> Result<()> {
    msg!(\"Processing USDC payment: {} USDC to {}\", 
         payment.amount, payment.destination);
    
    // Verify sufficient USDC balance in treasury
    if treasury_ata.amount < payment.amount {
        return Err(VaultError::InsufficientBalance.into());
    }
    
    // Create transfer instruction
    let transfer_instruction = Transfer {
        from: treasury_ata.to_account_info(),
        to: recipient_ata.to_account_info(),
        authority: treasury.to_account_info(),
    };
    
    let treasury_seeds = &[
        b\"treasury\",
        &[treasury.bump],
    ];
    let signer_seeds = &[&treasury_seeds[..]];
    
    // Execute USDC transfer
    token::transfer(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            transfer_instruction,
            signer_seeds,
        ),
        payment.amount,
    )?;
    
    msg!(\"USDC transfer completed: {} USDC\", payment.amount);
    
    Ok(())
}