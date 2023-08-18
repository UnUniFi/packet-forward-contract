use crate::{
    error::ContractError,
    execute::{swap::execute_swap, update_config::execute_update_config},
    msgs::{ExecuteMsg, InstantiateMsg, QueryMsg},
    query::config::query_config,
    state::CONFIG,
    types::{Config, Destination},
};
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw_utils::one_coin;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let first_forward_contract = deps.api.addr_validate(&msg.first_forward_contract)?;
    let treasury = deps.api.addr_validate(&msg.treasury)?;

    if msg.routes.len() == 0 {
        return Err(ContractError::EmptyRoutes);
    }

    if let Destination::Terminal = &msg.routes.last().unwrap().destination {
    } else {
        return Err(ContractError::InvalidLastRouteDestination);
    }

    let config = Config {
        owner: info.sender,
        input_denom: msg.input_denom,
        output_denom: msg.output_denom,
        first_forward_contract,
        routes: msg.routes,
        treasury,
        fee: msg.fee,
        timeout: msg.timeout,
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
        ExecuteMsg::Swap(msg) => {
            let coin: Coin = one_coin(&info).map_err(|err| ContractError::Payment(err))?;
            execute_swap(deps, env, info, coin, msg)
        }
        ExecuteMsg::UpdateConfig(msg) => execute_update_config(deps, env, info, msg),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
    }
}
