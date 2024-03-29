use cosmwasm_std::{OverflowError, StdError};
use cw_utils::PaymentError;
use thiserror::Error;

/// Never is a placeholder to ensure we don't return any errors
#[derive(Error, Debug)]
pub enum Never {}

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

    #[error("Only contract admin can do this")]
    Unauthorized,
}
