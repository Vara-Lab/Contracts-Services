use sails_rs::{
    prelude::*,
    cell::RefMut
};
use gstd::msg;

use crate::state::{
    KeyringAccounts,
    KeyringData
};
use crate::service_enums::KeyringError;

pub struct KeyringService<'a> {
    state: RefMut<'a, KeyringAccounts>
}

#[service]
impl<'a> KeyringService<'a> {
    // Service "Constructor"
    pub fn new(state: RefMut<'a, KeyringAccounts>) -> Self {
        Self {
            state
        }
    }

    /// ## Binds keyring data to an user address (command method - changes states)
    /// Remote call "keyring_address_from_user_address" exposed to external consumenrs
    /// Returns an enum variant (from KeyringEvent) that will be sent as a response to the user
    /// Is treated as a command, meaning that it will change the state (&mut self)
    /// Returns the keyring address from an user address
    pub fn bind_keyring_data_to_user_address(
        &mut self,
        user_coded_name: String,
        keyring_data: KeyringData
    ) -> KeyringEvent {
        let keyring_address = msg::source();

        let result = self.state
            .set_keyring_account_to_user_coded_name(
                keyring_address, 
                user_coded_name, 
                keyring_data
            );
        
        match result {
            Err(keyring_error) => KeyringEvent::Error(keyring_error),
            Ok(_) => KeyringEvent::KeyringAccountSet
        }
    }

    /// ## Binds keyring data to an user coded name (command method - changes state)
    /// Remote call "keyring_address_from_user_address" exposed to external consumenrs
    /// Returns an enum variant (from KeyringEvent) that will be sent as a response to the user
    /// Is treated as a command, meaning that it will change the state (&mut self)
    /// Returns the keyring address from an user coded name
    pub fn bind_keyring_data_to_user_coded_name(
        &mut self,
        no_wallet_account: String,
        keyring_data: KeyringData
    ) -> KeyringEvent {
        let keyring_address: ActorId = msg::source().into();

        let result = self.state
            .set_keyring_account_to_user_coded_name(
                keyring_address, 
                no_wallet_account, 
                keyring_data
            );

        match result {
            Err(keyring_error) => KeyringEvent::Error(keyring_error),
            Ok(_) => KeyringEvent::KeyringAccountSet
        }
    }
}

#[derive(Encode, Decode, TypeInfo, Clone)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]

pub enum KeyringEvent {
    KeyringAccountSet,
    Error(KeyringError)
}
