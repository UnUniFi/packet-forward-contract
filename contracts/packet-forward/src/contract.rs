use crate::error::ContractError;
use crate::execute::claim_failed_request::execute_claim_failed_request;
use crate::execute::forward::execute_forward;
use crate::execute::update_config::execute_update_config;
use crate::ibc_hooks::SudoMsg;
use crate::msgs::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::query::config::query_config;
use crate::query::failed_requests::query_failed_requests;
use crate::reply::err::reply_err;
use crate::reply::ok::reply_ok;
use crate::state::CONFIG;
use crate::sudo::ibc_lifecycle_complete::ibc_lifecycle_complete;
use crate::types::Config;
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
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let treasury = deps.api.addr_validate(&msg.treasury)?;

    let config = Config {
        owner: info.sender,
        treasury,
        fee: msg.fee,
    };

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
        ExecuteMsg::Forward(msg) => {
            let coin: Coin = one_coin(&info).map_err(|err| ContractError::Payment(err))?;
            execute_forward(deps, env, info, coin, msg)
        }
        ExecuteMsg::ClaimFailedRequest(msg) => execute_claim_failed_request(deps, env, info, msg),
        ExecuteMsg::UpdateConfig(msg) => execute_update_config(deps, env, info, msg),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    match msg.result {
        SubMsgResult::Ok(res) => reply_ok(deps, msg.id, res),
        SubMsgResult::Err(err) => reply_err(deps, msg.id, err),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
        QueryMsg::FailedRequests { address } => to_binary(&query_failed_requests(deps, address)?),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn sudo(deps: DepsMut, env: Env, msg: SudoMsg) -> Result<Response, ContractError> {
    match msg {
        SudoMsg::IBCLifecycleComplete(msg) => ibc_lifecycle_complete(deps, env, msg),
    }
}
