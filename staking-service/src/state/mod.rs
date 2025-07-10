use sails_rs::{
    prelude::*,
    collections::HashMap,
    gstd::{debug, msg}
};
use gstd::{actor_id, errors::Error};

use crate::service_enums::{RewardAccount, Request};

// use gbuiltin_staking::{
//     Request,
//     RewardAccount
// };


use vara_contract_utils::utils;

pub static mut STAKING_CONTRACT_STATE: Option<StakingBroker> = None;

// Staking proxy builtin actor program id (hardcoded for all runtimes)
pub const BUILTIN_ADDRESS: ActorId = actor_id!("0x77f65ef190e11bfecb8fc8970fd3749e94bed66a23ec2f7a3623e785d0816761");

/*

[
    "0xac5a66feb36787aa5cceb467ec48569821ee5e00090214cc52fded6c6691316e",
    "0xb09f5ae444516986141bb6d593c3f285b4e8d6c5d6a175ae68ce2f400434a467",
    "0x0e0aa939efe51316faa243858ab443d29c637faef87c36abacb84556710e0e0a",
    "0x0e3e62b9316a091a37a643f42e910e13be8fbfd38c6f57148cb7618ae2739846",
    "0x161d486a56c3f44d0c0041d117351ef71ce9e5c3ae1e13e540b1c5d2374a0e25"
]


*/

#[derive(Debug, Default)]
pub struct StakingBroker {
    /// Admins to make actions
    admins: Vec<ActorId>,
    /// Has bonded any amount yet
    has_bonded_any: bool,
    /// Total debit
    total_debit: u128,
    /// Registry of bonded deposits
    bonded: HashMap<ActorId, u128>,
    /// Reward payee account id
    reward_account: ActorId,
}

impl StakingBroker {
    /// Init the contract state
    pub fn init_state() {
        unsafe {
            STAKING_CONTRACT_STATE = Some(StakingBroker::default())
        }
    }

    pub fn get_total_debit(&self) -> u128 {
        self.total_debit
    }

    pub fn reward_account(&self) -> ActorId {
        self.reward_account
    }

    pub fn bonded_data(&self) -> Vec<(ActorId, u128)> {
        self.bonded
            .iter()
            .map(|(address, deposit)| (*address, *deposit))
            .collect()
    }

    pub async fn bond_only_value(&mut self, value: u128, payee: Option<RewardAccount>) -> Vec<u8> {
        // Prepare a message to the built-in actor
        // Checking the flag to decide whether to use `Bond` or `BondExtra`
        // Note: this is not how you'd do it in a real application, given the
        // Staking pallet `unbonding` logic, but it's enough for the example.
        let payload = if !self.has_bonded_any {
            Request::Bond {
                value,
                payee: payee.unwrap_or(RewardAccount::Program),
            }
        } else {
            Request::BondExtra { value }
        };
        debug!(
            "[StakingBroker] Sending `bond` message {:?} at broker's state {:?}",
            payload, self
        );
        do_send_message(0, payload, || {
            // Update local state to account for value transfer in pallet
            self.bonded
                .entry(msg::source())
                .and_modify(|old| *old += value)
                .or_insert(value);
            self.total_debit += value;
            self.has_bonded_any = true;
            self.reward_account = match payee {
                Some(RewardAccount::Custom(account_id)) => account_id,
                _ => msg::source(),
            };
        })
        .await
    }

    /// Add bonded amount for the contract as both stash and controller.
    pub async fn bond(&mut self, value: u128, payee: Option<RewardAccount>) -> Vec<u8> {
        // Prepare a message to the built-in actor
        // Checking the flag to decide whether to use `Bond` or `BondExtra`
        // Note: this is not how you'd do it in a real application, given the
        // Staking pallet `unbonding` logic, but it's enough for the example.
        let payload = if !self.has_bonded_any {
            Request::Bond {
                value,
                payee: payee.unwrap_or(RewardAccount::Program),
            }
        } else {
            Request::BondExtra { value }
        };
        debug!(
            "[StakingBroker] Sending `bond` message {:?} at broker's state {:?}",
            payload, self
        );
        do_send_message(value, payload, || {
            // Update local state to account for value transfer in pallet
            self.bonded
                .entry(msg::source())
                .and_modify(|old| *old += value)
                .or_insert(value);
            self.total_debit += value;
            self.has_bonded_any = true;
            self.reward_account = match payee {
                Some(RewardAccount::Custom(account_id)) => account_id,
                _ => msg::source(),
            };
        })
        .await
    }

