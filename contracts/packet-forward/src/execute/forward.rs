use crate::error::ContractError;
use crate::msgs::ForwardMsg;
use crate::proto::ibc::applications::transfer::v1::{MsgTransfer, MsgTransferResponse};
use crate::proto::traits::MessageExt;
use crate::state::{INITIATED_REQUESTS, PENDING_REQUESTS, REQUEST_ID};
use crate::types::Request;
use cosmwasm_std::{
    Binary, Coin, CosmosMsg, DepsMut, Env, MessageInfo, ReplyOn, Response, SubMsg, SubMsgResponse,
};
use prost::Message;

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

    let emergency_claimer = deps.api.addr_validate(&msg.emergency_claimer)?;

    INITIATED_REQUESTS.save(deps.storage, id, &(emergency_claimer, coin.clone()))?;

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
        timeout_timestamp: env.block.time.nanos() + msg.timeout.as_nanos() as u64,
        memo: msg.memo,
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

pub fn handle_reply_ok(
    deps: DepsMut,
    id: u64,
    res: SubMsgResponse,
) -> Result<Response, ContractError> {
    let transfer_response = MsgTransferResponse::decode(&res.data.unwrap().0[..])?;
    let (addr, coin) = INITIATED_REQUESTS.load(deps.storage, id)?;

    PENDING_REQUESTS.save(
        deps.storage,
        transfer_response.sequence,
        &Request {
            emergency_claimer: addr.clone(),
            coin,
        },
    )?;
    INITIATED_REQUESTS.remove(deps.storage, id);

    Ok(Response::new())
}

pub fn handle_reply_err(deps: DepsMut, id: u64, _err: String) -> Result<Response, ContractError> {
    INITIATED_REQUESTS.remove(deps.storage, id);

    Ok(Response::new())
}
