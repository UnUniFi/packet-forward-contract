use crate::error::ContractError;
use crate::msgs::ForwardMsg;
use crate::proto::ibc::applications::transfer::v1::MsgTransfer;
use crate::proto::traits::MessageExt;
use crate::state::get_request_id;
use crate::state::get_sub_msg_id;
use crate::state::SUB_MSG_TYPE;
use crate::state::{CONFIG, INITIATED_REQUESTS};
use crate::types::Request;
use crate::types::SubMsgType;
use cosmwasm_std::BankMsg;
use cosmwasm_std::Uint128;
use cosmwasm_std::{
    Binary, Coin, CosmosMsg, Decimal, DepsMut, Env, MessageInfo, ReplyOn, Response, SubMsg,
};
use serde_cw_value::Value;
use std::collections::BTreeMap;

const TRANSFER_PORT: &str = "transfer";

#[cfg(not(feature = "library"))]
pub fn execute_forward(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    coin: Coin,
    msg: ForwardMsg,
) -> Result<Response, ContractError> {
    let mut response = Response::new();

    let config = CONFIG.load(deps.storage)?;

    let emergency_claimer = deps.api.addr_validate(&msg.emergency_claimer)?;

    let (fee, subtracted) = fee_and_subtracted(coin.amount, config.fee.commission_rate)?;

    if !fee.is_zero() {
        // fee subtraction may happen separately from the swap process.
        // But, as of forward, there's high probability that the fee rate will be set at 0.
        let treasury_msg = CosmosMsg::Bank(BankMsg::Send {
            to_address: config.treasury.to_string(),
            amount: vec![Coin::new(fee.u128(), coin.denom.clone())],
        });
        response = response.add_message(treasury_msg);
    }

    let sdk_coin = crate::proto::cosmos::base::v1beta1::Coin {
        denom: coin.denom.clone(),
        amount: subtracted.to_string(),
    };

    let contract = env.contract.address.to_string();

    let msg_any = (MsgTransfer {
        source_port: TRANSFER_PORT.to_string(),
        source_channel: msg.channel.clone(),
        token: Some(sdk_coin),
        sender: contract.clone(),
        receiver: msg.receiver.clone(),
        timeout_height: None,
        timeout_timestamp: env.block.time.nanos() + msg.timeout.as_nanos() as u64,
        memo: insert_ibc_callback(&msg.memo, &contract),
    })
    .to_any()?;

    let sub_msg_id = get_sub_msg_id(deps.storage)?;
    let request_id = get_request_id(deps.storage)?;

    let sub_msg = SubMsg {
        id: sub_msg_id,
        msg: CosmosMsg::Stargate {
            type_url: msg_any.type_url,
            value: Binary(msg_any.value),
        },
        gas_limit: None,
        reply_on: ReplyOn::Always,
    };

    response = response.add_submessage(sub_msg);

    SUB_MSG_TYPE.save(deps.storage, sub_msg_id, &SubMsgType::InitiateRequest)?;

    INITIATED_REQUESTS.save(
        deps.storage,
        sub_msg_id,
        &Request {
            id: request_id,
            emergency_claimer,
            coin: Coin::new(subtracted.u128(), coin.denom),
        },
    )?;

    response = response.add_attribute("action", "forward");
    // TODO: add events

    Ok(response)
}

#[cfg(not(feature = "library"))]
fn fee_and_subtracted(
    amount: Uint128,
    commission_rate: Decimal,
) -> Result<(Uint128, Uint128), ContractError> {
    let fee = commission_rate
        .checked_mul(Decimal::new(amount))?
        .to_uint_floor();
    let subtracted = amount.checked_sub(fee)?;

    Ok((fee, subtracted))
}

#[cfg(not(feature = "library"))]
fn insert_ibc_callback(memo: &str, contract: &str) -> String {
    let mut memo_object: BTreeMap<String, Value> =
        serde_json_wasm::from_str(memo).unwrap_or_default();

    memo_object.insert(
        "ibc_callback".to_string(),
        Value::String(contract.to_string()),
    );
    let memo = serde_json_wasm::to_string(&memo_object).unwrap();

    memo
}

#[cfg(test)]
mod tests {
    use serde_cw_value::Value;
    use std::collections::BTreeMap;

    use crate::execute::forward::insert_ibc_callback;

    #[test]
    fn it_works() {
        let mut memo_object: BTreeMap<String, Value> = BTreeMap::new();
        memo_object.insert("forward".to_string(), Value::String("foo".to_string()));
        memo_object.insert("wasm".to_string(), Value::String("bar".to_string()));

        let memo = serde_json_wasm::to_string(&memo_object).unwrap();

        let inserted_memo = insert_ibc_callback(&memo, "hoge");

        println!("{}", inserted_memo);
        // TODO: assert
    }
}
