use crate::{error::ContractError, msgs::UpdateConfigMsg, state::CONFIG, types::Config};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

/// Only owner can execute it.
#[cfg(not(feature = "library"))]
pub fn execute_update_config(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: UpdateConfigMsg,
) -> Result<Response, ContractError> {
    let mut config: Config = CONFIG.load(deps.storage)?;

    // Permission check
    if info.sender != config.owner {
        return Err(ContractError::Unauthorized);
    }

    if let Some(owner) = msg.owner {
        config.owner = deps.api.addr_validate(&owner)?;
    }

    if let Some(treasury) = msg.treasury {
        config.treasury = deps.api.addr_validate(&treasury)?;
    }

    if let Some(fee) = msg.fee {
        config.fee = fee;
    }

    if let Some(timeout) = msg.timeout {
        config.timeout = timeout;
    }

    CONFIG.save(deps.storage, &config)?;
    let resp = Response::new()
        .add_attribute("action", "update_config")
        .add_attribute("owner", config.owner.to_string());

    Ok(resp)
}
