use sails_rs::{
    prelude::*,
    gstd::{
        exec,
        debug
    }
};
use super::StakingActions;
use crate::{
    state::StakingData,
    service_enums::{
        staking_errors::StakingError,
        built_in::{
            Request,
            RewardAccount
        }
    }
};

impl StakingActions {
    pub fn get_payee() -> Option<ActorId> {
        let staking_state_ref = StakingData::state_ref();

        staking_state_ref.reward_account
    }

    pub async fn set_payee(payee: RewardAccount) -> Result<(), StakingError> {
        let staking_state_mut = StakingData::state_mut();

        let payload = Request::SetPayee { payee };

        debug!(
            "[Contract] Sending `set_payee` message {:?} at broker's state",
            payload
        );

        let _ = StakingActions::send_to_built_in_actor(0, payload).await?;

        staking_state_mut.reward_account = match payee {
            RewardAccount::Program => Some(exec::program_id()),
            RewardAccount::Custom(account_id) => Some(account_id),
            _ => None
        };

        Ok(())
    }
}