    pub async fn unbond(&mut self, value: u128) -> Vec<u8> {
        let source = msg::source();

        // The sender can unbond only so much as they have bonded
        let value = self.bonded.get(&source).map_or(0, |v| (*v).min(value));

        if value == 0 {
            debug!("[StakingBroker::unbond] No bonded amount");
            utils::panic("No bonded amount");
        }

        // Prepare a message to the built-in actor
        let payload = Request::Unbond { value };
        debug!(
            "[StakingBroker] Sending `unbond` message {:?} at broker's state {:?}",
            payload, self
        );
        do_send_message(0, payload, || {
            // Update local state
            if let Some(old) = self.bonded.get_mut(&source) {
                *old = old.saturating_sub(value);
            }
            self.total_debit = self.total_debit.saturating_sub(value);
        })
        .await
    }

    pub async fn nominate(&mut self, targets: Vec<ActorId>) -> Vec<u8> {
        // Prepare a message to the built-in actor
        let payload = Request::Nominate { targets };
        debug!(
            "[StakingBroker] Sending `nominate` message {:?} at broker's state {:?}",
            payload, self
        );
        do_send_message(0, payload, || {}).await
    }

    pub async fn chill(&mut self) -> Vec<u8> {
        // Prepare a message to the built-in actor
        let payload = Request::Chill {};
        debug!(
            "[StakingBroker] Sending `chill` message {:?} at broker's state {:?}",
            payload, self
        );
        do_send_message(0, payload, || {}).await
    }

    pub async fn rebond(&mut self, value: u128) -> Vec<u8> {
        let source = msg::source();

        // Prepare a message to the built-in actor
        let payload = Request::Rebond { value };
        debug!(
            "[StakingBroker] Sending `rebond` message {:?} at broker's state {:?}",
            payload, self
        );
        do_send_message(value, payload, || {
            // Update local state
            if let Some(old) = self.bonded.get_mut(&source) {
                *old = old.saturating_add(value);
            }
            self.total_debit = self.total_debit.saturating_add(value);
        })
        .await
    }

    pub async fn withdraw_unbonded(&mut self) -> Vec<u8> {
        let _sender = msg::source();

        // Prepare a message to the built-in actor
        let payload = Request::WithdrawUnbonded {
            num_slashing_spans: 0,
        };
        debug!(
            "[StakingBroker] Sending `withdraw_unbonded` message {:?} at broker's state {:?}",
            payload, self
        );
        do_send_message(0, payload, || {
            // TODO: send a part of withdrawn amount to the sender and/or
            // some other users who requested unbonding earlier
        })
        .await
    }

    pub async fn set_payee(&mut self, payee: RewardAccount) -> Vec<u8> {
        // Prepare a message to the built-in actor
        let payload = Request::SetPayee { payee };
        debug!(
            "[StakingBroker] Sending `set_payee` message {:?} at broker's state {:?}",
            payload, self
        );
        do_send_message(0, payload, || {
            self.reward_account = match payee {
                RewardAccount::Custom(account_id) => account_id,
                _ => msg::source(),
            }
        })
        .await
    }

    pub async fn payout_stakers(&mut self, validator_stash: ActorId, era: u32) -> Vec<u8> {
        // Prepare a message to the built-in actor
        let payload = Request::PayoutStakers {
            validator_stash,
            era,
        };
        debug!(
            "[StakingBroker] Sending `payout_stakers` message {:?} at broker's state {:?}",
            payload, self
        );
        do_send_message(0, payload, || {
            // TODO: transfer fraction of rewards to nominators of the `validator_stash`
        })
        .await
    }

    pub fn state_mut() -> &'static mut StakingBroker {
        let state = unsafe { STAKING_CONTRACT_STATE.as_mut() };
        debug_assert!(state.is_some(), "State is not initialized");
        unsafe { state.unwrap_unchecked() }
    }

    pub fn state_ref() -> &'static StakingBroker {
        unsafe { STAKING_CONTRACT_STATE.as_ref().expect("state is not initialized") }
    }
}


