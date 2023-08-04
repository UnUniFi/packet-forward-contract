use std::num::TryFromIntError;
use std::string::FromUtf8Error;
use thiserror::Error;

use cosmwasm_std::StdError;
use cw_utils::PaymentError;

/// Never is a placeholder to ensure we don't return any errors
#[derive(Error, Debug)]
pub enum Never {}

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Bech32 error")]
    Bech32(#[from] bech32::Error),

    #[error("Proto decode error")]
    DecodeError(#[from] prost::DecodeError),

    #[error("{0}")]
    Payment(#[from] PaymentError),

    #[error("Amount larger than 2**64, not supported by ics20 packets")]
    AmountOverflow {},

    #[error("Insufficient funds to redeem voucher on channel")]
    InsufficientFunds {},

    #[error("Only contract admin can do this")]
    Unauthorized,

    #[error("No allowed token")]
    NoAllowedToken {},

    #[error("Execute msg unknown")]
    UnknownRequest {},

    #[error("Maximum address length")]
    MaxAddrLength {},

    #[error("Routes are empty")]
    EmptyRoutes {},
}

impl From<FromUtf8Error> for ContractError {
    fn from(_: FromUtf8Error) -> Self {
        ContractError::Std(StdError::invalid_utf8("parsing denom key"))
    }
}

impl From<TryFromIntError> for ContractError {
    fn from(_: TryFromIntError) -> Self {
        ContractError::AmountOverflow {}
    }
}
