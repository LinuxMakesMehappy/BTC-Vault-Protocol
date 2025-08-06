use anchor_lang::prelude::*;
use crate::errors::VaultError;

/// KYC compliance tiers with different limits and requirements
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum KYCTier {
    None,           // No KYC - limited to 1 BTC
    Basic,          // Basic KYC - up to 10 BTC
    Enhanced,       // Enhanced KYC - up to 100 BTC
    Institutional,  // Institutional KYC - unlimited
}

/// KYC verification status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum KYCStatus {
    NotStarted,     // User hasn't initiated KYC
    Pending,        // KYC verification in progress
    Approved,       // KYC approved
    Rejected,       // KYC rejected
    Expired,        // KYC expired (needs renewal)
    Suspended,      // KYC suspended due to compliance issues
}

/// Document types for KYC verification
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum DocumentType {
    Passport,
    DriversLicense,
    NationalId,
    ProofOfAddress,
    BankStatement,
    TaxDocument,
    CorporateRegistration,
    BeneficialOwnership,
}

/// Risk assessment levels from compliance screening
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Prohibited,
}

/// Compliance screening result from Chainalysis
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct ComplianceScreening {
    pub screening_id: String,
    pub risk_level: RiskLevel,
    pub sanctions_match: bool,
    pub pep_match: bool,          // Politically Exposed Person
    pub adverse_media: bool,
    pub screening_date: i64,
    pub expiry_date: i64,
    pub notes: String,
}

/// KYC document submission
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct KYCDocument {
    pub document_type: DocumentType,
    pub document_hash: [u8; 32],   // SHA-256 hash of document
    pub upload_date: i64,
    pub verified: bool,
    pub verification_date: Option<i64>,
    pub expiry_date: Option<i64>,
}

/// User's KYC profile and compliance status
#[account]
pub struct KYCProfile {
    pub user: Pubkey,
    pub tier: KYCTier,
    pub status: KYCStatus,
    pub documents: Vec<KYCDocument>,
    pub compliance_screening: Option<ComplianceScreening>,
    pub commitment_limit: u64,      // BTC commitment limit in satoshis
    pub daily_limit: u64,           // Daily transaction limit in satoshis
    pub monthly_volume: u64,        // Current month's transaction volume
    pub last_screening_date: i64,
    pub kyc_expiry_date: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
    pub compliance_officer: Option<Pubkey>,
    pub notes: String,
    pub bump: u8,
}

impl KYCProfile {
    pub const LEN: usize = 8 + // discriminator
        32 + // user
        1 + // tier
        1 + // status
        4 + (10 * (1 + 32 + 8 + 1 + 9 + 9)) + // documents (max 10)
        (1 + (4 + 64 + 1 + 1 + 1 + 8 + 8 + 4 + 256)) + // compliance_screening (optional)
        8 + // commitment_limit
        8 + // daily_limit
        8 + // monthly_volume
        8 + // last_screening_date
        9 + // kyc_expiry_date (optional)
        8 + // created_at
        8 + // updated_at
        33 + // compliance_officer (optional)
        4 + 512 + // notes (max 512 chars)
        1; // bump

    pub const MAX_DOCUMENTS: usize = 10;
    pub const MAX_NOTES_LENGTH: usize = 512;

    /// Initialize a new KYC profile
    pub fn initialize(
        &mut self,
        user: Pubkey,
        bump: u8,
    ) -> Result<()> {
        let clock = Clock::get()?;
        
        self.user = user;
        self.tier = KYCTier::None;
        self.status = KYCStatus::NotStarted;
        self.documents = Vec::new();
        self.compliance_screening = None;
        self.commitment_limit = 100_000_000; // 1 BTC in satoshis
        self.daily_limit = 10_000_000;       // 0.1 BTC daily limit
        self.monthly_volume = 0;
        self.last_screening_date = 0;
        self.kyc_expiry_date = None;
        self.created_at = clock.unix_timestamp;
        self.updated_at = clock.unix_timestamp;
        self.compliance_officer = None;
        self.notes = String::new();
        self.bump = bump;

        Ok(())
    }

    /// Start KYC verification process
    pub fn start_kyc_verification(&mut self, target_tier: KYCTier) -> Result<()> {
        if self.status == KYCStatus::Pending {
            return Err(VaultError::KYCAlreadyInProgress.into());
        }

        if self.status == KYCStatus::Approved && self.tier >= target_tier {
            return Err(VaultError::KYCAlreadyApproved.into());
        }

        self.status = KYCStatus::Pending;
        self.updated_at = Clock::get()?.unix_timestamp;

        msg!("KYC verification started for user {} targeting tier {:?}", 
             self.user, target_tier);

        Ok(())
    }

