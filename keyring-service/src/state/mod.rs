use sails_rs::{
    prelude::*,
    collections::HashMap
};

use crate::service_enums::KeyringError;

pub static mut KEYRING_SERVICE_STATE: Option<KeyringAccounts> = None;

// # Struct to manage keyrings account
// Handles all walletless and signless accounts
#[derive(Default, Clone)]
pub struct KeyringAccounts {
    // Binds the wallet user address with the keyring address (signless)
    pub keyring_accounts_address_by_user_address: HashMap<ActorId, ActorId>,
    // Binds the user coded name with the keyring address (walletless)
    pub keyring_accounts_address_by_user_coded_name: HashMap<String, ActorId>,
    // Binds the keyring address with its data (keyring encoded data)
    pub keyring_data_by_keyring_address: HashMap<ActorId, KeyringData>,
}

// Utils methods and related functions, used to init the state
// and get the state as ref or mut
impl KeyringAccounts {
    // ## Related function to init the state
    pub fn init_state() {
        unsafe {
            KEYRING_SERVICE_STATE = Some(Self::default())
        };
    }

    // ### Get the keyring service state as ref
    pub fn state_ref() -> &'static KeyringAccounts {
        let state = unsafe { KEYRING_SERVICE_STATE.as_ref() };
        debug_assert!(state.is_some(), "State is not initialized!");
        unsafe { state.unwrap_unchecked() }
    }

    // ### Get the keyring service state as mut
    pub fn state_mut() -> &'static mut KeyringAccounts {
        let state = unsafe { KEYRING_SERVICE_STATE.as_mut() };
        debug_assert!(state.is_some(), "State is not initialized!");
        unsafe { state.unwrap_unchecked() }
    }
}

// ## Methods to manage keyring accounts
impl KeyringAccounts {
    // ### Verify that the keyring address is linked to the user's address
    pub fn check_keyring_address_by_user_address(
        &self,
        keyring_address: ActorId,
        user_address: ActorId,
    ) -> Result<(), KeyringError> {
        // Check if the user and keyring address are the same
        if keyring_address == user_address {
            // If true, return an error
            return Err(KeyringError::UserAndKeyringAddressAreTheSame);
        }

        let singless_addres_from_user_address = self
            .keyring_accounts_address_by_user_address
            .get(&user_address) // Get the keyring address by the given user address
            .ok_or(KeyringError::UserDoesNotHasKeyringAccount)?; // if None, return an Err

        // Check if the stored keyring address is equal to the given keyring address
        if !keyring_address.eq(singless_addres_from_user_address) {
            // if not, returns an error
            return Err(KeyringError::SessionHasInvalidCredentials);
        }

        // Returns Ok if the given keyring and user address are related 
        Ok(())
    }

    // ### Verify that the keyring address is linked to the user's coded name
    pub fn check_keyring_address_by_user_coded_name(
        &self,
        keyring_address: ActorId,
        user_coded_name: String
    ) -> Result<(), KeyringError> {
        let signless_address_by_no_wallet_account = self
            .keyring_accounts_address_by_user_coded_name
            .get(&user_coded_name) // Get the keyring address by the user's coded name
            .ok_or(KeyringError::UserDoesNotHasKeyringAccount)?; // if None, return an error

        // Check if the stored keyring address is equal to the gives user's coded name
        if !keyring_address.eq(signless_address_by_no_wallet_account) {
            // in not, return an error
            return Err(KeyringError::SessionHasInvalidCredentials);
        }

        // returns Ok if the given keyring address and the user's coded name are related
        Ok(())
    }

