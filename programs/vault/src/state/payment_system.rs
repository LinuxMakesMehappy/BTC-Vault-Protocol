use anchor_lang::prelude::*;
use crate::errors::VaultError;

/// Payment method options for reward distribution
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum PaymentMethod {
    Lightning,  // Bitcoin Lightning Network (default)
    USDC,      // USDC on Solana
}

/// Payment status tracking
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum PaymentStatus {
    Pending,
    Processing,
    Completed,
    Failed,
    Cancelled,
}

/// Lightning Network payment configuration
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct LightningConfig {
    pub node_pubkey: [u8; 33],        // Lightning node public key
    pub channel_capacity: u64,         // Channel capacity in sats
    pub fee_rate: u16,                // Fee rate in ppm (parts per million)
    pub timeout_blocks: u16,          // Payment timeout in blocks
    pub max_payment_amount: u64,      // Maximum payment in sats
    pub min_payment_amount: u64,      // Minimum payment in sats
}

/// USDC payment configuration
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct UsdcConfig {
    pub mint_address: Pubkey,         // USDC mint address
    pub treasury_ata: Pubkey,         // Treasury associated token account
    pub fee_basis_points: u16,        // Fee in basis points (100 = 1%)
    pub max_payment_amount: u64,      // Maximum payment in USDC (6 decimals)
    pub min_payment_amount: u64,      // Minimum payment in USDC (6 decimals)
}

/// Auto-reinvestment configuration
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct ReinvestmentConfig {
    pub enabled: bool,                // Whether auto-reinvestment is enabled
    pub percentage: u8,               // Percentage to reinvest (0-100)
    pub min_threshold: u64,           // Minimum amount to trigger reinvestment
    pub compound_frequency: u32,      // Compounding frequency in seconds
}

/// Payment request structure
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct PaymentRequest {
    pub id: u64,                      // Unique payment ID
    pub user: Pubkey,                 // User requesting payment
    pub method: PaymentMethod,        // Payment method
    pub amount: u64,                  // Amount in base units
    pub destination: String,          // Payment destination (invoice/address)
    pub status: PaymentStatus,        // Current payment status
    pub created_at: i64,              // Creation timestamp
    pub processed_at: Option<i64>,    // Processing timestamp
    pub completed_at: Option<i64>,    // Completion timestamp
    pub failure_reason: Option<String>, // Failure reason if applicable
    pub retry_count: u8,              // Number of retry attempts
    pub multisig_required: bool,      // Whether multisig approval is required
}

#[account]
pub struct PaymentSystem {
    pub lightning_config: LightningConfig,
    pub usdc_config: UsdcConfig,
    pub payment_requests: Vec<PaymentRequest>,
    pub total_payments_processed: u64,
    pub total_lightning_volume: u64,
    pub total_usdc_volume: u64,
    pub failed_payments_count: u64,
    pub last_payment_id: u64,
    pub emergency_pause: bool,        // Emergency pause for payments
    pub multisig_wallet: Pubkey,      // Associated multisig wallet
    pub bump: u8,
}

impl PaymentSystem {
    pub const LEN: usize = 8 + // discriminator
        (33 + 8 + 2 + 2 + 8 + 8) + // lightning_config
        (32 + 32 + 2 + 8 + 8) + // usdc_config
        4 + (20 * (8 + 32 + 1 + 8 + 4 + 64 + 1 + 8 + 9 + 9 + 4 + 64 + 1 + 1)) + // payment_requests (max 20)
        8 + // total_payments_processed
        8 + // total_lightning_volume
        8 + // total_usdc_volume
        8 + // failed_payments_count
        8 + // last_payment_id
        1 + // emergency_pause
        32 + // multisig_wallet
        1; // bump

    pub const MAX_PAYMENT_REQUESTS: usize = 20;
    pub const MAX_RETRY_ATTEMPTS: u8 = 3;
    pub const PAYMENT_TIMEOUT_SECONDS: i64 = 3600; // 1 hour

    /// Initialize payment system with configurations
    pub fn initialize(
        &mut self,
        lightning_config: LightningConfig,
        usdc_config: UsdcConfig,
        multisig_wallet: Pubkey,
        bump: u8,
    ) -> Result<()> {
        self.lightning_config = lightning_config;
        self.usdc_config = usdc_config;
        self.payment_requests = Vec::new();
        self.total_payments_processed = 0;
        self.total_lightning_volume = 0;
        self.total_usdc_volume = 0;
        self.failed_payments_count = 0;
        self.last_payment_id = 0;
        self.emergency_pause = false;
        self.multisig_wallet = multisig_wallet;
        self.bump = bump;

        Ok(())
    }