/// Do the actual message sending and reply handling.
async fn do_send_message<E: Encode>(value: u128, payload: E, mut on_success: impl FnMut()) -> Vec<u8> {
    let temp = msg::send_for_reply(BUILTIN_ADDRESS, payload, value, 0);
        // .expect("Error sending message")
        // .await;

    let msg_future = match temp {
        Ok(msg) => msg,
        Err(e) => utils::panic(format!("Error first stage message: {}", e.to_string()))
    };

    let result = msg_future.await;
    
    match result {
        Ok(bytes) => {
            debug!("[StakingBroker] Success reply from builtin actor received");
            on_success();

            bytes
        },
        Err(e) => {
            debug!("[StakingBroker] Error reply from builtin actor received: {e:?}");
            let error = match e {
                Error::ErrorReply(payload, _reason) => {
                    // utils::panic(format!("{payload}"));
                    format!("Payload: {}, Reason: {}", payload.to_string(), _reason.to_string())
                },
                _ => "Error in upstream program".to_string()
                // _ => utils::panic("Error in upstream program"),
            };

            utils::panic(error);
        }
    }
}





































































































// // # Struct to manage keyrings account
// // Handles all walletless and signless accounts
// #[derive(Default, Clone)]
// pub struct KeyringAccounts {
//     // Binds the wallet user address with the keyring address (signless)
//     pub keyring_accounts_address_by_user_address: HashMap<ActorId, ActorId>,
//     // Binds the user coded name with the keyring address (walletless)
//     pub keyring_accounts_address_by_user_coded_name: HashMap<String, ActorId>,
//     // Binds the keyring address with its data (keyring encoded data)
//     pub keyring_data_by_keyring_address: HashMap<ActorId, KeyringData>,
// }

// // Utils methods and related functions, used to init the state
// // and get the state as ref or mut
// impl KeyringAccounts {
//     // ## Related function to init the state
//     pub fn init_state() {
//         unsafe {
//             KEYRING_SERVICE_STATE = Some(Self::default())
//         };
//     }

//     // ### Get the keyring service state as ref
//     pub fn state_ref() -> &'static KeyringAccounts {
//         let state = unsafe { KEYRING_SERVICE_STATE.as_ref() };
//         debug_assert!(state.is_some(), "State is not initialized!");
//         unsafe { state.unwrap_unchecked() }
//     }

//     // ### Get the keyring service state as mut
//     pub fn state_mut() -> &'static mut KeyringAccounts {
//         let state = unsafe { KEYRING_SERVICE_STATE.as_mut() };
//         debug_assert!(state.is_some(), "State is not initialized!");
//         unsafe { state.unwrap_unchecked() }
//     }
// }

// // ## Methods to manage keyring accounts
// impl KeyringAccounts {
//     // ### Verify that the keyring address is linked to the user's address
//     pub fn check_keyring_address_by_user_address(
//         &self,
//         keyring_address: ActorId,
//         user_address: ActorId,
//     ) -> Result<(), KeyringError> {
//         // Check if the user and keyring address are the same
//         if keyring_address == user_address {
//             // If true, return an error
//             return Err(KeyringError::UserAndKeyringAddressAreTheSame);
//         }

//         let singless_addres_from_user_address = self
//             .keyring_accounts_address_by_user_address
//             .get(&user_address) // Get the keyring address by the given user address
//             .ok_or(KeyringError::UserDoesNotHasKeyringAccount)?; // if None, return an Err

//         // Check if the stored keyring address is equal to the given keyring address
//         if !keyring_address.eq(singless_addres_from_user_address) {
//             // if not, returns an error
//             return Err(KeyringError::SessionHasInvalidCredentials);
//         }

//         // Returns Ok if the given keyring and user address are related 
//         Ok(())
//     }

//     // ### Verify that the keyring address is linked to the user's coded name
//     pub fn check_keyring_address_by_user_coded_name(
//         &self,
//         keyring_address: ActorId,
//         user_coded_name: String
//     ) -> Result<(), KeyringError> {
//         let signless_address_by_no_wallet_account = self
//             .keyring_accounts_address_by_user_coded_name
//             .get(&user_coded_name) // Get the keyring address by the user's coded name
//             .ok_or(KeyringError::UserDoesNotHasKeyringAccount)?; // if None, return an error

//         // Check if the stored keyring address is equal to the gives user's coded name
//         if !keyring_address.eq(signless_address_by_no_wallet_account) {
//             // in not, return an error
//             return Err(KeyringError::SessionHasInvalidCredentials);
//         }

