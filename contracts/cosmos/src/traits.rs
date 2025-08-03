use cosmwasm_std::{Addr, Uint128};
use serde::{Deserialize, Serialize};

/// Core trait for ATOM staking operations
pub trait AtomStakingInterface {
    fn stake_atom(validator: String, amount: Uint128) -> Result<(), crate::ContractError>;
    fn unstake_atom(validator: String, amount: Uint128) -> Result<(), crate::ContractError>;
    fn claim_rewards(validator: String) -> Result<Uint128, crate::ContractError>;
    fn redelegate(src_validator: String, dst_validator: String, amount: Uint128) -> Result<(), crate::ContractError>;
}

/// Core trait for cross-chain communication
pub trait CrossChainInterface {
    fn send_message(chain_id: String, message: Vec<u8>) -> Result<(), crate::ContractError>;
    fn receive_message(sender: Addr, message: Vec<u8>) -> Result<(), crate::ContractError>;
    fn verify_state(state_hash: String) -> Result<bool, crate::ContractError>;
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum StakingProvider {
    Everstake,
    Cephalopod,
    Osmosis,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ValidatorInfo {
    pub address: String,
    pub commission: String,
    pub provider: StakingProvider,
    pub allocation_percentage: u32, // basis points
}