    /// Submit a KYC document
    pub fn submit_document(
        &mut self,
        document_type: DocumentType,
        document_hash: [u8; 32],
    ) -> Result<()> {
        if self.documents.len() >= Self::MAX_DOCUMENTS {
            return Err(VaultError::TooManyDocuments.into());
        }

        // Check if document type already exists
        if self.documents.iter().any(|doc| doc.document_type == document_type) {
            return Err(VaultError::DocumentTypeAlreadyExists.into());
        }

        let clock = Clock::get()?;
        let document = KYCDocument {
            document_type: document_type.clone(),
            document_hash,
            upload_date: clock.unix_timestamp,
            verified: false,
            verification_date: None,
            expiry_date: None,
        };

        self.documents.push(document);
        self.updated_at = clock.unix_timestamp;

        msg!("Document {:?} submitted for user {}", document_type, self.user);

        Ok(())
    }

    /// Verify a submitted document
    pub fn verify_document(
        &mut self,
        document_type: DocumentType,
        compliance_officer: Pubkey,
        expiry_date: Option<i64>,
    ) -> Result<()> {
        let document = self.documents
            .iter_mut()
            .find(|doc| doc.document_type == document_type)
            .ok_or(VaultError::DocumentNotFound)?;

        if document.verified {
            return Err(VaultError::DocumentAlreadyVerified.into());
        }

        let clock = Clock::get()?;
        document.verified = true;
        document.verification_date = Some(clock.unix_timestamp);
        document.expiry_date = expiry_date;

        self.compliance_officer = Some(compliance_officer);
        self.updated_at = clock.unix_timestamp;

        msg!("Document {:?} verified for user {} by officer {}", 
             document_type, self.user, compliance_officer);

        Ok(())
    }

    /// Update compliance screening results
    pub fn update_compliance_screening(
        &mut self,
        screening: ComplianceScreening,
    ) -> Result<()> {
        // Check if user is prohibited
        if screening.risk_level == RiskLevel::Prohibited || screening.sanctions_match {
            self.status = KYCStatus::Rejected;
            msg!("User {} rejected due to compliance screening", self.user);
            return Err(VaultError::ComplianceViolation.into());
        }

        self.compliance_screening = Some(screening);
        self.last_screening_date = Clock::get()?.unix_timestamp;
        self.updated_at = Clock::get()?.unix_timestamp;

        Ok(())
    }

    /// Approve KYC for a specific tier
    pub fn approve_kyc(
        &mut self,
        tier: KYCTier,
        compliance_officer: Pubkey,
        expiry_months: Option<u32>,
    ) -> Result<()> {
        if self.status != KYCStatus::Pending {
            return Err(VaultError::InvalidKYCStatus.into());
        }

        // Verify required documents are submitted and verified
        self.validate_tier_requirements(&tier)?;

        let clock = Clock::get()?;
        self.tier = tier.clone();
        self.status = KYCStatus::Approved;
        self.compliance_officer = Some(compliance_officer);
        self.updated_at = clock.unix_timestamp;

        // Set expiry date if specified
        if let Some(months) = expiry_months {
            self.kyc_expiry_date = Some(clock.unix_timestamp + (months as i64 * 30 * 24 * 3600));
        }

        // Update limits based on tier
        self.update_limits_for_tier(&tier);

        msg!("KYC approved for user {} at tier {:?} by officer {}", 
             self.user, tier, compliance_officer);

        Ok(())
    }

    /// Reject KYC application
    pub fn reject_kyc(
        &mut self,
        compliance_officer: Pubkey,
        reason: String,
    ) -> Result<()> {
        if self.status != KYCStatus::Pending {
            return Err(VaultError::InvalidKYCStatus.into());
        }

        self.status = KYCStatus::Rejected;
        self.compliance_officer = Some(compliance_officer);
        self.notes = reason;
        self.updated_at = Clock::get()?.unix_timestamp;

        msg!("KYC rejected for user {} by officer {}", self.user, compliance_officer);

        Ok(())
    }

