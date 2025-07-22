use sails_rs::{
    prelude::*,
    gstd::{
        msg,
        exec,
        debug
    }
};
use super::StakingActions;
use crate::{
    state::StakingData,
    service_enums::{
        staking_errors::StakingError,
        built_in::Request
    },
    service_types::staking_history::StakingHistory
};

impl StakingActions {
    pub async fn withdraw(unbond_id: u64, address: ActorId) -> Result<u128, StakingError> {
        let staking_state_mut = StakingData::state_mut();

        let user_data = staking_state_mut.users_data
            .get_mut(&address)
            .ok_or(StakingError::UserHasNoUnbonds)?;

        if !user_data.unbond_data_ids.contains(&unbond_id) {
            return Err(StakingError::UnbondIdDoesNotExists);
        }

        if user_data.unbond_id_already_withdrawn(unbond_id) {
            return Err(StakingError::UnbondIdAlreadyWithdrawn(unbond_id));
        }

        // Prepare a message to the built-in actor
        let payload = Request::WithdrawUnbonded {
            num_slashing_spans: 0,
        };

        let unbond_data = staking_state_mut
            .unbonded_data
            .get_mut(&unbond_id)
            .unwrap();

        if unbond_data.blocks_left_to_withdraw() > 0 {
            return Err(StakingError::UnbondIdCanNotBeWithdraw { 
                can_withdraw_at_block: unbond_data.can_withdraw_at_block, 
                current_block: exec::block_height() as u64
            });
        }

        if unbond_data.rebonded {
            return Err(StakingError::UnbondIdWasRebonded(unbond_id));
        }

        // The first conditional to check if it has already been withdrawn 
        // is done in the conditional "unbond_id_already_withdrawn"

        debug!(
            "[Contract] Sending `withdraw_unbonded` message {:?} at contract's state",
            payload,
        );

        let _ = StakingActions::send_to_built_in_actor(0, payload).await?;

        let amount = unbond_data.amount;

        unbond_data.withdrawn = true;
        
        user_data.total_unbonded -= amount;
        user_data.add_to_history(StakingHistory::new_withdraw(amount));
        user_data.unbonds_already_withdrawn_by_id.push(unbond_id);
        
        msg::send(address, (), amount)
            .expect("Error while sending message");

        Ok(amount)
    }

}