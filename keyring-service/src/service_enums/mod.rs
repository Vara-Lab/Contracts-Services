use sails_rs::prelude::*;

use crate::state::KeyringData;

#[derive(Encode, Decode, TypeInfo, Clone, PartialEq, Eq, Debug)]
pub enum KeyringError {
    KeyringAddressAlreadyEsists,
    UserAddressAlreadyExists,
    UserCodedNameAlreadyExists,
    UserDoesNotHasKeyringAccount,
    KeyringAccountAlreadyExists,
    SessionHasInvalidCredentials,
    UserAndKeyringAddressAreTheSame
}

#[derive(Encode, Decode, TypeInfo, Clone)]
pub enum KeyringEvent {
    KeyringAccountSet,
    Error(KeyringError)
}

#[derive(Encode, Decode, TypeInfo, Clone)]
pub enum KeyringQueryEvent {
    KeyringAccountAddress(Option<ActorId>),
    KeyringAccountData(Option<KeyringData>),
}