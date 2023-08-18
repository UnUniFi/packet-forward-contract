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

    #[error("Only contract admin can do this")]
    Unauthorized,

    #[error("Routes are empty")]
    EmptyRoutes,

    #[error("The destination of the last route must be Destination::Terminal")]
    InvalidLastRouteDestination,

    #[error("The length of receivers must be same to the length of routes")]
    InvalidReceiversLength,

    #[error("Route is designated only for terminal")]
    InvalidRoutes,

    #[error("Wrong denom: {0}, expected: {1}")]
    WrongDenom(String, String),
}
