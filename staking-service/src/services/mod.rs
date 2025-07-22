use sails_rs::{
    prelude::*,
    gstd::msg,
    ActorId
};
use vara_contract_utils::utils;
use crate::{
    service_enums::{
        built_in::RewardAccount, staking_errors::StakingError
    }, service_types::{bond_data::BondData, staking_history::StakingHistory, unbond_data::UnbondData}, staking_actions::StakingActions, state::StakingData
};

#[derive(Decode, Encode, TypeInfo)]
pub enum StakingEvents {
    Bond {
        user: ActorId,
        bond: u128
    },
    Unbond {
        user: ActorId,
        unbond: u128
    },
    Nominated {
        targets: Vec<ActorId>
    },
    Chill,
    Rebond {
        user: ActorId,
        rebond: u128
    },
    Withdraw {
        user: ActorId,
        withdraw: u128
    }
}

#[derive(Default)]
pub struct StakingService;

#[service(events = StakingEvents)] 
impl StakingService {
    // # Init the state of the service
    // IMPORTANT: this related function need to be called in the program 
    // constructor, this initializes the state
    pub fn seed(on_mainnet: bool) {
        StakingData::init_state(on_mainnet);
    }

    // Service "Constructor"
    pub fn new() -> Self {
        Self
    }

    /// Locks the specified amount of tokens (`value`) for staking purposes.
    /// Optionally, sets the reward destination (`payee`) which can be a stash, controller, or program.
    pub async fn bond(&mut self) -> StakingResponse {
        let value = msg::value();
        let source = msg::source();

        let result = StakingActions::bond(value, source).await;

        if let Err(error) = result {
            utils::panic(error);
        }

        let _ = self.emit_event(StakingEvents::Bond { 
            user: source,
            bond: value.saturating_div(utils::ONE_TOKEN)
        });
        
        StakingResponse::Bonded(value)
    }

    /// Starts the unbonding process for the specified amount of staked tokens (`value`).
    /// Tokens will become available for withdrawal after the unbonding period.
    /// The value need to be passed as a normal number (1 = 1 Vara)
    pub async fn unbond(&mut self, value: u128) -> StakingResponse {
        let source = msg::source();
        let result = StakingActions::unbond(
            value.saturating_mul(utils::ONE_TOKEN),
            source
        ).await;

        if let Err(e) = result {
            utils::panic(e);
        }

        let _ = self.emit_event(StakingEvents::Unbond { user: source, unbond: value });

        StakingResponse::Unbonded(value)
    }

    /// Delegates voting power by nominating a list of validator accounts (`targets`)
    pub async fn nominate(&mut self,  targets: Vec<ActorId>) -> StakingResponse {
        let source = msg::source();
        let state = StakingData::state_mut();

        if !state.is_admin(source) {
            utils::panic(StakingError::ActionOnlyForAdmins);
        }

        let result = StakingActions::nominate(targets.clone()).await;

        if let Err(e) = result {
            utils::panic(e);
        }

        let _ = self.emit_event(StakingEvents::Nominated { targets: targets.clone() });

        StakingResponse::Nominated { validators: targets.clone(), total: targets.len() as u64 }
    }

    /// Stops participating in staking without unbonding the tokens.
    /// Useful to pause staking activity temporarily.
    pub async fn chill(&mut self) -> StakingResponse {
        let source = msg::source();
        let state = StakingData::state_ref();

        if !state.is_admin(source) {
            utils::panic(StakingError::ActionOnlyForAdmins);
        }

        let result = StakingActions::chill().await;

        if let Err(e) = result {
            utils::panic(e);
        }

        let _ = self.emit_event(StakingEvents::Chill);

        StakingResponse::Chill
    }

    /// Re-bonds tokens that are currently in the unbonding state,
    /// effectively cancelling the unbonding request for the specified `value`.
    pub async fn rebond(&mut self, unbond_id: u64) -> StakingResponse{
        let source = msg::source();

        let result = StakingActions::rebond(unbond_id, source).await;

        if let Err(e) = result {
            utils::panic(e);
        }

        let amount = result.unwrap();

        let _ = self.emit_event(StakingEvents::Rebond { user: source, rebond: amount });

        StakingResponse::Rebond(amount)
    }

