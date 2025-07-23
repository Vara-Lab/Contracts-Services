use sails_rs::prelude::*;

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum StakingError {
    ContractEraIsNotSynchronized,
    ActionOnlyForAdmins,
    ValueIsZero,
    ValueLessThanOne,
    ErrorInFirstStageMessage(String),
    ErrorInUpstreamProgram,
    ReplyError {
        payload: String, 
        reason: String
    },
    TokensReadyToWithdraw,
    TokensAlreadyWithdrawn,
    TokensAlreadyRebonded,
    UnbondIdDoesNotExists,
    BondIdOverflow,
    UnbondIdAlreadyWithdrawn(u64),
    UnbondIdWasRebonded(u64),
    UnbondIdOverflow,
    UnbondIdCanNotBeWithdraw {
        can_withdraw_at_block: u64,
        current_block: u64
    },
    RebondIdOverflow,
    UserBondOverflow,
    UserBondUnderflow,
    UserUnbondOverflow,
    UserUnbondUnderflow,
    UserInsufficientBond,
    UserHasNoBonds,
    UserHasNoUnbonds,
    NominateAtLeastOneAddress,
    NominationsAmountError {
        max: u8,
        received: u32
    }
}