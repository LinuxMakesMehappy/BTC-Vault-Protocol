use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};
use serde::{Deserialize, Serialize};
use crate::traits::{StakingProvider, ValidatorInfo};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Config {
    pub owner: Addr,
    pub solana_program_id: String,
    pub total_staked: Uint128,
    pub everstake_allocation: u32,  // 2000 basis points (20%)
    pub osmosis_allocation: u32,    // 1000 basis points (10%)
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct StakingState {
    pub validator: String,
    pub staked_amount: Uint128,
    pub rewards_accumulated: Uint128,
    pub last_claim: u64,
    pub provider: StakingProvider,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CrossChainMessage {
    pub sender_chain: String,
    pub message_id: u64,
    pub payload: Vec<u8>,
    pub timestamp: u64,
    pub processed: bool,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const STAKING_STATE: Map<String, StakingState> = Map::new("staking_state");
pub const VALIDATORS: Map<String, ValidatorInfo> = Map::new("validators");
pub const CROSS_CHAIN_MESSAGES: Map<u64, CrossChainMessage> = Map::new("cross_chain_messages");
pub const MESSAGE_COUNTER: Item<u64> = Item::new("message_counter");