use std::ops::Mul;

use crate::state::{Config, CONFIG};
use bech32::{self, ToBase32, Variant};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Coin, DepsMut, Env, IbcMsg, MessageInfo, Response};
use cw_utils::one_coin;
use ibc_denom_resolver::error::ContractError;
use ibc_denom_resolver::resolver::{ExecuteMsg, InstantiateMsg, Route, SwapMsg, UpdateConfigMsg};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

//Initialize the contract.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    if msg.routes.len() == 0 {
        return Err(ContractError::EmptyRoutes {});
    }

    let config = Config {
        owner: info.sender,
        denom: msg.denom,
        routes: msg.routes,
        fee: msg.fee,
    };

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
        ExecuteMsg::Swap(msg) => {
            let coin: Coin = one_coin(&info).map_err(|err| ContractError::Payment(err))?;
            execute_swap(deps, env, info, coin, msg)
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

/// https://medium.com/the-interchain-foundation/moving-beyond-simple-token-transfers-d42b2b1dc29b
/// https://github.com/strangelove-ventures/packet-forward-middleware
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
struct PacketMemo {
    receiver: String,
    port: String,
    channel: String,
    timeout: Option<String>,
    retries: Option<u32>,
    next: Option<Box<PacketMemo>>,
}

fn construct_packet_memo(routes: &[Route]) -> PacketMemo {}

pub fn execute_swap(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    coin: Coin,
    msg: SwapMsg,
) -> Result<Response, ContractError> {
    let config: Config = CONFIG.load(deps.storage)?;

    let fee_subtracted_amount = coin
        .amount
        .checked_sub(config.fee.commission_rate.mul(coin.amount))
        .or(ContractError::InsufficientFunds {})?;

    let address_bytes = info.sender.as_bytes();
    let receiver = bech32::encode(
        &config.routes[0].dst_bech32_prefix,
        address_bytes.to_base32(),
        Variant::Bech32,
    );

    let memo = construct_packet_memo(&config.routes);
    let data = FungibleTokenPacketData {
        denom: coin.denom,
        amount: fee_subtracted_amount,
        sender: info.sender.to_string(),
        receiver: receiver,
        memo: serde_json::to_string(&memo)?,
    };

    let mut response = Response::new().add_message(IbcMsg::SendPacket {
        channel_id: config.routes[0].src_channel_id,
        data: (),
        timeout: (),
    });

    Ok(response)
}
