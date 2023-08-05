use crate::error::ContractError;
use crate::state::INITIATED_REQUESTS;
use cosmwasm_std::{DepsMut, Response};

#[cfg(not(feature = "library"))]
pub fn reply_err(deps: DepsMut, id: u64, _err: String) -> Result<Response, ContractError> {
    INITIATED_REQUESTS.remove(deps.storage, id);

    let response = Response::new();
    // TODO: add events

    Ok(response)
}
