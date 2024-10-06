use sails_rs::{
    prelude::*,
    gstd::{msg, service}
};

use crate::state::{
    KeyringAccounts,
    KeyringData
};
use crate::service_enums::KeyringError;

#[derive(Clone)]
pub struct KeyringService();

#[service]
impl KeyringService {
    // # Init the state of the services
    // IMPORTANT: this related function need to be called in the program 
    // constructor, this initializes the state
    pub fn seed() {
        KeyringAccounts::init_state();
    }

    // Service "Constructor"
    pub fn new() -> Self {
        Self()
    }

    // ## Binds keyring data to an user address (command method - changes states)
    // Remote call "keyring_address_from_user_address" exposed to external consumenrs
    // Returns an enum variant (from KeyringEvent) that will be sent as a response to the user
    // Is treated as a command, meaning that it will change the state (&mut self)
    // Returns the keyring address from an user address
    pub fn bind_keyring_data_to_user_address(
        &mut self,
        user_address: ActorId,
        keyring_data: KeyringData
    ) -> KeyringEvent {
        let keyring_address = msg::source();

        let result = KeyringAccounts::state_mut()
            .set_keyring_account_to_user_address(
                keyring_address, 
                user_address, 
                keyring_data
            );
        
        match result {
            Err(keyring_error) => KeyringEvent::Error(keyring_error),
            Ok(_) => KeyringEvent::KeyringAccountSet
        }
    }

    // ## Binds keyring data to an user coded name (command method - changes state)
    // Remote call "keyring_address_from_user_address" exposed to external consumenrs
    // Returns an enum variant (from KeyringEvent) that will be sent as a response to the user
    // Is treated as a command, meaning that it will change the state (&mut self)
    // Returns the keyring address from an user coded name
    pub fn bind_keyring_data_to_user_coded_name(
        &mut self,
        user_coded_name: String,
        keyring_data: KeyringData
    ) -> KeyringEvent {
        let keyring_address: ActorId = msg::source().into();

        let result = KeyringAccounts::state_mut()
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
}

#[derive(Encode, Decode, TypeInfo, Clone)]
pub enum KeyringEvent {
    KeyringAccountSet,
    Error(KeyringError)
}

