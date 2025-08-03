use anchor_lang::prelude::*;

#[account]
pub struct MultisigWallet {
    pub signers: Vec<Pubkey>,
    pub threshold: u8, // 2-of-3
    pub transaction_count: u32,
    pub bump: u8,
}

impl MultisigWallet {
    pub const LEN: usize = 8 + // discriminator
        4 + (32 * 3) + // signers (max 3 signers)
        1 + // threshold
        4 + // transaction_count
        1; // bump

    pub const MAX_SIGNERS: usize = 3;
    pub const REQUIRED_THRESHOLD: u8 = 2;
}

#[account]
pub struct MultisigTransaction {
    pub multisig: Pubkey,
    pub transaction_id: u32,
    pub proposer: Pubkey,
    pub transaction_data: Vec<u8>,
    pub signatures: Vec<Vec<u8>>,
    pub executed: bool,
    pub created_at: i64,
    pub bump: u8,
}

impl MultisigTransaction {
    pub const LEN: usize = 8 + // discriminator
        32 + // multisig
        4 + // transaction_id
        32 + // proposer
        4 + 1024 + // transaction_data (max 1KB)
        4 + (3 * (4 + 64)) + // signatures (max 3 signatures, 64 bytes each)
        1 + // executed
        8 + // created_at
        1; // bump
}