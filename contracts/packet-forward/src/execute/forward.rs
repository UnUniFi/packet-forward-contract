use crate::error::ContractError;
use crate::msgs::ForwardMsg;
use crate::proto::ibc::applications::transfer::v1::MsgTransfer;
use crate::proto::traits::MessageExt;
use crate::state::{CONFIG, INITIATED_REQUESTS, REQUEST_ID};
use crate::types::Request;
use cosmwasm_schema::schemars::_serde_json::Value;
use cosmwasm_std::Uint128;
use cosmwasm_std::{
    Binary, Coin, CosmosMsg, Decimal, DepsMut, Env, MessageInfo, ReplyOn, Response, SubMsg,
};
use std::collections::HashMap;

const TRANSFER_PORT: &str = "transfer";

#[cfg(not(feature = "library"))]
pub fn execute_forward(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    coin: Coin,
    msg: ForwardMsg,
) -> Result<Response, ContractError> {
    use cosmwasm_std::BankMsg;

    let config = CONFIG.load(deps.storage)?;

    let id = REQUEST_ID.load(deps.storage)?;
    REQUEST_ID.save(deps.storage, &(id + 1))?;

    let emergency_claimer = deps.api.addr_validate(&msg.emergency_claimer)?;

    let (fee, subtracted) = fee_and_subtracted(coin.amount, config.fee.commission_rate)?;

    let treasury_msg = CosmosMsg::Bank(BankMsg::Send {
        to_address: config.treasury.to_string(),
        amount: vec![Coin::new(fee.u128(), coin.denom.clone())],
    });

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

    let sub_msg = SubMsg {
        id,
        msg: CosmosMsg::Stargate {
            type_url: msg_any.type_url,
            value: Binary(msg_any.value),
        },
        gas_limit: None,
        reply_on: ReplyOn::Always,
    };

    INITIATED_REQUESTS.save(
        deps.storage,
        id,
        &Request {
            id,
            emergency_claimer,
            coin: Coin::new(subtracted.u128(), coin.denom),
        },
    )?;

    let response = Response::new()
        .add_message(treasury_msg)
        .add_submessage(sub_msg);
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
    let mut memo_object: HashMap<String, Value> =
        cosmwasm_schema::schemars::_serde_json::from_str(memo).unwrap_or_default();

    memo_object.insert("ibc_callback".to_string(), contract.into());
    let memo = cosmwasm_schema::schemars::_serde_json::to_string(&memo_object).unwrap();

    memo
}

#[cfg(test)]
mod tests {
    use cosmwasm_schema::schemars::_serde_json::Value;
    use std::collections::HashMap;

    use crate::execute::forward::insert_ibc_callback;

    #[test]
    fn it_works() {
        let mut memo_object: HashMap<String, Value> = HashMap::new();
        memo_object.insert("forward".to_string(), "foo".into());
        memo_object.insert("wasm".to_string(), "bar".into());

        let memo = cosmwasm_schema::schemars::_serde_json::to_string(&memo_object).unwrap();

        let inserted_memo = insert_ibc_callback(&memo, "hoge");

        println!("{}", inserted_memo);
        // TODO: assert
    }
}
