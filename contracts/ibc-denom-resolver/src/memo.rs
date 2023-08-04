use std::time::Duration;

use crate::error::ContractError;
use crate::helpers::OptionRefMemo;
use crate::types::{Destination, Route};
use crate::types::{IbcHooksMetadata, Memo, Msg, PacketForwardMetadata};
use packet_forward::msgs::ForwardMsg;

pub fn construct_packet_memo(
    receivers: &[String],
    routes: &[Route],
    timeout: &Duration,
) -> Result<Memo, ContractError> {
    let memo: &mut Option<Memo> = &mut None;
    let mut last_receiver = receivers.last().ok_or(ContractError::EmptyRoutes {})?;

    for (receiver, route) in receivers.iter().zip(routes).rev() {
        let next_child_memo = match &route.destination {
            Destination::Terminal => None,
            Destination::PacketForwardMiddleware => Some(Memo {
                forward: Some(PacketForwardMetadata {
                    receiver: last_receiver.clone(),
                    port: route.src_port.clone(),
                    channel: route.src_channel.clone(),
                    timeout: Some(timeout.as_nanos().to_string()),
                    retries: None,
                    next: match memo {
                        Some(memo) => Some(Box::new(memo.clone())),
                        None => None,
                    },
                }),
                wasm: None,
            }),
            Destination::IbcHooks { contract } => Some(Memo {
                forward: None,
                wasm: Some(IbcHooksMetadata {
                    contract: contract.clone(),
                    msg: Msg {
                        raw_message_fields: serde_json_wasm::to_string(&ForwardMsg {
                            emergency_claimer: receiver.clone(),
                            port: route.src_port.clone(),
                            channel: route.src_channel.clone(),
                            receiver: last_receiver.clone(),
                            timeout: timeout.clone(),
                            memo: memo.as_ref().serialize_json(),
                        })
                        .unwrap(),
                    },
                }),
            }),
        };
        *memo = next_child_memo;
        last_receiver = receiver;
    }

    Ok(memo.clone().unwrap())
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::types::{Destination, Route};
    use packet_forward::msgs::ForwardMsg;

    use super::construct_packet_memo;

    #[test]
    fn it_works() {
        // let sender = Addr::unchecked("cosmos1xv9tklw7d82sezh9haa573wufgy59vmwe6xxe5");
        let receivers = vec![
            "osmo1xv9tklw7d82sezh9haa573wufgy59vmwe6xxe5".to_string(),
            "terra1xv9tklw7d82sezh9haa573wufgy59vmwe6xxe5".to_string(),
            "ununifi1xv9tklw7d82sezh9haa573wufgy59vmwe6xxe5".to_string(),
            "cosmos1xv9tklw7d82sezh9haa573wufgy59vmwe6xxe5".to_string(),
        ];
        let routes = vec![
            // Cosmos Hub
            Route {
                src_port: "transfer".to_string(),
                src_channel: "channel-0".to_string(),
                destination: Destination::IbcHooks {
                    contract: "osmo1contractaddress".to_string(),
                },
            },
            // Osmosis
            Route {
                src_port: "transfer".to_string(),
                src_channel: "channel-1".to_string(),
                destination: Destination::PacketForwardMiddleware,
            },
            // Terra
            Route {
                src_port: "transfer".to_string(),
                src_channel: "channel-2".to_string(),
                destination: Destination::IbcHooks {
                    contract: "ununifi1contractaddress".to_string(),
                },
            },
            // UnUniFi
            Route {
                src_port: "transfer".to_string(),
                src_channel: "channel32".to_string(),
                destination: Destination::Terminal,
            },
            // Cosmos Hub
        ];
        let timeout = Duration::from_nanos(1000000000000);

        let memo = construct_packet_memo(&receivers, &routes, &timeout).unwrap();

        println!("{:#?}", memo);

        assert!(memo.forward.is_none());
        assert!(memo.wasm.is_some());
        assert_eq!(
            &memo.wasm.as_ref().unwrap().contract,
            "osmo1contractaddress"
        );
        let forward_msg = serde_json_wasm::from_str::<ForwardMsg>(
            &memo.wasm.as_ref().unwrap().msg.raw_message_fields,
        )
        .unwrap();

        println!("{:#?}", forward_msg);
    }
}
