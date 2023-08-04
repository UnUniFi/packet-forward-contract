use crate::error::ContractError;
use crate::msg::{ExecuteMsg, ForwardMsg, InstantiateMsg, UpdateConfigMsg};
use crate::state::{CONFIG, FAILED, PENDING, REQUEST_ID};
use crate::types::Config;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    Coin, CosmosMsg, DepsMut, Env, IbcMsg, MessageInfo, Reply, ReplyOn, Response, StdResult,
    SubMsg, SubMsgResponse, SubMsgResult,
};
use cw_utils::one_coin;

//Initialize the contract.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let config = Config { owner: info.sender };

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
        ExecuteMsg::Forward(msg) => {
            let coin: Coin = one_coin(&info).map_err(|err| ContractError::Payment(err))?;
            execute_forward(deps, env, info, coin, msg)
        }
    }
}

/// Only owner can execute it.
pub fn execute_update_config(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: UpdateConfigMsg,
) -> Result<Response, ContractError> {
    let mut config: Config = CONFIG.load(deps.storage)?;

    // Permission check
    if info.sender != config.owner {
        return Err(ContractError::Unauthorized {});
    }

    if let Some(owner) = msg.owner {
        config.owner = deps.api.addr_validate(&owner)?;
    }

    CONFIG.save(deps.storage, &config)?;
    let resp = Response::new()
        .add_attribute("action", "update_config")
        .add_attribute("owner", config.owner.to_string());

    Ok(resp)
}

pub fn execute_forward(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    coin: Coin,
    msg: ForwardMsg,
) -> Result<Response, ContractError> {
    let config: Config = CONFIG.load(deps.storage)?;

    let id = REQUEST_ID.load(deps.storage)?;
    REQUEST_ID.save(deps.storage, &(id + 1))?;

    let memo = match msg.memo {
        Some(memo) => Some(serde_json_wasm::to_string(&memo).unwrap()),
        None => None,
    };

    let mut response = Response::new().add_submessage(SubMsg {
        id,
        msg: CosmosMsg::Ibc(IbcMsg::Transfer {
            channel_id: msg.channel,
            to_address: msg.receiver,
            amount: coin,
            timeout: msg.timeout,
            memo,
        }),
        gas_limit: None,
        reply_on: ReplyOn::Always,
    });

    Ok(response)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> StdResult<Response> {
    match msg.result {
        SubMsgResult::Ok(res) => handle_reply_ok(deps, msg.id, res),
        SubMsgResult::Err(err) => handle_reply_err(deps, msg.id, err),
    }
}

fn handle_reply_ok(deps: DepsMut, id: u64, res: SubMsgResponse) -> StdResult<Response> {
    PENDING.remove(deps.storage, id);

    Ok(Response::new())
}

fn handle_reply_err(deps: DepsMut, id: u64, err: String) -> StdResult<Response> {
    let request = PENDING.load(deps.storage, id)?;
    FAILED.save(deps.storage, id, &request)?;
    PENDING.remove(deps.storage, id);

    Ok(Response::new())
}
