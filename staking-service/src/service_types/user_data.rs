use sails_rs::prelude::*;
use super::staking_history::StakingHistory;
use crate::service_enums::staking_errors::StakingError;

#[derive(Debug, Default)]
pub struct UserData {
    pub total_bonded: u128,
    pub total_unbonded: u128,
    pub bond_data_ids: Vec<u64>,
    pub unbond_data_ids: Vec<u64>,
    pub rebond_data_ids: Vec<u64>,
    pub unbonds_already_withdrawn_by_id: Vec<u64>,
    pub history: Vec<StakingHistory>
}

impl UserData {
    /// Adds a new bond to the user data (bond id and the bond amount)
    pub fn new_bond(&mut self, bond_id: u64, amount: u128) -> Result<(), StakingError> {
        let value = self.total_bonded
            .checked_add(amount)
            .ok_or(StakingError::UserBondOverflow)?;

        self.bond_data_ids.push(bond_id);
        self.total_bonded = value;

        Ok(())
    }

    /// Adds a new rebond to the user data
    pub fn new_rebond(&mut self, rebond_id: u64, bond_id: u64, amount: u128) -> Result<(), StakingError> {
        self.new_bond(bond_id, amount)?;
        let value = self.total_unbonded
            .checked_sub(amount)
            .ok_or(StakingError::UserUnbondUnderflow)?;
        self.total_unbonded = value;
        self.rebond_data_ids.push(rebond_id);

        Ok(())
    }

    /// Add a new action to the user history
    pub fn add_to_history(&mut self, action: StakingHistory) {
        self.history.push(action);
    }

    /// Method to clear the user history
    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    pub fn unbond_id_already_withdrawn(&self, unbond_id: u64) -> bool {
        self.unbonds_already_withdrawn_by_id.contains(&unbond_id)
    }
}