    /// Create a new payment request
    pub fn create_payment_request(
        &mut self,
        user: Pubkey,
        method: PaymentMethod,
        amount: u64,
        destination: String,
    ) -> Result<u64> {
        if self.emergency_pause {
            return Err(VaultError::PaymentSystemPaused.into());
        }

        // Validate payment amount
        self.validate_payment_amount(&method, amount)?;

        // Validate destination format
        self.validate_destination(&method, &destination)?;

        // Check if we need multisig approval
        let multisig_required = self.requires_multisig_approval(&method, amount);

        // Clean up old payment requests
        self.cleanup_old_requests()?;

        if self.payment_requests.len() >= Self::MAX_PAYMENT_REQUESTS {
            return Err(VaultError::PaymentQueueFull.into());
        }

        let payment_id = self.last_payment_id.checked_add(1)
            .ok_or(VaultError::ArithmeticOverflow)?;
        let clock = Clock::get()?;

        let payment_request = PaymentRequest {
            id: payment_id,
            user,
            method: method.clone(),
            amount,
            destination,
            status: if multisig_required {
                PaymentStatus::Pending
            } else {
                PaymentStatus::Processing
            },
            created_at: clock.unix_timestamp,
            processed_at: None,
            completed_at: None,
            failure_reason: None,
            retry_count: 0,
            multisig_required,
        };

        self.payment_requests.push(payment_request);
        self.last_payment_id = payment_id;

        msg!("Payment request {} created for user {} (method: {:?}, amount: {})",
             payment_id, user, method, amount);

        Ok(payment_id)
    }

    /// Process a payment request
    pub fn process_payment(&mut self, payment_id: u64) -> Result<()> {
        let payment_index = self.payment_requests
            .iter()
            .position(|p| p.id == payment_id)
            .ok_or(VaultError::PaymentNotFound)?;

        let payment = &mut self.payment_requests[payment_index];

        if payment.status != PaymentStatus::Pending && payment.status != PaymentStatus::Processing {
            return Err(VaultError::InvalidPaymentStatus.into());
        }

        let clock = Clock::get()?;
        payment.status = PaymentStatus::Processing;
        payment.processed_at = Some(clock.unix_timestamp);

        // Execute payment based on method
        match payment.method {
            PaymentMethod::Lightning => {
                self.process_lightning_payment(payment)?;
            },
            PaymentMethod::USDC => {
                self.process_usdc_payment(payment)?;
            },
        }

        Ok(())
    }

    /// Complete a payment request
    pub fn complete_payment(&mut self, payment_id: u64, success: bool, failure_reason: Option<String>) -> Result<()> {
        let payment_index = self.payment_requests
            .iter()
            .position(|p| p.id == payment_id)
            .ok_or(VaultError::PaymentNotFound)?;

        let payment = &mut self.payment_requests[payment_index];
        let clock = Clock::get()?;

        if success {
            payment.status = PaymentStatus::Completed;
            payment.completed_at = Some(clock.unix_timestamp);
            
            // Update volume statistics
            match payment.method {
                PaymentMethod::Lightning => {
                    self.total_lightning_volume = self.total_lightning_volume
                        .checked_add(payment.amount).ok_or(VaultError::ArithmeticOverflow)?;
                },
                PaymentMethod::USDC => {
                    self.total_usdc_volume = self.total_usdc_volume
                        .checked_add(payment.amount).ok_or(VaultError::ArithmeticOverflow)?;
                },
            }
            
            self.total_payments_processed = self.total_payments_processed
                .checked_add(1).ok_or(VaultError::ArithmeticOverflow)?;

            msg!("Payment {} completed successfully", payment_id);
        } else {
            payment.retry_count = payment.retry_count.checked_add(1).unwrap();
            payment.failure_reason = failure_reason;

            if payment.retry_count >= Self::MAX_RETRY_ATTEMPTS {
                payment.status = PaymentStatus::Failed;
                self.failed_payments_count = self.failed_payments_count
                    .checked_add(1).unwrap();
                msg!("Payment {} failed after {} attempts", payment_id, payment.retry_count);
            } else {
                payment.status = PaymentStatus::Pending;
                msg!("Payment {} failed, retry {} of {}", payment_id, payment.retry_count, Self::MAX_RETRY_ATTEMPTS);
            }
        }

        Ok(())
    }

