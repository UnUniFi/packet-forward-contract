use crate::state::CONFIG;
use bech32::{self, ToBase32, Variant};
#[cfg(not(feature = "library"))]
use cosmwasm_std::{Coin, DepsMut, Env, IbcMsg, IbcTimeout, MessageInfo, Response, Timestamp};
use ibc_denom_resolver::msg::SwapMsg;
use ibc_denom_resolver::types::Destination;
use ibc_denom_resolver::{error::ContractError, types::Route};
use packet_forward::msg::ForwardMsg;
use packet_forward::types::{IbcHooksMetadata, Memo, Msg, PacketForwardMetadata};

fn construct_packet_memo(
    address_bytes: &[u8],
    last_recipient: String,
    routes: &[Route],
    timeout: &IbcTimeout,
) -> Result<Option<Memo>, ContractError> {
    let memo: &mut Option<Memo> = &mut None;
    let mut receiver = last_recipient;

    for route in routes.iter().rev() {
        let address_in_dst = bech32::encode(
            &route.dst_bech32_prefix,
            address_bytes.to_base32(),
            Variant::Bech32,
        )
        .unwrap();

        match &route.destination {
            Destination::PacketForwardMiddleware => {
                let next_child_memo = Memo {
                    forward: Some(PacketForwardMetadata {
                        receiver,
                        port: route.port.clone(),
                        channel: route.channel.clone(),
                        timeout: None,
                        retries: None,
                        next: match memo {
                            Some(memo) => Some(Box::new(memo.clone())),
                            None => None,
                        },
                    }),
                    wasm: None,
                };
                let next_receiver = address_in_dst;

                receiver = next_receiver;
                *memo = Some(next_child_memo);
            }
            Destination::IbcHooks { contract } => {
                let next_child_memo = Memo {
                    forward: None,
                    wasm: Some(IbcHooksMetadata {
                        contract: contract.clone(),
                        msg: Msg {
                            raw_message_fields: serde_json_wasm::to_string(&ForwardMsg {
                                emergency_claimer: address_in_dst,
                                port: route.port.clone(),
                                channel: route.channel.clone(),
                                receiver,
                                timeout: timeout.clone(),
                                memo: memo.clone(),
                            })
                            .unwrap(),
                        },
                    }),
                };
                let next_receiver = contract.clone();

                receiver = next_receiver;
                *memo = Some(next_child_memo);
            }
        };
    }

    Ok(memo.clone())
}

pub fn execute_swap(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    coin: Coin,
    msg: SwapMsg,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    let timeout = IbcTimeout::with_timestamp(Timestamp::from_seconds(config.timeout_seconds));
    let first_route_info = construct_packet_memo(
        &info.sender.as_bytes(),
        msg.recipient,
        &config.routes[..1],
        &timeout,
    )?
    .unwrap();
    let memo = construct_packet_memo(
        &info.sender.as_bytes(),
        msg.recipient,
        &config.routes[1..],
        &timeout,
    )?;

    let msg = if let Some(forward) = first_route_info.forward {
        IbcMsg::Transfer {
            channel_id: forward.channel.clone(),
            to_address: forward.receiver,
            amount: coin,
            timeout,
            memo: match memo {
                Some(memo) => Some(serde_json_wasm::to_string(memo).unwrap()),
                None => None,
            },
        }
    } else if let Some(wasm) = first_route_info.wasm {
        let forward_msg =
            serde_json_wasm::from_str::<ForwardMsg>(wasm.msg.raw_message_fields.as_str()).unwrap();

        IbcMsg::Transfer {
            channel_id: forward_msg.channel.clone(),
            to_address: forward_msg.receiver.clone(),
            amount: coin,
            timeout,
            memo: match memo {
                Some(memo) => Some(serde_json_wasm::to_string(memo).unwrap()),
                None => None,
            },
        }
    };

    let mut response = Response::new().add_message(msg);

    Ok(response)
}
