use sails_rs::{
    prelude::*,
    gstd::debug
};
use super::StakingActions;
use crate::{
    state::StakingData,
    service_enums::{
        staking_errors::StakingError,
        built_in::Request
    },
    service_types::{
        unbond_data::UnbondData,
        staking_history::StakingHistory
    }
};

impl StakingActions {
    pub fn user_unbonds(address: ActorId) -> Option<Vec<UnbondData>> {
        let staking_state_ref = StakingData::state_mut();

        let user_data = staking_state_ref
            .users_data
            .get(&address);

        if user_data.is_none() {
            return None;
        }

        let unbonded_data = user_data
            .unwrap()
            .unbond_data_ids
            .iter()
            .map(|unbonded_id| staking_state_ref
                .unbonded_data
                .get(unbonded_id)
                .unwrap()
                .clone()
            )
            .collect();

        Some(unbonded_data)
    }
    
    pub async fn unbond(value: u128, address: ActorId) -> Result<u64, StakingError> {
        StakingActions::check_value(value)?;

        let staking_state_mut = StakingData::state_mut();

        // Prepare a message to the built-in actor
        let payload = Request::Unbond { value };

        debug!(
            "[Contract] Sending `unbond` message {:?} at contract's state {:?}",
            payload,
            staking_state_mut
        );

        let user_data = staking_state_mut.users_data
            .get_mut(&address)
            .ok_or(StakingError::UserHasNoBonds)?;

        if user_data.total_bonded < value {
            return Err(StakingError::UserInsufficientBond);
        }

        let total_unbonded = user_data.total_unbonded
            .checked_add(value)
            .ok_or(StakingError::UserUnbondOverflow)?;
        let total_bonded = user_data.total_bonded
            .checked_sub(value)
            .ok_or(StakingError::UserBondUnderflow)?;
        let current_unbonded_id = staking_state_mut.current_unbonded_id
            .checked_add(1)
            .ok_or(StakingError::UnbondIdOverflow)?;
        let active_era = StakingActions::active_era();

        let unbond_data = UnbondData::new(value, active_era);
        let unbond_id = staking_state_mut.current_unbonded_id;

        let _ = Self::send_to_built_in_actor(0, payload).await?;

        user_data.total_bonded = total_bonded;
        user_data.total_unbonded = total_unbonded;
        user_data.unbond_data_ids.push(unbond_id);
        user_data.add_to_history(StakingHistory::new_unbond(value));

        staking_state_mut.current_unbonded_id = current_unbonded_id;
        staking_state_mut.unbonded_data.insert(unbond_id, unbond_data);

        Ok(unbond_id)
    }
}