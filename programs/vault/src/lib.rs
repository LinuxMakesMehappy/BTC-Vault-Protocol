use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod errors;
pub mod traits;

use instructions::btc_commitment::*;
use instructions::oracle::*;
use instructions::staking::*;

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

    // Staking instructions
    pub fn initialize_staking_pool(ctx: Context<InitializeStakingPool>) -> Result<()> {
        instructions::staking::initialize_staking_pool(ctx)
    }

    pub fn stake_protocol_assets(
        ctx: Context<StakeProtocolAssets>,
        total_treasury_usd: u64,
    ) -> Result<()> {
        instructions::staking::stake_protocol_assets(ctx, total_treasury_usd)
    }

    pub fn rebalance_allocations(ctx: Context<RebalanceAllocations>) -> Result<()> {
        instructions::staking::rebalance_allocations(ctx)
    }

    pub fn add_sol_validator(
        ctx: Context<AddValidator>,
        address: String,
        commission: u16,
        performance_score: u16,
    ) -> Result<()> {
        instructions::staking::add_sol_validator(ctx, address, commission, performance_score)
    }

    pub fn add_eth_validator(
        ctx: Context<AddValidator>,
        address: String,
        commission: u16,
        performance_score: u16,
    ) -> Result<()> {
        instructions::staking::add_eth_validator(ctx, address, commission, performance_score)
    }

    pub fn update_atom_config(
        ctx: Context<AddValidator>,
        everstake_validator: String,
        osmosis_validator: String,
    ) -> Result<()> {
        instructions::staking::update_atom_config(ctx, everstake_validator, osmosis_validator)
    }
}