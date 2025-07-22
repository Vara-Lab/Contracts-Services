use sails_rs::{
    prelude::*,
    gstd::debug
};
use super::StakingActions;
use crate::service_enums::{
    staking_errors::StakingError,
    built_in::Request
};

impl StakingActions {
    pub async fn chill() -> Result<(), StakingError> {
        // Prepare a message to the built-in actor
        let payload = Request::Chill {};

        debug!(
            "[Contract] Sending `chill` message {:?} at broker's state",
            payload
        );

        let _ = StakingActions::send_to_built_in_actor(0, payload).await?;

        Ok(())
    }
}