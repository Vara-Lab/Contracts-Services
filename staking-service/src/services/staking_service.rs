use sails_rs::{
    prelude::*,
    gstd::msg,
    ActorId
};
use crate::state::StakingBroker;
use crate::service_enums::*;
// use gbuiltin_staking::RewardAccount;

#[derive(Default)]
pub struct StakingService;

#[service]
impl StakingService {
    // # Init the state of the service
    // IMPORTANT: this related function need to be called in the program 
    // constructor, this initializes the state
    pub fn seed() {
        StakingBroker::init_state();
    }

    // Service "Constructor"
    pub fn new() -> Self {
        Self
    }

    pub async fn bond_contract_tokens(&mut self, value: u128, payee: Option<RewardAccount>) -> StakingResponse {
        let response = StakingBroker::state_mut()
            .bond(value, payee)
            .await;
        
        StakingResponse::BuiltInActorResponse(response)
    }

    pub async fn bond_only_value(&mut self, value: u128, payee: Option<RewardAccount>) -> StakingResponse {
        let response = StakingBroker::state_mut()
            .bond_only_value(value, payee)
            .await;

        StakingResponse::BuiltInActorResponse(response)
    }

    /// Locks the specified amount of tokens (`value`) for staking purposes.
    /// Optionally, sets the reward destination (`payee`) which can be a stash, controller, or program.
    // pub async fn bond(&mut self, value: u128, payee: Option<RewardAccount>) -> StakingResponse {
    pub async fn bond(&mut self, payee: Option<RewardAccount>) -> StakingResponse {
        let value = msg::value();
        let response = StakingBroker::state_mut()
            .bond(value, payee)
            .await;
        
        StakingResponse::BuiltInActorResponse(response)
    }

    /// Starts the unbonding process for the specified amount of staked tokens (`value`).
    /// Tokens will become available for withdrawal after the unbonding period.
    pub async fn unbond(&mut self, value: u128) -> StakingResponse {
        let response = StakingBroker::state_mut()
            .unbond(value)
            .await;

        StakingResponse::BuiltInActorResponse(response)
    }

    /// Delegates voting power by nominating a list of validator accounts (`targets`)
    pub async fn nominate(&mut self,  targets: Vec<ActorId>) -> StakingResponse {
        let response =  StakingBroker::state_mut()
            .nominate(targets)
            .await;

        StakingResponse::BuiltInActorResponse(response)
    }

    /// Stops participating in staking without unbonding the tokens.
    /// Useful to pause staking activity temporarily.
    pub async fn chill(&mut self) -> StakingResponse {
        let response = StakingBroker::state_mut()
            .chill()
            .await;

        StakingResponse::BuiltInActorResponse(response)
    }

    /// Re-bonds tokens that are currently in the unbonding state,
    /// effectively cancelling the unbonding request for the specified `value`.
    pub async fn rebond(&mut self, value: u128) -> StakingResponse{
        let response = StakingBroker::state_mut()
            .rebond(value)
            .await;

        StakingResponse::BuiltInActorResponse(response)
    }

    /// Finalizes the unbonding process by withdrawing tokens that have completed
    /// the unbonding period and are ready to be reclaimed.
    pub async fn withdraw_unbonded(&mut self) -> StakingResponse {
        let response = StakingBroker::state_mut()
            .withdraw_unbonded()
            .await;

        StakingResponse::BuiltInActorResponse(response)
    }

    /// Updates the destination where staking rewards will be sent.
    /// Can be set to stash, controller, or a smart contract program.
    pub async fn set_payee(&mut self, payee: RewardAccount) -> StakingResponse {
        let response = StakingBroker::state_mut()
            .set_payee(payee)
            .await;

        StakingResponse::BuiltInActorResponse(response)
    }

    /// Triggers reward payout for nominators and the specified validator
    /// for a given staking era.
    pub async fn payout_stakers(&mut self, validator_stash: ActorId, era: u32) -> StakingResponse {
        let response = StakingBroker::state_mut()
            .payout_stakers(validator_stash, era)
            .await;

        StakingResponse::BuiltInActorResponse(response)
    }

    pub fn get_reward_account(&self) -> u128 {
        StakingBroker::state_ref()
            .get_total_debit()
    }

    pub fn get_bonded_data(&self) -> Vec<(ActorId, u128)> {
        StakingBroker::state_ref()
            .bonded_data()
    }
}

#[derive(Encode, Decode, TypeInfo)]
pub enum StakingResponse {
    BuiltInActorResponse(Vec<u8>)
}