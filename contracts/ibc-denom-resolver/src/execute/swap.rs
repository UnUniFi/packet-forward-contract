use crate::error::ContractError;
use crate::helpers::OptionRefMemo;
use crate::memo::construct_packet_memo;
use crate::msgs::SwapMsg;
use crate::state::CONFIG;
use cosmwasm_std::{Binary, CosmosMsg, Decimal, Uint128};
use cosmwasm_std::{Coin, DepsMut, Env, MessageInfo, Response};
use packet_forward::msgs::ForwardMsg;
use packet_forward::proto::ibc::applications::transfer::v1::MsgTransfer;
use packet_forward::proto::traits::MessageExt;

const TRANSFER_PORT: &str = "transfer";

#[cfg(not(feature = "library"))]
pub fn execute_swap(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    coin: Coin,
    msg: SwapMsg,
) -> Result<Response, ContractError> {
    use cosmwasm_std::BankMsg;

    let config = CONFIG.load(deps.storage)?;

    let memo = construct_packet_memo(&msg.receivers, &config.routes, &config.timeout)?;

    let (fee, subtracted) = fee_and_subtracted(
        coin.amount,
        config.fee.commission_rate,
        config.fee.min,
        config.fee.max,
    )?;

    let treasury_msg = CosmosMsg::Bank(BankMsg::Send {
        to_address: config.fee.treasury.to_string(),
        amount: vec![Coin::new(fee.u128(), coin.denom.clone())],
    });

    let sdk_coin = packet_forward::proto::cosmos::base::v1beta1::Coin {
        denom: coin.denom,
        amount: subtracted.to_string(),
    };

    let msg_transfer = if let Some(forward) = memo.forward {
        Ok(MsgTransfer {
            source_port: TRANSFER_PORT.to_string(),
            source_channel: forward.channel.clone(),
            token: Some(sdk_coin),
            sender: env.contract.address.to_string(),
            receiver: forward.receiver,
            timeout_height: None,
            timeout_timestamp: 0,
            memo: forward.next.as_deref().serialize_json(),
        })
    } else {
        if let Some(wasm) = memo.wasm {
            let forward_msg =
                serde_json_wasm::from_str::<ForwardMsg>(wasm.msg.raw_message_fields.as_str())
                    .unwrap();

            Ok(MsgTransfer {
                source_port: TRANSFER_PORT.to_string(),
                source_channel: forward_msg.channel.clone(),
                token: Some(sdk_coin),
                sender: env.contract.address.to_string(),
                receiver: forward_msg.receiver.clone(),
                timeout_height: None,
                timeout_timestamp: 0,
                memo: forward_msg.memo,
            })
        } else {
            Err(ContractError::EmptyRoutes {})
        }
    }?;
    let msg_any = msg_transfer.to_any()?;
    let msg = CosmosMsg::Stargate {
        type_url: msg_any.type_url,
        value: Binary(msg_any.value),
    };

    let response = Response::new().add_message(treasury_msg).add_message(msg);
    // TODO: add events

    Ok(response)
}

#[cfg(not(feature = "library"))]
fn fee_and_subtracted(
    amount: Uint128,
    commission_rate: Decimal,
    min: Uint128,
    max: Uint128,
) -> Result<(Uint128, Uint128), ContractError> {
    let fee = commission_rate
        .checked_mul(Decimal::new(amount))?
        .to_uint_floor()
        .min(min)
        .max(max);
    let subtracted = amount.checked_sub(fee)?;

    Ok((fee, subtracted))
}
