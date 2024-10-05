// necesary crates
use sails_rs::prelude::*;

use crate::state::{
    KeyringAccounts,
    KeyringData
};

// Struct KeyringQueryService that will be used for all queries
#[derive(Default, Clone)]
pub struct KeyringQueryService;

#[service]
impl KeyringQueryService {
    // Service constructor
    pub fn new() -> Self {
        Self 
    }

    // Remote call "keyring_address_from_user_address" exposed to external consumenrs
    // Returns an enum variant (from KeyringQueryEvent) that will be sent as a response to the user
    // Is treated as a query, keeping everything unchanged and returning some data. (&self)
    // Returns the keyring address from an user address
    pub fn keyring_address_from_user_address(
        &self,
        user_address: ActorId
    ) -> KeyringQueryEvent {
        let keyring_address = KeyringAccounts::state_ref()
            .keyring_accounts_address_by_user_address
            .get(&user_address);

        KeyringQueryEvent::SignlessAccountAddress(keyring_address.copied())
    }
    
    // Remote call "keyring_address_from_no_wallet_coded_name" exposed to external consumenrs
    // Returns an enum variant (from KeyringQueryEvent) that will be sent as a response to the user
    // Is treated as a query, keeping everything unchanged and returning some data. (&self)
    // Returns the keyring address from an user coded name
    pub fn keyring_address_from_user_coded_name(
        &self,
        user_coded_name: String
    ) -> KeyringQueryEvent {
        let keyring_address = KeyringAccounts::state_ref()
            .keyring_accounts_address_by_user_coded_name
            .get(&user_coded_name);

        KeyringQueryEvent::SignlessAccountAddress(keyring_address.copied())
    }

    // Remote call "keyring_account_data" exposed to external consumenrs
    // Returns an enum variant (from KeyringQueryEvent) that will be sent as a response to the user
    // Is treated as a query, keeping everything unchanged and returning some data. (&self)
    // Returns the keyring coded account from an keyring address
    pub fn keyring_account_data(
        &self,
        keyring_address: ActorId
    ) -> KeyringQueryEvent {
        let signless_data = KeyringAccounts::state_ref()
            .keyring_data_by_keyring_address
            .get(&keyring_address);

        let response = match signless_data {
            Some(data) => Some(data.clone()),
            None => None
        };

        KeyringQueryEvent::SignlessAccountData(response)
    }
}

#[derive(Encode, Decode, TypeInfo, Clone)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum KeyringQueryEvent {
    LastWhoCall(ActorId),
    SignlessAccountAddress(Option<ActorId>),
    SignlessAccountData(Option<KeyringData>),
}

// // Impl to clone Ref of the state
// impl<'a> Clone for KeyringQueryService<'a> {
//     fn clone(&self) -> Self {
//         let keyring_state_ref = Ref::clone(&self.keyring_state_ref);
//         KeyringQueryService {
//             keyring_state_ref
//         }
//     }
// }