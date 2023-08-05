use crate::error::ContractError;
use crate::msgs::ForwardMsg;
use crate::proto::ibc::applications::transfer::v1::MsgTransfer;
use crate::proto::traits::MessageExt;
use crate::state::{CONFIG, INITIATED_REQUESTS, REQUEST_ID};
use crate::types::Request;
use cosmwasm_std::Uint128;
use cosmwasm_std::{
    Binary, Coin, CosmosMsg, Decimal, DepsMut, Env, MessageInfo, ReplyOn, Response, SubMsg,
};

const TRANSFER_PORT: &str = "transfer";

#[cfg(not(feature = "library"))]
fn fee_subtracted_amount(
    amount: Uint128,
    commission_rate: Decimal,
) -> Result<Uint128, ContractError> {
    let fee_subtracted_amount = amount.checked_sub(
        commission_rate
            .checked_mul(Decimal::new(amount))?
            .to_uint_floor(),
    )?;

    Ok(fee_subtracted_amount)
}

#[cfg(not(feature = "library"))]
pub fn execute_forward(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    coin: Coin,
    msg: ForwardMsg,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    let id = REQUEST_ID.load(deps.storage)?;
    REQUEST_ID.save(deps.storage, &(id + 1))?;

    let emergency_claimer = deps.api.addr_validate(&msg.emergency_claimer)?;

    let fee_subtracted_amount = fee_subtracted_amount(coin.amount, config.commission_rate)?;

    INITIATED_REQUESTS.save(
        deps.storage,
        id,
        &Request {
            id,
            emergency_claimer: emergency_claimer.clone(),
            coin: coin.clone(),
        },
    )?;

    let sdk_coin = crate::proto::cosmos::base::v1beta1::Coin {
        denom: coin.denom,
        amount: fee_subtracted_amount.to_string(),
    };

    let msg_any = (MsgTransfer {
        source_port: TRANSFER_PORT.to_string(),
        source_channel: msg.channel.clone(),
        token: Some(sdk_coin),
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
    // TODO: add events

    Ok(response)
}
