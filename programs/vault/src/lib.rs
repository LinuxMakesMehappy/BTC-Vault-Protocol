use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod errors;
pub mod traits;

use instructions::btc_commitment::*;
use instructions::oracle::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod vault {
    use super::*;

    pub fn commit_btc(
        ctx: Context<CommitBTC>,
        amount: u64,
        btc_address: String,
        ecdsa_proof: Vec<u8>,
        public_key: Vec<u8>,
    ) -> Result<()> {
        instructions::btc_commitment::commit_btc(ctx, amount, btc_address, ecdsa_proof, public_key)
    }

    pub fn verify_balance(ctx: Context<VerifyBalance>) -> Result<()> {
        instructions::btc_commitment::verify_balance(ctx)
    }

    pub fn update_commitment(
        ctx: Context<UpdateCommitment>,
        new_amount: u64,
        new_ecdsa_proof: Vec<u8>,
        new_public_key: Vec<u8>,
    ) -> Result<()> {
        instructions::btc_commitment::update_commitment(ctx, new_amount, new_ecdsa_proof, new_public_key)
    }

    // Oracle instructions
    pub fn initialize_oracle(
        ctx: Context<InitializeOracle>,
        btc_usd_feed: Pubkey,
    ) -> Result<()> {
        instructions::oracle::InitializeOracle::process(ctx, btc_usd_feed)
    }

    pub fn update_btc_price(
        ctx: Context<UpdateBTCPrice>,
        price: u64,
        round_id: u64,
        timestamp: i64,
    ) -> Result<()> {
        instructions::oracle::UpdateBTCPrice::process(ctx, price, round_id, timestamp)
    }

    pub fn verify_btc_balance(
        ctx: Context<VerifyBTCBalance>,
        btc_address: String,
        expected_balance: u64,
        ecdsa_proof: Vec<u8>,
    ) -> Result<()> {
        instructions::oracle::VerifyBTCBalance::process(ctx, btc_address, expected_balance, ecdsa_proof)
    }
}