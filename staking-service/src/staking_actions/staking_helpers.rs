use sails_rs::{
    prelude::*,
    gstd::{
        debug,
        msg
    }
};
use gstd::errors::Error;
use vara_contract_utils::utils;
use super::StakingActions;
use crate::{
    service_enums::staking_errors::StakingError,
    state::{
        StakingData, AMOUNT_OF_GAS, BUILTIN_ADDRESS
    }
};

impl StakingActions {
    pub fn active_era() -> u64 {
        let state = StakingData::state_ref();
        if state.on_mainnet {
            utils::mainnet_active_era()
        } else {
            utils::testnet_active_era()
        }
    }

    pub fn check_value(value: u128) -> Result<(), StakingError> {
        if value == 0 {
            return Err(StakingError::ValueIsZero);
        }

        if value < utils::ONE_TOKEN {
            return Err(StakingError::ValueLessThanOne);
        }

        Ok(())
    }

    pub async fn send_to_built_in_actor<E: Encode>(value: u128, payload: E) -> Result<(), StakingError> {
        let temp = msg::send_with_gas_for_reply(
            BUILTIN_ADDRESS, 
            payload, 
            AMOUNT_OF_GAS, 
            value, 
            0
        );

        let msg_future = match temp {
            Ok(msg) => msg,
            Err(e) => return Err(StakingError::ErrorInFirstStageMessage(e.to_string()))
        };

        let result = msg_future.await;
        
        match result {
            Ok(_) => {
                debug!("[Contract] Success reply from builtin actor received");
                Ok(())
            },
            Err(e) => {
                debug!("[Contract] Error reply from builtin actor received: {e:?}");
                let error = match e {
                    Error::ErrorReply(payload, reason) => {
                        StakingError::ReplyError { 
                            payload: payload.to_string(), 
                            reason: reason.to_string() 
                        }
                    },
                    _ => StakingError::ErrorInUpstreamProgram
                };

                Err(error)
            }
        }
    }
}