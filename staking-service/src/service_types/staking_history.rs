use sails_rs::{
    prelude::*,
    gstd::exec
};

#[derive(Debug, Encode, Decode, TypeInfo, Clone)]
pub enum StakingHistory {
    Bond {
        amount: u128,
        bond_at_block: u64,
        bond_at_timestamp: u64,
    },
    Unbond {
        amount: u128,
        unbond_at_block: u64,
        unbond_at_timestamp: u64,
    },
    Rebond {
        amount: u128,
        rebond_at_block: u64,
        rebond_at_timestamp: u64,
    },
    Withdraw {
        amount: u128,
        withdraw_at_block: u64,
        withdraw_at_timestamp: u64,
    }
}

impl StakingHistory {
    pub fn new_bond(value: u128) -> Self {
        let bond_at_block = exec::block_height() as u64;
        let bond_at_timestamp = exec::block_timestamp();
        
        Self::Bond { 
            amount: value, 
            bond_at_block,
            bond_at_timestamp
        }
    }
    
    pub fn new_unbond(value: u128) -> Self {
        let unbond_at_block = exec::block_height() as u64;
        let unbond_at_timestamp = exec::block_timestamp();
        
        Self::Unbond { 
            amount: value, 
            unbond_at_block,
            unbond_at_timestamp
        }
    }

    pub fn new_rebond(value: u128) -> Self {
        let rebond_at_block = exec::block_height() as u64;
        let rebond_at_timestamp = exec::block_timestamp();
        
        Self::Rebond {
            amount: value,
            rebond_at_block,
            rebond_at_timestamp
        }
    }
    
    pub fn new_withdraw(value: u128) -> Self {
        let withdraw_at_block = exec::block_height() as u64;
        let withdraw_at_timestamp = exec::block_timestamp();

        Self::Withdraw { 
            amount: value, 
            withdraw_at_block,
            withdraw_at_timestamp
        }
    }
}