// necesary crates
use sails_rs::{
    prelude::*,
    cell::Ref
};

use crate::state::{
    KeyringAccounts,
    KeyringData
};

// Struct KeyringQueryService that will be used for all queries
// Data is passed to the service as Ref (query, does not change state)
pub struct KeyringQueryService<'a> {
    keyring_state_ref: Ref<'a, KeyringAccounts>
}

#[service]
impl<'a> KeyringQueryService<'a> {
    // Service constructor
    pub fn new(
        keyring_state_ref: Ref<'a, KeyringAccounts>
    ) -> Self {
        Self {
            keyring_state_ref
        }
    }

    // Remote call "keyring_address_from_user_address" exposed to external consumenrs
    // Returns an enum variant (from KeyringQueryEvent) that will be sent as a response to the user
    // Is treated as a query, keeping everything unchanged and returning some data. (&self)
    // Returns the keyring address from an user address
    pub fn keyring_address_from_user_address(
        &self,
        user_address: ActorId
    ) -> KeyringQueryEvent {
        let keyring_address = self.keyring_state_ref
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
        let keyring_address = self.keyring_state_ref
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
        let signless_data = self.keyring_state_ref
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