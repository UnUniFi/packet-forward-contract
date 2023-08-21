use std::time::Duration;

use cosmwasm_std::{Decimal, Uint128, Coin};
use cosmwasm_std::testing::{mock_env, mock_info};
use cw_utils::one_coin;
use helpers::{th_query, CONTRACT_PORT, DEFAULT_TIMEOUT};
use packet_forward::error::ContractError;
use packet_forward::execute::forward::execute_forward;
use packet_forward::msgs::{QueryMsg, UpdateConfigMsg, ForwardMsg};
use packet_forward::execute::update_config::execute_update_config;
use packet_forward::state::{SUB_MSG_ID, REQUEST_ID};
use packet_forward::types::{Config, FeeConfig};

use crate::helpers::setup;

mod helpers;

#[test]
fn test_execute_forward() {
    let mut deps = setup();
    let sender = "anyone";

    let forward_msg = ForwardMsg {
        emergency_claimer: String::from("ununifi1hlmmymdrajk9whu8ndfxwf9gjh9fs2mjsdwys3"),
        receiver: String::from("ununifi1ph9raqr2d07qlhllfeth5a3tm4553xlaxswsuc"),
        port: String::from(CONTRACT_PORT),
        channel: String::from("channel-random"),
        timeout: Duration::from_secs(DEFAULT_TIMEOUT),
        memo: String::from("random-memo"),
    };

    // Success case
    {
        let info = mock_info(sender, &[Coin{denom: String::from("uguu"), amount: Uint128::one()}]);

        let res = execute_forward(
            deps.as_mut(),
            mock_env(),
            info.clone(),
            one_coin(&info).unwrap(),
            forward_msg.clone()
        )
            .unwrap();

        assert_eq!(1, res.messages.len());

        let sub_msg_id = SUB_MSG_ID.load(deps.as_ref().storage).unwrap();
        assert_eq!(1, sub_msg_id);
        let request_id = REQUEST_ID.load(deps.as_ref().storage).unwrap();
        assert_eq!(1, request_id);
    }

    // Success case with fee subtraction
    {
        // update config
        let new_fee_conf = FeeConfig {
            commission_rate: Decimal::percent(1),
        };
        execute_update_config(
            deps.as_mut(),
            mock_env(),
            mock_info(sender, &[]),
            UpdateConfigMsg {
                owner: None,
                treasury: None,
                fee: Some(new_fee_conf.clone()),
            },
        )
            .unwrap();

        let info = mock_info(sender, &[Coin{denom: String::from("uguu"), amount: Uint128::from(1000 as u32)}]);

        let res = execute_forward(
            deps.as_mut(),
            mock_env(),
            info.clone(),
            one_coin(&info).unwrap(),
            forward_msg.clone()
        )
            .unwrap();

            println!("{:?}", res);
        assert_eq!(2, res.messages.len());

        let sub_msg_id = SUB_MSG_ID.load(deps.as_ref().storage).unwrap();
        assert_eq!(2, sub_msg_id);
        let request_id = REQUEST_ID.load(deps.as_ref().storage).unwrap();
        assert_eq!(2, request_id);
    }
}
