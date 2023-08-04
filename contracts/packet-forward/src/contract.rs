use crate::error::ContractError;
use crate::execute::forward::{execute_forward, handle_reply_err, handle_reply_ok};
use crate::execute::update_config::execute_update_config;
use crate::msgs::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::query::config::query_config;
use crate::query::failed_requests::query_failed_requests;
use crate::query::pending_requests::query_pending_requests;
use crate::state::CONFIG;
use crate::types::Config;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult,
    SubMsgResult,
};
use cw_utils::one_coin;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let config = Config { owner: info.sender };

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateConfig(msg) => execute_update_config(deps, env, info, msg),
        ExecuteMsg::Forward(msg) => {
            let coin: Coin = one_coin(&info).map_err(|err| ContractError::Payment(err))?;
            execute_forward(deps, env, info, coin, msg)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
        QueryMsg::PendingRequests { address } => to_binary(&query_pending_requests(deps, address)?),
        QueryMsg::FailedRequests { address } => to_binary(&query_failed_requests(deps, address)?),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    match msg.result {
        SubMsgResult::Ok(res) => handle_reply_ok(deps, msg.id, res),
        SubMsgResult::Err(err) => handle_reply_err(deps, msg.id, err),
    }
}
