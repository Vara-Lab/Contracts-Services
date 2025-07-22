use sails_rs::{
    prelude::*,
    gstd::debug
};
use super::StakingActions;
use crate::{
    state::StakingData,
    service_enums::{
        staking_errors::StakingError,
        built_in::*
    },
    service_types::{
        bond_data::BondData,
        user_data::UserData,
        staking_history::StakingHistory
    }
};

impl StakingActions {
    pub fn user_bonds(address: ActorId) -> Option<Vec<BondData>> {
        let staking_state_ref = StakingData::state_ref();

        let user_data = staking_state_ref
            .users_data
            .get(&address);

        if user_data.is_none() {
            return None;
        }

        let bonded_data = user_data
            .unwrap()
            .bond_data_ids
            .iter()
            .map(|bond_id| staking_state_ref
                .bonded_data
                .get(bond_id)
                .unwrap()
                .clone()
            )
            .collect();

        Some(bonded_data)
    }

    pub async fn bond(value: u128, address: ActorId) -> Result<(), StakingError> {
        StakingActions::check_value(value)?;

        let staking_state_mut = StakingData::state_mut();

        let payload = if !staking_state_mut.has_bonded_any {
            Request::Bond { 
                value, 
                payee: RewardAccount::Program 
            }
        } else {
            Request::BondExtra { value }
        };

        debug!(
            "[Contract] Sending `bond` message {:?} at contract's state {:?}",
            payload,
            staking_state_mut
        );

        let current_bonded_id = staking_state_mut.current_bonded_id
            .checked_add(1)
            .ok_or(StakingError::BondIdOverflow)?;
        let bonded_id = staking_state_mut.current_bonded_id;
        let current_active_era = StakingActions::active_era();

        let _ = StakingActions::send_to_built_in_actor(0, payload).await?;

        // Update local state to account for value transfer in pallet
        let bonded_data = BondData::new(value, current_active_era);
        let mut total_bond_overflow = false;

        staking_state_mut.users_data 
            .entry(address)
            .and_modify(|data| {
                let result = data.new_bond(bonded_id, value);

                if let Ok(_) = result {
                    data.add_to_history(StakingHistory::new_bond(value));
                } else {
                    total_bond_overflow = true;
                }
            })
            .or_insert_with(|| {
                let mut user_data = UserData::default();
                let _ = user_data.new_bond(bonded_id, value);

                user_data.add_to_history(StakingHistory::new_bond(value));
                user_data
            });

        // If the overflow occurred, the contract state was not changed.
        if total_bond_overflow {
            return Err(StakingError::UserBondOverflow);
        }
        
        staking_state_mut.bonded_data.insert(
            bonded_id, 
            bonded_data
        );
        staking_state_mut.current_bonded_id = current_bonded_id;
        staking_state_mut.has_bonded_any = true;

        Ok(())
    }
}