    /// Finalizes the unbonding process by withdrawing tokens that have completed
    /// the unbonding period and are ready to be reclaimed.
    pub async fn withdraw_unbonded(&mut self, unbond_id: u64) -> StakingResponse {
        let source = msg::source();
        let result = StakingActions::withdraw(unbond_id, source).await;

        if let Err(e) = result {
            utils::panic(e);
        }

        let amount = result.unwrap();

        let _ = self.emit_event(StakingEvents::Withdraw { user: source, withdraw: amount });

        StakingResponse::Withdraw(amount)
    }

    /// Updates the destination where staking rewards will be sent.
    /// Can be set to stash, controller, or a smart contract program.
    pub async fn set_payee(&mut self, payee: RewardAccount) -> StakingResponse {
        let staking_state_mut = StakingData::state_ref();
        let source = msg::source();

        if !staking_state_mut.is_admin(source) {
            utils::panic(StakingError::ActionOnlyForAdmins);
        }

        let result = StakingActions::set_payee(payee).await;

        if let Err(error) = result {
            utils::panic(error);
        }

        StakingResponse::PayeeSet
    }

    // Collect all rewards from pending eras for staking stash (nominations)
    pub async fn collect_rewards(&mut self) ->  StakingResponse {
        let source = msg::source();
        let staking_state_ref = StakingData::state_ref();

        if !staking_state_ref.is_admin(source) {
            utils::panic(StakingError::ActionOnlyForAdmins);
        }

        let result = StakingActions::collect_rewards().await;

        if let Err(e) = result {
            utils::panic(e);
        }

        StakingResponse::RewardsCollected
    }
    
    // Returns the total number of pending eras to request rewards from the nominees
    pub fn num_of_eras_to_get_rewards(&self) -> u32 {
        let staking_state_ref = StakingData::state_ref();
        let active_era = StakingActions::active_era();
        let last_era_collected = staking_state_ref
            .service_data
            .last_era_rewards_collected;

        active_era.saturating_sub(last_era_collected) as u32
    }

    // Returns the list of nominees
    pub fn nominations(&self) -> Vec<ActorId> {
        let staking_state_ref = StakingData::state_mut();

        staking_state_ref.nominations.clone()
    }

    // Returns the history from a user address
    pub fn user_history(&self, user_address: ActorId) -> Option<Vec<StakingHistory>> {
        let staking_state_ref = StakingData::state_ref();
        staking_state_ref.user_history(user_address)
    }

    // Return all the bond that a user makes to the contract
    pub fn user_total_bond(&self, user_address: ActorId) -> Option<u128> {
        let staking_state_ref = StakingData::state_ref();
        staking_state_ref.total_bonded_by_user(user_address)
    }

    // Returns all the unbond that a user makes to the contract
    pub fn user_total_unbond(&self, user_address: ActorId) -> Option<u128> {
        let staking_state_ref = StakingData::state_ref();
        staking_state_ref.total_unbonded_by_user(user_address)
    }

    // Returns all the bonds data from a user address
    pub fn user_bonds(&self, user_address: ActorId) -> Option<Vec<BondData>> {
        let staking_state_ref = StakingData::state_ref();
        staking_state_ref.bonded_data_by_user(user_address)
    }

    // Returns all the unbonds data from a user address
    pub fn user_unbonds(&self, user_address: ActorId) -> Option<Vec<UnbondData>> {
        let staking_state_ref = StakingData::state_ref();
        staking_state_ref.unbonded_data_by_user(user_address)
    }

    // Returns all pending unbonds to be withdrawn from a user address
    pub fn user_pending_unbonds(&self, user_address: ActorId) -> Option<Vec<UnbondData>> {
        let staking_state_ref: &'static StakingData = StakingData::state_ref();
        staking_state_ref.user_pending_unbonds(user_address)
    }

    // Returns the unbonds that can be withdraw from a user
    pub fn user_unbonds_to_withdraw(&self, user_address: ActorId) -> Option<Vec<UnbondData>> {
        let staking_state_ref = StakingData::state_ref();
        staking_state_ref.user_unbonds_to_withdraw(user_address)
    }
}

#[derive(Encode, Decode, TypeInfo)]
pub enum StakingResponse {
    Bonded(u128),
    Unbonded(u128),
    Nominated {
        validators: Vec<ActorId>,
        total: u64 
    },
    Chill,
    Rebond(u128),
    Withdraw(u128),
    PayeeSet,
    RewardsCollected
}