    // ### Store the keyring data
    // Store and bind the given keyring data with the user's address
    pub fn set_keyring_account_to_user_address(
        &mut self, 
        keyring_address: ActorId,
        user_address: ActorId,
        keyring_data: KeyringData
    ) -> Result<(), KeyringError> {
        // Check if the user and keyring address are the same
        if keyring_address == user_address {
            // If true, return an error
            return Err(KeyringError::UserAndKeyringAddressAreTheSame);
        }

        // Check if the user address already exists
        if self.keyring_accounts_address_by_user_address.contains_key(&user_address) {
            // if exists, return an error
            return Err(KeyringError::UserAddressAlreadyExists);
        }

        // Check if the keyring address already exists
        if self.keyring_data_by_keyring_address.contains_key(&keyring_address) {
            // if exists, return an error
            return Err(KeyringError::KeyringAddressAlreadyEsists);
        }

        // Bind the keyring address with the keyring data
        self.add_keyring_data_to_state(keyring_address, keyring_data);

        // bind the user address with the keyring address
        self
            .keyring_accounts_address_by_user_address
            .insert(user_address, keyring_address);

        Ok(())
    }

    // ### Store the keyring data
    // Store and bind the given keyring data with the user's address
    pub fn set_keyring_account_to_user_coded_name(
        &mut self,
        keyring_address: ActorId,
        user_coded_name: String,
        keyring_data: KeyringData
    ) -> Result<(), KeyringError> {
        // Check if the user's coded name already exists in the contract
        if self.keyring_accounts_address_by_user_coded_name.contains_key(&user_coded_name) {
            // If exists, return an error
            return Err(KeyringError::UserCodedNameAlreadyExists);
        }

        // Check if the keyring address already exists in the contract
        if self.keyring_data_by_keyring_address.contains_key(&keyring_address) {
            // If exists, return an error
            return Err(KeyringError::KeyringAddressAlreadyEsists);
        }

        // Bing the keyring address with the keyring data
        self.add_keyring_data_to_state(keyring_address, keyring_data);

        // Bind the keyring address with de user's coded name
        self
            .keyring_accounts_address_by_user_coded_name
            .insert(user_coded_name, keyring_address);

        Ok(())
    }

    fn add_keyring_data_to_state(&mut self, keyring_address: ActorId, keyring_data: KeyringData) {
        self.keyring_data_by_keyring_address
            .insert(keyring_address, keyring_data);
    }
}

// # Struct to store the locked keyring data
#[derive(Encode, Decode, TypeInfo, Clone, Default)]
pub struct KeyringData {
    address: String,
    encoded: String,
}


#[cfg(test)]
mod tests {
    use core::str::FromStr;

    use super::*;

    #[test]
    fn store_keyring_data_with_user_address() {
        let mut accounts = KeyringAccounts::default();
        let user_address = user_address();
        let keyring_address = keyring_address();
        let keyring_data = keyring_data();

        // Add the keyring data to the accounts
        let temp = accounts.set_keyring_account_to_user_address(keyring_address, user_address, keyring_data);

        // check if it returns an error
        if let Err(error) = temp {
            let error_msg = format!("Give an error with: {:?}", error);
            panic!("{error_msg}");
        }

        // Checks that the keyring data and user address is stored
        let temp = accounts.keyring_accounts_address_by_user_address
            .contains_key(&user_address);

        assert!(temp, "User address is not stored!");

        // Check that the key address is linked to the user address
        let temp = accounts.keyring_accounts_address_by_user_address
            .get(&user_address)
            .expect("User address is not stored!");

        assert!(*temp == keyring_address, "The keyring address is not the same");

        // Check that the keyring address is stored 
        let temp = accounts.keyring_data_by_keyring_address
            .contains_key(&keyring_address);

        assert!(temp, "Keyring account is not stored");

        // Check if the keyring data and user address exists
        let temp = accounts.check_keyring_address_by_user_address(
            keyring_address, 
            user_address
        );

        assert!(temp.is_ok(), "must be Ok"); 
    
    }

