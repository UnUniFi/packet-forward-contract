use cosmwasm_std::{OverflowError, StdError};
use cw_utils::PaymentError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Payment(#[from] PaymentError),

    #[error("{0}")]
    Overflow(#[from] OverflowError),

    #[error("Proto encode error")]
    EncodeError(#[from] prost::EncodeError),

    #[error("Proto decode error")]
    DecodeError(#[from] prost::DecodeError),

    #[error("Bech32 error")]
    Bech32(#[from] bech32::Error),

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
