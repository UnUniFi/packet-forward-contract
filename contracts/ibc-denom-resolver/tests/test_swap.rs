use cosmwasm_std::{Decimal, Uint128, Coin, CosmosMsg, BankMsg, SubMsg};
use cosmwasm_std::testing::{mock_env, mock_info};
use cw_utils::one_coin;
use helpers::th_query;
use ibc_denom_resolver::error::ContractError;
use ibc_denom_resolver::execute::swap::execute_swap;
use ibc_denom_resolver::execute::update_config::execute_update_config;
use ibc_denom_resolver::msgs::{QueryMsg, UpdateConfigMsg, SwapMsg};
use ibc_denom_resolver::types::{Config, FeeConfig};

use crate::helpers::setup;

mod helpers;

#[test]
pub fn swap() {
    let mut deps = setup();
    let sender = "anyone";

    // Failure case due to the different number of the routes and recerivers
    {
        let swap_msg = SwapMsg {
            receivers: vec![],
        };
        let info = mock_info(sender, &[Coin{denom: String::from("fail"), amount: Uint128::one()}]);

        let err = execute_swap(
            deps.as_mut(),
            mock_env(),
            info.clone(),
            one_coin(&info).unwrap(),
            swap_msg,
        ).unwrap_err();

        assert_eq!(ContractError::InvalidReceiversLength, err)
    }

    let receivers = vec![
        "osmo1xv9tklw7d82sezh9haa573wufgy59vmwe6xxe5".to_string(),
        "terra1xv9tklw7d82sezh9haa573wufgy59vmwe6xxe5".to_string(),
        "ununifi1xv9tklw7d82sezh9haa573wufgy59vmwe6xxe5".to_string(),
        "cosmos1xv9tklw7d82sezh9haa573wufgy59vmwe6xxe5".to_string(),
    ];

    // Success case and fee rate is 0%
    {
        let swap_msg = SwapMsg {
             receivers: receivers.clone(),
        };
        let info = mock_info(sender, &[Coin{denom: String::from("uguu"), amount: Uint128::one()}]);

        let res = execute_swap(
            deps.as_mut(),
            mock_env(),
            info.clone(),
            one_coin(&info).unwrap(),
            swap_msg,
        ).unwrap();

        assert_eq!(1, res.messages.len());
    }

    // Success case and fee rate is not 0%
    {
        // update config
        let new_fee_conf = FeeConfig {
            commission_rate: Decimal::one(),
            min: Uint128::one(),
            max: Uint128::one().checked_add(Uint128::one()).unwrap(),
        };
        execute_update_config(
            deps.as_mut(),
            mock_env(),
            mock_info(sender, &[]),
            UpdateConfigMsg {
                owner: None,
                treasury: None,
                fee: Some(new_fee_conf.clone()),
                timeout: None,
            },
        )
        .unwrap();

        let swap_msg = SwapMsg {
            receivers: receivers.clone(),
        };
        let info = mock_info(sender, &[Coin{denom: String::from("uguu"), amount: Uint128::from(100 as u32)}]);

        let res = execute_swap(
            deps.as_mut(),
            mock_env(),
            info.clone(),
            one_coin(&info).unwrap(),
            swap_msg,
        ).unwrap();

        assert_eq!(2, res.messages.len());
        let config: Config = th_query(deps.as_ref(), QueryMsg::Config {});
        let send_msg = SubMsg::new(CosmosMsg::Bank(BankMsg::Send {
            to_address: config.treasury.to_string(),
            amount: vec![Coin::new(2 as u128, config.denom.clone())],
        }));
        assert_eq!(res.messages[0], send_msg)
    }

}
