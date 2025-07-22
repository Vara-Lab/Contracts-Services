use sails_rs::{
    prelude::*,
    gstd::debug
};
use super::StakingActions;
use crate::{
    service_enums::{
        built_in::Request, staking_errors::StakingError
    }, 
    service_types::{
        bond_data::BondData, rebond_data::RebondData, staking_history::StakingHistory
    }, 
    state::StakingData
};

impl StakingActions {
    pub async fn rebond(unbond_id: u64, address: ActorId) -> Result<u128, StakingError> {
        let staking_state_mut = StakingData::state_mut();

        let user_data = staking_state_mut
            .users_data
            .get_mut(&address)
            .ok_or(StakingError::UserHasNoUnbonds)?;

        if user_data.unbond_data_ids.contains(&unbond_id) {
            return Err(StakingError::UnbondIdDoesNotExists);
        }

        let unbond_data = staking_state_mut
            .unbonded_data
            .get(&unbond_id)
            .unwrap();


        if !unbond_data.can_rebond() {
            return Err(StakingError::TokensReadyToWithdraw);
        }

        if unbond_data.withdrawn {
            return Err(StakingError::TokensAlreadyWithdrawn);
        }

        if unbond_data.rebonded {
            return Err(StakingError::TokensAlreadyRebonded);
        }

        let value = unbond_data.amount;

        // Prepare a message to the built-in actor
        let payload = Request::Rebond { value };
        
        debug!(
            "[Contract] Sending `rebond` message {:?} at contract's state",
            payload
        );

        let user_data = staking_state_mut
            .users_data
            .get_mut(&address)
            .unwrap();

        let current_rebond_id = staking_state_mut.current_rebond_id
            .checked_add(1)
            .ok_or(StakingError::RebondIdOverflow)?;
        let current_bond_id = staking_state_mut.current_bonded_id
            .checked_add(1)
            .ok_or(StakingError::BondIdOverflow)?;
        let rebond_id = staking_state_mut.current_rebond_id;
        let bond_id = staking_state_mut.current_bonded_id;
        let current_active_era = StakingActions::active_era();
        let bond_data = BondData::new(value, current_active_era);
        let rebond_data = RebondData::new(value, current_active_era);

        let _ = StakingActions::send_to_built_in_actor(value, payload).await?;


        user_data.new_rebond(rebond_id, bond_id, value)?;
        user_data.add_to_history(StakingHistory::new_rebond(value));

        staking_state_mut.current_rebond_id = current_rebond_id;
        staking_state_mut.current_bonded_id = current_bond_id;
        staking_state_mut.bonded_data.insert(bond_id, bond_data);
        staking_state_mut.rebonded_data.insert(rebond_id, rebond_data);
        staking_state_mut
            .unbonded_data
            .entry(unbond_id)
            .and_modify(|unbond_data| unbond_data.rebonded = true);

        Ok(value)
    }
}