//         // returns Ok if the given keyring address and the user's coded name are related
//         Ok(())
//     }

//     // ### Store the keyring data
//     // Store and bind the given keyring data with the user's address
//     pub fn set_keyring_account_to_user_address(
//         &mut self, 
//         keyring_address: ActorId,
//         user_address: ActorId,
//         keyring_data: KeyringData
//     ) -> Result<(), KeyringError> {
//         // Check if the user and keyring address are the same
//         if keyring_address == user_address {
//             // If true, return an error
//             return Err(KeyringError::UserAndKeyringAddressAreTheSame);
//         }

//         // Check if the user address already exists
//         if self.keyring_accounts_address_by_user_address.contains_key(&user_address) {
//             // if exists, return an error
//             return Err(KeyringError::UserAddressAlreadyExists);
//         }

//         // Check if the keyring address already exists
//         if self.keyring_data_by_keyring_address.contains_key(&keyring_address) {
//             // if exists, return an error
//             return Err(KeyringError::KeyringAddressAlreadyEsists);
//         }

//         // Bind the keyring address with the keyring data
//         self.add_keyring_data_to_state(keyring_address, keyring_data);

//         // bind the user address with the keyring address
//         self
//             .keyring_accounts_address_by_user_address
//             .insert(user_address, keyring_address);

//         Ok(())
//     }

//     // ### Store the keyring data
//     // Store and bind the given keyring data with the user's address
//     pub fn set_keyring_account_to_user_coded_name(
//         &mut self,
//         keyring_address: ActorId,
//         user_coded_name: String,
//         keyring_data: KeyringData
//     ) -> Result<(), KeyringError> {
//         // Check if the user's coded name already exists in the contract
//         if self.keyring_accounts_address_by_user_coded_name.contains_key(&user_coded_name) {
//             // If exists, return an error
//             return Err(KeyringError::UserCodedNameAlreadyExists);
//         }

//         // Check if the keyring address already exists in the contract
//         if self.keyring_data_by_keyring_address.contains_key(&keyring_address) {
//             // If exists, return an error
//             return Err(KeyringError::KeyringAddressAlreadyEsists);
//         }

//         // Bing the keyring address with the keyring data
//         self.add_keyring_data_to_state(keyring_address, keyring_data);

//         // Bind the keyring address with de user's coded name
//         self
//             .keyring_accounts_address_by_user_coded_name
//             .insert(user_coded_name, keyring_address);

//         Ok(())
//     }

//     fn add_keyring_data_to_state(&mut self, keyring_address: ActorId, keyring_data: KeyringData) {
//         self.keyring_data_by_keyring_address
//             .insert(keyring_address, keyring_data);
//     }
// }

// // # Struct to store the locked keyring data
// #[derive(Encode, Decode, TypeInfo, Clone, Default)]
// pub struct KeyringData {
//     address: String,
//     encoded: String,
// }


// #[cfg(test)]
// mod tests {
//     use core::str::FromStr;

//     use super::*;

//     #[test]
//     fn store_keyring_data_with_user_address() {
//         let mut accounts = KeyringAccounts::default();
//         let user_address = user_address();
//         let keyring_address = keyring_address();
//         let keyring_data = keyring_data();

//         // Add the keyring data to the accounts
//         let temp = accounts.set_keyring_account_to_user_address(keyring_address, user_address, keyring_data);

//         // check if it returns an error
//         if let Err(error) = temp {
//             let error_msg = format!("Give an error with: {:?}", error);
//             panic!("{error_msg}");
//         }

//         // Checks that the keyring data and user address is stored
//         let temp = accounts.keyring_accounts_address_by_user_address
//             .contains_key(&user_address);

//         assert!(temp, "User address is not stored!");

//         // Check that the key address is linked to the user address
//         let temp = accounts.keyring_accounts_address_by_user_address
//             .get(&user_address)
//             .expect("User address is not stored!");

//         assert!(*temp == keyring_address, "The keyring address is not the same");

//         // Check that the keyring address is stored 
//         let temp = accounts.keyring_data_by_keyring_address
//             .contains_key(&keyring_address);

//         assert!(temp, "Keyring account is not stored");

//         // Check if the keyring data and user address exists
//         let temp = accounts.check_keyring_address_by_user_address(
//             keyring_address, 
//             user_address
//         );

