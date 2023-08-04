use crate::{
    error::ContractError,
    execute::{swap::execute_swap, update_config::execute_update_config},
    msg::{ExecuteMsg, InstantiateMsg},
    state::CONFIG,
    types::Config,
};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Coin, DepsMut, Env, MessageInfo, Response};
use cw_utils::one_coin;

//Initialize the contract.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    if msg.routes.len() == 0 {
        return Err(ContractError::EmptyRoutes {});
    }

    let config = Config {
        owner: info.sender,
        denom: msg.denom,
        timeout_seconds: msg.timeout_seconds,
        routes: msg.routes,
        fee: msg.fee,
    };

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new())
}

//Execute the handle messages.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateConfig(msg) => execute_update_config(deps, env, info, msg),
        ExecuteMsg::Swap(msg) => {
            let coin: Coin = one_coin(&info).map_err(|err| ContractError::Payment(err))?;
            execute_swap(deps, env, info, coin, msg)
        }
    }
}
