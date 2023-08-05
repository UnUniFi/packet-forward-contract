use crate::error::ContractError;
use crate::msgs::ClaimProfitMsg;
use crate::{state::CONFIG, types::Config};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

/// Only owner can execute it.
#[cfg(not(feature = "library"))]
pub fn execute_claim_profit(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ClaimProfitMsg,
) -> Result<Response, ContractError> {
    let config: Config = CONFIG.load(deps.storage)?;

    // Permission check
    if info.sender != config.owner {
        return Err(ContractError::Unauthorized);
    }

    let recipient = match msg.recipient {
        Some(recipient) => deps.api.addr_validate(&recipient)?,
        None => info.sender,
    };

    // TODO
    println!("{}", recipient);

    let response = Response::new();
    // TODO: add events

    Ok(response)
}
