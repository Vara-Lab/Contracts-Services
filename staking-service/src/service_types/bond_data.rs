use sails_rs::{
    prelude::*,
    gstd::exec
};

#[derive(Debug, Encode, Decode, TypeInfo)]
pub struct BondDataIO {
    pub data: BondData,
    pub id: u64
}

#[derive(Debug, Encode, Decode, TypeInfo, Clone, Copy)]
pub struct BondData {
    pub amount: u128,
    pub bonded_at_timestamp: u64,
    pub bonded_at_block: u32,
    pub bonded_at_era: u64,
}

impl BondData {
    pub fn new(amount: u128, active_era: u64) -> Self {
        let bonded_at_timestamp = exec::block_timestamp();
        let bonded_at_block = exec::block_height();

        Self { 
            amount, 
            bonded_at_timestamp, 
            bonded_at_block,
            bonded_at_era: active_era,
        }
    }
}