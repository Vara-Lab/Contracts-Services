use sails_rs::{
    prelude::*,
    gstd::exec
};

use vara_contract_utils::utils;

#[derive(Debug, Encode, Decode, TypeInfo, Clone, Copy)]
pub struct UnbondData {
    pub amount: u128,
    pub unbond_at_timestamp: u64,
    pub unbond_at_block: u64,
    pub unbond_at_era: u64,
    pub can_withdraw_at_block: u64,
    pub withdrawn: bool,
    pub rebonded: bool
}

impl UnbondData {
    pub fn new(amount: u128, active_era: u64) -> Self {
        let unbond_at_timestamp = exec::block_timestamp();
        let unbond_at_block = exec::block_height() as u64;
        let can_withdraw_at_block = unbond_at_block + utils::TOTAL_BLOCKS_TO_UNBOND;

        Self { 
            amount,
            unbond_at_timestamp,
            unbond_at_block,
            unbond_at_era: active_era,
            can_withdraw_at_block,
            withdrawn: false,
            rebonded: false
        }
    }

    pub fn blocks_left_to_withdraw(&self) -> u64 {
        let current_block = exec::block_height() as u64;
        let blocks_left = current_block.saturating_sub(self.unbond_at_block);

        blocks_left
    }

    pub fn seconds_left_to_withdraw(&self) -> u64 {
        let timestamp_in_seconds = exec::block_timestamp()
            .saturating_div(1000);
        let current_block  = exec::block_height() as u64;

        if self.can_withdraw_at_block < current_block {
            return 0;
        }

        let blocks_left_in_seconds = self.can_withdraw_at_block.saturating_sub(current_block) * 3;
        let seconds_left = blocks_left_in_seconds.saturating_sub(timestamp_in_seconds);

        seconds_left
    }

    pub fn can_withdraw(&self) -> bool {
        if self.withdrawn || self.rebonded {
            return false;
        }
        
        let current_block = exec::block_height() as u64;

        self.can_withdraw_at_block < current_block
    }

    pub fn can_rebond(&self) -> bool {
        if self.can_withdraw() {
            return false;
        }

        true
    }
}