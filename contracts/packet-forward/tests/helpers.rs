use cosmwasm_schema::serde;

use packet_forward::msgs::{InstantiateMsg, QueryMsg};
use packet_forward::contract::instantiate;
use packet_forward::contract::query;

use cosmwasm_std::testing::{
    mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage,
};
use cosmwasm_std::{
    from_binary, Decimal, Deps, OwnedDeps,
};

use packet_forward::types::FeeConfig;

pub const DEFAULT_TIMEOUT: u64 = 3600; // 1 hour,
pub const CONTRACT_PORT: &str = "ibc:wasm1234567890abcdef";

pub fn setup() -> OwnedDeps<MockStorage, MockApi, MockQuerier> {
    let mut deps = mock_dependencies();

    // instantiate an empty contract
    let instantiate_msg = InstantiateMsg {
        treasury: String::from("ununifi1f5vsnrwe7h9dhhyxkwr4yah5u0cke69h03latc"), // totally random. DONT use on mainnet
        fee: FeeConfig {
            commission_rate: Decimal::percent(0),
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
