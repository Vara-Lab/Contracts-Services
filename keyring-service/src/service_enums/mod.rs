use sails_rs::prelude::*;

use crate::state::KeyringData;

#[derive(Encode, Decode, TypeInfo, Clone, PartialEq, Eq, Debug)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
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
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum KeyringEvent {
    KeyringAccountSet,
    Error(KeyringError)
}

#[derive(Encode, Decode, TypeInfo, Clone)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum KeyringQueryEvent {
    KeyringAccountAddress(Option<ActorId>),
    KeyringAccountData(Option<KeyringData>),
}