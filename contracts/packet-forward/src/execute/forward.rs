use crate::error::ContractError;
use crate::msg::ForwardMsg;
use crate::proto::ibc::applications::transfer::v1::MsgTransfer;
use crate::proto::traits::MessageExt;
use crate::state::{FAILED, PENDING, REQUEST_ID};
use cosmwasm_std::{
    Binary, Coin, CosmosMsg, DepsMut, Env, MessageInfo, ReplyOn, Response, StdResult, SubMsg,
    SubMsgResponse,
};

const TRANSFER_PORT: &str = "transfer";

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
        source_port: TRANSFER_PORT.to_string(),
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

pub fn handle_reply_ok(deps: DepsMut, id: u64, _res: SubMsgResponse) -> StdResult<Response> {
    PENDING.remove(deps.storage, id);

    Ok(Response::new())
}

pub fn handle_reply_err(deps: DepsMut, id: u64, _err: String) -> StdResult<Response> {
    let request = PENDING.load(deps.storage, id)?;
    FAILED.save(deps.storage, id, &request)?;
    PENDING.remove(deps.storage, id);

    Ok(Response::new())
}
