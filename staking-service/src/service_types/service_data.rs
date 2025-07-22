use sails_rs::{
    prelude::*,
    gstd::exec
};
use vara_contract_utils::utils;

#[derive(Debug, Default)]
pub struct ServiceData {
    pub created_at_block: u64,
    pub created_at_era: u64,
    pub last_era_rewards_collected: u64
}

impl ServiceData {
    pub fn new(on_mainnet: bool) -> Self {
        let created_at_block = exec::block_height() as u64;
        let active_era = if on_mainnet {
            utils::mainnet_active_era()
        } else {
            utils::testnet_active_era()
        };

        Self { 
            created_at_block, 
            created_at_era: active_era,
            last_era_rewards_collected: active_era
        }
    }
}