//         assert!(temp.is_ok(), "must be Ok"); 
    
//     }

//     #[test]
//     fn store_keyring_data_with_user_coded_name() {
//         let mut accounts = KeyringAccounts::default();
//         let user_coded_name = "jsou3305bnsodheiJJCF9nc".to_string();
//         let keyring_address = keyring_address();
//         let keyring_data = keyring_data();

//         // Add the keyring data to the accounts
//         let temp = accounts.set_keyring_account_to_user_coded_name(keyring_address, user_coded_name.clone(), keyring_data);

//         // check if it returns an error
//         if let Err(error) = temp {
//             let error_msg = format!("Give an error with: {:?}", error);
//             panic!("{error_msg}");
//         }

//         // Checks that the keyring data and user coded name is stored
//         let temp = accounts.keyring_accounts_address_by_user_coded_name
//             .contains_key(&user_coded_name);

//         assert!(temp, "User coded name is not stored!");

//         // Check that the key address is linked to the user coded name
//         let temp = accounts.keyring_accounts_address_by_user_coded_name
//             .get(&user_coded_name)
//             .expect("User coded name is not stored!");

//         assert!(*temp == keyring_address, "The keyring address is not the same");

//         // Check that the keyring address is stored 
//         let temp = accounts.keyring_data_by_keyring_address
//             .contains_key(&keyring_address);

//         assert!(temp, "Keyring account is not stored");

//         // Check if the keyring data and user coded name exists
//         let temp = accounts.check_keyring_address_by_user_coded_name(
//             keyring_address, 
//             user_coded_name
//         );

//         assert!(temp.is_ok(), "Must be Ok!");
//     }

//     #[test]
//     fn fail_store_keyring_data_with_user_address() {
//         let mut accounts = KeyringAccounts::default();
//         let user_address = user_address();
//         let extra_address = extra_address();
//         let keyring_address = keyring_address();
//         let keyring_data = keyring_data();

//         // Must return an error (is the same address)
//         let temp = accounts.check_keyring_address_by_user_address(
//             user_address, 
//             user_address
//         );

//         if let Err(error) = temp {
//             assert_eq!(error, KeyringError::UserAndKeyringAddressAreTheSame, "Incorrect enum error");
//         } else {
//             panic!("The method must return an error!");
//         }

//         // Must return an error (the account does not exists)
//         let temp = accounts.check_keyring_address_by_user_address(
//             user_address, 
//             keyring_address
//         );

//         if let Err(error) = temp {
//             assert_eq!(error, KeyringError::UserDoesNotHasKeyringAccount, "Incorrect enum error");
//         } else {
//             panic!("The method must return an error!");
//         }

//         // Add the keyring data to the accounts
//         let temp = accounts.set_keyring_account_to_user_address(keyring_address, user_address, keyring_data.clone());

//         // check if it returns an error
//         assert!(temp.is_ok(), "Must return Ok");

//         // Check if the keyring data and user address exists
//         let temp = accounts.check_keyring_address_by_user_address(
//             keyring_address, 
//             user_address
//         );

//         assert!(temp.is_ok(), "must be Ok!");

//         // Checks that the keyring data and user address is stored
//         let temp = accounts.keyring_accounts_address_by_user_address
//             .contains_key(&user_address);

//         assert!(temp, "User address is not stored!");

//         // Check that the key address is linked to the user address
//         let temp = accounts.keyring_accounts_address_by_user_address
//             .get(&user_address)
//             .expect("User address is not stored!");

//         assert!(*temp == keyring_address, "The keyring address is not the same");

//         // Check that the keyring address is stored 
//         let temp = accounts.keyring_data_by_keyring_address
//             .contains_key(&keyring_address);

//         assert!(temp, "Keyring account is not stored");

//         // Storing the same user account, must return an error
//         let temp = accounts.set_keyring_account_to_user_address(keyring_address, user_address, keyring_data.clone());

//         if let Err(error) = temp {
//             assert_eq!(error, KeyringError::UserAddressAlreadyExists, "Incorrect enum error!");
//         } else {
//             panic!("The method must return an error!");
//         }

//         // Storing the same keyring address, must return an error
//         let temp = accounts.set_keyring_account_to_user_address(keyring_address, extra_address, keyring_data);

