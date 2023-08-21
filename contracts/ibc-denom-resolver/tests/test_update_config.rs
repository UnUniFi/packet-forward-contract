use std::time::Duration;

use cosmwasm_std::{Decimal, Uint128};
use cosmwasm_std::testing::{mock_env, mock_info};
use helpers::th_query;
use ibc_denom_resolver::error::ContractError;
use ibc_denom_resolver::msgs::{QueryMsg, UpdateConfigMsg};
use ibc_denom_resolver::execute::update_config::execute_update_config;
use ibc_denom_resolver::types::{Config, FeeConfig};

use crate::helpers::setup;

mod helpers;

#[test]
fn initialized_state() {
    let deps = setup();

    let config: Config = th_query(deps.as_ref(), QueryMsg::Config {});
    assert_eq!(Decimal::zero(), config.fee.commission_rate);
    assert_eq!("ibc/uguu", config.input_denom);
}

#[test]
fn update_config() {
    let mut deps = setup();
    let sender = "anyone";

    // Success case only for the change of owner
    {
        // Change with other values for further tests
        execute_update_config(
            deps.as_mut(),
            mock_env(),
            mock_info(sender, &[]),
            UpdateConfigMsg {
                owner: Some(String::from("ununifi1v2k8kt24uqes5l5js772eamzahpg53p38jytlj")),
                treasury: None,
                fee: None,
                timeout: None,
            },
        )
        .unwrap();

        let config: Config = th_query(deps.as_ref(), QueryMsg::Config {});

        assert_eq!("ununifi1v2k8kt24uqes5l5js772eamzahpg53p38jytlj", config.owner);
    }

    // Success case for the changes of all params
    {
        // Change with other values for further tests
        let new_fee_conf = FeeConfig {
            commission_rate: Decimal::one(),
            min: Uint128::one(),
            max: Uint128::one().checked_add(Uint128::one()).unwrap(),
        };
        execute_update_config(
            deps.as_mut(),
            mock_env(),
            mock_info("ununifi1v2k8kt24uqes5l5js772eamzahpg53p38jytlj", &[]),
            UpdateConfigMsg {
                owner: Some(String::from("ununifi13vs27vvzhdljpexf5zc2zs5vs66yywq8gu8g0x")),
                treasury: Some(String::from("ununifi13e0tws93sujjg40052a5jew4933saa9qksn7jn")),
                fee: Some(new_fee_conf.clone()),
                timeout: Some(Duration::from_secs(7200 as u64)),
            },
        )
        .unwrap();

        let config: Config = th_query(deps.as_ref(), QueryMsg::Config {});

        assert_eq!("ununifi13vs27vvzhdljpexf5zc2zs5vs66yywq8gu8g0x", config.owner);
        assert_eq!("ununifi13e0tws93sujjg40052a5jew4933saa9qksn7jn", config.treasury);
        assert_eq!(new_fee_conf, config.fee);
        assert_eq!(Duration::from_secs(7200 as u64), config.timeout);
    }

    {
        let bad_sender = "bad_sender";
        let err = execute_update_config(
                deps.as_mut(),
                mock_env(),
                mock_info(bad_sender, &[]),
                UpdateConfigMsg {
                    owner: None,
                    treasury: None,
                    fee: None,
                    timeout: None,
                },
            )
            .unwrap_err();
        assert_eq!(ContractError::Unauthorized {}, err);
    }
}