    /// Cancel a payment request
    pub fn cancel_payment(&mut self, payment_id: u64, user: Pubkey) -> Result<()> {
        let payment_index = self.payment_requests
            .iter()
            .position(|p| p.id == payment_id && p.user == user)
            .ok_or(VaultError::PaymentNotFound)?;

        let payment = &mut self.payment_requests[payment_index];

        if payment.status == PaymentStatus::Completed {
            return Err(VaultError::PaymentAlreadyCompleted.into());
        }

        if payment.status == PaymentStatus::Processing {
            return Err(VaultError::PaymentInProgress.into());
        }

        payment.status = PaymentStatus::Cancelled;
        msg!("Payment {} cancelled by user {}", payment_id, user);

        Ok(())
    }

    /// Get payment request by ID
    pub fn get_payment_request(&self, payment_id: u64) -> Option<&PaymentRequest> {
        self.payment_requests.iter().find(|p| p.id == payment_id)
    }

    /// Get user's payment requests
    pub fn get_user_payments(&self, user: Pubkey) -> Vec<&PaymentRequest> {
        self.payment_requests.iter()
            .filter(|p| p.user == user)
            .collect()
    }

    /// Emergency pause/unpause payment system
    pub fn set_emergency_pause(&mut self, paused: bool) -> Result<()> {
        self.emergency_pause = paused;
        msg!("Payment system emergency pause: {}", paused);
        Ok(())
    }

    /// Update Lightning configuration
    pub fn update_lightning_config(&mut self, config: LightningConfig) -> Result<()> {
        self.lightning_config = config;
        msg!("Lightning configuration updated");
        Ok(())
    }

    /// Update USDC configuration
    pub fn update_usdc_config(&mut self, config: UsdcConfig) -> Result<()> {
        self.usdc_config = config;
        msg!("USDC configuration updated");
        Ok(())
    }

    // Private helper methods

    fn validate_payment_amount(&self, method: &PaymentMethod, amount: u64) -> Result<()> {
        match method {
            PaymentMethod::Lightning => {
                if amount < self.lightning_config.min_payment_amount {
                    return Err(VaultError::PaymentAmountTooSmall.into());
                }
                if amount > self.lightning_config.max_payment_amount {
                    return Err(VaultError::PaymentAmountTooLarge.into());
                }
            },
            PaymentMethod::USDC => {
                if amount < self.usdc_config.min_payment_amount {
                    return Err(VaultError::PaymentAmountTooSmall.into());
                }
                if amount > self.usdc_config.max_payment_amount {
                    return Err(VaultError::PaymentAmountTooLarge.into());
                }
            },
        }
        Ok(())
    }

    fn validate_destination(&self, method: &PaymentMethod, destination: &str) -> Result<()> {
        match method {
            PaymentMethod::Lightning => {
                // Validate Lightning invoice format
                if !destination.starts_with("lnbc") && !destination.starts_with("lntb") {
                    return Err(VaultError::InvalidLightningInvoice.into());
                }
                if destination.len() < 50 || destination.len() > 2000 {
                    return Err(VaultError::InvalidLightningInvoice.into());
                }
            },
            PaymentMethod::USDC => {
                // Validate Solana address format
                if destination.len() != 44 {
                    return Err(VaultError::InvalidSolanaAddress.into());
                }
                // Additional base58 validation could be added here
            },
        }
        Ok(())
    }

    fn requires_multisig_approval(&self, method: &PaymentMethod, amount: u64) -> bool {
        // Large payments require multisig approval
        match method {
            PaymentMethod::Lightning => amount > 1000000, // 0.01 BTC in sats
            PaymentMethod::USDC => amount > 1000_000000,  // $1000 in USDC (6 decimals)
        }
    }

    fn cleanup_old_requests(&mut self) -> Result<()> {
        let clock = Clock::get()?;
        let cutoff_time = clock.unix_timestamp - Self::PAYMENT_TIMEOUT_SECONDS;

        self.payment_requests.retain(|payment| {
            if payment.status == PaymentStatus::Completed || payment.status == PaymentStatus::Failed {
                // Keep completed/failed payments for a while for history
                payment.created_at > cutoff_time - (24 * 3600) // 24 hours
            } else if payment.status == PaymentStatus::Pending {
                // Remove old pending payments
                payment.created_at > cutoff_time
            } else {
                true // Keep processing payments
            }
        });

        Ok(())
    }