    #[test]
    fn store_keyring_data_with_user_coded_name() {
        let mut accounts = KeyringAccounts::default();
        let user_coded_name = "jsou3305bnsodheiJJCF9nc".to_string();
        let keyring_address = keyring_address();
        let keyring_data = keyring_data();

        // Add the keyring data to the accounts
        let temp = accounts.set_keyring_account_to_user_coded_name(keyring_address, user_coded_name.clone(), keyring_data);

        // check if it returns an error
        if let Err(error) = temp {
            let error_msg = format!("Give an error with: {:?}", error);
            panic!("{error_msg}");
        }

        // Checks that the keyring data and user coded name is stored
        let temp = accounts.keyring_accounts_address_by_user_coded_name
            .contains_key(&user_coded_name);

        assert!(temp, "User coded name is not stored!");

        // Check that the key address is linked to the user coded name
        let temp = accounts.keyring_accounts_address_by_user_coded_name
            .get(&user_coded_name)
            .expect("User coded name is not stored!");

        assert!(*temp == keyring_address, "The keyring address is not the same");

        // Check that the keyring address is stored 
        let temp = accounts.keyring_data_by_keyring_address
            .contains_key(&keyring_address);

        assert!(temp, "Keyring account is not stored");

        // Check if the keyring data and user coded name exists
        let temp = accounts.check_keyring_address_by_user_coded_name(
            keyring_address, 
            user_coded_name
        );

        assert!(temp.is_ok(), "Must be Ok!");
    }

    #[test]
    fn fail_store_keyring_data_with_user_address() {
        let mut accounts = KeyringAccounts::default();
        let user_address = user_address();
        let extra_address = extra_address();
        let keyring_address = keyring_address();
        let keyring_data = keyring_data();

        // Must return an error (is the same address)
        let temp = accounts.check_keyring_address_by_user_address(
            user_address, 
            user_address
        );

        if let Err(error) = temp {
            assert_eq!(error, KeyringError::UserAndKeyringAddressAreTheSame, "Incorrect enum error");
        } else {
            panic!("The method must return an error!");
        }

        // Must return an error (the account does not exists)
        let temp = accounts.check_keyring_address_by_user_address(
            user_address, 
            keyring_address
        );

        if let Err(error) = temp {
            assert_eq!(error, KeyringError::UserDoesNotHasKeyringAccount, "Incorrect enum error");
        } else {
            panic!("The method must return an error!");
        }

        // Add the keyring data to the accounts
        let temp = accounts.set_keyring_account_to_user_address(keyring_address, user_address, keyring_data.clone());

        // check if it returns an error
        assert!(temp.is_ok(), "Must return Ok");

        // Check if the keyring data and user address exists
        let temp = accounts.check_keyring_address_by_user_address(
            keyring_address, 
            user_address
        );

        assert!(temp.is_ok(), "must be Ok!");

        // Checks that the keyring data and user address is stored
        let temp = accounts.keyring_accounts_address_by_user_address
            .contains_key(&user_address);

        assert!(temp, "User address is not stored!");

        // Check that the key address is linked to the user address
        let temp = accounts.keyring_accounts_address_by_user_address
            .get(&user_address)
            .expect("User address is not stored!");

        assert!(*temp == keyring_address, "The keyring address is not the same");

        // Check that the keyring address is stored 
        let temp = accounts.keyring_data_by_keyring_address
            .contains_key(&keyring_address);

        assert!(temp, "Keyring account is not stored");

        // Storing the same user account, must return an error
        let temp = accounts.set_keyring_account_to_user_address(keyring_address, user_address, keyring_data.clone());

        if let Err(error) = temp {
            assert_eq!(error, KeyringError::UserAddressAlreadyExists, "Incorrect enum error!");
        } else {
            panic!("The method must return an error!");
        }

        // Storing the same keyring address, must return an error
        let temp = accounts.set_keyring_account_to_user_address(keyring_address, extra_address, keyring_data);

        if let Err(error) = temp {
            assert_eq!(error, KeyringError::KeyringAddressAlreadyEsists, "Incorrect enum error!");
        } else {
            panic!("The method must return an error");
        }

        // Must return error, invalid session
        let temp = accounts.check_keyring_address_by_user_address(extra_address, user_address);

        if let Err(error) = temp {
            assert_eq!(error, KeyringError::SessionHasInvalidCredentials, "Incorrect enum error!");
        } else {
            panic!("The method must return an error");
        }
    }

