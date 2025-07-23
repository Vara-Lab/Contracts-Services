use sails_rs::{
    prelude::*,
    collections::HashMap
};
use gstd::{actor_id, exec, msg};
use crate::service_types::{staking_history::StakingHistory};

use super::service_types::{
    user_data::UserData,
    bond_data::{
        BondData,
        BondDataIO
    },
    rebond_data::RebondData,
    unbond_data::{
        UnbondData,
        UnbondDataIO
    },
    service_data::ServiceData
};

pub static mut STAKING_CONTRACT_STATE: Option<StakingData> = None;
        
// Staking proxy builtin actor program id (hardcoded for all runtimes)
pub const BUILTIN_ADDRESS: ActorId = actor_id!("0x77f65ef190e11bfecb8fc8970fd3749e94bed66a23ec2f7a3623e785d0816761");
pub const AMOUNT_OF_GAS: u64 = 10_000_000_000;

#[derive(Debug, Default)]
pub struct StakingData {
    /// Registry of the first active era at contract creation
    pub service_data: ServiceData,
    /// Admins to make actions
    pub admins: Vec<ActorId>,
    /// Nominations address
    pub nominations: Vec<ActorId>,
    /// Has bonded any amount yet
    pub has_bonded_any: bool,
    /// Data from user
    pub users_data: HashMap<ActorId, UserData>,
    /// Registry of bonded data by id
    pub bonded_data: HashMap<u64, BondData>,
    /// Registry of unbonded data by id
    pub unbonded_data: HashMap<u64, UnbondData>,
    /// Registry of rebonded data by id
    pub rebonded_data: HashMap<u64, RebondData>,
    /// Current id for bonded data
    pub current_bonded_id: u64,
    /// Current id for unbonded data
    pub current_unbonded_id: u64,
    /// Current id for rebond data
    pub current_rebond_id: u64,
    /// Current reward account where built in will send the rewards
    pub reward_account: Option<ActorId>, 
    /// Boolean to check if the contract is on mainnet
    pub on_mainnet: bool,
}

impl StakingData {
    /// Init the contract state
    pub fn init_state(on_mainnet: bool) {
        let mut state = StakingData::default();
        state.on_mainnet = on_mainnet;
        state.service_data = ServiceData::new(on_mainnet);
        state.reward_account = Some(exec::program_id());
        state.admins.push(msg::source());

        unsafe {
            STAKING_CONTRACT_STATE = Some(state)
        }
    }
}

impl StakingData {
    pub fn is_admin(&self, address: ActorId) -> bool {
        self.admins.contains(&address)
    }

    pub fn user_is_registered(&self, address: &ActorId) -> bool {
        self.users_data.contains_key(address)
    }

    pub fn user_history(&self, address: ActorId) -> Option<Vec<StakingHistory>> {
        if !self.user_is_registered(&address) {
            return None;
        }

        let user_data = self.users_data
            .get(&address)
            .unwrap();

        let history = user_data
            .history
            .clone();

        Some(history)
    }

    pub fn total_bonded_by_user(&self, address: ActorId) -> Option<u128> {
        if !self.user_is_registered(&address) {
            return None;
        }

        let user_data = self.users_data
            .get(&address)
            .unwrap();

        let total_bond = user_data
            .total_bonded;

        Some(total_bond)
    }

    pub fn total_unbonded_by_user(&self, address: ActorId) -> Option<u128> {
        if !self.user_is_registered(&address) {
            return None;
        }

        let user_data = self.users_data
            .get(&address)
            .unwrap();

        let total_unbond = user_data
            .total_unbonded;

        Some(total_unbond)
    }

    pub fn bonded_data_by_user(&self, address: ActorId) -> Option<Vec<BondDataIO>> {
        if !self.user_is_registered(&address) {
            return None;
        }

        let user_data = self.users_data
            .get(&address)
            .unwrap();

        let bonded_data = user_data
            .bond_data_ids
            .iter()
            .map(|bonded_id| {
                let data = self.bonded_data.get(bonded_id).unwrap().clone();
                BondDataIO {
                    data,
                    id: *bonded_id
                }
            })
            .collect();

        Some(bonded_data)
    }

    pub fn unbonded_data_by_user(&self, address: ActorId) -> Option<Vec<UnbondDataIO>> {
        if !self.user_is_registered(&address) {
            return None;
        }

        let user_data = self.users_data
            .get(&address)
            .unwrap();

        let unbonded_data = user_data
            .unbond_data_ids
            .iter()
            .map(|bonded_id| {
                let data = self.unbonded_data.get(bonded_id).unwrap().clone();
                UnbondDataIO { data, id:*bonded_id }
            })
            .collect();

        Some(unbonded_data)
    }

    pub fn user_pending_unbonds(&self, address: ActorId) -> Option<Vec<UnbondDataIO>> {
        if !self.user_is_registered(&address) {
            return None;
        }

        let user_data = self.users_data
            .get(&address)
            .unwrap();

        let mut pending_unbonds_id = Vec::new();

        for unbond_id in user_data.unbond_data_ids.iter() {
            if user_data.unbonds_already_withdrawn_by_id.contains(unbond_id) {
                continue;
            }

            let unbond_data = self.unbonded_data
                .get(unbond_id)
                .unwrap();

            if unbond_data.can_withdraw() {
                continue;
            }

            pending_unbonds_id.push(*unbond_id);
        }

        let pending_unbonds = pending_unbonds_id
            .into_iter()
            .map(|unbond_id| {
                let data = self.unbonded_data
                    .get(&unbond_id)
                    .unwrap()
                    .clone();
                UnbondDataIO { data, id: unbond_id }
            })
            .collect();

        Some(pending_unbonds)
    }

    pub fn user_unbonds_to_withdraw(&self, address: ActorId) -> Option<Vec<UnbondDataIO>> {
        if !self.user_is_registered(&address) {
            return None;
        }

        let user_data = self.users_data
            .get(&address)
            .unwrap();

        let mut pending_unbonds_id = Vec::new();

        for bond_id in user_data.unbond_data_ids.iter() {
            if user_data.unbonds_already_withdrawn_by_id.contains(bond_id) {
                continue;
            }

            let unbond_data = self.unbonded_data
                .get(bond_id)
                .unwrap();

            if !unbond_data.can_withdraw() {
                continue;
            }

            pending_unbonds_id.push(*bond_id);
        }

        let pending_unbonds = pending_unbonds_id
            .into_iter()
            .map(|unbond_id| {
                let data = self.unbonded_data.get(&unbond_id).unwrap().clone();
                UnbondDataIO { data, id: unbond_id }
            })
            .collect();

        Some(pending_unbonds)
    }

    
    pub fn state_mut() -> &'static mut StakingData {
        let state = unsafe { STAKING_CONTRACT_STATE.as_mut() };
        debug_assert!(state.is_some(), "State is not initialized");
        unsafe { state.unwrap_unchecked() }
    }

    pub fn state_ref() -> &'static StakingData {
        unsafe { STAKING_CONTRACT_STATE.as_ref().expect("state is not initialized") }
    }
}