    fn process_lightning_payment(&self, payment: &PaymentRequest) -> Result<()> {
        // In production, this would integrate with Lightning Network node
        // For now, we simulate the payment process
        msg!("Processing Lightning payment: {} sats to {}", 
             payment.amount, payment.destination);
        
        // Validate Lightning invoice
        if !payment.destination.starts_with("lnbc") && !payment.destination.starts_with("lntb") {
            return Err(VaultError::InvalidLightningInvoice.into());
        }

        // Check channel capacity and routing
        if payment.amount > self.lightning_config.channel_capacity {
            return Err(VaultError::InsufficientLightningCapacity.into());
        }

        // In production:
        // 1. Decode Lightning invoice
        // 2. Check route availability
        // 3. Send payment via Lightning node
        // 4. Monitor payment status
        
        Ok(())
    }

    fn process_usdc_payment(&self, payment: &PaymentRequest) -> Result<()> {
        // In production, this would execute USDC transfer
        msg!("Processing USDC payment: {} USDC to {}", 
             payment.amount, payment.destination);
        
        // Validate recipient address
        if payment.destination.len() != 44 {
            return Err(VaultError::InvalidSolanaAddress.into());
        }

        // In production:
        // 1. Create USDC transfer instruction
        // 2. Execute transfer from treasury ATA
        // 3. Verify transaction success
        
        Ok(())
    }

    /// Get payment system statistics
    pub fn get_statistics(&self) -> PaymentStatistics {
        PaymentStatistics {
            total_payments: self.total_payments_processed,
            total_lightning_volume: self.total_lightning_volume,
            total_usdc_volume: self.total_usdc_volume,
            failed_payments: self.failed_payments_count,
            pending_payments: self.payment_requests.iter()
                .filter(|p| p.status == PaymentStatus::Pending).count() as u64,
            processing_payments: self.payment_requests.iter()
                .filter(|p| p.status == PaymentStatus::Processing).count() as u64,
        }
    }
}

/// Payment system statistics
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct PaymentStatistics {
    pub total_payments: u64,
    pub total_lightning_volume: u64,
    pub total_usdc_volume: u64,
    pub failed_payments: u64,
    pub pending_payments: u64,
    pub processing_payments: u64,
}

/// User payment preferences
#[account]
pub struct UserPaymentPreferences {
    pub user: Pubkey,
    pub default_method: PaymentMethod,
    pub lightning_address: Option<String>,
    pub usdc_address: Option<Pubkey>,
    pub reinvestment_config: ReinvestmentConfig,
    pub notification_preferences: NotificationPreferences,
    pub bump: u8,
}

impl UserPaymentPreferences {
    pub const LEN: usize = 8 + // discriminator
        32 + // user
        1 + // default_method
        4 + 200 + // lightning_address (optional)
        33 + // usdc_address (optional)
        (1 + 1 + 8 + 4) + // reinvestment_config
        (1 + 1 + 1 + 1) + // notification_preferences
        1; // bump

    pub fn initialize(
        &mut self,
        user: Pubkey,
        default_method: PaymentMethod,
        bump: u8,
    ) -> Result<()> {
        self.user = user;
        self.default_method = default_method;
        self.lightning_address = None;
        self.usdc_address = None;
        self.reinvestment_config = ReinvestmentConfig {
            enabled: false,
            percentage: 0,
            min_threshold: 0,
            compound_frequency: 86400, // Daily
        };
        self.notification_preferences = NotificationPreferences {
            payment_completed: true,
            payment_failed: true,
            large_payment_approval: true,
            reinvestment_executed: false,
        };
        self.bump = bump;

        Ok(())
    }

    pub fn update_default_method(&mut self, method: PaymentMethod) -> Result<()> {
        self.default_method = method;
        Ok(())
    }

    pub fn update_lightning_address(&mut self, address: Option<String>) -> Result<()> {
        if let Some(ref addr) = address {
            if addr.len() > 200 {
                return Err(VaultError::InvalidLightningAddress.into());
            }
        }
        self.lightning_address = address;
        Ok(())
    }

    pub fn update_usdc_address(&mut self, address: Option<Pubkey>) -> Result<()> {
        self.usdc_address = address;
        Ok(())
    }

    pub fn update_reinvestment_config(&mut self, config: ReinvestmentConfig) -> Result<()> {
        if config.percentage > 100 {
            return Err(VaultError::InvalidReinvestmentPercentage.into());
        }
        self.reinvestment_config = config;
        Ok(())
    }
}

/// Notification preferences for payment events
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct NotificationPreferences {
    pub payment_completed: bool,
    pub payment_failed: bool,
    pub large_payment_approval: bool,
    pub reinvestment_executed: bool,
}
