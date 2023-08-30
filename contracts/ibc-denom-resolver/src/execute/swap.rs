use crate::error::ContractError;
use crate::memo::construct_packet_memo;
use crate::msgs::SwapMsg;
use crate::state::CONFIG;
use cosmwasm_std::{to_binary, BankMsg, WasmMsg};
use cosmwasm_std::{Coin, DepsMut, Env, MessageInfo, Response};
use cosmwasm_std::{CosmosMsg, Decimal, Uint128};
use packet_forward::msgs::ForwardMsg;
use packet_forward::msgs::ExecuteMsg as PacketForwardMsg;

#[cfg(not(feature = "library"))]
pub fn execute_swap(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    coin: Coin,
    msg: SwapMsg,
) -> Result<Response, ContractError> {
    let mut response = Response::new();
    let config = CONFIG.load(deps.storage)?;

    if msg.receivers.len() != config.routes.len() {
        return Err(ContractError::InvalidReceiversLength);
    }

    if !is_correct_denom(&coin.denom, &config.input_denom) {
        ContractError::WrongDenom(coin.denom.clone(), config.input_denom.clone());
    }

    let (fee, subtracted) = fee_and_subtracted(
        coin.amount,
        config.fee.commission_rate,
        config.fee.min,
        config.fee.max,
    )?;

    if !fee.is_zero() {
        // fee subtraction may happen separately from the forward process.
        let treasury_msg = CosmosMsg::Bank(BankMsg::Send {
            to_address: config.treasury.to_string(),
            amount: vec![Coin::new(fee.u128(), coin.denom.clone())],
        });
        response = response.add_message(treasury_msg);
    }

    let memo = construct_packet_memo(&msg.receivers, &config.routes, &config.timeout)?;

    let forward_msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.first_forward_contract.to_string(),
        msg: to_binary(&PacketForwardMsg::Forward(ForwardMsg {
            emergency_claimer: info.sender.to_string(),
            receiver: msg.receivers[0].clone(),
            port: config.routes[0].src_port.clone(),
            channel: config.routes[0].src_channel.clone(),
            timeout: config.timeout.clone(),
            memo: serde_json_wasm::to_string(&memo).unwrap(),
        }))?,
        funds: vec![Coin::new(subtracted.u128(), coin.denom.clone())],
    });
    response = response.add_message(forward_msg);

    // TODO: add events
    response = response.add_attribute("action", "swap");

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
        .checked_mul(Decimal::from_atomics(amount, 0).unwrap())?
        .to_uint_floor()
        .min(min)
        .max(max);
    let subtracted = amount.checked_sub(fee)?;

    Ok((fee, subtracted))
}

#[cfg(not(feature = "library"))]
fn is_correct_denom(swapping_denom: &str, correct_denom: &str) -> bool {
    swapping_denom == correct_denom
}
