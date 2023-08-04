use crate::state::{Config, CONFIG};
use bech32::{self, ToBase32, Variant};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Coin, DepsMut, Env, MessageInfo, Response, WasmMsg};
use cw_utils::one_coin;
use ibc_denom_resolver::error::ContractError;
use ibc_denom_resolver::msg::{
    Destination, ExecuteMsg, InstantiateMsg, Route, SwapMsg, UpdateConfigMsg,
};
use packet_forward::msg::{ForwardMsg, IbcHooksMetadata, Memo, Msg, PacketForwardMetadata};
use serde_json::value::RawValue;

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

fn construct_packet_memo(
    address_bytes: &[u8],
    last_recipient: String,
    routes: &[Route],
) -> Result<Option<Box<Memo>>, ContractError> {
    let memo: &mut Option<Box<Memo>> = &mut None;
    let mut receiver = last_recipient;

    for route in routes.iter().rev() {
        let address_in_dst = bech32::encode(
            &route.dst_bech32_prefix,
            address_bytes.to_base32(),
            Variant::Bech32,
        )
        .unwrap();

        match route.destination {
            Destination::PacketForwardMiddleware {} => {
                let next_child_memo = Box::new(Memo {
                    forward: Some(PacketForwardMetadata {
                        receiver: receiver,
                        port: route.port,
                        channel: route.channel,
                        timeout: None,
                        retries: None,
                        next: *memo,
                    }),
                    wasm: None,
                });
                let next_receiver = address_in_dst;

                receiver = next_receiver;
                *memo = Some(next_child_memo);
            }
            Destination::IbcHooks { contract } => {
                let next_child_memo = Box::new(Memo {
                    forward: None,
                    wasm: Some(IbcHooksMetadata {
                        contract,
                        msg: Msg {
                            raw_message_fields: serde_json::to_string(&ForwardMsg {
                                emergency_claimer: address_in_dst,
                                port: route.port,
                                channel: route.channel,
                                receiver: receiver,
                                memo: memo,
                            })
                            .unwrap(),
                        },
                    }),
                });
                let next_receiver = contract;

                receiver = next_receiver;
                *memo = Some(next_child_memo);
            }
        };
    }

    Ok(*memo)
}

pub fn execute_swap(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    coin: Coin,
    msg: SwapMsg,
) -> Result<Response, ContractError> {
    let mut response = Response::new().add_message(WasmMsg::Execute {
        contract_addr: (),
        msg: (),
        funds: (),
    });

    Ok(response)
}
