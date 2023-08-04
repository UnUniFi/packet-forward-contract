use crate::error::ContractError;
use crate::msg::{ExecuteMsg, ForwardMsg, InstantiateMsg, UpdateConfigMsg};
use crate::proto::ibc::applications::transfer::v1::MsgTransfer;
use crate::proto::traits::MessageExt;
use crate::state::{CONFIG, FAILED, PENDING, REQUEST_ID};
use crate::types::Config;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    Binary, Coin, CosmosMsg, DepsMut, Env, MessageInfo, Reply, ReplyOn, Response, StdResult,
    SubMsg, SubMsgResponse, SubMsgResult,
};
use cw_utils::one_coin;

//Initialize the contract.
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
    _info: MessageInfo,
    coin: Coin,
    msg: ForwardMsg,
) -> Result<Response, ContractError> {
    let id = REQUEST_ID.load(deps.storage)?;
    REQUEST_ID.save(deps.storage, &(id + 1))?;

    let memo = match msg.memo {
        Some(memo) => serde_json_wasm::to_string(&memo).unwrap(),
        None => "".to_string(),
    };

    let msg_any = (MsgTransfer {
        source_port: "transfer".to_string(),
        source_channel: msg.channel.clone(),
        token: Some(crate::proto::cosmos::base::v1beta1::Coin {
            denom: coin.denom,
            amount: coin.amount.to_string(),
        }),
        sender: env.contract.address.to_string(),
        receiver: msg.receiver.clone(),
        timeout_height: None,
        timeout_timestamp: 0,
        memo,
    })
    .to_any()?;

    let response = Response::new().add_submessage(SubMsg {
        id,
        msg: CosmosMsg::Stargate {
            type_url: msg_any.type_url,
            value: Binary(msg_any.value),
        },
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

fn handle_reply_ok(deps: DepsMut, id: u64, _res: SubMsgResponse) -> StdResult<Response> {
    PENDING.remove(deps.storage, id);

    Ok(Response::new())
}

fn handle_reply_err(deps: DepsMut, id: u64, _err: String) -> StdResult<Response> {
    let request = PENDING.load(deps.storage, id)?;
    FAILED.save(deps.storage, id, &request)?;
    PENDING.remove(deps.storage, id);

    Ok(Response::new())
}
