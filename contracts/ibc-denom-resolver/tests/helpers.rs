use std::time::Duration;

use cosmwasm_schema::serde;

use ibc_denom_resolver::error::ContractError;
use ibc_denom_resolver::msgs::{InstantiateMsg, QueryMsg, UpdateConfigMsg};
use ibc_denom_resolver::execute::update_config::*;
use ibc_denom_resolver::contract::instantiate;
use ibc_denom_resolver::contract::query;

use cosmwasm_std::testing::{
    mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage,
};
use cosmwasm_std::{
    from_binary, Decimal, Deps, OwnedDeps, Uint128,
};

use ibc_denom_resolver::types::{FeeConfig, Route, Destination, };

pub const DEFAULT_TIMEOUT: u64 = 3600; // 1 hour,
pub const CONTRACT_PORT: &str = "ibc:wasm1234567890abcdef";

pub fn setup() -> OwnedDeps<MockStorage, MockApi, MockQuerier> {
    let mut deps = mock_dependencies();

    // instantiate an empty contract
    let routes = vec![
        // Cosmos Hub
        Route {
            src_port: "transfer".to_string(),
            src_channel: "channel-0".to_string(),
            destination: Destination::PacketForwardContract("osmo1contractaddress".to_string()),
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
            destination: Destination::PacketForwardContract(
                "ununifi1contractaddress".to_string(),
            ),
        },
        // UnUniFi
        Route {
            src_port: "transfer".to_string(),
            src_channel: "channel-3".to_string(),
            destination: Destination::Terminal,
        },
        // Cosmos Hub
    ];
    let instantiate_msg = InstantiateMsg {
        denom: String::from("uguu"), // random leteral
        first_forward_contract: String::from("ununifi14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sm5z28e"), // random address
        routes: routes,
        timeout: Duration::from_secs(DEFAULT_TIMEOUT),
        treasury: String::from("ununifi1f5vsnrwe7h9dhhyxkwr4yah5u0cke69h03latc"), // totally random. DONT use on mainnet
        fee: FeeConfig {
            commission_rate: Decimal::percent(0),
            min: Uint128::zero(),
            max: Uint128::zero(),
        },
    };
    let info = mock_info(&String::from("anyone"), &[]);
    let res = instantiate(deps.as_mut(), mock_env(), info, instantiate_msg).unwrap();
    assert_eq!(0, res.messages.len());

    deps
}

pub fn th_query<T: serde::de::DeserializeOwned>(deps: Deps, msg: QueryMsg) -> T {
    from_binary(&query(deps, mock_env(), msg).unwrap()).unwrap()
}