//         if let Err(error) = temp {
//             assert_eq!(error, KeyringError::KeyringAddressAlreadyEsists, "Incorrect enum error!");
//         } else {
//             panic!("The method must return an error");
//         }

//         // Must return error, invalid session
//         let temp = accounts.check_keyring_address_by_user_address(extra_address, user_address);

//         if let Err(error) = temp {
//             assert_eq!(error, KeyringError::SessionHasInvalidCredentials, "Incorrect enum error!");
//         } else {
//             panic!("The method must return an error");
//         }
//     }

//     #[test]
//     fn fail_store_keyring_data_with_user_coded_name() {
//         let mut accounts = KeyringAccounts::default();
//         let user_coded_name = "jsou3305bnsodheiJJCF9nc".to_string();
//         let keyring_address = keyring_address();
//         let keyring_data = keyring_data();

//         // Check if the keyring data and user coded name exists
//         let temp = accounts.check_keyring_address_by_user_coded_name(
//             keyring_address, 
//             user_coded_name.clone()
//         );

//         if let Err(error) = temp {
//             assert_eq!(error, KeyringError::UserDoesNotHasKeyringAccount);
//         } else {
//             panic!("Must return an error!");
//         }

//         // Add the keyring data to the accounts
//         let temp = accounts.set_keyring_account_to_user_coded_name(keyring_address, user_coded_name.clone(), keyring_data.clone());

//         // check if it returns an error
//         if let Err(error) = temp {
//             let error_msg = format!("Give an error with: {:?}", error);
//             panic!("{error_msg}");
//         }

//         // Checks that the keyring data and user coded name is stored
//         let temp = accounts.keyring_accounts_address_by_user_coded_name
//             .contains_key(&user_coded_name);

//         assert!(temp, "User coded name is not stored!");

//         // Check that the key address is linked to the user coded name
//         let temp = accounts.keyring_accounts_address_by_user_coded_name
//             .get(&user_coded_name)
//             .expect("User coded name is not stored!");

//         assert!(*temp == keyring_address, "The keyring address is not the same");

//         // Check that the keyring address is stored 
//         let temp = accounts.keyring_data_by_keyring_address
//             .contains_key(&keyring_address);

//         assert!(temp, "Keyring account is not stored");

//         // Must return an error (coded name already exists);
//         let temp = accounts.set_keyring_account_to_user_coded_name(keyring_address, user_coded_name.clone(), keyring_data.clone());

//         if let Err(error) = temp {
//             assert_eq!(error, KeyringError::UserCodedNameAlreadyExists, "Incorrect enum error");
//         } else {
//             panic!("Must be an error!");
//         }

//         // Must return an error (keyring address already exists)
//         let temp = accounts.set_keyring_account_to_user_coded_name(keyring_address, "testing".to_string(), keyring_data);

//         if let Err(error) = temp {
//             let message = format!("Incorrect enum: {:?}", error);
//             assert_eq!(error, KeyringError::KeyringAddressAlreadyEsists, "{message}");
//         } else {
//             panic!("Must be an error");
//         }

//         // Must return an error (Bad credentials)
//         let temp = accounts.check_keyring_address_by_user_coded_name(
//             extra_address(), 
//             user_coded_name
//         );

//         if let Err(error) = temp {
//             assert_eq!(error, KeyringError::SessionHasInvalidCredentials, "Incorrect enum error");
//         } else {
//             panic!("Must be a error");
//         }
//     }


//     fn user_address() -> ActorId {
//         ActorId::from_str("0xce1e72b25e9bb6894faae535ee72f987168ed0b7af802a97ad5aeee300f85367")
//             .expect("Error while setting user address")
//     }

//     fn keyring_address() -> ActorId {
//         ActorId::from_str("0x74765f2d8a520d08b54828847b37cc0d912e07fd03881c8c13e2605976e60c58")
//             .expect("Error while setting keyring address")
//     }

//     fn extra_address() -> ActorId {
//         ActorId::from_str("0x74765f2d8a520d08b54828847b37cc0d912e07fd03881c8c13e2605976e63c59")
//             .expect("Error while setting keyring address")
//     }

//     fn keyring_data() -> KeyringData {
//         KeyringData {
//             address: "KCIE83445HJSDS".to_string(),
//             encoded: "fdnn3200jOIO92Noaa".to_string()
//         }
//     }
// }