    /// Check if user can commit a specific amount
    pub fn can_commit(&self, amount: u64) -> Result<bool> {
        if self.status != KYCStatus::Approved && amount > 100_000_000 {
            return Ok(false); // Non-KYC users limited to 1 BTC
        }

        if amount > self.commitment_limit {
            return Ok(false);
        }

        // Check if KYC has expired
        if let Some(expiry) = self.kyc_expiry_date {
            let clock = Clock::get()?;
            if clock.unix_timestamp > expiry {
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Check daily transaction limit
    pub fn check_daily_limit(&self, amount: u64) -> Result<bool> {
        // For simplicity, we'll track daily limits in a separate mechanism
        // This is a placeholder for the daily limit check
        Ok(amount <= self.daily_limit)
    }

    /// Update monthly volume
    pub fn update_monthly_volume(&mut self, amount: u64) -> Result<()> {
        self.monthly_volume = self.monthly_volume.checked_add(amount).unwrap();
        self.updated_at = Clock::get()?.unix_timestamp;
        Ok(())
    }

    /// Check if KYC needs renewal
    pub fn needs_renewal(&self) -> Result<bool> {
        if let Some(expiry) = self.kyc_expiry_date {
            let clock = Clock::get()?;
            // Warn 30 days before expiry
            Ok(clock.unix_timestamp > (expiry - 30 * 24 * 3600))
        } else {
            Ok(false)
        }
    }

    /// Suspend KYC due to compliance issues
    pub fn suspend_kyc(
        &mut self,
        compliance_officer: Pubkey,
        reason: String,
    ) -> Result<()> {
        self.status = KYCStatus::Suspended;
        self.compliance_officer = Some(compliance_officer);
        self.notes = reason;
        self.updated_at = Clock::get()?.unix_timestamp;

        msg!("KYC suspended for user {} by officer {}", self.user, compliance_officer);

        Ok(())
    }

    // Private helper methods

    fn validate_tier_requirements(&self, tier: &KYCTier) -> Result<()> {
        let required_docs = match tier {
            KYCTier::None => vec![],
            KYCTier::Basic => vec![
                DocumentType::Passport,
                DocumentType::ProofOfAddress,
            ],
            KYCTier::Enhanced => vec![
                DocumentType::Passport,
                DocumentType::ProofOfAddress,
                DocumentType::BankStatement,
            ],
            KYCTier::Institutional => vec![
                DocumentType::CorporateRegistration,
                DocumentType::BeneficialOwnership,
                DocumentType::BankStatement,
            ],
        };

        for required_doc in required_docs {
            let doc_verified = self.documents
                .iter()
                .any(|doc| doc.document_type == required_doc && doc.verified);
            
            if !doc_verified {
                return Err(VaultError::RequiredDocumentMissing.into());
            }
        }

        // Check compliance screening for higher tiers
        if matches!(tier, KYCTier::Enhanced | KYCTier::Institutional) {
            if self.compliance_screening.is_none() {
                return Err(VaultError::ComplianceScreeningRequired.into());
            }

            let screening = self.compliance_screening.as_ref().unwrap();
            if screening.risk_level == RiskLevel::High || screening.sanctions_match {
                return Err(VaultError::ComplianceViolation.into());
            }
        }

        Ok(())
    }

    fn update_limits_for_tier(&mut self, tier: &KYCTier) {
        match tier {
            KYCTier::None => {
                self.commitment_limit = 100_000_000;    // 1 BTC
                self.daily_limit = 10_000_000;          // 0.1 BTC
            },
            KYCTier::Basic => {
                self.commitment_limit = 1_000_000_000;  // 10 BTC
                self.daily_limit = 100_000_000;         // 1 BTC
            },
            KYCTier::Enhanced => {
                self.commitment_limit = 10_000_000_000; // 100 BTC
                self.daily_limit = 1_000_000_000;       // 10 BTC
            },
            KYCTier::Institutional => {
                self.commitment_limit = u64::MAX;       // Unlimited
                self.daily_limit = u64::MAX;            // Unlimited
            },
        }
    }

    /// Get tier requirements for display
    pub fn get_tier_requirements(tier: &KYCTier) -> Vec<DocumentType> {
        match tier {
            KYCTier::None => vec![],
            KYCTier::Basic => vec![
                DocumentType::Passport,
                DocumentType::ProofOfAddress,
            ],
            KYCTier::Enhanced => vec![
                DocumentType::Passport,
                DocumentType::ProofOfAddress,
                DocumentType::BankStatement,
            ],
            KYCTier::Institutional => vec![
                DocumentType::CorporateRegistration,
                DocumentType::BeneficialOwnership,
                DocumentType::BankStatement,
            ],
        }
    }

    /// Get tier limits for display
    pub fn get_tier_limits(tier: &KYCTier) -> (u64, u64) {
        match tier {
            KYCTier::None => (100_000_000, 10_000_000),        // 1 BTC, 0.1 BTC daily
            KYCTier::Basic => (1_000_000_000, 100_000_000),    // 10 BTC, 1 BTC daily
            KYCTier::Enhanced => (10_000_000_000, 1_000_000_000), // 100 BTC, 10 BTC daily
            KYCTier::Institutional => (u64::MAX, u64::MAX),    // Unlimited
        }
    }
}

/// Compliance monitoring and reporting
#[account]
pub struct ComplianceReport {
    pub report_id: u64,
    pub report_type: ComplianceReportType,
    pub period_start: i64,
    pub period_end: i64,
    pub total_users: u64,
    pub kyc_approved_users: u64,
    pub high_risk_users: u64,
    pub suspicious_activities: u64,
    pub generated_by: Pubkey,
    pub generated_at: i64,
    pub data_hash: [u8; 32],
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum ComplianceReportType {
    Monthly,
    Quarterly,
    Annual,
    Suspicious,
    Regulatory,
}

impl ComplianceReport {
    pub const LEN: usize = 8 + // discriminator
        8 + // report_id
        1 + // report_type
        8 + // period_start
        8 + // period_end
        8 + // total_users
        8 + // kyc_approved_users
        8 + // high_risk_users
        8 + // suspicious_activities
        32 + // generated_by
        8 + // generated_at
        32 + // data_hash
        1; // bump
}