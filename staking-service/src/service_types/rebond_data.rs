use sails_rs::{
    prelude::*,
    gstd::exec
};

#[derive(Debug, Encode, Decode, TypeInfo)]
pub struct RebondDataIO {
    pub data: RebondData,
    pub id: u64
}

#[derive(Debug, Encode, Decode, TypeInfo, Clone, Copy)]
pub struct RebondData {
    pub amount: u128,
    pub rebond_at_timestamp: u64,
    pub rebond_at_block: u32,
    pub rebond_at_era: u64
}

impl RebondData {
    pub fn new(amount: u128, active_era: u64) -> Self {
        let rebond_at_timestamp = exec::block_timestamp();
        let rebond_at_block = exec::block_height();

        Self {
            amount,
            rebond_at_timestamp,
            rebond_at_block,
            rebond_at_era: active_era
        }
    }
}