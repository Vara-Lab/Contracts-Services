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
    pub async fn nominate(targets: Vec<ActorId>) -> Result<(), StakingError> {
        let staking_state_mut = StakingData::state_mut();

        if targets.len() > 15 {
            return Err(StakingError::NominationsAmountError { 
                max: 15, 
                received: targets.len() as u32 
            });
        }

        if targets.is_empty() {
            return Err(StakingError::NominationsAmountError { 
                max: 15, 
                received: 0 
            });
        }
        
        // Prepare a message to the built-in actor
        let payload = Request::Nominate { targets: targets.clone() };

        debug!(
            "[Contract] Sending `nominate` message {:?} at broker's state {:?}",
            payload, 
            staking_state_mut
        );

        let _ = StakingActions::send_to_built_in_actor(0, payload).await?;

        staking_state_mut.nominations = targets;

        Ok(())
    }
}