    #[test]
    fn fail_store_keyring_data_with_user_coded_name() {
        let mut accounts = KeyringAccounts::default();
        let user_coded_name = "jsou3305bnsodheiJJCF9nc".to_string();
        let keyring_address = keyring_address();
        let keyring_data = keyring_data();

        // Check if the keyring data and user coded name exists
        let temp = accounts.check_keyring_address_by_user_coded_name(
            keyring_address, 
            user_coded_name.clone()
        );

        if let Err(error) = temp {
            assert_eq!(error, KeyringError::UserDoesNotHasKeyringAccount);
        } else {
            panic!("Must return an error!");
        }

        // Add the keyring data to the accounts
        let temp = accounts.set_keyring_account_to_user_coded_name(keyring_address, user_coded_name.clone(), keyring_data.clone());

        // check if it returns an error
        if let Err(error) = temp {
            let error_msg = format!("Give an error with: {:?}", error);
            panic!("{error_msg}");
        }

        // Checks that the keyring data and user coded name is stored
        let temp = accounts.keyring_accounts_address_by_user_coded_name
            .contains_key(&user_coded_name);

        assert!(temp, "User coded name is not stored!");

        // Check that the key address is linked to the user coded name
        let temp = accounts.keyring_accounts_address_by_user_coded_name
            .get(&user_coded_name)
            .expect("User coded name is not stored!");

        assert!(*temp == keyring_address, "The keyring address is not the same");

        // Check that the keyring address is stored 
        let temp = accounts.keyring_data_by_keyring_address
            .contains_key(&keyring_address);

        assert!(temp, "Keyring account is not stored");

        // Must return an error (coded name already exists);
        let temp = accounts.set_keyring_account_to_user_coded_name(keyring_address, user_coded_name.clone(), keyring_data.clone());

        if let Err(error) = temp {
            assert_eq!(error, KeyringError::UserCodedNameAlreadyExists, "Incorrect enum error");
        } else {
            panic!("Must be an error!");
        }

        // Must return an error (keyring address already exists)
        let temp = accounts.set_keyring_account_to_user_coded_name(keyring_address, "testing".to_string(), keyring_data);

        if let Err(error) = temp {
            let message = format!("Incorrect enum: {:?}", error);
            assert_eq!(error, KeyringError::KeyringAddressAlreadyEsists, "{message}");
        } else {
            panic!("Must be an error");
        }

        // Must return an error (Bad credentials)
        let temp = accounts.check_keyring_address_by_user_coded_name(
            extra_address(), 
            user_coded_name
        );

        if let Err(error) = temp {
            assert_eq!(error, KeyringError::SessionHasInvalidCredentials, "Incorrect enum error");
        } else {
            panic!("Must be a error");
        }
    }


    fn user_address() -> ActorId {
        ActorId::from_str("0xce1e72b25e9bb6894faae535ee72f987168ed0b7af802a97ad5aeee300f85367")
            .expect("Error while setting user address")
    }

    fn keyring_address() -> ActorId {
        ActorId::from_str("0x74765f2d8a520d08b54828847b37cc0d912e07fd03881c8c13e2605976e60c58")
            .expect("Error while setting keyring address")
    }

    fn extra_address() -> ActorId {
        ActorId::from_str("0x74765f2d8a520d08b54828847b37cc0d912e07fd03881c8c13e2605976e63c59")
            .expect("Error while setting keyring address")
    }

    fn keyring_data() -> KeyringData {
        KeyringData {
            address: "KCIE83445HJSDS".to_string(),
            encoded: "fdnn3200jOIO92Noaa".to_string()
        }
    }
}