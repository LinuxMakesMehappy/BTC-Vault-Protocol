use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
pub struct ProposeMultisigTransaction<'info> {
    #[account(
        init_if_needed,
        payer = proposer,
        space = MultisigWallet::LEN,
        seeds = [b"multisig_wallet"],
        bump
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

pub fn propose_transaction(
    ctx: Context<ProposeMultisigTransaction>,
    transaction_data: Vec<u8>,
) -> Result<()> {
    let multisig_wallet = &mut ctx.accounts.multisig_wallet;
    let multisig_transaction = &mut ctx.accounts.multisig_transaction;
    let clock = Clock::get()?;

    // Initialize multisig wallet if needed
    if multisig_wallet.signers.is_empty() {
        multisig_wallet.threshold = MultisigWallet::REQUIRED_THRESHOLD;
        multisig_wallet.bump = ctx.bumps.multisig_wallet;
    }

    // Create new transaction proposal
    multisig_transaction.multisig = multisig_wallet.key();
    multisig_transaction.transaction_id = multisig_wallet.transaction_count;
    multisig_transaction.proposer = ctx.accounts.proposer.key();
    multisig_transaction.transaction_data = transaction_data;
    multisig_transaction.signatures = Vec::new();
    multisig_transaction.executed = false;
    multisig_transaction.created_at = clock.unix_timestamp;
    multisig_transaction.bump = ctx.bumps.multisig_transaction;

    // Increment transaction counter
    multisig_wallet.transaction_count = multisig_wallet.transaction_count
        .checked_add(1).unwrap();

    Ok(())
}