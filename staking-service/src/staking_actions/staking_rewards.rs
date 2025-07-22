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
    }
};

impl StakingActions {
    pub async fn collect_rewards() -> Result<(), StakingError> {
        Self::do_collect_rewards(|| {}).await?;
        Ok(())
    }

    pub async fn do_collect_rewards(on_rewards_collected: impl FnOnce()) -> Result<(), StakingError> {
         let staking_state_mut = StakingData::state_mut();
        // let staking_data = &mut staking_state_mut.service_data;
        let last_era_collected = staking_state_mut
            .service_data
            .last_era_rewards_collected;
        let active_era = StakingActions::active_era();

        if last_era_collected > active_era {
            return Err(StakingError::ContractEraIsNotSynchronized);
        }

        if last_era_collected == active_era {
            return Ok(());
        }

        let eras_to_collect_rewards = active_era - last_era_collected;

        let nominations = staking_state_mut
            .nominations
            .iter();

        for nomination in nominations {
            for i in 0..eras_to_collect_rewards {
                let payload = Request::PayoutStakers { 
                    validator_stash: *nomination, 
                    era: (last_era_collected + i + 1) as u32
                };

                debug!(
                    "[Contract] Sending `payout_stakers` message {:?} at staking's state {:?}",
                    payload,
                    staking_state_mut
                );

                let _ = StakingActions::send_to_built_in_actor(0, payload).await?;
            };
        }

        staking_state_mut.service_data.last_era_rewards_collected = active_era;

        on_rewards_collected();

        Ok(())
    }
}