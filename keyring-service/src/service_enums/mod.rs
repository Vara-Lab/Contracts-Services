use sails_rs::prelude::*;

#[derive(Encode, Decode, TypeInfo, Clone, PartialEq, Eq, Debug)]
// #[codec(crate = sails_rs::scale_codec)]
// #[scale_info(crate = sails_rs::scale_info)]
pub enum KeyringError {
    KeyringAddressAlreadyEsists,
    UserAddressAlreadyExists,
    UserCodedNameAlreadyExists,
    UserDoesNotHasKeyringAccount,
    KeyringAccountAlreadyExists,
    SessionHasInvalidCredentials,
    UserAndKeyringAddressAreTheSame
}